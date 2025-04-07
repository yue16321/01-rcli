use clap::Parser;
use rcli::{Opts, SubCommand, process_csv};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(opts)?;
        }
    }
    Ok(())
}
