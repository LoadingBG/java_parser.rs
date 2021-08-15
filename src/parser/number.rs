use regex::Regex;

use crate::parser::{ Parser, ParserContext, Token };



struct BinIntParser {
    regex: Regex,
}

impl BinIntParser {
    fn new() -> Self {
        Self { regex: Regex::new(r"^([\+-]\s*)?0[bB]([01](_?[01])*)\b").unwrap() }
    }
}

impl Parser for BinIntParser {
    fn parse(&self, code: &String, _: ParserContext) -> Option<Vec<Token>> {
        self.regex.captures(code).and_then(|captures|
            match (captures.get(1), captures.get(2)) {
                (sign, Some(number)) if i32::from_str_radix(&number.as_str().replace("_", ""), 2).is_ok() => {
                    if let Some(sign) = sign {
                        Some(vec![
                            Token::new(String::from("number.sign"), sign.as_str().len()),
                            Token::new(String::from("number.prefix"), 2),
                            Token::new(String::from("number"), number.as_str().len()),
                        ])
                    } else {
                        Some(vec![
                            Token::new(String::from("number.prefix"), 2),
                            Token::new(String::from("number"), number.as_str().len()),
                        ])
                    }
                },
                _ => None
            }
        )
    }
}



struct OctIntParser {
    regex: Regex,
}

impl OctIntParser {
    fn new() -> Self {
        Self { regex: Regex::new(r"^([\+-]\s*)?(0)((_?[0-7])+)\b").unwrap() }
    }
}

impl Parser for OctIntParser {
    fn parse(&self, code: &String, _: ParserContext) -> Option<Vec<Token>> {
        if let Some(captures) = self.regex.captures(code) {
            if let (Some(sign), Some(_), Some(number)) = (captures.get(1), captures.get(2), captures.get(3)) {
                let sign = sign.as_str();
                let number = number.as_str();
                if i32::from_str_radix(&number.replace("_", ""), 8).is_ok() && !number.ends_with("_") {
                    if sign.is_empty() {
                        Some(vec![
                            Token::new(String::from("number.prefix"), 1),
                            Token::new(String::from("number"), number.len()),
                        ])
                    } else {
                        Some(vec![
                            Token::new(String::from("number.sign"), sign.len()),
                            Token::new(String::from("number.prefix"), 1),
                            Token::new(String::from("number"), number.len()),
                        ])
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}



struct DecIntParser {
    regex: Regex,
}

impl DecIntParser {
    fn new() -> Self {
        Self { regex: Regex::new(r"^([\+-]\s*)?((0|[1-9](_?\d)*))\b").unwrap() }
    }
}

impl Parser for DecIntParser {
    fn parse(&self, code: &String, _: ParserContext) -> Option<Vec<Token>> {
        if let Some(captures) = self.regex.captures(&code) {
            if let (Some(sign), Some(capture)) = (captures.get(1), captures.get(2)) {
                let sign = sign.as_str();
                let capture = capture.as_str();
                if capture.replace("_", "").parse::<i32>().is_ok() {
                    if sign.is_empty() {
                        Some(vec![
                            Token::new(String::from("number"), capture.len())
                        ])
                    } else {
                        Some(vec![
                            Token::new(String::from("number.sign"), sign.len()),
                            Token::new(String::from("number"), capture.len()),
                        ])
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}



struct HexIntParser {
    regex: Regex,
}

impl HexIntParser {
    fn new() -> Self {
        Self { regex: Regex::new(r"^[\+-]?").unwrap() }
    }
}



pub struct NumberParser {
    parsers: Vec<Box<dyn Parser>>,
}

impl NumberParser {
    pub fn new() -> Self {
        Self { parsers: vec![
            Box::new(BinIntParser::new()),
            Box::new(OctIntParser::new()),
            Box::new(DecIntParser::new()),
        ] }
    }
}

impl Parser for NumberParser {
    fn parse(&self, code: &String, context: ParserContext) -> Option<Vec<Token>> {
        self.parsers
            .iter()
            .filter_map(|p| p.parse(code, context))
            .nth(0)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{ Parser, ParserContext, Token };

    #[test]
    fn bin_int_parser() {
        let p = BinIntParser::new();
        assert_eq!(
            Some(vec![
                Token::new(String::from("number.prefix"), 2),
                Token::new(String::from("number"), 3),
            ]),
            p.parse(&String::from("0b111"), ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new(String::from("number.prefix"), 2),
                Token::new(String::from("number"), 7),
            ]),
            p.parse(&String::from("0B111_111"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("0b_1"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("0b111_"), ParserContext::new())
        );
    }

    #[test]
    fn oct_int_parser() {
        let p = OctIntParser::new();
        assert_eq!(
            Some(vec![
                Token::new(String::from("number.prefix"), 1),
                Token::new(String::from("number"), 2),
            ]),
            p.parse(&String::from("017"), ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new(String::from("number.prefix"), 1),
                Token::new(String::from("number"), 7),
            ]),
            p.parse(&String::from("0123_456"), ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new(String::from("number.prefix"), 1),
                Token::new(String::from("number"), 8),
            ]),
            p.parse(&String::from("0_123_456"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("0"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("0_"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("0_123_"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("12"), ParserContext::new())
        );
    }

    #[test]
    fn dec_int_parser() {
        let p = DecIntParser::new();
        assert_eq!(
            Some(vec![
                Token::new(String::from("number"), 3)
            ]),
            p.parse(&String::from("123"), ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new(String::from("number"), 5)
            ]),
            p.parse(&String::from("1_000"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("12345678909876543"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("01"), ParserContext::new())
        );
        assert_eq!(
            None,
            p.parse(&String::from("0_"), ParserContext::new())
        ); 
        assert_eq!(
            None,
            p.parse(&String::from("123_"), ParserContext::new())
        ); 
    }

    #[test]
    fn number_parser() {
        let p = NumberParser::new();
        assert_eq!(
            Some(vec![
                Token::new(String::from("number"), 1)
            ]),
            p.parse(&String::from("0"), ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new(String::from("number.sign"), 1),
                Token::new(String::from("number.prefix"), 2),
                Token::new(String::from("number"), 1),
            ]),
            p.parse(&String::from("+0b1"), ParserContext::new())
        );
    }
}
