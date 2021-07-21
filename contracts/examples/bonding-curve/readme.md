# Bonding Curve Contract

This module enables using a bonding curve for defining the behaviour of the price of the token as its balance changes.

The module allows depositing any type of ESDT token. The supply together withn the function that gives the price will be stored together in an entity named `BondingCurve` structured as follows:
  - `FunctionSelector` - an enum that contains the functions available for setting
  - `CurveArguments` - containing:
    - available_supply
    - balance
  - `TokenIdentifier` - containing the accepted payment token
  - `Biguint` - containing the payment for the sold tokens
  
**Important!** Only 1 seller can have a specific token to be sold at a time.

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