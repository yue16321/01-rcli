use crate::cli::{Csv, OutputFormat};
use csv::Reader;
use serde_json::Value;
use std::fs;

pub fn process_csv(opts: Csv) -> anyhow::Result<()> {
    let output = if let Some(output) = opts.output {
        output
    } else {
        format!("output.{}", <OutputFormat as Into<&str>>::into(opts.format))
    };
    let mut reader = Reader::from_path(opts.input)?;
    let mut records = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // value:FromIterator<(A::Item, B::Item)>
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }
    let content = match opts.format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
    };
    fs::write(output, content)?;
    Ok(())
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }
