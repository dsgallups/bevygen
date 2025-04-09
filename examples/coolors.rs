use anyhow::Result;
use bevygen::*;

fn main() -> Result<()> {
    handle_args(Args::new(
        "examples/example.json",
        "example_output/coolors_output.rs",
    ))
}
