extern crate json;

use crate::providers::bilibili::util;
use json::JsonValue;
use std::clone::Clone;
use std::convert::From;
use std::convert::TryFrom;

/// The information of a part
#[derive(Debug, PartialEq)]
pub struct PartInfo {
    /// CID, every part should have a different CID, used in some API
    pub cid: usize,
    /// Part duration in seconds
    pub duration: Option<usize>,
    /// Part ID
    pub page: usize,
    /// Part name
    pub part: String,
}

impl PartInfo {
    pub fn new(cid: usize, page: usize, part: &str) -> Self {
        Self {
            cid,
            duration: None,
            page,
            part: String::from(part),
        }
    }
}

impl Clone for PartInfo {
    fn clone(&self) -> Self {
        Self {
            cid: self.cid.clone(),
            duration: self.duration.clone(),
            page: self.page.clone(),
            part: self.part.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PartInfoList {
    pub list: Vec<PartInfo>,
}

impl Clone for PartInfoList {
    fn clone(&self) -> Self {
        Self {
            list: self.list.clone(),
        }
    }
}

impl From<Vec<PartInfo>> for PartInfoList {
    fn from(list: Vec<PartInfo>) -> Self {
        Self { list: list.clone() }
    }
}

impl TryFrom<&JsonValue> for PartInfoList {
    type Error = &'static str;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        if !value.is_array() {
            return Err("The object must be an array.");
        }
        let mut pn = 1;
        let mut list: Vec<PartInfo> = [].to_vec();
        for i in value.members() {
            if !i.is_object() {
                return Err("The object in array must be an object.");
            }
            let ocid = &i["cid"];
            if ocid.is_null() {
                return Err("Cid not exists");
            }
            let tcid = ocid.as_usize();
            let cid: Option<usize>;
            if tcid.is_none() {
                let tcid = ocid.as_str();
                if tcid.is_none() {
                    return Err("Unknown cid.");
                }
                let tcid = util::atou(tcid.unwrap());
                cid = tcid.clone();
            } else {
                cid = tcid.clone();
            }
            if cid.is_none() {
                return Err("Unkown cid.");
            }
            let cid = cid.unwrap();
            let opart = &i["part"];
            if opart.is_null() {
                return Err("part is not exists.");
            }
            let part = opart.as_str();
            if part.is_none() {
                return Err("part is not a string.");
            }
            let part = part.unwrap();
            let opage = &i["page"];
            let tpage = opage.as_usize();
            let mut page: Option<usize> = None;
            if tpage.is_none() {
                let tpage = opage.as_str();
                if !tpage.is_none() {
                    let tpage = util::atou(tpage.unwrap());
                    page = tpage.clone();
                }
            } else {
                page = tpage.clone();
            }
            let page = match page {
                Some(p) => p,
                None => pn,
            };
            let mut p = PartInfo::new(cid, page, part);
            let odur = &i["duration"];
            let tdur = odur.as_usize();
            let mut dur: Option<usize> = None;
            if tdur.is_none() {
                let tdur = odur.as_str();
                if !tdur.is_none() {
                    dur = util::atou(tdur.unwrap());
                }
            } else {
                dur = tdur.clone();
            }
            if !dur.is_none() {
                p.duration = Some(dur.unwrap());
            }
            list.push(p);
            pn += 1;
        }
        if list.len() == 0 {
            return Err("Empty list.");
        }
        Ok(Self::from(list))
    }
}

#[test]
fn test_part_info_list_try_from_json() {
    let obj = json::array![{"cid": 174798944, "page": 1, "duration": 16, "part": "开头"}];
    let mut pi = PartInfo::new(174798944, 1, "开头");
    pi.duration = Some(16);
    let pl = PartInfoList::from(vec![pi]);
    assert_eq!(Ok(pl), PartInfoList::try_from(&obj));
}
