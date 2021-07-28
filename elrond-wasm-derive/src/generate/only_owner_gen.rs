use crate::model::{Method, PublicRole};

pub fn generate_only_owner_snippet(m: &Method) -> proc_macro2::TokenStream {
    if let PublicRole::Endpoint(endpoint_metadata) = &m.public_role {
        if endpoint_metadata.only_owner {
            return quote! {
                self.blockchain().check_caller_is_owner();
            };
        }
    }
    return quote! {};
}
