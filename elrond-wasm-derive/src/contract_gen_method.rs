use super::arg_def::*;
use super::arg_extract::*;
use super::arg_regular::*;
use super::contract_gen_finish::*;
use super::contract_gen_payable::*;
use super::parse_attr::*;
use super::util::*;
use super::reserved;

/// Method visibility from the point of view of the smart contract
#[derive(Clone, Debug)]
pub enum Visibility {
    /// Means it gets a smart contract function generated for it
    Public,

    /// Can be used only inside the smart contract, even if it is public in the module.
    Private
}

#[derive(Clone, Debug)]
pub enum MethodMetadata {
    Regular{ visibility: Visibility, payable: bool },
    Event{ identifier: Vec<u8> },
    Callback,
    CallbackRaw,
    StorageGetter{ visibility: Visibility, identifier: String },
    StorageSetter{ visibility: Visibility, identifier: String },
    Module{ impl_path: proc_macro2::TokenTree },
}

impl MethodMetadata {
    pub fn is_public(&self) -> bool {
        match self {
            MethodMetadata::Regular{ visibility: Visibility::Public, ..} |
            MethodMetadata::StorageGetter{ visibility: Visibility::Public, ..} |
            MethodMetadata::StorageSetter{ visibility: Visibility::Public, ..} => true,
            _ => false
        }
    }

    pub fn has_implementation(&self) -> bool {
        match self {
            MethodMetadata::Regular{..} | 
            MethodMetadata::Callback | 
            MethodMetadata::CallbackRaw => true,
            _ => false
        }
    }
}

#[derive(Clone, Debug)]
pub struct Method {
    pub metadata: MethodMetadata,
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub method_args: Vec<MethodArg>,
    pub return_type: syn::ReturnType,
    pub body: Option<syn::Block>,
}

fn extract_metadata(m: &syn::TraitItemMethod) -> MethodMetadata {
    let payable = is_payable(m);
    let private = is_private(m);
    let visibility = if private { Visibility::Private } else { Visibility::Public };
    let callback = is_callback_decl(m);
    let callback_raw = is_callback_raw_decl(m);
    let event_opt = EventAttribute::parse(m);
    let storage_get_opt = StorageGetAttribute::parse(m);
    let storage_set_opt = StorageSetAttribute::parse(m);
    let module_opt = ModuleAttribute::parse(m);

    if let Some(event_attr) = event_opt {
        if payable {
            panic!("Events cannot be payable.");
        }
        if private {
            panic!("Events cannot be marked private, they are private by definition.");
        }
        if callback || callback_raw {
            panic!("Events cannot be callbacks.");
        }
        if storage_get_opt.is_some() {
            panic!("Events cannot be storage getters.");
        }
        if storage_set_opt.is_some() {
            panic!("Events cannot be storage setters.");
        }
        if module_opt.is_some() {
            panic!("Events cannot be modules.");
        }
        if m.default.is_some() {
            panic!("Events cannot have an implementations provided in the trait.");
        }
        MethodMetadata::Event{ identifier: event_attr.identifier }
    } else if callback || callback_raw {
        if payable {
            panic!("Callback methods cannot be marked payable.");
        }
        if private {
            panic!("Callbacks cannot be marked private, they are private by definition.");
        }
        if storage_get_opt.is_some() {
            panic!("Callbacks cannot be storage getters.");
        }
        if storage_set_opt.is_some() {
            panic!("Callbacks cannot be storage setters.");
        }
        if module_opt.is_some() {
            panic!("Callbacks cannot be modules.");
        }
        if m.default.is_none() {
            panic!("Callback methods need an implementation.");
        }
        if callback && callback_raw {
            panic!("It is either the default callback, or regular callback, not both.");
        }
        if callback_raw {
            MethodMetadata::CallbackRaw
        } else {
            MethodMetadata::Callback
        }
    } else if let Some(storage_get) = storage_get_opt {
        if payable {
            panic!("Storage getters cannot be marked payable.");
        }
        if m.default.is_some() {
            panic!("Storage getters cannot have an implementations provided in the trait.");
        }
        if module_opt.is_some() {
            panic!("Storage getters cannot be modules.");
        }
        MethodMetadata::StorageGetter{
            visibility: visibility,
            identifier: storage_get.identifier,
        }
    } else if let Some(storage_set) = storage_set_opt {
        if payable {
            panic!("Storage setters cannot be marked payable.");
        }
        if m.default.is_some() {
            panic!("Storage setters cannot have an implementations provided in the trait.");
        }
        if module_opt.is_some() {
            panic!("Storage setters cannot be modules.");
        }
        MethodMetadata::StorageSetter{
            visibility: visibility,
            identifier: storage_set.identifier,
        }
    } else if let Some(module_attr) = module_opt {
        if m.default.is_some() {
            panic!("Storage getters cannot have an implementations provided in the trait.");
        }
        MethodMetadata::Module{
            impl_path: module_attr.arg,
        }
    } else {
        if m.default.is_none() {
            panic!("Regular methods need an implementation.");
        }
        if !private {
            let fn_name_str = &m.sig.ident.to_string();
            if reserved::is_reserved(fn_name_str) {
                panic!("Cannot declare public method with name '{}', because that name is reserved by the Arwen API.", fn_name_str);
            }
        }
        MethodMetadata::Regular{
            visibility: visibility,
            payable: payable,
        }
    }
}

impl Method {
    pub fn parse(m: &syn::TraitItemMethod) -> Method {
        let metadata = extract_metadata(m);
        let allow_callback_args = if let MethodMetadata::Callback = metadata { true } else { false };
        let method_args = extract_method_args(m, is_payable(m), allow_callback_args);
        Method {
            metadata: metadata,
            name: m.sig.ident.clone(),
            generics: m.sig.generics.clone(),
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
        let generics = &self.generics;
        let generics_where = &self.generics.where_clause;
        let arg_decl = arg_declarations(&self.method_args);
        let ret_tok = match &self.return_type {
            syn::ReturnType::Default => quote!{},
            syn::ReturnType::Type(_, ty) => quote!{ -> #ty },
        };
        let result = quote!{ fn #method_name #generics ( &self , #(#arg_decl),* ) #ret_tok #generics_where };
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

    pub fn has_variable_nr_args(&self) -> bool {
        self.method_args.iter()
            .any(|arg| {
                match &arg.metadata {
                    ArgMetadata::Multi(_) => true,
                    ArgMetadata::VarArgs => true,
                    _ => false,
                }
            })
    }

    pub fn generate_call_method(&self) -> proc_macro2::TokenStream {
        if self.has_variable_nr_args() {
            self.generate_call_method_variable_nr_args()
        } else {
            self.generate_call_method_fixed_args()
        }
    }

    pub fn generate_call_method_fixed_args(&self) -> proc_macro2::TokenStream {
        let payable_snippet = generate_payable_snippet(self);

        let mut arg_index = -1i32;
        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
            self.method_args
                .iter()
                .map(|arg| {
                    if arg.is_callback_arg {
                        panic!("callback args not allowed in endpoints");
                    }

                    match &arg.metadata {
                        ArgMetadata::Single => {
                            arg_index += 1;
                            let pat = &arg.pat;
                            let arg_get = arg_regular_new(arg, &quote!{ #arg_index });
                            quote! {
                                let #pat = #arg_get; 
                            }
                        },
                        ArgMetadata::Payment =>
                            generate_payment_snippet(arg), // #[payment]
                        ArgMetadata::Multi(_) =>
                            panic!("multi args not accepted in function generate_call_method_fixed_args"),
                        ArgMetadata::VarArgs =>
                            panic!("var_args not accepted in function generate_call_method_fixed_args"),
                    }
                })
                .collect();

        let call_method_ident = generate_call_method_name(&self.name);
        let call = self.generate_call_to_method();
        let body_with_result = generate_body_with_result(&self.return_type, &call);
        let nr_args = arg_index + 1;

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


    fn generate_call_method_variable_nr_args(&self) -> proc_macro2::TokenStream {
        let payable_snippet = generate_payable_snippet(self);

        // let arg_expr = quote!{
        //     {
        //         if ___current_arg >= ___nr_args {
        //             self.api.signal_error(err_msg::ARG_WRONG_NUMBER);
        //         }
        //         ___current_arg += 1;
        //         ___current_arg - 1
        //     }
        // };

        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
            self.method_args
                .iter()
                .map(|arg| {
                    if arg.is_callback_arg {
                        panic!("callback args not allowed in public functions");
                    }

                    match &arg.metadata {
                        ArgMetadata::Single | ArgMetadata::VarArgs => {
                            dyn_endpoint_args_init(arg,
                                &quote! { &mut ___arg_loader },
                                &quote! { &___err_handler })
                        },
                        ArgMetadata::Payment => generate_payment_snippet(arg), // #[payment]
                        ArgMetadata::Multi(multi_attr) => { // #[multi(...)]
                            let count_expr = &multi_attr.count_expr; // TODO: parse count_expr and make sure it is a an expression in parantheses
                            
                            dyn_endpoint_multi_args_init(arg,
                                &quote! { &mut ___arg_loader },
                                &quote! { &___err_handler },
                                &quote! { #count_expr })
                        }
                        // ArgMetadata::VarArgs => { // #[var_args]
                        //     let pat = &arg.pat;
                        //     let push_snippet = arg_regular_multi(&arg, &arg_expr);
                        //     quote! {
                        //         let mut #pat = Vec::with_capacity((___nr_args - ___current_arg) as usize);
                        //         while ___current_arg < ___nr_args {
                        //             #push_snippet
                        //         }
                        //     }
                        // }
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

                let ___arg_loader = DynEndpointArgLoader::new(&self.api);
                let ___err_handler = DynEndpointErrHandler::new(&self.api);

                #(#arg_init_snippets)*

                // if ___current_arg < ___nr_args {
                //     self.api.signal_error(err_msg::ARG_WRONG_NUMBER);
                // }

                //#body_with_result
            }
        }
    }

}