pub trait JsonValue {
    fn to_json(&self) -> Option<serde_json::Value>;
}