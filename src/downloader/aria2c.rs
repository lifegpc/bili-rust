extern crate subprocess;

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

/// Aria2c interface
pub struct Aria2c {
    /// Executable path
    exe: String,
    /// HTTP Headers
    pub headers: HashMap<String, String>,
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
        })
    }
}

impl Clone for Aria2c {
    fn clone(&self) -> Self {
        Self {
            exe: self.exe.clone(),
            headers: self.headers.clone(),
        }
    }
}
