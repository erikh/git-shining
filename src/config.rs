use crate::{builder::build_dates, consts::GRID_SIZE, state::StateMap};
use anyhow::anyhow;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config(pub Vec<String>);

impl Config {
    fn valid(&self) -> Result<(), anyhow::Error> {
        if self.0.len() == GRID_SIZE {
            return Ok(());
        }

        Err(anyhow!("Invalid Size: must be {}", GRID_SIZE))
    }

    pub fn to_grid(&self) -> Result<StateMap, anyhow::Error> {
        self.valid()?;
        let mut dates = build_dates();

        let mut i = 0;
        for x in &mut dates.0 {
            x.1 = self.0[i].trim() == "t";
            i += 1;
        }

        Ok(dates)
    }
}
