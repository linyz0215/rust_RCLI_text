mod csv;
mod genpass;
mod base64;
mod text;
use std::path::Path;

use clap::Parser;

use self::{csv::CsvOpts, genpass::GenPassOpts};
pub use self::{
    csv::OutputFormat,
    base64::{Base64SubCommand, Base64Format},
    text::{TestSubCommand,TextSignFormat}
};
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about = "lalalalallalalal、", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSVS, or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TestSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is  "-" or fire exists, 
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file does not exist.")
    }
}
