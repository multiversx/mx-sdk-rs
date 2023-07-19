/// The WebAssembly page size, in bytes.
pub const WASM_PAGE_SIZE: usize = 65536;

pub const DEFAULT_STACK_SIZE: usize = 2 * WASM_PAGE_SIZE;

pub const STACK_SIZE_SUFFIX_KILO: &str = "k";
pub const STACK_SIZE_SUFFIX_PAGES: &str = "pages";
pub const STACK_SIZE_MULIPLIER_KILO: usize = 1024;

pub fn parse_stack_size(stack_size: &Option<String>) -> usize {
    if let Some(stack_size_str) = stack_size {
        parse_stack_size_expr(stack_size_str)
    } else {
        DEFAULT_STACK_SIZE
    }
}

fn parse_stack_size_expr(stack_size_str: &str) -> usize {
    if let Some(s) = stack_size_str.strip_suffix(STACK_SIZE_SUFFIX_KILO) {
        parse_stack_size_str(s) * STACK_SIZE_MULIPLIER_KILO
    } else if let Some(s) = stack_size_str.strip_suffix(STACK_SIZE_SUFFIX_PAGES) {
        parse_stack_size_str(s) * WASM_PAGE_SIZE
    } else {
        parse_stack_size_str(stack_size_str)
    }
}

fn parse_stack_size_str(s: &str) -> usize {
    s.trim()
        .parse()
        .unwrap_or_else(|_| panic!("could not parse stack size expression: {s}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stack_size_expr() {
        assert_eq!(parse_stack_size_expr("1234"), 1234);
        assert_eq!(parse_stack_size_expr("1k"), 1024);
        assert_eq!(parse_stack_size_expr("2  k"), 2 * 1024);
        assert_eq!(parse_stack_size_expr(" 10 k"), 10 * 1024);
        assert_eq!(parse_stack_size_expr("1 pages"), 65536);
        assert_eq!(parse_stack_size_expr("2 pages"), 65536 * 2);
        assert_eq!(parse_stack_size_expr("10 pages"), 65536 * 10);
    }
}
