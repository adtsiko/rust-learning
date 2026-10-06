use std::path::Path;
use std::io::{self, Result, Write};
use std::fs;

use crate::models::Score;

#[derive(Clone)]
pub struct ScoreRepository {
    path: String
}

impl ScoreRepository {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }

    pub fn load_all(&self) -> Result<Vec<Score>> {
        if !Path::new(&self.path).exists() {
            self.init_file()?;
        }

        let contents = fs::read_to_string(&self.path)?;

        serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn save_all(&self, scores: &[Score]) -> Result<()> {
        let json = serde_json::to_string_pretty(scores)?;

        fs::write(&self.path, json)?;
        Ok(())
    }

    fn init_file(&self) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.path)?;
        
        file.write_all(b"[]")?;
        Ok(())
    }
}