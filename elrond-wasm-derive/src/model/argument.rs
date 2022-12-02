/// Models any method argument from a contract, module or callable proxy trait.
/// Contains processed data from argument annotations.
#[derive(Clone, Debug)]
pub struct MethodArgument {
    pub original_pat: syn::Pat,
    pub pat: syn::Pat,
    pub ty: syn::Type,
    pub unprocessed_attributes: Vec<syn::Attribute>,
    pub metadata: ArgMetadata,
}

/// Models any method argument from a contract, module or callable proxy trait.
#[derive(Clone, Debug)]
pub struct ArgMetadata {
    pub payment: ArgPaymentMetadata,
    pub callback_call_result: bool,
    pub event_topic: bool,
}

impl Default for ArgMetadata {
    fn default() -> Self {
        ArgMetadata {
            payment: ArgPaymentMetadata::NotPayment,
            callback_call_result: false,
            event_topic: false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ArgPaymentMetadata {
    NotPayment,
    PaymentAmount,
    PaymentToken,
    PaymentNonce,
    PaymentMulti,
}

impl ArgPaymentMetadata {
    pub fn is_payment_arg(&self) -> bool {
        matches!(
            self,
            ArgPaymentMetadata::PaymentAmount
                | ArgPaymentMetadata::PaymentToken
                | ArgPaymentMetadata::PaymentNonce
                | ArgPaymentMetadata::PaymentMulti
        )
    }
}

impl MethodArgument {
    pub fn is_endpoint_arg(&self) -> bool {
        matches!(self.metadata.payment, ArgPaymentMetadata::NotPayment)
    }
}

#[derive(Clone, Debug, Default)]
pub struct TraitProperties {
    pub only_owner: bool,
    pub only_admin: bool,
    pub only_user_account: bool,
}
