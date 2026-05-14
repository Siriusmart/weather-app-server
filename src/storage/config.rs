use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub sqlite_db_path: PathBuf,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}
