use crate::getopt::OptDes;
use crate::i18n::gettext;

pub fn get_opt_list() -> Vec<OptDes> {
    vec![
        OptDes::new("cookies", None, gettext("The location of cookies file. Default: \"bili.cookies.json\" in executable's path."), true, true, Some("path")).unwrap(),
        OptDes::new("cookie-jar", Some("j"), gettext("The name of cookie jar which cookies will be stored."), true, true, Some("name")).unwrap(),
        OptDes::new("help", Some("h"), gettext("Print help message"), true, false, Some("full|provider name")).unwrap(),
        OptDes::new("help-deps", None, gettext("Print all options which provider depended on. Exclude basic options"), false, false, None).unwrap(),
        OptDes::new("login", None, gettext("If not logined, force to login."), false, false, None).unwrap(),
        OptDes::new("version", Some("V"), gettext("Print version of bili"), false, false, None).unwrap(),
    ]
}

pub fn get_webdriver_options() -> Vec<OptDes> {
    vec![
        OptDes::new("chrome", None, gettext("Start browser with chromedriver"), false, false, None).unwrap(),
        OptDes::new("chromedriver", None, gettext("The location of the chromedriver executable"), true, true, Some("location")).unwrap(),
        OptDes::new("chromedriver-server", None, gettext("The location of the chromedriver server. Such as http://locahost:4444"), true, true, Some("url")).unwrap(),
    ]
}
