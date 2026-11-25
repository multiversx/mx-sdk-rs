//! Encoding of untrusted text (scenario files, ABI files, file names) into generated Rust code.
//!
//! Generated code gets compiled and executed, so such text must never be inserted as-is.
//! Each position in the code (literal, comment, identifier) has its own function here.

/// Rust keywords, including the reserved ones.
const KEYWORDS: &[&str] = &[
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if",
    "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub",
    "ref", "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Keywords which are not allowed as raw identifiers (`r#...`) either.
const NON_RAW_KEYWORDS: &[&str] = &["Self", "crate", "self", "super"];

/// Produces a complete Rust string literal, including the surrounding quotes.
pub fn string_literal(text: &str) -> String {
    format!("{text:?}")
}

/// Prepares text to be placed after `//`, by keeping it on a single line.
pub fn line_comment_text(text: &str) -> String {
    text.chars()
        .map(|c| if is_unsafe_in_comment(c) { ' ' } else { c })
        .collect()
}

/// Prepares text to be placed between `/*` and `*/`.
///
/// Keeps it on a single line and breaks up both comment delimiters,
/// since block comments can be nested in Rust.
pub fn block_comment_text(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for c in line_comment_text(text).chars() {
        if matches!(
            (result.chars().last(), c),
            (Some('*'), '/') | (Some('/'), '*')
        ) {
            result.push(' ');
        }
        result.push(c);
    }
    result
}

/// Control characters, line separators and bidirectional text overrides.
fn is_unsafe_in_comment(c: char) -> bool {
    c.is_control()
        || matches!(
            c,
            '\u{2028}' | '\u{2029}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}'
        )
}

/// Checks that the text can be used as-is as a Rust identifier, e.g. as a method name.
///
/// Only ASCII identifiers are accepted.
pub fn is_identifier(text: &str) -> bool {
    if let Some(raw) = text.strip_prefix("r#") {
        has_identifier_syntax(raw) && !NON_RAW_KEYWORDS.contains(&raw)
    } else {
        has_identifier_syntax(text) && !KEYWORDS.contains(&text)
    }
}

fn has_identifier_syntax(text: &str) -> bool {
    text != "_"
        && text.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && text.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Turns arbitrary text into a valid Rust identifier.
///
/// All unsupported characters are replaced with `_`.
/// A `_` is also added where the result would otherwise still not be a valid identifier.
pub fn sanitize_identifier(text: &str) -> String {
    let mut result: String = text
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();

    if result.is_empty() || result.starts_with(|c: char| c.is_ascii_digit()) {
        result.insert(0, '_');
    }

    // covers `_` and keywords
    if !is_identifier(&result) {
        result.push('_');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_literal_escapes() {
        assert_eq!(string_literal("abc"), r#""abc""#);
        assert_eq!(string_literal(r#"a"b\c"#), r#""a\"b\\c""#);
        assert_eq!(string_literal("a\nb\r\t"), r#""a\nb\r\t""#);
        assert_eq!(string_literal("a\u{202E}b"), r#""a\u{202e}b""#);
    }

    #[test]
    fn line_comment_single_line() {
        assert_eq!(line_comment_text("plain \"text\""), "plain \"text\"");
        assert_eq!(line_comment_text("a\nb\r\nc"), "a b  c");
        assert_eq!(line_comment_text("a\u{2028}b\u{202E}c"), "a b c");
    }

    #[test]
    fn block_comment_no_delimiters() {
        assert_eq!(block_comment_text("str:abc"), "str:abc");
        assert_eq!(block_comment_text("a */ b"), "a * / b");
        assert_eq!(block_comment_text("a /* b"), "a / * b");
        assert_eq!(block_comment_text("*/*/"), "* / * /");
        assert_eq!(block_comment_text("//**"), "// **");
        assert_eq!(block_comment_text("a\nb"), "a b");
    }

    #[test]
    fn identifier_check() {
        assert!(is_identifier("add"));
        assert!(is_identifier("_add_2"));
        assert!(is_identifier("addValue"));
        assert!(is_identifier("r#type"));

        assert!(!is_identifier(""));
        assert!(!is_identifier("_"));
        assert!(!is_identifier("2add"));
        assert!(!is_identifier("type"));
        assert!(!is_identifier("r#self"));
        assert!(!is_identifier("add()"));
        assert!(!is_identifier("a b"));
        assert!(!is_identifier("a::b"));
    }

    #[test]
    fn identifier_sanitize() {
        assert_eq!(sanitize_identifier("adder_scen"), "adder_scen");
        assert_eq!(sanitize_identifier("a-b.c d#e"), "a_b_c_d_e");
        assert_eq!(sanitize_identifier("x(){}"), "x____");
        assert_eq!(sanitize_identifier("1abc"), "_1abc");
        assert_eq!(sanitize_identifier("loop"), "loop_");
        assert_eq!(sanitize_identifier(""), "__");
        assert_eq!(sanitize_identifier("-"), "__");
    }
}
