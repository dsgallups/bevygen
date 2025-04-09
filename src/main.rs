use anyhow::Result;
use bevygen::{Args, handle_args};
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    handle_args(args)
}
