use crate::util::ToJsonValue;
use super::component::ComponentContainer;

#[derive(Clone, Copy, PartialEq)]
pub enum HoverEventAction {
    ShowText,
    ShowItem,
    ShowEntity,
}

impl HoverEventAction {
    pub fn name(&self) -> &str {
        match self {
            HoverEventAction::ShowText => "show_text",
            HoverEventAction::ShowItem => "show_item",
            HoverEventAction::ShowEntity => "show_entity",
        }
    }
}

#[derive(Clone)]
pub struct HoverEvent {
    pub action: HoverEventAction,
    pub value: Box<ComponentContainer>,
}

impl ToJsonValue for HoverEvent {
    fn to_json(&self) -> Option<serde_json::Value> {
        Some(json!({
            "action": self.action.name(),
            "value": self.value.to_json().unwrap(),
        }))
    }
}