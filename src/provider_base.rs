use crate::cookies_json::CookiesJar;

pub trait Provider {
    fn new(jar: Option<&CookiesJar>) -> Self;
    fn can_login(&self) -> bool {
        return false;
    }
    fn check_logined(&self) -> Option<bool> {
        return Some(false);
    }
    fn login(&self, _jar: &mut CookiesJar) -> bool {
        return false;
    }
    fn match_url(_url: &str) -> bool {
        return false;
    }
}
