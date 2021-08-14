extern crate json;

use crate::getopt::OptDes;
use crate::i18n::gettext;
use crate::settings::JsonValueType;
use crate::settings::SettingDes;
use json::JsonValue;

/// If value is positive, return true.
fn check_positive(value: JsonValue) -> bool {
    let a = value.as_u64();
    if a.is_none() {
        false
    } else {
        let a = a.unwrap();
        if a == 0 {
            false
        } else {
            true
        }
    }
}

pub fn get_bili_normal_video_options() -> Vec<OptDes> {
    vec![OptDes::new("part", Some("p"), gettext("The video part number of a page."), true, true, Some("part number")).unwrap()]
}

pub fn get_bili_normal_video_settings() -> Vec<SettingDes> {
    vec![SettingDes::new("part", gettext("The video part number of a page."), JsonValueType::Number, Some(check_positive)).unwrap()]
}
