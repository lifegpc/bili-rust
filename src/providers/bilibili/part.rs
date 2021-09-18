extern crate json;
extern crate regex;

use crate::providers::bilibili::util;
use json::JsonValue;
use regex::Regex;
use std::clone::Clone;
use std::convert::From;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^ *(?P<start>\d+)? *((?P<concat>-) *(?P<end>\d+)?)? *$").unwrap();
}

/// A range of part
/// if start/end is 0, it means the minimum/maximum part number
#[derive(Debug, PartialEq)]
pub struct Part {
    pub start: usize,
    pub end: usize,
}

impl Part {
    /// Create a new one
    /// * `start` - start part number
    /// * `end` - end part number
    /// # Notes
    /// If start is bigger than end, the function will return None.
    pub fn new(start: usize, end: usize) -> Option<Part> {
        if start != 0 && end != 0 {
            if start > end {
                return None;
            }
        }
        Some(Part { start, end })
    }

    /// Parse part from str
    /// * `s` - part. such as `1-`, `1-3`, `-4`, `1`, `-`
    pub fn parse_from_str(s: &str) -> Option<Part> {
        let re = RE.captures(s);
        if re.is_none() {
            return None;
        }
        let re = re.unwrap();
        let s = re.name("start");
        let c = re.name("concat");
        let e = re.name("end");
        if !s.is_none() {
            let st = util::atou(s.unwrap().as_str());
            if st.is_none() {
                return None;
            }
            let st = st.unwrap();
            if c.is_none() {
                // 1
                return Part::new(st, st);
            } else {
                if e.is_none() {
                    // 1-
                    return Part::new(st, 0);
                } else {
                    // 1-3
                    let ed = util::atou(e.unwrap().as_str());
                    if ed.is_none() {
                        return None;
                    }
                    return Part::new(st, ed.unwrap());
                }
            }
        } else {
            if !c.is_none() {
                if e.is_none() {
                    // -
                    return Part::new(0, 0);
                } else {
                    // -3
                    let ed = util::atou(e.unwrap().as_str());
                    if ed.is_none() {
                        return None;
                    }
                    return Part::new(0, ed.unwrap());
                }
            }
        }
        None
    }
}

impl Clone for Part {
    fn clone(&self) -> Part {
        Part {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

/// Part number list
#[derive(Debug, PartialEq)]
pub struct PartList {
    list: Vec<Part>,
}

impl PartList {
    /// Create a blank list
    pub fn new() -> PartList {
        PartList { list: [].to_vec() }
    }

    pub fn parse_from_json(v: &JsonValue) -> Option<PartList> {
        let mut pi = PartList::new();
        if v.is_string() {
            return PartList::parse_from_str(v.as_str().unwrap());
        } else if v.is_number() {
            let n = v.as_usize();
            if n.is_none() {
                return None;
            }
            let n = n.unwrap();
            let r = Part::new(n, n);
            if r.is_none() {
                return None;
            }
            pi.list.push(r.unwrap());
        } else if v.is_array() {
            for val in v.members() {
                if val.is_number() {
                    let n = val.as_usize();
                    if n.is_none() {
                        return None;
                    }
                    let n = n.unwrap();
                    let r = Part::new(n, n);
                    if r.is_none() {
                        return None;
                    }
                    pi.list.push(r.unwrap());
                } else if val.is_string() {
                    let r = Part::parse_from_str(val.as_str().unwrap());
                    if r.is_none() {
                        return None;
                    }
                    pi.list.push(r.unwrap());
                }
            }
        }
        if pi.list.len() > 0 {
            return Some(pi);
        }
        None
    }

    /// Parse part number list from string
    /// * `s` - input string. such as `1-34,36`
    pub fn parse_from_str(s: &str) -> Option<PartList> {
        let mut pi = PartList::new();
        let li = s.split(",");
        for i in li {
            let r = Part::parse_from_str(i);
            if r.is_none() {
                return None;
            }
            pi.list.push(r.unwrap());
        }
        if pi.list.len() > 0 {
            return Some(pi);
        }
        None
    }
}

impl Clone for PartList {
    fn clone(&self) -> PartList {
        PartList {
            list: self.list.clone(),
        }
    }
}

impl From<Vec<Part>> for PartList {
    fn from(list: Vec<Part>) -> PartList {
        PartList { list: list.clone() }
    }
}

#[test]
fn test_part_parse_from_str() {
    assert_eq!(Part::new(0, 0), Part::parse_from_str(" - "));
    assert_eq!(None, Part::parse_from_str("34-2"));
    assert_eq!(Part::new(0, 30), Part::parse_from_str(" -30"));
    assert_eq!(Part::new(12, 23), Part::parse_from_str(" 12 - 23 "));
    assert_eq!(Part::new(13, 0), Part::parse_from_str("13-"));
    assert_eq!(Part::new(33, 33), Part::parse_from_str("33"));
}

#[test]
fn test_part_list_parse_from_str() {
    assert_eq!(
        Some(PartList::from(vec![
            Part::new(1, 3).unwrap(),
            Part::new(5, 11).unwrap()
        ])),
        PartList::parse_from_str(" 1 - 3, 5-11")
    );
    assert_eq!(None, PartList::parse_from_str("3,,4"));
    assert_eq!(
        Some(PartList::from(vec![
            Part::new(1, 34).unwrap(),
            Part::new(37, 37).unwrap()
        ])),
        PartList::parse_from_str("1-34, 37")
    );
}

#[test]
fn test_part_list_parse_from_json() {
    assert_eq!(
        Some(PartList::from(vec![
            Part::new(1, 33).unwrap(),
            Part::new(38, 38).unwrap()
        ])),
        PartList::parse_from_json(&json::parse("\"1-33, 38\"").unwrap())
    );
    assert_eq!(
        Some(PartList::from(vec![Part::new(4, 4).unwrap()])),
        PartList::parse_from_json(&json::parse("4").unwrap())
    );
    assert_eq!(
        Some(PartList::from(vec![
            Part::new(3, 3).unwrap(),
            Part::new(5, 0).unwrap()
        ])),
        PartList::parse_from_json(&json::parse("[3, \"5-\"]").unwrap())
    )
}
