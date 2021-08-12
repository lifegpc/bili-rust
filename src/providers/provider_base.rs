use crate::cookies_json::CookiesJar;
use crate::getopt::OptDes;
use crate::getopt::OptStore;
use crate::settings::SettingDes;
use crate::settings::SettingStore;

pub trait Provider {
    fn new() -> Self;
    fn add_custom_options(&self, _opt: &mut OptStore) {}
    fn add_custom_settings(&self, _store: &mut SettingStore) {}
    fn can_login(&self) -> bool {
        false
    }
    fn check_logined(&mut self) -> Option<bool> {
        Some(false)
    }
    fn get_custom_options() -> Vec<OptDes> {
        [].to_vec()
    }
    fn get_custom_settings() -> Vec<SettingDes> {
        [].to_vec()
    }
    fn get_default_cookie_jar_name(&self) -> Option<&str> {
        None
    }
    fn has_custom_options(&self) -> bool {
        false
    }
    fn has_custom_settings(&self) -> bool {
        false
    }
    fn init(&mut self, _jar: Option<&CookiesJar>, _opt: OptStore, _settings: SettingStore) -> bool {
        false
    }
    fn login(&mut self, _jar: &mut CookiesJar) -> bool {
        false
    }
    fn logined(&self) -> bool {
        false
    }
    fn login_required(&self) -> bool {
        false
    }
    fn match_url(_url: &str) -> bool {
        false
    }
    fn provider_name(&self) -> &'static str;
}
