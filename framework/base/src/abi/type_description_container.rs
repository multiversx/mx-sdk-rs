use super::*;
use multiversx_sc_codec::Vec;

pub trait TypeDescriptionContainer {
    fn new() -> Self;

    fn contains_type(&self, type_name: &str) -> bool;

    // A placeholder gets inserted while computing field descriptions for a type,
    // to avoid an infinite loop for recursive types (if the same type appears again lower in the tree).
    fn reserve_type_name(&mut self, type_name: TypeName) {
        self.insert(type_name, TypeDescription::PLACEHOLDER);
    }

    fn insert(&mut self, type_name: TypeName, type_description: TypeDescription);

    fn insert_all(&mut self, other: &Self);
}

#[derive(Clone, Default, Debug)]
pub struct TypeDescriptionContainerImpl(pub Vec<(TypeName, TypeDescription)>);

impl TypeDescriptionContainer for TypeDescriptionContainerImpl {
    fn new() -> Self {
        TypeDescriptionContainerImpl(Vec::new())
    }

    fn contains_type(&self, type_name: &str) -> bool {
        self.0
            .iter()
            .any(|(existing_type_name, _)| existing_type_name == type_name)
    }

    fn insert(&mut self, type_name: TypeName, type_description: TypeDescription) {
        if let Some((_existing_type_name, exisiting_type_description)) =
            self.0.iter_mut().find(|(name, _)| name == &type_name)
        {
            *exisiting_type_description = type_description;
        } else {
            self.0.push((type_name, type_description));
        }
    }

    fn insert_all(&mut self, other: &Self) {
        for (key, value) in other.0.iter() {
            self.insert(key.clone(), value.clone());
        }
    }
}
