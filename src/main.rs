use clap::{Parser};

use RCLI::{Base64SubCommand, Opts, SubCommand,TestSubCommand, TextSignFormat, process_csv, process_decode, process_encode, process_genpass,process_text_sign, process_text_verify};
use anyhow::Result;
use zxcvbn::zxcvbn;
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
            let password = process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.numbers, opts.symbols)?;
            println!("{}", password);

            let estimate = zxcvbn::zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64SubCommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                let decode = String::from_utf8(decode)?;
                println!("{}", decode);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TestSubCommand::Sign(opts) => {
                process_text_sign(&opts.input, &opts.key, opts.format)?;
            }
            TestSubCommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
            }
        },
    }
    Ok(())
}