multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait UniqueIdMapperFeatures {
    #[endpoint]
    fn init_unique_id_mapper(&self, len: usize) {
        self.unique_id_mapper().set_initial_len(len);
    }

    #[endpoint]
    fn unique_id_mapper_get(&self, index: usize) -> UniqueId {
        self.unique_id_mapper().get(index)
    }

    #[endpoint]
    fn unique_id_mapper_swap_remove(&self, index: usize) -> UniqueId {
        self.unique_id_mapper().swap_remove(index)
    }

    #[endpoint]
    fn unique_id_mapper_set(&self, index: usize, id: UniqueId) {
        self.unique_id_mapper().set(index, id);
    }

    #[view]
    #[storage_mapper("unique_id_mapper")]
    fn unique_id_mapper(&self) -> UniqueIdMapper<Self::Api>;
}
