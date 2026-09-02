# Release: SpaceCraft SDK v0.66.2

Date: 2026-06-18


## Short description:

SpaceCraft v0.66.2 is a patch release fixing a ManagedVec slice operation under the DebugApi environment, renaming it to the more descriptive clone_range, adding a copy-type optimization, and fixing an edge case in BoxedBytes::from_concat.


## Full description:

### Overview

v0.66.2 is a focused patch release. It resolves a bug in the ManagedVec slice method that only manifested in the DebugApi environment (used during testing and debugging), decouples the requires_drop logic from the execution environment, introduces the more clearly named clone_range method as a replacement for slice, adds a clone optimization for Copy types, and fixes an edge case allocation bug in BoxedBytes::from_concat.


### ManagedVec slice fix under DebugApi

The ManagedVec::slice method, which produces a cloned sub-range of the vector, contained a bug that only surfaced when running under the DebugApi environment (i.e., in blackbox tests and the Rust VM debugger).

The root cause was in the ManagedVecItem::requires_drop() method: it was querying the current execution environment to decide whether items need dropping, instead of relying solely on the type itself. Under DebugApi, this returned the wrong answer, causing items to be handled incorrectly during the slice operation.

The fix makes requires_drop() a purely type-level predicate, independent of the environment. The method now reflects only whether the Rust type itself requires dropping, which is the correct and consistent behavior across all API environments.


### Renamed slice to clone_range

The ManagedVec::slice method has been renamed to clone_range to better communicate its semantics: it clones a contiguous range of elements from the vector into a new ManagedVec.

The old slice name was misleading because it implied a non-owning view (as in Rust's standard &[T] slices), whereas the method actually allocates and returns a newly owned ManagedVec. The name clone_range makes the cloning and range-selection behavior explicit.

The original slice method is retained as a deprecated alias for backwards compatibility.


### Clone optimization for Copy types

When cloning a range of elements where the item type implements Copy, the framework can now perform the operation more efficiently. Since Copy types do not require individual item-level drop logic or deep cloning, the range can be copied in bulk without the overhead of per-element processing.

This optimization applies transparently whenever clone_range (or the deprecated slice) is called on a ManagedVec whose item type is Copy.


### BoxedBytes::from_concat edge case allocation fix

BoxedBytes::from_concat concatenates a slice of byte slices into a single BoxedBytes. An edge case was identified where passing an empty input (no slices, or all slices empty) could result in an incorrect allocation. This has been corrected so that the function always produces a well-formed BoxedBytes regardless of input.
