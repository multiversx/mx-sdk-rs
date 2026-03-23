//! Tests for `StaticApi` and managed types in a multi-threaded environment.
//!
//! `StaticApi` stores its `ManagedTypeContainer` in thread-local storage, so every OS
//! thread owns a fully independent handle space.  These tests verify three properties:
//!
//! 1. **Thread isolation** – handles with the same numeric value on different threads
//!    hold independent data; writing to one thread's container leaves the other intact.
//!
//! 2. **Reset isolation** – calling `StaticApi::reset()` on thread A does *not* clear
//!    the handles that are live on thread B.
//!
//! 3. **Concurrent construction safety** – many threads can create managed types in
//!    parallel without panics, deadlocks, or data corruption.

use std::{
    sync::{Arc, Barrier},
    thread,
};

use multiversx_sc::imports::*;
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

fn main() {
    println!("\n=== StaticApi multi-thread tests ===\n");
    test_thread_isolation();
    test_reset_isolation();
    test_concurrent_construction();
    println!("\nAll tests passed.");
}
