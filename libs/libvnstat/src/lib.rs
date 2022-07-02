#[macro_use]
extern crate diesel;
pub mod config;
pub mod daemon;
pub mod db;
pub mod info;
pub mod interface;
pub mod traffic;

use anyhow::Result;

pub use config::*;
pub use daemon::*;
pub use db::*;
pub use info::*;
pub use interface::*;
pub use traffic::*;

pub struct VnStat;

impl VnStat {
    pub fn database(&self) -> Result<VnStatDatabase> {
        VnStatDatabase::default()
    }
    pub fn config(&self) -> VnStatConfig {
        VnStatConfig::default()
    }
    pub fn info(&self) -> VnStatInfo {
        VnStatInfo
    }
    pub fn interface(&self) -> VnStatInterface {
        VnStatInterface
    }
    pub fn traffic(&self, interval: &str) -> VnStatTraffic {
        VnStatTraffic::new(interval)
    }
    pub fn daemon(&self) -> VnStatDaemon {
        VnStatDaemon
    }
}
