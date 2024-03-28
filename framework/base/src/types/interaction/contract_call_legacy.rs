mod contract_call_convert;
mod contract_call_exec;
mod contract_call_no_payment;
mod contract_call_trait;
mod contract_call_with_any_payment;
mod contract_call_with_egld;
mod contract_call_with_egld_or_single_esdt;
mod contract_call_with_multi_esdt;

pub use contract_call_no_payment::ContractCallNoPayment;
pub use contract_call_trait::{ContractCall, ContractCallBase};
pub use contract_call_with_any_payment::ContractCallWithAnyPayment;
pub use contract_call_with_egld::ContractCallWithEgld;
pub use contract_call_with_egld_or_single_esdt::ContractCallWithEgldOrSingleEsdt;
pub use contract_call_with_multi_esdt::ContractCallWithMultiEsdt;
