use super::util::*;
use super::parse_attr::*;
use super::contract_gen::*;

#[derive(Clone, Debug)]
pub struct CallableMethod {
    pub payable: Option<PayableAttribute>,
    pub callback: Option<CallbackCallAttribute>,
    pub public_args: Vec<PublicArg>,
    pub syn_m: syn::TraitItemMethod,
}

impl CallableMethod {
    pub fn parse(m: &syn::TraitItemMethod) -> CallableMethod {
        let payable_opt = PayableAttribute::parse(m);
        let callback_opt = CallbackCallAttribute::parse(m);
        let public_args = extract_public_args(m, &payable_opt);
        CallableMethod {
            payable: payable_opt,
            callback: callback_opt,
            public_args: public_args,
            syn_m: m.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Callable {
    pub trait_name: proc_macro2::Ident,
    pub callable_impl_name: proc_macro2::Ident,
    pub contract_impl_name: proc_macro2::Ident,
    methods: Vec<CallableMethod>,
}

impl Callable {
    pub fn new(args: syn::AttributeArgs, contract_trait: &syn::ItemTrait) -> Self {
        let callable_impl_name = generate_callable_interface_impl_struct_name(&contract_trait.ident);
        let contract_impl_name = extract_struct_name(args);

        let methods: Vec<CallableMethod> = contract_trait
            .items
            .iter()
            .map(|itm| match itm {
                syn::TraitItem::Method(m) => CallableMethod::parse(m),
                _ => panic!("Only methods allowed in callable traits")
            })
            .collect();

        //let trait_methods = extract_methods(&contract_trait);
        Callable {
            trait_name: contract_trait.ident.clone(),
            callable_impl_name: callable_impl_name,
            contract_impl_name: contract_impl_name,
            methods: methods,
        }
    }
}

impl Callable {
    pub fn extract_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter().map(|m| {
            let msig = &m.syn_m.sig;
            let sig = quote! {
                #msig;
            };
            sig
        }).collect()
    }

    pub fn generate_method_impl(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter().map(|m| {
            let msig = &m.syn_m.sig;
            let arg_push_snippets: Vec<proc_macro2::TokenStream> = 
                m.public_args
                    .iter()
                    .map(|arg| generate_arg_push_snippet(arg))
                    .collect();

            let amount_snippet = if let Some(payment_arg) = &m.payable {
                if let Some(payment_fn_attr) = &payment_arg.payment_arg {
                    match &payment_fn_attr {
                        syn::FnArg::Captured(arg_captured) => {
                            let pat = &arg_captured.pat;
                            quote! {
                                let amount = #pat;
                            }
                        },
                        _ => panic!("Payment arg not captured")
                    }
                } else {
                    panic!("Explicit payment arg required in callable function")
                }
            } else {
                quote! {
                    let amount = BigUint::from(0);
                }
            };

            let callback_snippet = if let Some(callback_ident) = &m.callback {
                let cb_name_str = &callback_ident.arg.to_string();
                let cb_name_literal = array_literal(cb_name_str.as_bytes());
                quote! {
                    elrond_wasm::str_util::push_bytes(&mut data, & #cb_name_literal);
                }
            } else {
                quote! {
                    elrond_wasm::str_util::push_empty(&mut data);
                }
            };

            let msig_str = msig.ident.to_string();
            let sig = quote! {
                #msig {
                    #amount_snippet
                    let mut data = String::from(#msig_str);
                    #(#arg_push_snippets)*
                    #callback_snippet
                    self.api.async_call(&self.address, &amount, data.as_str());
                }
            };
            sig
        }).collect()
    }
}

fn generate_push_snippet_for_arg_type(type_path_segment: &syn::PathSegment, pat: &syn::Pat, _arg_index_i32: i32) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "Address" => quote!{
            elrond_wasm::str_util::push_bytes(&mut data, #pat.as_bytes());
        },
        "Vec" => {
                match &type_path_segment.arguments {
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
                        if args.len() != 1 {
                            panic!("[callable] Vec type must have exactly 1 generic type argument");
                        }
                        if let syn::GenericArgument::Type(vec_type) = args.first().unwrap().into_value() {
                            match vec_type {                
                                syn::Type::Path(type_path) => {
                                    let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                                    let type_str = type_path_segment.ident.to_string();
                                    match type_str.as_str() {
                                        "u8" => quote!{
                                            elrond_wasm::str_util::push_bytes(&mut data, #pat.as_slice());
                                        },
                                        other_type => panic!("[callable] Unsupported type: Vec<{:?}>", other_type)
                                    }
                                },
                                other_type => panic!("[callable] Unsupported Vec generic type: {:?}, not a path", other_type)
                            }
                        } else {
                            panic!("[callable] Vec type arguments must be types")
                        }
                    },
                    _ => panic!("[callable] Vec angle brackets expected")
                }
            },
        "BigInt" =>
            // quote!{
            //     elrond_wasm::str_util::push_bytes(&mut data, #pat.to_bytes_be().as_slice());
            // },
            panic!("[callable] BigInt arguments not yet supported"),
        "BigUint" =>
            quote!{
                elrond_wasm::str_util::push_bytes(&mut data, #pat.to_bytes_be().as_slice());
            },
        "i32" =>
            quote!{
                elrond_wasm::str_util::push_i32(&mut data, #pat);
            },
        "i64" =>
            quote!{
                elrond_wasm::str_util::push_i64(&mut data, #pat);
            },
        other_stype_str => {
            panic!("[callable] Unsupported argument type {:?} for arg init snippet", other_stype_str)
        }
    }
}

pub fn generate_arg_push_snippet(arg: &PublicArg) -> proc_macro2::TokenStream {
    match &arg.syn_arg {
        syn::FnArg::Captured(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            let arg_index = arg.index;
            match ty {                
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                    generate_push_snippet_for_arg_type(&type_path_segment, pat, arg_index)
                },
                syn::Type::Reference(type_reference) => {
                    if type_reference.mutability != None {
                        panic!("Mutable references not supported as contract method arguments");
                    }
                    match &*type_reference.elem {
                        syn::Type::Path(type_path) => {
                            let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                            generate_push_snippet_for_arg_type(&type_path_segment, pat, arg_index)
                        },
                        _ => {
                            panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                        }
                    }
                },
                other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
			}
        }
        other_arg => panic!("Unsupported argument type: {:?}, not captured", other_arg)
    }
}