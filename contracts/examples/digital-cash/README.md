# Digital Cash Contract

The basic idea of MultiversX Digital Cash is that a cryptographic check (represented by a blockchain address) can hold tokens (ESDT or EGLD) on-chain. This check can be transferred from one person to another through a simple link. The recipient doesn't need a wallet initially - they just need to prove ownership of the check's private key through an ED25519 signature to claim the funds.

## Overview

Each deposit is stored at a specific address (the check address). To claim funds:
1. The recipient must provide a valid ED25519 signature proving they control the check address's private key
2. The deposit must not have expired
3. Fees must have been paid upfront by the depositor

## Fee System

The contract owner whitelists tokens that can be used to pay fees. Each whitelisted token has a configurable per-token-transfer fee. Fees must be paid upfront when creating a deposit and cover the cost of transferring all deposited tokens.

## Creating Deposits

### Option 1: `payFeeAndFund` (One-Step)
The primary way to create a deposit. This endpoint accepts multiple payments where:
- **First payment**: Contains the fee (and optionally funds as well)
  - Can be exactly the fee amount for the remaining tokens
  - Can exceed the fee to include both fee and funds in one payment
- **Remaining payments**: All considered as funds to deposit

Parameters:
- `address`: The check address where funds will be stored
- `expiration`: Timestamp in milliseconds when the deposit expires

### Option 2: `depositFees` + `fund` (Two-Step)
Alternative approach for more complex scenarios:
1. First call `depositFees` to pay the fee in a whitelisted token
2. Then call `fund` to deposit the actual tokens (must be same depositor)

The `fund` endpoint verifies that sufficient fees have been paid to cover the number of tokens being deposited.

## Claiming Funds

Use the `claim` endpoint with:
- `address`: The check address containing the deposit
- `signature`: ED25519 signature proving ownership of the check's private key

The signature is verified against the check address and the caller's address. If valid and the deposit hasn't expired:
- All deposited funds are transferred to the caller
- Fees for the transfer are deducted and collected by the contract
- Remaining unused fees are returned to the original depositor

**Important**: The deposit must not be expired (current timestamp must be before the expiration timestamp).

## Withdrawing Funds

If a deposit has expired and hasn't been claimed, the original depositor can reclaim everything by calling `withdraw`:
- `address`: The check address containing the expired deposit

This returns all deposited funds plus all unused fees to the original depositor.

## Forwarding Funds

Funds can be forwarded to another check address using the `forward` endpoint:
- `address`: The current check address (source)
- `forward_address`: The new check address (destination)
- `signature`: ED25519 signature proving ownership of the source check

The destination address must already have fees deposited. The forward operation:
- Consumes fees from the current deposit
- Moves funds to the forwarded address
- Returns any remaining fees to the original depositor

After forwarding, if the new deposit expires and is withdrawn, funds go to the depositor address recorded in the forwarded deposit.
