# Interaction

The contract allows transfering token with the `ESDTRoleTransfer` role.

The owner can set a fee for the contract, being one of 2 possible types:

- `ExactValue` - `EsdtTokenPayment` type with desired token  + amount per token transfered
- `Percentage` - % of the transfered token (this number is multiplied by 100 so that we can have 2 decimal percentages. ex.: 12,50% percentage fee will be set with 1250)

The transfer endpoint requires the tokens having a `ExactValue` type fee to have the fee as the following token in exact amount.
The `Percentage` type will make the fee to be taken from the value transfered.

Tokens that have no fee set will be simply transfered without additional requirements.
