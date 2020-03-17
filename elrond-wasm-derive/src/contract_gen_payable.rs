// use super::util::*;
use super::parse_attr::*;
use super::contract_gen::*;

pub fn generate_payable_snippet(m: &Method) -> proc_macro2::TokenStream {
    if let MethodMetadata::Public(payable_opt) = &m.metadata {
        if let Some(PayableAttribute{ payment_arg: payment_arg_opt }) = payable_opt {
            if let Some(payment_arg) = payment_arg_opt {
                match payment_arg {
                    syn::FnArg::Captured(arg_captured) => {
                        let pat = &arg_captured.pat;
                        quote!{
                            let #pat = self.api.get_call_value_big_uint();
                        }
                    },
                    _ => panic!("payable argument must be captured")
                }
            } else {
                // nothing extra to do here, the developer will handle the payment herself, or not
                quote!{}
            }
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