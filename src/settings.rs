extern crate json;

use crate::i18n::gettext;
use crate::opt_list::get_settings_list;
use crate::utils::path::get_exe_path;
use crate::utils::path::path_to_str;
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
    Multiple,
}

pub type SettingDesCallback = fn(obj: &JsonValue) -> bool;

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
        if (typ == JsonValueType::Array
            || typ == JsonValueType::Object
            || typ == JsonValueType::Multiple)
            && callback.is_none()
        {
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

    pub fn type_name(&self) -> &'static str {
        if self._type == JsonValueType::Array {
            return "Array";
        } else if self._type == JsonValueType::Boolean {
            return "Boolean";
        } else if self._type == JsonValueType::Multiple {
            return gettext("Multiple type");
        } else if self._type == JsonValueType::Number {
            return "Number";
        } else if self._type == JsonValueType::Object {
            return "Object";
        } else {
            return "String";
        }
    }

    pub fn is_vaild_value(&self, value: &JsonValue) -> bool {
        if self._type == JsonValueType::Array {
            if value.is_array() {
                return self._fun.unwrap()(&value);
            }
        } else if self._type == JsonValueType::Boolean {
            if value.is_boolean() {
                return true;
            }
        } else if self._type == JsonValueType::Multiple {
            return self._fun.unwrap()(&value);
        } else if self._type == JsonValueType::Number {
            if value.is_number() {
                match self._fun {
                    Some(fun) => {
                        return fun(&value);
                    }
                    None => {
                        return true;
                    }
                }
            }
        } else if self._type == JsonValueType::Object {
            if value.is_object() {
                return self._fun.unwrap()(&value);
            }
        } else if self._type == JsonValueType::Str {
            if value.is_string() {
                match self._fun {
                    Some(fun) => {
                        return fun(&value);
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
                return Some(i.is_vaild_value(&value));
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.list.len()
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

    pub fn get(&self, key: &str) -> Option<JsonValue> {
        if self.settings.contains_key(key) {
            return Some(self.settings.get(key).unwrap().value());
        }
        None
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
    pub des_dep: HashMap<String, Vec<String>>,
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

    pub fn add(&mut self, des_key: &str, list: Vec<SettingDes>) {
        self.des_map
            .insert(String::from(des_key), SettingDesStore::new(list));
    }

    pub fn add_value(&mut self, map_key: &str, key: &str, value: &str, force: bool) -> bool {
        let obj = json::parse(value);
        match obj {
            Ok(_) => {}
            Err(_) => {
                let s =
                    gettext("\"<value>\" is not a vaild JSON object.").replace("<value>", value);
                println!("{}", s);
                return false;
            }
        }
        let obj = obj.unwrap();
        if map_key == "basic" || self.des_map.contains_key(map_key) {
            let des = if map_key == "basic" {
                &self.basic
            } else {
                self.des_map.get(map_key).unwrap()
            };
            let re = des.check_valid(key, obj.clone());
            if re.is_none() {
                let t = if map_key == "basic" {
                    String::from("bili --help-settings")
                } else {
                    format!("bili --help-settings {}", map_key)
                };
                let s = gettext("Unknown key.\nPlease use \"<command>\" to see available key.")
                    .replace("<command>", t.as_str());
                println!("{}", s);
                return false;
            }
            let re = re.unwrap();
            if !re {
                let t = if map_key == "basic" {
                    String::from("bili --help-settings")
                } else {
                    format!("bili --help-settings {}", map_key)
                };
                let s = gettext("Invalid value.\nPlease use \"<cmd>\" to see more information.")
                    .replace("<cmd>", t.as_str());
                println!("{}", s);
                return false;
            }
            if !self.maps.contains_key(map_key) {
                self.maps.insert(String::from(map_key), SettingJar::new());
            }
            let jar = self.maps.get_mut(map_key);
            let jar = jar.unwrap();
            if jar.settings.contains_key(key) && !force {
                println!(
                    "{}",
                    gettext("Already have this key in settings, please use \"<command>\".")
                        .replace("<command>", "bili config set <provider> <key> <value>")
                );
                return false;
            }
            jar.add(key, obj.clone());
            return true;
        }
        println!("{}", gettext("Unknown provider name.\nPlease use \"<command>\" to see all available name.\nNOTE: you can always use \"basic\".").replace("<command>", "bili --help-settings --list-providers-only"));
        return false;
    }

    pub fn add_with_dependence(
        &mut self,
        des_key: &str,
        list: Vec<SettingDes>,
        deps: Vec<&'static str>,
    ) -> Option<bool> {
        let mut ndeps: Vec<String> = Vec::new();
        for dep in deps.iter() {
            if !self.des_map.contains_key(*dep) {
                return None;
            }
            ndeps.push(String::from(*dep));
        }
        let des = SettingDesStore::new(list);
        self.des_map.insert(String::from(des_key), des);
        self.des_dep.insert(String::from(des_key), ndeps);
        Some(true)
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

    pub fn get_des_dependence(&self, key: &str) -> Option<Vec<String>> {
        if self.des_dep.contains_key(key) {
            let mut list: Vec<String> = Vec::new();
            self.get_des_dependence_internal(key, &mut list);
            return Some(list);
        }
        None
    }

    fn get_des_dependence_internal(&self, key: &str, list: &mut Vec<String>) {
        if self.des_dep.contains_key(key) {
            let l = self.des_dep.get(key).unwrap();
            for i in l.iter() {
                if !list.contains(i) {
                    list.push(i.clone());
                    self.get_des_dependence_internal(i.as_str(), list);
                }
            }
        }
    }

    pub fn get_settings(&self, map_key: &str, key: &str) -> Option<JsonValue> {
        if self.maps.contains_key(map_key) {
            let map = self.maps.get(map_key).unwrap();
            return map.get(key);
        }
        None
    }

    pub fn get_settings_as_bool(&self, map_key: &str, key: &str) -> Option<bool> {
        let re = self.get_settings(map_key, key);
        if !re.is_none() {
            let re = re.unwrap();
            return re.as_bool();
        }
        None
    }

    pub fn print_help(&self, detail: Option<String>, help_deps: bool) {
        println!("{}", gettext("Format: Key: Type Description"));
        if detail.is_none() || detail.clone().unwrap() == "full" {
            println!("{}", gettext("Basic settings:"));
            self.basic.print_help();
        }
        for (name, val) in self.des_map.iter() {
            if detail.is_none() {
                let s = gettext("<provider> provide <num> settings, use --help-settings full or --help-settings <provider> to see details.").replace("<provider>", name).replace("<num>", format!("{}", val.len()).as_str());
                println!("{}", s);
            } else {
                let d = detail.clone().unwrap();
                if d == "full" || &d == name {
                    let s =
                        gettext("Settings provided from <provider>: ").replace("<provider>", name);
                    println!("{}", s);
                    val.print_help();
                    if d != "full" {
                        let deps = self.get_des_dependence(name);
                        match deps {
                            Some(deps) => {
                                for dep in deps.iter() {
                                    let dd = self.des_map.get(dep).unwrap();
                                    if !help_deps {
                                        let s = gettext("<provider> provider <num> settings for <provider2>, add --help-deps to see.").replace("<provider>", dep.as_str()).replace("<num>", format!("{}", dd.len()).as_str()).replace("<provider2>", name);
                                        println!("{}", s);
                                    } else {
                                        let s = gettext("Settings provided from <provider>: ")
                                            .replace("<provider>", dep);
                                        println!("{}", s);
                                        dd.print_help();
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }

    pub fn print_providers(&self) {
        println!("{}", gettext("All available providers:"));
        for (name, _) in self.des_map.iter() {
            println!("{}", name);
        }
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
                    if !fix_invalid {
                        println!(
                            "{}\"{}\"",
                            gettext("Settings file is empty: "),
                            path_to_str(path)
                        );
                        return false;
                    }
                    return true;
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
                if !fix_invalid {
                    println!(
                        "{}\"{}\"",
                        gettext("Can not parse settings file: "),
                        path_to_str(path)
                    );
                    return false;
                }
                return true;
            }
        }
        let obj = re.unwrap();
        if obj.is_object() == false {
            if !fix_invalid {
                println!(
                    "{}\"{}\"",
                    gettext("Unknown settings file: "),
                    path_to_str(path)
                );
                return false;
            }
            return true;
        }
        for (key, o) in obj.entries() {
            let mut jar = SettingJar::new();
            if !o.is_object() {
                if !fix_invalid {
                    let s = gettext("Key \"<key>\" in settings file is not a object.")
                        .replace("<key>", key);
                    println!("{}", s);
                    return false;
                }
                return true;
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

    pub fn set_value(&mut self, map_key: &str, key: &str, value: &str, force: bool) -> bool {
        let obj = json::parse(value);
        match obj {
            Ok(_) => {}
            Err(_) => {
                let s =
                    gettext("\"<value>\" is not a vaild JSON object.").replace("<value>", value);
                println!("{}", s);
                return false;
            }
        }
        let obj = obj.unwrap();
        if map_key == "basic" || self.des_map.contains_key(map_key) {
            let des = if map_key == "basic" {
                &self.basic
            } else {
                self.des_map.get("map_key").unwrap()
            };
            let re = des.check_valid(key, obj.clone());
            if re.is_none() {
                let t = if map_key == "basic" {
                    String::from("bili --help-settings")
                } else {
                    format!("bili --help-settings {}", map_key)
                };
                let s = gettext("Unknown key.\nPlease use \"<command>\" to see available key.")
                    .replace("<command>", t.as_str());
                println!("{}", s);
                return false;
            }
            let re = re.unwrap();
            if !re {
                let t = if map_key == "basic" {
                    String::from("bili --help-settings")
                } else {
                    format!("bili --help-settings {}", map_key)
                };
                let s = gettext("Invalid value.\nPlease use \"<cmd>\" to see more information.")
                    .replace("<cmd>", t.as_str());
                println!("{}", s);
                return false;
            }
            if !self.maps.contains_key(map_key) {
                if force {
                    self.maps.insert(String::from(map_key), SettingJar::new());
                } else {
                    println!(
                        "{}",
                        gettext("Current settings file don't have this setting, please use <cmd>")
                            .replace("<cmd>", "bili config add <provider> <key> <value>")
                    );
                    return false;
                }
            }
            let jar = self.maps.get_mut(map_key).unwrap();
            if !jar.settings.contains_key(key) {
                if !force {
                    println!(
                        "{}",
                        gettext("Current settings file don't have this setting, please use <cmd>")
                            .replace("<cmd>", "bili config add <provider> <key> <value>")
                    );
                    return false;
                }
            }
            jar.add(key, obj);
            return true;
        }
        println!("{}", gettext("Unknown provider name.\nPlease use \"<command>\" to see all available name.\nNOTE: you can always use \"basic\".").replace("<command>", "bili --help-settings --list-providers-only"));
        return false;
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
        if obj.is_none() {
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
