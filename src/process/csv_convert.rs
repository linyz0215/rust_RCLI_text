//use serde::{Deserialize, Serialize};
use std::fs;
use csv::Reader;
use anyhow::Result;
use serde_json::Value;
use crate::cli::OutputFormat;
//#[derive(Debug, Serialize, Deserialize)]//不需要了，因为我们不再定义具体的结构体来映射CSV行，而是使用动态的Value类型

// struct Player {
//     #[serde(rename = "Name")]
//     name: String,
//     #[serde(rename = "Position")]
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     #[serde(rename = "Nationality")]    
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();//需要clone一份headers，否则后面迭代reader时会发生two mutable borrow issue
    for result in reader.records() {//two mutable borrow issue 用clone()
        //这个result存的是一个人的一行记录
        let record= result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();//toml没有类似的动态类型，所以只能支持json和yaml
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(&ret)?
        },
        OutputFormat::Yaml => {
            serde_yaml::to_string(&ret)?
        },
    };
    fs::write(output, content)?;
    Ok(())
}