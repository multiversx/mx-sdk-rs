# NFT-Minter smart contract

## Introduction

The NFT-minter SC is meant to act as a template for NFT projects. This contract allows creating NFTs and immediately listing them for sale.  

## Initial setup

The setup is pretty simple, you need to:
- deploy the contract
- issue the NFT
- set the local roles

The init function has no arguments and does nothing, so the deployment is straight-forward. Just make sure you give it enough gas (around 10 million seems to be enough for the current version, but you might need to give it more, depending on how much more logic you decide to add).  

To issue the NFT (i.e. create the NFT "brand"), you need to use the `issueToken` endpoint:

```
#[endpoint(issueToken)]
fn issue_token(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer)
```

The endpoint needs a 0.05 EGLD payment for the token issue (note: if you use mxpy directly, you need to pass 0.05 * 10^18 as --value, i.e. 50000000000000000).  

`token_name` is the disply name that will be shown on the explorer in the user's balance, while `token_ticker` is the prefix of the token identifier that will be created. The token identifier is the token ticker + a dash (`-`) and 6 random hex characters. This is done to be able to differentiate between tokens with the same ticker. More information can be found here: https://docs.multiversx.com/tokens/esdt-tokens

Once the issue is complete, you have to set the NFTCreate local role, which can be done through the `setLocalRoles` endpoint. No arguments and no payment required.  

Keep in mind both issue and set roles are done through an async call, so it might take up to like 30s to 1 minute until the transaction is fully completed. Even if the explorer might show "success" initially, you have to wait until the transaction is fully processed to get the "real" status.  

## Creating NFTs

NFT creation is done through the `createNft` endpoint:

```
fn create_nft(
    &self,
    name: ManagedBuffer,
    royalties: BigUint,
    uri: ManagedBuffer,
    selling_price: BigUint,
    #[var_args] opt_token_used_as_payment: OptionalArg<TokenIdentifier>,
    #[var_args] opt_token_used_as_payment_nonce: OptionalArg<u64>,
)
```

`name` is the display name that will be shown on the explorer and on NFT marketplaces and such. This is not the same as the collection name, it's the NFTs name.  
`royalties` is a percentage of the selling price that will be taken as royalties from any sale on marketplaces. The percentage is represented as a BigUint with 2 decimals, between 0 (0%) and 10_000 (100%). For example, if you wanted 54.30% royalties, you would use 5_430 as value.  
`uri` is the URI for any external image/video/audio for the NFT. If your NFT has none, you can pass an empty argument here (i.e. `0x` in mxpy, or `@@` if you're using the webwallet and such).  
`selling_price` is the selling price for the NFT. Keep in mind you have to take decimals into account. So, for example, if you wanted to sell the NFT for 2 EGLD, you can't simply pass "2". You need to pass 2 * 10^18 (since EGLD has 18 decimals), which would be 2000000000000000000.  

Then, we have some optional arguments. First one is the token ID, which you only need to pass if you want to use a different token besides EGLD. The other one is a token nonce, which you only use if you want to receive a specific SFT/NFT as payment. Keep in mind you can NOT use an SFT token ID with nonce 0 to accept any nonce. This can be of course modified to work like that, but the current code-base will just reject the payment.  

## Buying NFTs

For users to buy the NFTs, they can use the `buyNft` endpoint:  

```
#[endpoint(buyNft)]
fn buy_nft(&self, nft_nonce: u64)
```

The only required argument is the nonce of the NFT the user is willing to buy, and the respective payment.  

## Claiming NFT marketplace royalties

If the NFT is sold on a marketplace at a later time by the buyer, the owner of the NFT-minter SC can claim the royalties through the `claimRoyaltiesFromMarketplace` endpoint:

```
#[endpoint(claimRoyaltiesFromMarketplace)]
fn claim_royalties_from_marketplace(
    &self,
    marketplace_address: ManagedAddress,
    token_id: TokenIdentifier,
    token_nonce: u64,
)
```

The arguments required are the marketplace's address, the token that was used to buy the NFT, and the token nonce. Unfortunately, there is currently no way to automatically claim all royalties without knowing the tokens that were used as payment.  

This is needed because the NFT-minter SC is the creator for the NFT, not the caller, so the royalties come to the SC.  

## Conclusion

We hope this makes NFT creation a bit easier. Keep in mind you do not need a smart contract to mint NFTs. You can also do it directly from the wallet, but this SC makes it easier to also sell those NFTs directly, also acting as a marketplace.  
