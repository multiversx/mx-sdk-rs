// use super::util::*;
// use super::parse_attr::*;
use super::contract_gen_method::*;
use super::arg_def::*;

pub fn generate_payable_snippet(m: &Method) -> proc_macro2::TokenStream {
    if let MethodMetadata::Regular{ payable, .. } = &m.metadata {
        if *payable {
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
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            generate_payment_snippet_for_arg_type(&type_path_segment, &arg.pat)
        },             
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    generate_payment_snippet_for_arg_type(&type_path_segment, &arg.pat)
                },
                _ => {
                    panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                }
            }
        },
        other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
    }
}

fn generate_payment_snippet_for_arg_type(type_path_segment: &syn::PathSegment, pat: &syn::Pat) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "BigUint" =>
            quote!{
                let #pat = self.api.get_call_value_big_uint();
            },
        other_stype_str => {
            panic!("Arguments annotated with #[payment] must be of type BigUint. Found: {}", other_stype_str)
        }
    }
}
