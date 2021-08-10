use crate::bilibili::base::BiliBaseProvider;
use crate::cookies_json::CookiesJson;
use crate::provider_base::Provider;

pub fn match_provider(url: &str, js: &CookiesJson) -> Option<impl Provider> {
    if BiliBaseProvider::match_url(url) {
        let jar = js.get("bili");
        return Some(BiliBaseProvider::new(jar));
    }
    return None;
}
