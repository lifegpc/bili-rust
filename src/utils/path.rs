extern crate regex;

use crate::utils::convert::ToStr;
use regex::Regex;
use std::env;
use std::path::Path;
use std::path::PathBuf;

lazy_static! {
    static ref RE: Regex = Regex::new(r"[[:cntrl:]]").unwrap();
}

/// Get executable location, if not found, return None
pub fn get_exe_path() -> Option<PathBuf> {
    let re = env::current_exe();
    match re {
        Ok(pa) => {
            let mut p = pa.clone();
            p.pop();
            Some(p)
        }
        Err(_) => None,
    }
}

/// Get executable location, if not found, return current directory (./)
pub fn get_exe_path_else_current() -> PathBuf {
    let re = env::current_exe();
    match re {
        Ok(pa) => {
            let mut p = pa.clone();
            p.pop();
            p
        }
        Err(_) => {
            let p = Path::new("./");
            p.to_path_buf()
        }
    }
}

/// Convert `&Path` to `&str`
/// * `p` - Origin path
pub fn path_to_str(p: &Path) -> &str {
    let f = p.to_str();
    match f {
        Some(n) => n,
        None => "<Convert Error>",
    }
}

pub fn filter_file_name<U: ToStr>(f: &U) -> Option<String> {
    let s = f.to_str();
    if s.is_none() {
        return None;
    }
    let s = s.unwrap();
    let s = RE
        .replace_all(s, "_")
        .into_owned()
        .replace("\\", "_")
        .replace("/", "_")
        .replace(":", "_")
        .replace("*", "_")
        .replace("?", "_")
        .replace("\"", "_")
        .replace("<", "_")
        .replace(">", "_")
        .replace("|", "_");
    Some(s)
}

#[test]
fn test_filter_file_name() {
    assert_eq!(Some(String::from("T_NM_WTF_A_❤测试")), filter_file_name(&"T\tNM\rWTF?A<❤测试"));
}
