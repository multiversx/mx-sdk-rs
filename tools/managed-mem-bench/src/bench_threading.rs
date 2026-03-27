//! Tests for `StaticApi` and managed types in a multi-threaded environment.
//!
//! `StaticApi` stores its `ManagedTypeContainer` in thread-local storage, so every OS
//! thread owns a fully independent handle space.  These tests verify five properties:
//!
//! 1. **Thread isolation** – handles with the same numeric value on different threads
//!    hold independent data; writing to one thread's container leaves the other intact.
//!
//! 2. **Reset isolation** – calling `StaticApi::reset()` on thread A does *not* clear
//!    the handles that are live on thread B.
//!
//! 3. **Concurrent construction safety** – many threads can create managed types in
//!    parallel without panics, deadlocks, or data corruption.
//!
//! 4. **Handle identity is thread-local** – `ManagedBuffer<StaticApi>` is `!Send`:
//!    the compiler prevents moving managed values across threads, which is correct
//!    because the underlying storage is thread-local.  Each thread allocates handles
//!    starting at 0, so the same i32 value on two threads refers to completely
//!    different entries in each thread's independent container.
//!
//! 5. **Correct cross-thread data transfer** – the safe pattern is to materialise
//!    managed-type values into plain Rust types (`BoxedBytes`, `Vec<u8>`, `u64`, …)
//!    on the source thread, send those plain values across the thread boundary, and
//!    reconstruct the managed types on the destination thread.

use std::{
    sync::{Arc, Barrier},
    thread,
};

use multiversx_sc::api::HandleConstraints;
use multiversx_sc::imports::*;
use multiversx_sc::types::ManagedType;
use multiversx_sc_scenario::api::StaticApi;

// ---------------------------------------------------------------------------
// Test 1 – Thread isolation
// ---------------------------------------------------------------------------
// Each thread creates a ManagedBuffer with its own unique bytes.  Because the
// ManagedTypeContainer is thread-local, the first handle allocated on every
// thread is handle 0, yet it stores completely different data.  We verify this
// by reading the bytes back on each thread after all threads have finished
// writing, ensuring no thread's payload leaked into another thread's container.
fn test_thread_isolation() {
    const NUM_THREADS: usize = 8;
    let barrier = Arc::new(Barrier::new(NUM_THREADS));

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|id| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                StaticApi::reset();

                // Each thread writes a unique 4-byte pattern derived from its id.
                let payload = vec![id as u8; 4];
                let buf = ManagedBuffer::<StaticApi>::new_from_bytes(&payload);

                // Synchronise: all threads must have written before any reads.
                barrier.wait();

                // Read back on the same thread – must match what *this* thread wrote.
                let result = buf.to_boxed_bytes();
                let bytes = result.as_slice();
                assert_eq!(
                    bytes,
                    &payload[..],
                    "Thread {id}: expected {:?}, got {:?}",
                    payload,
                    bytes
                );
            })
        })
        .collect();

    for h in handles {
        h.join().expect("thread panicked");
    }

    println!("[PASS] test_thread_isolation");
}

// ---------------------------------------------------------------------------
// Test 2 – Reset isolation
// ---------------------------------------------------------------------------
// Thread B creates a ManagedBuffer, then signals thread A to call
// `StaticApi::reset()`.  After the reset, thread B reads its buffer back and
// asserts that the value is still intact – proving that thread A's reset did
// not touch thread B's container.
fn test_reset_isolation() {
    use std::sync::{Condvar, Mutex};

    // Two-phase hand-shake: B signals A to reset, then A signals B to check.
    let pair = Arc::new((Mutex::new(0u8), Condvar::new()));
    let pair_b = Arc::clone(&pair);

    let thread_b = thread::spawn(move || {
        StaticApi::reset();
        let payload = b"hello from B";
        let buf = ManagedBuffer::<StaticApi>::new_from_bytes(payload);

        // Phase 1: tell thread A it may call reset().
        {
            let (lock, cvar) = &*pair_b;
            let mut state = lock.lock().unwrap();
            *state = 1;
            cvar.notify_one();
        }

        // Phase 2: wait for thread A to finish its reset.
        {
            let (lock, cvar) = &*pair_b;
            let mut state = lock.lock().unwrap();
            while *state < 2 {
                state = cvar.wait(state).unwrap();
            }
        }

        // Thread B's buffer must still hold the original bytes.
        let result = buf.to_boxed_bytes();
        let bytes = result.as_slice();
        assert_eq!(
            bytes, payload,
            "Thread B's buffer was corrupted after thread A's reset"
        );
    });

    // Thread A: wait for B to create its buffer, reset *A's* container, notify B.
    {
        let (lock, cvar) = &*pair;
        let mut state = lock.lock().unwrap();
        while *state < 1 {
            state = cvar.wait(state).unwrap();
        }
    }
    StaticApi::reset(); // only clears thread A's container
    {
        let (lock, cvar) = &*pair;
        let mut state = lock.lock().unwrap();
        *state = 2;
        cvar.notify_one();
    }

    thread_b.join().expect("thread B panicked");
    println!("[PASS] test_reset_isolation");
}

// ---------------------------------------------------------------------------
// Test 3 – Concurrent construction safety
// ---------------------------------------------------------------------------
// Spin up many threads, each allocating a batch of managed types as fast as
// possible, all starting at the same instant (via a Barrier).  No panics or
// deadlocks should occur, and every value must round-trip correctly.
fn test_concurrent_construction() {
    const NUM_THREADS: usize = 16;
    const ITEMS_PER_THREAD: usize = 1_000;
    let barrier = Arc::new(Barrier::new(NUM_THREADS));

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|id| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                StaticApi::reset();
                barrier.wait(); // start all threads simultaneously

                // Create a mix of managed types and verify each one.
                for i in 0..ITEMS_PER_THREAD {
                    let n = (id * ITEMS_PER_THREAD + i) as u64;

                    let big = BigUint::<StaticApi>::from(n);
                    assert_eq!(
                        big.to_u64(),
                        Some(n),
                        "Thread {id} item {i}: BigUint round-trip failed"
                    );

                    let payload = n.to_be_bytes();
                    let buf = ManagedBuffer::<StaticApi>::new_from_bytes(&payload);
                    let result = buf.to_boxed_bytes();
                    assert_eq!(
                        result.as_slice(),
                        &payload,
                        "Thread {id} item {i}: ManagedBuffer round-trip failed"
                    );
                }

                StaticApi::reset();
            })
        })
        .collect();

    for h in handles {
        h.join().expect("thread panicked");
    }

    println!("[PASS] test_concurrent_construction");
}

// ---------------------------------------------------------------------------
// Test 4 – Handle identity is thread-local
// ---------------------------------------------------------------------------
// `ManagedBuffer<StaticApi>` is `!Send`: the compiler prevents moving managed
// values across threads, which is correct because the underlying storage is
// thread-local.
//
// Every fresh thread-local container assigns handles starting at 0, so two
// independent threads both call their first allocation "handle 0", yet the
// data stored at handle 0 is completely independent per thread.
//
// The safe way to observe this is to materialise only the raw i32 handle
// number and the serialised bytes on the source thread, then compare on the
// destination thread.
fn test_handle_identity_is_thread_local() {
    use std::sync::mpsc;

    // Thread A stores "hello from A" and reports which handle number it was
    // assigned, along with the actual bytes it read back.
    let (tx, rx) = mpsc::channel::<(i32, Vec<u8>)>();

    let thread_a = thread::spawn(move || {
        StaticApi::reset();
        let buf = ManagedBuffer::<StaticApi>::new_from_bytes(b"hello from A");
        // get_handle() returns i32 for StaticApi (HandleType = RawHandle = i32).
        let raw: i32 = buf.get_handle().get_raw_handle();
        let bytes = buf.to_boxed_bytes().as_slice().to_vec();
        tx.send((raw, bytes)).unwrap();
        // buf is dropped here, inside thread A's container – no cross-thread drop.
    });
    thread_a.join().unwrap();

    let (handle_from_a, data_from_a) = rx.recv().unwrap();

    // Main thread: fresh container, first allocation also gets handle 0.
    StaticApi::reset();
    let buf_main = ManagedBuffer::<StaticApi>::new_from_bytes(b"hello from main");
    let raw_main: i32 = buf_main.get_handle().get_raw_handle();

    // Both threads assigned the same handle number from their own containers.
    assert_eq!(
        handle_from_a, raw_main,
        "fresh thread-local containers both start numbering handles at 0"
    );

    // The data at that handle on the main thread is NOT thread A's data.
    let main_bytes = buf_main.to_boxed_bytes().as_slice().to_vec();
    assert_ne!(
        main_bytes, data_from_a,
        "same handle number holds DIFFERENT data on main thread vs thread A"
    );
    assert_eq!(main_bytes, b"hello from main");

    println!(
        "[PASS] test_handle_identity_is_thread_local  \
         (handle #{handle_from_a}: thread A had {:?}, main thread has {:?})",
        String::from_utf8_lossy(&data_from_a),
        String::from_utf8_lossy(&main_bytes),
    );
}

// ---------------------------------------------------------------------------
// Test 5 – Correct cross-thread data transfer via serialisation
// ---------------------------------------------------------------------------
// Because handles are thread-local, you cannot move a managed-type *object*
// across threads and expect it to work.  The correct pattern is:
//
//   1. Materialise the value into a plain Rust type on the source thread.
//   2. Send that plain value (it is genuinely Send/Sync).
//   3. Reconstruct the managed type from the plain value on the destination
//      thread.
//
// This test runs a tiny "pipeline": a producer thread creates several managed
// values, serialises them, and sends them through an `mpsc` channel.  The
// consumer thread (main) deserialises them back into managed types and verifies
// the round-trip.
fn test_cross_thread_data_transfer() {
    use std::sync::mpsc;

    // The messages we send across the boundary are plain Rust types – no
    // managed handles, no thread-local state.
    struct Payload {
        buffer_bytes: Vec<u8>,
        biguint_bytes: Vec<u8>, // big-endian serialisation of a BigUint
        native_u64: u64,
    }

    let (tx, rx) = mpsc::channel::<Payload>();

    let producer = thread::spawn(move || {
        StaticApi::reset();

        let buf = ManagedBuffer::<StaticApi>::new_from_bytes(b"cross-thread payload");
        let big = BigUint::<StaticApi>::from(0xDEAD_BEEF_u64);
        let n: u64 = 42;

        // Materialise before sending.
        tx.send(Payload {
            buffer_bytes: buf.to_boxed_bytes().as_slice().to_vec(),
            biguint_bytes: big.to_bytes_be().as_slice().to_vec(),
            native_u64: n,
        })
        .unwrap();
    });
    producer.join().unwrap();

    let payload = rx.recv().unwrap();

    // Consumer (main thread): reconstruct managed types from the plain values.
    StaticApi::reset();

    let buf = ManagedBuffer::<StaticApi>::new_from_bytes(&payload.buffer_bytes);
    assert_eq!(buf.to_boxed_bytes().as_slice(), b"cross-thread payload");

    let big = BigUint::<StaticApi>::from_bytes_be(&payload.biguint_bytes);
    assert_eq!(big.to_u64(), Some(0xDEAD_BEEF_u64));

    assert_eq!(payload.native_u64, 42u64);

    println!("[PASS] test_cross_thread_data_transfer");
}

fn main() {
    println!("\n=== StaticApi multi-thread tests ===\n");
    test_thread_isolation();
    test_reset_isolation();
    test_concurrent_construction();
    test_handle_identity_is_thread_local();
    test_cross_thread_data_transfer();
    println!("\nAll tests passed.");
}
