extern crate json;

use crate::getopt::OptDes;
use crate::i18n::gettext;
use crate::providers::bilibili::part::PartList;
use crate::settings::JsonValueType;
use crate::settings::SettingDes;
use json::JsonValue;

fn check_part(value: JsonValue) -> bool {
    let re = PartList::parse_from_json(value);
    match re {
        Some(_) => true,
        None => false,
    }
}

pub fn get_bili_normal_video_options() -> Vec<OptDes> {
    vec![OptDes::new("part", Some("p"), gettext("The video part number of a page."), true, true, Some("part number")).unwrap()]
}

pub fn get_bili_normal_video_settings() -> Vec<SettingDes> {
    vec![SettingDes::new("part", gettext("The video part number of a page.\nExample: \n2\tSelect part 2\n\"2-34\"\tSelect from part 2 to part 34.\n\"3, 5-10\" or [3, \"5-10\"]\tSelect part 3 and from part 5 to part 10.\n\"3-\"\tSelect from part 3 to last part.\n\"-10\"\tSelect from first part to part 10.\n\"-\"\tSelect all parts."), JsonValueType::Multiple, Some(check_part)).unwrap()]
}
