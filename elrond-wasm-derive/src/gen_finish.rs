
pub fn generate_result_finish_snippet(result_ident: &syn::Ident, ty: &syn::Type) -> proc_macro2::TokenStream {
    match ty {                
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
            let type_str = type_path_segment.ident.to_string();
            match type_str.as_str() {
                "Result" => {    
                    match &type_path_segment.arguments {
                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
                            if args.len() != 2 {
                                panic!("Result type must have exactly 2 generic type arguments");
                            }

                            if let (syn::GenericArgument::Type(result_type), syn::GenericArgument::Type(err_type)) =
                                   (args.first().unwrap().into_value(), args.last().unwrap().into_value()) {
                                let ok_res_ident = syn::Ident::new("ok_res", proc_macro2::Span::call_site());
                                let ok_snippet = generate_result_finish_snippet(&ok_res_ident, result_type);
                                let err_res_ident = syn::Ident::new("err_res", proc_macro2::Span::call_site());
                                let err_snippet = generate_result_err_snippet(&err_res_ident, err_type);

                                quote!{
                                    match #result_ident {
                                        Ok(#ok_res_ident) => {
                                            #ok_snippet
                                        },
                                        Err(#err_res_ident) => {
                                            #err_snippet
                                        }
                                    }
                                }                                
                            } else {
                                panic!("Result type arguments must be types")
                            }
                        },
                        _ => panic!("Result angle brackets expected")
                    }
                    
                },
                "Address" =>
                    quote!{
                        self.api.finish(&#result_ident[0], 32);
                    },
                "Vec" => // TODO: better solution here, must capture type argument <u8>
                    quote!{
                        self.api.finish_vec(#result_ident);
                    },
                "BigInt" =>
                    quote!{
                        self.api.finish_big_int_signed(#result_ident);
                    },
                "BI" =>
                    quote!{
                        self.api.finish_big_int_signed(#result_ident);
                    },
                "BU" =>
                    quote!{
                        self.api.finish_big_int_unsigned(#result_ident);
                    },
                "i64" =>
                    quote!{
                        self.api.finish_i64(#result_ident);
                    },
                "bool" =>
                    quote!{
                        self.api.finish_i64( if #result_ident { 1i64 } else { 0i64 });
                    },
                other_stype_str => {
                    panic!("Unsupported return type: {:?}", other_stype_str)
                }
            }
        },
        syn::Type::Tuple(syn::TypeTuple{elems, ..}) => {
            let mut i = 0;
            let tuple_snippets = elems.iter().map(|t| {
                let tuple_i=syn::Index::from(i);
                let temp_name = format!("tuple_{}", i);
                let temp_ident = syn::Ident::new(temp_name.as_str(), proc_macro2::Span::call_site());
                i = i + 1;
                let snippet = generate_result_finish_snippet(&temp_ident, t);
                quote!{ let #temp_ident = #result_ident.#tuple_i; #snippet }
            });
            quote!{ #(#tuple_snippets)* }
        },
        other_type => panic!("Unsupported return type: {:#?}, not a path", other_type)
    }
}

pub fn generate_result_err_snippet(err_ident: &syn::Ident, _ty: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        let (message_ptr, message_len) = ErrorMessage::message_ptr_and_len(#err_ident);
        self.api.signal_error_raw(message_ptr, message_len);
    }
}

pub fn generate_body_with_result(return_type: &syn::ReturnType, mbody: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match return_type.clone() {
        syn::ReturnType::Default => quote!{#mbody;},
        syn::ReturnType::Type(_, ty) => {
            let result_ident = syn::Ident::new("result", proc_macro2::Span::call_site());
            let finish = generate_result_finish_snippet(&result_ident, &ty);
            quote!{
                let #result_ident = { #mbody };
                #finish
            }
        },
    }
}