#![doc = include_str!("README.md")]

use quote::quote;
use tailwind::TailwindMap;
mod ir;
mod tailwind;
use anyhow::Result;
use std::{
    fs::{self, File},
    io::{self, Write},
};

use clap::Parser;
#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct ColorArgs {
    #[arg(short, long)]
    pub input_file: Option<String>,
    #[arg(short, long)]
    pub output_file: Option<String>,
}
impl ColorArgs {
    pub fn new(input_file: impl ToString, output_file: impl ToString) -> Self {
        Self {
            input_file: Some(input_file.to_string()),
            output_file: Some(output_file.to_string()),
        }
    }
}

/// The main entry point for converting a file with the CLI tool.
///
/// If you don't need to open files/use stdin, then see [`color::generate`].
pub fn handle_args(args: ColorArgs) -> Result<()> {
    let input = match args.input_file {
        Some(file) => fs::read_to_string(file)?,
        None => loop {
            let mut input = String::new();
            println!("Paste your JSON [press ENTER to continue]:");
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input.");
                continue;
            }
            break input;
        },
    };
    let result = generate(&input)?;

    if let Some(output_path) = args.output_file {
        let Ok(mut of) = File::create(output_path) else {
            println!("Error creating file at path. Generated Output:\n=====\n{result}\n=====");
            return Ok(());
        };

        if let Err(e) = of.write_all(result.to_string().as_bytes()) {
            println!(
                "Error at path:\n{:?}\nGenerated Output:\n=====\n{result}\n=====",
                e
            );
            return Ok(());
        };
        println!("Result written to output.");

        return Ok(());
    }
    println!("Output\n=====\n{result}\n=====");
    Ok(())
}

/// Takes in a JSObject or JSON and returns file contents
pub fn generate(input: &str) -> Result<String> {
    let val = TailwindMap::from_json(input)?;
    let header = quote! {
        /// Generated using `bevygen` v0.1

        use bevy::color::Color;
    };

    let token_colors = val.to_token_colors();

    let output = quote! {
        #header

        #(#token_colors)*
    };

    let file = syn::parse_file(&output.to_string())?;
    let result = prettyplease::unparse(&file);

    Ok(result)
}
