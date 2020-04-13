use crate::util::JsonValue;

#[derive(Clone, Copy, PartialEq)]
pub enum ClickEventAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

impl ClickEventAction {
    pub fn name(&self) -> &str {
        match self {
            ClickEventAction::OpenUrl => "open_url",
            ClickEventAction::OpenFile => "open_file",
            ClickEventAction::RunCommand => "run_command",
            ClickEventAction::SuggestCommand => "suggest_command",
            ClickEventAction::ChangePage => "change_page",
            ClickEventAction::CopyToClipboard => "copy_to_clipboard",
        }
    }
}

pub struct ClickEvent {
    pub action: ClickEventAction,
    pub value: String,
}

impl JsonValue for ClickEvent {
    fn to_json(&self) -> Option<serde_json::Value> {
        Some(json!({
            "action": self.action.name(),
            "value": &self.value,
        }))
    }
}