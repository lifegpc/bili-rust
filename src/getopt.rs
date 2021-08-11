use crate::i18n::gettext;
use crate::opt_list::get_opt_list;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::From;
use std::default::Default;

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
                None => String::from(""),
            };
            let mut ss = format!("{}\t--{}", sn.as_str(), i.name());
            if i.has_value() {
                let nn = if i.need_value() {
                    format!("<{}>", i.value_display_name().unwrap().as_str())
                } else {
                    format!("[{}]", i.value_display_name().unwrap().as_str())
                };
                ss += " ";
                ss += nn.as_str();
            }
            ss += "\t";
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
}

impl OptStore {
    pub fn new() -> OptStore {
        OptStore {
            list: [].to_vec(),
            des: OptDesStore::default(),
            args: std::env::args().collect(),
            ind: 1,
            out_des: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, list: Vec<OptDes>) {
        let des = OptDesStore::from(list);
        self.out_des.insert(String::from(key), des);
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

    pub fn print_help(&self, detail: Option<String>) {
        if detail.is_none() || detail.clone().unwrap() == "full" {
            println!("bili <url> [options]");
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
                }
            }
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
        }
    }
}
