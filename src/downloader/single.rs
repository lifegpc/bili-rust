use crate::downloader::aria2c::Aria2c;
use crate::downloader::downloader::Downloader;
use crate::downloader::downloader::DownloaderType;
use crate::downloader::downloader::VideoDownloader;
use crate::getopt::OptStore;
use crate::http_client::gen_cookie_header;
use crate::metadata::VideoInfo;
use crate::metadata::VideoPlayInfoType;
use crate::settings::SettingStore;
use crate::utils::headers::header_map_to_hash_map;
use std::clone::Clone;

/// A downloader for single URL.
pub struct SignleUrlDownloader {
    /// Extract Information
    vi: VideoInfo,
    /// Options
    opt: OptStore,
    /// Settings
    se: SettingStore,
    /// Aria2c interface
    a2: Option<Aria2c>,
}

impl SignleUrlDownloader {
    pub fn new(vi: &VideoInfo, opt: &OptStore, se: &SettingStore, a2: Option<&Aria2c>) -> Self {
        let a2 = if a2.is_none() {
            None
        } else {
            Some(a2.unwrap().clone())
        };
        Self {
            vi: vi.clone(),
            opt: opt.clone(),
            se: se.clone(),
            a2,
        }
    }
}

impl Downloader for SignleUrlDownloader {
    fn download(&mut self) -> bool {
        let url = self.vi.url.as_ref().unwrap();
        if self.a2.is_some() {
            let a2 = self.a2.as_mut().unwrap();
            if self.vi.headers.is_some() {
                header_map_to_hash_map(self.vi.headers.as_ref().unwrap(), &mut a2.headers);
            }
            if self.vi.cookies.is_some() {
                let c = gen_cookie_header(self.vi.cookies.as_ref().unwrap(), url);
                if c.len() > 0 {
                    a2.headers.insert(String::from("cookie"), c);
                }
            }
            if self.vi.meta.title.is_some() {
                let output = self.vi.meta.title.as_ref().unwrap().clone() + ".mp4";
                a2.set_output(Some(output).as_ref());
            }
            a2.download(url);
        }
        false
    }

    fn typ() -> DownloaderType {
        DownloaderType::Video
    }
}

impl VideoDownloader for SignleUrlDownloader {
    fn match_vi(vi: &VideoInfo) -> bool {
        if vi.typ == VideoPlayInfoType::SignleUrl {
            return true;
        }
        false
    }
}

impl Clone for SignleUrlDownloader {
    fn clone(&self) -> Self {
        Self {
            vi: self.vi.clone(),
            opt: self.opt.clone(),
            se: self.se.clone(),
            a2: self.a2.clone(),
        }
    }
}
