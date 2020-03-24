// use super::util::*;
// use super::parse_attr::*;
use super::contract_gen_method::*;
use super::contract_gen_arg::*;

pub fn generate_payable_snippet(m: &Method) -> proc_macro2::TokenStream {
    if let MethodMetadata::Public(pub_data) = &m.metadata {
        if pub_data.payable {
            quote!{}
        } else {
            quote!{
                if !self.api.check_not_payable() {
                    return;
                }
            }
        }
    } else {
        quote!{}
    }
}

pub fn generate_payment_snippet(arg: &MethodArg) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    return quote!{
        let #pat = self.api.get_call_value_big_uint();
    };
}
