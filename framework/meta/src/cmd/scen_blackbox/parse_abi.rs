use multiversx_sc::abi::TypeName;

/// Parses a complex ABI type string into its base type name and a list of type arguments.
///
/// Handles generic/parameterized types such as:
/// - `multi<A, B, C>`        → `("multi",        ["A", "B", "C"])`
/// - `variadic<T>`           → `("variadic",      ["T"])`
/// - `OptionalValue<T>`      → `("OptionalValue", ["T"])`
/// - `List<T>`               → `("List",          ["T"])`
/// - `u64` (plain type)      → `("u64",           [])`
/// - `array32<u8>`           → `("array",         ["32", "u8"])`
///
/// For `arrayN<T>` types the numeric size is extracted as the first type argument.
/// Nested angle brackets are handled correctly, e.g.
/// `variadic<multi<u64, Address>>` → `("variadic", ["multi<u64, Address>"])`.
pub(super) fn parse_abi_type(abi_type: &str) -> (TypeName, Vec<TypeName>) {
    // Special case: arrayN<T> — e.g. "array32<u8>"
    if let Some(rest) = abi_type.strip_prefix("array") {
        if let Some(open) = rest.find('<') {
            let size_str = &rest[..open];
            if !size_str.is_empty()
                && size_str.chars().all(|c| c.is_ascii_digit())
                && rest.ends_with('>')
            {
                let inner = &rest[open + 1..rest.len() - 1];
                let mut type_args = vec![size_str.to_string()];
                type_args.extend(parse_multi_fields(inner));
                return ("array".to_string(), type_args);
            }
        }
    }

    // Find the first '<' that opens the type-argument list.
    let Some(open) = abi_type.find('<') else {
        // No angle bracket – plain type with no arguments.
        return (abi_type.to_string(), Vec::new());
    };

    // The last character must close the outermost bracket.
    if !abi_type.ends_with('>') {
        return (abi_type.to_string(), Vec::new());
    }

    let base = abi_type[..open].trim().to_string();
    let inner = &abi_type[open + 1..abi_type.len() - 1];
    let type_args = parse_multi_fields(inner);

    (base, type_args)
}

/// Parses a comma-separated list of type names, respecting nested angle brackets.
///
/// Used to split `multi<A,B,...>` or the contents of any parameterized ABI type.
pub(super) fn parse_multi_fields(s: &str) -> Vec<TypeName> {
    let mut fields = Vec::new();
    let mut depth = 0;
    let mut current = String::new();

    for ch in s.chars() {
        match ch {
            '<' => {
                depth += 1;
                current.push(ch);
            }
            '>' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                fields.push(current.trim().to_string());
                current = String::new();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        fields.push(trimmed);
    }

    fields
}

#[cfg(test)]
mod tests {
    use super::parse_abi_type;

    fn parse(s: &str) -> (String, Vec<String>) {
        parse_abi_type(s)
    }

    #[test]
    fn plain_types_have_no_args() {
        assert_eq!(parse("u64"), ("u64".to_string(), vec![]));
        assert_eq!(parse("bool"), ("bool".to_string(), vec![]));
        assert_eq!(parse("BigUint"), ("BigUint".to_string(), vec![]));
        assert_eq!(parse("Address"), ("Address".to_string(), vec![]));
    }

    #[test]
    fn variadic_single_arg() {
        assert_eq!(
            parse("variadic<T>"),
            ("variadic".to_string(), vec!["T".to_string()])
        );
        assert_eq!(
            parse("variadic<u64>"),
            ("variadic".to_string(), vec!["u64".to_string()])
        );
    }

    #[test]
    fn optional_value() {
        assert_eq!(
            parse("OptionalValue<u32>"),
            ("OptionalValue".to_string(), vec!["u32".to_string()])
        );
    }

    #[test]
    fn multi_multiple_args() {
        assert_eq!(
            parse("multi<u64, Address, BigUint>"),
            (
                "multi".to_string(),
                vec![
                    "u64".to_string(),
                    "Address".to_string(),
                    "BigUint".to_string()
                ]
            )
        );
    }

    #[test]
    fn list_single_arg() {
        assert_eq!(
            parse("List<TokenIdentifier>"),
            ("List".to_string(), vec!["TokenIdentifier".to_string()])
        );
    }

    #[test]
    fn array_with_size() {
        assert_eq!(
            parse("array32<u8>"),
            (
                "array".to_string(),
                vec!["32".to_string(), "u8".to_string()]
            )
        );
        assert_eq!(
            parse("array16<TokenIdentifier>"),
            (
                "array".to_string(),
                vec!["16".to_string(), "TokenIdentifier".to_string()]
            )
        );
        assert_eq!(
            parse("array1<bool>"),
            (
                "array".to_string(),
                vec!["1".to_string(), "bool".to_string()]
            )
        );
    }

    #[test]
    fn nested_variadic_multi() {
        assert_eq!(
            parse("variadic<multi<u64, Address>>"),
            (
                "variadic".to_string(),
                vec!["multi<u64, Address>".to_string()]
            )
        );
    }

    #[test]
    fn nested_optional_list() {
        assert_eq!(
            parse("OptionalValue<List<u64>>"),
            ("OptionalValue".to_string(), vec!["List<u64>".to_string()])
        );
    }

    #[test]
    fn deeply_nested() {
        assert_eq!(
            parse("variadic<multi<u64, List<Address>>>"),
            (
                "variadic".to_string(),
                vec!["multi<u64, List<Address>>".to_string()]
            )
        );
    }

    #[test]
    fn malformed_unclosed_bracket_treated_as_plain() {
        // Missing closing '>' — falls back to plain type
        let (base, args) = parse("variadic<u64");
        assert_eq!(base, "variadic<u64");
        assert!(args.is_empty());
    }
}
