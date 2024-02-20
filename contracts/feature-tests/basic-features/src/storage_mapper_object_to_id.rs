use crate::types::ExampleStructManaged;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ObjectToIdMapperFeatures {
    #[endpoint]
    fn object_to_id_mapper_get_id(&self, object: ExampleStructManaged<Self::Api>) -> AddressId {
        self.object_ids().get_id(&object)
    }

    #[endpoint]
    fn object_to_id_mapper_get_object(
        &self,
        object_id: ObjectId,
    ) -> Option<ExampleStructManaged<Self::Api>> {
        self.object_ids().get_object(object_id)
    }

    #[endpoint]
    fn object_to_id_mapper_contains(&self, object_id: ObjectId) -> bool {
        self.object_ids().contains_id(object_id)
    }

    #[endpoint]
    fn object_to_id_mapper_set(&self, object: &ExampleStructManaged<Self::Api>) -> AddressId {
        self.object_ids().insert_new(object)
    }

    #[endpoint]
    fn object_to_id_mapper_get_id_or_insert(
        &self,
        object: ExampleStructManaged<Self::Api>,
    ) -> AddressId {
        self.object_ids().get_id_or_insert(object)
    }

    #[endpoint]
    fn object_to_id_mapper_remove_by_id(
        &self,
        object_id: ObjectId,
    ) -> Option<ExampleStructManaged<Self::Api>> {
        self.object_ids().remove_by_id(object_id)
    }

    #[endpoint]
    fn address_to_id_mapper_remove_by_address(
        &self,
        object: ExampleStructManaged<Self::Api>,
    ) -> AddressId {
        self.object_ids().remove_by_object(&object)
    }

    #[storage_mapper("object_ids")]
    fn object_ids(&self) -> ObjectToIdMapper<Self::Api, ExampleStructManaged<Self::Api>>;
}
