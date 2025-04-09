use anyhow::Result;
use bevygen::color::*;

fn main() -> Result<()> {
    handle_args(ColorArgs::new(
        "examples/example.json",
        "example_output/coolors_output.rs",
    ))
}
