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

#[derive(Clone, Copy, PartialEq)]
pub enum InfoType {
    Video,
    VideoList,
}

pub struct VideoInfo {
    pub meta: VideoMetadata,
    pub cover: Option<String>,
}

impl Clone for VideoInfo {
    fn clone(&self) -> VideoInfo {
        VideoInfo {
            meta: self.meta.clone(),
            cover: self.cover.clone(),
        }
    }
}

pub struct ExtractInfo {
    pub typ: InfoType,
    pub video: Option<VideoInfo>,
    pub videos: Option<Vec<VideoInfo>>,
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
