use super::db::{models::Traffic, Database};
use anyhow::{anyhow, Result};

pub fn get_traffic(interval: String) -> Result<Vec<Traffic>> {
    if !is_validated_interval(interval.clone()) {
        return Err(anyhow!("invalid interval"));
    }
    match Database::default()?
        .connect()?
        .select_table::<Traffic>(interval.as_str())
    {
        Err(err) => Err(anyhow!(err)),
        Ok(result) => Ok(result),
    }
}

pub fn is_validated_interval(interval: String) -> bool {
    let available_intervals = Vec::from(["fiveminute", "hour", "day", "month", "year", "top"]);
    available_intervals.contains(&interval.as_str())
}
