use dialogue_macro::Asker;
use serde_json::{json, Value};
use std::{borrow::BorrowMut, ffi::OsStr, fs};

use crate::IconInfo;

#[derive(Debug, Asker)]
struct Fusion {
    #[select(prompt = "请选择中文json")]
    json_a: String,
    #[select(prompt = "请选择英文json")]
    json_b: String,
    #[input(prompt = "请输入输出文件路径", default = "fusion.json")]
    output: String,
}

fn read_json_files() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let entries = fs::read_dir(current_dir)?;
    let mut res = vec![];
    for i in entries {
        let entry = i?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("json")) {
            let str = path.to_string_lossy().to_string();
            res.push(str);
        }
    }
    Ok(res)
}

pub fn fusion_run() -> Result<(), Box<dyn std::error::Error>> {
    let json_files = read_json_files()?;

    let fusion = Fusion::asker()
        .json_a(&json_files)
        .json_b(&json_files)
        .output()
        .finish();

    let reg = regex::Regex::new(r"\{.*\}")?;
    let json_str1 = fs::read_to_string(&fusion.json_a)?;
    let mut json1 = reg
        .find_iter(&json_str1)
        .map(|i| serde_json::from_str::<IconInfo>(i.as_str()).unwrap())
        .collect::<Vec<_>>();
    let json_str2 = fs::read_to_string(&fusion.json_b)?;
    let json2 = reg
        .find_iter(&json_str2)
        .map(|i| serde_json::from_str::<IconInfo>(i.as_str()).unwrap())
        .collect::<Vec<_>>();

    let mut res: Vec<Value> = vec![];
    for i in 0..json1.len() {
        let value = json1[i].borrow_mut();
        let keys = value.category_key.split("_").collect::<Vec<_>>();
        value.category_key = keys.last().unwrap().to_string();
        let keys = value.title_key.split("_").collect::<Vec<_>>();
        value.title_key = keys.last().unwrap().to_string();
        let mut icon = json!(value);
        icon["enTitle"] = Value::String(json2[i].title.clone());
        icon["enCategoryTitle"] = Value::String(json2[i].category_title.clone());
        res.push(icon);
    }
    fs::write(fusion.output, serde_json::to_vec_pretty(&res).unwrap())?;
    Ok(())
}
