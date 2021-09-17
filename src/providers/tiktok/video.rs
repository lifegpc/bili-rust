extern crate futures;
extern crate json;
extern crate regex;

use crate::cookies_json::CookiesJar;
use crate::getopt::OptStore;
use crate::i18n::gettext;
use crate::metadata::ExtractInfo;
use crate::providers::provider_base::Provider;
use crate::providers::tiktok::base::TiktokBaseProvider;
use crate::settings::SettingStore;
use futures::executor::block_on;
use json::JsonValue;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*tiktok\.com/@(?P<name>[^\\/?]+)/video/(?P<id>\d+)(\?.*)?$").unwrap();
    static ref RE2: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*tiktok\.com/i18n/share/video/(?P<id>\d+)(\?.*)?$").unwrap();
}

pub struct TiktokVideoProvider {
    base: TiktokBaseProvider,
    /// Optional. User name
    user_name: Option<String>,
    /// Used to extract video information.
    video_id: Option<String>,
    /// Video information extracted from page.
    video_info: Option<JsonValue>,
}

impl TiktokVideoProvider {
    fn get_info(&mut self) -> bool {
        if self.video_id.is_none() {
            return false;
        }
        let url = if self.user_name.is_none() {
            format!(
                "https://t.tiktok.com/i18n/share/video/{}",
                self.video_id.as_ref().unwrap(),
            )
        } else {
            format!(
                "https://www.tiktok.com/@{}/video/{}",
                self.user_name.as_ref().unwrap(),
                self.video_id.as_ref().unwrap()
            )
        };
        let c = self.base.client.as_mut().unwrap();
        let r = c.get(url);
        if r.is_none() {
            return false;
        }
        let r = r.unwrap();
        if r.url().as_str() == "https://www.tiktok.com/hk/notfound" {
            println!("{}", gettext("Hong Kong was blocked by tiktok."));
            return false;
        }
        if r.status().as_u16() >= 400 {
            println!("{}{}", gettext("Can not get video page: "), r.status());
            return false;
        }
        let t = block_on(r.text_with_charset("UTF8"));
        if t.is_err() {
            println!("{}{}", gettext("Can not get video page: "), t.unwrap_err());
            return false;
        }
        let t = t.unwrap();
        let re = self.base.extract_info(t.as_str());
        if re.is_none() {
            println!("{}", gettext("Can not get video information from page."));
            return false;
        }
        let re = re.unwrap();
        let j = json::parse(re.as_str());
        if j.is_err() {
            println!(
                "{}{}",
                gettext("Can not parse video infomation: "),
                j.unwrap_err()
            );
            return false;
        }
        self.video_info = Some(j.unwrap());
        true
    }
}

impl Provider for TiktokVideoProvider {
    fn new() -> Self {
        Self {
            base: TiktokBaseProvider::new(),
            user_name: None,
            video_id: None,
            video_info: None,
        }
    }

    fn extract(&mut self, url: &str) -> Option<ExtractInfo> {
        let r = RE.captures(url);
        let r2 = RE2.captures(url);
        if r.is_none() && r2.is_none() {
            return None;
        }
        if r.is_some() {
            let r = r.unwrap();
            let un = r.name("name").unwrap();
            self.user_name = Some(String::from(un.as_str()));
            let vid = r.name("id").unwrap();
            self.video_id = Some(String::from(vid.as_str()));
        } else if r2.is_some() {
            let r2 = r2.unwrap();
            let vid = r2.name("id").unwrap();
            self.video_id = Some(String::from(vid.as_str()));
        }
        if !self.get_info() {
            return None;
        }
        println!("{}", self.video_info.as_ref().unwrap().pretty(2));
        None
    }

    fn init(&mut self, jar: Option<&CookiesJar>, _opt: OptStore, _settings: SettingStore) -> bool {
        if !self.base.init_client(jar) {
            return false;
        }
        true
    }

    fn match_url(url: &str) -> bool {
        let r = RE.captures(url);
        let r2 = RE2.captures(url);
        r.is_some() || r2.is_some()
    }

    fn provider_name(&self) -> &'static str {
        "TiktokVideoProvider"
    }
}
