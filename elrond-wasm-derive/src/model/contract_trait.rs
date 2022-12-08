use super::{Method, PublicRole, Supertrait, TraitProperties};

/// Models a contract or module trait.
pub struct ContractTrait {
    pub docs: Vec<String>,
    pub original_attributes: Vec<syn::Attribute>,
    pub trait_name: proc_macro2::Ident,
    pub supertraits: Vec<Supertrait>,

    /// It is possible to automatically implement a contract module for all contracts that use it indirectly.
    /// The drawback is that the developer make sure multiple inheritance does not happen.
    /// This feature is currently disabled.
    pub auto_inheritance_modules: Vec<Supertrait>,

    pub methods: Vec<Method>,
    pub trait_attributes: TraitProperties,
}

impl ContractTrait {
    pub fn callback_count(&self) -> usize {
        self.methods
            .iter()
            .filter(|m| {
                matches!(
                    m.public_role,
                    PublicRole::Callback(_) | PublicRole::CallbackPromise(_)
                )
            })
            .count()
    }
}
