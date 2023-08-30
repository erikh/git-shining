use crate::{builder::build_dates, consts::GRID_SIZE, state::StateMap};
use anyhow::anyhow;
use serde_derive::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

const DEFAULT_CONFIG: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config(pub Vec<u8>);

impl Config {
    fn read_file<'a>(filename: Option<PathBuf>) -> Result<String, anyhow::Error> {
        Ok(std::fs::read_to_string(
            filename.unwrap_or(PathBuf::from_str(DEFAULT_CONFIG)?),
        )?)
    }

    pub fn from_path(filename: Option<PathBuf>) -> Result<Self, anyhow::Error> {
        if let Some(filename) = filename {
            let extension = filename
                .extension()
                .unwrap_or(std::ffi::OsStr::new("json"))
                .to_str()
                .unwrap_or("json");
            match extension.to_lowercase().as_str() {
                "json" => Self::from_json(Some(filename)),
                "txt" => Self::from_txt(Some(filename)),
                _ => {
                    return Err(anyhow!(
                        "Expected filename to have extension `.json` or `.txt`"
                    ))
                }
            }
        } else {
            Self::from_json(filename)
        }
    }

    pub fn from_json(filename: Option<PathBuf>) -> Result<Self, anyhow::Error> {
        Ok(serde_json::from_str(&Self::read_file(filename)?)?)
    }

    pub fn from_txt(filename: Option<PathBuf>) -> Result<Self, anyhow::Error> {
        let mut v = Vec::new();
        for byt in Self::read_file(filename)?.as_bytes() {
            if *byt == b'\n' {
                continue;
            }

            v.push(match byt {
                b'a' | b'A' => 10,
                b'9' => 9,
                b'8' => 8,
                b'7' => 7,
                b'6' => 6,
                b'5' => 5,
                b'4' => 4,
                b'3' => 3,
                b'2' => 2,
                b'1' => 1,
                _ => 0,
            });
        }
        Ok(Self(v))
    }

    fn valid(&self) -> Result<(), anyhow::Error> {
        if self.0.len() == GRID_SIZE {
            return Ok(());
        }

        Err(anyhow!("Invalid Size: must be {}", GRID_SIZE))
    }

    pub fn to_grid(&self, origin: Option<chrono::NaiveDate>) -> Result<StateMap, anyhow::Error> {
        self.valid()?;
        let mut dates = build_dates(origin);

        let mut i = 0;
        for x in &mut dates.0 {
            x.1 = self.0[i];
            i += 1;
        }

        Ok(dates)
    }
}
