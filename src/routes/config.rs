use rocket::*;
use rocket_contrib::json::Json;
use serde::{ser::SerializeTupleStruct, Serialize};
use std::fs;

use std::io::{
    Error,
    ErrorKind::{Interrupted, InvalidData, NotFound},
};

const VNSTAT_CONFIG_FILE_PATH: &str = "/etc/vnstat.test.conf";

#[get("/config")]
pub fn get_configs() -> Result<(), String> {
    todo!()
}

#[post("/config")]
pub fn edit_configs() -> Result<(), String> {
    todo!()
}

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
    pub fn read_props(&mut self) -> Result<&mut Self, Error> {
        let file_content = fs::read_to_string(VNSTAT_CONFIG_FILE_PATH).unwrap();
        let mut props: Vec<ConfigProp> = Vec::new();
        file_content.lines().into_iter().for_each(|line| {
            if !line.is_empty() && !line.starts_with("#") {
                let prop = line
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .filter(|e| !e.is_empty())
                    .collect::<Vec<&str>>();
                props.push(ConfigProp {
                    name: prop[0].to_owned(),
                    value: prop[1].to_owned(),
                });
            }
        });
        self.props = Some(props);
        Ok(self)
    }
    pub fn get_props(&mut self) -> Result<Vec<ConfigProp>, Error> {
        match self.read_props() {
            Ok(e) => match e.to_owned().props {
                Some(props) => Ok(props),

                None => Err(Error::new(
                    NotFound,
                    "No props is found, Please check if vnstat.conf is exist or not.",
                )),
            },
            Err(err) => Err(err),
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
