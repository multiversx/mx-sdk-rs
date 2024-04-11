# Function Selector 

The `FunctionSelector` stores the pre-defined and user defined functions and needs to be added to the contract together with the following endpoints and views:
	- buyToken
	- claim
	- deposit
	- getTokenAvailability
	- sellToken
	- setBondingCurve
	- setLocalRoles
	- unsetLocalRoles
	- view_buy_price
	- view_sell_price

 This entity is passed as a generic to the module reason why some of the endpoints and views will also need to be defined in the contract calling their defined counterpart from the module with `FunctionSelector` as a generic.

An example of predefined curve function is [Linear](linear.md).

When setting the bonding curve by a predefined function one mush pay attention by the parameters requested by the certain function. All the predefined functions are available in the curves folder and are implementing the `CurveFunction` trait.

Custom functions can be defined by adding the name of it in `FunctionSelector`, followed by defining the behaviour in the implementation of `CurveFunction`, in the `match` contained by the `calculate_price` function.

```rust
pub enum FunctionSelector<Self::Api>
{
	Linear(LinearFunction<M>),
	CustomExample(BigUint<M>),
	None,
}
```
