extern crate chrono;
extern crate futures;
extern crate json;
extern crate regex;

use crate::cookies_json::CookiesJar;
use crate::getopt::OptDes;
use crate::getopt::OptStore;
use crate::i18n::gettext;
use crate::metadata::ExtractInfo;
use crate::metadata::NoInTotal;
use crate::metadata::VideoMetadata;
use crate::providers::bilibili::base::BiliBaseProvider;
use crate::providers::bilibili::interaction::InteractionVideoParser;
use crate::providers::bilibili::opt_list::get_bili_normal_video_options;
use crate::providers::bilibili::opt_list::get_bili_normal_video_settings;
use crate::providers::bilibili::parser::HTMLDataInJS;
use crate::providers::bilibili::part_info::PartInfoList;
use crate::providers::bilibili::util;
use crate::providers::provider_base::Provider;
use crate::settings::SettingDes;
use crate::settings::SettingStore;
use chrono::TimeZone;
use chrono::Utc;
use futures::executor::block_on;
use json::JsonValue;
use regex::Regex;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::TryFrom;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)^(av(?P<av>\d+)|(?P<bv>bv[a-z0-9]{9,10}))$").unwrap();
    static ref RE2: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*bilibili\.com(?:/s)?/video/(av(?P<av>\d+)|(?P<bv>bv[a-z0-9]{9,10}))(\?.*?p=(?P<part>[^&]+)&?.*)?$").unwrap();
    static ref RE3: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*b23\.tv/(av(?P<av>\d+)|(?P<bv>bv[a-z0-9]{9,10}))(\?.*?p=(?P<part>[^&]+)&?.*)?$").unwrap();
}

#[derive(Debug, PartialEq)]
/// Information from video url
pub struct UrlInfo {
    /// AV number
    pub av: usize,
    /// BV number
    pub bv: String,
    /// Part number
    pub part: Option<usize>,
}

impl UrlInfo {
    /// Create from AV number
    /// * `av` - AV number
    /// * `part` - Part number
    pub fn from_av(av: usize, part: Option<usize>) -> UrlInfo {
        UrlInfo {
            av,
            bv: util::av_to_bv(av),
            part,
        }
    }

    pub fn from_bv(bv: String, part: Option<usize>) -> UrlInfo {
        UrlInfo {
            av: util::bv_to_av(bv.clone()),
            bv,
            part,
        }
    }
}

impl Clone for UrlInfo {
    fn clone(&self) -> UrlInfo {
        UrlInfo {
            av: self.av.clone(),
            bv: self.bv.clone(),
            part: self.part.clone(),
        }
    }
}

pub struct BiliNormalVideoProvider {
    base: BiliBaseProvider,
    /// Basic information extract from HTML (`window.__INITIAL_STATE__`)
    videoinfo: Option<JsonValue>,
    /// Player url extract from HTML (`window.__playinfo__`)
    playinfo: Option<JsonValue>,
    /// Part list (from videoinfo or API)
    partinfo: Option<PartInfoList>,
    /// Information from API (`https://api.bilibili.com/x/player/v2`)
    cidinfo: HashMap<usize, JsonValue>,
    /// Input Url Information (Set in [`basic_info`](#method.basic_info) function)
    url: Option<UrlInfo>,
}

impl BiliNormalVideoProvider {
    /// Extract basic information
    /// * `url` - Input url
    /// return true if `videoinfo` is ok
    fn basic_info(&mut self, url: UrlInfo) -> bool {
        const PLAYERINFO: &str = "window.__playinfo__";
        const INITIAL: &str = "window.__INITIAL_STATE__";
        self.url = Some(url.clone());
        let link = format!("https://www.bilibili.com/video/{}", url.bv);
        {
            let c = self.base.client.as_ref().unwrap();
            let r = c.get(link);
            match r {
                Some(_) => {}
                None => return false,
            }
            let r = r.unwrap();
            if r.status().as_u16() >= 400 {
                println!(
                    "{}\"{}\"",
                    gettext("Error when geting the webpage: "),
                    r.status().as_str()
                );
                return false;
            }
            let t = block_on(r.text_with_charset("UTF-8"));
            match t {
                Ok(_) => {}
                Err(e) => {
                    println!("{}\"{}\"", gettext("Error when geting the webpage: "), e);
                    return false;
                }
            }
            let t = t.unwrap();
            let mut js = HTMLDataInJS::new();
            if !js.parse(t.as_str(), vec![PLAYERINFO, INITIAL]) {
                return false;
            }
            let data = js.maps.get(INITIAL);
            if data.is_none() {
                return false;
            }
            let data = data.unwrap();
            let dat = &data[..data.len() - 122];
            let data = json::parse(dat);
            match data {
                Ok(_) => {}
                Err(e) => {
                    println!("{}\"{}\"", gettext("Can not parse as JSON: "), e);
                    return false;
                }
            }
            self.videoinfo = Some(data.unwrap());
            let pinfo = js.maps.get(PLAYERINFO);
            if !pinfo.is_none() {
                let pinfo = pinfo.unwrap();
                let pinfo = json::parse(pinfo.as_str());
                match pinfo {
                    Ok(pinfo) => {
                        self.playinfo = Some(pinfo);
                    }
                    Err(e) => {
                        println!("{}\"{}\"", gettext("Can not parse as JSON: "), e);
                    }
                }
            }
            let pages = &self.videoinfo.as_ref().unwrap()["videoData"]["pages"];
            let pl = PartInfoList::try_from(pages);
            if pl.is_err() {
                let re = c.get_with_param(
                    "https://api.bilibili.com/x/player/pagelist",
                    json::object! {"bvid": url.bv.clone(), "jsonp": "jsonp"},
                );
                if re.is_none() {
                    println!("{}", gettext("Can not get page list."));
                    return false;
                }
                let re = re.unwrap();
                if re.status().as_u16() >= 400 {
                    println!("{}\n{}", gettext("Can not get page list."), re.status());
                    return false;
                }
                let t = block_on(re.text_with_charset("UTF-8"));
                if t.is_err() {
                    println!("{}", t.unwrap_err());
                    return false;
                }
                let t = t.unwrap();
                let pages = json::parse(t.as_str());
                if pages.is_err() {
                    println!("{}", pages.unwrap_err());
                    return false;
                }
                let pages = pages.unwrap();
                let code = pages["code"].as_i64().unwrap();
                if code != 0 {
                    println!("{} {}", code, pages["message"].as_str().unwrap());
                    return false;
                }
                let pages = &pages["data"];
                let pl = PartInfoList::try_from(pages);
                if pl.is_err() {
                    println!("{}", pl.unwrap_err());
                    return false;
                }
                self.partinfo = Some(pl.unwrap());
            } else {
                self.partinfo = Some(pl.unwrap());
            }
        }
        let fcid = self.partinfo.as_ref().unwrap().first_cid();
        if fcid.is_none() {
            println!("{}", gettext("Can not find CID."));
            return false;
        }
        let fcid = fcid.unwrap();
        if !self.get_cid_info(fcid) {
            return false;
        }
        let interaction = self.is_interaction_video();
        if interaction.is_none() {
            return false;
        }
        let interaction = interaction.unwrap();
        if interaction {
            let gv = &self.cidinfo.get(&fcid).unwrap()["interaction"]["graph_version"]
                .as_usize()
                .unwrap();
            let buvid3 = self.base.client.as_ref().unwrap().get_cookie("buvid3");
            let pc = self.part_count();
            let mut parser = InteractionVideoParser::new(
                *gv,
                buvid3,
                self.partinfo.as_ref().unwrap().clone(),
                url.clone(),
                pc,
                self.base.opt.clone(),
                self.base.se.clone(),
            );
            if !parser.parse(self.base.client.as_mut().unwrap()) {
                return false;
            }
            self.partinfo = Some(parser.part_list);
        }
        true
    }

    /// Get cid info from API (`https://api.bilibili.com/x/player/v2`) and write to [`cidinfo`](#structfield.cidinfo) if success
    /// * cid - CID
    ///
    /// Return true if successed.
    fn get_cid_info(&mut self, cid: usize) -> bool {
        let c = self.base.client.as_ref().unwrap();
        let url = self.url.as_ref().unwrap().clone();
        let r = c.get_with_param(
            "https://api.bilibili.com/x/player/v2",
            json::object! {"aid": url.av, "bvid": url.bv, "cid": cid},
        );
        if r.is_none() {
            println!("{}", gettext("Can not get part info."));
            return false;
        }
        let r = r.unwrap();
        if r.status().as_u16() > 400 {
            println!("{}\n{}", gettext("Can not get part info."), r.status());
            return false;
        }
        let t = block_on(r.text_with_charset("UTF-8"));
        if t.is_err() {
            println!("{}", t.unwrap_err());
            return false;
        }
        let t = t.unwrap();
        let re = json::parse(t.as_str());
        if re.is_err() {
            println!("{}", re.unwrap_err());
            return false;
        }
        let re = re.unwrap();
        let code = re["code"].as_i64().unwrap();
        if code != 0 {
            println!("{} {}", code, re["message"].as_str().unwrap());
            return false;
        }
        let data = &re["data"];
        self.cidinfo.insert(cid, data.clone());
        true
    }

    /// Generate video metadata.
    /// * `p` - Part number
    fn gen_video_metadata(&self, p: Option<usize>) -> Option<VideoMetadata> {
        let mut md = VideoMetadata::default();
        if self.videoinfo.is_none() {
            return None;
        }
        let vi = self.videoinfo.as_ref().unwrap();
        println!("{}", vi.pretty(2));
        let c = self.part_count();
        let n = match p {
            Some(p) => p,
            None => 1,
        };
        if c.is_some() {
            let tr = NoInTotal::new(n, c.unwrap());
            if tr.is_some() {
                md.track = Some(tr.unwrap());
            }
        }
        let t = vi["videoData"]["title"].as_str();
        let pa = &self.partinfo.as_ref().unwrap().list[n - 1];
        md.extra.insert(String::from("part"), pa.part.clone());
        if t.is_some() {
            if c.is_none() || c.unwrap() == 1 {
                md.title = Some(String::from(t.unwrap()));
            } else {
                md.title = Some(format!("{} - {}", t.unwrap(), pa.part));
            }
            md.album = Some(String::from(t.unwrap()));
        }
        let des = vi["videoData"]["desc"].as_str();
        if des.is_some() {
            md.description = Some(String::from(des.unwrap()));
        }
        let au = vi["videoData"]["owner"]["name"].as_str();
        if au.is_some() {
            md.author = Some(String::from(au.unwrap()));
            md.album_artist = Some(String::from(au.unwrap()));
        }
        let bv = vi["videoData"]["bvid"].as_str();
        if bv.is_some() {
            md.video_id = Some(String::from(bv.unwrap()));
            md.extra
                .insert(String::from("bvid"), String::from(bv.unwrap()));
        }
        let pubdate = vi["videoData"]["pubdate"].as_i64();
        if pubdate.is_some() {
            let t = Utc.timestamp(pubdate.unwrap(), 0);
            md.date = Some(t);
            md.extra.insert(String::from("pubdate"), format!("{:?}", t));
        }
        let ctime = vi["videoData"]["ctime"].as_i64();
        if ctime.is_some() {
            let t = Utc.timestamp(ctime.unwrap(), 0);
            if md.date.is_none() {
                md.date = Some(t);
            }
            md.extra.insert(String::from("ctime"), format!("{:?}", t));
        }
        let tags = &vi["tags"];
        for tag in tags.members() {
            let t = tag["tag_name"].as_str();
            if t.is_some() {
                md.tags.push(String::from(t.unwrap()));
            }
        }
        let aid = vi["videoData"]["aid"].as_i64();
        if aid.is_some() {
            md.extra
                .insert(String::from("aid"), format!("AV{}", aid.unwrap()));
        }
        println!("{:?}", md);
        Some(md)
    }

    /// Return true if it is a interactive video.  
    /// Need first CID's info in [`cidinfo`](#structfield.cidinfo)
    fn is_interaction_video(&self) -> Option<bool> {
        let fcid = self.partinfo.as_ref().unwrap().first_cid();
        if fcid.is_none() {
            return None;
        }
        let fcid = fcid.unwrap();
        let info = self.cidinfo.get(&fcid);
        if info.is_none() {
            return None;
        }
        let info = info.unwrap();
        let i = &info["interaction"];
        if i.is_object() {
            return Some(true);
        }
        Some(false)
    }

    fn parse_url(url: &str) -> Option<UrlInfo> {
        let caps = RE.captures(url);
        match caps {
            Some(caps) => {
                let av = caps.name("av");
                let bv = caps.name("bv");
                if av.is_none() {
                    let bv = bv.unwrap();
                    return Some(UrlInfo::from_bv(String::from(bv.as_str()), None));
                } else {
                    let av = av.unwrap().as_str();
                    let av = av.parse::<usize>();
                    match av {
                        Ok(av) => {
                            return Some(UrlInfo::from_av(av, None));
                        }
                        Err(_) => {
                            println!("{}", gettext("AV number is too big."));
                            return None;
                        }
                    }
                }
            }
            None => {}
        }
        for i in 0..2 {
            let re = if i == 0 { &*RE2 } else { &*RE3 };
            let caps = re.captures(url);
            match caps {
                Some(caps) => {
                    let av = caps.name("av");
                    let bv = caps.name("bv");
                    let part = caps.name("part");
                    let mut p: Option<usize> = None;
                    if !part.is_none() {
                        let part = part.unwrap().as_str();
                        p = util::atou(part);
                    }
                    if av.is_none() {
                        let bv = bv.unwrap();
                        return Some(UrlInfo::from_bv(String::from(bv.as_str()), p));
                    } else {
                        let av = av.unwrap().as_str();
                        let av = av.parse::<usize>();
                        match av {
                            Ok(av) => {
                                return Some(UrlInfo::from_av(av, p));
                            }
                            Err(_) => {
                                println!("{}", gettext("AV number is too big."));
                                return None;
                            }
                        }
                    }
                }
                None => {}
            }
        }
        None
    }

    /// Get the count of parts from [`videoinfo`](#structfield.videoinfo)
    fn part_count(&self) -> Option<usize> {
        if !self.videoinfo.is_none() {
            let vi = self.videoinfo.as_ref().unwrap();
            let c = &vi["videoData"]["videos"];
            let c = c.as_usize();
            if !c.is_none() {
                return Some(c.unwrap());
            }
        }
        None
    }
}

impl Provider for BiliNormalVideoProvider {
    fn new() -> BiliNormalVideoProvider {
        BiliNormalVideoProvider {
            base: BiliBaseProvider::new(),
            videoinfo: None,
            playinfo: None,
            partinfo: None,
            cidinfo: HashMap::new(),
            url: None,
        }
    }

    fn add_custom_options(&self, opt: &mut OptStore) {
        self.base.add_custom_options(opt);
        opt.add(self.provider_name(), get_bili_normal_video_options());
    }

    fn add_custom_settings(&self, store: &mut SettingStore) {
        self.base.add_custom_settings(store);
        store.add(self.provider_name(), get_bili_normal_video_settings());
    }

    fn can_login(&self) -> bool {
        true
    }

    fn check_logined(&mut self) -> Option<bool> {
        self.base.check_logined()
    }

    fn extract(&mut self, url: &str) -> Option<ExtractInfo> {
        let u = Self::parse_url(url).unwrap();
        if !self.basic_info(u) {
            return None;
        }
        let re = self.gen_video_metadata(None);
        if re.is_none() {
            return None;
        }
        None
    }

    fn get_custom_options() -> Vec<OptDes> {
        get_bili_normal_video_options()
    }

    fn get_custom_settings() -> Vec<SettingDes> {
        get_bili_normal_video_settings()
    }

    fn get_default_cookie_jar_name(&self) -> Option<&str> {
        Some("bili")
    }

    fn has_custom_options(&self) -> bool {
        true
    }

    fn has_custom_settings(&self) -> bool {
        true
    }

    fn init(&mut self, jar: Option<&CookiesJar>, opt: OptStore, settings: SettingStore) -> bool {
        self.base.init(jar, opt, settings)
    }

    fn login(&mut self, jar: &mut CookiesJar) -> bool {
        self.base.login(jar)
    }

    fn logined(&self) -> bool {
        self.base.logined()
    }

    fn match_url(url: &str) -> bool {
        let le = BiliNormalVideoProvider::parse_url(url);
        if le.is_none() {
            false
        } else {
            true
        }
    }

    fn provider_name(&self) -> &'static str {
        "BiliNormalVideoProvider"
    }
}

#[test]
fn test_parse_url() {
    assert_eq!(
        Some(UrlInfo {
            av: 170001,
            bv: String::from("BV17x411w7KC"),
            part: None
        }),
        BiliNormalVideoProvider::parse_url("av170001")
    );
    assert_eq!(
        Some(UrlInfo {
            av: 9,
            bv: String::from("BV1xx411c7mC"),
            part: None
        }),
        BiliNormalVideoProvider::parse_url("BV1xx411c7mC")
    );
    assert_eq!(None, BiliNormalVideoProvider::parse_url("BV2331"));
    assert_eq!(
        Some(UrlInfo {
            av: 207281893,
            bv: String::from("BV1nh411B7G5"),
            part: None
        }),
        BiliNormalVideoProvider::parse_url("https://www.bilibili.com/video/BV1nh411B7G5")
    );
    assert_eq!(
        Some(UrlInfo {
            av: 170001,
            bv: String::from("BV17x411w7KC"),
            part: Some(2)
        }),
        BiliNormalVideoProvider::parse_url("https://www.bilibili.com/video/av170001?test3&p=2&d")
    );
    assert_eq!(
        Some(UrlInfo {
            av: 170001,
            bv: String::from("BV17x411w7KC"),
            part: None
        }),
        BiliNormalVideoProvider::parse_url("https://www.bilibili.com/video/av170001?test3&p=3f&d")
    );
    assert_eq!(
        Some(UrlInfo {
            av: 170001,
            bv: String::from("BV17x411w7KC"),
            part: Some(2)
        }),
        BiliNormalVideoProvider::parse_url("https://b23.tv/av170001?test3&p=2&d")
    );
}
