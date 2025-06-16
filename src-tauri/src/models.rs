use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationMilliSeconds, TimestampSeconds};

pub trait Model {}

#[serde_as]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "camelCase")]
pub struct Solve {
    #[serde_as(as = "DurationMilliSeconds<i64>")]
    pub solve_time: TimeDelta,

    #[serde_as(as = "TimestampSeconds<i64>")]
    pub timestamp: DateTime<Utc>,
}

impl Model for Solve {}
