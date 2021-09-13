elrond_wasm::imports!();

/// Storage mapper test.
#[elrond_wasm::module]
pub trait LinkedListMapperFeatures {
    #[view(getListMapper)]
    #[storage_mapper("list_mapper")]
    fn list_mapper(&self) -> LinkedListMapper<u32>;

    #[endpoint(listMapperPushBack)]
    fn list_mapper_push_back(&self, item: u32) {
        let _ = self.list_mapper().push_back(item);
    }

    #[endpoint(listMapperPushFront)]
    fn list_mapper_push_front(&self, item: u32) {
        let _ = self.list_mapper().push_front(item);
    }

    #[endpoint(listMapperPopFront)]
    fn list_mapper_pop_front(&self) -> OptionalResult<u32> {
        let node_op = self.list_mapper().pop_front();

        match node_op {
            Some(node) => OptionalResult::Some(node.into_value()),
            None => OptionalResult::None,
        }
    }

    #[endpoint(listMapperPopBack)]
    fn list_mapper_pop_back(&self) -> OptionalResult<u32> {
        let node_op = self.list_mapper().pop_back();

        match node_op {
            Some(node) => OptionalResult::Some(node.into_value()),
            None => OptionalResult::None,
        }
    }

    #[endpoint(listMapperFront)]
    fn list_mapper_front(&self) -> OptionalResult<u32> {
        if let Some(front) = self.list_mapper().front() {
            OptionalResult::Some(front.into_value())
        } else {
            OptionalResult::None
        }
    }

    #[endpoint(listMapperBack)]
    fn list_mapper_back(&self) -> OptionalResult<u32> {
        if let Some(front) = self.list_mapper().back() {
            OptionalResult::Some(front.into_value())
        } else {
            OptionalResult::None
        }
    }

    #[endpoint(listMapperPushAfter)]
    fn list_mapper_push_after(&self, node_id: u32, element: u32) -> OptionalResult<u32> {
        let mut node_opt = self.list_mapper().get_node_by_id(node_id);
        if node_opt.is_none() {
            return OptionalResult::None;
        }

        let mut node = node_opt.unwrap();
        node_opt = self.list_mapper().push_after(&mut node, element);
        match node_opt {
            Some(node) => OptionalResult::Some(node.into_value()),
            None => OptionalResult::None,
        }
    }

    #[endpoint(listMapperPushBefore)]
    fn list_mapper_push_before(&self, node_id: u32, element: u32) -> OptionalResult<u32> {
        let mut node_opt = self.list_mapper().get_node_by_id(node_id);
        if node_opt.is_none() {
            return OptionalResult::None;
        }

        let mut node = node_opt.unwrap();
        node_opt = self.list_mapper().push_before(&mut node, element);
        match node_opt {
            Some(node) => OptionalResult::Some(node.into_value()),
            None => OptionalResult::None,
        }
    }

    #[endpoint(listMapperRemoveNode)]
    fn list_mapper_remove_node(&self, node_id: u32) {
        let node_opt = self.list_mapper().get_node_by_id(node_id);
        if node_opt.is_none() {
            return;
        }

        let node = node_opt.unwrap();
        self.list_mapper().remove_node(&node);
    }

    #[endpoint(listMapperRemoveNodeById)]
    fn list_mapper_remove_node_by_id(&self, node_id: u32) -> OptionalResult<u32> {
        let node_op = self.list_mapper().remove_node_by_id(node_id);

        match node_op {
            Some(node) => OptionalResult::Some(node.into_value()),
            None => OptionalResult::None,
        }
    }

    #[endpoint(listMapperIterateByHand)]
    fn list_mapper_iterate_by_hand(&self, node_id: u32) -> MultiResultVec<u32> {
        let mut result = Vec::new();

        let mut node_opt = self.list_mapper().get_node_by_id(node_id);
        while node_opt.is_some() {
            let node = node_opt.unwrap();

            result.push(node.into_value());

            node_opt = self.list_mapper().get_node_by_id(node.get_next_node_id());
        }

        result.into()
    }

    #[endpoint(listMapperIterateByIter)]
    fn list_mapper_iterate_by_iter(&self, node_id: u32) -> MultiResultVec<u32> {
        let mut result = Vec::new();

        for value in self.list_mapper().iter_from_node_id(node_id) {
            result.push(value.into_value());
        }

        result.into()
    }
}
