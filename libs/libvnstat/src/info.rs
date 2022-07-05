use super::db::{models::Info, VnStatDatabase};
use anyhow::{anyhow, Result};
pub struct VnStatInfo;

impl VnStatInfo {
    pub fn get(&self) -> Result<Vec<Info>> {
        Ok(VnStatDatabase::default()?
            .connect()?
            .select_table::<Info>("info")?)
    }
}

#[test]
fn get_vnstat_info() {
    println!("{:#?}", VnStatInfo.get());
    assert!(VnStatInfo.get().is_ok())
}
