This contract enables using a bonding curve for defining the behaviour of the price of the token as its balance changes.

The contract allows issuing of any ESDT token and together with its issue elements such as details about the supply will be stored together with the Balance in an entity called CurveArguments. Because of working with different types of ESDT, the entity under which we will make the mapping with the curve function will be called Token, containing the TokenIdentifier and the nonce.

The behaviour however is differend depending on the issued token:
    -FT:
		* defines one bonding curve
		* the nonce from Token should be set 0
		* the supply and balance are indicated by the amount minted
    - SFT:
		* defines multiple bonding curves (one per each nonce)
		* the supply and balance are indicated by the amount of each nonce
    - NFT:
		* defines one bonding curve
		* the nonce from Token should be set 0
		* the supply and balance are indicated by the number of nonces

The bonding curve function configurations are set in function_selector.rs
Here is where you would like to set your custom functions if the predefined ones are not what you are looking for

When using this contract one should do the following process for each issued token:
	- issue the token
    - mint the token
	- set the curve function
