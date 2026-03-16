//! Benchmark that measures heap memory usage across the lifecycle of managed buffers.
//!
//! Note: after `StaticApi::reset()` some residual heap allocation is expected and normal.
//! Thread-locals, internal caches, and Rust runtime structures may retain a small amount
//! of memory that is not tied to the managed-type data itself. The numbers reported here
//! should be used to track *relative* changes over time, not to assert a zero residual.

use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicI64, Ordering},
};

use multiversx_sc::types::ManagedBuffer;
use multiversx_sc_scenario::api::StaticApi;

/// Global allocator wrapper that tracks net allocated heap bytes.
struct TrackingAllocator;

static ALLOCATED: AtomicI64 = AtomicI64::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            ALLOCATED.fetch_add(layout.size() as i64, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        ALLOCATED.fetch_sub(layout.size() as i64, Ordering::Relaxed);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn allocated_bytes() -> i64 {
    ALLOCATED.load(Ordering::Relaxed)
}

const NUM_BUFFERS: usize = 100_000;
const BUFFER_SIZE: usize = 100;

fn main() {
    // Warm up thread-locals so their one-time initialization cost is excluded
    // from the baseline measurement.
    StaticApi::reset();

    let baseline = allocated_bytes();
    println!("Baseline allocated bytes:    {baseline}");

    // --- Phase 1: create buffers ---
    let data = vec![0x42u8; BUFFER_SIZE];
    let mut handles = Vec::with_capacity(NUM_BUFFERS);
    for _ in 0..NUM_BUFFERS {
        handles.push(ManagedBuffer::<StaticApi>::new_from_bytes(&data));
    }

    let after_create = allocated_bytes();
    println!(
        "After creating {NUM_BUFFERS} x {BUFFER_SIZE}-byte ManagedBuffers: {after_create} bytes"
    );
    println!(
        "  Net increase:              {} bytes",
        after_create - baseline
    );

    // --- Phase 2: drop the Rust-side handles ---
    // Only the thin Rust-side handle structs are freed here. The actual buffer
    // data lives inside the ManagedTypeContainer and is not released until
    // StaticApi::reset() is called.
    drop(handles);

    let after_drop = allocated_bytes();
    println!("After dropping Rust handles: {after_drop} bytes");
    println!(
        "  Net change from baseline:  {} bytes",
        after_drop - baseline
    );

    // --- Phase 3: reset the static API (clears ManagedTypeContainer) ---
    // Most managed-type memory is freed here, but a small residual is expected:
    // thread-locals, allocator metadata, and Rust runtime structures may keep
    // some bytes alive beyond this point.
    StaticApi::reset();

    let after_reset = allocated_bytes();
    let residual = after_reset - baseline;
    println!("After StaticApi::reset():    {after_reset} bytes");
    println!("  Net change from baseline:  {residual} bytes");

    if residual == 0 {
        println!("Result: all tracked memory was released after reset.");
    } else {
        println!(
            "Result: {residual} bytes remain after reset (some residual is expected \
             from thread-locals and runtime structures)."
        );
    }
}
