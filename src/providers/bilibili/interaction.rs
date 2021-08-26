extern crate futures;
extern crate json;

use crate::getopt::OptStore;
use crate::http_client::CookieClient;
use crate::i18n::gettext;
use crate::providers::bilibili::normal_video::UrlInfo;
use crate::providers::bilibili::part_info::PartInfo;
use crate::providers::bilibili::part_info::PartInfoList;
use crate::settings::SettingStore;
use futures::executor::block_on;
use json::JsonValue;
use std::clone::Clone;
use std::convert::TryFrom;
use std::default::Default;

/// The information of an edge
#[derive(Debug, PartialEq)]
pub struct EdgeInfo {
    /// CID
    pub cid: usize,
    pub condition: String,
    /// Edge ID
    pub id: usize,
    pub is_default: bool,
    /// Used in API
    pub native_action: String,
    /// Option name
    pub option: String,
}

impl Clone for EdgeInfo {
    fn clone(&self) -> Self {
        Self {
            cid: self.cid.clone(),
            condition: self.condition.clone(),
            id: self.id.clone(),
            is_default: self.is_default.clone(),
            native_action: self.native_action.clone(),
            option: self.option.clone(),
        }
    }
}

impl Default for EdgeInfo {
    fn default() -> Self {
        Self {
            cid: usize::MAX,
            condition: String::from(""),
            id: usize::MAX,
            is_default: false,
            native_action: String::from(""),
            option: String::from(""),
        }
    }
}

impl TryFrom<&JsonValue> for EdgeInfo {
    type Error = &'static str;
    fn try_from(v: &JsonValue) -> Result<Self, Self::Error> {
        let cid = v["cid"].as_usize();
        if cid.is_none() {
            return Err("CID is needed.");
        }
        let cid = cid.unwrap();
        let id = v["id"].as_usize();
        if id.is_none() {
            return Err("Edge ID is needed.");
        }
        let id = id.unwrap();
        let mut r = Self::default();
        r.cid = cid;
        r.id = id;
        let cond = &v["condition"];
        if cond.is_string() {
            r.condition = String::from(cond.as_str().unwrap());
        }
        let isd = &v["is_default"];
        if isd.is_number() {
            if isd.as_number().unwrap() != 0 {
                r.is_default = true;
            }
        }
        let na = &v["native_action"];
        if na.is_string() {
            r.native_action = String::from(na.as_str().unwrap());
        }
        let opt = &v["option"];
        if opt.is_string() {
            r.option = String::from(opt.as_str().unwrap());
        }
        Ok(r)
    }
}

/// A parser to parse the page list of ineraction video
pub struct InteractionVideoParser {
    /// Needed in parse API
    graph_version: usize,
    /// From cookies, may needed.  
    /// According to the UI, if not logined, some choices may not return.
    buvid3: Option<String>,
    /// The list of edge ID. Used to remove duplicate.
    edge_list: Vec<usize>,
    /// Store the result
    pub part_list: PartInfoList,
    /// Url information
    url: UrlInfo,
    /// Part count
    part_count: Option<usize>,
    /// Options information
    opt: Option<OptStore>,
    /// Settings information
    settings: Option<SettingStore>,
}

impl InteractionVideoParser {
    pub fn new(
        graph_version: usize,
        buvid3: Option<String>,
        part_list: PartInfoList,
        url: UrlInfo,
        part_count: Option<usize>,
        opt: Option<OptStore>,
        settings: Option<SettingStore>,
    ) -> Self {
        Self {
            graph_version,
            buvid3,
            edge_list: [].to_vec(),
            part_list,
            url,
            part_count,
            opt,
            settings,
        }
    }

    /// Add a node's information to [`part_list`](#structfield.part_list)
    /// * `data` - data from [`get_edge_info`](#method.get_edge_info)
    fn add_node(&mut self, data: &JsonValue) -> bool {
        let pn: usize = self.part_list.list.len() + 1;
        let title = data["title"].as_str();
        if title.is_none() {
            return false;
        }
        let title = title.unwrap();
        let id = data["edge_id"].as_number();
        if id.is_none() {
            return false;
        }
        let id = id.unwrap();
        for k in data["story_list"].members() {
            let eid = k["edge_id"].as_number();
            if eid.is_some() {
                if eid.unwrap() == id {
                    let cid = k["cid"].as_usize();
                    if cid.is_none() {
                        return false;
                    }
                    self.part_list
                        .list
                        .push(PartInfo::new(cid.unwrap(), pn, title));
                    return true;
                }
            }
        }
        false
    }

    /// Deal questions in a node
    /// * `data` - data from [`get_edge_info`](#method.get_edge_info)
    fn deal_question(&mut self, c: &mut CookieClient, data: &JsonValue) -> bool {
        for q in data["edges"]["questions"].members() {
            for q2 in q["choices"].members() {
                let ei = EdgeInfo::try_from(q2);
                match ei {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                        return false;
                    }
                }
                let ei = ei.unwrap();
                let mut it = self.edge_list.iter();
                let re = it.find(|&&x| x == ei.id);
                if re.is_none() {
                    let data = self.get_edge_info(c, Some(ei.clone()));
                    if data.is_none() {
                        return false;
                    }
                    let data = data.unwrap();
                    if !self.add_node(&data) {
                        return false;
                    }
                    self.edge_list.push(ei.id);
                    if !self.deal_question(c, &data) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Get edge information from API
    /// * `c` - HTTP Session
    /// * `edge` - Edge information. If is None, means first node.
    fn get_edge_info(&self, c: &mut CookieClient, edge: Option<EdgeInfo>) -> Option<JsonValue> {
        let mut param = json::object! {"bvid": self.url.bv.clone(), "graph_version": self.graph_version, "platform": "pc", "portal": 0, "screen": 0};
        if self.buvid3.is_some() {
            match param.insert("buvid3", self.buvid3.as_ref().unwrap().clone()) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}\n{}", gettext("Warning: "), e);
                }
            }
        }
        if edge.is_some() {
            let e = edge.unwrap();
            match param.insert("edge_id", e.id) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    return None;
                }
            }
            match param.insert("choice", e.native_action.clone()) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    return None;
                }
            }
        }
        let r = c.get_with_param("https://api.bilibili.com/x/stein/edgeinfo_v2", param);
        if r.is_none() {
            return None;
        }
        let r = r.unwrap();
        if r.status().as_u16() > 400 {
            println!("{}\n{}", gettext("Can not get edge info."), r.status());
            return None;
        }
        let t = block_on(r.text_with_charset("UTF-8"));
        if t.is_err() {
            println!("{}", t.unwrap_err());
            return None;
        }
        let t = t.unwrap();
        let re = json::parse(t.as_str());
        if re.is_err() {
            println!("{}", re.unwrap_err());
            return None;
        }
        let re = re.unwrap();
        let code = re["code"].as_i64().unwrap();
        if code != 0 {
            println!("{} {}", code, re["message"].as_str().unwrap());
            return None;
        }
        Some(re["data"].clone())
    }

    /// Get settings from options and settings.
    fn no_use_storylist(&self) -> bool {
        if self.opt.is_some() && self.opt.as_ref().unwrap().has_option("no-use-storylist") {
            return true;
        }
        if self.settings.is_some() {
            let se = self.settings.as_ref().unwrap();
            let re = se.get_settings_as_bool("BiliNormalVideoProvider", "no-use-storylist");
            if re.is_some() {
                return re.unwrap();
            }
        }
        false
    }

    /// Parse part list. Return true if Ok
    /// * `c` - HTTP Session
    pub fn parse(&mut self, c: &mut CookieClient) -> bool {
        let data = self.get_edge_info(c, None);
        if data.is_none() {
            return false;
        }
        let data = data.unwrap();
        if self.part_count.is_some() && !self.no_use_storylist() {
            let count = self.part_count.unwrap();
            let li = self.parse_list_from_story_list(&data["story_list"]);
            if li.is_some() {
                let li = li.unwrap();
                if li.list.len() == count {
                    self.part_list = li;
                    return true;
                }
            }
        }
        if !self.deal_question(c, &data) {
            return false;
        }
        if self.part_count.is_some() && self.part_list.list.len() != self.part_count.unwrap() {
            let s =
                gettext("Video information say there are <total> parts, but only get <num> parts.")
                    .replace("<total>", format!("{}", self.part_count.unwrap()).as_str())
                    .replace("<num>", format!("{}", self.part_list.list.len()).as_str());
            println!("{}", s);
            return false;
        }
        true
    }

    /// In most case, story_list in first node already include all parts.  
    /// So try use story_list first.
    fn parse_list_from_story_list(&self, list: &JsonValue) -> Option<PartInfoList> {
        if !list.is_array() {
            return None;
        }
        let mut r: Vec<PartInfo> = [].to_vec();
        let mut cl: Vec<usize> = [].to_vec();
        let mut pn: usize = 1;
        for i in list.members() {
            let cid = i["cid"].as_usize();
            if cid.is_none() {
                return None;
            }
            let cid = cid.unwrap();
            let mut it = cl.iter();
            let re = it.find(|&&x| x == cid);
            if re.is_none() {
                cl.push(cid);
                let t = i["title"].as_str();
                if t.is_none() {
                    return None;
                }
                let p = PartInfo::new(cid, pn, t.unwrap());
                r.push(p);
                pn += 1;
            }
        }
        let r = PartInfoList::from(r);
        Some(r)
    }
}

impl Clone for InteractionVideoParser {
    fn clone(&self) -> Self {
        Self {
            graph_version: self.graph_version.clone(),
            buvid3: self.buvid3.clone(),
            edge_list: self.edge_list.clone(),
            part_list: self.part_list.clone(),
            url: self.url.clone(),
            part_count: self.part_count.clone(),
            opt: self.opt.clone(),
            settings: self.settings.clone(),
        }
    }
}

#[test]
fn test_edge_info_try_from_json() {
    let mut e = EdgeInfo::default();
    e.cid = 174955203;
    e.id = 7501739;
    e.is_default = true;
    e.option = String::from("A 去上学");
    assert_eq!(
        Ok(e),
        EdgeInfo::try_from(
            &json::object! {"id":7501739,"platform_action":"JUMP 7501739 174955203","native_action":"","condition":"","cid":174955203,"option":"A 去上学","is_default":1}
        )
    );
}
