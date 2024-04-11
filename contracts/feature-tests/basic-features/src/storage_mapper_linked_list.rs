multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
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
    fn list_mapper_pop_front(&self) -> OptionalValue<u32> {
        let node_op = self.list_mapper().pop_front();

        match node_op {
            Some(node) => OptionalValue::Some(node.into_value()),
            None => OptionalValue::None,
        }
    }

    #[endpoint(listMapperPopBack)]
    fn list_mapper_pop_back(&self) -> OptionalValue<u32> {
        let node_op = self.list_mapper().pop_back();

        match node_op {
            Some(node) => OptionalValue::Some(node.into_value()),
            None => OptionalValue::None,
        }
    }

    #[endpoint(listMapperFront)]
    fn list_mapper_front(&self) -> OptionalValue<u32> {
        if let Some(front) = self.list_mapper().front() {
            OptionalValue::Some(front.into_value())
        } else {
            OptionalValue::None
        }
    }

    #[endpoint(listMapperBack)]
    fn list_mapper_back(&self) -> OptionalValue<u32> {
        if let Some(front) = self.list_mapper().back() {
            OptionalValue::Some(front.into_value())
        } else {
            OptionalValue::None
        }
    }

    #[endpoint(listMapperPushAfter)]
    fn list_mapper_push_after(&self, node_id: u32, element: u32) -> OptionalValue<u32> {
        let mut node_opt = self.list_mapper().get_node_by_id(node_id);
        if node_opt.is_none() {
            return OptionalValue::None;
        }

        let mut node = node_opt.unwrap();
        node_opt = self.list_mapper().push_after(&mut node, element);
        match node_opt {
            Some(node) => OptionalValue::Some(node.into_value()),
            None => OptionalValue::None,
        }
    }

    #[endpoint(listMapperPushBefore)]
    fn list_mapper_push_before(&self, node_id: u32, element: u32) -> OptionalValue<u32> {
        let mut node_opt = self.list_mapper().get_node_by_id(node_id);
        if node_opt.is_none() {
            return OptionalValue::None;
        }

        let mut node = node_opt.unwrap();
        node_opt = self.list_mapper().push_before(&mut node, element);
        match node_opt {
            Some(node) => OptionalValue::Some(node.into_value()),
            None => OptionalValue::None,
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
    fn list_mapper_remove_node_by_id(&self, node_id: u32) -> OptionalValue<u32> {
        let node_op = self.list_mapper().remove_node_by_id(node_id);

        match node_op {
            Some(node) => OptionalValue::Some(node.into_value()),
            None => OptionalValue::None,
        }
    }

    #[endpoint(listMapperSetValue)]
    fn list_mapper_set_value(&self, node_id: u32, new_value: u32) {
        let node = self.list_mapper().get_node_by_id(node_id).unwrap();
        self.list_mapper().set_node_value(node, new_value);
    }

    #[endpoint(listMapperSetValueById)]
    fn list_mapper_set_value_by_id(&self, node_id: u32, new_value: u32) {
        self.list_mapper().set_node_value_by_id(node_id, new_value);
    }

    #[endpoint(listMapperIterateByHand)]
    fn list_mapper_iterate_by_hand(&self, node_id: u32) -> MultiValueEncoded<u32> {
        let mut result = MultiValueEncoded::new();

        let mut node_opt = self.list_mapper().get_node_by_id(node_id);
        while node_opt.is_some() {
            let node = node_opt.unwrap();

            result.push(node.into_value());

            node_opt = self.list_mapper().get_node_by_id(node.get_next_node_id());
        }

        result
    }

    #[endpoint(listMapperIterateByIter)]
    fn list_mapper_iterate_by_iter(&self, node_id: u32) -> MultiValueEncoded<u32> {
        let mut result = MultiValueEncoded::new();

        for value in self.list_mapper().iter_from_node_id(node_id) {
            result.push(value.into_value());
        }

        result
    }
}
