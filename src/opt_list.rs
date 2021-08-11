use crate::getopt::OptDes;
use crate::i18n::gettext;

pub fn get_opt_list() -> Vec<OptDes> {
    vec![
        OptDes::new("cookies", None, gettext("The location of cookies file. Default: \"bili.cookies.json\" in executable's path."), true, true, Some("path")).unwrap(),
        OptDes::new("cookie-jar", Some("j"), gettext("The name of cookie jar which cookies will be stored."), true, true, Some("name")).unwrap(),
        OptDes::new("help", Some("h"), gettext("Print help message"), true, false, Some("full|provider name")).unwrap(),
        OptDes::new("login", None, gettext("If not logined, force to login."), false, false, None).unwrap(),
        OptDes::new("version", Some("V"), gettext("Print version of bili"), false, false, None).unwrap(),
    ]
}
