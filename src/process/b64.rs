use crate::{Base64Format, get_reader};
use base64::prelude::*;
use base64::{engine::general_purpose::{STANDARD,URL_SAFE_NO_PAD},
};
//use base64::{engine::general_purpose::{STANDARD,URL_SAFE}, Engine as _};

use std::{fs::File, io::{Read}};
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}",encoded);
    Ok(())
    
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader:Box<dyn Read> = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };
    //TODO：decoded data might not be string (but  for this demo we just print it as string)
    let decoded = String::from_utf8(decoded)?;
    println!("{}",decoded);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{Base64Format, process_decode, process_encode};


    #[test]
    fn test_process_encode(){
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode(){
        let input = "tmp.b64";
        let format = Base64Format::UrlSafe;
        process_decode(input, format).unwrap();
    }
}
