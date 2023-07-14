use super::{ErrorApiImpl, HandleTypeInfo, ManagedTypeApiImpl};

pub trait CallValueApi: HandleTypeInfo {
    type CallValueApiImpl: CallValueApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn call_value_api_impl() -> Self::CallValueApiImpl;
}

pub trait CallValueApiImpl: ErrorApiImpl + ManagedTypeApiImpl + Sized {
    fn check_not_payable(&self);

    /// Retrieves the EGLD call value from the VM.
    /// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn load_egld_value(&self, dest_handle: Self::BigIntHandle);

    /// Loads all ESDT call values into a managed vec. Overwrites destination.
    fn load_all_esdt_transfers(&self, dest_handle: Self::ManagedBufferHandle);

    /// Gets the total number of ESDT transfers (Fungible/SFT/NFT).
    ///
    /// It is redundant, since the number can also be retrieved from `load_all_esdt_transfers`,
    /// but it is easier and cheaper to call when the content of those transfers is of no interest.
    fn esdt_num_transfers(&self) -> usize;
}
