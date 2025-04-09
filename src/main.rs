use std::path::PathBuf;

use anyhow::Result;
use bevygen::color::ColorArgs;
use clap::{Parser, Subcommand};
use tracing::level_filters::LevelFilter;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Tracing verbosity
    #[arg(global = true, short, long, value_enum, default_value = "ERROR")]
    log: LevelFilter,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize a bevygen.toml file used to configure
    /// this tool
    Init,
    /// Generate a color file from a JSON or JSObject
    ///
    /// Recommended to project an input json.
    Color(ColorArgs),
    /// Scaffold a set of files
    #[command(subcommand)]
    Scaffold(ScaffoldCommand),
}

#[derive(Subcommand)]
enum ScaffoldCommand {
    /// Generates a palette directory
    Palette {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// The name of the palette directory
        #[arg(short, long, default_value = "palettes")]
        name: String,
    },
}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    todo!()
    // match cli.command {
    //     Command::Color(args) => bevygen::color::handle_args(args),
    // }
}
