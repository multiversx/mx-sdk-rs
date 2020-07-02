

pub fn generate_body_with_result(return_type: &syn::ReturnType, mbody: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match return_type.clone() {
        syn::ReturnType::Default => quote!{
            #mbody;
        },
        syn::ReturnType::Type(_, _) => {
            quote!{
                let result = #mbody;
                EndpointResult::<'_, T, BigInt, BigUint>::finish(&result, &self.api);
            }
        },
    }
}