use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct TypeDescription {
	pub docs: &'static [&'static str],
	pub name: String,
	pub contents: TypeContents,
}

impl TypeDescription {
	/// Used as temporary value.
	/// To avoid an infinite loop for recursive types,
	/// we must reserve the type key (type name) before computing its fields.
	/// We use this as value while the fields are being computed.
	pub const PLACEHOLDER: TypeDescription = TypeDescription {
		docs: &[],
		name: String::new(),
		contents: TypeContents::NotSpecified,
	};
}

#[derive(Clone, Debug)]
pub enum TypeContents {
	NotSpecified,
	Enum(Vec<EnumVariantDescription>),
	Struct(Vec<StructFieldDescription>),
}

impl TypeContents {
	pub fn is_specified(&self) -> bool {
		!matches!(*self, TypeContents::NotSpecified)
	}
}

#[derive(Clone, Debug)]
pub struct EnumVariantDescription {
	pub docs: &'static [&'static str],
	pub name: &'static str,
	pub fields: Vec<StructFieldDescription>,
}

#[derive(Clone, Debug)]
pub struct StructFieldDescription {
	pub docs: &'static [&'static str],
	pub name: &'static str,
	pub field_type: String,
}
