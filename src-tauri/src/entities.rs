use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{DurationMilliSeconds, TimestampSeconds, serde_as};

pub trait Entity {}

#[serde_as]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Solve {
    #[serde(default)]
    pub id: i64,

    #[serde_as(as = "DurationMilliSeconds<i64>")]
    pub solve_time: TimeDelta,

    #[serde_as(as = "TimestampSeconds<i64>")]
    pub timestamp: DateTime<Utc>,
}

impl Entity for Solve {}
