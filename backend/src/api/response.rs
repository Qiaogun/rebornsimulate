use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub contry: String,
    pub data: Value,
}