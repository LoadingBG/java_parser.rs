use regex::Regex;

fn main() {
    let r = Regex::new(r"^([\+-]\s*)?(0[bB])([01](_?[01])*)\b").unwrap();
    println!("{:?}", r.captures("+0b1"));
}
