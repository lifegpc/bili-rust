use crate::cookies_json::CookiesJar;

pub trait Provider {
    fn new() -> Self;
    fn can_login(&self) -> bool {
        false
    }
    fn check_logined(&self) -> Option<bool> {
        Some(false)
    }
    fn get_default_cookie_jar_name(&self) -> Option<&str> {
        None
    }
    fn init(&mut self, _jar: Option<&CookiesJar>) -> bool {
        false
    }
    fn login(&self, _jar: &mut CookiesJar) -> bool {
        false
    }
    fn match_url(_url: &str) -> bool {
        false
    }
}
