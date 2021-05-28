/// Models any method argument from a contract, module or callable proxy trait.
/// Contains processed data from argument annotations.
#[derive(Clone, Debug)]
pub struct MethodArgument {
	pub pat: syn::Pat,
	pub ty: syn::Type,
	pub remaining_attributes: Vec<syn::Attribute>,
	pub metadata: ArgMetadata,
}

/// Models any method argument from a contract, module or callable proxy trait.
#[derive(Clone, Debug)]
pub struct ArgMetadata {
	pub payment: ArgPaymentMetadata,
	pub var_args: bool,
	pub callback_call_result: bool,
	pub event_topic: bool,
}

#[derive(Clone, Debug)]
pub enum ArgPaymentMetadata {
	NotPayment,
	PaymentAmount,
	PaymentToken,
	PaymentNonce,
}

impl ArgPaymentMetadata {
	pub fn is_payment_arg(&self) -> bool {
		matches!(
			self,
			ArgPaymentMetadata::PaymentAmount
				| ArgPaymentMetadata::PaymentToken
				| ArgPaymentMetadata::PaymentNonce
		)
	}
}

impl MethodArgument {
	pub fn is_endpoint_arg(&self) -> bool {
		matches!(self.metadata.payment, ArgPaymentMetadata::NotPayment)
	}
}
