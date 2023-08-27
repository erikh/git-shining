mod builder;
mod config;
mod consts;
mod fonts;
mod git;
mod state;

use std::{path::PathBuf, str::FromStr};

use crate::{
    builder::{build_grid, generate_json_grid, generate_txt_grid},
    config::Config,
    fonts::render_font,
    git::GeneratedRepository,
    state::StateMap,
};
use anyhow::anyhow;
use builder::build_dates;
use clap::{Parser, Subcommand, ValueEnum};

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
    GenerateConfig { format: ConfigFormat },
    #[command(about = "Render a graph from a font and message to configuration")]
    RenderFont {
        font: String,
        message: String,
        format: Option<ConfigFormat>,
    },
    #[command(about = "Build a change plan from a configuration")]
    BuildPlan { input: Option<PathBuf> },
    #[command(about = "Generate a repository for pushing, given a configuration")]
    GenerateRepository {
        target_path: PathBuf,
        #[arg(short = 'a', long = "author", help = "Customize the author name")]
        author: Option<String>,
        #[arg(short = 'e', long = "email", help = "Customize the author email")]
        email: Option<String>,
        #[arg(short = 'm', long = "message", help = "Customize the commit message")]
        message: Option<String>,
        #[arg(
            short = 'i',
            long = "input",
            help = "Specify the input file; `config.json` by default"
        )]
        input: Option<PathBuf>,
    },
}

#[derive(Debug, ValueEnum, Clone)]
enum ConfigFormat {
    Json,
    Txt,
}

impl std::str::FromStr for ConfigFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ConfigFormat::Json),
            "txt" => Ok(ConfigFormat::Txt),
            _ => Err(anyhow!("Expected `txt` or `json`")),
        }
    }
}

impl ToString for ConfigFormat {
    fn to_string(&self) -> String {
        match self {
            ConfigFormat::Json => "json",
            ConfigFormat::Txt => "txt",
        }
        .to_string()
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = ArgParser::parse();
    match cli.command {
        Command::Build { filename } => {
            println!("{}", build_grid(Config::from_path(filename)?.to_grid()?));
        }
        Command::GenerateConfig { format } => match format {
            ConfigFormat::Json => {
                println!("{}", generate_json_grid(StateMap::default()));
            }
            ConfigFormat::Txt => {
                println!("{}", generate_txt_grid(StateMap::default()));
            }
        },
        Command::RenderFont {
            font,
            message,
            format,
        } => {
            let format = format.unwrap_or(ConfigFormat::Json);
            let map = render_font(&message, PathBuf::from_str(&font)?, build_dates())?;
            println!(
                "{}",
                match format {
                    ConfigFormat::Json => generate_json_grid(map),
                    ConfigFormat::Txt => generate_txt_grid(map),
                }
            );
        }
        Command::BuildPlan { input } => {
            println!(
                "{}",
                serde_json::to_string(&Config::from_path(input)?.to_grid()?)?
            );
        }
        Command::GenerateRepository {
            target_path,
            input,
            message,
            author,
            email,
        } => {
            let map: StateMap = Config::from_path(input)?.to_grid()?;
            let mut repo = GeneratedRepository::new(target_path.clone(), map);

            if let Some(message) = message {
                repo.set_message(message);
            }

            if let Some(author) = author {
                repo.set_author(author);
            }

            if let Some(email) = email {
                repo.set_email(email);
            }

            repo.init_repository()?;
            repo.run()?;
            println!("Repository generated at '{}'", target_path.display());
        }
    }
    Ok(())
}
