use super::{verify_input_file, verify_path};
use clap::{Args, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Subcommand)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a text with public/session key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new text")]
    Gen(TextKeyGenOpts),
    #[command(about = "encrypt a text ")]
    Encrypt(EncryptOpts),
    #[command(about = "decrypt a text")]
    Decrypt(DecryptOpts),
}

#[derive(Debug, Args)]
pub struct TextSignOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=verify_input_file)]
    pub key: String,
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Args)]
pub struct TextVerifyOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=verify_input_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Args)]
pub struct TextKeyGenOpts {
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

#[derive(Debug, Args)]
pub struct EncryptOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=verify_input_file)]
    pub key: String,
}

#[derive(Debug, Args)]
pub struct DecryptOpts {
    #[arg(short, long,value_parser=verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=verify_input_file)]
    pub key: String,
}
