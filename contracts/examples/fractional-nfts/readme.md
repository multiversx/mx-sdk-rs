# Fractional Ownership
## SFTs owning NFTs

Following the ideas from NFTs owning NFTs, we can go into fractal ownership of NFTs as well. Let’s call the new module/contract fractalNFT. The NFT token creator deploys a new contract, through which ownership of NFTs can be fractionalized. 
We add the same protection as before - only those NFTs can be fractionalized, whose royalty address is the same as the one owning the fractalNFT contract or it is the fractalNFT contract. This ensures both parties - user and creator as well from misuse.

## fractionalizeNFT:
User comes with an NFT and says he wants to make an NFT with 1000 as value out of it. The contract checks the royalty address owner as stated above, keeps the NFT and creates a NEW SFT with the following things:
Royalties - the same as the NFT
Hash - nothing
Attributes - open ended
URIs - [TokenID, nonce, value, INITIAL_TOKEN_AMOUNT] as first URI.

Initial_Token_amount is in how many fractions is the NFT separated.

## unFractalNFT:
The user needs to come with the SFT and a VALUE to be equal with INITIAL_TOKEN_AMOUNT. In that case the SC will burn the fractionalized position and send back the actual NFT to the user.

In a sense of integration, we need to register these contracts - using the contract address, hash, tokenID in API in order to make sure the mergedNFTs will appear as they should be.

As you can see above - this fractalization can happen for SFTs as well as we save [TokenID, Nonce and Value]. So even SFTs can be fractionalized. Furthermore, merging and fractalization can be actually in the same contract. So mergedNFTs can be fractionalized as well.
