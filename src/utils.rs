use std::io::Read;
use anyhow::Result;
use std::fs::File;
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        let file = File::open(input)?;
        Ok(Box::new(file))
    }
}