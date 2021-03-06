#[macro_use]
extern crate lazy_static;

mod cookies_json;
mod downloader;
mod getopt;
mod http_client;
mod i18n;
mod metadata;
mod opt_list;
mod providers;
mod settings;
#[cfg(test)]
mod testutils;
mod utils;
mod webdriver;

use cookies_json::CookiesJar;
use cookies_json::CookiesJson;
use downloader::downloader::MDownloader;
use getopt::ConfigCommand;
use getopt::CookieCommand;
use getopt::OptStore;
use i18n::gettext;
use providers::bilibili::normal_video::BiliNormalVideoProvider;
use providers::provider_base::Provider;
use providers::tiktok::video::TiktokVideoProvider;
use settings::SettingStore;

struct Main {
    cookies: CookiesJson,
    opt: OptStore,
    se: SettingStore,
}

impl Main {
    fn new() -> Main {
        let cookies = CookiesJson::new();
        Main {
            cookies: cookies,
            opt: OptStore::default(),
            se: SettingStore::new(),
        }
    }

    fn get_cookies(&self) -> Option<String> {
        let re = self.opt.get_option("cookies");
        if !re.is_none() {
            return re;
        }
        let re = self.se.get_settings("basic", "cookies");
        if !re.is_none() {
            let re = re.unwrap();
            return Some(String::from(re.as_str().unwrap()));
        }
        None
    }

    fn print_config_basic_usage(&self) {
        println!(
            "bili config add <provider> <key> <value> [options] \t{}",
            gettext("Add entry to settings file.")
        );
        println!(
            "bili config delete <provider> <key> [Options] \t\t{}",
            gettext("Delete an entry from settings file.")
        );
        println!(
            "bili config fix [options] \t\t\t\t{}",
            gettext("Fix broken settings file.")
        );
        println!(
            "bili config get <provider> <key> [options] \t\t{}",
            gettext("Get entry value from settings file.")
        );
        println!(
            "bili config set <provider> <key> <value> [Options] \t{}",
            gettext("Set value for an entry.")
        );
    }

    fn print_cookie_basic_usage(&self) {
        println!(
            "bili cookie load <jar_name> <file> [Options] \t{}",
            gettext("Load cookies from file.")
        )
    }

    fn print_version(&self) {
        let v = env!("CARGO_PKG_VERSION");
        println!("bili  v{}  Copyright (C) 2021  lifegpc", v);
        println!("This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.");
        println!("This is free software, and you are welcome to redistribute it");
        println!("under certain conditions.");
    }

    fn run(&mut self) -> i32 {
        let url = self.opt.parse_url();
        if url.is_none() {
            if !self.opt.parse_options() {
                return 1;
            }
            if self.opt.has_option("help") {
                providers::add_all_opts(&mut self.opt);
                if self.opt.has_option("list-providers-only") {
                    self.opt.print_providers();
                    return 0;
                }
                println!("bili <url> [options]");
                println!(
                    "bili config -h \t\t\t\t{}",
                    gettext("Print how to manage config file by using command line.")
                );
                println!(
                    "bili cookie -h \t\t\t\t{}",
                    gettext("Print how to manage cookies file by using command line.")
                );
                let help = self.opt.get_option("help");
                self.opt.print_help(help, self.opt.has_option("help-deps"));
                return 0;
            } else if self.opt.has_option("version") {
                self.print_version();
                return 0;
            } else if self.opt.has_option("help-settings") {
                providers::add_all_settings(&mut self.se);
                if self.opt.has_option("list-providers-only") {
                    self.se.print_providers();
                    return 0;
                }
                self.se.print_help(
                    self.opt.get_option("help-settings"),
                    self.opt.has_option("help-deps"),
                );
                return 0;
            }
            println!("{}", gettext("Url is needed."));
            return 1;
        }
        let url = url.unwrap();
        if url == "config" {
            return self.run_config();
        }
        if url == "cookie" {
            return self.run_cookie();
        }
        self.match_provider(url.as_str())
    }

    fn match_provider(&mut self, url: &str) -> i32 {
        if BiliNormalVideoProvider::match_url(url) {
            return self.run_iternal(&mut BiliNormalVideoProvider::new(), String::from(url));
        }
        if TiktokVideoProvider::match_url(url) {
            return self.run_iternal(&mut TiktokVideoProvider::new(), String::from(url));
        }
        println!("{}", gettext("Can not find suitable provider."));
        return 1;
    }

    fn run_iternal(&mut self, pro: &mut impl Provider, url: String) -> i32 {
        if pro.has_custom_options() {
            pro.add_custom_options(&mut self.opt);
        }
        if !self.opt.parse_options() {
            return 1;
        }
        if pro.has_custom_settings() {
            pro.add_custom_settings(&mut self.se);
        }
        if !self.se.read(self.opt.get_option("config"), false) {
            return 1;
        }
        self.cookies.read(self.get_cookies());
        let jar = match self.opt.get_option("cookie-jar") {
            Some(s) => self.cookies.get(s.as_str()),
            None => match pro.get_default_cookie_jar_name() {
                Some(s) => self.cookies.get(s),
                None => None,
            },
        };
        if !pro.init(jar, self.opt.clone(), self.se.clone()) {
            println!("{}", gettext("Can not initialize provider."));
            return 1;
        }
        if pro.can_login() {
            let p = pro.check_logined();
            if p.is_none() {
                println!("{}", gettext("Error occured when checking login."));
                if pro.login_required() || self.opt.has_option("login") {
                    return 1;
                }
            } else {
                let mut p = p.unwrap();
                if !p && (pro.login_required() || self.opt.has_option("login")) {
                    let k = match self.opt.get_option("cookie-jar") {
                        Some(s) => s,
                        None => match pro.get_default_cookie_jar_name() {
                            Some(s) => String::from(s),
                            None => {
                                println!("{}", gettext("Name is needed for cookie jar."));
                                return 1;
                            }
                        },
                    };
                    let mut jar = CookiesJar::new();
                    p = pro.login(&mut jar);
                    if !p {
                        println!("{}", gettext("Login failed."));
                        return 1;
                    }
                    self.cookies.add(k.as_str(), jar);
                    if !self.cookies.save(self.get_cookies()) {
                        return 1;
                    }
                }
                let s = pro.logined();
                if s != p {
                    println!(
                        "{}",
                        gettext("Warn: fuction check_logined and logined return different result.")
                    );
                    p = s;
                }
                if p {
                    println!("{}", gettext("Verify login successfully."));
                }
            }
        } else if self.opt.has_option("login") {
            let s = gettext("<provider> don't support login.")
                .replace("<provider>", pro.provider_name());
            println!("{}", s);
            return -1;
        }
        let e = pro.extract(url.as_str());
        if e.is_none() {
            println!("{}", gettext("Can not extract info."));
            return 1;
        }
        let e = e.unwrap();
        if !e.check() {
            println!("{}", gettext("Extract informtaion is invalid."));
            return 1;
        }
        let d = MDownloader::new(&self.se, &self.opt, &e);
        d.run();
        return 0;
    }

    fn run_config(&mut self) -> i32 {
        self.opt = OptStore::new(opt_list::get_config_opt_list());
        let cmd = self.opt.parse_config_command();
        if cmd.is_none() {
            if !self.opt.parse_options() {
                return 1;
            }
            if self.opt.has_option("help") {
                self.print_config_basic_usage();
                self.opt.print_help(None, false);
                return 0;
            }
            self.print_config_basic_usage();
            return 1;
        }
        let cmd = cmd.unwrap();
        if !self.opt.parse_options() {
            return 1;
        }
        providers::add_all_settings(&mut self.se);
        let fix_invalid = self.opt.has_option("fix") || cmd.typ == ConfigCommand::Fix;
        if !self.se.read(self.opt.get_option("config"), fix_invalid) {
            return 1;
        }
        if cmd.typ == ConfigCommand::Add {
            let s = if self.opt.has_option("str") {
                let j = json::JsonValue::String(cmd.list[2].clone());
                j.dump()
            } else {
                cmd.list[2].clone()
            };
            if !self.se.add_value(
                cmd.list[0].as_str(),
                cmd.list[1].as_str(),
                s.as_str(),
                self.opt.has_option("force"),
            ) {
                return 1;
            }
            if !self.se.save(self.opt.get_option("config")) {
                return 1;
            }
            return 0;
        }
        if cmd.typ == ConfigCommand::Delete {
            let re = self
                .se
                .get_settings(cmd.list[0].as_str(), cmd.list[1].as_str());
            match re {
                Some(_) => {
                    if self.se.delete(cmd.list[0].as_str(), cmd.list[1].as_str()) {
                        if self.se.save(self.opt.get_option("config")) {
                            return 0;
                        }
                        println!("{}", gettext("Can not save settings."));
                        return 1;
                    }
                    println!("{}", gettext("Key not found"));
                    return 1;
                }
                None => {
                    println!("{}", gettext("Key not found."));
                    return 1;
                }
            }
        }
        if cmd.typ == ConfigCommand::Fix {
            if !self.se.save(self.opt.get_option("config")) {
                return 1;
            }
            return 0;
        }
        if cmd.typ == ConfigCommand::Get {
            let re = self
                .se
                .get_settings(cmd.list[0].as_str(), cmd.list[1].as_str());
            match re {
                Some(obj) => {
                    println!("{}", obj.pretty(2));
                    return 0;
                }
                None => {
                    println!("{}", gettext("No value found."));
                    return 1;
                }
            }
        }
        if cmd.typ == ConfigCommand::Set {
            let s = if self.opt.has_option("str") {
                let j = json::JsonValue::String(cmd.list[2].clone());
                j.dump()
            } else {
                cmd.list[2].clone()
            };
            if !self.se.set_value(
                cmd.list[0].as_str(),
                cmd.list[1].as_str(),
                s.as_str(),
                self.opt.has_option("force"),
            ) {
                return 1;
            }
            if !self.se.save(self.opt.get_option("config")) {
                return 1;
            }
            return 0;
        }
        return 0;
    }

    fn run_cookie(&mut self) -> i32 {
        self.opt = OptStore::new(opt_list::get_cookie_opt_list());
        let cmd = self.opt.parse_cookie_command();
        if cmd.is_none() {
            if !self.opt.parse_options() {
                return 1;
            }
            if self.opt.has_option("help") {
                self.print_cookie_basic_usage();
                self.opt.print_help(None, false);
                return 0;
            }
            self.print_cookie_basic_usage();
            return 1;
        }
        let cmd = cmd.unwrap();
        if !self.opt.parse_options() {
            return 1;
        }
        let mut c = CookiesJson::new();
        let co = match self.opt.get_option("cookies") {
            Some(c) => c,
            None => {
                let mut p = utils::path::get_exe_path().unwrap();
                p.push("bili.cookies.json");
                String::from(p.to_str().unwrap())
            }
        };
        let p = std::path::PathBuf::from(&co);
        if p.exists() {
            if !c.read(Some(co.clone())) {
                return 1;
            }
        }
        if cmd.typ == CookieCommand::Load {
            let j = CookiesJar::from_netscape_cookie_file(&cmd.list[1]);
            if j.is_none() {
                return 1;
            }
            c.add(&cmd.list[0], j.unwrap());
        }
        if !c.save(Some(co.clone())) {
            return 1;
        }
        0
    }
}

#[tokio::main]
async fn main() {
    let mut m = Main::new();
    std::process::exit(m.run());
}
