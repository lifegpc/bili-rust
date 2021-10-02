use std::cmp::PartialEq;
use std::fmt::Debug;
use std::io::Write;

/// A template writable file which store data in memory.
pub struct MemoryFile {
    buffer: Vec<u8>,
}

impl MemoryFile {
    pub fn new() -> Self {
        Self {
            buffer: [].to_vec(),
        }
    }
}

impl Write for MemoryFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer = [&self.buffer, buf].concat();
        Ok(buf.len())
    }
    /// Do nothing.
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Debug for MemoryFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf: &[u8] = &self.buffer;
        buf.fmt(f)
    }
}

impl PartialEq for MemoryFile {
    fn eq(&self, other: &Self) -> bool {
        let buf: &[u8] = &self.buffer;
        let buf2: &[u8] = &other.buffer;
        buf == buf2
    }
}

impl PartialEq<&str> for MemoryFile {
    fn eq(&self, other: &&str) -> bool {
        let buf: &[u8] = &self.buffer;
        let buf2 = other.as_bytes();
        buf == buf2
    }
}

impl PartialEq<&[u8]> for MemoryFile {
    fn eq(&self, other: &&[u8]) -> bool {
        let buf: &[u8] = &self.buffer;
        buf == *other
    }
}

#[test]
fn test_memory_file() {
    let mut f = MemoryFile::new();
    f.write("23".as_bytes()).unwrap();
    let mut f2 = MemoryFile::new();
    f2.write("23".as_bytes()).unwrap();
    assert_eq!(f, f2);
    assert_eq!(f, "23");
    assert_eq!(f, "23".as_bytes());
}
