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

    #[endpoint]
    fn managed_vec_set(
        &self,
        mv: ManagedVec<BigUint>,
        index: usize,
        item: &BigUint,
    ) -> ManagedVec<BigUint> {
        let mut result = mv;
        if result.set(index, item).is_ok() {
            result
        } else {
            sc_panic!("index out of bounds")
        }
    }

    #[endpoint]
    fn managed_vec_remove(&self, mv: ManagedVec<BigUint>, index: usize) -> ManagedVec<BigUint> {
        let mut result = mv;
        result.remove(index);

        result
    }

    #[endpoint]
    fn managed_vec_find(&self, mv: ManagedVec<BigUint>, item: BigUint) -> Option<usize> {
        mv.find(&item)
    }

    #[endpoint]
    fn managed_vec_contains(&self, mv: ManagedVec<BigUint>, item: BigUint) -> bool {
        mv.contains(&item)
    }
}
