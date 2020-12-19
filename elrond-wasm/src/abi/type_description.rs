use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct TypeDescription {
	pub docs: &'static [&'static str],
	pub name: String,
	pub contents: TypeContents,
}

#[derive(Clone, Debug)]
pub enum TypeContents {
	NotSpecified,
	Enum(Vec<EnumVariantDescription>),
	Struct(Vec<StructFieldDescription>),
}

impl TypeContents {
	pub fn is_specified(&self) -> bool {
		match *self {
			TypeContents::NotSpecified => false,
			_ => true,
		}
	}
}

#[derive(Clone, Debug)]
pub struct EnumVariantDescription {
	pub docs: &'static [&'static str],
	pub name: &'static str,
}

#[derive(Clone, Debug)]
pub struct StructFieldDescription {
	pub docs: &'static [&'static str],
	pub name: &'static str,
	pub field_type: String,
}
