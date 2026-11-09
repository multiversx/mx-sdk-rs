//! Tests around the static buffer cache, used by the `managed-buffer-preload` (decoding)
//! and `managed-buffer-builder-cached` (encoding) features.
//!
//! All expected results are the same with or without the features,
//! the CI runs this file with each combination.

use multiversx_sc::codec::{DecodeError, NestedEncode, TopDecode, TopEncode};
use multiversx_sc::types::{
    BigInt, BigUint, LockableStaticBuffer, ManagedBuffer, ManagedBufferBuilder, ManagedVec,
    ManagedVecEncoded, Payment, StaticBufferRef,
};
use multiversx_sc_scenario::api::StaticApi;

const CAPACITY: usize = LockableStaticBuffer::capacity();

/// Encodes to 30 bytes.
fn payment(i: u64) -> Payment<StaticApi> {
    Payment::new(
        "TOKEN-123456",
        i,
        BigUint::from(1000 + i).into_non_zero_or_panic(),
    )
}

fn payments(n: u64) -> ManagedVec<StaticApi, Payment<StaticApi>> {
    (0..n).map(payment).collect()
}

/// Item counts that encode to less, and more than the static buffer capacity.
const SMALL: u64 = 100;
const LARGE: u64 = 400;

fn top_encode_to_buffer<T: TopEncode>(value: &T) -> ManagedBuffer<StaticApi> {
    let mut buffer = ManagedBuffer::new();
    value.top_encode(&mut buffer).unwrap();
    buffer
}

/// The static buffer must always be released after decoding/encoding.
fn assert_static_buffer_free() {
    assert!(
        StaticBufferRef::<StaticApi>::try_new(b"").is_some(),
        "static buffer still locked"
    );
}

/// Takes the static buffer, so that the code under test cannot use it.
fn lock_static_buffer() -> StaticBufferRef<StaticApi> {
    StaticBufferRef::<StaticApi>::try_new(b"locked").unwrap()
}

/// Nested-encoded `(u64, ManagedBuffer)`: 8 bytes + 4 length bytes + payload.
fn u64_and_buffer_bytes(payload_len: usize, declared_len: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&7u64.to_be_bytes());
    bytes.extend_from_slice(&(declared_len as u32).to_be_bytes());
    bytes.extend((0..payload_len).map(|i| i as u8));
    bytes
}

fn check_decode_u64_and_buffer(total_len: usize) {
    let payload_len = total_len - 12;
    let bytes = u64_and_buffer_bytes(payload_len, payload_len);
    assert_eq!(bytes.len(), total_len);

    let decoded = <(u64, ManagedBuffer<StaticApi>)>::top_decode(
        ManagedBuffer::<StaticApi>::new_from_bytes(&bytes),
    )
    .unwrap();
    assert_eq!(decoded.0, 7);
    assert_eq!(decoded.1.to_boxed_bytes().as_slice(), &bytes[12..]);
    assert_static_buffer_free();
}

#[test]
fn decode_at_static_buffer_capacity_test() {
    check_decode_u64_and_buffer(CAPACITY - 1);
    check_decode_u64_and_buffer(CAPACITY);
    check_decode_u64_and_buffer(CAPACITY + 1);
}

fn check_decode_payments(n: u64) {
    let vec = payments(n);
    let encoded = top_encode_to_buffer(&vec);

    let decoded = ManagedVec::<StaticApi, Payment<StaticApi>>::top_decode(encoded.clone()).unwrap();
    assert_eq!(decoded, vec);
    assert_static_buffer_free();

    // with the static buffer taken, decoding falls back to VM calls
    let lock = lock_static_buffer();
    let decoded = ManagedVec::<StaticApi, Payment<StaticApi>>::top_decode(encoded).unwrap();
    assert_eq!(decoded, vec);
    assert!(lock.contents_eq(b"locked"));
    drop(lock);
    assert_static_buffer_free();
}

#[test]
fn decode_payments_test() {
    check_decode_payments(SMALL);
    check_decode_payments(LARGE);
}

#[test]
fn decode_managed_types_test() {
    let value = (
        ManagedBuffer::<StaticApi>::from("abcdef"),
        BigUint::<StaticApi>::from(1_000_000_000_000_000_000u64),
        BigInt::<StaticApi>::from(-1_000_000_000_000i64),
    );
    let encoded = top_encode_to_buffer(&value);

    type Decoded = (
        ManagedBuffer<StaticApi>,
        BigUint<StaticApi>,
        BigInt<StaticApi>,
    );

    let decoded = Decoded::top_decode(encoded.clone()).unwrap();
    assert_eq!(decoded, value);
    assert_static_buffer_free();

    let lock = lock_static_buffer();
    let decoded = Decoded::top_decode(encoded).unwrap();
    assert_eq!(decoded, value);
    drop(lock);
}

fn check_decode_truncated(total_len: usize) {
    // the length prefix claims one more byte than available
    let payload_len = total_len - 12;
    let bytes = u64_and_buffer_bytes(payload_len, payload_len + 1);

    let result = <(u64, ManagedBuffer<StaticApi>)>::top_decode(
        ManagedBuffer::<StaticApi>::new_from_bytes(&bytes),
    );
    assert_eq!(result, Err(DecodeError::INPUT_TOO_SHORT));
    assert_static_buffer_free();

    // input shorter than the fixed-size field
    let result = <(u64, u64)>::top_decode(ManagedBuffer::<StaticApi>::new_from_bytes(
        &bytes[..total_len.min(12)],
    ));
    assert_eq!(result, Err(DecodeError::INPUT_TOO_SHORT));
    assert_static_buffer_free();
}

#[test]
fn decode_truncated_test() {
    check_decode_truncated(100);
    check_decode_truncated(CAPACITY + 100);
}

fn check_zip_encoded_iterators(n: u64) {
    let first = ManagedVecEncoded::<StaticApi, Payment<StaticApi>>::from(payments(n));
    let second = ManagedVecEncoded::<StaticApi, Payment<StaticApi>>::from(payments(n));

    // the first iterator holds the static buffer, the second must not read from it
    let mut count = 0;
    for (i, (a, b)) in first.iter().zip(second.iter()).enumerate() {
        assert_eq!(a, payment(i as u64));
        assert_eq!(b, payment(i as u64));
        count += 1;
    }
    assert_eq!(count, n);
    assert_static_buffer_free();
}

#[test]
fn zip_encoded_iterators_test() {
    check_zip_encoded_iterators(SMALL);
    check_zip_encoded_iterators(LARGE);
}

fn chunk(len: usize, seed: u8) -> Vec<u8> {
    (0..len).map(|i| (i as u8).wrapping_add(seed)).collect()
}

fn check_builder_past_capacity() {
    let mut builder = ManagedBufferBuilder::<StaticApi>::default();
    let mut expected = Vec::new();

    // crosses the static buffer capacity with byte slices
    for seed in 0..4 {
        let bytes = chunk(3000, seed);
        builder.append_bytes(&bytes);
        expected.extend_from_slice(&bytes);
    }

    // and with managed buffers
    let bytes = chunk(4000, 100);
    builder.append_managed_buffer(&ManagedBuffer::new_from_bytes(&bytes));
    expected.extend_from_slice(&bytes);

    let bytes = chunk(10, 200);
    builder.append_bytes(&bytes);
    expected.extend_from_slice(&bytes);

    let result = builder.into_managed_buffer();
    assert_eq!(result.to_boxed_bytes().as_slice(), expected.as_slice());
}

#[test]
fn builder_past_capacity_test() {
    check_builder_past_capacity();
    assert_static_buffer_free();
}

#[test]
fn builder_static_buffer_locked_test() {
    let lock = lock_static_buffer();
    check_builder_past_capacity();
    assert!(lock.contents_eq(b"locked"));
    drop(lock);
    assert_static_buffer_free();
}

#[test]
fn builder_single_managed_buffer_over_capacity_test() {
    let bytes = chunk(CAPACITY + 1, 0);
    let mut builder = ManagedBufferBuilder::<StaticApi>::new_from_slice(b"prefix");
    builder.append_managed_buffer(&ManagedBuffer::new_from_bytes(&bytes));
    let result = builder.into_managed_buffer();

    let mut expected = b"prefix".to_vec();
    expected.extend_from_slice(&bytes);
    assert_eq!(result.to_boxed_bytes().as_slice(), expected.as_slice());
    assert_static_buffer_free();
}

fn check_encode_while_iterating(n: u64) {
    let mve = ManagedVecEncoded::<StaticApi, Payment<StaticApi>>::from(payments(n));

    // decoding (iterator) and encoding (builder) compete for the static buffer
    let mut builder = ManagedBufferBuilder::<StaticApi>::default();
    for item in mve.iter() {
        item.dep_encode(&mut builder).unwrap();
    }
    let result = builder.into_managed_buffer();

    assert_eq!(&result, mve.as_raw_buffer());
    assert_static_buffer_free();
}

#[test]
fn encode_while_iterating_test() {
    check_encode_while_iterating(SMALL);
    check_encode_while_iterating(LARGE);
}
