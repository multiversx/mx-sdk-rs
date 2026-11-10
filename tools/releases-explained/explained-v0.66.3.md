# Release: SpaceCraft SDK v0.66.3

Date: 2026-11-09


## Short description:

SpaceCraft v0.66.3 is a patch release introducing the opt-in managed-buffer-preload feature, which greatly reduces the gas cost of decoding large values from managed buffers, the new ManagedVecEncoded type for lists that are kept in encoded form, and a fix for a StaticApi deadlock in static buffer acquisition.


## Full description:

### Overview

v0.66.3 focuses on the gas cost of working with large collections stored in contract storage. It adds an opt-in framework feature that serves decoding reads from a cached copy of the buffer in wasm memory, a new ManagedVecEncoded type that avoids decoding lists that are only appended to or passed through, and a fix that prevents a deadlock when the static buffer is accessed re-entrantly. A few documentation corrections and a new developer tool are also included.


### The managed-buffer-preload feature

Decoding a large value from a ManagedBuffer (for example a long ManagedVec loaded from storage) normally performs many small reads, each of them a VM call. Those calls are charged for the length of the entire source buffer, so the total cost grows quadratically with the size of the value.

The new multiversx-sc feature managed-buffer-preload changes this. When enabled, the buffer is copied once into the static buffer, which lives in wasm memory, and subsequent fixed-size reads are served from that copy instead of through VM calls. The feature is off by default. It is enabled in a contract by adding managed-buffer-preload to the features of the multiversx-sc dependency.

Details:
- Nested managed buffers and big ints that are decoded from the buffer are created directly from the cached bytes. This matters for gas, because copying a slice through the VM is charged for the length of the whole source buffer, while creating a buffer from bytes is only charged for the length of the new buffer.
- If the buffer does not fit in the static buffer, or the static buffer is already in use, decoding transparently falls back to the previous behavior.
- The change is invisible to contract code: no source changes are needed besides enabling the feature.

A benchmark contract (large-vec-storage), with a preload variant, was added to the repository to measure the effect.


### ManagedVecEncoded

ManagedVecEncoded is a new list type that keeps its items in encoded form, in a single managed buffer. The buffer has the same layout as the top-encoding of a ManagedVec of the same item type, so the two are interchangeable in storage and in endpoint arguments and results.

As a consequence:
- Saving it to storage or returning it from an endpoint is a no-op.
- Loading it from storage or from an argument does not decode anything.
- push only encodes the new item and appends it to the buffer.
- Items are decoded lazily, one by one, while iterating.
- It converts to and from ManagedVec, and implements Clone, PartialEq, Eq, Debug, Default, the top-encoding traits and the ABI traits.
- It can wrap or expose its raw buffer (from_raw_buffer, as_raw_buffer, into_raw_buffer). No validation is performed when wrapping.

It is well suited for large lists that are mostly appended to, or that are loaded and saved without being inspected, where decoding into a ManagedVec and re-encoding would cost a lot of gas.

Two caveats:
- Iterating costs about as much as decoding the whole list. Without managed-buffer-preload it grows quadratically with the list size. With the feature enabled it is roughly linear, but the static buffer remains locked until the iterator is dropped.
- Item types whose nested encoding can be empty (such as the unit type) are not supported.


### StaticApi deadlock fix

With managed-buffer-preload enabled, decoding inside a ManagedBuffer::with_buffer_contents callback under StaticApi could deadlock, because both the callback and the preload logic tried to acquire the same guarded static buffer.

A new StaticVarApiImpl method, try_with_lockable_static_buffer, never blocks: it returns None if the static buffer is currently being accessed. StaticBufferRef::try_new_from_copy_bytes now uses it, so when the buffer is busy, the code falls back to the non-cached path instead of waiting. The default implementation simply delegates to the regular method, since in production the static data is not guarded. The blocking case only exists in debug environments, where the static data is protected by a lock.

Related to this, buffer handles are now allocated before the static buffer is accessed, because on some APIs both are guarded by the same lock.


### Documentation corrections

The doc comments of SetMapper and UnorderedSetMapper contained an inverted storage-cost claim. They now state correctly that SetMapper has the higher storage cost (3N + 1 entries), while UnorderedSetMapper is more compact (2N + 1 entries). The UnorderedSetMapper trade-off notes were also clarified to point out that swap_remove changes the positions of other elements. Thanks to the community contributor who reported this.


### Tooling and CI

- The framework feature tests in CI were extended to cover the new feature.
- The LLDB pretty printers script received a small fix.
