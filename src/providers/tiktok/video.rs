extern crate regex;

use crate::cookies_json::CookiesJar;
use crate::getopt::OptStore;
use crate::providers::provider_base::Provider;
use crate::providers::tiktok::base::TiktokBaseProvider;
use crate::settings::SettingStore;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*tiktok\.com/@(?P<name>[^\\/?]+)/video/(?P<id>\d+)(\?.*)?$").unwrap();
}

pub struct TiktokVideoProvider {
    base: TiktokBaseProvider,
}

impl Provider for TiktokVideoProvider {
    fn new() -> Self {
        Self {
            base: TiktokBaseProvider::new(),
        }
    }

    fn init(&mut self, jar: Option<&CookiesJar>, _opt: OptStore, _settings: SettingStore) -> bool {
        if !self.base.init_client(jar) {
            return false;
        }
        true
    }

    fn match_url(url: &str) -> bool {
        let r = RE.captures(url);
        r.is_some()
    }

    fn provider_name(&self) -> &'static str {
        "TiktokVideoProvider"
    }
}
