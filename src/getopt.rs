use crate::i18n::gettext;
use crate::opt_list::get_opt_list;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::From;
use std::default::Default;

#[derive(Clone, Copy, PartialEq)]
pub enum ConfigCommand {
    Add,
    Fix,
    Get,
    List,
    Set,
    Tree,
}

pub struct ConfigCommandResult {
    pub typ: ConfigCommand,
    pub list: Vec<String>,
}

impl ConfigCommandResult {
    pub fn new(typ: ConfigCommand, list: Vec<String>) -> ConfigCommandResult {
        ConfigCommandResult { typ, list }
    }
}

impl Clone for ConfigCommandResult {
    fn clone(&self) -> ConfigCommandResult {
        ConfigCommandResult {
            typ: self.typ.clone(),
            list: self.list.clone(),
        }
    }
}

pub struct OptDes {
    _name: String,
    _short_name: Option<String>,
    _description: String,
    _has_value: bool,
    _need_value: bool,
    _value_display_name: Option<String>,
}

impl OptDes {
    pub fn new(
        name: &str,
        short_name: Option<&str>,
        description: &str,
        has_value: bool,
        need_value: bool,
        value_display_name: Option<&str>,
    ) -> Option<OptDes> {
        if !short_name.is_none() && short_name.unwrap().len() != 1 {
            return None;
        }
        if has_value && value_display_name.is_none() {
            return None;
        }
        Some(OptDes {
            _name: String::from(name),
            _short_name: match short_name {
                Some(r) => Some(String::from(r)),
                None => None,
            },
            _description: String::from(description),
            _has_value: has_value,
            _need_value: need_value,
            _value_display_name: match value_display_name {
                Some(r) => Some(String::from(r)),
                None => None,
            },
        })
    }

    pub fn name(&self) -> &str {
        self._name.as_str()
    }

    pub fn short_name(&self) -> Option<String> {
        match &self._short_name {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    pub fn description(&self) -> &str {
        self._description.as_str()
    }

    pub fn has_value(&self) -> bool {
        self._has_value
    }

    pub fn need_value(&self) -> bool {
        self._need_value
    }

    pub fn value_display_name(&self) -> Option<String> {
        match &self._value_display_name {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }
}

impl Clone for OptDes {
    fn clone(&self) -> OptDes {
        OptDes {
            _name: self._name.clone(),
            _short_name: self._short_name.clone(),
            _description: self._description.clone(),
            _has_value: self._has_value.clone(),
            _need_value: self._need_value.clone(),
            _value_display_name: self._value_display_name.clone(),
        }
    }
}

pub struct Opt {
    _name: String,
    _value: Option<String>,
}

impl Opt {
    pub fn new(name: &str, value: Option<&str>) -> Opt {
        Opt {
            _name: String::from(name),
            _value: match value {
                Some(v) => Some(String::from(v)),
                None => None,
            },
        }
    }

    pub fn name(&self) -> &str {
        self._name.as_str()
    }

    pub fn value(&self) -> Option<String> {
        self._value.clone()
    }
}

impl Clone for Opt {
    fn clone(&self) -> Opt {
        Opt {
            _name: self._name.clone(),
            _value: self._value.clone(),
        }
    }
}

pub struct OptDesStore {
    list: Vec<OptDes>,
}

impl OptDesStore {
    pub fn get(&self, key: &str) -> Option<OptDes> {
        for i in self.list.iter() {
            if i.name() == key {
                return Some(i.clone());
            }
        }
        None
    }

    pub fn get_by_short_name(&self, key: &str) -> Option<OptDes> {
        for i in self.list.iter() {
            match i.short_name() {
                Some(sn) => {
                    if key == sn {
                        return Some(i.clone());
                    }
                }
                None => {}
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
            let sn = match i.short_name() {
                Some(r) => format!("-{}", r.as_str()),
                None => String::from("  "),
            };
            let mut ss = format!("{}  --{}", sn.as_str(), i.name());
            if i.has_value() {
                let nn = if i.need_value() {
                    format!("<{}>", i.value_display_name().unwrap().as_str())
                } else {
                    format!("[{}]", i.value_display_name().unwrap().as_str())
                };
                ss += " ";
                ss += nn.as_str();
            }
            if ss.len() >= 40 {
                ss += "\t";
            } else {
                ss += " ".repeat(40 - ss.len()).as_str();
            }
            ss += i.description();
            if s.len() > 0 {
                s += "\n";
            }
            s += ss.as_str();
        }
        println!("{}", s);
    }
}

impl Clone for OptDesStore {
    fn clone(&self) -> OptDesStore {
        OptDesStore {
            list: self.list.clone(),
        }
    }
}

impl Default for OptDesStore {
    fn default() -> OptDesStore {
        OptDesStore {
            list: get_opt_list(),
        }
    }
}

impl From<Vec<OptDes>> for OptDesStore {
    fn from(list: Vec<OptDes>) -> OptDesStore {
        OptDesStore { list: list.clone() }
    }
}

pub struct OptStore {
    list: Vec<Opt>,
    des: OptDesStore,
    args: Vec<String>,
    ind: usize,
    out_des: HashMap<String, OptDesStore>,
    des_dep: HashMap<String, Vec<String>>,
}

impl OptStore {
    pub fn new(default_des: Vec<OptDes>) -> OptStore {
        OptStore {
            list: [].to_vec(),
            des: OptDesStore::from(default_des),
            args: std::env::args().collect(),
            ind: 1,
            out_des: HashMap::new(),
            des_dep: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, list: Vec<OptDes>) {
        let des = OptDesStore::from(list);
        self.out_des.insert(String::from(key), des);
    }

    pub fn add_with_dependence(
        &mut self,
        key: &str,
        list: Vec<OptDes>,
        deps: Vec<&'static str>,
    ) -> Option<bool> {
        let mut ndeps: Vec<String> = Vec::new();
        for dep in deps.iter() {
            if !self.out_des.contains_key(*dep) {
                return None;
            }
            ndeps.push(String::from(*dep));
        }
        let des = OptDesStore::from(list);
        self.out_des.insert(String::from(key), des);
        self.des_dep.insert(String::from(key), ndeps);
        Some(true)
    }

    pub fn get_des(&self, key: &str) -> Option<OptDes> {
        let re = self.des.get(key);
        if !re.is_none() {
            return re;
        }
        for (_, val) in self.out_des.iter() {
            let re = val.get(key);
            if !re.is_none() {
                return re;
            }
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
    /// If option not found or option don't have any value, return None
    pub fn get_option(&self, key: &str) -> Option<String> {
        let mut last = None;
        for i in self.list.iter() {
            if i.name() == key {
                last = i.value();
            }
        }
        last
    }

    pub fn get_des_by_short_name(&self, key: &str) -> Option<OptDes> {
        let re = self.des.get_by_short_name(key);
        if !re.is_none() {
            return re;
        }
        for (_, val) in self.out_des.iter() {
            let re = val.get(key);
            if !re.is_none() {
                return re;
            }
        }
        None
    }

    pub fn has_option(&self, key: &str) -> bool {
        for i in self.list.iter() {
            if i.name() == key {
                return true;
            }
        }
        false
    }

    pub fn parse_config_command(&mut self) -> Option<ConfigCommandResult> {
        self.ind += 1;
        if self.ind < self.args.len() {
            let s = &self.args[self.ind];
            self.ind += 1;
            if s.starts_with("-") {
                self.ind -= 1;
                return None;
            }
            if s == "add" && self.args.len() >= self.ind + 3 {
                self.ind += 3;
                return Some(ConfigCommandResult::new(
                    ConfigCommand::Add,
                    self.args[self.ind - 3..self.ind].to_vec(),
                ));
            }
            self.ind -= 1;
        }
        None
    }

    pub fn parse_options(&mut self) -> bool {
        self.list.clear();
        while self.ind < self.args.len() {
            let s = &self.args[self.ind];
            self.ind += 1;
            if s.starts_with("--") {
                let name = s.strip_prefix("--").unwrap();
                let optdes = self.get_des(name);
                if optdes.is_none() {
                    let s = gettext("<s> is not a vaild option.")
                        .replace("<s>", format!("--{}", name).as_str());
                    println!("{}", s);
                    return false;
                }
                let optdes = optdes.unwrap();
                if !optdes.has_value() {
                    self.list.push(Opt::new(name, None));
                } else {
                    if !optdes.need_value() {
                        if self.ind >= self.args.len() {
                            self.list.push(Opt::new(name, None));
                        } else {
                            let v = &self.args[self.ind];
                            self.ind += 1;
                            if v.starts_with("-") {
                                self.ind -= 1;
                                self.list.push(Opt::new(name, None));
                            } else {
                                self.list.push(Opt::new(name, Some(v.as_str())));
                            }
                        }
                    } else {
                        if self.ind >= self.args.len() {
                            let s = gettext("<option> need an argument.")
                                .replace("<option>", format!("--{}", name).as_str());
                            println!("{}", s);
                            return false;
                        } else {
                            let v = &self.args[self.ind];
                            self.ind += 1;
                            if v.starts_with("-") {
                                self.ind -= 1;
                                let s = gettext("<option> need an argument.")
                                    .replace("<option>", format!("--{}", name).as_str());
                                println!("{}", s);
                                return false;
                            } else {
                                self.list.push(Opt::new(name, Some(v.as_str())));
                            }
                        }
                    }
                }
            } else if s.starts_with("-") {
                let opts = String::from(s.strip_prefix("-").unwrap());
                if opts.len() == 0 {
                    let s = gettext("<s> is not a vaild option.").replace("<s>", "-");
                    println!("{}", s);
                    return false;
                }
                let mut i = 0;
                while i < opts.len() {
                    let opt = &opts[i..i + 1];
                    i += 1;
                    let optdes = self.get_des_by_short_name(opt);
                    if optdes.is_none() {
                        let s = gettext("<s> is not a vaild option.")
                            .replace("<s>", format!("-{}", opt).as_str());
                        println!("{}", s);
                        return false;
                    }
                    let optdes = optdes.unwrap();
                    if !optdes.has_value() {
                        self.list.push(Opt::new(optdes.name(), None));
                    } else {
                        if !optdes.need_value() {
                            if i < opts.len() {
                                let v = &opts[i..opts.len()];
                                i = opts.len();
                                self.list.push(Opt::new(optdes.name(), Some(v)));
                            } else if self.ind < self.args.len() {
                                let v = &self.args[self.ind];
                                self.ind += 1;
                                if v.starts_with("-") {
                                    self.ind -= 1;
                                    self.list.push(Opt::new(optdes.name(), None));
                                } else {
                                    self.list.push(Opt::new(optdes.name(), Some(v)));
                                }
                            } else {
                                self.list.push(Opt::new(optdes.name(), None));
                            }
                        } else {
                            if i < opts.len() {
                                let v = &opts[i..opts.len()];
                                i = opts.len();
                                self.list.push(Opt::new(optdes.name(), Some(v)));
                            } else if self.ind < self.args.len() {
                                let v = &self.args[self.ind];
                                self.ind += 1;
                                if v.starts_with("-") {
                                    self.ind -= 1;
                                    let s = gettext("<option> need an argument.")
                                        .replace("<option>", format!("-{}", opt).as_str());
                                    println!("{}", s);
                                    return false;
                                } else {
                                    self.list.push(Opt::new(optdes.name(), Some(v)));
                                }
                            } else {
                                let s = gettext("<option> need an argument.")
                                    .replace("<option>", format!("-{}", opt).as_str());
                                println!("{}", s);
                                return false;
                            }
                        }
                    }
                }
            } else {
                self.ind -= 1;
                break;
            }
        }
        return true;
    }

    pub fn parse_url(&mut self) -> Option<String> {
        while self.ind < self.args.len() {
            let s = &self.args[self.ind];
            self.ind += 1;
            if s.starts_with("-") {
                self.ind -= 1;
                return None;
            }
            return Some(s.clone());
        }
        return None;
    }

    pub fn print_help(&self, detail: Option<String>, help_deps: bool) {
        if detail.is_none() || detail.clone().unwrap() == "full" {
            println!("{}", gettext("Basic options:"));
            self.des.print_help();
        }
        for (name, val) in self.out_des.iter() {
            if detail.is_none() {
                let s = gettext("<provider> provide <num> options, use --help full or --help <provider> to see details.").replace("<provider>", name).replace("<num>", format!("{}", val.len()).as_str());
                println!("{}", s);
            } else {
                let d = detail.clone().unwrap();
                if d == "full" || &d == name {
                    let s =
                        gettext("Options provided from <provider>: ").replace("<provider>", name);
                    println!("{}", s);
                    val.print_help();
                    if d != "full" {
                        let deps = self.get_des_dependence(name);
                        match deps {
                            Some(deps) => {
                                for dep in deps.iter() {
                                    let dd = self.out_des.get(dep).unwrap();
                                    if !help_deps {
                                        let s = gettext("<provider> provider <num> options for <provider2>, add --help-deps to see.").replace("<provider>", dep.as_str()).replace("<num>", format!("{}", dd.len()).as_str()).replace("<provider2>", name);
                                        println!("{}", s);
                                    } else {
                                        let s = gettext("Options provided from <provider>: ")
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
        for (name, _) in self.out_des.iter() {
            println!("{}", name);
        }
    }
}

impl Clone for OptStore {
    fn clone(&self) -> OptStore {
        OptStore {
            list: self.list.clone(),
            des: self.des.clone(),
            args: self.args.clone(),
            ind: self.ind.clone(),
            out_des: self.out_des.clone(),
            des_dep: self.des_dep.clone(),
        }
    }
}

impl Default for OptStore {
    fn default() -> OptStore {
        OptStore {
            list: [].to_vec(),
            des: OptDesStore::default(),
            args: std::env::args().collect(),
            ind: 1,
            out_des: HashMap::new(),
            des_dep: HashMap::new(),
        }
    }
}
