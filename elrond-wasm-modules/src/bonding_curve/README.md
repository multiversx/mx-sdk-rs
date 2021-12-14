# Bonding Curve Contract

This module enables using a bonding curve for defining the behaviour of the price of the token as its balance changes.

Depositing a token will set 2 storages:
  - `owned_tokens` which will store a list of `TokenIdentifier` of the tokens the seller deposits in the contract under the seller address as key and its pair
  - `token_details` which will storea `TokenOwnershipData` object containing a list of the stored nonces and the address of the owner.
  
This approach with 2 storages was importand because as seller I am interested only of what tokens I have available and for claiming everything they should be easy to find without me requiring to give arguments of what token I want to claim. From buyer point of view I am not interested of who is the owner, but at the same time the contract needs to make sure the payment goes to the right address. 

The payment was chosen to be stored under a storage named `BondingCurve` because here we have elements such as:
  - `FunctionSelector` - an enum that contains the functions available for setting
  - `CurveArguments` - containing:
    - available_supply
    - balance
  - `TokenIdentifier` - containing the accepted payment token
  - `Biguint` - containing the payment for the sold tokens
  
Here the balance and the payment amount are variable and they will usually get changed together, reason why it was chosen for these elements to be kept away from `token_details`.

**Important!** Only 1 seller can have a specific token to be sold at a time, avoiding this way scenarion of which token from which seller should be selled at one point.

There is an option of `sell_availability` which can be set from the `init` of the contract allowing or denying a token once bought by a buyer to be sold back.

The token availability can be checked via `getTokenAvailability` returning pairs of (`nonce`, `amount`) of the requested token.

The buying function is constructed so that through the amount the buyer will receive the amount of tokens irrelevant of the nonces. If the buyer desires a specific token, he can provide in the optional parameter `requested_nonce` the nonce he desires and if it is available under the specified amount he will receive it.

# Usage

When using this contract one should do the following process for each deposited token:
  - deposit the token
  - set the curve function
  - claim (when he wants to receive the payment and the unsold tokens)

# Setting up the Curve Function

For setting up the curve function the seller is requires to use the `setBondingCurve` endpoint providing a function for the seposited token.

The bonding curve function configurations are set in the [function selector](docs/selector.md)
Here is where you would like to set your custom functions if the predefined ones are not what you are looking for.

In the case where the curve function is not set, `FunctionSelector::None` will be the value of it, throwing an error until a proper function is set.