# Abstract

The lottery smart contract is designed to allow anyone to create their very own lottery, directly on the blockchain. Having said that, the purpose of this contract is just to have a bit of fun and show what’s possible on the current version of the MultiversX blockchain. We do not endorse gambling. 

This is the esdt version, which allows any arbitrary token to be used as currency.

Now that that’s out of the way, there’s not much else to say in this section. It’s just a lottery! You buy tickets and hope to win. The difference between this and the traditional lottery is that you don’t pick some numbers, you just buy a ticket and at the end, one (more winning tickets not supported as of yet) of the tickets is declared the “winning ticket”.  

# Available actions

There is no difference in rights between users. Everyone is equal in the eyes of the lottery SC.

Deployment of the SC requires no arguments. All you need to do is perform the scDeploy transaction.

# Actions after deploy

Once the SC has been deployed, anyone can start a lottery, using the following function: 

```
start(lottery_name: Vec<u8>,
        token_identifier: Vec<u8>,
        ticket_price: BigUint, 
        opt_total_tickets: Option<u32>, 
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<u32>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<Address>>)
```

You may also call its twin function: *createLotteryPool*. There is no difference in the implementation.

It is also worth noting that the smart contract has a one-to-many relationship with lotteries. A single smart contract can run multiple lotteries at the same time, even started by the same user.

The function requires the following arguments: (Note: Optional arguments still have to be passed.  
        If you want to skip an argument, pass 0x as value.  
        If you want to use that argument, pass 0x01, followed by the value - both concatenated.  
        Example: Option&lt;u32&gt; want to pass 4 as value, pass 0x0100000004)  

- lottery_name: Each lottery has to have a unique, case-sensitive name, using ASCII characters only.
- token_identifier: The identifier of the esdt token that will be used as currency for this lottery.
- ticket_price: The price of the ticket, currency is the esdt token set above.
- total_tickets (Optional): The total available tickets for the lottery. If they're sold out, the lottery can be ended. Default is "unlimited"
- deadline (Optional): The deadline for the lottery, expressed as a timestamp. The default and the maximum is 30 days in the future.
- max_entries_per_user (Optional): The max number of tickets each user can buy. The default is unlimited.
- prize_distribution (Optional): Not supported in the current version. In the future, you will be able to split the prize pool. Current version only supports one winner per lottery.
- whitelist (Optional): If provided, only the addresses on the list can participate in this lottery.

# Actions after lottery start

## 1) Query functions

Once a lottery has started, people can start buying tickets. Generally, anyone can buy tickets to any lottery, unless the lottery has been configured with a whitelist, which restricts the pool of people who can participate.

Currently, there is no way to query the contract about which lotteries are active. You will have to be announced by whoever starts the lottery. This is done so it’s a bit easier to have a semi-private lottery. Instead of having to add many accounts to the whitelist, you just keep the name private and share it only with the people you want to know about it.

However, there is one query function you can use to verify if a lottery is active.

```
fn status(lottery_name: Vec<u8>)
```

The function will return:  
0 -> Inactive (doesn't exist)  
1 -> Running (is currently selling tickets)  
2 -> Ended (is waiting for someone to call the determine_winner function)  

Alternatively, you may also query the following function, for a similar result:  

```
#[view(lotteryExists)]
#[storage_get("lotteryExists")]
fn get_lottery_exists(lottery_name: &Vec<u8>) -> bool
```
  
Although there is not much reason to call this function instead of the previous one (except maybe very slightly lower gas consumption), you have the option if you wish to do so.

The functions described above only give very basic information about a lottery. If you want more detailed information, you have to call the following function:

```
#[view(lotteryInfo)]
#[storage_get_mut("lotteryInfo")]
fn get_mut_lottery_info(lottery_name: &Vec<u8>) -> mut_storage!(LotteryInfo<Self::Api>)
```

*LotteryInfo* is a struct, with its definition as follows:

```
pub struct LotteryInfo<BigUint:BigUintApi> {
    pub token_identifier: Vec<u8>
    pub ticket_price: BigUint,
    pub tickets_left: u32,
    pub deadline: u64,
    pub max_entries_per_user: u32,
    pub prize_distribution: Vec<u8>,
    pub whitelist: Vec<Address>,
    pub current_ticket_number: u32,
    pub prize_pool: BigUint
}
```

It basically contains the exact same fields as the arguments of the start function, plus an additional two fields:

*current\_ticket\_number*: Could be called “tickets sold”, but this name makes more sense in the implementation.  
*prize\_pool*: the accumulated esdt tokens up until now.

## 2) State-altering functions

Up until now, we’ve only looked at functions that allow you to get information about the contract. But how do you actually interact with the contract?

Using the following function, you may buy a ticket for one of the available lotteries:

```
fn buy_ticket(lottery_name: Vec<u8>)
```

All you need to do is pass along the name of the lottery you wish to purchase the ticket for and deposit the appropriate sum of the specific esdt tokens, corresponding to the ticket cost.

Don’t know the ticket cost? Simply ask the lottery creator, or use the query function described in part 1.

Once all tickets have been sold out or deadline has passed, anyone may call the following function to trigger the distribution of prizes:

```
fn determine_winner(lottery_name: Vec<u8>)
```

When called, if at least one of the end conditions (tickets sold out or deadline passed) has been fulfilled, the lottery will randomly pick the winning tickets and distribute the prizes, according to the *prize\_distribution* set up at the start.

The storage is then cleaned, and another lottery with the same name may be started at any point in the future.
