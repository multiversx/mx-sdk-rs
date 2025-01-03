pub fn extract_number_data(input: syn::LitStr) -> (u64, usize) {
    let value_str = input.value();

    let parts: Vec<&str> = value_str.split('.').collect();
    let raw_val = parts.join("");
    let raw_int = raw_val.parse::<u64>().expect("Invalid integer value");

    let decimals = if parts.len() > 1 { parts[1].len() } else { 0 };

    (raw_int, decimals)
}
