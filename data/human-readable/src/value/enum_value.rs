use crate::AnyValue;

pub struct EnumVariant {
    pub discriminant: usize,
    pub value: AnyValue,
}
