mod cli;
mod process;

pub use cli::{Base64SubCommand, Opts, SubCommand};
pub use process::{gen_pass, process_csv, process_decode, process_encode};
