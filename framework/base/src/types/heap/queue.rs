use crate::{
    abi::{TypeAbi, TypeName},
    codec::*,
};
use alloc::vec::Vec;

/// A simple queue struct that is able to push and pop without moving elements.
/// New items are pushed at the end, just like for a regular Vec.
/// When popping, instead of performing a regular Vec remove that would shift items,
/// a start index is moved up 1 position.
/// When serializing, items before the start index are ignored.
pub struct Queue<T> {
    vec: Vec<T>,
    start: usize,
}

impl<T> Queue<T> {
    #[inline]
    pub fn new() -> Self {
        Queue {
            vec: Vec::new(),
            start: 0,
        }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len() - self.start
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.vec[self.start..]
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    /// Returns a referenece to the first item in the queue, without removing it.
    /// Returns None if the queue is empty.
    pub fn peek(&self) -> Option<&T> {
        if self.start == self.vec.len() {
            return None;
        }
        let head_ref = &self.vec[self.start];
        Some(head_ref)
    }

    /// Returns a mutable referenece to the first item in the queue, without removing it.
    /// Returns None if the queue is empty.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.start == self.vec.len() {
            return None;
        }
        let head_ref = &mut self.vec[self.start];
        Some(head_ref)
    }

    /// Removes the first element from the queue and returns a reference to it.
    /// Does not physically extract the element from the underlying structure.
    /// Does nothing and returns None if the queue is empty.
    pub fn pop(&mut self) -> Option<&T> {
        if self.start == self.vec.len() {
            return None;
        }
        let head_ref = &self.vec[self.start];
        self.start += 1;
        Some(head_ref)
    }
}

/// Serializes identically to a Vec, entries before start index are ignored.
impl<T: NestedEncode> NestedEncode for Queue<T> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_slice().dep_encode_or_handle_err(dest, h)
    }
}

impl<T: NestedEncode> TopEncode for Queue<T> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_slice().top_encode_or_handle_err(output, h)
    }
}

/// Deserializes like a Vec.
impl<T: NestedDecode> NestedDecode for Queue<T> {
    #[inline]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Queue {
            vec: Vec::<T>::dep_decode_or_handle_err(input, h)?,
            start: 0,
        })
    }
}

/// Deserializes like a Vec.
impl<T: NestedDecode> TopDecode for Queue<T> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Queue {
            vec: Vec::<T>::top_decode_or_handle_err(input, h)?,
            start: 0,
        })
    }
}

impl<T: TypeAbi> TypeAbi for Queue<T> {
    fn type_name() -> TypeName {
        let mut repr = TypeName::from("Queue<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }
}
