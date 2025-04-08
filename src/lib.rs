mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::{gen_pass, process_csv};
