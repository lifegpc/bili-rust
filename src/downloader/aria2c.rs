extern crate reqwest;
extern crate subprocess;

use crate::utils::convert::ToStr;
use crate::utils::number::ToUsize;
use crate::utils::path::filter_file_name;
use crate::utils::size::ToSize;
use core::time::Duration;
use reqwest::IntoUrl;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::Into;
use std::convert::TryFrom;
use subprocess::ExitStatus;
use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;

#[derive(Clone, Copy, Debug, PartialEq)]
/// Aria2c file allocation method
pub enum Aria2cFileAllocation {
    None,
    Prealloc,
    Trunc,
    Falloc,
}

impl Into<&'static str> for Aria2cFileAllocation {
    fn into(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Prealloc => "prealloc",
            Self::Trunc => "trunc",
            Self::Falloc => "falloc",
        }
    }
}

impl ToStr for Aria2cFileAllocation {
    fn to_str(&self) -> Option<&str> {
        Some(self.clone().into())
    }
}

impl TryFrom<&str> for Aria2cFileAllocation {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.to_lowercase();
        if v == "none" {
            Ok(Self::None)
        } else if v == "prealloc" {
            Ok(Self::Prealloc)
        } else if v == "trunc" {
            Ok(Self::Trunc)
        } else if v == "falloc" {
            Ok(Self::Falloc)
        } else {
            Err("Unknown type")
        }
    }
}

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
        Ok(_) => {}
        Err(_) => {}
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

/// Check aria2c settings.
/// * `inp` - Input object
pub fn check_file_allocation<U: ToStr>(inp: &U) -> bool {
    let i = inp.to_str();
    if i.is_none() {
        return false;
    }
    match Aria2cFileAllocation::try_from(i.unwrap()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Check aria2c settings.
/// * `inp` - Input object
pub fn check_max_connection_per_server<U: ToUsize>(inp: &U) -> bool {
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
    /// Aria2c settings: file allocation method
    file_allocation: Aria2cFileAllocation,
    /// Aria2c settings: the maximum number of connections to one server for each download
    max_connection_per_server: usize,
    /// Output file name
    output: Option<String>,
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
            file_allocation: Aria2cFileAllocation::Prealloc,
            max_connection_per_server: 1,
            output: None,
        })
    }

    /// Perfrom download
    pub fn download<U: IntoUrl>(&mut self, url: &U) -> Option<i32> {
        let mut li = vec![self.exe.clone()];
        for (k, v) in &self.headers {
            let t = format!("--header={}: {}", k, v);
            li.push(t);
        }
        let t = format!("{}", self.min_split_size);
        li.push(String::from("-k"));
        li.push(t);
        let t = format!("{}", self.split);
        li.push(String::from("-s"));
        li.push(t);
        let t = format!("--file-allocation={}", self.file_allocation.to_str().unwrap());
        li.push(t);
        let t = format!("{}", self.max_connection_per_server);
        li.push(String::from("-x"));
        li.push(t);
        if self.output.is_some() {
            li.push(String::from("-o"));
            li.push(self.output.as_ref().unwrap().clone());
        }
        li.push(String::from(url.as_str()));
        li.push(String::from("--auto-file-renaming"));
        li.push(String::from("false"));
        println!("{:?}", &li);
        let r = Popen::create(&li, PopenConfig::default());
        match r {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                return None;
            }
        }
        let mut p = r.unwrap();
        let r = p.wait();
        match r {
            Ok(e) => {
                match e {
                    ExitStatus::Exited(s) => {
                        Some(s as i32)
                    }
                    _ => {
                        None
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    /// Set settings.
    /// * `inp` - Input object
    pub fn set_file_allocation<U: ToStr>(&mut self, inp: &U) -> bool {
        let s = inp.to_str();
        if s.is_none() {
            return false;
        }
        let r = Aria2cFileAllocation::try_from(s.unwrap());
        match r {
            Ok(r) => {
                self.file_allocation = r;
                true
            }
            Err(_) => false,
        }
    }

    /// Set settings.
    /// * `inp` - Input object
    pub fn set_max_connection_per_server<U: ToUsize>(&mut self, inp: &U) -> bool {
        let s = inp.to_usize();
        if s.is_none() {
            return false;
        }
        let s = s.unwrap();
        if s >= 1 {
            self.max_connection_per_server = s;
            true
        } else {
            false
        }
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

    pub fn set_output<U: ToStr>(&mut self, inp: Option<&U>) -> bool {
        if inp.is_none() {
            self.output = None;
            true
        } else {
            let s = inp.unwrap().to_str();
            if s.is_none() {
                false
            } else {
                let f = filter_file_name(&s.unwrap());
                if f.is_some() {
                    self.output = f;
                    true
                } else {
                    false
                }
            }
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
            file_allocation: self.file_allocation.clone(),
            max_connection_per_server: self.max_connection_per_server.clone(),
            output: self.output.clone(),
        }
    }
}
