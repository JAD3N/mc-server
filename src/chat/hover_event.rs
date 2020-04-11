use super::component::Component;

#[derive(Clone, Copy, PartialEq)]
pub enum HoverEventAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}
pub struct HoverEvent {
    pub action: HoverEventAction,
    pub value: Box<dyn Component>,
}