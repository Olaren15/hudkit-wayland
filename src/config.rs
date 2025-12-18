/*
OverWay. Create wayland overlays with web technologies
Copyright (C) 2025  Catherine Gilbert

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
