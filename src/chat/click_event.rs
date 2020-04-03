#[derive(Clone, Copy, PartialEq)]
pub enum ClickEventAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

#[derive(Clone, PartialEq)]
pub struct ClickEvent {
    pub action: ClickEventAction,
    pub value: String,
}