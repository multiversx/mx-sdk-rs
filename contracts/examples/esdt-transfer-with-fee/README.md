# Interaction

The contract allows transfering token with the `ESDTRoleTransfer` role.

The owner can set a fee for the contract, being one of 2 possible types:

- `ExactValue` - `EsdtTokenPayment` type with desired token  + amount per token transfered
- `Percentage` - % of the transfered token

The transfer endpoints calculated fees per token and afterwards takes that  amount out of the payments. 
By this matter of for example transfering `tokenA` and `tokenB`, `tokenA` has a fee of `tokenB` and `tokenB` also has a set fee, the fee will be calculated initially on the payments, not after extracting the fee for `tokenA` out of the amount of `tokenB`. 

Tokens that have no fee set will be simply transfered.
