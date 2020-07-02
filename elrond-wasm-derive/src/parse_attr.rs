static ATTR_PAYABLE: &str = "payable";
static ATTR_PAYMENT: &str = "payment";
static ATTR_VAR_ARGS: &str = "var_args";
static ATTR_EVENT: &str = "event";
static ATTR_INIT: &str = "init";
static ATTR_VIEW: &str = "view";
static ATTR_ENDPOINT: &str = "endpoint";
static ATTR_CALLBACK_DECL: &str = "callback";
static ATTR_CALLBACK_RAW_DECL: &str = "callback_raw";
static ATTR_CALLBACK_CALL: &str = "callback";
static ATTR_CALLBACK_ARG: &str = "callback_arg";
static ATTR_MULTI: &str = "multi";
static ATTR_STORAGE_GET: &str = "storage_get";
static ATTR_STORAGE_SET: &str = "storage_set";
static ATTR_STORAGE_GET_MUT: &str = "storage_get_mut";
static ATTR_MODULE: &str = "module";

fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
	attrs.iter().any(|attr| {
        if let Some(first_seg) = attr.path.segments.first() {
			return first_seg.ident == name
		};
		false
	})
}

pub fn is_callback_decl(m: &syn::TraitItemMethod) -> bool {
    has_attribute(&m.attrs, ATTR_CALLBACK_DECL)
}

pub fn is_callback_raw_decl(m: &syn::TraitItemMethod) -> bool {
    has_attribute(&m.attrs, ATTR_CALLBACK_RAW_DECL)
}

pub fn is_init(m: &syn::TraitItemMethod) -> bool {
    has_attribute(&m.attrs, ATTR_INIT)
}

pub fn is_payable(m: &syn::TraitItemMethod) -> bool {
    has_attribute(&m.attrs, ATTR_PAYABLE)
}

pub fn is_payment(pat: &syn::PatType) -> bool {
    has_attribute(&pat.attrs, ATTR_PAYMENT)
}

pub fn is_var_args(pat: &syn::PatType) -> bool {
    has_attribute(&pat.attrs, ATTR_VAR_ARGS)
}

pub fn is_callback_arg(pat: &syn::PatType) -> bool {
    has_attribute(&pat.attrs, ATTR_CALLBACK_ARG)
}

fn find_attr_one_string_arg(m: &syn::TraitItemMethod, attr_name: &str) -> Option<String> {
    let event_attr = m.attrs.iter().find(|attr| {
        if let Some(first_seg) = attr.path.segments.first() {
            first_seg.ident == attr_name
        } else {
            false
        }
    });
    match event_attr {        
        None => None,
        Some(attr) => {
            let result_str: String;
            let mut iter = attr.clone().tokens.into_iter();
            match iter.next() {
                Some(proc_macro2::TokenTree::Group(group)) => {
                    if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                        panic!("event paranthesis expected");
                    }
                    let mut iter2 = group.stream().into_iter();
                    match iter2.next() {
                        Some(proc_macro2::TokenTree::Literal(lit)) => {
                            let str_val = lit.to_string();
                            if !str_val.starts_with("\"") || !str_val.ends_with("\"") {
                                panic!("string literal expected as attribute argument");
                            }
                            let substr = &str_val[1..str_val.len()-1];
                            result_str = substr.to_string();
                        },
                        _ => panic!("literal expected as event identifier")
                    }
                },
                _ => panic!("missing event identifier")
            }

            if let Some(_) = iter.next() {
                panic!("event too many tokens in event attribute");
            }

            Some(result_str)
        }
    }
}

pub struct EventAttribute {
    pub identifier: Vec<u8>
}

impl EventAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<EventAttribute> {
        match find_attr_one_string_arg(m, ATTR_EVENT) {
            None => None,
            Some(event_str) => {
                if !event_str.starts_with("0x") {
                    panic!("event id should start with '0x'");
                }
                if event_str.len() != 64 + 2 {
                    panic!("event id should be 64 characters long (32 bytes)");
                }
                let substr = &event_str[2..];
                let result_str = substr.to_string();
                match hex::decode(result_str) {
                    Ok(v) => Some(EventAttribute{ identifier: v }),
                    Err(_) => panic!("could not parse event id"),
                }
            }
        }
    }
}

pub struct StorageGetAttribute {
    pub identifier: String
}

impl StorageGetAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<StorageGetAttribute> {
        match find_attr_one_string_arg(m, ATTR_STORAGE_GET) {
            None => None,
            Some(arg_str) => {
                Some(StorageGetAttribute{identifier: arg_str})
            }
        }
    }
}

pub struct StorageSetAttribute {
    pub identifier: String
}

impl StorageSetAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<StorageSetAttribute> {
        match find_attr_one_string_arg(m, ATTR_STORAGE_SET) {
            None => None,
            Some(arg_str) => {
                Some(StorageSetAttribute{identifier: arg_str})
            }
        }
    }
}

pub struct StorageGetMutAttribute {
    pub identifier: String
}

impl StorageGetMutAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<StorageSetAttribute> {
        match find_attr_one_string_arg(m, ATTR_STORAGE_GET_MUT) {
            None => None,
            Some(arg_str) => {
                Some(StorageSetAttribute{identifier: arg_str})
            }
        }
    }
}

/// Finds a method attribute with given name and 1 single optional argument.
/// In the result, the first option is for the attribute, the second for the argument.
fn find_attr_with_one_opt_token_tree_arg(m: &syn::TraitItemMethod, attr_name: &str) -> Option<Option<proc_macro2::TokenTree>> {
    let cc_attr = m.attrs.iter().find(|attr| {
        if let Some(first_seg) = attr.path.segments.first() {
            first_seg.ident == attr_name
        } else {
            false
        }
    });

    match cc_attr {        
        None => None,
        Some(attr) => {
            let mut iter = attr.clone().tokens.into_iter();
            let arg_token_tree: Option<proc_macro2::TokenTree> =
                match iter.next() {
                    Some(proc_macro2::TokenTree::Group(group)) => {
                        if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                            panic!("attribute paranthesis expected");
                        }
                        let mut iter2 = group.stream().into_iter();
                        match iter2.next() {
                            Some(token_tree) => Some(token_tree),
                            _ => panic!("attribute argument expected")
                        }
                    },
                    Some(_) => panic!("unexpected attribute argument tokens"),
                    None => None,
                };

            if let Some(_) = iter.next() {
                panic!("too many tokens in attribute");
            }
            
            Some(arg_token_tree)
        }
    }
}

#[derive(Clone, Debug)]
pub struct EndpointAttribute {
    pub endpoint_name: Option<syn::Ident>
}

impl EndpointAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<EndpointAttribute> {
        match find_attr_with_one_opt_token_tree_arg(m, ATTR_ENDPOINT) {
            None => None,
            Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(EndpointAttribute {
                endpoint_name: Some(ident),
            }),
            Some(None) => Some(EndpointAttribute {
                endpoint_name: None,
            }),
            _ => panic!("unexpected endpoint argument tokens"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ViewAttribute {
    pub view_name: Option<syn::Ident>
}

impl ViewAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<ViewAttribute> {
        match find_attr_with_one_opt_token_tree_arg(m, ATTR_VIEW) {
            None => None,
            Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(ViewAttribute {
                view_name: Some(ident),
            }),
            Some(None) => Some(ViewAttribute {
                view_name: None,
            }),
            _ => panic!("unexpected endpoint argument tokens"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CallbackCallAttribute {
    pub arg: syn::Ident
}

impl CallbackCallAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<CallbackCallAttribute> {
        match find_attr_with_one_opt_token_tree_arg(m, ATTR_CALLBACK_CALL) {
            None => None,
            Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(CallbackCallAttribute {
                arg: ident,
            }),
            _ => panic!("single identifier expected as callback argument"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleAttribute {
    pub arg: proc_macro2::TokenTree
}

impl ModuleAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<ModuleAttribute> {
        match find_attr_with_one_opt_token_tree_arg(m, ATTR_MODULE) {
            None => None,
            Some(Some(arg)) => Some(ModuleAttribute {
                arg: arg.clone(),
            }),
            Some(_) => panic!("module name required")
        }
    }
}

#[derive(Clone, Debug)]
pub struct MultiAttribute {
    pub count_expr: proc_macro2::TokenStream,
}

impl MultiAttribute { 
    pub fn parse(pat: &syn::PatType) -> Option<MultiAttribute> {
        let multi_attr = pat.attrs.iter().find(|attr| {
            if let Some(first_seg) = attr.path.segments.first() {
                first_seg.ident == ATTR_MULTI
            } else {
                false
            }
        });

        match multi_attr {        
            None => None,
            Some(attr) => {
                let mut iter = attr.clone().tokens.into_iter();
                let count_expr: proc_macro2::TokenStream =
                    match iter.next() {
                        Some(count_expr_group) => {
                            // some validation
                            match &count_expr_group {
                                proc_macro2::TokenTree::Group(group_data) => {
                                    match group_data.delimiter() {
                                        proc_macro2::Delimiter::Parenthesis | proc_macro2::Delimiter::Bracket => { /* ok */ },
                                        _ => panic!("paranetheses of brackets expected in #[multi] attribute")
                                    }
                                }
                                _ => panic!("illegal argument in #[multi] attribute")
                            }

                            // simply flatten to token stream and return
                            quote! { #count_expr_group }
                        },
                        _ => panic!("callback argument expected")
                    };

                if let Some(_) = iter.next() {
                    panic!("too many tokens in payable attribute");
                }
                
                Some(MultiAttribute {
                    count_expr: count_expr,
                })
            }
        }
    }
}
