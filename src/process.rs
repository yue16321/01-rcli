use crate::opts::Csv;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

pub fn process_csv(opts: Csv) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(opts.input)?;
    let mut records = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        println!("{:?}", record);
        records.push(record);
    }
    let json = serde_json::to_string_pretty(&records)?;
    fs::write(opts.output, json)?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
