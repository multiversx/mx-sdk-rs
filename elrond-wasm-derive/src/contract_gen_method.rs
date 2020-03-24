use super::contract_gen_arg::*;
use super::contract_gen_finish::*;
use super::contract_gen_payable::*;
use super::parse_attr::*;
use super::util::*;
use super::reserved;

#[derive(Clone, Debug)]
pub enum MethodMetadata {
    Public(PublicMethodMetadata),
    Private(),
    Event(Vec<u8>),
    Callback(),
}

#[derive(Clone, Debug)]
pub struct PublicMethodMetadata {
    pub payable: bool,
}

#[derive(Clone, Debug)]
pub struct Method {
    pub metadata: MethodMetadata,
    pub name: syn::Ident,
    pub method_args: Vec<MethodArg>,
    pub return_type: syn::ReturnType,
    pub body: Option<syn::Block>,
}

fn extract_metadata(m: &syn::TraitItemMethod) -> MethodMetadata {
    let payable = is_payable(m);
    let private = is_private(m);
    let callback = is_callback_decl(m);
    let event_opt = EventAttribute::parse(m);

    if let Some(event_attr) = event_opt {
        if payable {
            panic!("Events cannot be payable.");
        }
        if private {
            panic!("Events cannot be marked private, they are private by definition.");
        }
        if callback {
            panic!("Events cannot be callbacks.");
        }
        if let Some(_) = m.default {
            panic!("Events cannot have provided implementations in the trait.");
        }
        MethodMetadata::Event(event_attr.identifier)
    } else if callback {
        if payable {
            panic!("Callback methods cannot be marked payable.");
        }
        if private {
            panic!("Callbacks cannot be marked private, they are private by definition.");
        }
        if m.default == None {
            panic!("Callback methods need an implementation.");
        }
        MethodMetadata::Callback()
    } else if private {
        if payable {
            panic!("Private methods cannot be marked payable.");
        }
        if m.default == None {
            panic!("Private methods need an implementation.");
        }
        MethodMetadata::Private()
    } else {
        if m.default == None {
            panic!("Public methods need an implementation.");
        }
        let fn_name_str = &m.sig.ident.to_string();
        if reserved::is_reserved(fn_name_str) {
            panic!("Cannot declare public method with name '{}', because that name is reserved by the Arwen API.", fn_name_str);
        }

        MethodMetadata::Public(PublicMethodMetadata{
            payable: payable,
        })
    }
}

impl Method {
    pub fn parse(m: &syn::TraitItemMethod) -> Method {
        let metadata = extract_metadata(m);
        let method_args = extract_method_args(m, is_payable(m));
        Method {
            metadata: metadata,
            name: m.sig.ident.clone(),
            method_args: method_args,
            return_type: m.sig.output.clone(),
            body: m.default.clone(),
        }
    }
}

pub fn arg_declarations(method_args: &Vec<MethodArg>) -> Vec<proc_macro2::TokenStream>  {
    method_args
        .iter()
        .map(|arg| {
            let pat = &arg.pat;
            let ty = &arg.ty;
            quote!{#pat : #ty }
        })
        .collect()
}

impl Method {
    pub fn generate_sig(&self) -> proc_macro2::TokenStream {
        let method_name = &self.name;
        let arg_decl = arg_declarations(&self.method_args);
        let ret_tok = match &self.return_type {
            syn::ReturnType::Default => quote!{},
            syn::ReturnType::Type(_, ty) => quote!{ -> #ty },
        };
        let result = quote!{ fn #method_name ( &self , #(#arg_decl),* ) #ret_tok };
        result
    }

    pub fn generate_call_to_method(&self) -> proc_macro2::TokenStream {
        let fn_ident = &self.name;
        let arg_values: Vec<proc_macro2::TokenStream> = self.method_args
            .iter()
            .map(|arg| generate_arg_call_name(arg))
            .collect();
        quote! {
            self.#fn_ident (#(#arg_values),*)
        }
    }

    pub fn generate_call_method(&self) -> proc_macro2::TokenStream {
        let payable_snippet = generate_payable_snippet(self);

        let mut nr_args = 0i32;
        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
            self.method_args
                .iter()
                .map(|arg| {
                    if let ArgMetadata::Payment = arg.metadata {
                        generate_payment_snippet(arg) // #[payment]
                    } else {
                        nr_args += 1;
                        generate_arg_init_snippet(arg, 0)
                    }
                })
                .collect();

        let call_method_ident = generate_call_method_name(&self.name);
        let call = self.generate_call_to_method();
        let body_with_result = generate_body_with_result(&self.return_type, &call);

        quote! {
            #[inline]
            fn #call_method_ident (&self) {
                #payable_snippet
                if !self.api.check_num_arguments(#nr_args) {
                    return;
                }
                #(#arg_init_snippets)*
                #body_with_result
            }
        }
    }
}