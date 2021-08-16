use lazy_static::lazy_static;
use regex::{Match, Regex};

use crate::parser::{Parser, ParserContext, Token};

fn is_int_in_bounds(sign: Option<Match>, number: Match, suffix: Option<Match>, radix: u32) -> bool {
    let number = &number.as_str().replace("_", "");
    match suffix {
        Some(_) => {
            if let Some(sign) = sign {
                i64::from_str_radix(&(String::from(&sign.as_str()[..1]) + number), radix).is_ok()
            } else {
                i64::from_str_radix(number, radix).is_ok()
            }
        }
        None => {
            if let Some(sign) = sign {
                i32::from_str_radix(&(String::from(&sign.as_str()[..1]) + number), radix).is_ok()
            } else {
                i32::from_str_radix(number, radix).is_ok()
            }
        }
    }
}

fn is_float_in_bounds(
    sign: Option<Match>,
    whole: Option<Match>,
    frac: Option<Match>,
    e: Option<Match>,
    exp_sign: Option<Match>,
    exp_power: Option<Match>,
    suffix: Option<Match>,
) -> bool {
    let mut num = String::new();
    if let Some(sign) = sign {
        num += sign.as_str();
    }
    if let Some(whole) = whole {
        num += &whole.as_str().replace("_", "");
    }
    if let Some(frac) = frac {
        num += ".";
        num += &frac.as_str().replace("_", "");
    }
    if e.is_some() {
        num += "e";
        if let Some(exp_sign) = exp_sign {
            num += exp_sign.as_str();
        }
        num += &exp_power.unwrap().as_str().replace("_", "");
    }
    println!("Parsing: {}", num);
    match suffix {
        Some(s) if s.as_str().to_lowercase() == "f" => num.parse::<f32>().is_ok(),
        _ => num.parse::<f64>().is_ok(),
    }
}

fn tokenize_matches(
    condition: bool,
    sign: Option<Match>,
    whitespaces: Option<Match>,
    prefix_len: Option<usize>,
    whole: Option<Match>,
    dot: bool,
    fraction: Option<Match>,
    e: Option<Match>,
    exp_sign: Option<Match>,
    exp_power: Option<Match>,
    suffix: Option<Match>,
) -> Option<Vec<Token>> {
    if !condition {
        return None;
    }
    let mut tokens = vec![];
    if sign.is_some() {
        tokens.push(Token::number_sign());
    }
    if let Some(ws) = whitespaces {
        tokens.push(Token::whitespace(ws.as_str().len()));
    }
    if let Some(prefix_len) = prefix_len {
        tokens.push(Token::new("number.prefix", prefix_len));
    }
    if let Some(whole) = whole {
        tokens.push(Token::new("number", whole.as_str().len()));
    }
    if dot {
        tokens.push(Token::number_dot());
    }
    if let Some(frac) = fraction {
        tokens.push(Token::new("number", frac.as_str().len()));
    }
    if e.is_some() {
        tokens.push(Token::number_e());
    }
    if exp_sign.is_some() {
        tokens.push(Token::number_e_sign());
    }
    if let Some(exp_power) = exp_power {
        tokens.push(Token::new("number.e.power", exp_power.as_str().len()));
    }
    if suffix.is_some() {
        tokens.push(Token::number_suffix());
    }
    Some(tokens)
}

enum DigitsOrCustom<'a> {
    Digits(&'a str),
    Custom(&'a str),
}

use DigitsOrCustom::{Custom, Digits};

fn create_int_regex(prefix: &str, digits_or_custom: DigitsOrCustom) -> Regex {
    let r = match digits_or_custom {
        Digits(d) => Regex::new(&format!(
            r"^([\+-])?(\s+)?{p}({d}(?:_?{d})*)([lL])?\b",
            p = prefix,
            d = d
        )),
        Custom(c) => Regex::new(&format!(
            r"^([\+-])?(\s+)?{p}({c})([lL])?\b",
            p = prefix,
            c = c
        )),
    };
    match r {
        Ok(r) => r,
        Err(e) => panic!(e),
    }
}

fn create_float_regex(middle_pat: &str) -> Regex {
    let r = Regex::new(&format!(
        r"^([\+-])?(\s+)?{m}([eE]([\+-])?(\d(?:_?\d)*))?([dDfF])?\b",
        m = middle_pat
            .replace("d", r"\.")
            .replace("o", r"(\.)?")
            .replace("n", r"(\d(?:_?\d)*)")
    ));
    match r {
        Ok(r) => r,
        Err(e) => panic!(e),
    }
}

lazy_static! {
    static ref BIN_INT_REGEX: Regex = create_int_regex("0[bB]", Digits(r"[01]"));
    static ref OCT_INT_REGEX: Regex = create_int_regex("0", Custom(r"(?:_?[0-7])*"));
    static ref DEC_INT_REGEX: Regex = create_int_regex("", Custom(r"0|[1-9](?:_?\d)*"));
    static ref HEX_INT_REGEX: Regex = create_int_regex("0[xX]", Digits(r"[\da-fA-F]"));
    static ref FULL_FLOAT_REGEX: Regex = create_float_regex("ndn");
    static ref FRAC_FLOAT_REGEX: Regex = create_float_regex("dn");
    static ref WHOLE_FLOAT_REGEX: Regex = create_float_regex("no");
}

struct BinIntParser;

impl BinIntParser {
    fn new() -> Self {
        Self
    }
}

impl Parser for BinIntParser {
    fn parse(&self, code: &str, _: ParserContext) -> Option<Vec<Token>> {
        BIN_INT_REGEX.captures(code).and_then(|captures| {
            let (sign, ws, number, suffix) = (
                captures.get(1),
                captures.get(2),
                captures.get(3),
                captures.get(4),
            );
            tokenize_matches(
                is_int_in_bounds(sign, number.unwrap(), suffix, 2),
                sign,
                ws,
                Some(2),
                number,
                false,
                None,
                None,
                None,
                None,
                suffix,
            )
        })
    }
}

struct OctIntParser;

impl OctIntParser {
    fn new() -> Self {
        Self
    }
}

impl Parser for OctIntParser {
    fn parse(&self, code: &str, _: ParserContext) -> Option<Vec<Token>> {
        OCT_INT_REGEX.captures(code).and_then(|captures| {
            let (sign, ws, number, suffix) = (
                captures.get(1),
                captures.get(2),
                captures.get(3),
                captures.get(4),
            );
            tokenize_matches(
                is_int_in_bounds(sign, number.unwrap(), suffix, 8),
                sign,
                ws,
                Some(1),
                number,
                false,
                None,
                None,
                None,
                None,
                suffix,
            )
        })
    }
}

struct DecIntParser;

impl DecIntParser {
    fn new() -> Self {
        Self
    }
}

impl Parser for DecIntParser {
    fn parse(&self, code: &str, _: ParserContext) -> Option<Vec<Token>> {
        DEC_INT_REGEX.captures(code).and_then(|captures| {
            let (sign, ws, number, suffix) = (
                captures.get(1),
                captures.get(2),
                captures.get(3),
                captures.get(4),
            );
            tokenize_matches(
                is_int_in_bounds(sign, number.unwrap(), suffix, 10),
                sign,
                ws,
                None,
                number,
                false,
                None,
                None,
                None,
                None,
                suffix,
            )
        })
    }
}

struct HexIntParser;

impl HexIntParser {
    fn new() -> Self {
        Self
    }
}

impl Parser for HexIntParser {
    fn parse(&self, code: &str, _: ParserContext) -> Option<Vec<Token>> {
        HEX_INT_REGEX.captures(code).and_then(|captures| {
            let (sign, ws, number, suffix) = (
                captures.get(1),
                captures.get(2),
                captures.get(3),
                captures.get(4),
            );
            tokenize_matches(
                is_int_in_bounds(sign, number.unwrap(), suffix, 16),
                sign,
                ws,
                Some(2),
                number,
                false,
                None,
                None,
                None,
                None,
                suffix,
            )
        })
    }
}

struct FullFloatParser;

impl FullFloatParser {
    fn new() -> Self {
        Self
    }
}

impl Parser for FullFloatParser {
    fn parse(&self, code: &str, _: ParserContext) -> Option<Vec<Token>> {
        FULL_FLOAT_REGEX.captures(code).and_then(|captures| {
            let (sign, ws, whole, frac, e, exp_sign, exp_power, suffix) = (
                captures.get(1),
                captures.get(2),
                captures.get(3),
                captures.get(4),
                captures.get(5),
                captures.get(6),
                captures.get(7),
                captures.get(8),
            );
            tokenize_matches(
                is_float_in_bounds(sign, whole, frac, e, exp_sign, exp_power, suffix),
                sign,
                ws,
                None,
                whole,
                true,
                frac,
                e,
                exp_sign,
                exp_power,
                suffix,
            )
        })
    }
}

struct FracFloatParser;

impl FracFloatParser {
    fn new() -> Self {
        Self
    }
}

impl Parser for FracFloatParser {
    fn parse(&self, code: &str, _: ParserContext) -> Option<Vec<Token>> {
        FRAC_FLOAT_REGEX.captures(code).and_then(|captures| {
            let (sign, ws, frac, e, exp_sign, exp_power, suffix) = (
                captures.get(1),
                captures.get(2),
                captures.get(3),
                captures.get(4),
                captures.get(5),
                captures.get(6),
                captures.get(7),
            );
            tokenize_matches(
                is_float_in_bounds(sign, None, frac, e, exp_sign, exp_power, suffix),
                sign,
                ws,
                None,
                None,
                true,
                frac,
                e,
                exp_sign,
                exp_power,
                suffix,
            )
        })
    }
}

struct WholeFloatParser;

impl WholeFloatParser {
    fn new() -> Self {
        Self
    }
}

impl Parser for WholeFloatParser {
    fn parse(&self, code: &str, _: ParserContext) -> Option<Vec<Token>> {
        WHOLE_FLOAT_REGEX.captures(code).and_then(|captures| {
            let (sign, ws, whole, dot, e, exp_sign, exp_power, suffix) = (
                captures.get(1),
                captures.get(2),
                captures.get(3),
                captures.get(4),
                captures.get(5),
                captures.get(6),
                captures.get(7),
                captures.get(8),
            );
            tokenize_matches(
                is_float_in_bounds(sign, whole, None, e, exp_sign, exp_power, suffix)
                    && (dot.is_some() || e.is_some() || suffix.is_some()),
                sign,
                ws,
                None,
                whole,
                dot.is_some(),
                None,
                e,
                exp_sign,
                exp_power,
                suffix,
            )
        })
    }
}

pub struct NumberParser {
    parsers: Vec<Box<dyn Parser>>,
}

impl NumberParser {
    pub fn new() -> Self {
        Self {
            parsers: vec![
                Box::new(BinIntParser::new()),
                Box::new(OctIntParser::new()),
                Box::new(DecIntParser::new()),
                Box::new(HexIntParser::new()),
                Box::new(FullFloatParser::new()),
                Box::new(FracFloatParser::new()),
                Box::new(WholeFloatParser::new()),
            ],
        }
    }
}

impl Parser for NumberParser {
    fn parse(&self, code: &str, context: ParserContext) -> Option<Vec<Token>> {
        self.parsers
            .iter()
            .filter_map(|p| p.parse(code, context))
            .nth(0)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Parser, ParserContext, Token};
    use super::*;

    #[test]
    fn bin_int_parser() {
        let p = BinIntParser::new();
        assert_eq!(
            Some(vec![
                Token::new("number.prefix", 2),
                Token::new("number", 3),
            ]),
            p.parse(&"0b111", ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new("number.prefix", 2),
                Token::new("number", 7),
            ]),
            p.parse(&"0B111_111", ParserContext::new())
        );
        assert_eq!(None, p.parse(&"0b_1", ParserContext::new()));
        assert_eq!(None, p.parse(&"0b111_", ParserContext::new()));
    }

    #[test]
    fn oct_int_parser() {
        let p = OctIntParser::new();
        assert_eq!(
            Some(vec![
                Token::new("number.prefix", 1),
                Token::new("number", 2),
            ]),
            p.parse(&"017", ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new("number.prefix", 1),
                Token::new("number", 7),
            ]),
            p.parse(&"0123_456", ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new("number.prefix", 1),
                Token::new("number", 8),
            ]),
            p.parse(&"0_123_456", ParserContext::new())
        );
        assert_eq!(None, p.parse(&"0", ParserContext::new()));
        assert_eq!(None, p.parse(&"0_", ParserContext::new()));
        assert_eq!(None, p.parse(&"0_123_", ParserContext::new()));
        assert_eq!(None, p.parse(&"12", ParserContext::new()));
    }

    #[test]
    fn dec_int_parser() {
        let p = DecIntParser::new();
        assert_eq!(
            Some(vec![Token::new("number", 3)]),
            p.parse(&"123", ParserContext::new())
        );
        assert_eq!(
            Some(vec![Token::new("number", 5)]),
            p.parse(&"1_000", ParserContext::new())
        );
        assert_eq!(None, p.parse(&"12345678909876543", ParserContext::new()));
        assert_eq!(None, p.parse(&"01", ParserContext::new()));
        assert_eq!(None, p.parse(&"0_", ParserContext::new()));
        assert_eq!(None, p.parse(&"123_", ParserContext::new()));
    }

    #[test]
    fn hex_int_parser() {
        let p = HexIntParser::new();
        assert_eq!(
            Some(vec![
                Token::new("number.prefix", 2),
                Token::new("number", 3),
            ]),
            p.parse("0x9aF", ParserContext::new())
        );
        assert_eq!(None, p.parse("0x_A", ParserContext::new()));
        assert_eq!(None, p.parse("0xf_", ParserContext::new()));
    }

    #[test]
    fn full_float_parser() {
        let p = FullFloatParser::new();
        assert_eq!(
            Some(vec![
                Token::new("number", 1),
                Token::number_dot(),
                Token::new("number", 1),
            ]),
            p.parse("1.0", ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::new("number", 7),
                Token::number_dot(),
                Token::new("number", 7),
                Token::number_suffix(),
            ]),
            p.parse("1_2_3_4.1_2_3_4f", ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::number_sign(),
                Token::whitespace(2),
                Token::new("number", 3),
                Token::number_dot(),
                Token::new("number", 3),
                Token::number_e(),
                Token::number_e_sign(),
                Token::new("number.e.power", 3),
                Token::number_suffix(),
            ]),
            p.parse("-  1_0.1_0e+1_0d", ParserContext::new())
        );
        assert_eq!(None, p.parse("123_.4f", ParserContext::new()));
        assert_eq!(None, p.parse("123._4f", ParserContext::new()));
        assert_eq!(None, p.parse(".1", ParserContext::new()));
    }

    #[test]
    fn frac_float_parser() {
        let p = FracFloatParser::new();
        assert_eq!(
            Some(vec![Token::number_dot(), Token::new("number", 1),]),
            p.parse(".1", ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::number_dot(),
                Token::new("number", 1),
                Token::number_suffix(),
            ]),
            p.parse(".1d", ParserContext::new())
        );
    }

    #[test]
    fn whole_float_parser() {
        let p = WholeFloatParser::new();
        assert_eq!(
            Some(vec![Token::new("number", 1), Token::number_suffix(),]),
            p.parse("1f", ParserContext::new())
        );
    }

    #[test]
    fn number_parser() {
        let p = NumberParser::new();
        assert_eq!(
            Some(vec![Token::new("number", 1)]),
            p.parse(&"0", ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::number_sign(),
                Token::new("number.prefix", 2),
                Token::new("number", 1),
            ]),
            p.parse(&"+0b1", ParserContext::new())
        );
        assert_eq!(
            Some(vec![
                Token::number_sign(),
                Token::whitespace(2),
                Token::new("number.prefix", 1),
                Token::new("number", 3),
            ]),
            p.parse(&"-\n 0234", ParserContext::new())
        );
        assert_eq!(
            Some(vec![Token::new("number", 10)]),
            p.parse("2147483647", ParserContext::new())
        );
        assert_eq!(None, p.parse("2147483648", ParserContext::new()));
        assert_eq!(
            Some(vec![Token::number_sign(), Token::new("number", 10)]),
            p.parse("-2147483648", ParserContext::new())
        );
    }
}
