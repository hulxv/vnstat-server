use super::db::{models::Traffic as TrafficModel, VnStatDatabase};
use anyhow::{anyhow, Result};
use std::io::{Error, ErrorKind::InvalidInput};

pub struct VnStatTraffic {
    interval: TrafficInterval,
}

impl VnStatTraffic {
    pub fn new(interval: &str) -> Self {
        Self {
            interval: TrafficInterval::new(interval),
        }
    }
    pub fn get(&self) -> Result<Vec<TrafficModel>> {
        if !self.interval.clone().is_validated() {
            return Err(anyhow!(Error::new(InvalidInput, "invalid interval")));
        }
        match VnStatDatabase::default()?
            .connect()?
            .select_table::<TrafficModel>(self.interval.get())
        {
            Err(err) => Err(anyhow!(err)),
            Ok(result) => Ok(result),
        }
    }
}

#[derive(Clone)]
pub struct TrafficInterval {
    interval: String,
}

impl TrafficInterval {
    pub fn new(interval: &str) -> Self {
        TrafficInterval {
            interval: interval.to_string(),
        }
    }
    pub fn is_validated(&self) -> bool {
        vec!["fiveminute", "hour", "day", "month", "year", "top"].contains(&self.get())
    }
    fn get(&self) -> &str {
        self.interval.as_str()
    }
}

#[test]

pub fn interval_validation() {
    assert_eq!(TrafficInterval::new("top").is_validated(), true)
}
