#[cfg(test)]
use crate::testutils::memory_file::MemoryFile;
use std::clone::Clone;
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::io::Write;

#[derive(Debug)]
/// ffmpeg metadata file
/// See [here](https://ffmpeg.org/ffmpeg-formats.html#Metadata-1) for more information.
pub struct FFMetaFile {
    /// Basic metadata
    pub basic: HashMap<String, String>,
    /// Metadata in specify section
    pub extra: HashMap<String, HashMap<String, String>>,
}

/// Escape key and value
fn escape(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("=", "\\=")
        .replace(";", "\\;")
        .replace("#", "\\#")
        .replace("\n", "\\\n")
}

impl FFMetaFile {
    pub fn new() -> Self {
        Self {
            basic: HashMap::new(),
            extra: HashMap::new(),
        }
    }
    /// Save file
    pub fn save<U: Write>(&self, f: &mut U) -> Result<(), Box<dyn Error>> {
        f.write(";FFMETADATA1\n".as_bytes())?;
        for (k, v) in self.basic.iter() {
            f.write(escape(k).as_bytes())?;
            f.write("=".as_bytes())?;
            f.write(escape(v).as_bytes())?;
            f.write("\n".as_bytes())?;
        }
        for (s, v) in self.extra.iter() {
            f.write("[".as_bytes())?;
            f.write(s.as_bytes())?;
            f.write("]\n".as_bytes())?;
            for (k, v) in v.iter() {
                f.write(escape(k).as_bytes())?;
                f.write("=".as_bytes())?;
                f.write(escape(v).as_bytes())?;
                f.write("\n".as_bytes())?;
            }
        }
        Ok(())
    }
}

impl Clone for FFMetaFile {
    fn clone(&self) -> Self {
        Self {
            basic: self.basic.clone(),
            extra: self.extra.clone(),
        }
    }
}

impl Default for FFMetaFile {
    fn default() -> Self {
        Self {
            basic: HashMap::new(),
            extra: HashMap::new(),
        }
    }
}

#[test]
fn test_save() {
    let mut f = MemoryFile::new();
    let mut f2 = FFMetaFile::new();
    f2.basic
        .insert(String::from("title"), String::from("张小龙没🐎"));
    f2.basic
        .insert(String::from("a\\2"), String::from("a=d2\\#\ntest"));
    let mut m = HashMap::new();
    m.insert(String::from("tes=t"), String::from("a\nf"));
    f2.extra.insert(String::from("test"), m);
    f2.save(&mut f).unwrap();
    assert_eq!(
        f,
        ";FFMETADATA1\ntitle=张小龙没🐎\na\\\\2=a\\=d2\\\\\\#\\\ntest\n[test]\ntes\\=t=a\\\nf\n"
            .as_bytes()
    );
}
