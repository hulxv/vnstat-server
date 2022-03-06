use serde::Serialize;

use anyhow::{anyhow, Result};
use serde_json::json;
use std::fs;
use std::io::{
    Error,
    ErrorKind::{Interrupted, InvalidData, NotFound},
};

const VNSTAT_CONFIG_FILE_PATH: &str = "/etc/vnstat.test.conf";

#[derive(Debug, Clone, Serialize)]
struct ConfigProp {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, Serialize)]
struct VnStatConfigs {
    pub path: String,
    pub props: Option<Vec<ConfigProp>>,
}

impl VnStatConfigs {
    pub fn new(path: String) -> Self {
        VnStatConfigs { path, props: None }
    }
    pub fn default() -> Self {
        VnStatConfigs {
            path: VNSTAT_CONFIG_FILE_PATH.to_owned(),
            props: None,
        }
    }
    pub fn read_props(&mut self) -> Result<Self> {
        let file_content = fs::read_to_string(VNSTAT_CONFIG_FILE_PATH)?;
        let mut props: Vec<ConfigProp> = Vec::new();

        file_content.lines().into_iter().for_each(|line| {
            if !line.is_empty() && !line.starts_with("#") {
                let prop = line
                    .split(" ")
                    .filter(|e| !e.is_empty())
                    .collect::<Vec<&str>>();

                props.push(ConfigProp {
                    name: prop[0].to_owned(),
                    value: prop[1].to_owned(),
                });
            }
        });
        Ok(Self {
            props: Some(props),
            ..self.to_owned()
        })
    }
    pub fn get_props(&mut self) -> Result<Vec<ConfigProp>> {
        match self.read_props() {
            Ok(e) => match e.to_owned().props {
                Some(props) => Ok(props),

                None => Err(anyhow!(Error::new(
                    NotFound,
                    "No props is found, Please check if vnstat.conf is exist or not.",
                ))),
            },
            Err(err) => Err(anyhow!(err)),
        }
    }
}

#[test]
fn read_vnstat_config_file() {
    let props = VnStatConfigs::default().get_props().unwrap();

    for prop in props {
        println!("{}: {}", prop.name, prop.value);
    }

    assert!(true)
}
