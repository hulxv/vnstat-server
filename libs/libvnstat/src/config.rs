use app::Configs;

use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs,
    process::{ExitStatus, Stdio},
};
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct VnStatConfig {
    path: String,
}

impl VnStatConfig {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    pub fn get_props(&self) -> Result<HashMap<String, String>> {
        let mut props: HashMap<String, String> = HashMap::new();
        let file_content = fs::read_to_string(&self.path)?;
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

    pub async fn set_prop(&self, key: &str, value: &str) -> Result<ExitStatus> {
        let sed_script = format!("s/.*{key} .*/{key} {value}/g");
        let status = Command::new("sed")
            .args(vec!["-i", &sed_script, &self.path])
            .stdout(Stdio::null())
            .spawn()?
            .wait_with_output()
            .await?
            .status;

        if !status.success() {
            return Err(anyhow!("{}:  operation doesn't success", status));
        }
        Ok(status)
    }
}

impl Default for VnStatConfig {
    fn default() -> Self {
        Self {
            path: Configs::init().unwrap().vnstat().config_file(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{copy, File},
        path::Path,
    };
    use tokio::test as async_test;
    #[test]
    fn read_vnstat_config_file() {
        // use serde_json::{json, to_string_pretty};
        let props = VnStatConfig::default().get_props().unwrap();
        println!("{:#?}", props);
        assert!(true)
    }

    #[async_test]
    async fn edit_prop_in_vnstat_config_file() {
        let test_config_file = Path::new("/etc/vnstat.test.conf");
        if !test_config_file.exists() {
            File::create(test_config_file).unwrap();
            copy("/etc/vnstat.conf", test_config_file.to_str().unwrap()).unwrap();
        }

        VnStatConfig::new(test_config_file.to_str().unwrap())
            .set_prop("List5Mins", "99")
            .await
            .map_err(|e| println!("{e}"))
            .unwrap();

        let props = VnStatConfig::new(test_config_file.to_str().unwrap())
            .get_props()
            .unwrap();

        assert_eq!(props.get("List5Mins").unwrap().as_str(), "105")
    }
}
