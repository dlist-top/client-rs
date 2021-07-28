use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VoteData {
    #[serde(rename = "authorID")]
    pub author_id: String,
    pub entity_type: String,
    #[serde(rename = "entityID")]
    pub entity_id: String,
    #[serde(with = "ts_milliseconds")]
    pub date: DateTime<Utc>,
    pub total_votes: i32,
    pub user_votes: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RateData {
    #[serde(rename = "authorID")]
    pub author_id: String,
    pub entity_type: String,
    #[serde(rename = "entityID")]
    pub entity_id: String,
    #[serde(with = "ts_milliseconds")]
    pub date: DateTime<Utc>,
    pub rating: i8,
    pub details: String,
}