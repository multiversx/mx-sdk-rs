use crate::abi::TypeAbi;
use alloc::{string::String, vec::Vec};
use elrond_codec::*;

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
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_slice().dep_encode(dest)
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_slice().dep_encode_or_exit(dest, c, exit);
    }
}

impl<T: NestedEncode> TopEncode for Queue<T> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_slice().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_slice().top_encode_or_exit(output, c, exit)
    }
}

/// Deserializes like a Vec.
impl<T: NestedDecode> NestedDecode for Queue<T> {
    #[inline]
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(Queue {
            vec: Vec::<T>::dep_decode(input)?,
            start: 0,
        })
    }

    #[inline]
    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        Queue {
            vec: Vec::<T>::dep_decode_or_exit(input, c, exit),
            start: 0,
        }
    }
}

/// Deserializes like a Vec.
impl<T: NestedDecode> TopDecode for Queue<T> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(Queue {
            vec: Vec::<T>::top_decode(input)?,
            start: 0,
        })
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        Queue {
            vec: Vec::<T>::top_decode_or_exit(input, c, exit),
            start: 0,
        }
    }
}

impl<T: TypeAbi> TypeAbi for Queue<T> {
    fn type_name() -> String {
        let mut repr = String::from("Queue<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }
}
