//! Benchmark that measures heap memory usage across the lifecycle of managed types.
//!
//! For each type, three numbers are reported:
//!   create  – net bytes allocated after creating NUM_ITEMS instances
//!   hold    – net bytes still live after dropping the Rust-side handles
//!             (managed data stays inside ManagedTypeContainer until reset)
//!   residual – net bytes still live after `StaticApi::reset()`
//!
//! Note: after `StaticApi::reset()` some residual heap allocation is expected and normal.
//! Thread-locals, internal caches, and Rust runtime structures may retain a small amount
//! of memory that is not tied to the managed-type data itself. The numbers reported here
//! should be used to track *relative* changes over time, not to assert a zero residual.

use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicI64, Ordering},
};

use multiversx_sc::types::{
    BigFloat, BigInt, BigUint, EgldDecimals, EgldOrEsdtTokenIdentifier, EsdtTokenPayment,
    ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedDecimal, ManagedVec, ManagedVecItem,
    NumDecimals, TokenIdentifier,
};
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

/// Number of managed-type instances to allocate per benchmark phase.
const NUM_ITEMS: usize = 100_000;

/// Payload size used when constructing `ManagedBuffer` and `ManagedByteArray` instances.
const BUFFER_SIZE: usize = 100;

/// Measure and print memory usage for `NUM_ITEMS` instances of a managed type.
///
/// Columns printed:
///   `create`   – net bytes after allocating all instances
///   `hold`     – net bytes after dropping the Rust handles (data still in container)
///   `residual` – net bytes after `StaticApi::reset()` (ideally near zero)
fn bench_type<T, F>(label: &str, factory: F)
where
    F: Fn() -> T,
{
    StaticApi::reset();
    let baseline = allocated_bytes();

    let mut handles = Vec::with_capacity(NUM_ITEMS);
    for _ in 0..NUM_ITEMS {
        handles.push(factory());
    }
    let create = allocated_bytes() - baseline;

    drop(handles);
    let hold = allocated_bytes() - baseline;

    StaticApi::reset();
    let residual = allocated_bytes() - baseline;

    println!("  {label:<45} create={create:>10}, hold={hold:>10}, residual={residual:>8}");
}

/// Measure and print memory usage for a `ManagedVec` containing `NUM_ITEMS` items.
fn bench_managed_vec<T, F>(label: &str, factory: F)
where
    T: ManagedVecItem,
    F: Fn() -> T,
{
    StaticApi::reset();
    let baseline = allocated_bytes();

    let mut mv = ManagedVec::<StaticApi, T>::new();
    for _ in 0..NUM_ITEMS {
        mv.push(factory());
    }
    let create = allocated_bytes() - baseline;

    drop(mv);
    let hold = allocated_bytes() - baseline;

    StaticApi::reset();
    let residual = allocated_bytes() - baseline;

    println!("  {label:<45} create={create:>10}, hold={hold:>10}, residual={residual:>8}");
}

fn main() {
    // Warm up thread-locals so their one-time initialization cost is excluded
    // from every baseline measurement below.
    StaticApi::reset();

    let data = [0x42u8; BUFFER_SIZE];

    // -------------------------------------------------------------------------
    // Individual managed types
    // -------------------------------------------------------------------------
    println!(
        "\n=== Individual managed types ({NUM_ITEMS} instances each, {BUFFER_SIZE}-byte payloads where applicable) ===\n"
    );
    println!(
        "  {:<45} {:>16}  {:>14}  {:>12}",
        "type", "create (bytes)", "hold (bytes)", "residual"
    );
    println!("  {}", "-".repeat(95));

    bench_type("ManagedBuffer", || {
        ManagedBuffer::<StaticApi>::new_from_bytes(&data)
    });

    bench_type("BigUint", || BigUint::<StaticApi>::from(42u64));

    bench_type("BigInt", || BigInt::<StaticApi>::from(-42i64));

    bench_type("BigFloat", || BigFloat::<StaticApi>::from_frac(1, 2));

    bench_type("ManagedAddress", ManagedAddress::<StaticApi>::zero);

    bench_type("TokenIdentifier (EsdtTokenIdentifier)", || {
        TokenIdentifier::<StaticApi>::from("MYTOKEN-123456")
    });

    bench_type("EgldOrEsdtTokenIdentifier (EGLD)", || {
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld()
    });

    bench_type("EgldOrEsdtTokenIdentifier (ESDT)", || {
        EgldOrEsdtTokenIdentifier::<StaticApi>::esdt(TokenIdentifier::from("MYTOKEN-123456"))
    });

    bench_type("ManagedByteArray<32>", || {
        ManagedByteArray::<StaticApi, BUFFER_SIZE>::new_from_bytes(&data)
    });

    bench_type("ManagedDecimal<EgldDecimals (18)>", || {
        ManagedDecimal::<StaticApi, EgldDecimals>::from_raw_units(
            BigUint::from(1_000_000_000_000_000_000u64),
            EgldDecimals::new(),
        )
    });

    bench_type("ManagedDecimal<NumDecimals>", || {
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
            BigUint::from(1_000_000_000_000_000_000u64),
            18,
        )
    });

    bench_type("EsdtTokenPayment", || {
        EsdtTokenPayment::<StaticApi>::new(
            TokenIdentifier::from("MYTOKEN-123456"),
            0,
            BigUint::from(1000u64),
        )
    });

    // -------------------------------------------------------------------------
    // ManagedVec of managed types
    // -------------------------------------------------------------------------
    println!("\n=== ManagedVec of managed types ({NUM_ITEMS} items per vec) ===\n");
    println!(
        "  {:<45} {:>16}  {:>14}  {:>12}",
        "element type", "create (bytes)", "hold (bytes)", "residual"
    );
    println!("  {}", "-".repeat(95));

    bench_managed_vec("ManagedVec<ManagedBuffer>", || {
        ManagedBuffer::<StaticApi>::new_from_bytes(&data)
    });

    bench_managed_vec("ManagedVec<BigUint>", || BigUint::<StaticApi>::from(42u64));

    bench_managed_vec("ManagedVec<BigInt>", || BigInt::<StaticApi>::from(-42i64));

    bench_managed_vec("ManagedVec<ManagedAddress>", || {
        ManagedAddress::<StaticApi>::zero()
    });

    bench_managed_vec("ManagedVec<TokenIdentifier>", || {
        TokenIdentifier::<StaticApi>::from("MYTOKEN-123456")
    });

    bench_managed_vec("ManagedVec<EgldOrEsdtTokenIdentifier>", || {
        EgldOrEsdtTokenIdentifier::<StaticApi>::egld()
    });

    bench_managed_vec("ManagedVec<ManagedByteArray<100>>", || {
        ManagedByteArray::<StaticApi, BUFFER_SIZE>::new_from_bytes(&data)
    });

    bench_managed_vec("ManagedVec<ManagedDecimal<NumDecimals>>", || {
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
            BigUint::from(1_000_000_000_000_000_000u64),
            18,
        )
    });

    bench_managed_vec("ManagedVec<EsdtTokenPayment>", || {
        EsdtTokenPayment::<StaticApi>::new(
            TokenIdentifier::from("MYTOKEN-123456"),
            0,
            BigUint::from(1000u64),
        )
    });

    println!();
}
