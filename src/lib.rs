use serde::{Deserialize, Serialize};
use std::fs;
pub mod fusion;
use clap::{Parser, Subcommand};

#[derive(Debug, Deserialize, Serialize)]
struct IconInfo {
    title: String,
    #[serde(rename = "categoryTitle")]
    category_title: String,
    path: String,
    category: String,
    #[serde(rename = "titleKey")]
    title_key: String,
    #[serde(rename = "categoryKey")]
    category_key: String,
}

pub fn process(path: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let reg = regex::Regex::new(r"\{.*\}")?;
    let json_str = fs::read_to_string(path)?;
    let res = reg
        .find_iter(&json_str)
        .map(|i| serde_json::from_str::<IconInfo>(i.as_str()).unwrap())
        .collect::<Vec<_>>();
    fs::write(output, serde_json::to_vec_pretty(&res).unwrap())?;
    Ok(())
}

/// 处理json文件，将json转换成正常的格式
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct ProcessJson {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// 处理异常格式的json文件
    Process {
        /// json 文件路径
        #[arg(short, long)]
        path: String,
        /// 输出文件路径
        #[arg(short, long)]
        output: String,
    },
    /// 融合两个json文件，并将英文title放入一个json中
    Fusion,
}

impl SubCommand {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            SubCommand::Process { path, output } => {
                process(path, output)?;
            }
            SubCommand::Fusion => {
                fusion::fusion_run()?;
            }
        }
        Ok(())
    }
}
