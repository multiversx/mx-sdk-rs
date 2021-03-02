/// Contains metdata from the `#[payable(...)]` attribute.
#[derive(Clone, Debug)]
pub enum MethodPayableMetadata {
	NoMetadata,
	NotPayable,
	Egld,
	SingleEsdtToken(String),
	AnyToken,
}


impl MethodPayableMetadata {
	pub fn is_payable(&self) -> bool {
		!matches!(self, MethodPayableMetadata::NotPayable)
	}

	pub fn no_esdt(&self) -> bool {
		!matches!(self, MethodPayableMetadata::NotPayable | MethodPayableMetadata::Egld)
	}

	pub fn abi_strings(&self) -> Vec<String> {
		match self {
			MethodPayableMetadata::NoMetadata | MethodPayableMetadata::NotPayable => Vec::new(),
			MethodPayableMetadata::Egld => vec!["EGLD".to_string()],
			MethodPayableMetadata::SingleEsdtToken(s) => vec![s.clone()],
			MethodPayableMetadata::AnyToken => vec!["*".to_string()],
		}
	}
}
