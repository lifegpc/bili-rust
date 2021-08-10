extern crate futures;
extern crate reqwest;

use crate::cookies_json::CookiesJar;
use crate::i18n::gettext;
use futures::executor::block_on;
use reqwest::Client;
use reqwest::IntoUrl;
use reqwest::RequestBuilder;
use reqwest::Response;
use std::clone::Clone;
use std::collections::HashMap;

pub struct CookieClient {
    client: Client,
    jar: CookiesJar,
}

impl CookieClient {
    pub fn new(client: Client, jar: Option<&CookiesJar>) -> CookieClient {
        let j = match jar {
            Some(ja) => ja.clone(),
            None => CookiesJar::new(),
        };
        CookieClient {
            client: client.clone(),
            jar: j,
        }
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> Option<Response> {
        let r = self.aget(url);
        let r = r.send();
        let r = block_on(r);
        match r {
            Ok(_) => {}
            Err(e) => {
                println!("{}{}", gettext("Error when request: "), e);
                return None;
            }
        }
        Some(r.unwrap())
    }

    pub fn aget<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        let s = url.as_str();
        let mut r = self.client.get(s);
        let mut h: HashMap<String, String> = HashMap::new();
        let mut domain: Option<String> = None;
        let u = url.into_url().unwrap();
        match u.host_str() {
            Some(hs) => {
                domain = Some(String::from(hs));
            }
            None => {}
        }
        let upath = u.path();
        for (_, val) in self.jar.iter() {
            match val.domain() {
                Some(dm) => match &domain {
                    Some(url_dm) => {
                        if !dm.starts_with(".") {
                            if url_dm != dm {
                                continue;
                            }
                        } else {
                            let dmm = dm.strip_prefix(".").unwrap();
                            if !url_dm.ends_with(dmm) {
                                continue;
                            }
                        }
                    }
                    None => {
                        continue;
                    }
                },
                None => {}
            }
            match val.path() {
                Some(pt) => {
                    if !upath.starts_with(pt) {
                        continue;
                    }
                }
                None => {}
            }
            h.insert(String::from(val.name()), String::from(val.value()));
        }
        let mut cs = String::from("");
        for (k, v) in h.iter() {
            let s = format!("{}={}", k, v);
            if cs.len() > 0 {
                cs += "; ";
            }
            cs += s.as_str();
        }
        r = r.header("Cookie", cs);
        r
    }
}

impl Clone for CookieClient {
    fn clone(&self) -> CookieClient {
        CookieClient {
            client: self.client.clone(),
            jar: self.jar.clone(),
        }
    }
}
