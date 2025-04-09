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
    /// Generate a color file from a JSON or JSObject
    ///
    /// Recommended to project an input json.
    ///
    /// This will probably be removed
    Color {
        #[arg(short, long)]
        input_file: Option<String>,
        #[arg(short, long)]
        output_file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
}
