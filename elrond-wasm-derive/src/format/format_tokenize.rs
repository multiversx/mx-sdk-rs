use proc_macro::Group;

fn token_tree_is_comma(tt: &proc_macro::TokenTree) -> bool {
    if let proc_macro::TokenTree::Punct(punct) = &tt {
        punct.as_char() == ','
    } else {
        false
    }
}

fn flush_token_buffer(
    output: &mut Vec<proc_macro::TokenTree>,
    mut buffer: Vec<proc_macro::TokenTree>,
) {
    match buffer.len() {
        0 => panic!("empty tokens not allowed in push_format macro"),
        1 => output.append(&mut buffer),
        _ => output.push(proc_macro::TokenTree::Group(Group::new(
            proc_macro::Delimiter::Parenthesis,
            buffer.into_iter().collect(),
        ))),
    }
}

pub fn tokenize(input: proc_macro::TokenStream) -> Vec<proc_macro::TokenTree> {
    let mut buffer = Vec::new();
    let mut output = Vec::new();
    for tt in input.into_iter() {
        if token_tree_is_comma(&tt) {
            flush_token_buffer(&mut output, core::mem::take(&mut buffer));
        } else {
            buffer.push(tt);
        }
    }
    flush_token_buffer(&mut output, buffer);
    output
}
