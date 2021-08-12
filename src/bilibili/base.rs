extern crate futures;
extern crate json;
extern crate reqwest;
extern crate thirtyfour;

use crate::bilibili::opt_list::get_bili_base_options;
use crate::cookies_json::Cookie;
use crate::cookies_json::CookiesJar;
use crate::getopt::OptDes;
use crate::getopt::OptStore;
use crate::http_client::CookieClient;
use crate::i18n::gettext;
use crate::opt_list::get_webdriver_options;
use crate::provider_base::Provider;
use crate::webdriver::WebDriverStarter;
use crate::webdriver::WebDriverType;
use futures::executor::block_on;
use json::JsonValue;
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::thread::sleep;
use std::time::Duration;
use subprocess::Popen;
use thirtyfour::prelude::DesiredCapabilities;
use thirtyfour::prelude::WebDriver;
use thirtyfour::prelude::WebDriverCommands;

pub struct BiliBaseProvider {
    client: Option<CookieClient>,
    user_info: Option<JsonValue>,
    opt: Option<OptStore>,
}

impl BiliBaseProvider {
    pub fn init_client(&mut self, jar: Option<&CookiesJar>) -> bool {
        let mut builder = Client::builder();
        let mut h = HeaderMap::new();
        h.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.111 Safari/537.36".parse().unwrap());
        h.insert("Connection", "keep-alive".parse().unwrap());
        h.insert(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"
                .parse()
                .unwrap(),
        );
        h.insert("Accept-Language", "zh-CN,zh;q=0.8".parse().unwrap());
        builder = builder.default_headers(h);
        builder = builder.gzip(true);
        builder = builder.brotli(true);
        builder = builder.deflate(true);
        let r = builder.build();
        match r {
            Ok(_) => {}
            Err(_) => {
                return false;
            }
        }
        self.client = Some(CookieClient::new(r.unwrap(), jar));
        return true;
    }
}

impl Provider for BiliBaseProvider {
    fn new() -> BiliBaseProvider {
        BiliBaseProvider {
            client: None,
            user_info: None,
            opt: None,
        }
    }

    fn add_custom_options(&self, opt: &mut OptStore) {
        opt.add("WebDriver", get_webdriver_options());
        opt.add(self.provider_name(), BiliBaseProvider::get_custom_options());
    }

    fn can_login(&self) -> bool {
        return true;
    }

    fn check_logined(&mut self) -> Option<bool> {
        match self.client {
            Some(_) => {}
            None => {
                return None;
            }
        }
        let client = self.client.as_ref().unwrap();
        let r = client.get("https://api.bilibili.com/x/web-interface/nav");
        match r {
            Some(_) => {}
            None => {
                return None;
            }
        }
        let r = r.unwrap();
        let st = r.status().as_u16();
        if st != 200 {
            return None;
        }
        let text = r.text_with_charset("UTF-8");
        let text = block_on(text);
        match text {
            Ok(_) => {}
            Err(_) => {
                return None;
            }
        }
        let text = text.unwrap();
        let re = json::parse(text.as_str());
        match re {
            Ok(_) => {}
            Err(_) => {
                return None;
            }
        }
        let obj = re.unwrap();
        let code = obj["code"].as_i64();
        match code {
            Some(_) => {}
            None => {
                println!(
                    "{}",
                    gettext("Error: code return from API is not an integer.")
                );
                return None;
            }
        }
        let code = code.unwrap();
        if code == 0 {
            let result = &obj["data"];
            let s = result.dump();
            self.user_info = Some(json::parse(s.as_str()).unwrap());
            return Some(true);
        } else if code == -101 {
            return Some(false);
        }
        println!("{}{}", gettext("Unknown codition: "), text);
        return None;
    }

    fn get_custom_options() -> Vec<OptDes> {
        get_bili_base_options()
    }

    fn get_default_cookie_jar_name(&self) -> Option<&str> {
        Some("bili")
    }

    fn has_custom_options(&self) -> bool {
        true
    }

    fn login(&mut self, jar: &mut CookiesJar) -> bool {
        let starter = WebDriverStarter::new(self.opt.clone());
        let re = starter.get();
        if re.is_none() {
            return false;
        }
        let re = re.unwrap();
        let mut p: Option<Popen> = None;
        if !re.cml.is_none() {
            p = starter.start_server(re.cml.unwrap());
            if p.is_none() {
                return false;
            }
            println!("{}", gettext("Started webdriver server."));
        }
        if re.typ == WebDriverType::Chrome {
            let caps = DesiredCapabilities::chrome();
            let driver = WebDriver::new(re.url.as_str(), caps);
            let driver = block_on(driver);
            match driver {
                Ok(_) => {}
                Err(_) => {
                    println!("{}", gettext("Can not connect to chrome driver."));
                    if !p.is_none() {
                        starter.kill_server(&mut p.unwrap());
                    }
                    return false;
                }
            }
            let driver = driver.unwrap();
            let url = "https://passport.bilibili.com/ajax/miniLogin/minilogin";
            let re = block_on(driver.get(url));
            match re {
                Ok(_) => {}
                Err(_) => {
                    let s = gettext("Can not open \"<url>\" in browser.").replace("<url>", url);
                    println!("{}", s);
                    starter.quit_driver(driver);
                    if !p.is_none() {
                        starter.kill_server(&mut p.unwrap());
                    }
                    return false;
                }
            }
            loop {
                let re = block_on(driver.current_url());
                match re {
                    Ok(_) => {}
                    Err(_) => {
                        println!("{}", gettext("Can not get current url from web driver."));
                        starter.quit_driver(driver);
                        if !p.is_none() {
                            starter.kill_server(&mut p.unwrap());
                        }
                        return false;
                    }
                }
                let url = re.unwrap();
                if url.starts_with("https://passport.bilibili.com/ajax/miniLogin/redirect") {
                    break;
                }
                sleep(Duration::new(10, 0));
            }
            let re = block_on(driver.get_cookies());
            match re {
                Ok(_) => {}
                Err(_) => {
                    println!("{}", gettext("Can not get cookies from web driver."));
                    starter.quit_driver(driver);
                    if !p.is_none() {
                        starter.kill_server(&mut p.unwrap());
                    }
                    return false;
                }
            }
            let cookies = re.unwrap();
            for cookie in cookies.iter() {
                let c = Cookie::from_thirtyfour_cookie(cookie.clone());
                match c {
                    Some(c) => {
                        jar.add(c);
                    }
                    None => {}
                }
            }
            starter.quit_driver(driver);
            self.client.as_mut().unwrap().set_cookies_jar(jar.clone());
            match self.check_logined() {
                Some(_) => {}
                None => {}
            }
        }
        if !p.is_none() {
            starter.kill_server(&mut p.unwrap());
        }
        return self.logined();
    }

    fn logined(&self) -> bool {
        match &self.user_info {
            Some(ui) => {
                let o = &ui["isLogin"];
                let t = o.as_bool();
                match t {
                    Some(t) => t,
                    None => {
                        println!(
                            "{}",
                            "BiliBaseProvider: can not get is_login from user_info."
                        );
                        false
                    }
                }
            }
            None => false,
        }
    }

    fn init(&mut self, jar: Option<&CookiesJar>, opt: OptStore) -> bool {
        self.opt = Some(opt);
        self.init_client(jar)
    }

    fn match_url(_url: &str) -> bool {
        return true;
    }

    fn provider_name(&self) -> &'static str {
        "BiliBaseProvider"
    }
}
