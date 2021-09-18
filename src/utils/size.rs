extern crate json;
extern crate regex;

use json::JsonValue;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)^(\d+)([kmgtpezy]i?)?b?$").unwrap();
}

/// Convert multiple-byte units to bytes
/// * `s` - Input string
pub fn parse_size(s: &str) -> Option<usize> {
    let r = RE.captures(s);
    if r.is_some() {
        let r = r.unwrap();
        let f = r.get(1);
        if f.is_none() {
            return None;
        }
        let b = f.as_ref().unwrap().as_str().parse::<usize>();
        if b.is_err() {
            return None;
        }
        let b = b.unwrap();
        let p = r.get(2);
        if p.is_none() {
            return Some(b);
        }
        let p = p.unwrap().as_str();
        let po: usize = if p.len() == 1 {
            1000
        } else {
            1024
        };
        let c = p.chars().next().unwrap().to_ascii_lowercase();
        let pow = if c == 'k' {
            1
        } else if c == 'm' {
            2
        } else if c == 'g' {
            3
        } else if c == 't' {
            4
        } else if c == 'p' {
            5
        } else if c == 'e' {
            6
        } else if c == 'z' {
            7
        } else {
            8
        };
        return Some(b * po.pow(pow));
    }
    None
}

pub trait ToSize {
    /// Convert multiple-byte units to bytes
    fn to_size(&self) -> Option<usize>;
}

impl ToSize for String {
    fn to_size(&self) -> Option<usize> {
        parse_size(self.as_str())
    }
}

impl ToSize for str {
    fn to_size(&self) -> Option<usize> {
        parse_size(&self)
    }
}

impl ToSize for usize {
    fn to_size(&self) -> Option<usize> {
        Some(self.clone())
    }
}

impl ToSize for JsonValue {
    fn to_size(&self) -> Option<usize> {
        if self.is_number() {
            self.as_usize()
        } else if self.is_string() {
            parse_size(self.as_str().unwrap())
        } else {
            None
        }
    }
}

#[test]
fn test_parse_size() {
    assert_eq!(Some(123), parse_size("123"));
    assert_eq!(Some(345), parse_size("345B"));
    assert_eq!(Some(1000), parse_size("1K"));
    assert_eq!(Some(2048), parse_size("2KiB"));
}
