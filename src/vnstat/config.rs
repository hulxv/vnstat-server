use crate::app::config::Configs;
use anyhow::{anyhow, Result};
use serde::{
    ser::{self, SerializeMap, SerializeSeq, Serializer},
    Serialize,
};
use serde_json::json;
use std::io::{
    Error,
    ErrorKind::{Interrupted, InvalidData, NotFound},
};
use std::{collections::HashMap, fs};

#[derive(Debug, Serialize, Clone)]
pub struct VnStatConfigs;
    // pub props: Option<HashMap<&'static str, &'static str>>,


impl VnStatConfigs {
    
    pub fn props() -> Result<HashMap<String,String>> {
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
    let props = VnStatConfigs::props().unwrap();
    println!("{}", json!(props));
    assert!(true)
}
