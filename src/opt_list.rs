use crate::downloader::aria2c::check_file_allocation;
use crate::downloader::aria2c::check_max_connection_per_server;
use crate::downloader::aria2c::check_min_split_size;
use crate::downloader::aria2c::check_split;
use crate::getopt::OptDes;
use crate::settings::JsonValueType;
use crate::settings::SettingDes;
use crate::i18n::gettext;

pub fn get_config_opt_list() -> Vec<OptDes> {
    vec![
        OptDes::new("config", Some("c"), gettext("The location of settings file. Default: bili.settings.json"), true, true, Some("path")).unwrap(),
        OptDes::new("fix", None, gettext("Ignore invalid value when reading file"), false, false, None).unwrap(),
        OptDes::new("force", Some("f"), gettext("Overwrite exists value."), false, false, None).unwrap(),
        OptDes::new("help", Some("h"), gettext("Print help message"), false, false, None).unwrap(),
        OptDes::new("str", Some("s"), gettext("Treat value as string"), false, false, None).unwrap(),
    ]
}

pub fn get_cookie_opt_list() -> Vec<OptDes> {
    vec![
        OptDes::new("cookies", Some("c"), gettext("The location of cookies file. Default: \"bili.cookies.json\" in executable's path."), true, true, Some("path")).unwrap(),
        OptDes::new("help", Some("h"), gettext("Print help message"), false, false, None).unwrap(),
    ]
}

pub fn get_opt_list() -> Vec<OptDes> {
    vec![
        OptDes::new("aria2c", None, gettext("Whether to enable arai2c."), true, true, Some("boolean")).unwrap(),
        OptDes::new("aria2c-file-allocation", None, gettext("The file allocation method used by aria2c. Available value: none, prealloc, trunc, falloc."), true, true, Some("METHOD")).unwrap(),
        OptDes::new("aria2c-max-connection-per-server", None, gettext("The maximum number of connections to one server for each download when using aria2c to download."), true, true, Some("NUM")).unwrap(),
        OptDes::new("aria2c-min-split-size", None, gettext("Let aria2 does not split less than 2*SIZE byte range."), true, true, Some("SIZE")).unwrap(),
        OptDes::new("aria2c-split", None, gettext("The number of connections used when downloading a file."), true, true, Some("N")).unwrap(),
        OptDes::new("config", Some("c"), gettext("The location of settings file. Default: bili.settings.json"), true, true, Some("path")).unwrap(),
        OptDes::new("cookies", None, gettext("The location of cookies file. Default: \"bili.cookies.json\" in executable's path."), true, true, Some("path")).unwrap(),
        OptDes::new("cookie-jar", Some("j"), gettext("The name of cookie jar which cookies will be stored."), true, true, Some("name")).unwrap(),
        OptDes::new("help", Some("h"), gettext("Print help message"), true, false, Some("full|provider name")).unwrap(),
        OptDes::new("help-deps", None, gettext("Print all options/settings which provider depended on. Exclude basic options"), false, false, None).unwrap(),
        OptDes::new("help-settings", None, gettext("Print all settings"), true, false, Some("full|provider name")).unwrap(),
        OptDes::new("list-providers-only", None, gettext("List only providers name when print help message"), false, false, None).unwrap(),
        OptDes::new("login", None, gettext("If not logined, force to login."), false, false, None).unwrap(),
        OptDes::new("version", Some("V"), gettext("Print version of bili"), false, false, None).unwrap(),
    ]
}

pub fn get_settings_list() -> Vec<SettingDes> {
    vec![
        SettingDes::new("aria2c", gettext("Whether to enable arai2c."), JsonValueType::Boolean, None).unwrap(),
        SettingDes::new("aria2c-file-allocation", gettext("The file allocation method used by aria2c. Available value: none, prealloc, trunc, falloc."), JsonValueType::Str, Some(check_file_allocation)).unwrap(),
        SettingDes::new("aria2c-max-connection-per-server", gettext("The maximum number of connections to one server for each download when using aria2c to download."), JsonValueType::Multiple, Some(check_max_connection_per_server)).unwrap(),
        SettingDes::new("aria2c-min-split-size", gettext("Let aria2 does not split less than 2*SIZE byte range."), JsonValueType::Multiple, Some(check_min_split_size)).unwrap(),
        SettingDes::new("aria2c-split", gettext("The number of connections used when downloading a file."), JsonValueType::Multiple, Some(check_split)).unwrap(),
        SettingDes::new("cookies", gettext("The location of cookies file. Default: \"bili.cookies.json\" in executable's path."), JsonValueType::Str, None).unwrap(),
    ]
}

pub fn get_webdriver_options() -> Vec<OptDes> {
    vec![
        OptDes::new("chrome", None, gettext("Start browser with chromedriver"), false, false, None).unwrap(),
        OptDes::new("chromedriver", None, gettext("The location of the chromedriver executable"), true, true, Some("location")).unwrap(),
        OptDes::new("chromedriver-server", None, gettext("The location of the chromedriver server. Such as http://locahost:4444"), true, true, Some("url")).unwrap(),
    ]
}

pub fn get_webdriver_settings() -> Vec<SettingDes> {
    vec![
        SettingDes::new("chrome", gettext("Start browser with chromedriver"), JsonValueType::Boolean, None).unwrap(),
    ]
}
