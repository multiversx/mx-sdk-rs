use generic_array::typenum::U1;

use super::{
    ManagedVecItem, ManagedVecItemEmptyPayload, ManagedVecItemPayload, ManagedVecItemPayloadAdd,
    ManagedVecItemPayloadBuffer, ManagedVecItemPayloadMax,
};

/// Syntactic sugar, that allows us to more easily represent struct payloads as nested tuples.
pub trait ManagedVecItemStructPlTuple {
    type StructPayload: ManagedVecItemPayload;
}

/// Syntactic sugar, that allows us to more easily represent enum payloads as nested tuples.
pub trait ManagedVecItemEnumPlTuple {
    type EnumPayload: ManagedVecItemPayload;
}

/// End of the list.
impl ManagedVecItemStructPlTuple for () {
    type StructPayload = ManagedVecItemEmptyPayload;
}

/// End of the list.
impl ManagedVecItemEnumPlTuple for () {
    type EnumPayload = ManagedVecItemPayloadBuffer<U1>; // for the discriminant
}

impl<Head, Tail> ManagedVecItemStructPlTuple for (Head, Tail)
where
    Head: ManagedVecItem,
    Tail: ManagedVecItemStructPlTuple,
    Head::PAYLOAD: ManagedVecItemPayloadAdd<Tail::StructPayload>,
{
    type StructPayload = <Head::PAYLOAD as ManagedVecItemPayloadAdd<Tail::StructPayload>>::Output;
}

impl<Head, Tail> ManagedVecItemEnumPlTuple for (Head, Tail)
where
    Head: ManagedVecItem,
    Tail: ManagedVecItemStructPlTuple,
    Head::PAYLOAD: ManagedVecItemPayloadAdd<ManagedVecItemPayloadBuffer<U1>>,
    <Head::PAYLOAD as ManagedVecItemPayloadAdd<ManagedVecItemPayloadBuffer<U1>>>::Output:
        ManagedVecItemPayloadMax<Tail::StructPayload>,
{
    type EnumPayload = <<Head::PAYLOAD as ManagedVecItemPayloadAdd<
        ManagedVecItemPayloadBuffer<U1>,
    >>::Output as ManagedVecItemPayloadMax<Tail::StructPayload>>::Max;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn managed_vec_item_nested_tuple_struct_test() {
        assert_struct_payload_size::<()>(0);
        assert_struct_payload_size::<(u8, ())>(1);
        assert_struct_payload_size::<(usize, ())>(4);
        assert_struct_payload_size::<(usize, (usize, ()))>(8);
        assert_struct_payload_size::<(Option<usize>, ())>(5);
    }

    fn assert_struct_payload_size<N: ManagedVecItemStructPlTuple>(expected_size: usize) {
        assert_eq!(N::StructPayload::payload_size(), expected_size);
    }

    #[test]
    fn managed_vec_item_nested_tuple_enum_test() {
        assert_enum_payload_size::<()>(1);
        assert_enum_payload_size::<(u8, ())>(2);
        assert_enum_payload_size::<(usize, ())>(5);
        assert_enum_payload_size::<(usize, (usize, ()))>(5);
        assert_enum_payload_size::<(Option<usize>, ())>(6);
    }

    fn assert_enum_payload_size<N: ManagedVecItemEnumPlTuple>(expected_size: usize) {
        assert_eq!(N::EnumPayload::payload_size(), expected_size);
    }
}
