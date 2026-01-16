use core::ops::Add;

use generic_array::{
    ArrayLength, GenericArray, IntoArrayLength,
    typenum::{Const, Max, U1},
};

/// Describes the binary representation of a ManagedVecItem.
///
/// It is always an array that can be allocated directly on stack.
pub trait ManagedVecItemPayload {
    fn new_buffer() -> Self;

    fn payload_size() -> usize;

    fn payload_slice(&self) -> &[u8];

    fn payload_slice_mut(&mut self) -> &mut [u8];

    /// Takes a sub-payload item.
    ///
    /// ## Safety
    ///
    /// Only works correctly if the given index is correct, otherwise undefined behavior is possible.
    unsafe fn slice_unchecked<S: ManagedVecItemPayload>(&self, index: usize) -> &S;

    /// Takes a sub-payload item.
    ///
    /// ## Safety
    ///
    /// Only works correctly if the given index is correct, otherwise undefined behavior is possible.
    unsafe fn slice_unchecked_mut<S: ManagedVecItemPayload>(&mut self, index: usize) -> &mut S;
}

/// Empty ManagedVecItem.
///
/// Only used as type, never as implementation, since all ManagedVecItem have some data in them.
pub struct ManagedVecItemEmptyPayload;

impl ManagedVecItemPayload for ManagedVecItemEmptyPayload {
    fn new_buffer() -> Self {
        ManagedVecItemEmptyPayload
    }

    fn payload_size() -> usize {
        0
    }

    fn payload_slice(&self) -> &[u8] {
        &[]
    }

    fn payload_slice_mut(&mut self) -> &mut [u8] {
        &mut []
    }

    unsafe fn slice_unchecked<S: ManagedVecItemPayload>(&self, _index: usize) -> &S {
        unimplemented!()
    }

    unsafe fn slice_unchecked_mut<S: ManagedVecItemPayload>(&mut self, _index: usize) -> &mut S {
        unimplemented!()
    }
}

/// The main ManagedVecItemPayload implementation. Uses an array in its implementation.
#[repr(transparent)]
pub struct ManagedVecItemPayloadBuffer<N: ArrayLength> {
    pub buffer: GenericArray<u8, N>,
}

pub type ManagedVecItemPayloadBuffer1 = ManagedVecItemPayloadBuffer<U1>;

impl<const U: usize, N> From<[u8; U]> for ManagedVecItemPayloadBuffer<N>
where
    N: ArrayLength,
    Const<U>: IntoArrayLength<ArrayLength = N>,
{
    fn from(value: [u8; U]) -> Self {
        ManagedVecItemPayloadBuffer {
            buffer: GenericArray::from_array(value),
        }
    }
}

impl<N> ManagedVecItemPayloadBuffer<N>
where
    N: ArrayLength,
{
    pub fn into_array<const U: usize>(self) -> [u8; U]
    where
        Const<U>: IntoArrayLength<ArrayLength = N>,
    {
        self.buffer.into_array()
    }
}

impl<N: ArrayLength> ManagedVecItemPayload for ManagedVecItemPayloadBuffer<N> {
    fn new_buffer() -> Self {
        ManagedVecItemPayloadBuffer {
            buffer: GenericArray::default(),
        }
    }

    fn payload_size() -> usize {
        N::to_usize()
    }

    fn payload_slice(&self) -> &[u8] {
        &self.buffer[..]
    }

    fn payload_slice_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[..]
    }

    unsafe fn slice_unchecked<S: ManagedVecItemPayload>(&self, index: usize) -> &S {
        unsafe {
            let ptr = self.buffer.as_ptr().add(index);
            &*ptr.cast::<S>()
        }
    }

    unsafe fn slice_unchecked_mut<S: ManagedVecItemPayload>(&mut self, index: usize) -> &mut S {
        unsafe {
            let ptr = self.buffer.as_mut_ptr().add(index);
            &mut *ptr.cast::<S>()
        }
    }
}

/// Describes concatantion of smaller payloads into a larger one.
///
/// There is no runtime implementation, just a type-level addition.
///
/// Implemented via macros, because generic const expressions are currently unstable.
pub trait ManagedVecItemPayloadAdd<Rhs>: ManagedVecItemPayload
where
    Rhs: ManagedVecItemPayload,
{
    type Output: ManagedVecItemPayload;

    fn split_from_add(payload: &Self::Output) -> (&Self, &Rhs);

    fn split_mut_from_add(payload: &mut Self::Output) -> (&mut Self, &mut Rhs);
}

impl<N> ManagedVecItemPayloadAdd<ManagedVecItemEmptyPayload> for ManagedVecItemPayloadBuffer<N>
where
    N: ArrayLength,
{
    type Output = Self;

    fn split_from_add(payload: &Self::Output) -> (&Self, &ManagedVecItemEmptyPayload) {
        (payload, &ManagedVecItemEmptyPayload)
    }

    fn split_mut_from_add(
        _payload: &mut Self::Output,
    ) -> (&mut Self, &mut ManagedVecItemEmptyPayload) {
        unimplemented!()
    }
}

impl<N> ManagedVecItemPayloadAdd<ManagedVecItemPayloadBuffer<N>> for ManagedVecItemEmptyPayload
where
    N: ArrayLength,
{
    type Output = ManagedVecItemPayloadBuffer<N>;

    fn split_from_add(
        payload: &Self::Output,
    ) -> (&ManagedVecItemEmptyPayload, &ManagedVecItemPayloadBuffer<N>) {
        (&ManagedVecItemEmptyPayload, payload)
    }

    fn split_mut_from_add(
        _payload: &mut Self::Output,
    ) -> (
        &mut ManagedVecItemEmptyPayload,
        &mut ManagedVecItemPayloadBuffer<N>,
    ) {
        unimplemented!()
    }
}

impl<N1, N2> ManagedVecItemPayloadAdd<ManagedVecItemPayloadBuffer<N2>>
    for ManagedVecItemPayloadBuffer<N1>
where
    N1: ArrayLength,
    N2: ArrayLength,
    N1: Add<N2>,
    <N1 as Add<N2>>::Output: ArrayLength,
{
    type Output = ManagedVecItemPayloadBuffer<<N1 as Add<N2>>::Output>;

    fn split_from_add(
        payload: &Self::Output,
    ) -> (
        &ManagedVecItemPayloadBuffer<N1>,
        &ManagedVecItemPayloadBuffer<N2>,
    ) {
        unsafe {
            let ptr1 = payload.buffer.as_ptr();
            let ptr2 = ptr1.add(N1::to_usize());
            (
                &*ptr1.cast::<ManagedVecItemPayloadBuffer<N1>>(),
                &*ptr2.cast::<ManagedVecItemPayloadBuffer<N2>>(),
            )
        }
    }

    fn split_mut_from_add(
        payload: &mut Self::Output,
    ) -> (
        &mut ManagedVecItemPayloadBuffer<N1>,
        &mut ManagedVecItemPayloadBuffer<N2>,
    ) {
        unsafe {
            let ptr1 = payload.buffer.as_mut_ptr();
            let ptr2 = ptr1.add(N1::to_usize());
            (
                &mut *ptr1.cast::<ManagedVecItemPayloadBuffer<N1>>(),
                &mut *ptr2.cast::<ManagedVecItemPayloadBuffer<N2>>(),
            )
        }
    }
}

pub trait ManagedVecItemPayloadMax<Rhs>: ManagedVecItemPayload
where
    Rhs: ManagedVecItemPayload,
{
    type Max: ManagedVecItemPayload;
}

impl<P> ManagedVecItemPayloadMax<ManagedVecItemEmptyPayload> for P
where
    P: ManagedVecItemPayload,
{
    type Max = Self;
}

impl<N1, N2> ManagedVecItemPayloadMax<ManagedVecItemPayloadBuffer<N2>>
    for ManagedVecItemPayloadBuffer<N1>
where
    N1: ArrayLength,
    N2: ArrayLength,
    N1: Max<N2>,
    <N1 as Max<N2>>::Output: ArrayLength,
{
    type Max = ManagedVecItemPayloadBuffer<<N1 as Max<N2>>::Output>;
}
