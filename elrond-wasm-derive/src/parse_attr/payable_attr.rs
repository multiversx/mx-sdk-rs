use super::attr_names::*;
use super::util::*;

pub fn is_payable(m: &syn::TraitItemMethod) -> bool {
	has_attribute(&m.attrs, ATTR_PAYABLE)
}

pub fn is_payment(pat: &syn::PatType) -> bool {
	has_attribute(&pat.attrs, ATTR_PAYMENT)
}
