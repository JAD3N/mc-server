#[derive(Clone, Copy, PartialEq)]
pub enum HoverEventAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

#[derive(Clone, PartialEq)]
pub struct HoverEvent {
    pub action: HoverEventAction,
    // pub value: ComponentContainer,
}