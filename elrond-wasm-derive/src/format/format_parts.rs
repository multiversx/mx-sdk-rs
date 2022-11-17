const OPEN_BRACE: u8 = b'{';
const CLOSED_BRACE: u8 = b'}';
const TWO_DOTS: u8 = b':';
const X_LETTER: u8 = b'x';
const B_LETTER: u8 = b'b';
const C_LETTER: u8 = b'c';

const UNMATCHED_BRACE_ERR_MSG: &str = "Unmatched `{` in the format string";

#[derive(Debug, PartialEq, Eq)]
pub enum FormatPartType {
    StaticAscii(String),
    LowerHex,
    Display,
    Codec,
    Bytes,
}

pub fn parse_format_string(raw_string: &str) -> Vec<FormatPartType> {
    if !raw_string.is_ascii() {
        panic!("Only ASCII strings allowed");
    }

    let ascii_bytes = raw_string.as_bytes();
    let mut parts = Vec::new();
    let mut start_index = 1;
    let mut format_byte;

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
                        if start_index <= end_index {
                            let static_part = &ascii_bytes[start_index..=end_index];
                            let as_str = String::from_utf8(static_part.to_vec()).unwrap();
                            parts.push(FormatPartType::StaticAscii(as_str));
                        }
                    }

                    parts.push(FormatPartType::Display);

                    start_index = i + 2;
                },
                TWO_DOTS => {
                    match ascii_bytes.get(i + 2) {
                        Some(letter) => {
                            if *letter != X_LETTER && *letter != B_LETTER && *letter != C_LETTER {
                                panic!("{}", UNMATCHED_BRACE_ERR_MSG);
                            }
                            format_byte = letter;
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
                    if *format_byte == X_LETTER {
                        parts.push(FormatPartType::LowerHex);
                    } else if *format_byte == B_LETTER {
                        parts.push(FormatPartType::Bytes);
                    } else if *format_byte == C_LETTER {
                        parts.push(FormatPartType::Codec);
                    }

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
            FormatPartType::Display => nr_args += 1,
            FormatPartType::LowerHex => nr_args += 1,
            FormatPartType::Codec => nr_args += 1,
            FormatPartType::Bytes => nr_args += 1,
            FormatPartType::StaticAscii(_) => {},
        }
    }

    nr_args
}

#[test]
fn test_format_parts_single_char_delimiter() {
    assert_eq!(
        parse_format_string("\"test{}/{}.json\""),
        vec![
            FormatPartType::StaticAscii("test".to_string()),
            FormatPartType::Display,
            FormatPartType::StaticAscii("/".to_string()),
            FormatPartType::Display,
            FormatPartType::StaticAscii(".json".to_string())
        ]
    );
}

#[test]
fn test_format_parts_no_delimiter() {
    assert_eq!(
        parse_format_string("\"{}{}.txt\""),
        vec![
            FormatPartType::Display,
            FormatPartType::Display,
            FormatPartType::StaticAscii(".txt".to_string())
        ]
    );
}

#[test]
fn test_format_parts_multiple_chars_between_displays() {
    assert_eq!(
        parse_format_string("\"{}test{}\""),
        vec![
            FormatPartType::Display,
            FormatPartType::StaticAscii("test".to_string()),
            FormatPartType::Display
        ]
    );
}
