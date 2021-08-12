extern crate json;

use crate::i18n::gettext;
use crate::opt_list::get_settings_list;
use crate::path::get_exe_path;
use crate::path::path_to_str;
use json::JsonValue;
use std::clone::Clone;
use std::collections::HashMap;
use std::default::Default;
use std::fs::remove_file;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
pub enum JsonValueType {
    Str,
    Number,
    Boolean,
    Object,
    Array,
}

pub type SettingDesCallback = fn(JsonValue) -> bool;

pub struct SettingDes {
    _name: String,
    _description: String,
    _type: JsonValueType,
    _fun: Option<SettingDesCallback>,
}

impl SettingDes {
    pub fn new(
        name: &str,
        description: &str,
        typ: JsonValueType,
        callback: Option<SettingDesCallback>,
    ) -> Option<SettingDes> {
        if (typ == JsonValueType::Array || typ == JsonValueType::Object) && callback == None {
            return None;
        }
        Some(SettingDes {
            _name: String::from(name),
            _description: String::from(description),
            _type: typ.clone(),
            _fun: callback,
        })
    }

    pub fn name(&self) -> &str {
        self._name.as_str()
    }

    pub fn description(&self) -> &str {
        self._description.as_str()
    }

    pub fn typ(&self) -> JsonValueType {
        self._type.clone()
    }

    pub fn type_name(&self) -> &'static str {
        if self._type == JsonValueType::Array {
            return "Array";
        } else if self._type == JsonValueType::Boolean {
            return "Boolean";
        } else if self._type == JsonValueType::Number {
            return "Number";
        } else if self._type == JsonValueType::Object {
            return "Object";
        } else {
            return "String";
        }
    }

    pub fn is_vaild_value(&self, value: JsonValue) -> bool {
        if self._type == JsonValueType::Array {
            if value.is_array() {
                return self._fun.unwrap()(value);
            }
        } else if self._type == JsonValueType::Boolean {
            if value.is_boolean() {
                return true;
            }
        } else if self._type == JsonValueType::Number {
            if value.is_number() {
                match self._fun {
                    Some(fun) => {
                        return fun(value);
                    }
                    None => {
                        return true;
                    }
                }
            }
        } else if self._type == JsonValueType::Object {
            if value.is_object() {
                return self._fun.unwrap()(value);
            }
        } else if self._type == JsonValueType::Str {
            if value.is_string() {
                match self._fun {
                    Some(fun) => {
                        return fun(value);
                    }
                    None => {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

impl Clone for SettingDes {
    fn clone(&self) -> SettingDes {
        SettingDes {
            _name: self._name.clone(),
            _description: self._description.clone(),
            _type: self._type.clone(),
            _fun: self._fun.clone(),
        }
    }
}

pub struct SettingDesStore {
    list: Vec<SettingDes>,
}

impl SettingDesStore {
    pub fn new(list: Vec<SettingDes>) -> SettingDesStore {
        SettingDesStore { list }
    }

    pub fn check_valid(&self, key: &str, value: JsonValue) -> Option<bool> {
        for i in self.list.iter() {
            if i.name() == key {
                return Some(i.is_vaild_value(value));
            }
        }
        None
    }

    pub fn print_help(&self) {
        let mut s = String::from("");
        for i in self.list.iter() {
            let mut t = format!("{}: {}", i.name(), i.type_name());
            if t.len() >= 20 {
                t += "\t";
            } else {
                t += " ".repeat(20 - t.len()).as_str();
            }
            t += i.description();
            if s.len() > 0 {
                s += "\n";
            }
            s += t.as_str();
        }
        println!("{}", s);
    }
}

impl Clone for SettingDesStore {
    fn clone(&self) -> SettingDesStore {
        SettingDesStore {
            list: self.list.clone(),
        }
    }
}

impl Default for SettingDesStore {
    fn default() -> SettingDesStore {
        SettingDesStore {
            list: get_settings_list(),
        }
    }
}

pub struct SettingOpt {
    _name: String,
    _value: JsonValue,
}

impl SettingOpt {
    pub fn new(name: String, value: JsonValue) -> SettingOpt {
        SettingOpt {
            _name: name.clone(),
            _value: value.clone(),
        }
    }

    pub fn name(&self) -> String {
        self._name.clone()
    }

    pub fn value(&self) -> JsonValue {
        self._value.clone()
    }
}

impl Clone for SettingOpt {
    fn clone(&self) -> SettingOpt {
        SettingOpt {
            _name: self._name.clone(),
            _value: self._value.clone(),
        }
    }
}

pub struct SettingJar {
    pub settings: HashMap<String, SettingOpt>,
}

impl SettingJar {
    pub fn new() -> SettingJar {
        SettingJar {
            settings: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, opt: JsonValue) {
        self.settings
            .insert(String::from(key), SettingOpt::new(String::from(key), opt));
    }

    pub fn to_json(&self) -> Option<JsonValue> {
        let mut v = JsonValue::new_object();
        for (_, val) in self.settings.iter() {
            match v.insert(val.name().as_str(), val.value()) {
                Ok(_) => {}
                Err(_) => {
                    println!("{}", gettext("Can not insert setting to JSON object."));
                    return None;
                }
            }
        }
        Some(v)
    }
}

impl Clone for SettingJar {
    fn clone(&self) -> SettingJar {
        SettingJar {
            settings: self.settings.clone(),
        }
    }
}

pub struct SettingStore {
    pub basic: SettingDesStore,
    pub maps: HashMap<String, SettingJar>,
    pub des_map: HashMap<String, SettingDesStore>,
    pub des_dep: HashMap<String, String>,
}

impl SettingStore {
    pub fn new() -> SettingStore {
        SettingStore {
            basic: SettingDesStore::default(),
            maps: HashMap::new(),
            des_map: HashMap::new(),
            des_dep: HashMap::new(),
        }
    }

    pub fn check_valid(&self, store_key: &str, key_in_map: &str, value: JsonValue) -> Option<bool> {
        if store_key == "basic" {
            return self.basic.check_valid(key_in_map, value);
        } else if self.des_map.contains_key(store_key) {
            let i = self.des_map.get(store_key).unwrap();
            return i.check_valid(key_in_map, value);
        }
        None
    }

    pub fn read(&mut self, file_name: Option<String>, fix_invalid: bool) -> bool {
        self.maps.clear();
        match file_name {
            Some(f) => {
                let re = self.read_internal(Path::new(f.as_str()), fix_invalid);
                if !re {
                    println!(
                        "{}\"{}\"",
                        gettext("Can not load custom settings file: "),
                        f
                    );
                    self.maps.clear();
                }
                re
            }
            None => {
                let re = get_exe_path();
                match re {
                    Some(pb) => {
                        let mut tpb = pb;
                        tpb.push("bili.settings.json");
                        let r = self.read_internal(tpb.as_path(), fix_invalid);
                        if !r {
                            self.maps.clear();
                        }
                        r
                    }
                    None => false,
                }
            }
        }
    }

    fn read_internal(&mut self, path: &Path, fix_invalid: bool) -> bool {
        if !path.exists() {
            return false;
        }
        let re = File::open(path);
        match re {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "{}\"{}\"",
                    gettext("Can not open settings file: "),
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
                        gettext("Settings file is empty: "),
                        path_to_str(path)
                    );
                    return false;
                }
            }
            Err(_) => {
                println!(
                    "{}\"{}\"",
                    gettext("Can not read from settings file: "),
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
                    gettext("Can not parse settings file: "),
                    path_to_str(path)
                );
                return false;
            }
        }
        let obj = re.unwrap();
        if obj.is_object() == false {
            println!(
                "{}\"{}\"",
                gettext("Unknown settings file: "),
                path_to_str(path)
            );
            return false;
        }
        for (key, o) in obj.entries() {
            let mut jar = SettingJar::new();
            if !o.is_object() {
                let s = gettext("Key \"<key>\" in settings file is not a object.")
                    .replace("<key>", key);
                println!("{}", s);
                return false;
            }
            for (key2, o) in o.entries() {
                let re = self.check_valid(key, key2, o.clone());
                match re {
                    Some(re) => {
                        if !re {
                            if !fix_invalid {
                                let s = gettext("\"<key>\" is invalid, you can use \"bili config fix\" to remove all invalid value.").replace("<key>", format!("{}.{}", key, key2).as_str());
                                println!("{}", s);
                                return false;
                            }
                        } else {
                            jar.add(key2, o.clone());
                        }
                    }
                    None => {
                        jar.add(key2, o.clone());
                    }
                }
            }
            self.maps.insert(String::from(key), jar);
        }
        return true;
    }

    pub fn save(&self, file_name: Option<String>) -> bool {
        let s = self.to_str();
        match s {
            Some(_) => {}
            None => {
                return false;
            }
        }
        let s = s.unwrap();
        match file_name {
            Some(f) => {
                let re = self.save_internal(s, Path::new(f.as_str()));
                re
            }
            None => {
                let re = get_exe_path();
                match re {
                    Some(pb) => {
                        let mut tpb = pb;
                        tpb.push("bili.settings.json");
                        let r = self.save_internal(s, tpb.as_path());
                        r
                    }
                    None => false,
                }
            }
        }
    }

    fn save_internal(&self, s: String, path: &Path) -> bool {
        if path.exists() {
            match remove_file(path) {
                Ok(_) => {}
                Err(_) => {
                    println!(
                        "{}\"{}\"",
                        gettext("Can not remove file: "),
                        path_to_str(path)
                    );
                    return false;
                }
            }
        }
        let r = File::create(path);
        match r {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "{}\"{}\"",
                    gettext("Can not save to settings file: "),
                    path_to_str(path)
                );
                return false;
            }
        }
        let mut f = r.unwrap();
        match f.write(s.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "{}\"{}\"",
                    gettext("Can not write data to settings file: "),
                    path_to_str(path)
                );
                return false;
            }
        }
        match f.flush() {
            Ok(_) => {}
            Err(_) => {}
        }
        return true;
    }

    pub fn to_json(&self) -> Option<JsonValue> {
        let mut v = JsonValue::new_object();
        for (key, val) in self.maps.iter() {
            let obj = val.to_json();
            if obj.is_none() {
                return None;
            }
            let obj = obj.unwrap();
            match v.insert(key, obj) {
                Ok(_) => {}
                Err(_) => {
                    println!("{}", gettext("Can not insert settings jar to JSON object."));
                    return None;
                }
            }
        }
        Some(v)
    }

    pub fn to_str(&self) -> Option<String> {
        let obj = self.to_json();
        if !obj.is_none() {
            return None;
        }
        let obj = obj.unwrap();
        Some(obj.dump())
    }
}

impl Clone for SettingStore {
    fn clone(&self) -> SettingStore {
        SettingStore {
            basic: self.basic.clone(),
            maps: self.maps.clone(),
            des_map: self.des_map.clone(),
            des_dep: self.des_dep.clone(),
        }
    }
}
