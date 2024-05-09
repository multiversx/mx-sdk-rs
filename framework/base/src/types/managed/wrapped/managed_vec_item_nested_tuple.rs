use super::{
    ManagedVecItem, ManagedVecItemEmptyPayload, ManagedVecItemPayload, ManagedVecItemPayloadAdd,
};

/// Syntactic sugar, that allows us to more easily represent composite payloads as nested tuples.
pub trait ManagedVecItemNestedTuple {
    type PAYLOAD: ManagedVecItemPayload;
}

/// End of the list.
impl ManagedVecItemNestedTuple for () {
    type PAYLOAD = ManagedVecItemEmptyPayload;
}

impl<Head, Tail> ManagedVecItemNestedTuple for (Head, Tail)
where
    Head: ManagedVecItem,
    Tail: ManagedVecItemNestedTuple,
    Head::PAYLOAD: ManagedVecItemPayloadAdd<Tail::PAYLOAD>,
{
    type PAYLOAD = <Head::PAYLOAD as ManagedVecItemPayloadAdd<Tail::PAYLOAD>>::Output;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn managed_vec_item_nesteds_tuple_test() {
        assert_payload_size::<()>(0);
        assert_payload_size::<(u8, ())>(1);
        assert_payload_size::<(usize, ())>(4);
        assert_payload_size::<(usize, (usize, ()))>(8);
        assert_payload_size::<(Option<usize>, ())>(5);
    }

    fn assert_payload_size<N: ManagedVecItemNestedTuple>(expected_size: usize) {
        assert_eq!(N::PAYLOAD::payload_size(), expected_size);
    }
}
