#![no_std]
#![allow(clippy::type_complexity)]

mod forwarder_raw_alt_init;
mod forwarder_raw_async;
mod forwarder_raw_common;
mod forwarder_raw_deploy_upgrade;
mod forwarder_raw_sync;

multiversx_sc::imports!();

/// Test contract for investigating async calls.
/// TODO: split into modules
#[multiversx_sc::contract]
pub trait ForwarderRaw:
    forwarder_raw_alt_init::ForwarderRawAlterativeInit
    + forwarder_raw_common::ForwarderRawCommon
    + forwarder_raw_async::ForwarderRawAsync
    + forwarder_raw_sync::ForwarderRawSync
    + forwarder_raw_deploy_upgrade::ForwarderRawDeployUpgrade
{
    #[init]
    fn init(&self) {}

    #[callback_raw]
    fn callback_raw(&self, args: MultiValueEncoded<ManagedBuffer>) {
        let payments = self.call_value().all_esdt_transfers();
        if payments.is_empty() {
            let egld_value = self.call_value().egld_value();
            if *egld_value > 0 {
                let _ = self.callback_payments().push(&(
                    EgldOrEsdtTokenIdentifier::egld(),
                    0,
                    egld_value.clone_value(),
                ));
            }
        } else {
            for payment in payments.into_iter() {
                let _ = self.callback_payments().push(&(
                    EgldOrEsdtTokenIdentifier::esdt(payment.token_identifier),
                    payment.token_nonce,
                    payment.amount,
                ));
            }
        }

        let args_as_vec = args.into_vec_of_buffers();
        self.callback_raw_event(&args_as_vec);

        let _ = self.callback_args().push(&args_as_vec);
    }

    #[event("callback_raw")]
    fn callback_raw_event(&self, arguments: &ManagedVec<Self::Api, ManagedBuffer>);
}
