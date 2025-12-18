use std::{fs, path::Path};

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub title: String,
    pub url: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub monitor: u32,
    pub zoom: f64,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config> {
        let data = fs::read(path)?;
        let config: Config = serde_json::from_slice(&data)?;

        Ok(config)
    }
}
