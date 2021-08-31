elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ManagedVecFeatures {
    #[endpoint]
    fn managed_vec_biguint_push(
        &self,
        mv: ManagedVec<Self::TypeManager, BigUint>,
        item: BigUint,
    ) -> ManagedVec<Self::TypeManager, BigUint> {
        let mut result = mv;
        result.push(item);
        result
    }

    #[endpoint]
    fn managed_vec_address_push(
        &self,
        mv: ManagedVec<Self::TypeManager, ManagedAddress>,
        item: ManagedAddress,
    ) -> ManagedVec<Self::TypeManager, ManagedAddress> {
        let mut result = mv;
        result.push(item);
        result
    }
}
