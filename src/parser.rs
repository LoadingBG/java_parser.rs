use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
};

//// Standard keywords:
/// Not in use:
// _ (9), const, goto
/// Packages:
// import, package
/// Literals:
// false, null, true
/// Primitives:
// boolean, byte, char, double, float, int, long, short, var (10), void
/// Field modifiers:
// transient, volatile
/// Type identifiers:
// super, this
/// Creation:
// @interface, class, enum, interface, record (16)
/// Relationship:
// extends, implements, permits (17)
/// Access modifiers:
// private, protected, public
/// Restricting modifiers:
// final, non-sealed (17), sealed (17)
/// Method modifiers:
// default, native, synchronized, throws
/// Other modifiers
// abstract, static, strictfp
/// Control flow
// case, catch, do, else, finally, for, if, switch, try, while
/// Flow breakers:
// assert, break, continue, return, throw, yield (14)
/// Operators:
// instanceof, new
//// Module keywords:
/// Not in use:
// _
/// Module:
// module
/// Module modifiers:
// open
/// Relationship:
// exports, opens, requires, provides, uses
/// Relationship modifiers:
// static, transitive
/// Others:
// to, with
pub mod number;

pub struct Token {
    pub name: String,
    pub len: usize,
    pub metadata: HashMap<String, i32>,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Token")
            .field("name", &self.name)
            .field("len", &self.len)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl Token {
    pub fn new(name: &str, len: usize) -> Self {
        Self::with_meta(name, len, HashMap::new())
    }

    pub fn whitespace(len: usize) -> Self {
        Self::new("whitespace", len)
    }

    pub fn number_dot() -> Self {
        Self::new("number.dot", 1)
    }

    pub fn number_sign() -> Self {
        Self::new("number.sign", 1)
    }

    pub fn number_suffix() -> Self {
        Self::new("number.suffix", 1)
    }

    pub fn number_e() -> Self {
        Self::new("number.e", 1)
    }

    pub fn number_e_sign() -> Self {
        Self::new("number.e.sign", 1)
    }

    pub fn with_meta(name: &str, len: usize, metadata: HashMap<String, i32>) -> Self {
        Self {
            name: String::from(name),
            len,
            metadata,
        }
    }

    pub fn add_metadata(&mut self, addition: HashMap<String, i32>) -> &Self {
        self.metadata.extend(addition);
        self
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.len == other.len && self.metadata.eq(&other.metadata)
    }
}

//pub struct Modifiers {
//    modifiers: Vec<bool>,
//}
//
//impl Modifiers {
//    pub fn default() -> Self {
//        Self { modifiers: vec![] }
//    }
//
//    pub fn new(modifiers: &Vec<bool>) -> Self {
//        Self { modifiers: modifiers.clone() }
//    }
//
//    pub fn is_abstract(&self) -> bool {
//        self.modifiers[0]
//    }
//}

#[derive(Copy, Clone)]
pub struct ParserContext {
    //modifiers: Modifiers,
}

impl ParserContext {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait Parser {
    fn parse(&self, code: &str, context: ParserContext) -> Option<Vec<Token>>;
}
