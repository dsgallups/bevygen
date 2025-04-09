use anyhow::Result;
use quote::quote;
use tailwind::TailwindMap;
mod ir;
mod tailwind;

/// Takes in a JSObject or JSON and returns file contents
pub fn generate(input: &str) -> Result<String> {
    let val = TailwindMap::from_json(input)?;
    let header = quote! {
        /// Generated using `color-gen` v0.2

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
