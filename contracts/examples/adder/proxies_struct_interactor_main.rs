#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
multiversx_sc::imports!();

pub struct TxProxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{
    pub address: ManagedOption<A, ManagedAddress<A>>,
}

impl<A> TxProxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{	fn init<
		Arg0: multiversx_sc::codec::CodecInto<BigUint<A>>,
	>(
		&mut self,
		initial_value: Arg0,
	) -> ContractDeploy<A, ()> {
		let ___opt_address___ = multiversx_sc::extract_opt_address!(self);
		let mut ___contract_deploy___ = multiversx_sc::constructors_proxy!(___opt_address___);
		___contract_deploy___.push_endpoint_arg(&initial_value);
		___contract_deploy___
	}

	fn sum(
		&mut self,
	) -> ContractCallNoPayment<A, BigUint<A>> {
		let ___address___ = multiversx_sc::extract_address!(self);
		let mut ___contract_call___ = multiversx_sc::endpoints_proxy!(getSum, ___address___);
		___contract_call___
	}

	fn upgrade<
		Arg0: multiversx_sc::codec::CodecInto<BigUint<A>>,
	>(
		&mut self,
		initial_value: Arg0,
	) -> ContractCallNoPayment<A, ()> {
		let ___address___ = multiversx_sc::extract_address!(self);
		let mut ___contract_call___ = multiversx_sc::endpoints_proxy!(upgrade, ___address___);
		ContractCall::proxy_arg(&mut ___contract_call___, &initial_value);
		___contract_call___
	}

	//Add desired amount to the storage variable. 
	fn add<
		Arg0: multiversx_sc::codec::CodecInto<BigUint<A>>,
	>(
		&mut self,
		value: Arg0,
	) -> ContractCallNoPayment<A, ()> {
		let ___address___ = multiversx_sc::extract_address!(self);
		let mut ___contract_call___ = multiversx_sc::endpoints_proxy!(add, ___address___);
		ContractCall::proxy_arg(&mut ___contract_call___, &value);
		___contract_call___
	}

}
