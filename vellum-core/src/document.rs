use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub struct Document {
    rope: Rope,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    pub fn new() -> Self {
        Document { rope: Rope::new() }
    }

    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader).map_err(std::io::Error::other)?;
        Ok(Document { rope })
    }

    pub fn to_file(&self, path: &Path) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.rope
            .write_to(&mut writer)
            .map_err(std::io::Error::other)?;
        Ok(())
    }
}
