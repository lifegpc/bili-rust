extern crate futures;
extern crate regex;

use crate::cookies_json::CookiesJar;
use crate::getopt::OptDes;
use crate::getopt::OptStore;
use crate::i18n::gettext;
use crate::metadata::ExtractInfo;
use crate::providers::bilibili::base::BiliBaseProvider;
use crate::providers::bilibili::opt_list::get_bili_normal_video_options;
use crate::providers::bilibili::opt_list::get_bili_normal_video_settings;
use crate::providers::bilibili::parser::HTMLDataInJS;
use crate::providers::bilibili::util;
use crate::providers::provider_base::Provider;
use crate::settings::SettingDes;
use crate::settings::SettingStore;
use futures::executor::block_on;
use regex::Regex;
use std::clone::Clone;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)^(av(?P<av>\d+)|(?P<bv>bv[a-z0-9]{9,10}))$").unwrap();
    static ref RE2: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*bilibili\.com(?:/s)?/video/(av(?P<av>\d+)|(?P<bv>bv[a-z0-9]{9,10}))(\?.*?p=(?P<part>[^&]+)&?.*)?$").unwrap();
    static ref RE3: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*b23\.tv/(av(?P<av>\d+)|(?P<bv>bv[a-z0-9]{9,10}))(\?.*?p=(?P<part>[^&]+)&?.*)?$").unwrap();
}

#[derive(Debug, PartialEq)]
/// Information from video url
pub struct UrlInfo {
    /// AV number
    pub av: usize,
    /// BV number
    pub bv: String,
    /// Part number
    pub part: Option<usize>,
}

impl UrlInfo {
    /// Create from AV number
    /// * `av` - AV number
    /// * `part` - Part number
    pub fn from_av(av: usize, part: Option<usize>) -> UrlInfo {
        UrlInfo {
            av,
            bv: util::av_to_bv(av),
            part,
        }
    }

    pub fn from_bv(bv: String, part: Option<usize>) -> UrlInfo {
        UrlInfo {
            av: util::bv_to_av(bv.clone()),
            bv,
            part,
        }
    }
}

impl Clone for UrlInfo {
    fn clone(&self) -> UrlInfo {
        UrlInfo {
            av: self.av.clone(),
            bv: self.bv.clone(),
            part: self.part.clone(),
        }
    }
}

pub struct BiliNormalVideoProvider {
    base: BiliBaseProvider,
}

impl BiliNormalVideoProvider {
    fn basic_info(&self, url: UrlInfo) -> bool {
        const PLAYERINFO: &str = "window.__playinfo__";
        const INITIAL: &str = "window.__INITIAL_STATE__";
        let link = format!("https://www.bilibili.com/video/{}", url.bv);
        let r = self.base.client.as_ref().unwrap().get(link);
        match r {
            Some(_) => {}
            None => return false,
        }
        let r = r.unwrap();
        if r.status().as_u16() >= 400 {
            println!(
                "{}\"{}\"",
                gettext("Error when geting the webpage: "),
                r.status().as_str()
            );
            return false;
        }
        let t = block_on(r.text_with_charset("UTF-8"));
        match t {
            Ok(_) => {}
            Err(e) => {
                println!("{}\"{}\"", gettext("Error when geting the webpage: "), e);
                return false;
            }
        }
        let t = t.unwrap();
        let mut js = HTMLDataInJS::new();
        if !js.parse(t.as_str(), vec![PLAYERINFO, INITIAL]) {
            return false;
        }
        true
    }

    fn parse_url(url: &str) -> Option<UrlInfo> {
        let caps = RE.captures(url);
        match caps {
            Some(caps) => {
                let av = caps.name("av");
                let bv = caps.name("bv");
                if av.is_none() {
                    let bv = bv.unwrap();
                    return Some(UrlInfo::from_bv(String::from(bv.as_str()), None));
                } else {
                    let av = av.unwrap().as_str();
                    let av = av.parse::<usize>();
                    match av {
                        Ok(av) => {
                            return Some(UrlInfo::from_av(av, None));
                        }
                        Err(_) => {
                            println!("{}", gettext("AV number is too big."));
                            return None;
                        }
                    }
                }
            }
            None => {}
        }
        for i in 0..2 {
            let re = if i == 0 { &*RE2 } else { &*RE3 };
            let caps = re.captures(url);
            match caps {
                Some(caps) => {
                    let av = caps.name("av");
                    let bv = caps.name("bv");
                    let part = caps.name("part");
                    let mut p: Option<usize> = None;
                    if !part.is_none() {
                        let part = part.unwrap().as_str();
                        p = util::atou(part);
                    }
                    if av.is_none() {
                        let bv = bv.unwrap();
                        return Some(UrlInfo::from_bv(String::from(bv.as_str()), p));
                    } else {
                        let av = av.unwrap().as_str();
                        let av = av.parse::<usize>();
                        match av {
                            Ok(av) => {
                                return Some(UrlInfo::from_av(av, p));
                            }
                            Err(_) => {
                                println!("{}", gettext("AV number is too big."));
                                return None;
                            }
                        }
                    }
                }
                None => {}
            }
        }
        None
    }
}

impl Provider for BiliNormalVideoProvider {
    fn new() -> BiliNormalVideoProvider {
        BiliNormalVideoProvider {
            base: BiliBaseProvider::new(),
        }
    }

    fn add_custom_options(&self, opt: &mut OptStore) {
        self.base.add_custom_options(opt);
        opt.add(self.provider_name(), get_bili_normal_video_options());
    }

    fn add_custom_settings(&self, store: &mut SettingStore) {
        self.base.add_custom_settings(store);
        store.add(self.provider_name(), get_bili_normal_video_settings());
    }

    fn can_login(&self) -> bool {
        true
    }

    fn check_logined(&mut self) -> Option<bool> {
        self.base.check_logined()
    }

    fn extract(&mut self, url: &str) -> Option<ExtractInfo> {
        let u = Self::parse_url(url).unwrap();
        if !self.basic_info(u) {
            return None;
        }
        None
    }

    fn get_custom_options() -> Vec<OptDes> {
        get_bili_normal_video_options()
    }

    fn get_custom_settings() -> Vec<SettingDes> {
        get_bili_normal_video_settings()
    }

    fn get_default_cookie_jar_name(&self) -> Option<&str> {
        Some("bili")
    }

    fn has_custom_options(&self) -> bool {
        true
    }

    fn has_custom_settings(&self) -> bool {
        true
    }

    fn init(&mut self, jar: Option<&CookiesJar>, opt: OptStore, settings: SettingStore) -> bool {
        self.base.init(jar, opt, settings)
    }

    fn login(&mut self, jar: &mut CookiesJar) -> bool {
        self.base.login(jar)
    }

    fn logined(&self) -> bool {
        self.base.logined()
    }

    fn match_url(url: &str) -> bool {
        let le = BiliNormalVideoProvider::parse_url(url);
        if le.is_none() {
            false
        } else {
            true
        }
    }

    fn provider_name(&self) -> &'static str {
        "BiliNormalVideoProvider"
    }
}

#[test]
fn test_parse_url() {
    assert_eq!(
        Some(UrlInfo {
            av: 170001,
            bv: String::from("BV17x411w7KC"),
            part: None
        }),
        BiliNormalVideoProvider::parse_url("av170001")
    );
    assert_eq!(
        Some(UrlInfo {
            av: 9,
            bv: String::from("BV1xx411c7mC"),
            part: None
        }),
        BiliNormalVideoProvider::parse_url("BV1xx411c7mC")
    );
    assert_eq!(None, BiliNormalVideoProvider::parse_url("BV2331"));
    assert_eq!(
        Some(UrlInfo {
            av: 207281893,
            bv: String::from("BV1nh411B7G5"),
            part: None
        }),
        BiliNormalVideoProvider::parse_url("https://www.bilibili.com/video/BV1nh411B7G5")
    );
    assert_eq!(
        Some(UrlInfo {
            av: 170001,
            bv: String::from("BV17x411w7KC"),
            part: Some(2)
        }),
        BiliNormalVideoProvider::parse_url("https://www.bilibili.com/video/av170001?test3&p=2&d")
    );
    assert_eq!(
        Some(UrlInfo {
            av: 170001,
            bv: String::from("BV17x411w7KC"),
            part: None
        }),
        BiliNormalVideoProvider::parse_url("https://www.bilibili.com/video/av170001?test3&p=3f&d")
    );
    assert_eq!(
        Some(UrlInfo {
            av: 170001,
            bv: String::from("BV17x411w7KC"),
            part: Some(2)
        }),
        BiliNormalVideoProvider::parse_url("https://b23.tv/av170001?test3&p=2&d")
    );
}
