use crate::getopt::OptDes;
use crate::i18n::gettext;

pub fn get_bili_base_options() -> Vec<OptDes> {
    vec![OptDes::new("part", Some("p"), gettext("The video part number of a page."), true, true, Some("part number")).unwrap()]
}
