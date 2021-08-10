mod bilibili;
mod cookies_json;
mod http_client;
mod i18n;
mod path;
mod provider_base;
mod providers;

use cookies_json::CookiesJar;
use cookies_json::CookiesJson;
use i18n::gettext;
use provider_base::Provider;

struct Main {
    cookies: CookiesJson,
}

impl Main {
    fn new() -> Main {
        let mut cookies = CookiesJson::new();
        cookies.read(None);
        Main { cookies: cookies }
    }

    fn run(&mut self) -> i32 {
        let pro = providers::match_provider("");
        match pro {
            Some(_) => {}
            None => {
                println!("{}", gettext("Can not find suitable provider."));
                return 1;
            }
        }
        let mut pro = pro.unwrap();
        let jar = match pro.get_default_cookie_jar_name() {
            Some(s) => self.cookies.get(s),
            None => None,
        };
        if !pro.init(jar) {
            println!("{}", gettext("Can not initialize provider."));
            return 1;
        }
        if pro.can_login() {
            let p = pro.check_logined();
            if p.is_none() {
                println!("{}", gettext("Error occured when checking login."));
                if pro.login_required() {
                    return 1;
                }
            } else {
                let mut p = p.unwrap();
                if !p && pro.login_required() {
                    let k = match pro.get_default_cookie_jar_name() {
                        Some(s) => s,
                        None => {
                            println!("{}", gettext("Name is needed for cookie jar."));
                            return 1;
                        },
                    };
                    let mut jar = CookiesJar::new();
                    p = pro.login(&mut jar);
                    if !p {
                        println!("{}", gettext("Login failed."));
                        return 1;
                    }
                    self.cookies.add(k, jar);
                }
                let s = pro.logined();
                if s != p {
                    println!("{}", gettext("Warn: fuction check_logined and logined return different result."));
                    p = s;
                }
                if p {
                    println!("{}", gettext("Verify login successfully."));
                }
            }
        }
        return 0;
    }
}

#[tokio::main]
async fn main() {
    let mut m = Main::new();
    std::process::exit(m.run());
}
