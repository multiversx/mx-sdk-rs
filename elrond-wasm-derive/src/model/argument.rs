#[derive(Clone, Debug)]
pub struct MethodArgument {
	pub index: i32,
	pub pat: syn::Pat,
	pub ty: syn::Type,
	pub remaining_attributes: Vec<syn::Attribute>,
	pub metadata: ArgMetadata,
}

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
	Payment,
	PaymentToken,
}
