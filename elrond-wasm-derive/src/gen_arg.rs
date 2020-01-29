
pub fn generate_arg_call_name(arg: &syn::FnArg, arg_index: isize) -> Option<proc_macro2::TokenStream> {
    match arg {
        syn::FnArg::SelfRef(ref selfref) => {
            if !selfref.mutability.is_none() || arg_index != -1 {
                panic!("ABI function must have `&self` as its first argument.");
            }
            None
        },
        syn::FnArg::Captured(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            match ty {                
                syn::Type::Path(_) => Some(quote!{ #pat }),
                syn::Type::Reference(_) => Some(quote!{ &#pat }),
                other_arg => panic!("Unsupported argument type {:?} in generate_arg_call_name", other_arg),
            }            
        },
        other_arg => panic!("Unsupported argument type {:?} in generate_arg_call_name, neither self, nor captured", other_arg)
    }
}

pub fn generate_arg_init_snippet(arg: &syn::FnArg, arg_index: isize) -> proc_macro2::TokenStream {
    match arg {
        syn::FnArg::SelfRef(ref selfref) => {
            if !selfref.mutability.is_none() || arg_index != -1 {
                panic!("ABI function must have `&self` as its first argument.");
            }
            quote!{}
        },
        syn::FnArg::Captured(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            let arg_index_i32 = arg_index as i32;
            match ty {                
                syn::Type::Path(type_path) => {
                    let type_str = type_path.path.segments.last().unwrap().value().ident.to_string();
                    match type_str.as_str() {
                        "Address" =>
                            quote!{
                                let #pat: Address = self.api.get_argument_address(#arg_index_i32);
                            },
                        "Vec" =>
                            quote!{
                                let #pat: Vec<u8> = self.api.get_argument_vec(#arg_index_i32);
                            },
                        "BigInt" =>
                            quote!{
                                let #pat = self.api.get_argument_big_int_signed(#arg_index_i32);
                            },
                        "BI" =>
                            quote!{
                                let #pat = self.api.get_argument_big_int_signed(#arg_index_i32);
                            },
                        "BU" =>
                            quote!{
                                let #pat = self.api.get_argument_big_int_unsigned(#arg_index_i32);
                            },
                        "i64" =>
                            quote!{
                                let #pat: i64 = self.api.get_argument_i64(#arg_index_i32);
                            },
                        other_stype_str => {
                            panic!("Unsupported argument type {:?} for arg init snippet", other_stype_str)
                        }
                    }
                },             
                syn::Type::Reference(type_reference) => {
                    if type_reference.mutability != None {
                        panic!("Mutable references not supported as contract method arguments");
                    }
                    match &*type_reference.elem {
                        syn::Type::Path(type_path) => {
                            let type_str = type_path.path.segments.last().unwrap().value().ident.to_string();
                            match type_str.as_str() {
                                "Address" =>
                                    quote!{
                                        let #pat: Address = self.api.get_argument_address(#arg_index_i32);
                                    },
                                "Vec" =>
                                    quote!{
                                        let #pat: Vec<u8> = self.api.get_argument_vec(#arg_index_i32);
                                    },
                                "BigInt" =>
                                    quote!{
                                        let #pat: BigInt = self.api.get_argument_big_int_signed(#arg_index_i32);
                                    },
                                "BI" =>
                                    quote!{
                                        let #pat = self.api.get_argument_big_int_signed(#arg_index_i32);
                                    },
                                "BU" =>
                                    quote!{
                                        let #pat = self.api.get_argument_big_int_unsigned(#arg_index_i32);
                                    },
                                other_stype_str => {
                                    panic!("Unsupported reference argument type: {:?}", other_stype_str)
                                }
                            }
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
