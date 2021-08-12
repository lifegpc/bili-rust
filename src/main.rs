mod cookies_json;
mod getopt;
mod http_client;
mod i18n;
mod opt_list;
mod path;
mod providers;
mod settings;
mod webdriver;

use cookies_json::CookiesJar;
use cookies_json::CookiesJson;
use getopt::OptStore;
use i18n::gettext;
use providers::provider_base::Provider;
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
            opt: OptStore::new(),
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
            return Some(String::from(re.as_str().unwrap()))
        }
        None
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
                let help = self.opt.get_option("help");
                self.opt.print_help(help, self.opt.has_option("help-deps"));
                return 0;
            } else if self.opt.has_option("version") {
                self.print_version();
                return 0;
            } else if self.opt.has_option("help-settings") {
                providers::add_all_settings(&mut self.se);
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
        let pro = providers::match_provider(url.as_str());
        match pro {
            Some(_) => {}
            None => {
                println!("{}", gettext("Can not find suitable provider."));
                return 1;
            }
        }
        let mut pro = pro.unwrap();
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
        return 0;
    }
}

#[tokio::main]
async fn main() {
    let mut m = Main::new();
    std::process::exit(m.run());
}
