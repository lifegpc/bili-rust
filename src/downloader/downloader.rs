use crate::downloader::aria2c::Aria2c;
use crate::downloader::single::SignleUrlDownloader;
use crate::getopt::OptStore;
use crate::i18n::gettext;
use crate::metadata::ExtractInfo;
use crate::metadata::InfoType;
use crate::metadata::VideoInfo;
use crate::settings::SettingStore;
use std::clone::Clone;

/// Downloader Type
pub enum DownloaderType {
    Video,
}

/// Downloader interface
pub trait Downloader {
    /// Perform download
    fn download(&mut self) -> bool;
    fn typ() -> DownloaderType;
}

/// Video Downloader
pub trait VideoDownloader {
    // Check Video Information Type
    fn match_vi(vi: &VideoInfo) -> bool;
}

/// Main downloader
pub struct MDownloader {
    /// Settings
    se: SettingStore,
    /// Options
    opt: OptStore,
    /// Extract information
    ei: ExtractInfo,
    /// Aria2c interface
    a2: Option<Aria2c>,
}

impl MDownloader {
    /// Create a new downloader
    /// # Panic
    /// If some settings not available, will be panic
    pub fn new(se: &SettingStore, opt: &OptStore, ei: &ExtractInfo) -> Self {
        let mut t = Self {
            se: se.clone(),
            opt: opt.clone(),
            ei: ei.clone(),
            a2: None,
        };
        if t.enable_arai2c() {
            t.a2 = Aria2c::new(None);
            let r = t.opt.get_option_as_size("aria2c-min-split-size");
            let r2 = t.se.get_settings("basic", "aria2c-min-split-size");
            if r.is_some() {
                if !t.a2.as_mut().unwrap().set_min_split_size(&r.unwrap()) {
                    panic!(
                        "{}{}",
                        gettext("Can not set settings: "),
                        gettext("aria2c-min-split-size should be 1048576(1MiB)-1073741824(1GiB).")
                    );
                }
            }
            if r2.is_some() {
                if !t
                    .a2
                    .as_mut()
                    .unwrap()
                    .set_min_split_size(r2.as_ref().unwrap())
                {
                    panic!(
                        "{}{}",
                        gettext("Can not set settings: "),
                        gettext("aria2c-min-split-size should be 1048576(1MiB)-1073741824(1GiB).")
                    );
                }
            }
        }
        t
    }

    /// Preform download
    /// * `d` - Downloader
    fn download(&self, d: &mut impl Downloader) -> bool {
        d.download()
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

    /// Match a suitable Video Downloader
    /// * `vi` - Video information
    fn match_vi(&self, vi: &VideoInfo) -> bool {
        if SignleUrlDownloader::match_vi(vi) {
            return self.download(&mut SignleUrlDownloader::new(
                vi,
                &self.opt,
                &self.se,
                self.a2.as_ref(),
            ));
        }
        println!("{}", gettext("Can not find a suitable video downloader"));
        false
    }

    /// Run downloader
    pub fn run(&self) -> bool {
        if self.ei.typ == InfoType::Video {
            return self.match_vi(self.ei.video.as_ref().unwrap());
        } else if self.ei.typ == InfoType::VideoList {
            for vi in self.ei.videos.as_ref().unwrap().iter() {
                let r = self.match_vi(vi);
                if !r {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

impl Clone for MDownloader {
    fn clone(&self) -> Self {
        Self {
            se: self.se.clone(),
            opt: self.opt.clone(),
            ei: self.ei.clone(),
            a2: self.a2.clone(),
        }
    }
}
