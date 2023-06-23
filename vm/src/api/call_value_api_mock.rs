use crate::{num_bigint, tx_mock::TxPanic, DebugApi};
use multiversx_sc::{
    api::{CallValueApi, CallValueApiImpl},
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
        self.m_types_borrow_mut()
            .mb_set_vec_of_esdt_payments(dest_handle.get_raw_handle_unchecked(), transfers);
    }

    fn esdt_num_transfers(&self) -> usize {
        self.input_ref().received_esdt().len()
    }
}
