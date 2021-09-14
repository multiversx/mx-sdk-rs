elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ManagedVecFeatures {
    #[endpoint]
    fn managed_vec_biguint_push(
        &self,
        mv: ManagedVec<Self::Api, BigUint>,
        item: BigUint,
    ) -> ManagedVec<Self::Api, BigUint> {
        let mut result = mv;
        result.push(item);
        result
    }

    #[endpoint]
    fn managed_vec_address_push(
        &self,
        mv: ManagedVec<Self::Api, ManagedAddress>,
        item: ManagedAddress,
    ) -> ManagedVec<Self::Api, ManagedAddress> {
        let mut result = mv;
        result.push(item);
        result
    }
}
