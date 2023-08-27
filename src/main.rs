mod builder;
mod config;
mod consts;
mod fonts;
mod state;

use std::{path::PathBuf, str::FromStr};

use crate::{
    builder::{build_grid, generate_json_grid},
    config::Config,
    fonts::render_font,
    state::StateMap,
};
use builder::build_dates;
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
    #[command(about = "Render a graph from a font and message to configuration")]
    RenderFont { font: String, message: String },
    #[command(about = "Build a change plan from a configuration")]
    BuildPlan { input: Option<PathBuf> },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = ArgParser::parse();
    match cli.command {
        Command::Build { filename } => {
            println!("{}", build_grid(Config::from_file(filename)?.to_grid()?));
        }
        Command::GenerateConfig => {
            println!("{}", generate_json_grid(StateMap::default()));
        }
        Command::RenderFont { font, message } => {
            println!(
                "{}",
                generate_json_grid(render_font(
                    &message,
                    PathBuf::from_str(&font)?,
                    build_dates(),
                )?)
            );
        }
        Command::BuildPlan { input } => {
            serde_json::to_string(&Config::from_file(input)?.to_grid()?)?;
        }
    }
    Ok(())
}
