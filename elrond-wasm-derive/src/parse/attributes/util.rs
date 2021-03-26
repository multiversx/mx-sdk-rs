pub(super) fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
	attrs.iter().any(|attr| {
		if let Some(first_seg) = attr.path.segments.first() {
			return first_seg.ident == name;
		};
		false
	})
}

pub(super) fn attr_one_string_arg(attr: &syn::Attribute) -> String {
	let result_str: String;
	let mut iter = attr.clone().tokens.into_iter();
	match iter.next() {
		Some(proc_macro2::TokenTree::Group(group)) => {
			if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
				panic!("annotation paranthesis expected (check events and storage)");
			}
			let mut iter2 = group.stream().into_iter();
			match iter2.next() {
				Some(proc_macro2::TokenTree::Literal(lit)) => {
					let str_val = lit.to_string();
					if !str_val.starts_with('\"') || !str_val.ends_with('\"') {
						panic!("string literal expected as attribute argument (check events and storage)");
					}
					let substr = &str_val[1..str_val.len() - 1];
					result_str = substr.to_string();
				},
				_ => panic!("literal expected as annotation identifier (check events and storage)"),
			}
		},
		_ => panic!("missing annotation identifier (check events and storage)"),
	}

	if iter.next().is_some() {
		panic!("too many tokens in attribute (check events and storage)");
	}

	result_str
}

pub(super) fn find_attr_one_string_arg(
	m: &syn::TraitItemMethod,
	attr_name: &str,
) -> Option<String> {
	let attribute = m.attrs.iter().find(|attr| {
		if let Some(first_seg) = attr.path.segments.first() {
			first_seg.ident == attr_name
		} else {
			false
		}
	});
	attribute.map(|attr| attr_one_string_arg(attr))
}

/// Finds a method attribute with given name and 1 single optional argument.
/// In the result, the first option is for the attribute, the second for the argument.
pub(super) fn find_attr_with_one_opt_token_tree_arg(
	m: &syn::TraitItemMethod,
	attr_name: &str,
) -> Option<Option<proc_macro2::TokenTree>> {
	let cc_attr = m.attrs.iter().find(|attr| {
		if let Some(first_seg) = attr.path.segments.first() {
			first_seg.ident == attr_name
		} else {
			false
		}
	});

	match cc_attr {
		None => None,
		Some(attr) => {
			let mut iter = attr.clone().tokens.into_iter();
			let arg_token_tree: Option<proc_macro2::TokenTree> = match iter.next() {
				Some(proc_macro2::TokenTree::Group(group)) => {
					if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
						panic!("attribute paranthesis expected");
					}
					let mut iter2 = group.stream().into_iter();
					match iter2.next() {
						Some(token_tree) => Some(token_tree),
						_ => panic!("attribute argument expected"),
					}
				},
				Some(_) => panic!("unexpected attribute argument tokens"),
				None => None,
			};

			if iter.next().is_some() {
				panic!("too many tokens in attribute");
			}

			Some(arg_token_tree)
		},
	}
}
