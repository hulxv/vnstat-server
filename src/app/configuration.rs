use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{Error, ErrorKind::Interrupted, Write},
    path::Path,
};

use config::Config;
use dirs;

use crate::utils::create_file;

struct AppConfigProp {
    name: String,
    value: String,
}

impl AppConfigProp {
    fn from_hashmap(props: HashMap<&str, &str>) -> Vec<AppConfigProp> {
        let mut container: Vec<AppConfigProp> = Vec::new();
        for prop in props {
            container.push(AppConfigProp {
                name: prop.0.to_owned(),
                value: prop.1.to_owned(),
            })
        }
        container
    }
    fn as_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Debug for AppConfigProp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}\n", self.name, self.value)
    }
}
impl fmt::Display for AppConfigProp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}\n", self.name, self.value)
    }
}

struct AppConfigs {
    props: Option<Vec<AppConfigProp>>,
}

impl AppConfigs {
    fn init() -> Result<Self, Error> {
        let config_dir = match dirs::config_dir() {
            Some(path) => path.into_os_string().into_string(),
            None => panic!("Can't find \"~/.config\" directory"),
        };
        let file_path = [config_dir.unwrap(), "/vcs/vcs.config.toml".to_owned()].concat();
        match Path::new(&file_path).exists() {
            false => {
                let mut file = create_file(&file_path).unwrap();

                let mut props_as_string = String::from("");

                Self::default_props()
                    .iter()
                    .for_each(|prop| props_as_string.push_str(format!("{}", prop).as_str()));

                match file.write_all(props_as_string.as_bytes()) {
                    Err(e) => Err(e),
                    Ok(_) => {
                        println!(
                            "Configuration file was created successfully (in {})",
                            file_path
                        );
                        Ok(())
                    }
                };
            }
            _ => (),
        };

        Ok(AppConfigs { props: None })
    }
    fn get_props(&self) -> Result<Vec<AppConfigProp>, Error> {
        todo!()
    }
    fn default_props() -> Vec<AppConfigProp> {
        let default_props: HashMap<&str, &str> =
            HashMap::from([("address", "0.0.0.0"), ("port", "8888")]);

        AppConfigProp::from_hashmap(default_props)
    }
    fn reset_props(&self) -> Result<(), Error> {
        todo!()
    }
}

#[test]

fn build_configuration_file() {
    AppConfigs::init();

    assert!(true)
}
