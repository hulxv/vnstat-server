use std::{
    io::{Error, ErrorKind::NotFound, Result},
    process::ExitStatus,
};
use systemctl::{exists, is_active, restart, stop};
pub struct VnStatDaemon;

const VNSTAT_SERVICE: &str = "vnstat";

impl VnStatDaemon {
    pub fn new() -> Result<Self> {
        if !exists(VNSTAT_SERVICE).unwrap() {
            return Err(Error::new(NotFound, "vnStat service not found."));
        }
        Ok(Self)
    }
    pub fn is_active(&self) -> Result<bool> {
        is_active(VNSTAT_SERVICE)
    }

    pub fn restart(&self) -> Result<ExitStatus> {
        restart(VNSTAT_SERVICE)
    }

    pub fn stop(&self) -> Result<ExitStatus> {
        stop(VNSTAT_SERVICE)
    }
}
