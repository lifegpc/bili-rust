extern crate chrono;

use chrono::DateTime;
use chrono::Utc;
use std::clone::Clone;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::Debug;

/// Present a current number in a total value
pub struct NoInTotal {
    /// Current value
    _no: usize,
    /// Total value
    _total: usize,
}

impl NoInTotal {
    pub fn new(no: usize, total: usize) -> Option<Self> {
        if no > total {
            None
        } else {
            Some(Self {
                _no: no,
                _total: total,
            })
        }
    }
}

impl Clone for NoInTotal {
    fn clone(&self) -> NoInTotal {
        NoInTotal {
            _no: self._no.clone(),
            _total: self._total.clone(),
        }
    }
}

impl Debug for NoInTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}/{}", self._no, self._total))
    }
}

#[derive(Debug)]
/// Video metadata
pub struct VideoMetadata {
    /// Video title
    pub title: Option<String>,
    /// Video description.  
    /// In mp4 container, description will write to comment because comment have higher priority than description.
    pub description: Option<String>,
    /// Video author.  
    /// In mp4 container, author will write to artist.
    pub author: Option<String>,
    /// Video series name.  
    pub album: Option<String>,
    /// Video id.  
    /// In mp4 container, it will write to episode_id.
    pub video_id: Option<String>,
    /// Video part number
    pub track: Option<NoInTotal>,
    /// Video tags.  
    /// In mp4 container, it will write to genre.
    pub tags: Vec<String>,
    /// Video series's author
    pub album_artist: Option<String>,
    /// Publish date
    pub date: Option<DateTime<Utc>>,
    /// Video comment
    pub comment: Option<String>,
    /// Extra metadata
    pub extra: HashMap<String, String>,
}

impl Clone for VideoMetadata {
    fn clone(&self) -> VideoMetadata {
        VideoMetadata {
            title: self.title.clone(),
            description: self.description.clone(),
            author: self.author.clone(),
            album: self.album.clone(),
            video_id: self.video_id.clone(),
            track: self.track.clone(),
            tags: self.tags.clone(),
            album_artist: self.album_artist.clone(),
            date: self.date.clone(),
            comment: self.comment.clone(),
            extra: self.extra.clone(),
        }
    }
}

impl Default for VideoMetadata {
    fn default() -> VideoMetadata {
        VideoMetadata {
            title: None,
            description: None,
            author: None,
            album: None,
            video_id: None,
            track: None,
            tags: [].to_vec(),
            album_artist: None,
            date: None,
            comment: None,
            extra: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// The type of [`ExtractInfo`](struct.ExtractInfo.html)
pub enum InfoType {
    /// A single video
    Video,
    /// A list of videos
    VideoList,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// The video play information type used in [`VideoInfo`](struct.VideoInfo.html)
pub enum VideoPlayInfoType {
    /// Only have a single url which point to the video.
    SignleUrl,
}

#[derive(Debug)]
/// Video information
pub struct VideoInfo {
    /// The metadata of the video
    pub meta: VideoMetadata,
    /// The url of cover image
    pub cover: Option<String>,
    /// The type of video playurl information
    pub typ: VideoPlayInfoType,
    /// The playback url of video.
    /// Used when `typ` is [`VideoPlayInfoType::SignleUrl`](enum.VideoPlayInfoType.html#variant.SignleUrl)
    pub url: Option<String>,
}

impl VideoInfo {
    /// Check the information
    pub fn check(&self) -> bool {
        if self.typ == VideoPlayInfoType::SignleUrl {
            if self.url.is_none() {
                return false;
            }
        }
        true
    }
}

impl Clone for VideoInfo {
    fn clone(&self) -> VideoInfo {
        VideoInfo {
            meta: self.meta.clone(),
            cover: self.cover.clone(),
            typ: self.typ.clone(),
            url: self.url.clone(),
        }
    }
}

impl Default for VideoInfo {
    fn default() -> Self {
        Self {
            meta: VideoMetadata::default(),
            cover: None,
            typ: VideoPlayInfoType::SignleUrl,
            url: None,
        }
    }
}

#[derive(Debug)]
pub struct ExtractInfo {
    pub typ: InfoType,
    pub video: Option<VideoInfo>,
    pub videos: Option<Vec<VideoInfo>>,
}

impl ExtractInfo {
    /// Check the information.
    pub fn check(&self) -> bool {
        if self.typ == InfoType::Video {
            if self.video.is_none() {
                return false;
            }
            if !self.video.as_ref().unwrap().check() {
                return false;
            }
        } else if self.typ == InfoType::VideoList {
            if self.videos.is_none() {
                return false;
            }
            let it = self.videos.as_ref().unwrap().iter();
            for v in it {
                if !v.check() {
                    return false;
                }
            }
        }
        true
    }
}

impl Clone for ExtractInfo {
    fn clone(&self) -> ExtractInfo {
        ExtractInfo {
            typ: self.typ.clone(),
            video: self.video.clone(),
            videos: self.videos.clone(),
        }
    }
}

impl Default for ExtractInfo {
    fn default() -> Self {
        Self {
            typ: InfoType::Video,
            video: None,
            videos: None,
        }
    }
}
