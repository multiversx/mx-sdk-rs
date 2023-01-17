# NFTs owning NFTs

Currently MultiversX has a set of built in functions for NFTs: token manager, create, burn, update, mint. It implements transfer and execute, multi transfer. Smart contracts can own NFTs, they can create new NFTs, can update attributes, update URIs.

Equipping. Our goal is to be able to equip NFTs with other NFTs, enabling ownership, attribute updates and some extra features. This can be done through a new SC standard - we will find some name for it - for now let’s call it MergeNFT.  The basic contract should be as simple as possible, as it is a small standard. On top of that, developers and the market can create multiple usecases. So in the first place we will focus on building the MINIMUM VIABLE SMART CONTRACT to enable the NFTs owning NFTs feature.

Let’s think about the first usecase: user owns a character and buys a sword for it. He wants to equip that sword to the character and have a new image with both of the things combined. There is no game involved, it is only about 2 NFTs, each of them having one image, a set of attributes, and royalties.

The user makes a multiESDTNFTTransfer for those 2 NFTs to the new contract, the contract keeps the 2 NFTs and will send the user a new one, which contains some merged information on those 2. We do not need to duplicate the information of URIs, Attributes as those are already stored in the blockchain - we need to only create the smallest possible information through which every TOOL (marketplaces, indexer, xSpotLight, API, etc) will understand and can show the results of such merge. Furthermore, merging and selling a merged NFT should send the royalties back to the original owners - this is somewhat harder to do - but let’s try it - discussing a little bit later. If the merging of tokens is from the same artist there is no problem as royalties go to the same place. So in order to help creators we will check if the address of royalties for the selected NFTs is the same. To make it clear for users, we could even check that the address of royalties is the same as the owner of the SC.

## MergeNFTs:
Take the list of NFTs from the multiESDTNFTTransfer, check that all NFTs have the same address for royalties (this can be eliminated in some usecases). Create a new NFT with the following information:
Royalties - it has to be the maximum of all the NFTs merged.
Hash - nothing
Attributes - open ended - up to developer to do what he wants
URIs - list of [TokenID, nonce and value] as first URI

No need to keep any storage in the contract.

## UnMergeNFTs:
User comes with a merged NFT. The contract will check if the tokenID is the one the contract generated. The contract takes the list of NFTs from the URIs of the merged NFT, burns the NFT and sends back as multiTransferESDT the list of tokens from the attributes.

## Royalties:
This contract has to implement the claimRoyalties module - as will create NFTs and needs to claim royalties from marketplaces.
Distribute royalties: needs a new function to distribute the royalties to the original royalty owner.

In a sense of integration, we need to register these contracts - using the contract address, hash, tokenID in API in order to make sure the mergedNFTs will appear as they should be.

## NFTs owning SFTs:
As you can see from the above resolution, there is no limitation of NFTs owning actual SFTs. So your character could own 10 mana potions, 100 apples, or stuff like that - everything being encoded in the attributes.
