extern crate json;
extern crate thirtyfour;

use crate::i18n::gettext;
use crate::path::get_exe_path;
use crate::path::path_to_str;
use json::object;
use json::JsonValue;
use std::clone::Clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Cookie {
    _name: String,
    _value: String,
    _domain: Option<String>,
    _path: Option<String>,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Cookie {
        Cookie {
            _name: String::from(name),
            _value: String::from(value),
            _domain: None,
            _path: None,
        }
    }

    pub fn name(&self) -> &str {
        self._name.as_str()
    }

    pub fn value(&self) -> &str {
        self._value.as_str()
    }

    pub fn domain(&self) -> Option<&str> {
        match &self._domain {
            Some(dm) => Some(dm.as_str()),
            None => None,
        }
    }

    pub fn set_domain(&mut self, domain: Option<&str>) {
        match domain {
            Some(dm) => {
                self._domain = Some(String::from(dm));
            }
            None => {
                self._domain = None;
            }
        }
    }

    pub fn path(&self) -> Option<&str> {
        match &self._path {
            Some(ph) => Some(ph.as_str()),
            None => None,
        }
    }

    pub fn set_path(&mut self, path: Option<&str>) {
        match path {
            Some(p) => {
                self._path = Some(String::from(p));
            }
            None => {
                self._path = None;
            }
        }
    }

    pub fn to_json(&self) -> Option<JsonValue> {
        let mut obj = object! {
            "name": self._name.as_str(),
            "value": self._value.as_str(),
        };
        match &self._domain {
            Some(dm) => match obj.insert("domain", dm.as_str()) {
                Ok(_) => {}
                Err(_) => {
                    println!(
                        "{}",
                        gettext("Can not insert domain to cookie's json object.")
                    );
                    return None;
                }
            },
            None => {}
        }
        match &self._path {
            Some(p) => match obj.insert("path", p.as_str()) {
                Ok(_) => {}
                Err(_) => {
                    println!(
                        "{}",
                        gettext("Can not insert path to cookie's json object.")
                    );
                    return None;
                }
            },
            None => {}
        }
        return Some(obj);
    }
}

impl Clone for Cookie {
    fn clone(&self) -> Cookie {
        Cookie {
            _name: self._name.clone(),
            _value: self._value.clone(),
            _domain: self._domain.clone(),
            _path: self._path.clone(),
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

    pub fn add(&mut self, c: Cookie) {
        let n = c.name();
        self.cookies.insert(String::from(n), c);
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Cookie> {
        self.cookies.iter()
    }

    pub fn to_json(&self) -> Option<JsonValue> {
        let mut arr = JsonValue::new_array();
        for (_, val) in self.cookies.iter() {
            let obj = val.to_json();
            match obj {
                Some(obj) => match arr.push(obj) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("{}", gettext("Can not append a cookie to cookies jar."));
                        return None;
                    }
                },
                None => {
                    return None;
                }
            }
        }
        Some(arr)
    }
}

impl Clone for CookiesJar {
    fn clone(&self) -> CookiesJar {
        CookiesJar {
            cookies: self.cookies.clone(),
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

    pub fn add(&mut self, key: &str, jar: CookiesJar) {
        self.cookies.insert(String::from(key), jar);
    }

    pub fn get(&self, key: &str) -> Option<&CookiesJar> {
        self.cookies.get(key)
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
                        tpb.push("bili.cookies.json");
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

    fn read_internal(&mut self, path: &Path) -> bool {
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
            if !e.1.is_array() {
                println!(
                    "{}\"{}\"",
                    gettext("Unknown cookies file: "),
                    path_to_str(path)
                );
                return false;
            }
            let key = e.0;
            if key.len() == 0 {
                println!(
                    "{}\"{}\"",
                    gettext("The provider name shoule not be empty in cookies file: "),
                    path_to_str(path)
                );
                return false;
            }
            let mut jar = CookiesJar::new();
            let mut it = e.1.members();
            let mut tco = it.next();
            while !tco.is_none() {
                let co = tco.unwrap();
                if !co.is_object() {
                    println!(
                        "{}\"{}\"",
                        gettext("Unknown cookies file: "),
                        path_to_str(path)
                    );
                    return false;
                }
                if !co.has_key("name") || !co.has_key("value") {
                    println!(
                        "{}\"{}\"",
                        gettext("Cookie must have name and value: "),
                        path_to_str(path)
                    );
                    return false;
                }
                let name = &co["name"];
                let value = &co["value"];
                if !name.is_string() || !value.is_string() {
                    println!(
                        "{}\"{}\"",
                        gettext("Cookie's name or value is non-string: "),
                        path_to_str(path)
                    );
                    return false;
                }
                let r = name.as_str();
                match r {
                    Some(_) => {}
                    None => {
                        println!(
                            "{}\"{}\"",
                            gettext("Cookie's name or value is non-string: "),
                            path_to_str(path)
                        );
                        return false;
                    }
                }
                let name = r.unwrap();
                let r = value.as_str();
                match r {
                    Some(_) => {}
                    None => {
                        println!(
                            "{}\"{}\"",
                            gettext("Cookie's name or value is non-string: "),
                            path_to_str(path)
                        );
                        return false;
                    }
                }
                let value = r.unwrap();
                let mut c = Cookie::new(name, value);
                if co.has_key("domain") {
                    let dm = &co["domain"];
                    if !dm.is_string() {
                        println!(
                            "{}\"{}\"",
                            gettext("Cookie's domain is non-string: "),
                            path_to_str(path)
                        );
                        return false;
                    }
                    let r = dm.as_str();
                    match r {
                        Some(_) => {}
                        None => {
                            println!(
                                "{}\"{}\"",
                                gettext("Cookie's domain is non-string: "),
                                path_to_str(path)
                            );
                            return false;
                        }
                    }
                    let dm = r.unwrap();
                    c.set_domain(Some(dm));
                }
                if co.has_key("path") {
                    let ph = &co["path"];
                    if !ph.is_string() {
                        println!(
                            "{}\"{}\"",
                            gettext("Cookie's path is non-string: "),
                            path_to_str(path)
                        );
                        return false;
                    }
                    let r = ph.as_str();
                    match r {
                        Some(_) => {}
                        None => {
                            println!(
                                "{}\"{}\"",
                                gettext("Cookie's path is non-string: "),
                                path_to_str(path)
                            );
                            return false;
                        }
                    }
                    let ph = r.unwrap();
                    c.set_path(Some(ph));
                }
                jar.add(c);
                tco = it.next();
            }
            self.add(key, jar);
            en = ent.next();
        }
        return true;
    }
}

impl Clone for CookiesJson {
    fn clone(&self) -> CookiesJson {
        CookiesJson {
            cookies: self.cookies.clone(),
        }
    }
}
