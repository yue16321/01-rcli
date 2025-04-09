mod base64;
mod csv;
mod genpass;

use clap::{Parser, Subcommand};
use std::path::Path;

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::{Csv, OutputFormat};
pub use self::genpass::GenPassOpts;

// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Parser, Debug)]
#[command(name = "rcli",author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Csv(Csv),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exist
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file {} does not exist", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(
            verify_input_file("Cargo.toml"),
            Ok("Cargo.toml".to_string())
        );
        assert_eq!(
            verify_input_file("not-exist"),
            Err("Input file not-exist does not exist".to_string())
        );
    }
}
