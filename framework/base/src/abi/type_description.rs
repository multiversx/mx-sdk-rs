use alloc::{string::String, vec::Vec};

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
    ExplicitEnum(Vec<ExplicitEnumVariantDescription>),
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
    pub discriminant: usize,
    pub fields: Vec<StructFieldDescription>,
}

#[derive(Clone, Debug)]
pub struct StructFieldDescription {
    pub docs: &'static [&'static str],
    pub name: &'static str,
    pub field_type: String,
}

/// An explicit enum is an enum that gets serialized by name instead of discriminant.
///
/// This makes it easier for humans to read readable in the transaction output.
///
/// It cannot have data fields, only simple enums allowed.
#[derive(Clone, Debug)]
pub struct ExplicitEnumVariantDescription {
    pub docs: &'static [&'static str],
    pub name: &'static str,
}
