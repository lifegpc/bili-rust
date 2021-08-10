mod bilibili;
mod cookies_json;
mod i18n;
mod path;
mod provider_base;
mod providers;

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
        let pro = providers::match_provider("", &self.cookies);
        match pro {
            Some(_) => {}
            None => {
                println!("{}", gettext("Can not find suitable provider."));
                return 1;
            }
        }
        let pro = pro.unwrap();
        if pro.can_login() {
            let p = pro.check_logined();
        }
        return 0;
    }
}

#[tokio::main]
async fn main() {
    let mut m = Main::new();
    std::process::exit(m.run());
}
