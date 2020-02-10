use super::util::*;

pub struct Callable {
    pub trait_name: proc_macro2::Ident,
    pub callable_impl_name: proc_macro2::Ident,
    pub contract_impl_name: proc_macro2::Ident,
    trait_methods: Vec<syn::TraitItemMethod>,
}

impl Callable {
    pub fn new(args: syn::AttributeArgs, contract_trait: &syn::ItemTrait) -> Self {
        let callable_impl_name = generate_callable_interface_impl_struct_name(&contract_trait.ident);
        let contract_impl_name = extract_struct_name(args);
        let trait_methods = extract_methods(&contract_trait);
        Callable {
            trait_name: contract_trait.ident.clone(),
            callable_impl_name: callable_impl_name,
            contract_impl_name: contract_impl_name,
            trait_methods: trait_methods,
        }
    }
}

impl Callable {
    pub fn extract_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
        self.trait_methods.iter().map(|m| {
            let msig = &m.sig;
            let sig = quote! {
                #msig;
            };
            sig
        }).collect()
    }

    pub fn generate_method_impl(&self) -> Vec<proc_macro2::TokenStream> {
        self.trait_methods.iter().map(|m| {
            let msig = &m.sig;
            let sig = quote! {
                #msig { }
            };
            sig
        }).collect()
    }
}
