use super::{TypeAbi, TypeDescriptionContainerImpl, TypeName};

#[derive(Clone, Debug)]
pub struct EsdtAttributeAbi {
    pub ticker: &'static str,
    pub ty: TypeName,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl EsdtAttributeAbi {
    pub fn new<T: TypeAbi>(arg_name: &'static str) -> EsdtAttributeAbi {
        let mut type_descriptions = TypeDescriptionContainerImpl::default();
        T::provide_type_descriptions(&mut type_descriptions);
        EsdtAttributeAbi {
            ticker: arg_name,
            ty: T::type_name(),
            type_descriptions,
        }
    }
}
