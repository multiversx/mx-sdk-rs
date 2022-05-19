elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait SparseArrayFeatures {
    #[endpoint]
    fn sparse_array_create(&self, len: usize) -> SparseArray<Self::Api, 100> {
        SparseArray::new(len)
    }

    #[endpoint]
    fn sparse_array_get(&self, array: SparseArray<Self::Api, 100>, index: usize) -> usize {
        array.get(index)
    }

    #[endpoint]
    fn sparse_array_create_and_get(
        &self,
        len: usize,
        index: usize,
    ) -> MultiValue2<usize, SparseArray<Self::Api, 100>> {
        let array = SparseArray::new(len);
        let value = array.get(index);

        (value, array).into()
    }

    #[endpoint]
    fn sparse_array_set(
        &self,
        array: SparseArray<Self::Api, 100>,
        index: usize,
        value: usize,
    ) -> SparseArray<Self::Api, 100> {
        let mut array_clone = array.clone();
        array_clone.set(index, value);

        array_clone
    }

    #[endpoint]
    fn sparse_array_swap_remove(
        &self,
        array: SparseArray<Self::Api, 100>,
        index: usize,
    ) -> MultiValue2<usize, SparseArray<Self::Api, 100>> {
        let mut array_clone = array.clone();
        let value = array_clone.swap_remove(index);

        (value, array_clone).into()
    }
}
