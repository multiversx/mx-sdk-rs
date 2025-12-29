# Release: SpaceCraft SDK v0.64.0

Date: 2025-11-17


## Short description:

SpaceCraft v0.64.0 brings significant improvements to the MultiversX smart contract development experience, featuring a modernized payments API, enhanced type safety, and updated toolchain compatibility.


## Full description:


### Overview

This major release brings significant improvements to the MultiversX smart contract development experience, featuring a modernized payments API, enhanced type safety, and updated toolchain compatibility.


### 🚀 Rust Edition 2024 Upgrade

The SDK has been upgraded to **Rust Edition 2024**, requiring a minimum compiler version of **1.85**.

This change is mostly internal, developers will only be affected by the minimum Rust version increase.

Note: because there are still some issues with wasmer 6 on linux machines for compilers starting with 1.89, we are making sure to keep compatibility with older compilers for a while longer.



### 💰 Revolutionary Payments API


The new payments API introduces a unified approach to handling both EGLD and ESDT tokens, treating them equally for consistent and predictable behavior.

#### New Types

- `TokenId`: A more concise and ergonomic replacement for token identifiers with consistent EGLD.
    - EGLD is always serialized as `EGLD-000000`
    - Shorter, cleaner code
    - Unified handling across all token types

- `Payment`: The new payment type that combines token identifier and amount.
    - **Type Safety**: Uses `NonZeroBigUint` amounts to prevent zero-value payments at compile time
    - **Consistency**: Treats EGLD and ESDTs uniformly
    - **Ergonomics**: Cleaner API surface for common payment operations


#### Enhanced `call_value` API

- `all()` - Complete Payment Collection
```rust
// Get all payments sent with the transaction
let payments = self.call_value().all();
for payment in payments.iter() {
    // Handle each payment uniformly
    let token_id = payment.token_identifier;
    let amount = payment.amount;
}
```


- `single()` - Strict Single Payment
```rust
// Expect exactly one payment, crash otherwise
let payment = self.call_value().single();
// Safe to use - guaranteed to be exactly one payment
```


- `single_optional()` - Flexible Single Payment
```rust
// Handle zero or one payments gracefully
match self.call_value().single_optional() {
    Some(payment) => {
        // Process the payment
        self.process_payment(payment);
    },
    None => {
        // Handle no payment scenario
        self.handle_no_payment();
    }
}
```

- `array()` - Fixed-Size Payment Array
```rust
// Expect exactly N payments
let [payment1, payment2, payment3] = self.call_value().array();
// All three payments are guaranteed to exist
```


#### Advanced Features


### `MultiTransfer` Marker
Force ESDT multi-transfer even when not strictly required:

```rust
        let payment = self.call_value().all();
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .reject_funds()
            .payment(MultiTransfer(payment))
            .async_call_and_exit()
```


#### Deprecated APIs

Should no longer be used, since they rely on the assumption that EGLD cannot be transferred together with ESDTs, which is no longer valid, since Spica.

- `EgldOrMultiEsdtPayment` → Use `Payment`
- `call_value().any_payment()` → `call_value().all()`, or other appropriate methods.



### 🔢 Enhanced Numeric Types


#### `NonZeroBigUint` Implementation

A new refinement type that prevents zero-value operations at compile time. Designed with payments in mind, but can be used for anything in a smart contract.

The feature was heavily tested, using a large suite of auto-generated tests. These tests have also increased the security of `BigInt` and `BigUint` operators.

```rust
// Compile-time safety - cannot create zero values
let amount = NonZeroBigUint::new(BigUint::from(1000u64))?;

// All standard operators supported
let doubled = amount * 2u64;
let halved = amount / 2u64;
```

**Benefits:**
- **Compile-Time Safety**: Prevents zero-value payments and operations
- **Full Operator Support**: All mathematical operations available
- **Performance**: Some operations, such as addition of two non-zero numbers require no runtime checks.


### 🛠️ SDK Improvements


#### Transaction Parsing Fix
Resolved issues with parsing transactions containing large values in Smart Contract Results (SCRs):
- Better handling of big number serialization
- Improved transaction parsing reliability
- Enhanced compatibility with complex transaction scenarios


### 📦 Dependency Updates

All framework dependencies have been upgraded to their latest versions, providing:
- Security patches and bug fixes
- Performance improvements
- Enhanced compatibility with the broader Rust ecosystem

### 🎯 Benefits Summary

- **Enhanced Type Safety**: Compile-time prevention of zero-value payments
- **Unified Token Handling**: EGLD and ESDTs treated consistently
- **Improved Developer Experience**: Cleaner APIs and better error handling
- **Future-Ready**: Latest Rust features and dependency versions

This release represents a significant step forward in smart contract development ergonomics while maintaining the security and performance standards expected in blockchain development.
