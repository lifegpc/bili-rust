extern crate json;
extern crate thirtyfour;

use crate::i18n::gettext;
use crate::path::get_exe_path;
use crate::path::path_to_str;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Cookie {
    name: String,
    value: String,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Cookie {
        Cookie {
            name: String::from(name),
            value: String::from(value),
        }
    }
}

pub struct CookiesJar {
    pub cookies: HashMap<String, Cookie>,
}

impl CookiesJar {
    pub fn new() -> CookiesJar {
        CookiesJar {
            cookies: HashMap::new(),
        }
    }
}

pub struct CookiesJson {
    pub cookies: HashMap<String, CookiesJar>,
}

impl CookiesJson {
    pub fn new() -> CookiesJson {
        CookiesJson {
            cookies: HashMap::new(),
        }
    }

    pub fn read(&mut self, file_name: Option<&str>) -> bool {
        self.cookies.clear();
        match file_name {
            Some(f) => {
                let re = self.read_internal(Path::new(f));
                if !re {
                    println!("{}\"{}\"", gettext("Can not load custom cookies file: "), f);
                    self.cookies.clear();
                }
                re
            }
            None => {
                let re = get_exe_path();
                match re {
                    Some(pb) => {
                        let mut tpb = pb;
                        tpb.push("cookies.json");
                        let r = self.read_internal(tpb.as_path());
                        if !r {
                            self.cookies.clear();
                        }
                        r
                    }
                    None => false,
                }
            }
        }
    }

    fn read_internal(&self, path: &Path) -> bool {
        if !path.exists() {
            return false;
        }
        let re = File::open(path);
        match re {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "{}\"{}\"",
                    gettext("Can not open cookies file: "),
                    path_to_str(path)
                );
                return false;
            }
        }
        let mut f = re.unwrap();
        let mut s = String::from("");
        let r = f.read_to_string(&mut s);
        match r {
            Ok(le) => {
                if le == 0 {
                    println!(
                        "{}\"{}\"",
                        gettext("Cookies file is empty: "),
                        path_to_str(path)
                    );
                    return false;
                }
            }
            Err(_) => {
                println!(
                    "{}\"{}\"",
                    gettext("Can not read from cookies file: "),
                    path_to_str(path)
                );
                return false;
            }
        }
        let re = json::parse(s.as_str());
        match re {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "{}\"{}\"",
                    gettext("Can not parse cookies file: "),
                    path_to_str(path)
                );
                return false;
            }
        }
        let obj = re.unwrap();
        if obj.is_object() == false {
            println!(
                "{}\"{}\"",
                gettext("Unknown cookies file: "),
                path_to_str(path)
            );
            return false;
        }
        let mut ent = obj.entries();
        let mut en = ent.next();
        while !en.is_none() {
            let e = en.unwrap();
            if self.cookies.contains_key(e.0) {
                println!(
                    "{}\"{}\"",
                    gettext("Cookies file contains two same keys: "),
                    path_to_str(path)
                );
                return false;
            }
            if !e.1.is_object() {
                println!(
                    "{}\"{}\"",
                    gettext("Unknown cookies file: "),
                    path_to_str(path)
                );
                return false;
            }
            if !e.1.has_key("name") || !e.1.has_key("value") {
                println!(
                    "{}\"{}\"",
                    gettext("Cookie must have name and value: "),
                    path_to_str(path)
                );
                return false;
            }
            let name = &e.1["name"];
            let value = &e.1["value"];
            if !name.is_string() || !value.is_string() {
                println!(
                    "{}\"{}\"",
                    gettext("Cookie's Name or value is non-string : "),
                    path_to_str(path)
                );
                return false;
            }
            let r = name.as_str();
            let r2 = value.as_str();
            if r.is_none() || r2.is_none() {
                println!(
                    "{}\"{}\"",
                    gettext("Cookie's Name or value is non-string : "),
                    path_to_str(path)
                );
                return false;
            }
            let name = r.unwrap();
            let value = r2.unwrap();
            let c = Cookie::new(name, value);
            let keys = ["path", "domain", "secure"];
            let mut it = keys.iter();
            let mut ke = it.next();
            while !ke.is_none() {
                let key = *(ke.unwrap());
                if e.1.has_key(key) {
                    let v = &e.1[key];
                    if v.is_string() {
                        if key == "path" {}
                    }
                }
                ke = it.next();
            }
            en = ent.next();
        }
        return false;
    }
}
