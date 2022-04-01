# Abstract

The cryptokitties contracts suite is meant to show a relatively simple example of NFT (non-fungible tokens) usage. In these contracts, we use "kitties" as NFTs. Each of them has certain properties that makes them unique.

The implementation is split into three contracts: 
- kitty-ownership - responsible for keeping track of kitties, their characteristics and their owners, and it also handles the breeding logic
- kitty-genetic-alg - mock genetic algorithm, which can be further extended if needed
- kitty-auction - lets user auction their kitties, either outright selling them or auctioning their right to be used as a sire for breeding

Their role will be detailed in their own section.  

# Kitty

A *Kitty* is represented by the following struct:

```
struct Kitty {
	genes: KittyGenes,
	birth_time: u64,
	cooldown_end: u64,
	matron_id: u32,
	sire_id: u32,
	siring_with_id: u32,
	nr_children: u16,
	generation: u16,
}
```

*Kitty.genes* represents the genes of a kitty. For the sake of the example, we kept them as simple as possible. No actual "genes" are used, instead we used visual properties. The struct name might be a bit misleading, but keep in mind that these can easily be extended to contain actual genes and perform real genetic mutations and such. The current implementation of *KittyGenes* is as follows:

```
struct KittyGenes {
	fur_color: Color,
	eye_color: Color,
	meow_power: u8, // the higher the value, the louder the cat
}
```

*Color* is just an RGB color.

```
struct Color {
	r: u8,
	g: u8,
	b: u8,
}
```

*Kitty.birth_time* is a timestamp, representing the birth time of the kitty. In the current implementation, it's taken from the current block timestamp.

*Kitty.cooldown_end* is a timestamp, representing the cooldown end time for pregnancy (matron kitty) and siring cooldown (sire kitty). The cooldown time increases with each breeding/siring. Generation also has a negative effect on the cooldown period, making it so earlier generation kitties are more valuable. The formula for cooldown period is 2<sup>nr_children + generation</sup> minutes. In the current implementation, *Kitty.cooldown_end* is created as the sum of the current block timestamp and cooldown period.

*Kitty.matron_id* and *Kitty.sire_id* are the ids of the parents.

*Kitty.siring_with_id* is used for pregnant kitties to keep track of the sire's id. This value is set to 0 otherwise.

*Kitty.nr_children* keeps track of the number of children, but is mostly used to calculate the new cooldown period.

*Kitty.generation* is the generation of the kitty. The generation of a new kitty is `max(sire.generation, matron.generation) + 1`.

# Kitty Genetic Alg Contract

This is the contract that handles the new genes generation for newborn kitties. Due to the simple *KittyGenes* we're using, we can also implement a really simple "genetic" algorithm.

The init function requires no arguments and does nothing, so we'll skip it this time.

The contract has only one function:

```
#[endpoint(generateKittyGenes)]
fn generate_kitty_genes(matron: Kitty, sire: Kitty) -> KittyGenes
```

It takes two kitties, the matron and the sire, and generates a *KittyGenes* instance. In the current implementation, this simply combines their `fur_color` and `eye_color` using a random percentage of each, and the average of their `meow_power`s.

Of course, this can be made as simple or as complex as wanted. You could even implement a *real* genetic algorithm for this.

# Kitty Auction Contract

This is the contract that handles kitty auctioning. The are two types of auctions supported:
- selling auction: Once the auction is complete, the kitty is sold to the winner
- siring auction: The winner of the auction gets the right to use the kitty as a sire for one of their matron kitties

An auction is described by the following struct:

```
struct Auction {
	auction_type: AuctionType,
	starting_price: BigUint,
	ending_price: BigUint,
	deadline: u64,
	kitty_owner: Address,
	current_bid: BigUint,
	current_winner: Address,
}
```

`AuctionType` is a simple enum

```
enum AuctionType {
	Selling,
	Siring,
}
```

## Deployment

The init function is as follows:

```
fn init(
	gen_zero_kitty_starting_price: BigUint,
	gen_zero_kitty_ending_price: BigUint,
	gen_zero_kitty_auction_duration: u64,
	#[var_args] opt_kitty_ownership_contract_address: OptionalArg<Address>,
)
```

The first 3 arguments are used as parameters for the auction whenever a gen zero kitty is created. A gen zero kitty is a kitty that's created out of "thin air" by the contract and is auctioned immediately. Gen zero kitties are very valuable, both because of their rarity (they're supposed to be created not very often at all) and because of their low cooldown period (at least initially).

The last argument is the address of the `Kitty Ownership` contract, which can either be set now or later by the owner using the appropriate setter method.

## Owner-only

The following throw an error if the caller isn't the contract owner.

`#[endpoint(setKittyOwnershipContractAddress)]`  
`fn set_kitty_ownership_contract_address_endpoint(address: Address)`

This simply sets the `Kitty-Ownership` contract address.

`#[endpoint(createAndAuctionGenZeroKitty)]`  
`fn create_and_auction_gen_zero_kitty()`

This function creates a new gen zero kitty (generating random genes) and puts it up for auction, using the parameters described above.

`fn claim()`

Owner claims funds.

## Views

`#[view(isUpForAuction)]`  
`fn is_up_for_auction(kitty_id: u32) -> bool`

Returns `true` if the kitty is up for auction, `false` otherwise.

`#[view(getAuctionStatus)]`  
`fn get_auction_status(kitty_id: u32) -> Auction`

Returns the relevant `Auction` struct (described above) if it exists, throws an error otherwise.

`#[view(getCurrentWinningBid)]`  
`fn get_current_winning_bid(kitty_id: u32) -> BigUint`

Returns the current winning bid for a kitty's auction if it exists, throws an error otherwise. Cheaper version of `get_auction_status` is you're only interested in the winning bid.

## Endpoints

```
#[endpoint(createSaleAuction)]
fn create_sale_auction(
	kitty_id: u32,
	starting_price: BigUint,
	ending_price: BigUint,
	duration: u64,
)
```

Puts the kitty up for a sale auction. Only the owner of the kitty may call this function,

```
#[endpoint(createSiringAuction)]
fn create_siring_auction(
	kitty_id: u32,
	starting_price: BigUint,
	ending_price: BigUint,
	duration: u64,
)
```

Puts the kitty up for siring auction. Only the owner of the kitty may call this function.

`fn bid(kitty_id: u32)`

Payable function. Bid must abide the following rule:
`current_bid < payment <= ending_price`

If this is the first bid in the auction, the rule changes to:
`starting_price <= payment <= ending_price`

If the bid is valid, the `current_bid` is sent back to the `current_winner`, `current_bid` is set to `payment`, and `current_winner` is set to the caller's address.

`#[endpoint(endAuction)]`  
`fn end_auction(kitty_id: u32)`

This function ends the auction for the kitty if one of the end conditions has been met:
1) `deadline` has passed
2) The `current_bid` is equal to `ending_price`

Anyone may call this function, not only the ones involved in the auction.

If this was a selling auction, the `current_bid` is sent to the `kitty_owner` and the kitty's ownership is transfered to `current_winner`. Auction is then cleared from storage.

If this was a siring auction, the `current_bid` is sent to the `kitty_owner` and the kitty's `sire_allowed_address` is set to `current_winner`.

# Kitty Ownership Contract

This is the main contract. It keeps track of all the existing kitties, their owners, their properties, and it also handles breeding logic. 

## ERC721 functions

Kitty Ownership also implements the ERC721 interface. We won't be going over these functions though, as their implementation (not counting validity checks) is a one-liner in most cases, but we'll still list them here for reference.

`fn total_supply() -> u32`  
`fn balance_of(address: Address) -> u32`  
`fn owner_of(kitty_id: u32) -> Address`  
`fn approve(to: Address, kitty_id: u32) `  
`fn transfer(to: Address, kitty_id: u32) `  
`fn transfer_from(from: Address, to: Address, kitty_id: u32) `  
`fn tokens_of_owner(address: Address) -> Vec<u32>`  

## Deployment

The *init* method is as follows:

```
fn init(
	birth_fee: BigUint,
	#[var_args] opt_gene_science_contract_address: OptionalArg<Address>,
	#[var_args] opt_kitty_auction_contract_address: OptionalArg<Address>,
)
```

Each breeding will cost a fixed amount of eGLD. The birth operation can cost a lot of gas, depending on the implementation of the genetic algortihm, so whoever calls the give_birth method (will be discussed later) will get the deposited `birth_fee`.

The next two arguments are the addresses of the other two contracts: the kitty-auction contract and the kitty-genetic-alg contract. These can either be set now or later by the owner, using the appropriate setter methods.

## Views

The following throw an error if the kitty does not exist.

`#[view(getKittyById)]`  
`fn get_kitty_by_id_endpoint(kitty_id: u32) -> Kitty`

Gets a kitty by id.

`#[view(isReadyToBreed)]`  
`fn is_ready_to_breed(kitty_id: u32) -> bool`

Checks if the kitty is ready to breed by checking if `siring_with_id` is 0 and cooldown period has passed.

`#[view(isPregnant)]`  
`fn is_pregnant(kitty_id: u32) -> bool`

Checks if the kitty is pregnant by checking if `siring_with_id` is not 0.

`#[view(canBreedWith)]`  
`fn can_breed_with(matron_id: u32, sire_id: u32) -> bool`

Checks if the matron can breed with the sire. Kitties can't breed with themselves, their parents, nor their siblings/half-siblings.

#[view(birthFee)]
fn get_birth_fee() -> BigUint;

Gets the `birth_fee` set by the owner.

## Endpoints

`#[endpoint(approveSiring)]`  
`fn approve_siring(&self, address: Address, kitty_id: u32) `

Approves an address to use the kitty as a sire. Only the owner of `kitty_id` may call this function, and it may not override an already existing approved address.

```
#[payable("EGLD")]
#[endpoint(breedWith)]
fn breed_with(
	matron_id: u32,
	sire_id: u32,
)
```

Breeds `matron_id` with `sire_id`. The `payment` must be equal to the `birth_fee` set by the contract owner. Only the owner of the `matron` may call this function, and the `sire` must either be owned the by the same account that owns the `matron` OR the caller must be the `sire_allowed_address` for the sire.

If the call is successful, the `cooldown` period is triggered and the `sire_allowed_address` is reset for both kitties.

`#[endpoint(giveBirth)]`  
`fn give_birth(matron_id: u32)`

If the kitty is pregant and the gestation period has passed, a new kitty is created by the `Kitty Genetic Alg` contract and its ownership is given to the `matron`'s owner. Anyone may call this function.

# General Flow

Everything starts with the gen zero kitties randomly created and auctioned by the owner, using the `Kitty-Auction` contract. 

Once there is a sufficient pool of kitties available, people can start breeding and creating more and more kitties, each with their own unique properties, by using the `Kitty-Ownership` contract, or they can sell their kitties by using the `Kitty-Auction` contract.

The `Kitty-Genetic-Alg` contract is never meant to be called directly. People can still do it for fun if they want to see different combinations, but they must keep in mind that depending on its complexity, it can become quite gas-extensive to make such calls.

# Conclusion

Crpytokitties aims to show the power of NFTs by also making it easier to understand for people not familiar with the technology, and we hope this example gives birth to a whole lot of community-driven projects!
