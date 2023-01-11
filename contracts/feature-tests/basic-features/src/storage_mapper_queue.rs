multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait QueueMapperFeatures {
    #[view]
    #[storage_mapper("queue_mapper")]
    fn queue_mapper(&self) -> QueueMapper<u32>;

    #[endpoint]
    fn queue_mapper_push_back(&self, item: u32) {
        let mut queue_mapper = self.queue_mapper();
        queue_mapper.push_back(item);
    }

    #[endpoint]
    fn queue_mapper_pop_front(&self) -> Option<u32> {
        let mut queue_mapper = self.queue_mapper();
        queue_mapper.pop_front()
    }

    #[endpoint]
    fn queue_mapper_front(&self) -> SCResult<u32> {
        if let Some(front) = self.queue_mapper().front() {
            return Ok(front);
        }
        sc_panic!("Queue empty!")
    }
}
