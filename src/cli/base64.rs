use super::verify_input_file;
use clap::{Args, Subcommand};
use std::str::FromStr;

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode a base64 string to string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Args)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Args)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err("Invalid base64 format".into()),
        }
    }
}
