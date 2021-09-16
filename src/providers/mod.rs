pub mod bilibili;
pub mod provider_base;
pub mod tiktok;

use crate::getopt::OptStore;
use crate::opt_list::get_webdriver_options;
use crate::opt_list::get_webdriver_settings;
use crate::settings::SettingStore;
use bilibili::normal_video::BiliNormalVideoProvider;
use provider_base::Provider;

pub fn add_all_opts(opt: &mut OptStore) {
    opt.add("WebDriver", get_webdriver_options());
    opt.add_with_dependence(
        "BiliNormalVideoProvider",
        BiliNormalVideoProvider::get_custom_options(),
        vec!["WebDriver"],
    );
}

pub fn add_all_settings(store: &mut SettingStore) {
    store.add("WebDriver", get_webdriver_settings());
    store.add_with_dependence(
        "BiliNormalVideoProvider",
        BiliNormalVideoProvider::get_custom_settings(),
        vec!["WebDriver"],
    );
}
