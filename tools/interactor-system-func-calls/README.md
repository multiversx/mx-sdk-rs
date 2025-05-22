# System SC function calls interactor

Fungible Tokens docs: https://docs.multiversx.com/tokens/fungible-tokens/
NFT/SFT/Meta-ESDT docs: https://docs.multiversx.com/tokens/nft-tokens/

### Functions:

- `issue_fungible_token`: Issues a fungible token, it registers the token and sends the initial supply to your wallet
- `issue_non_fungible_collection`: Issues an NFT Collection
- `issue_semi_fungible_collection`: Issues an SFT Collection
- `issue_token`: Registers any kind of token (Fungible/NFT/SFT/Meta-ESDT) and sets all the roles for it. This function doesn't transfer any tokens to your wallet
- `set_roles`: Sets the roles for your address over a specified token ID
- `mint_sft`: Mints an SFT/Meta-ESDT corresponding to a collection
- `register_meta_esdt`: Registers a Meta-ESDT token
- `change_sft_meta_esdt`: Changes an SFT to a Meta-ESDT
- `mint_token`: Mints a fungible token
- `burn_token`: Burns a token
- `pause_token`: Pauses all the transfers for a token
- `unpause_token`: Reverse function of `pause_token`
- `freeze_token`: Freezes a token for an address
- `unfreeze_token`: Reverse function of `freeze_token`
- `freeze_nft`: Freezes an NFT/SFT/Meta-ESDT token for an address
- `unfreeze_nft`: Reverse function of `freeze_nft`
- `wipe_token`: Wipes a token from an address
- `wipe_nft`: Wipes an NFT/SFT/Meta-ESDT token from an address
- `mint_nft`: Mints an NFT corresponding to a collection
- `unset roles`: Unsets the roles for an address over a specified tokenID
- `transfer_ownership`: Transfers the ownership of your token to another address
- `transfer_nft_create_role`: Transfers the NFT Create role to a new address
- `control_changes`: Sets/Unsets properties of a specified tokenID

### How to use tips

#### Token types for `issue_token` function

For CLI use, insert one of the numbers below for the `token-type` parameter

- 0 => `Fungible`
- 1 => `NonFungible`
- 2 => `SemiFungible`
- 3 => `Meta`
- any other number => `Invalid`

#### Set/Unset roles

Before trying to make any kind of interaction with a token (e.g. Mint, Burn, Transfer) you should set the necessary roles, even if you are the owner of the token. For CLI use, insert one or more numbers from below for the `roles` parameter, each corresponding to a role, each separated by one comma `e.g. --roles 1,2,8`

- 1 => `Mint`
- 2 => `Burn`
- 3 => `NftCreate`
- 4 => `NftAddQuantity`
- 5 => `NftBurn`
- 6 => `NftAddUri`
- 7 => `NftUpdateAttributes`
- 8 => `Transfer`
- any other number => `None`

#### Issue fungible token

- Method 1: `issue_fungible_token` (set the token properties inside the function)
- Method 2: `issue_token`-> `set_roles` (**Mint** role needed for minting the tokens) -> `mint_token` to receive the tokens in your wallet

#### Issue NFT Collection + Mint an NFT

- `issue_non_fungible_collection`/`issue_token` -> `set_roles` (**NFTCreate** role needed for minting) -> Mint an NFT by calling `mint_nft`

#### Issue SFT Collection

- Issue Collection: `issue_semi_fungible_collection`/`issue_token` -> `set_roles` (**NFTCreate** role needed for minting) -> Mint the amount of SFTs wantes by calling `mint_sft`

### Register a Meta-ESDT

- `register_meta_esdt` -> `set_roles`(**NFTCreate**) -> `mint_sft` to mint the Meta-ESDT

By using `set_roles`(**NFTAddQuantity**) -> `mint_token` you can add quantity to a Meta-ESDT/SFT with a specified nonce
