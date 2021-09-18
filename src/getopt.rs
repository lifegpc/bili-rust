use crate::i18n::gettext;
use crate::opt_list::get_opt_list;
use crate::utils::size::parse_size;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::From;
use std::default::Default;

/// The config command type parsed from command line
#[derive(Clone, Copy, PartialEq)]
pub enum ConfigCommand {
    Add,
    Fix,
    Get,
    Set,
}

/// Config command type and arguments
pub struct ConfigCommandResult {
    /// config command type
    pub typ: ConfigCommand,
    /// arguments of command
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

/// The description struct of an option
pub struct OptDes {
    _name: String,
    _short_name: Option<String>,
    _description: String,
    _has_value: bool,
    _need_value: bool,
    _value_display_name: Option<String>,
}

impl OptDes {
    /// Create a new description sturct of an option
    /// * `name` - Name
    /// * `short_name` - Short name (one letter)
    /// * `description` - Description
    /// * `has_value` - Can this option have an argument
    /// * `need_value` - Should this option must have an argument
    /// * `value_display_name` - Display name in print help message (This argument should be None when `has_value` is false, and must have a value when `has_value` is true)
    /// 
    /// # Examples
    /// ```
    /// let opt = OptDes::new("help", Some("h"), "Print help message", true, false, Some("type"));
    /// ```
    /// 
    /// When printing help message, it will output something like this: `-h  --help [type] Print help message`
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

    /// Return the name of the option
    pub fn name(&self) -> &str {
        self._name.as_str()
    }

    /// Return the short name of the option
    pub fn short_name(&self) -> Option<String> {
        match &self._short_name {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    /// Return the description of the option
    pub fn description(&self) -> &str {
        self._description.as_str()
    }

    /// Return whether to this option can have an argument
    pub fn has_value(&self) -> bool {
        self._has_value
    }

    /// Return whether to this option must have an argument
    pub fn need_value(&self) -> bool {
        self._need_value
    }

    /// the display name in print help message
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

/// Option struct, only used in getopt. Please use function in [`OptStore`](struct.OptStore.html) to access user's command line options.
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

/// A list of [`OptDes`](struct.OptDes.html)
pub struct OptDesStore {
    list: Vec<OptDes>,
}

impl OptDesStore {
    /// Return a description struct of option by description name
    /// * `key` - Name
    pub fn get(&self, key: &str) -> Option<OptDes> {
        for i in self.list.iter() {
            if i.name() == key {
                return Some(i.clone());
            }
        }
        None
    }

    /// Return a description struct of option by description short name
    /// * `key` - Short name
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

    /// Return the list len
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Print help message by using the information in the list
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
    /// Return a list of basic command options' description struct of main program.
    fn default() -> OptDesStore {
        OptDesStore {
            list: get_opt_list(),
        }
    }
}

impl From<Vec<OptDes>> for OptDesStore {
    /// Generate from a `Vec`
    fn from(list: Vec<OptDes>) -> OptDesStore {
        OptDesStore { list: list.clone() }
    }
}

/// A structure used to parse command line
pub struct OptStore {
    /// The list of option already parsed
    list: Vec<Opt>,
    /// The list of basic options
    des: OptDesStore,
    /// Command line arguments
    args: Vec<String>,
    /// The next argument's index should be parsed
    ind: usize,
    /// Other providers' options list.
    out_des: HashMap<String, OptDesStore>,
    /// The dependent list for providers. `basic` is always included and don't need on the list.
    des_dep: HashMap<String, Vec<String>>,
}

impl OptStore {
    /// Create a new one with custom basic options list
    /// * `default_des` - The list of basic options
    /// # Examples
    /// ```
    /// let mut opt = OptStore::new(vec![OptDes::new("some", Some("s"), "Description", false, false, None).unwrap(),]);
    /// ```
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

    /// Add a provider's options
    /// * `key` - Provider's name
    /// * `list` - Provider's options
    /// # Examples
    /// ```
    /// let mut opt = OptStore::default();
    /// opt.add("Provider name", vec![OptDes::new("some", Some("s"), "Description", false, false, None).unwrap(),]);
    /// ```
    /// # Notes
    /// If this provider depend on another provider, you need use [`add_with_dependence`](#method.add_with_dependence) in function [`add_all_opts`](../providers/fn.add_all_opts.html).
    pub fn add(&mut self, key: &str, list: Vec<OptDes>) {
        let des = OptDesStore::from(list);
        self.out_des.insert(String::from(key), des);
    }

    /// Add a provider's options with its dependencies
    /// * `key` - Provider's name
    /// * `list` - Provider's options
    /// * `deps` - Provider's dependencies
    /// # Examples
    /// ```
    /// let mut opt = OptStore::default();
    /// opt.add("Provider1", vec![OptDes::new("some", Some("s"), "Description", false, false, None).unwrap(),]);
    /// opt.add_with_dependence("Provider2", vec![OptDes::new("some2", None, "Description", false, false, None).unwrap(),], vec!["Provider1"]);
    /// ```
    /// # Notes
    /// 1. The name in dependencies must be added before.
    /// 2. Dependencies information now only used on help message. You just need use this function in function [`add_all_opts`](../providers/fn.add_all_opts.html). You can use [`add`](#method.add) in any other location.
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

    /// Get a description struct by using option's name
    /// * `key` - Option's name
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

    /// Get dependencies list of a provider
    /// * `key` - Provider's name
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
    /// Get option's argument
    /// * `key` - Option's name
    /// # Examples
    /// ```
    /// let mut opt = OptStore::default();
    /// if opt.parse_options() {
    ///     let cookies = opt.get_option("cookies");
    /// }
    /// ```
    /// # Notes
    /// If option not found or option don't have any argument, return None
    pub fn get_option(&self, key: &str) -> Option<String> {
        let mut last = None;
        for i in self.list.iter() {
            if i.name() == key {
                last = i.value();
            }
        }
        last
    }

    /// Get option's argument as boolean
    /// * `key` - Option's name
    /// # Notes
    /// If an option not found, will retrun `None`. If option's value is not vaild, will panic.
    /// # Panic
    /// When an option's value is not valid, will panic.
    pub fn get_option_as_bool(&self, key: &str) -> Option<bool> {
        let c = self.get_option(key);
        if c.is_none() {
            return None;
        }
        let c = c.as_ref().unwrap().trim();
        let i = c.parse::<i128>();
        if i.is_ok() {
            return Some(i.unwrap() != 0);
        }
        let lc = c.to_lowercase();
        if lc == "true" {
            return Some(true);
        } else if lc == "false" {
            return Some(false);
        }
        let s = gettext("The value of option \"<key>\" should be a boolean or number. For example: true, 1, false, 0.").replace("<key>", key);
        panic!("{}", s)
    }

    /// Get option's argument and convert it to bytes by using [`parse_size`](../utils/size/fn.parse_size.html)
    /// * `key` - Option's name
    /// # Notes
    /// If an option not found, will retrun `None`. If option's value is not vaild, will panic.
    /// # Panic
    /// When an option's value is not valid, will panic.
    pub fn get_option_as_size(&self, key: &str) -> Option<usize> {
        let c = self.get_option(key);
        if c.is_none() {
            return None;
        }
        let r = parse_size(c.unwrap().as_str());
        if r.is_none() {
            let s = gettext("The value of option \"<key>\" should be a vaild size. For example: 1, 2B, 300K, 78Mi, 900GiB.").replace("<key>", key);
            panic!("{}", s);
        }
        r
    }

    /// Get option's argument and convert it to `usize`
    /// * `key` - Option's name
    /// # Notes
    /// If an option not found, will retrun `None`. If option's value is not vaild, will panic.
    /// # Panic
    /// When an option's value is not valid, will panic.
    pub fn get_option_as_usize(&self, key: &str) -> Option<usize> {
        let c = self.get_option(key);
        if c.is_none() {
            return None;
        }
        let r = c.unwrap().as_str().parse::<usize>();
        match r {
            Ok(r) => Some(r),
            Err(_) => {
                let s = gettext("The value of option \"<key>\" should be a positive interger or 0. For example: 1, 0.").replace("<key>", key);
                panic!("{}", s);
            }
        }
    }

    /// Get a description struct by using option's short name
    /// * `key` - Option's short name
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

    /// Return whether have this option
    /// * `key` - Option's name
    /// # Examples
    /// ```
    /// let mut opt = OptStore::default();
    /// if opt.parse_options() {
    ///     if opt.has_option("help") {
    ///         // Print help message
    ///     }
    /// }
    /// ```
    pub fn has_option(&self, key: &str) -> bool {
        for i in self.list.iter() {
            if i.name() == key {
                return true;
            }
        }
        false
    }

    /// Parse config command, only used in `bili config` command.
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
            if s == "fix" {
                return Some(ConfigCommandResult::new(ConfigCommand::Fix, [].to_vec()));
            }
            if s == "get" && self.args.len() >= self.ind + 2 {
                self.ind += 2;
                return Some(ConfigCommandResult::new(
                    ConfigCommand::Get,
                    self.args[self.ind - 2..self.ind].to_vec(),
                ));
            }
            if s == "set" && self.args.len() >= self.ind + 3 {
                self.ind += 3;
                return Some(ConfigCommandResult::new(
                    ConfigCommand::Set,
                    self.args[self.ind - 3..self.ind].to_vec(),
                ));
            }
            self.ind -= 1;
        }
        None
    }

    /// Parse options, if any error occured, will return false
    /// # Examples
    /// ```
    /// let opt = OptStore::default();
    /// if opt.parse_options() {
    ///     // Do something
    /// }
    /// ```
    /// # Notes
    /// 1. This function will clear [`self.list`](#structfield.list), so [`get_option`](#method.get_option) and [`has_option`](#method.has_option) will not return previous result.
    /// 2. If found a non-option argument, this function will stop parse and return true.
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

    /// Parse url from argument
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

    /// Print help message
    /// * `detail` - The provider name that want to print help message
    /// * `help_deps` - Whether to print provider's dependencies' help message. If `detail` is `None` or `full`, this argument no any effects.
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

    /// Print all prviders' name (key) in [`self.out_des`](#structfield.out_des)
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
    /// Create a new one with default basic options.
    /// The options list can be found in function [`get_opt_list`](../opt_list/fn.get_opt_list.html)
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
