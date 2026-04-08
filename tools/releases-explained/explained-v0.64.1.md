# Release: SpaceCraft SDK v0.64.1

Date: 2026-01-13


## Short description:

SpaceCraft v0.64.1 is a patch release that improves backwards compatibility for the `TokenId` type and fixes critical debugger issues affecting error reporting.


## Full description:


### Overview

This patch release addresses two important areas: ensuring seamless backwards compatibility for token identifier handling and fixing debugger functionality that was broken in recent VM changes.


### 🔄 TokenId Backwards Compatibility

Enhanced backwards compatibility for the new `TokenId` type introduced in v0.64.0, ensuring smooth migration from legacy code.

#### Automatic Conversions

The framework now automatically converts legacy token identifier formats to the new standard:

- **Empty token identifiers** → `EGLD-000000`
- **Plain `EGLD` string** → `EGLD-000000`

These conversions now happen in multiple contexts:

**1. Deserialization (Decode)**

Whenever a `TokenId` is passed as argument or read from storage, the conversion occurs. This is to ease upgrade for old contracts that have data saved in the legacy format.

**2. Constructors**

Calling `new` is now the same as `new_backwards_compatible`.

**3. Type Conversions (`from`)**

Example: `TokenId::from("12345-6258d2")` or `TokenId::from("")`.

It is possible that some legacy contracts are using this pattern.

#### Unsafe Unchecked Constructor

Constructor `new_unchecked` bypasses all checks, but it is `unsafe`.


#### EgldOrEsdtTokenIdentifier Compatibility

The same conversion logic also applies to `EgldOrEsdtTokenIdentifier`, ensuring consistency across both token identifier types:

Code like `EgldOrEsdtTokenIdentifier::from(ManagedBuffer::new());`, while not recommended, will produce an instance equal to `EgldOrEsdtTokenIdentifier::egld()`.



### 🐛 Debugger Fixes

Critical fixes to the debugging experience that were broken following VM changes in recent releases.

#### Error Message Restoration

**The Problem:**
Following VM architectural changes a few releases ago, error messages when using the `StaticApi` (used extensively in testing and debugging) were being silently swallowed, making it extremely difficult to diagnose errors in tests or other tools.

The problem became apparent when working on the `TokenId` tests.

**The Fix:**
Error messages are now properly propagated and displayed when using the `StaticApi`, restoring full visibility into contract errors during development and testing.


#### Crash Prevention

**The Problem:**
The error trace mechanism in `StaticApi` was causing crashes when signal error was being called. While this is a panic scenario anyway, it was interfering with the correct processing of the error message.

**The Fix:**
Fixed the error trace implementation to prevent crashes, ensuring stable debugging sessions even when errors occur.


### Migration Notes

If you're upgrading from v0.64.0:

1. **No code changes required** - the backwards compatibility conversions are automatic
2. **Testing improvements** - you'll now see proper error messages in your test output
3. **Performance option** - consider the unsafe unchecked constructor only for proven hot paths

This release ensures that the revolutionary changes introduced in v0.64.0 work smoothly with existing codebases and provide a better debugging experience.
