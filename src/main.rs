mod cookies_json;
mod i18n;
mod path;

use cookies_json::CookiesJson;

struct Main {
    cookies: CookiesJson
}

impl Main {
    fn new() -> Main {
        let mut cookies = CookiesJson::new();
        cookies.read(None);
        Main {
            cookies: cookies
        }
    }
}

fn main() {
    let m = Main::new();
}
