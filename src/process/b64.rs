use crate::cli::Base64Format;
use base64::prelude::*;
use std::fs;
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&buf),
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
    };

    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buf)?,
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
    };

    Ok(decoded)
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(fs::File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        assert!(process_encode(input, Base64Format::UrlSafe).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        assert!(process_decode(input, Base64Format::UrlSafe).is_ok());
    }
}
