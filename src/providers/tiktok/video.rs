extern crate chrono;
extern crate futures;
extern crate json;
extern crate regex;

use crate::cookies_json::CookiesJar;
use crate::getopt::OptStore;
use crate::i18n::gettext;
use crate::json_util::jv_multikey_value;
use crate::metadata::ExtractInfo;
use crate::metadata::InfoType;
use crate::metadata::VideoInfo;
use crate::metadata::VideoMetadata;
use crate::providers::provider_base::Provider;
use crate::providers::tiktok::base::TiktokBaseProvider;
use crate::settings::SettingStore;
use chrono::TimeZone;
use chrono::Utc;
use futures::executor::block_on;
use json::JsonValue;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*tiktok\.com/@(?P<name>[^\\/?]+)/video/(?P<id>\d+)(\?.*)?$").unwrap();
    static ref RE2: Regex = Regex::new(r"(?i)^(?:https?://)?(?:[a-z0-9-]+\.)*tiktok\.com/i18n/share/video/(?P<id>\d+)(\?.*)?$").unwrap();
}

pub struct TiktokVideoProvider {
    base: TiktokBaseProvider,
    /// Optional. User name
    user_name: Option<String>,
    /// Used to extract video information.
    video_id: Option<String>,
    /// Video information extracted from page.
    video_info: Option<JsonValue>,
}

impl TiktokVideoProvider {
    /// Get video information
    fn get_info(&mut self) -> bool {
        if self.video_id.is_none() {
            return false;
        }
        let url = if self.user_name.is_none() {
            format!(
                "https://t.tiktok.com/i18n/share/video/{}",
                self.video_id.as_ref().unwrap(),
            )
        } else {
            format!(
                "https://www.tiktok.com/@{}/video/{}",
                self.user_name.as_ref().unwrap(),
                self.video_id.as_ref().unwrap()
            )
        };
        let c = self.base.client.as_mut().unwrap();
        let r = c.get(url);
        if r.is_none() {
            return false;
        }
        let r = r.unwrap();
        if r.url().as_str() == "https://www.tiktok.com/hk/notfound" {
            println!("{}", gettext("Hong Kong was blocked by tiktok."));
            return false;
        }
        if r.status().as_u16() >= 400 {
            println!("{}{}", gettext("Can not get video page: "), r.status());
            return false;
        }
        let t = block_on(r.text_with_charset("UTF8"));
        if t.is_err() {
            println!("{}{}", gettext("Can not get video page: "), t.unwrap_err());
            return false;
        }
        let t = t.unwrap();
        let re = self.base.extract_info(t.as_str());
        if re.is_none() {
            println!("{}", gettext("Can not get video information from page."));
            return false;
        }
        let re = re.unwrap();
        let j = json::parse(re.as_str());
        if j.is_err() {
            println!(
                "{}{}",
                gettext("Can not parse video infomation: "),
                j.unwrap_err()
            );
            return false;
        }
        self.video_info = Some(j.unwrap());
        true
    }

    /// Generate metadata
    fn gen_metadata(&self) -> Option<VideoMetadata> {
        if self.video_info.is_none() {
            return None;
        }
        let vi = self.video_info.as_ref().unwrap();
        let props = &vi["props"]["pageProps"];
        if !props.is_object() {
            println!(
                "{}",
                gettext("Can not get metadata from video information.")
            );
            return None;
        }
        let mut m = VideoMetadata::default();
        let mt = &props["seoProps"]["metaParams"];
        if mt.is_object() {
            let ti = mt["title"].as_str();
            if ti.is_some() {
                m.title = Some(String::from(ti.unwrap()));
                m.description = Some(String::from(ti.unwrap()));
            }
        }
        let is = &props["itemInfo"]["itemStruct"];
        if is.is_object() {
            if m.title.is_none() {
                let desc = is["desc"].as_str();
                if desc.is_some() {
                    m.title = Some(String::from(desc.unwrap()));
                    m.description = Some(String::from(desc.unwrap()));
                }
            }
            let ct = is["createTime"].as_i64();
            if ct.is_some() {
                let t = Utc.timestamp(ct.unwrap(), 0);
                m.date = Some(t);
                m.extra
                    .insert(String::from("createTime"), format!("{:?}", t));
            }
            let au = &is["author"];
            let aun = au["nickname"].as_str();
            if aun.is_some() {
                m.author = Some(String::from(aun.unwrap()));
            }
            let auun = au["uniqueId"].as_str();
            if auun.is_some() {
                m.extra
                    .insert(String::from("authorUniqueId"), String::from(auun.unwrap()));
            }
            let auid = au["id"].as_str();
            if auid.is_some() {
                m.extra
                    .insert(String::from("authorId"), String::from(auid.unwrap()));
            }
            let ausign = au["signature"].as_str();
            if ausign.is_some() {
                m.extra.insert(
                    String::from("authorSignature"),
                    String::from(ausign.unwrap()),
                );
            }
            let id = is["id"].as_str();
            if id.is_some() {
                m.video_id = Some(String::from(id.unwrap()));
            }
            let cha = &is["challenges"];
            for c in cha.members() {
                let ti = c["title"].as_str();
                if ti.is_some() {
                    m.tags.push(String::from(ti.unwrap()));
                }
            }
        }
        Some(m)
    }

    fn extract_playinfo(&self, i: &mut VideoInfo) -> bool {
        let vi = self.video_info.as_ref().unwrap();
        let v = &vi["props"]["pageProps"]["itemInfo"]["itemStruct"]["video"];
        if !v.is_object() {
            println!(
                "{}",
                gettext("Can not get playback url from video information.")
            );
            return false;
        }
        let cov = jv_multikey_value(v, vec!["originCover", "cover"]);
        if cov.is_some() {
            let cov = cov.unwrap().as_str();
            if cov.is_some() {
                i.cover = Some(String::from(cov.unwrap()));
            }
        }
        let url = jv_multikey_value(v, vec!["downloadAddr", "playAddr"]);
        if url.is_none() {
            println!(
                "{}",
                gettext("Can not get playback url from video information.")
            );
            return false;
        }
        let u = url.unwrap().as_str();
        if u.is_none() {
            println!(
                "{}",
                gettext("Can not get playback url from video information.")
            );
            return false;
        }
        i.url = Some(String::from(u.unwrap()));
        true
    }
}

impl Provider for TiktokVideoProvider {
    fn new() -> Self {
        Self {
            base: TiktokBaseProvider::new(),
            user_name: None,
            video_id: None,
            video_info: None,
        }
    }

    fn extract(&mut self, url: &str) -> Option<ExtractInfo> {
        let r = RE.captures(url);
        let r2 = RE2.captures(url);
        if r.is_none() && r2.is_none() {
            return None;
        }
        if r.is_some() {
            let r = r.unwrap();
            let un = r.name("name").unwrap();
            self.user_name = Some(String::from(un.as_str()));
            let vid = r.name("id").unwrap();
            self.video_id = Some(String::from(vid.as_str()));
        } else if r2.is_some() {
            let r2 = r2.unwrap();
            let vid = r2.name("id").unwrap();
            self.video_id = Some(String::from(vid.as_str()));
        }
        if !self.get_info() {
            return None;
        }
        let m = self.gen_metadata();
        if m.is_none() {
            return None;
        }
        let m = m.unwrap();
        let mut vi = VideoInfo {
            meta: m,
            ..Default::default()
        };
        if !self.extract_playinfo(&mut vi) {
            return None;
        }
        vi.headers = Some(TiktokBaseProvider::default_headers());
        vi.cookies = Some(self.base.client.as_ref().unwrap().get_cookie_jar().clone());
        let ei = ExtractInfo {
            typ: InfoType::Video,
            video: Some(vi),
            ..Default::default()
        };
        Some(ei)
    }

    fn init(&mut self, jar: Option<&CookiesJar>, _opt: OptStore, _settings: SettingStore) -> bool {
        if !self.base.init_client(jar) {
            return false;
        }
        true
    }

    fn match_url(url: &str) -> bool {
        let r = RE.captures(url);
        let r2 = RE2.captures(url);
        r.is_some() || r2.is_some()
    }

    fn provider_name(&self) -> &'static str {
        "TiktokVideoProvider"
    }
}
