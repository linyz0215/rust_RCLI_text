use clap::Parser;
use super::verify_file;
use std::str::FromStr;
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}


//生命周期 box::leak,'static 

// fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
//     match format.to_lowercase().as_str() {
//         "json" => Ok(OutputFormat::Json),
//         "toml" => Ok(OutputFormat::Toml),
//         "yaml" | "yml" => Ok(OutputFormat::Yaml),
//         _ => Err("Unsupported output format. Supported formats are: json, toml, yaml."),
//     }
// }

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {//这个anyhow::Error能转成&'static str
    format.parse()//parse调用FromStr trait 将字符串转换成其他类型（前提是要实现FromStr）
}


impl FromStr for OutputFormat {//从字符串到枚举的转换
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" | "yml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Unsupported output format. Supported formats are: json, yaml.")),
        }
    }
}

impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// impl fmt::Display for OutputFormat {//实现Display trait 用于格式化输出
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f,"{}",self.as_str())
//     }
// }