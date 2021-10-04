elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ManagedVecFeatures {
    #[endpoint]
    fn managed_vec_new(&self) -> ManagedVec<BigUint> {
        ManagedVec::new()
    }

    #[endpoint]
    fn managed_vec_biguint_push(
        &self,
        mv: ManagedVec<BigUint>,
        item: BigUint,
    ) -> ManagedVec<BigUint> {
        let mut result = mv;
        result.push(item);
        result
    }

    #[endpoint]
    fn managed_vec_biguint_eq(&self, mv1: &ManagedVec<BigUint>, mv2: &ManagedVec<BigUint>) -> bool {
        mv1 == mv2
    }

    #[endpoint]
    fn managed_vec_address_push(
        &self,
        mv: ManagedVec<ManagedAddress>,
        item: ManagedAddress,
    ) -> ManagedVec<ManagedAddress> {
        let mut result = mv;
        result.push(item);
        result
    }
}
