use super::attr_names::*;
use super::util::*;

static EGLD_DEFAULT: &str = "EGLD";

pub fn is_payment_amount(pat: &syn::PatType) -> bool {
	has_attribute(&pat.attrs, ATTR_PAYMENT_AMOUNT) || has_attribute(&pat.attrs, ATTR_PAYMENT)
}

pub fn is_payment_token(pat: &syn::PatType) -> bool {
	has_attribute(&pat.attrs, ATTR_PAYMENT_TOKEN)
}

pub fn is_payment_nonce(pat: &syn::PatType) -> bool {
	has_attribute(&pat.attrs, ATTR_PAYMENT_NONCE)
}

pub struct PayableAttribute {
	pub identifier: Option<String>,
}

impl PayableAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<PayableAttribute> {
		let payable_attr = m.attrs.iter().find(|attr| {
			if let Some(first_seg) = attr.path.segments.first() {
				first_seg.ident == ATTR_PAYABLE
			} else {
				false
			}
		});
		payable_attr.map(|attr| PayableAttribute {
			identifier: extract_token_identifier(attr),
		})
	}
}

/// Current implementation only works with 1 token name.
/// Might be extended in the future.
fn extract_token_identifier(attr: &syn::Attribute) -> Option<String> {
	let mut iter = attr.clone().tokens.into_iter();
	let result_str = match iter.next() {
		Some(proc_macro2::TokenTree::Group(group)) => {
			if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
				panic!("payable token name must be specified in parantheses");
			}
			let mut iter2 = group.stream().into_iter();
			match iter2.next() {
				Some(proc_macro2::TokenTree::Literal(lit)) => {
					let str_val = lit.to_string();
					if !str_val.starts_with('\"') || !str_val.ends_with('\"') {
						panic!("string literal expected as attribute argument");
					}
					let substr = &str_val[1..str_val.len() - 1];
					Some(substr.to_string())
				},
				_ => panic!("literal expected as event identifier"),
			}
		},
		None => None,
		_ => panic!("unexpected payable attribute format"),
	};

	if iter.next().is_some() {
		panic!("event too many tokens in event attribute");
	}

	result_str
}
