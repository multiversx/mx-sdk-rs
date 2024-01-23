use alloc::string::{String, ToString};

use super::{TypeAbi, TypeDescriptionContainerImpl, TypeName};

#[derive(Clone, Debug)]
pub struct EsdtAttributeAbi {
    pub ticker: String,
    pub ty: TypeName,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl EsdtAttributeAbi {
    /// Used in code generation.
    pub fn new<T: TypeAbi>(arg_name: &str) -> EsdtAttributeAbi {
        let mut type_descriptions = TypeDescriptionContainerImpl::default();
        T::provide_type_descriptions(&mut type_descriptions);
        EsdtAttributeAbi {
            ticker: arg_name.to_string(),
            ty: T::type_name(),
            type_descriptions,
        }
    }
}
