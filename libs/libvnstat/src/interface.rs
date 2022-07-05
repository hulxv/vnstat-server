use super::db::{models::Interface, VnStatDatabase};
use anyhow::{anyhow, Result};
pub struct VnStatInterface;

impl VnStatInterface {
    pub fn get(&self) -> Result<Vec<Interface>> {
        Ok(VnStatDatabase::default()?
            .connect()?
            .select_table::<Interface>("interface")?)
    }
}

#[test]
fn get_vnstat_interface_list() {
    println!("{:#?}", VnStatInterface.get());
    assert!(VnStatInterface.get().is_ok())
}
