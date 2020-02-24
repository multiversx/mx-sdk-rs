static ATTR_PAYABLE: &str = "payable";
static ATTR_EVENT: &str = "event";
static ATTR_PRIVATE: &str = "private";
static ATTR_CALLBACK_DECL: &str = "callback";
static ATTR_CALLBACK_CALL: &str = "callback";

fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
	attrs.iter().any(|attr| {
        if let Some(first_seg) = attr.path.segments.first() {
			return first_seg.value().ident == name
		};
		false
	})
}

pub fn is_private(m: &syn::TraitItemMethod) -> bool {
    has_attribute(&m.attrs, ATTR_PRIVATE)
}

pub fn is_callback_decl(m: &syn::TraitItemMethod) -> bool {
    has_attribute(&m.attrs, ATTR_CALLBACK_DECL)
}

#[derive(Clone, Debug)]
pub struct PayableAttribute {
    pub payment_arg: Option<syn::FnArg>
}

impl PayableAttribute { 
    pub fn parse(m: &syn::TraitItemMethod) -> Option<PayableAttribute> {
        let payable_attr = m.attrs.iter().find(|attr| {
            if let Some(first_seg) = attr.path.segments.first() {
                first_seg.value().ident == ATTR_PAYABLE
            } else {
                false
            }
        });

        match payable_attr {        
            None => None,
            Some(attr) => {
                let mut iter = attr.clone().tts.into_iter();
                let payment_arg_ident: Option<syn::Ident> =
                    match iter.next() {
                        Some(proc_macro2::TokenTree::Group(group)) => {
                            if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                                panic!("payable paranthesis expected");
                            }
                            let mut iter2 = group.stream().into_iter();
                            match iter2.next() {
                                Some(proc_macro2::TokenTree::Ident(ident)) => Some(ident),
                                _ => panic!("payable argument name expected")
                            }
                        },
                        _ => None
                    };

                if let Some(_) = iter.next() {
                    panic!("too many tokens in payable attribute");
                }

                // find the payment argument
                let payment_arg = if let Some(arg_ident) = payment_arg_ident {
                    let arg_opt = m.sig.decl.inputs
                        .iter()
                        .find(|arg| {
                            match *arg {
                                syn::FnArg::Captured(arg_captured) => {
                                    let pat = &arg_captured.pat;
                                    match pat {
                                        syn::Pat::Ident(pat_ident) => {
                                            pat_ident.ident == arg_ident
                                        },
                                        _ => false
                                    }
                                }
                                _ => false
                            }
                        });
                    if let Some(arg) = arg_opt {
                        Some(arg.clone())
                    } else {
                        panic!("Payment argument not found. Payable attribute argument must be the name of an argument of the method.")
                    }
                } else {
                    None
                };
                
                Some(PayableAttribute {
                    payment_arg: payment_arg
                })
            }
        }
    }
}

pub struct EventAttribute {
    pub identifier: Vec<u8>
}

impl EventAttribute {
    pub fn parse(m: &syn::TraitItemMethod) -> Option<EventAttribute> {
        let event_attr = m.attrs.iter().find(|attr| {
            if let Some(first_seg) = attr.path.segments.first() {
                first_seg.value().ident == ATTR_EVENT
            } else {
                false
            }
        });
        match event_attr {        
            None => None,
            Some(attr) => {
                let result_str: String;
                let mut iter = attr.clone().tts.into_iter();
                match iter.next() {
                    Some(proc_macro2::TokenTree::Group(group)) => {
                        if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                            panic!("event paranthesis expected");
                        }
                        let mut iter2 = group.stream().into_iter();
                        match iter2.next() {
                            Some(proc_macro2::TokenTree::Literal(lit)) => {
                                let str_val = lit.to_string();
                                if !str_val.starts_with("\"0x") || !str_val.ends_with("\"") {
                                    panic!("string literal expected in event id");
                                }
                                if str_val.len() != 64 + 4 {
                                    panic!("event id should be 64 characters long");
                                }
                                let substr = &str_val[3..str_val.len()-1];
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
                
                match hex::decode(result_str) {
                    Ok(v) => Some(EventAttribute{ identifier: v }),
                    Err(_) => panic!("could not parse event id"),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CallbackCallAttribute {
    pub arg: syn::Ident
}

impl CallbackCallAttribute { 
    pub fn parse(m: &syn::TraitItemMethod) -> Option<CallbackCallAttribute> {
        let cc_attr = m.attrs.iter().find(|attr| {
            if let Some(first_seg) = attr.path.segments.first() {
                first_seg.value().ident == ATTR_CALLBACK_CALL
            } else {
                false
            }
        });

        match cc_attr {        
            None => None,
            Some(attr) => {
                let mut iter = attr.clone().tts.into_iter();
                let callback_method_ident: syn::Ident =
                    match iter.next() {
                        Some(proc_macro2::TokenTree::Group(group)) => {
                            if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                                panic!("callback paranthesis expected");
                            }
                            let mut iter2 = group.stream().into_iter();
                            match iter2.next() {
                                Some(proc_macro2::TokenTree::Ident(ident)) => ident,
                                _ => panic!("callback argument name expected")
                            }
                        },
                        _ => panic!("callback argument expected")
                    };

                if let Some(_) = iter.next() {
                    panic!("too many tokens in payable attribute");
                }
                
                Some(CallbackCallAttribute {
                    arg: callback_method_ident
                })
            }
        }
    }
}
