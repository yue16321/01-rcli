use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use clap::Parser;
use rcli::{
    Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSubCommand, gen_pass, get_content,
    get_reader, process_csv, process_decode, process_encode, process_http_serve,
    process_text_decrypt, process_text_encrypt, process_text_key_generate, process_text_sign,
    process_text_verify,
};
use std::fs;
use std::io::Write;
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(opts)?;
        }
        SubCommand::GenPass(opts) => {
            let password = gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            print!("{}", password);
            std::io::stdout().flush()?;
            eprintln!(); // 使用标准
            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{}", String::from_utf8_lossy(&decoded));
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut get_reader(&opts.input)?, &key, opts.format)?;
                let encoded = BASE64_URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubCommand::Verify(opts) => {
                let key = get_content(&opts.key)?;
                let encoded = BASE64_URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(
                    &mut get_reader(&opts.input)?,
                    &key,
                    &encoded,
                    opts.format,
                )?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubCommand::Gen(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
            TextSubCommand::Encrypt(opts) => {
                let key = get_content(&opts.key)?;
                let mut input = get_reader(&opts.input)?;
                let ciphertext = process_text_encrypt(&mut input, &key)?;
                println!("encrypt: {}", ciphertext);
            }
            TextSubCommand::Decrypt(opts) => {
                let key = get_content(&opts.key)?;
                let mut input = get_reader(&opts.input)?;
                let plaintext = process_text_decrypt(&mut input, &key)?;
                println!("decrypt: {}", String::from_utf8_lossy(&plaintext));
            }
        },
        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
