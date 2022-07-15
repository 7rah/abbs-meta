use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub abbs_path: String,
    pub thread: usize,
    pub commits_db_path: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)?;
        let config: Config = toml::from_str(&toml_str)?;
        Ok(config)
    }
}