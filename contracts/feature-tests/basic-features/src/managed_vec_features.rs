elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ManagedVecFeatures {
    #[endpoint]
    fn managed_vec_push(
        &self,
        mv: ManagedVec<Self::TypeManager, BigUint>,
        item: BigUint,
    ) -> ManagedVec<Self::TypeManager, BigUint> {
        let mut result = mv;
        result.push(item);
        result
    }
}
