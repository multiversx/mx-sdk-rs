#[derive(Clone, Debug)]
pub struct EndpointLocationMetadata {
    pub locations: &'static str,
}

impl EndpointLocationMetadata {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        quote! {
            for item in self.locations.split("|") {           
                elrond_wasm::abi::EndpointLocationAbi{location: item} 
            }
        }
    }
}
