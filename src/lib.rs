mod cli;
mod process;

pub use cli::{Base64Format,Opts, SubCommand,Base64SubCommand};
pub use process::process_csv;
pub use process::process_genpass;
pub use process::{process_encode,process_decode};