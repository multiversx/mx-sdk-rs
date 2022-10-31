use proc_macro2::TokenStream;

#[derive(Clone, Debug)]
pub struct EndpointLocationMetadata {
    pub locations: &'static str,
}

impl EndpointLocationMetadata {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        let mut locations: Vec<TokenStream> = vec![];

        for item in self.locations.split("|") {           
            locations.push(self.get_location_as_token(item));
        }

        quote! {
            #(#locations)*
        }
    }

    pub fn get_location_as_token(&self, location: &str) -> proc_macro2::TokenStream {
        quote! {   
                elrond_wasm::abi::EndpointLocationAbi{location: #location}
        }
    }
}
