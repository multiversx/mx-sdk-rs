use multiversx_sc::types::{ManagedArgBuffer, ManagedBuffer, ManagedVec};
use multiversx_sc_scenario::{api::StaticApi, managed_test_util::check_managed_top_encode_decode};

type SA = StaticApi;

// -----------------------------------------------------------------------
// ManagedArgBuffer serialized alone
// -----------------------------------------------------------------------

/// Empty arg buffer top-encodes to empty bytes.
#[test]
fn test_managed_arg_buffer_empty() {
    let arg_buffer = ManagedArgBuffer::<SA>::new();
    check_managed_top_encode_decode(arg_buffer, &[]);
}

/// A single empty argument is encoded as a 4-byte zero length prefix.
#[test]
fn test_managed_arg_buffer_single_empty_arg() {
    let mut arg_buffer = ManagedArgBuffer::<SA>::new();
    arg_buffer.push_arg_raw(ManagedBuffer::new());
    // dep_encode("") = [0, 0, 0, 0]
    check_managed_top_encode_decode(arg_buffer, &[0, 0, 0, 0]);
}

/// A single non-empty argument is length-prefixed.
#[test]
fn test_managed_arg_buffer_single_arg() {
    let mut arg_buffer = ManagedArgBuffer::<SA>::new();
    arg_buffer.push_arg_raw(ManagedBuffer::new_from_bytes(b"abc"));
    // dep_encode("abc") = [0, 0, 0, 3, 97, 98, 99]
    check_managed_top_encode_decode(arg_buffer, &[0, 0, 0, 3, b'a', b'b', b'c']);
}

/// Two arguments are concatenated, each with a 4-byte length prefix.
#[test]
fn test_managed_arg_buffer_two_args() {
    let mut arg_buffer = ManagedArgBuffer::<SA>::new();
    arg_buffer.push_arg_raw(ManagedBuffer::new_from_bytes(b"abc"));
    arg_buffer.push_arg_raw(ManagedBuffer::new_from_bytes(b"de"));
    // dep_encode("abc") ++ dep_encode("de")
    // = [0,0,0,3, 97,98,99, 0,0,0,2, 100,101]
    check_managed_top_encode_decode(
        arg_buffer,
        &[0, 0, 0, 3, b'a', b'b', b'c', 0, 0, 0, 2, b'd', b'e'],
    );
}

/// Three arguments with varying content.
#[test]
fn test_managed_arg_buffer_three_args() {
    let mut arg_buffer = ManagedArgBuffer::<SA>::new();
    arg_buffer.push_arg_raw(ManagedBuffer::new_from_bytes(b"foo"));
    arg_buffer.push_arg_raw(ManagedBuffer::new());
    arg_buffer.push_arg_raw(ManagedBuffer::new_from_bytes(b"bar"));
    // dep_encode("foo") ++ dep_encode("") ++ dep_encode("bar")
    // = [0,0,0,3, 102,111,111, 0,0,0,0, 0,0,0,3, 98,97,114]
    check_managed_top_encode_decode(
        arg_buffer,
        &[
            0, 0, 0, 3, b'f', b'o', b'o', // "foo"
            0, 0, 0, 0, // ""
            0, 0, 0, 3, b'b', b'a', b'r', // "bar"
        ],
    );
}

// -----------------------------------------------------------------------
// ManagedArgBuffer as an element of ManagedVec
// -----------------------------------------------------------------------

/// Empty outer vec top-encodes to empty bytes.
#[test]
fn test_managed_vec_of_arg_buffers_empty() {
    let vec = ManagedVec::<SA, ManagedArgBuffer<SA>>::new();
    check_managed_top_encode_decode(vec, &[]);
}

/// A vec containing a single empty arg buffer.
/// dep_encode(empty ManagedArgBuffer) = [0,0,0,0]  (inner count = 0)
#[test]
fn test_managed_vec_of_single_empty_arg_buffer() {
    let mut vec = ManagedVec::<SA, ManagedArgBuffer<SA>>::new();
    vec.push(ManagedArgBuffer::<SA>::new());
    check_managed_top_encode_decode(vec, &[0, 0, 0, 0]);
}

/// A vec containing a single arg buffer with one arg.
/// dep_encode(ManagedArgBuffer{["abc"]}) = [0,0,0,1, 0,0,0,3, 97,98,99]
///   (inner count = 1, then dep_encode("abc"))
#[test]
fn test_managed_vec_of_single_arg_buffer_with_one_arg() {
    let mut vec = ManagedVec::<SA, ManagedArgBuffer<SA>>::new();
    let mut ab = ManagedArgBuffer::<SA>::new();
    ab.push_arg_raw(ManagedBuffer::new_from_bytes(b"abc"));
    vec.push(ab);
    check_managed_top_encode_decode(vec, &[0, 0, 0, 1, 0, 0, 0, 3, b'a', b'b', b'c']);
}

/// A vec with two arg buffers, each containing one argument.
/// Outer top-encode = concat of dep-encodes of each element.
///   dep_encode(["abc"]) = [0,0,0,1, 0,0,0,3, 97,98,99]
///   dep_encode(["de"])  = [0,0,0,1, 0,0,0,2, 100,101]
#[test]
fn test_managed_vec_of_two_arg_buffers() {
    let mut vec = ManagedVec::<SA, ManagedArgBuffer<SA>>::new();

    let mut ab1 = ManagedArgBuffer::<SA>::new();
    ab1.push_arg_raw(ManagedBuffer::new_from_bytes(b"abc"));
    vec.push(ab1);

    let mut ab2 = ManagedArgBuffer::<SA>::new();
    ab2.push_arg_raw(ManagedBuffer::new_from_bytes(b"de"));
    vec.push(ab2);

    check_managed_top_encode_decode(
        vec,
        &[
            // dep_encode(ab1): inner_count=1, "abc"
            0, 0, 0, 1, 0, 0, 0, 3, b'a', b'b', b'c',
            // dep_encode(ab2): inner_count=1, "de"
            0, 0, 0, 1, 0, 0, 0, 2, b'd', b'e',
        ],
    );
}

/// A vec where one arg buffer carries multiple arguments.
///   dep_encode(["abc","de"]) = [0,0,0,2, 0,0,0,3,97,98,99, 0,0,0,2,100,101]
///   dep_encode([])           = [0,0,0,0]
#[test]
fn test_managed_vec_of_mixed_arg_buffers() {
    let mut vec = ManagedVec::<SA, ManagedArgBuffer<SA>>::new();

    let mut ab1 = ManagedArgBuffer::<SA>::new();
    ab1.push_arg_raw(ManagedBuffer::new_from_bytes(b"abc"));
    ab1.push_arg_raw(ManagedBuffer::new_from_bytes(b"de"));
    vec.push(ab1);

    vec.push(ManagedArgBuffer::<SA>::new()); // empty arg buffer

    check_managed_top_encode_decode(
        vec,
        &[
            // dep_encode(ab1): inner_count=2, "abc", "de"
            0, 0, 0, 2, 0, 0, 0, 3, b'a', b'b', b'c', 0, 0, 0, 2, b'd', b'e',
            // dep_encode(ab2 empty): inner_count=0
            0, 0, 0, 0,
        ],
    );
}
