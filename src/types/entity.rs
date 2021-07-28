use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "bots" )]
    Bot,
    #[serde(rename = "servers" )]
    Server,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    pub name: String,
}

