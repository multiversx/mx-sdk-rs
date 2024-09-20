use std::fmt::Display;

#[derive(Default)]
pub enum PanicMessage {
    #[default]
    None,
    WithoutMessage,
    WithMessage,
}

impl Display for PanicMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let panic_status = match self {
            PanicMessage::None => "None",
            PanicMessage::WithoutMessage => "without message",
            PanicMessage::WithMessage => "with message",
        };
        write!(f, "{}", panic_status)
    }
}

pub struct ReportCreator {
    pub path: String,
    pub has_allocator: bool,
    pub has_panic: PanicMessage,
}

impl ReportCreator {}
