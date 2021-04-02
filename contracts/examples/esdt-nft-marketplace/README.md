# ESDT NFT Marketplace

## Introduction

Whether you've been familiar with the world of cryptocurrency and blockchains for a while or you're just starting out, you've most likely heard of NFTs (which stands for Non Fungible Token). To put it simply, an NFT is used to represent a unique item, by providing something known as "digital scarcity". NFTs could be used to represent collectibles, contracts, and much more.  

As we all know, there really isn't such thing as "unique data" when it comes to computers. They copy and share data all the time. The key difference here is the immutability of data on the blockchain: Once an NFT is created, you will be able to trace its history through time, without having to worry about a third party modifying, or even deleting that said NFT.  

But what if someone wants to exchange said collectible item for something else? That's where the NFT marketplace comes in.  

## Difference between ESDT NFTs and ERC1155 NFTs

Before we start with the marketplace, there's something else that needs to be covered. If you're familiar with Ethereum, you probably heard about the ERC1155 standard for NFTs. There are a few problems with that standard.  

The main problem is that it is very centralized. A single smart contract holds the data for all NFTs, and all the related operations, like transfers, are also done through that contract. This can lead to trust issues or bottlenecks.  

Another problem is that using said NFTs as payment for smart contracts is very difficult. It requires a call to the main ERC1155 contract, which then calls the receiver smart contract, which has to implement a very specific method for receiving the token. As you can probably guess, this adds a lot of overhead and costs a lot of gas. And now the receiver smart contract owner has another problem: how to get that NFT to its own user account? This requires yet another call to the ERC1155 contract, through the receiver contract, which is another two SC calls. More overhead, more gas.  

By implementing NFTs on the protocol level, we've solved those problems by treating NFTs as ESDTs with additional metadata. This means your user account can *own* the NFT natively instead of relying on an external contract. You can also transfer that NFT at will, and even use it as payment for smart contracts. And what's more, said contract doesn't even need to implement some standardized interface!  

An ESDT NFT has a token identifier (like all the rest), which is also referred to as its "type" or "class". Then, a type can have "subtypes" or "subclasses", differentiated by `nonce`, `name`, and additional user-defined `attributes`.  

Now that we have introduced NFT, it is time to see how they can be sold on the marketplace.  

## The Marketplace

The marketplace has been designed to be as simple as possible. All it does is accept an NFT and auction it with your terms. It accepts bids from other accounts, and finishes the auction by either sending the token to the winner, or back to the owner if no bids were received.  

## Royalties

Royalties represent a specific percentage of currency the original creator of the NFT receives after each selling. The marketplace itself also takes its own percentage of royalties for hosting the auction.  

## Starting an auction

First, you have to transfer the NFT to the marketplace, by calling the `auctionToken` endpoint, described below:

```
#[endpoint(auctionToken)]
fn auction_token(
	&self,
	min_bid: BigUint,
	max_bid: BigUint,
	deadline: u64,
	accepted_payment_token: TokenIdentifier,
	#[var_args] opt_accepted_payment_token_nonce: OptionalArg<u64>,
)
```

Arguments are about what you would expect for an marketplace: 
`minimum bid` - lowest amount someone can bid.  
`maximum bid` - maximum bid. If this is reached, auction can be ended before the deadline.  
`deadline` - the deadline for the auction, expressed as a unix timestamp.  
`accepted_payment_token` - The token you wish to receive as payment. For eGLD, input `EGLD`.  
`opt_accepted_payment_token_nonce` - "nonce" (also known as "id") for the ESDT token. For usual ESDTs (not NFTs), this is 0 and can be skipped.  

To perform the transfer from your account to the smart contract, you have to use the following transaction format:

```
TransferTransaction {
    Sender: <account address of the sender>
    Receiver: <same as sender>
    Value: 0
    GasLimit: 500000 + extra for smart contract call
    Data: "ESDTNFTTransfer" +
          "@" + <token identifier in hexadecimal encoding> +
          "@" + <the nonce after the NFT creation in hexadecimal encoding> + 
          "@" + <quantity to transfer in hexadecimal encoding> +
          "@" + <destination address in hexadecimal encoding> + 
          "@" + <name of method to call in hexadecimal encoding> +
          "@" + <first argument of the method in hexadecimal encoding> +
          "@" + <second argument of the method in hexadecimal encoding> + 
          <...>
}
```

Your work as the seller is now done! It's time for others to start bidding.  

## Bidding

To bid on an auctioned token, you call the `bid` endpoint:  

```
#[endpoint]
fn bid(&self, nft_type: TokenIdentifier, nft_nonce: u64)
```

Pretty straight forward, the `nft_type` is the "class" of tokens you want to bid on, and the `nft_nonce` is the specific nft id.  

If the bid is valid (higher than the previous), the previous bid (if any) is cancelled and the payment tokens are sent back to the previous bidder.  

## Ending an auction

Once the deadline has passed or the maximum bid has been made, the auction can be ended by calling the `endAuction` endpoint: 

```
#[endpoint(endAuction)]
fn end_auction(&self, nft_type: TokenIdentifier, nft_nonce: u64)
```

Arguments are the same as the `bid` ones. If no bids were made, the NFT is returned to the owner. If bids were made, the NFT is sent to the highest bidder, and the bid is split between the NFT creator, marketplace SC and NFT owner.  

## Conclusion

NFTs are the future of blockchain, and can be used both for entertainment purposes (like cryptokitties, for example) and for serious purposes (like ownership of art, music, etc.). In both cases, the marketplace can be used to sell NFTs as easily as possible.  
