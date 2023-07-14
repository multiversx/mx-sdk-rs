use super::VmApiImpl;
use multiversx_sc::api::{CallValueApi, CallValueApiImpl};

extern "C" {
    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);

    fn managedGetMultiESDTCallValue(resultHandle: i32);

    fn getNumESDTTransfers() -> i32;
}

impl CallValueApi for VmApiImpl {
    type CallValueApiImpl = VmApiImpl;

    #[inline]
    fn call_value_api_impl() -> Self::CallValueApiImpl {
        VmApiImpl {}
    }
}

impl CallValueApiImpl for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn load_egld_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetCallValue(dest);
        }
    }

    fn load_all_esdt_transfers(&self, dest_handle: Self::ManagedBufferHandle) {
        unsafe {
            managedGetMultiESDTCallValue(dest_handle);
        }
    }

    fn esdt_num_transfers(&self) -> usize {
        unsafe { getNumESDTTransfers() as usize }
    }
}
