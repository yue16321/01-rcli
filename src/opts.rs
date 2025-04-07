use clap::{Args, Parser, Subcommand};
use std::path::Path;

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
}

/// Show CSV, or convert CSV to other formats
#[derive(Debug, Args)]
pub struct Csv {
    #[arg(short, long,value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file {} does not exist", filename))
    }
}
