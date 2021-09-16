extern crate reqwest;

use crate::cookies_json::CookiesJar;
use crate::http_client::CookieClient;
use crate::providers::provider_base::Provider;
use reqwest::header::HeaderMap;
use reqwest::Client;

pub struct TiktokBaseProvider {
    pub client: Option<CookieClient>,
}

impl TiktokBaseProvider {
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
        let mut cli = CookieClient::new(r.unwrap(), jar);
        cli.enable_set_cookie();
        cli.get("https://www.tiktok.com");
        self.client = Some(cli);
        return true;
    }
}

impl Provider for TiktokBaseProvider {
    fn new() -> Self {
        Self { client: None }
    }

    fn provider_name(&self) -> &'static str {
        "TiktokBaseProvider"
    }
}
