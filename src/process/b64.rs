use crate::Base64Format;
use base64::prelude::*;
use base64::{engine::general_purpose::{STANDARD,URL_SAFE_NO_PAD}, Engine as _};
//use base64::{engine::general_purpose::{STANDARD,URL_SAFE}, Engine as _};
use std::{fs::File, io::Read};
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader:Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    let data = reader.read_to_end(&mut buf);
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}",encoded);
    Ok(())
    
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader:Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };
    //TODO：decoded data might not be string (but  for this demo we just print it as string)
    let decoded = String::from_utf8(decoded)?;
    println!("{}",decoded);
    Ok(())
}