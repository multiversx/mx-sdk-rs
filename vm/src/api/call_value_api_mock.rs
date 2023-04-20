use crate::{num_bigint, tx_mock::TxPanic, DebugApi};
use multiversx_sc::{
    api::{handle_to_be_bytes, CallValueApi, CallValueApiImpl, ManagedBufferApi},
    err_msg,
};
use num_traits::Zero;

impl CallValueApi for DebugApi {
    type CallValueApiImpl = DebugApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        DebugApi::new_from_static()
    }
}

impl CallValueApiImpl for DebugApi {
    fn check_not_payable(&self) {
        if self.input_ref().egld_value > num_bigint::BigUint::zero() {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_EGLD.to_string(),
            });
        }
        if self.esdt_num_transfers() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_ESDT.to_string(),
            });
        }
    }

    fn load_egld_value(&self, dest: Self::BigIntHandle) {
        self.set_big_uint(dest, self.input_ref().received_egld().clone())
    }

    fn load_all_esdt_transfers(&self, dest_handle: Self::ManagedBufferHandle) {
        let transfers = self.input_ref().received_esdt();
        self.mb_overwrite(dest_handle.clone(), &[]);

        for transfer in transfers {
            let token_identifier_handle = self.mb_new_from_bytes(&transfer.token_identifier);
            let amount_handle = self.bi_new_from_big_int(transfer.value.clone().into());

            self.mb_append_bytes(
                dest_handle.clone(),
                &handle_to_be_bytes(token_identifier_handle)[..],
            );
            self.mb_append_bytes(dest_handle.clone(), &transfer.nonce.to_be_bytes()[..]);
            self.mb_append_bytes(dest_handle.clone(), &handle_to_be_bytes(amount_handle)[..]);
        }
    }

    fn esdt_num_transfers(&self) -> usize {
        self.input_ref().received_esdt().len()
    }
}
