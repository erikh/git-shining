mod builder;
mod config;
mod consts;
mod state;

use std::{path::PathBuf, str::FromStr};

use crate::{
    builder::{build_grid, generate_json_grid},
    config::Config,
};
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(
    author = "Erik Hollensbe <erik+github@hollensbe.org>",
    version,
    about = "Generate art from Github Contributor Graphs"
)]
#[command(propagate_version = true)]
struct ArgParser {
    #[command(subcommand)]
    command: Command,
}
#[derive(Debug, Subcommand)]
enum Command {
    #[command(alias = "b", about = "Also `b`. Build the graph and export as HTML")]
    Build { filename: Option<PathBuf> },
    #[command(about = "Generate a configuration file to edit to stdout")]
    GenerateConfig,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = ArgParser::parse();
    match cli.command {
        Command::Build { filename } => {
            let res: Config = serde_json::from_str(&std::fs::read_to_string(
                filename.unwrap_or(PathBuf::from_str("config.json")?),
            )?)?;
            println!("{}", build_grid(res.to_grid()?));
        }
        Command::GenerateConfig => {
            println!("{}", generate_json_grid());
        }
    }
    Ok(())
}
