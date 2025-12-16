use std::io::Read;
use anyhow::Result;
use std::fs;
use base64::{engine::general_purpose::{STANDARD,URL_SAFE_NO_PAD},
};
use base64::Engine;
use crate::get_reader;
use crate::TextSignFormat;
trait TextSign {
    fn sign(&self, reader:&mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify<R: Read>(&self,reader: R, sig: &[u8]) -> Result<bool>; 
}

struct Blake3 {
    key: [u8; 32],
}

// struct Ed25519Signer {
//     key: [u8; 32],
// }

// struct Ed25519Verifier {
//     key: [u8; 32],
// }
pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 =>{
            let key: Vec<u8> = fs::read(key)?;
            let key = &key[..32];
            let key = key.try_into()?;
            let signer = Blake3 {key};
            signer.sign(&mut reader)?

        },
        TextSignFormat::Ed25519 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}",signed);
    Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::hash(&buf);
        Ok(hash.as_bytes() == sig)
    }
}