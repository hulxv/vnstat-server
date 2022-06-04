use std::{fs::File as fsFile, io::Write, path::Path};

use anyhow::{anyhow, Result};

pub struct File {
    path: String,
}

impl File {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    pub fn create(&self, content: String) -> Result<()> {
        let path = Path::new(&self.path);
        match Path::new(path.parent().unwrap()).exists() {
            false => std::fs::create_dir_all(path.parent().unwrap())?,
            _ => (),
        };
        match fsFile::create(path)?.write_all(content.as_bytes()) {
            Err(err) => Err(anyhow!(err)),
            Ok(_) => Ok(()),
        }
    }
    pub fn exists(&self) -> bool {
        Path::new(Path::new(&self.path)).exists()
    }
    pub fn parent_exists(&self) -> bool {
        Path::new(Path::new(&self.path).parent().unwrap()).exists()
    }
}
