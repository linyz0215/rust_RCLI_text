use std::{io::Read, path::{self, Path}, vec};
use anyhow::Result;



use std::fs;
use base64::{engine::general_purpose::{STANDARD,URL_SAFE_NO_PAD},
};
use base64::Engine;
use crate::{get_reader, process_genpass};
use crate::TextSignFormat;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
trait TextSign {
    /// Sign the data read from the reader and return the signature bytes
    fn sign(&self, reader:&mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    /// Verify the data from the reader with the signature
    fn verify<R: Read>(&self,reader: R, sig: &[u8]) -> Result<bool>; 
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}
pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?

        },
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        },
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}",signed);
    Ok(())
}

pub fn process_text_verify(input: &str, key: &str, format: TextSignFormat, sig: &str) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        },
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        },
    };
    if verified {
        println!("Signature is valid.");
    } else {
        println!("Signature is invalid.");
    }
    Ok(())
}   
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}
impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf,&sig).is_ok();
        Ok(ret)

    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key: Vec<u8> = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key: Vec<u8> = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key: Vec<u8> = fs::read(path)?;
        Self::try_new(&key)
    }
}


impl Blake3 {
    pub fn new(key:[u8;32]) -> Self {
        Self {key}
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self {key}
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key: Vec<u8> = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self {key}
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }


}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3:Blake3= Blake3::load(".\\blake3.txt")?;
        let data = b"Hello, world!";
        let sig = blake3.sign(&mut &data[..])?;
        println!("Signature: {}", URL_SAFE_NO_PAD.encode(&sig));
        assert!(blake3.verify(&mut &data[..], &sig).unwrap());
        Ok(())
    }
}