use super::attributes::PayableAttribute;
use crate::model::MethodPayableMetadata;

pub fn process_payable(m: &syn::TraitItemMethod) -> MethodPayableMetadata {
	let payable_attr_opt = PayableAttribute::parse(m);
	if let Some(payable_attr) = payable_attr_opt {
		if let Some(identifier) = payable_attr.identifier {
			match identifier.as_str() {
				"EGLD" => MethodPayableMetadata::Egld,
				"*" => MethodPayableMetadata::AnyToken,
				"" => panic!("empty token name not allowed in #[payable] attribute"),
				_ => MethodPayableMetadata::SingleEsdtToken(identifier),
			}
		} else {
			panic!(
				"Endpoint `payable` attribute requires one argument. Replace with `#[payable(\"*\")]` or `#[payable(\"EGLD\")]`. Method name: {}",
				m.sig.ident.to_string())
		}
	} else {
		MethodPayableMetadata::NotPayable
	}
}
