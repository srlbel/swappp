mod cli;
mod colors;
mod core;

use clap::Parser;
use image::Rgb;

use crate::cli::PaletteOption;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    let selected_palette: &[Rgb<u8>] = match args.palette {
        Some(PaletteOption::CatppucinMocha) => colors::CATPPUCCIN_MOCHA,
        None => colors::CATPPUCCIN_MOCHA
    };

    match core::change_color_scheme(&args.input, &args.output, selected_palette) {
        Ok(_) => println!("Ok"),
        Err(e) => eprintln!("Error: {e}"),
    }

    Ok(())
}
