use clap::{Parser};

use RCLI::{Base64SubCommand, Opts, SubCommand, TestSubCommand, TextSignFormat, process_csv, process_decode, process_encode, process_genpass,process_text_sign};
use anyhow::Result;

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {//output类型为Option<String> Some or None
                output.clone()
            } else {
                format!("output.{}",opts.format.as_str())//直接实现OutputFormat的as_str方法
            };
            process_csv(&opts.input, output,opts.format)?;
        }

        SubCommand::GenPass(opts) => {
            process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.numbers, opts.symbols)?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TestSubCommand::Sign(opts) => {
                match opts.format {
                    TextSignFormat::Blake3 => {
                        process_text_sign(&opts.input, &opts.key, opts.format)?;
                    }
                    TextSignFormat::Ed25519 => {
                        println!("Signing in Base64 format: {:?}",opts);
                    }
                }
            }
            TestSubCommand::Verify(opts) => {
                println!("{:?}",opts);
            }
        },
    }
    Ok(())
}