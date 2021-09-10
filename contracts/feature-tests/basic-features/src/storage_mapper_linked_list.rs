elrond_wasm::imports!();

/// Storage mapper test.
#[elrond_wasm::module]
pub trait LinkedListMapperFeatures {
    #[view]
    #[storage_mapper("list_mapper")]
    fn list_mapper(&self) -> LinkedListMapper<u32>;

    #[endpoint]
    fn list_mapper_push_back(&self, item: u32) {
        let mut list_mapper = self.list_mapper();
        list_mapper.push_back(item);
    }

    #[endpoint]
    fn list_mapper_pop_front(&self) -> Option<u32> {
        let mut list_mapper = self.list_mapper();
        list_mapper.pop_front()
    }

    #[endpoint]
    fn list_mapper_front(&self) -> SCResult<u32> {
        if let Some(front) = self.list_mapper().front() {
            return Ok(front);
        }
        sc_error!("List empty!")
    }
}
