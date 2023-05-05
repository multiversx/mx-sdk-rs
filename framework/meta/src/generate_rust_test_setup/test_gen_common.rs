const UNDERSCORE: char = '_';
const MINUS: char = '-';

pub(crate) fn to_camel_case(input: String) -> String {
    let mut output = String::new();
    let mut iterator = input.chars().peekable();
    while let Some(character) = iterator.next() {
        if character.is_ascii_lowercase() || character.is_ascii_uppercase() {
            output.push(character);
            continue;
        }
        if character != UNDERSCORE && character != MINUS {
            continue;
        }

        let opt_next_character = iterator.peek();
        if opt_next_character.is_none() {
            break;
        }

        let next_character = unsafe { opt_next_character.unwrap_unchecked() };
        if !next_character.is_ascii_lowercase() {
            continue;
        }

        output.push(next_character.to_ascii_uppercase());
        let _ = iterator.advance_by(1); // will never fail, as we've already peeked

        // ignore special characters
    }

    output
}

pub(crate) fn capitalize_first_letter(string: &mut String) {
    if let Some(first_letter_slice) = string.get_mut(0..1) {
        first_letter_slice.make_ascii_uppercase();
    }
}

pub(crate) fn is_last_element<T>(array: &[T], index: usize) -> bool {
    index == array.len() - 1
}
