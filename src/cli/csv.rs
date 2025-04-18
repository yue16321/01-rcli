use super::verify_input_file;
use clap::Args;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "yml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format")),
        }
    }
}
impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> &'static str {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

/// Show CSV, or convert CSV to other formats
#[derive(Debug, Args)]
pub struct Csv {
    #[arg(short, long,value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)] // "output.json".into()
    pub output: Option<String>,
    #[arg(long, default_value = "json")]
    pub format: OutputFormat, // 只要实现了 FromStr 就可以
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}
