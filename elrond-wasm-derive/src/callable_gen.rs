use super::util::*;
use super::parse_attr::*;
use super::contract_gen_method::*;
use super::contract_gen_arg::*;
//use super::contract_gen_payable::*;

#[derive(Clone, Debug)]
pub struct CallableMethod {
    pub name: syn::Ident,
    pub payable: bool,
    pub callback: Option<CallbackCallAttribute>,
    pub method_args: Vec<MethodArg>,
}

impl CallableMethod {
    pub fn parse(m: &syn::TraitItemMethod) -> CallableMethod {
        let payable = is_payable(m);
        let callback_opt = CallbackCallAttribute::parse(m);
        let method_args = extract_method_args(m, payable, callback_opt.is_some());
        CallableMethod {
            name: m.sig.ident.clone(),
            payable: payable,
            callback: callback_opt,
            method_args: method_args,
        }
    }

    // TODO: deduplicate
    pub fn generate_sig(&self) -> proc_macro2::TokenStream {
        let method_name = &self.name;
        let span = self.name.span();
        let arg_decl = arg_declarations(&self.method_args);
        let result = quote_spanned!{span=> fn #method_name ( &self , #(#arg_decl),* ) -> () };
        result
    }
}

#[derive(Clone, Debug)]
pub struct Callable {
    pub trait_name: proc_macro2::Ident,
    pub callable_impl_name: proc_macro2::Ident,
    pub contract_impl_name: syn::Path,
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
    pub fn extract_pub_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter().map(|m| {
            let sig = m.generate_sig();
            quote! { #sig ; }
        }).collect()
    }

    pub fn generate_method_impl(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter().map(|m| {
            let msig = m.generate_sig();

            let mut payment_count = 0;
            let arg_push_snippets: Vec<proc_macro2::TokenStream> = 
                m.method_args
                    .iter()
                    .map(|arg| {
                        let arg_accumulator = if arg.is_callback_arg {
                            quote! { callback_data }
                        } else {
                            quote! { call_data }
                        };

                        match &arg.metadata {
                            ArgMetadata::Single => {
                                generate_arg_push_snippet(arg, &arg_accumulator)
                            },
                            ArgMetadata::Payment => {
                                // #[payment]
                                payment_count += 1;
                                let pat = &arg.pat;
                                quote! { let amount = #pat; }
                            },
                            ArgMetadata::Multi(multi_attr) => {
                                let pat = &arg.pat;
                                let count_expr = &multi_attr.count_expr;
                                let vec_iter_push = generate_multi_arg_push_snippet(
                                    &arg,
                                    &arg_accumulator,
                                    &quote! { multi_arg_elem });
                                quote! {
                                    if #pat.len() != (#count_expr as usize) {
                                        self.api.signal_error("wrong number of arguments in async call");
                                    }
                                    for (_, multi_arg_elem) in #pat.iter().enumerate() {
                                        #vec_iter_push ;
                                    }
                                }
                            },
                        }
                    })
                    .collect();

            let amount_snippet = match payment_count {
                0 => quote! { let amount = BigUint::from(0); },
                1 => quote! {},
                _ => panic!("Only one payment argument allowed in call proxy")
            };

            let (callback_init, callback_store) = if let Some(callback_ident) = &m.callback {
                let cb_name_str = &callback_ident.arg.to_string();
                let cb_name_literal = array_literal(cb_name_str.as_bytes());
                let callback_init = quote! {
                    let mut callback_data = elrond_wasm::CallData::new( & #cb_name_literal );
                };
                let callback_store = quote! {
                    self.api.storage_store(&self.api.get_tx_hash(), callback_data.as_slice());
                };
                (callback_init, callback_store)
            } else {
                (quote! {}, quote! {})
            };

            let m_name_literal = array_literal(m.name.to_string().as_bytes());
            let sig = quote! {
                #msig {
                    #amount_snippet
                    let mut call_data = elrond_wasm::CallData::new( & #m_name_literal );
                    #callback_init
                    #(#arg_push_snippets)*
                    #callback_store
                    self.api.async_call(&self.address, &amount, call_data.as_slice());
                }
            };
            sig
        }).collect()
    }
}

fn generate_push_snippet_for_arg_type(
        type_path_segment: &syn::PathSegment,
        arg_accumulator: &proc_macro2::TokenStream,
        var_name: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {

    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "Address" | "StorageKey" | "H256" => quote!{
            #arg_accumulator.push_bytes(#var_name.as_bytes());
        },
        "Vec" => {
                match &type_path_segment.arguments {
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
                        if args.len() != 1 {
                            panic!("[callable] Vec type must have exactly 1 generic type argument");
                        }
                        if let syn::GenericArgument::Type(vec_type) = args.first().unwrap() {
                            match vec_type {
                                syn::Type::Path(type_path) => {
                                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                                    let type_str = type_path_segment.ident.to_string();
                                    match type_str.as_str() {
                                        "u8" => quote!{
                                            #arg_accumulator.push_bytes(#var_name.as_slice());
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
            panic!("[callable] BigInt arguments not yet supported"),
        "BigUint" =>
            quote!{
                #arg_accumulator.push_bytes(#var_name.to_bytes_be().as_slice());
            },
        "i64" =>
            quote!{
                #arg_accumulator.push_i64(#var_name);
            },
        "i32" =>
            quote!{
                #arg_accumulator.push_i32(#var_name);
            },
        "u32" | "isize" | "usize" | "i8" | "u8" =>
            quote!{
                #arg_accumulator.push_i64(#var_name as i64);
            },
        other_stype_str => {
            panic!("[callable] Unsupported argument type {:?} for arg init snippet", other_stype_str)
        }
    }
}

pub fn generate_arg_push_snippet(
        arg: &MethodArg,
        arg_accumulator: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {

    let pat = &arg.pat;
    let var_name = quote!{ #pat };
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            generate_push_snippet_for_arg_type(&type_path_segment, &arg_accumulator, &var_name)
        },
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    generate_push_snippet_for_arg_type(&type_path_segment, arg_accumulator, &var_name)
                },
                _ => {
                    panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                }
            }
        },
        other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
    }
}

pub fn generate_multi_arg_push_snippet(
        arg: &MethodArg,
        arg_accumulator: &proc_macro2::TokenStream,
        var_name: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            let type_str = type_path_segment.ident.to_string();
            match type_str.as_str() {
                "Vec" => {
                    let vec_generic_type_segm = generic_type_single_arg_segment(&"Vec", &type_path_segment);
                    generate_push_snippet_for_arg_type(&vec_generic_type_segm, arg_accumulator, var_name)
                },
                other_stype_str => {
                    panic!("Unsupported argument type {:?} for multi argument", other_stype_str)
                }
            }
        },
        other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
    }
}