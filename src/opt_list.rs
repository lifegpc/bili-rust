use crate::getopt::OptDes;
use crate::i18n::gettext;

pub fn get_opt_list() -> Vec<OptDes> {
    vec![
        OptDes::new("help", Some("h"), gettext("Print help message"), true, false, Some("full|provider name")).unwrap(),
        OptDes::new("version", Some("V"), gettext("Print version of bili"), false, false, None).unwrap(),
    ]
}
