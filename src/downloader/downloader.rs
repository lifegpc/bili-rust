use crate::downloader::aria2c::Aria2c;
use crate::getopt::OptStore;
use crate::metadata::ExtractInfo;
use crate::settings::SettingStore;
use std::clone::Clone;

/// Downloader
pub struct Downloader {
    /// Settings
    se: SettingStore,
    /// Options
    opt: OptStore,
    /// Extract information
    ei: ExtractInfo,
    /// Aria2c interface
    a2: Option<Aria2c>,
}

impl Downloader {
    /// Create a new downloader interfaces
    pub fn new(se: &SettingStore, opt: &OptStore, ei: &ExtractInfo) -> Self {
        let mut t = Self {
            se: se.clone(),
            opt: opt.clone(),
            ei: ei.clone(),
            a2: None,
        };
        if t.enable_arai2c() {
            t.a2 = Aria2c::new(None);
        }
        t
    }

    /// Check whether to enable aria2c
    fn enable_arai2c(&self) -> bool {
        let p = self.opt.get_option_as_bool("aria2c");
        if p.is_some() {
            return p.unwrap();
        }
        let v = self.se.get_settings_as_bool("basic", "aria2c");
        if v.is_some() {
            return v.unwrap();
        }
        true
    }

    /// Run downloader
    pub fn run(&self) -> bool {
        false
    }
}

impl Clone for Downloader {
    fn clone(&self) -> Self {
        Self {
            se: self.se.clone(),
            opt: self.opt.clone(),
            ei: self.ei.clone(),
            a2: self.a2.clone(),
        }
    }
}
