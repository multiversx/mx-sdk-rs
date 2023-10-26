multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ManagedVecFeatures {
    #[endpoint]
    fn managed_vec_new(&self) -> ManagedVec<BaseBigUint> {
        ManagedVec::new()
    }

    #[endpoint]
    fn managed_vec_biguint_push(
        &self,
        mv: ManagedVec<BaseBigUint>,
        item: BaseBigUint,
    ) -> ManagedVec<BaseBigUint> {
        let mut result = mv;
        result.push(item);
        result
    }

    #[endpoint]
    fn managed_vec_biguint_eq(&self, mv1: &ManagedVec<BaseBigUint>, mv2: &ManagedVec<BaseBigUint>) -> bool {
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
        mv: ManagedVec<BaseBigUint>,
        index: usize,
        item: &BaseBigUint,
    ) -> ManagedVec<BaseBigUint> {
        let mut result = mv;
        if result.set(index, item).is_ok() {
            result
        } else {
            sc_panic!("index out of bounds")
        }
    }

    #[endpoint]
    fn managed_vec_remove(&self, mv: ManagedVec<BaseBigUint>, index: usize) -> ManagedVec<BaseBigUint> {
        let mut result = mv;
        result.remove(index);

        result
    }

    #[endpoint]
    fn managed_vec_find(&self, mv: ManagedVec<BaseBigUint>, item: BaseBigUint) -> Option<usize> {
        mv.find(&item)
    }

    #[endpoint]
    fn managed_vec_contains(&self, mv: ManagedVec<BaseBigUint>, item: BaseBigUint) -> bool {
        mv.contains(&item)
    }

    #[endpoint]
    fn managed_vec_array_push(
        &self,
        mut mv: ManagedVec<[u8; 5]>,
        item: [u8; 5],
    ) -> ManagedVec<[u8; 5]> {
        mv.push(item);
        mv
    }

    #[endpoint]
    fn managed_ref_explicit(&self, mv: ManagedVec<BaseBigUint>, index: usize) -> BaseBigUint {
        let value: ManagedRef<BaseBigUint> = mv.get(index);
        let with_explicit_lifetime: ManagedRef<'_, BaseBigUint> = value;
        (*with_explicit_lifetime).clone()
    }
}
