use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, Error, ErrorKind};
use std::path::Path;

pub struct Cache {
    inner: Vec<InnerCache>,
    fd: String,
}

#[derive(Serialize, Deserialize)]
struct InnerCache {
    id: i64,
    content: String,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            inner: Vec::new(),
            fd: String::new(),
        }
    }

    pub fn init(&mut self, fd: &str) -> io::Result<()> {
        if !Path::new(fd).exists() {
            let _ = File::create(fd)?;
        }
        let mut fh = File::open(fd)?;
        let mut contents = String::new();
        self.fd = fd.to_owned();
        if contents.len() == 0 {
            return Ok(());
        }
        fh.read_to_string(&mut contents)?;
        let parsed: Vec<InnerCache> = serde_json::from_str(&contents)?;
        self.inner = parsed;
        Ok(())
    }

    pub fn save(&mut self) -> io::Result<()> {
        if self.fd == "" {
            return Err(Error::new(
                ErrorKind::NotFound,
                "No file saved, cache broken?",
            ));
        }
        let mut fh = OpenOptions::new().write(true).open(&self.fd)?;
        let serialized = serde_json::to_vec(&self.inner)?;
        fh.write_all(&serialized)?;
        Ok(())
    }

    pub fn add(&mut self, id: i64, content: &str) -> io::Result<()> {
        if self.inner.iter().any(|c| c.id == id) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "item already in cache",
            ));
        }
        self.inner.push(InnerCache::new(1234, "foo"));

        Ok(())
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        self.save().expect("Could not write cache to file");
    }
}

impl InnerCache {
    fn new(id: i64, content: &str) -> InnerCache {
        InnerCache {
            id,
            content: content.to_owned(),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut c = Cache::new();
        match c.init("foo") {
            Err(e) => {
                eprintln!("{}", e);
            }
            _ => {}
        }
        c.add(123, "fred");
    }
}
