use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum GatewayOp {
    Hello = 1,
    Identify = 2,
    Ready = 3,
    Disconnect = 4,
    Event = 5,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub op: GatewayOp,
    pub data: serde_json::Value,
    #[serde(default)]
    pub event: String,
}