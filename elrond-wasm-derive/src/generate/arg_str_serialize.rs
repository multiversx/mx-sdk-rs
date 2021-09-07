use crate::model::MethodArgument;

pub fn arg_serialize_push(
    arg: &MethodArgument,
    arg_accumulator: &proc_macro2::TokenStream,
    error_api_getter: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    arg_serialize_call(
        &quote! {elrond_wasm::io::serialize_contract_call_arg},
        arg,
        arg_accumulator,
        error_api_getter,
    )
}

fn arg_serialize_call(
    serialize_method: &proc_macro2::TokenStream,
    arg: &MethodArgument,
    arg_accumulator: &proc_macro2::TokenStream,
    error_api_getter: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    let var_name = quote! { #pat };
    let arg_ty = &arg.ty;
    match arg_ty {
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            quote! {
                #serialize_method(#var_name, #arg_accumulator, #error_api_getter);
            }
        },
        _ => {
            quote! {
                #serialize_method(#var_name, #arg_accumulator, #error_api_getter);
            }
        },
    }
}
