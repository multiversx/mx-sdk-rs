const OPEN_BRACE: u8 = b'{';
const CLOSED_BRACE: u8 = b'}';
const TWO_DOTS: u8 = b':';
const X_LETTER: u8 = b'x';

const UNMATCHED_BRACE_ERR_MSG: &str = "Unmatched `{` in the format string";

pub enum FormatPartType {
    StaticAscii(String),
    Hex,
    Ascii,
}

pub fn parse_format_string(raw_string: &str) -> Vec<FormatPartType> {
    if !raw_string.is_ascii() {
        panic!("Only ASCII strings allowed");
    }

    let ascii_bytes = raw_string.as_bytes();
    let mut parts = Vec::new();
    let mut start_index = 1;

    // starting from 1 and up to len - 1 to skip the ""
    let str_len = ascii_bytes.len() - 1;
    for i in 1..str_len {
        if ascii_bytes[i] != OPEN_BRACE {
            continue;
        }

        match ascii_bytes.get(i + 1) {
            Some(byte) => match *byte {
                CLOSED_BRACE => {
                    if i > 1 {
                        let end_index = i - 1;
                        if start_index != end_index {
                            let static_part = &ascii_bytes[start_index..=end_index];
                            let as_str = String::from_utf8(static_part.to_vec()).unwrap();
                            parts.push(FormatPartType::StaticAscii(as_str));
                        }
                    }

                    parts.push(FormatPartType::Ascii);

                    start_index = i + 2;
                },
                TWO_DOTS => {
                    match ascii_bytes.get(i + 2) {
                        Some(x_letter) => {
                            if *x_letter != X_LETTER {
                                panic!("{}", UNMATCHED_BRACE_ERR_MSG);
                            }
                        },
                        None => panic!("{}", UNMATCHED_BRACE_ERR_MSG),
                    }
                    match ascii_bytes.get(i + 3) {
                        Some(closed_brace) => {
                            if *closed_brace != CLOSED_BRACE {
                                panic!("{}", UNMATCHED_BRACE_ERR_MSG);
                            }
                        },
                        None => panic!("{}", UNMATCHED_BRACE_ERR_MSG),
                    }

                    if i > 1 {
                        let end_index = i - 1;
                        if start_index != end_index {
                            let static_part = &ascii_bytes[start_index..=end_index];
                            let as_str = String::from_utf8(static_part.to_vec()).unwrap();
                            parts.push(FormatPartType::StaticAscii(as_str));
                        }
                    }

                    parts.push(FormatPartType::Hex);

                    start_index = i + 4;
                },
                _ => panic!("{}", UNMATCHED_BRACE_ERR_MSG),
            },
            None => panic!("{}", UNMATCHED_BRACE_ERR_MSG),
        }
    }

    if start_index < str_len {
        let static_part = &ascii_bytes[start_index..str_len];
        let as_str = String::from_utf8(static_part.to_vec()).unwrap();
        parts.push(FormatPartType::StaticAscii(as_str));
    }

    parts
}

pub(crate) fn count_args(format_types: &[FormatPartType]) -> usize {
    let mut nr_args = 0;
    for f in format_types {
        match *f {
            FormatPartType::Ascii => nr_args += 1,
            FormatPartType::Hex => nr_args += 1,
            FormatPartType::StaticAscii(_) => {},
        }
    }

    nr_args
}
