pub trait ToJsonValue {
    fn to_json(&self) -> Option<serde_json::Value>;
}