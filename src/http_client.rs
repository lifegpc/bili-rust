extern crate futures;
extern crate json;
extern crate reqwest;

use crate::cookies_json::Cookie;
use crate::cookies_json::CookiesJar;
use crate::i18n::gettext;
use futures::executor::block_on;
use json::JsonValue;
use reqwest::Client;
use reqwest::IntoUrl;
use reqwest::RequestBuilder;
use reqwest::Response;
use std::clone::Clone;
use std::collections::HashMap;

/// A HTTP Client which support send Cookie
pub struct CookieClient {
    /// HTTP Client
    client: Client,
    /// Cookies
    jar: CookiesJar,
    /// Enable handle set-cookie HTTP header
    set_cookie: bool,
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
            set_cookie: false,
        }
    }

    pub fn enable_set_cookie(&mut self) {
        self.set_cookie = true;
    }

    pub fn handle_set_cookie(&mut self, r: &Response) {
        let u = r.url();
        let h = r.headers();
        let v = h.get_all("Set-Cookie");
        for val in v {
            let val = val.to_str();
            match val {
                Ok(v) => {
                    let c = Cookie::from_set_cookie(v);
                    if c.is_some() {
                        let mut c = c.unwrap();
                        let dms = c.domain();
                        if dms.is_none() {
                            let dm = u.domain();
                            c.set_domain(dm);
                        }
                        let path = c.path();
                        if path.is_none() {
                            let pa = u.path();
                            c.set_path(Some(pa));
                        }
                        self.jar.add(c);
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }

    /// Send GET requests with parameters
    /// * `param` - GET parameters. Should be a JSON object/array. If value in map is not a string, will dump it
    /// # Examples
    /// ```
    /// let c = Client::builder().build().unwrap();
    /// let client = CookieClient::new(c, None);
    /// client.get_with_param("https://test.com/a", json::object!{"data": "param1"});
    /// client.get_with_param("https://test.com/a", json::object!{"daa": {"ad": "test"}});
    /// client.get_with_param("https://test.com/a", json::array![["daa", "param1"]]);
    /// ```
    /// It will GET `https://test.com/a?data=param1`, `https://test.com/a?daa=%7B%22ad%22%3A%22test%22%7D`, `https://test.com/a?daa=param1`
    pub fn get_with_param<U: IntoUrl>(&mut self, url: U, param: JsonValue) -> Option<Response> {
        let u = url.into_url();
        if u.is_err() {
            println!("{}\"{}\"", gettext("Can not parse URL: "), u.unwrap_err());
            return None;
        }
        let mut u = u.unwrap();
        if !param.is_object() && !param.is_array() {
            println!(
                "{}\"{}\"",
                gettext("Parameters should be object or array: "),
                param
            );
            return None;
        }
        {
            let mut query = u.query_pairs_mut();
            if param.is_object() {
                for (k, v) in param.entries() {
                    let s: String;
                    if v.is_string() {
                        s = String::from(v.as_str().unwrap());
                    } else {
                        s = v.dump();
                    }
                    query.append_pair(k, s.as_str());
                }
            } else {
                for v in param.members() {
                    if !v.is_object() {
                        println!("{}\"{}\"", gettext("Parameters should be array: "), v);
                        return None;
                    }
                    if v.len() < 2 {
                        println!("{}\"{}\"", gettext("Parameters need at least a value: "), v);
                        return None;
                    }
                    let okey = &v[0];
                    let key: String;
                    if okey.is_string() {
                        key = String::from(okey.as_str().unwrap());
                    } else {
                        key = okey.dump();
                    }
                    let mut mems = v.members();
                    mems.next();
                    for val in mems {
                        let s: String;
                        if val.is_string() {
                            s = String::from(val.as_str().unwrap());
                        } else {
                            s = val.dump();
                        }
                        query.append_pair(key.as_str(), s.as_str());
                    }
                }
            }
        }
        self.get(u.as_str())
    }

    /// Send GET requests
    pub fn get<U: IntoUrl>(&mut self, url: U) -> Option<Response> {
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
        let r = r.unwrap();
        if self.set_cookie {
            self.handle_set_cookie(&r);
        }
        Some(r)
    }

    /// Get a request builder which contains cookies information.
    /// # Notes
    /// You need call `handle_set_cookie` to handle `set-cookie` headers in response.
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

    pub fn set_cookies_jar(&mut self, jar: CookiesJar) {
        self.jar = jar.clone();
    }

    /// Get cookie's value.
    /// * key - cookie's name
    pub fn get_cookie(&self, key: &str) -> Option<String> {
        let c = self.jar.cookies.get(key);
        if !c.is_none() {
            let c = c.unwrap();
            return Some(String::from(c.value()));
        }
        None
    }
}

impl Clone for CookieClient {
    fn clone(&self) -> CookieClient {
        CookieClient {
            client: self.client.clone(),
            jar: self.jar.clone(),
            set_cookie: self.set_cookie.clone(),
        }
    }
}
