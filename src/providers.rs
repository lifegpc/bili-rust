use crate::bilibili::base::BiliBaseProvider;
use crate::provider_base::Provider;

pub fn match_provider(url: &str) -> Option<impl Provider> {
    if BiliBaseProvider::match_url(url) {
        return Some(BiliBaseProvider::new());
    }
    return None;
}
