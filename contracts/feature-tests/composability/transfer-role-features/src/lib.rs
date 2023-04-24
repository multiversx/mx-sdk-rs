#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait TransferRoleFeatures:
    multiversx_sc_modules::transfer_role_proxy::TransferRoleProxyModule
{
    #[init]
    fn init(&self, whitelist: MultiValueEncoded<ManagedAddress>) {
        let mut whitelist_mapper = self.destination_whitelist();
        for addr in whitelist {
            let _ = whitelist_mapper.insert(addr);
        }
    }

    #[payable("*")]
    #[endpoint(forwardPayments)]
    fn forward_payments(
        &self,
        dest: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let original_caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();
        if payments.is_empty() {
            return;
        }

        if !self.blockchain().is_smart_contract(&dest) {
            self.transfer_to_user(original_caller, dest, payments.clone_value(), endpoint_name);
        } else {
            let mut args_buffer = ManagedArgBuffer::new();
            for arg in args {
                args_buffer.push_arg(arg);
            }

            self.transfer_to_contract_raw(
                original_caller,
                dest,
                payments.clone_value(),
                endpoint_name,
                args_buffer,
                None,
            );
        }
    }
}
