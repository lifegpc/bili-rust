extern crate subprocess;

use crate::utils::number::ToUsize;
use crate::utils::size::ToSize;
use core::time::Duration;
use std::clone::Clone;
use std::collections::HashMap;
use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;

/// Test aria2c whether to works
/// * `p` - The path of aria2c
pub fn test_aria2c(p: &str) -> bool {
    let l = vec![p, "-h"];
    let r = Popen::create(
        &l,
        PopenConfig {
            stdin: Redirection::Pipe,
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..PopenConfig::default()
        },
    );
    match r {
        Ok(_) => {}
        Err(_) => {
            return false;
        }
    }
    let mut r = r.unwrap();
    match r.communicate(Some("")) {
        Ok(_) => {},
        Err(_) => {},
    }
    let re = r.wait_timeout(Duration::new(5, 0));
    match re {
        Ok(_) => {}
        Err(_) => {
            return false;
        }
    }
    let re = re.unwrap();
    if re.is_none() {
        match r.kill() {
            Ok(_) => {}
            Err(_) => {}
        }
        return false;
    }
    let re = re.unwrap();
    if re.success() {
        return true;
    }
    false
}

/// Check aria2c settings.
/// * `inp` - Input object
pub fn check_min_split_size(inp: &impl ToSize) -> bool {
    let r = inp.to_size();
    if r.is_none() {
        return false;
    }
    let r = r.unwrap();
    if r >= 1048576 && r <= 1073741824 {
        true
    } else {
        false
    }
}

/// Check aria2c settings.
/// * `inp` - Input object
pub fn check_split<U: ToUsize>(inp: &U) -> bool {
    let r = inp.to_usize();
    if r.is_none() {
        return false;
    }
    let r = r.unwrap();
    if r >= 1 {
        true
    } else {
        false
    }
}

/// Aria2c interface
pub struct Aria2c {
    /// Executable path
    exe: String,
    /// HTTP Headers
    pub headers: HashMap<String, String>,
    /// Aria2c settings: aria2 does not split less than 2*SIZE byte range.
    min_split_size: usize,
    /// Aria2c settings: the number of connections used when downloading a file.
    split: usize,
}

impl Aria2c {
    /// Create a new interface
    /// * `exe` - The path of executable
    pub fn new(exe: Option<&str>) -> Option<Self> {
        let e = if exe.is_none() {
            "aria2c"
        } else {
            exe.unwrap()
        };
        if !test_aria2c(e) {
            return None;
        }
        Some(Self {
            exe: String::from(e),
            headers: HashMap::new(),
            min_split_size: 20971520,
            split: 5,
        })
    }

    /// Set settings.
    /// * `inp` - Input object
    pub fn set_min_split_size(&mut self, inp: &impl ToSize) -> bool {
        let s = inp.to_size();
        if s.is_none() {
            return false;
        }
        let s = s.unwrap();
        if s >= 1048576 && s <= 1073741824 {
            self.min_split_size = s;
            true
        } else {
            false
        }
    }

    /// Set settings.
    /// * `inp` - Input object
    pub fn set_split<U: ToUsize>(&mut self, inp: &U) -> bool {
        let s = inp.to_usize();
        if s.is_none() {
            return false;
        }
        let s = s.unwrap();
        if s >= 1 {
            self.split = s;
            true
        } else {
            false
        }
    }
}

impl Clone for Aria2c {
    fn clone(&self) -> Self {
        Self {
            exe: self.exe.clone(),
            headers: self.headers.clone(),
            min_split_size: self.min_split_size.clone(),
            split: self.split.clone(),
        }
    }
}
