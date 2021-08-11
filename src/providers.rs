use crate::bilibili::base::BiliBaseProvider;
use crate::getopt::OptStore;
use crate::provider_base::Provider;

pub fn match_provider(url: &str) -> Option<impl Provider> {
    if BiliBaseProvider::match_url(url) {
        return Some(BiliBaseProvider::new());
    }
    return None;
}

pub fn add_all_opts(opt: &mut OptStore) {
    opt.add("BiliBaseProvider", BiliBaseProvider::get_custom_options());
}
