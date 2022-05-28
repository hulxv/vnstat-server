#[macro_use]
extern crate diesel;
pub mod config;
pub mod daemon;
pub mod db;
pub mod info;
pub mod interface;
pub mod traffic;

pub use config::*;
pub use daemon::*;
pub use db::*;
pub use info::*;
pub use interface::*;
pub use traffic::*;

pub struct VnStat;

impl VnStat {
    pub fn database(&self) -> VnStatDatabase {
        todo!()
    }
    pub fn config(&self) -> VnStatConfig {
        todo!()
    }
    pub fn info(&self) -> VnStatInfo {
        todo!()
    }
    pub fn interface(&self) -> VnStatInterface {
        todo!()
    }
    pub fn traffic(&self, interval: &str) -> VnStatTraffic {
        VnStatTraffic::new(interval)
    }
    pub fn daemon(&self) -> VnStatDaemon {
        todo!()
    }
}
