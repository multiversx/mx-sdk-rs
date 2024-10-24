use std::fmt::Display;

use wasmparser::DataSectionReader;
const PANIC_WITH_MESSAGE: &[u8; 16] = b"panic occurred: ";
const PANIC_WITHOUT_MESSAGE: &[u8; 14] = b"panic occurred";

#[derive(Default, PartialEq, Clone)]
pub enum PanicReport {
    #[default]
    None,
    WithoutMessage,
    WithMessage,
}

impl Display for PanicReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let panic_status = match self {
            PanicReport::None => "None",
            PanicReport::WithoutMessage => "without message",
            PanicReport::WithMessage => "with message",
        };
        write!(f, "{}", panic_status)
    }
}

impl PanicReport {
    pub fn data_section_severity(&self, data_section: DataSectionReader) -> Self {
        if is_panic_with_message_triggered(data_section.clone()) {
            return Self::WithMessage;
        }

        if is_panic_without_message_triggered(data_section) {
            return Self::WithoutMessage;
        }

        Self::None
    }

    pub fn max_severity(&mut self, data_section: DataSectionReader) {
        if *self == PanicReport::WithMessage {
            return;
        }

        let panic_report = self.data_section_severity(data_section);
        if panic_report == PanicReport::None {
            return;
        }

        *self = panic_report;
    }
}

fn is_panic_with_message_triggered(data_section: DataSectionReader) -> bool {
    for data_fragment in data_section.into_iter().flatten() {
        if data_fragment
            .data
            .windows(PANIC_WITH_MESSAGE.len())
            .any(|data| data == PANIC_WITH_MESSAGE)
        {
            return true;
        }
    }

    false
}

fn is_panic_without_message_triggered(data_section: DataSectionReader) -> bool {
    for data_fragment in data_section.into_iter().flatten() {
        if data_fragment
            .data
            .windows(PANIC_WITHOUT_MESSAGE.len())
            .any(|data| data == PANIC_WITHOUT_MESSAGE)
        {
            return true;
        }
    }

    false
}
