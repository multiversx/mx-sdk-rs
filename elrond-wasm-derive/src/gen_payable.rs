use super::utils::*;

pub fn generate_payable_snippet(m: &syn::TraitItemMethod) -> proc_macro2::TokenStream {
    let payable = has_attribute(&m.attrs, "payable");
    if payable {
        quote!{}
    } else {
        quote!{
            if !self.api.check_not_payable() {
                return;
            }
        }
    }
}