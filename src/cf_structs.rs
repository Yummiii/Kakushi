use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Message {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CfResponse<T> {
    #[serde(default)]
    pub result: T,
    pub success: bool,
    #[serde(default)]
    pub messages: Vec<Message>,
    #[serde(default)]
    pub errors: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub content: String,
    pub name: String,
    pub ttl: i32,
    #[serde(rename = "type")]
    pub record_type: String,
}
