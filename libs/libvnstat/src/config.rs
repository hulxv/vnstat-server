use app::Configs;

use anyhow::Result;
use serde::Serialize;

use std::{collections::HashMap, fs};

#[derive(Debug, Serialize, Clone)]
pub struct VnStatConfig;

impl VnStatConfig {
    pub fn get_props(&self) -> Result<HashMap<String, String>> {
        let mut props: HashMap<String, String> = HashMap::new();
        let file_content = fs::read_to_string(Configs::init()?.vnstat.config_file.as_str())?;
        for line in file_content.lines() {
            if !line.is_empty() && !line.starts_with("#") {
                let prop = line
                    .split(" ")
                    .filter(|e| !e.is_empty())
                    .collect::<Vec<&str>>();
                props.insert(prop[0].to_owned(), prop[1].to_owned());
            }
        }
        Ok(props)
    }
}

#[test]
fn read_vnstat_config_file() {
    use serde_json::{json, to_string_pretty};
    let props = VnStatConfig.get_props().unwrap();
    println!("{:#?}", to_string_pretty(&json!(props)).unwrap());
    assert!(true)
}
