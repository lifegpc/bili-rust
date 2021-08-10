extern crate futures;
extern crate json;
extern crate reqwest;

use crate::cookies_json::CookiesJar;
use crate::i18n::gettext;
use crate::provider_base::Provider;
use futures::executor::block_on;
use reqwest::header::HeaderMap;
use reqwest::Client;

pub struct BiliBaseProvider {
    client: Option<Client>,
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
        self.client = Some(r.unwrap());
        return true;
    }
}

impl Provider for BiliBaseProvider {
    fn new(jar: Option<&CookiesJar>) -> BiliBaseProvider {
        let mut r = BiliBaseProvider { client: None };
        r.init_client(jar);
        r
    }

    fn can_login(&self) -> bool {
        return true;
    }

    fn check_logined(&self) -> Option<bool> {
        match self.client {
            Some(_) => {}
            None => {
                return None;
            }
        }
        let client = self.client.as_ref().unwrap();
        let r = client.get("https://api.bilibili.com/x/web-interface/nav");
        let r = r.send();
        let r = block_on(r);
        match r {
            Ok(_) => {}
            Err(e) => {
                println!("{}{}", gettext("Error when request: "), e);
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
        println!("{}", text);
        let re = json::parse(text.as_str());
        match re {
            Ok(_) => {}
            Err(_) => {
                return None;
            }
        }
        let obj = re.unwrap();
        return Some(false);
    }

    fn login(&self, _jar: &mut CookiesJar) -> bool {
        return false;
    }

    fn match_url(_url: &str) -> bool {
        return true;
    }
}
