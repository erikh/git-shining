mod builder;
mod config;
mod consts;
mod state;

use crate::{
    builder::build_grid,
    config::Config,
    consts::{GRID_SIZE, WEEKS},
};

#[allow(dead_code)]
fn generate_json_grid() -> String {
    let mut res = String::from("[");
    for y in 0..7 {
        res += "\n";
        for x in 0..(WEEKS + 1) {
            res += r#""f""#;

            if (y * x) < GRID_SIZE - 1 {
                res += ",";
            }
        }
    }
    res += "\n]";
    res
}

fn main() -> Result<(), anyhow::Error> {
    let res: Config = serde_json::from_str(&std::fs::read_to_string("config.json")?)?;
    println!("{}", build_grid(res.to_grid()?));
    //    println!("{}", generate_json_grid());
    Ok(())
}
