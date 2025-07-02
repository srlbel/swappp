use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: PathBuf,

    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(short, long, value_enum)]
    pub palette: Option<PaletteOption>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PaletteOption {
    CatppucinMocha,
}