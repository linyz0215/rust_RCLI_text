use clap::Parser;

use RCLI::{Opts, SubCommand, process_csv};
use anyhow::Result;

fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}",opts.format)
            };
            process_csv(&opts.input, output,opts.format)?;
        }
    }
    Ok(())
}
