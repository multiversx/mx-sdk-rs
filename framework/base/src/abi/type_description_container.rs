use super::*;
use alloc::borrow::ToOwned;
use multiversx_sc_codec::Vec;

pub trait TypeDescriptionContainer {
    fn new() -> Self;

    fn contains_type(&self, type_name: &str) -> bool;

    // A placeholder gets inserted while computing field descriptions for a type,
    // to avoid an infinite loop for recursive types (if the same type appears again lower in the tree).
    fn reserve_type_name(&mut self, type_names: TypeNames) {
        self.insert(type_names, TypeDescription::PLACEHOLDER);
    }

    fn insert(&mut self, type_names: TypeNames, type_description: TypeDescription);

    fn insert_all(&mut self, other: &Self);
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TypeDescriptionContainerImpl(pub Vec<(TypeNames, TypeDescription)>);

impl TypeDescriptionContainerImpl {
    pub fn find(&self, abi_type_name: &str) -> Option<&TypeDescription> {
        self.0
            .iter()
            .find(|(existing_type_name, _)| existing_type_name.abi == abi_type_name)
            .map(|(_, description)| description)
    }

    pub fn find_or_default(&self, abi_type_name: &str) -> TypeDescription {
        if let Some(type_description) = self.find(abi_type_name) {
            type_description.clone()
        } else {
            TypeDescription {
                docs: Vec::new(),
                names: TypeNames::from_abi(abi_type_name.to_owned()),
                contents: TypeContents::NotSpecified,
                macro_attributes: Vec::new(),
            }
        }
    }
}

impl TypeDescriptionContainer for TypeDescriptionContainerImpl {
    fn new() -> Self {
        TypeDescriptionContainerImpl(Vec::new())
    }

    fn contains_type(&self, type_name: &str) -> bool {
        self.0
            .iter()
            .any(|(existing_type_name, _)| existing_type_name.abi == type_name)
    }

    fn insert(&mut self, type_names: TypeNames, type_description: TypeDescription) {
        if let Some((_existing_type_name, existing_type_description)) = self
            .0
            .iter_mut()
            .find(|(name, _)| name.abi == type_names.abi)
        {
            *existing_type_description = type_description;
        } else {
            self.0.push((type_names, type_description));
        }
    }

    fn insert_all(&mut self, other: &Self) {
        for (key, value) in other.0.iter() {
            self.insert(key.clone(), value.clone());
        }
    }
}
