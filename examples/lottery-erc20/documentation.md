# Abstract

This is a lottery smart contract designed to be used with the erc20 token.

The lottery smart contract is designed to allow anyone to create their very own lottery, directly on the blockchain. Having said that, the purpose of this contract is just to have a bit of fun and show what’s possible on the current version of the Elrond blockchain. We do not endorse gambling and this should never really be used with high sums of erc20 tokens.

Now that that’s out of the way, there’s not much else to say in this section. It’s just a lottery! You buy tickets and hope to win. The difference between this and the traditional lottery is that you don’t pick some numbers, you just buy a ticket and at the end, one (or more) of the tickets are declared the “winning tickets”.  

# Available actions

There is no difference in rights between users. Everyone is equal in the eyes of the lottery SC.

Deployment of the lottery SC requires the addresss of the erc20 SC as argument.

# Actions after deploy

Once the SC has been deployed, anyone can start a lottery, using the following function: 

*start(lottery\_name: Vec&lt;u8&gt;,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*ticket\_price: BigUint,*   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*opt\_total\_tickets: Option&lt;u32&gt;,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*opt\_deadline: Option&lt;u64&gt;,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*opt\_max\_entries\_per\_user: Option&lt;u32&gt;,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*opt\_prize\_distribution: Option&lt;Vec&lt;u8&gt;&gt;,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*opt\_whitelist: Option&lt;Vec&lt;Address&gt;&gt;)*  

You may also call its twin function: *createLotteryPool*. There is no difference in the implementation.

It is also worth noting that the smart contract has a one-to-many relationship with lotteries. A single smart contract can run multiple lotteries at the same time, even started by the same user.

The function requires the following arguments: (Note: Optional arguments still have to be passed.  
        If you want to skip an argument, pass 0x as value.  
        If you want to use that argument, pass 0x01, followed by the value - both concatenated.  
        Example: Option&lt;u32&gt; want to pass 4 as value, pass 0x0100000004)  

- lottery_name: Each lottery has to have a unique, case-sensitive name, using ASCII characters only.
- ticket_price: The price of the ticket in erc20 tokens.  
- total_tickets (Optional): The total available tickets for the lottery. If they're sold out, the lottery can be ended. Default is "unlimited"
- deadline (Optional): The deadline for the lottery, expressed as a timestamp. The default and the maximum is 30 days in the future.
- max_entries_per_user (Optional): The max number of tickets each user can buy. The default is unlimited.
- prize_distribution (Optional): An array of unsigned integer values that add up to exactly 100 (%). Example: [75, 15, 10]
    [0] -> 1st ticket winner prize (75% of total pool)
    [1] -> 2nd ticket winner prize (15% of total pool)
    [2] -> 3rd ticket winner prize (10% of total pool)
- whitelist (Optional): If provided, only the addresses on the list can participate in this lottery.

# Actions after lottery start

## 1) Query functions

Once a lottery has started, people can start buying tickets. Generally, anyone can buy tickets to any lottery, unless the lottery has been configured with a whitelist, which restricts the pool of people who can participate.

Currently, there is no way to query the contract about which lotteries are active. You will have to be announced by whoever starts the lottery. This is done so it’s a bit easier to have a semi-private lottery. Instead of having to add many accounts to the whitelist, you just keep the name private and share it only with the people you want to know about it.

However, there is one query function you can use to verify if a lottery is active.

*fn status(lottery\_name: Vec&lt;u8&gt;)*

The function will return:  
0 -> Inactive (doesn't exist)  
1 -> Running (is currently selling tickets)  
2 -> Ended (is waiting for someone to call the determine_winner function)  
3 -> DistributingPrizes (prizes are currently being distributed. Lottery will then go into "Inactive" state)  

Alternatively, you may also query the following function, for a similar result:  
  
*\#\[view(lotteryExists)\]*  
*\#\[storage\_get("lotteryExists")\]*  
*fn get\_lottery\_exists(lottery\_name: &Vec&lt;u8&gt;) -> bool*  
  
Although there is not much reason to call this function instead of the previous one (except maybe very slightly lower gas consumption), you have the option if you wish to do so.

The functions described above only give very basic information about a lottery. If you want more detailed information, you have to call the following function:

*\#\[view(lotteryInfo)\]*  
*\#\[storage\_get\_mut("lotteryInfo")\]*  
*fn get\_mut\_lottery\_info(lottery\_name: &Vec&lt;u8&gt;) -> mut\_storage!(LotteryInfo&lt;BigUint&gt;)*


*LotteryInfo* is a struct, with its definition as follows:

*pub struct LotteryInfo&lt;BigUint:BigUintApi&gt; {*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub ticket\_price: BigUint,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub tickets\_left: u32,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub deadline: u64,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub max\_entries\_per\_user: u32,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub prize\_distribution: Vec&lt;u8&gt;,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub whitelist: Vec&lt;Address&gt;,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub current\_ticket\_number: u32,*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub prize\_pool: BigUint*  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*pub queued\_tickets: u32*  
*}*

It basically contains the exact same fields as the arguments of the start function, plus an additional two fields:

*current\_ticket\_number*: Could be called “tickets sold”, but this name makes more sense in the implementation.  
*prize\_pool*: the accumulated eGLD up until now.
*queued\_tickets*: the number of tickets that are currently being processed

## 2) State-altering functions

Up until now, we’ve only looked at functions that allow you to get information about the contract. But how do you actually interact with the contract?

Using the following function, you may buy a ticket for one of the available lotteries:

*fn buy\_ticket(lottery\_name: Vec&lt;u8&gt;, payment: BigUint)*  

All you need to do is pass along the name of the lottery you wish to purchase the ticket for and the appropriate sum of erc20 tokens, corresponding to the ticket cost.

Don’t know the ticket cost? Simply ask the lottery creator, or use the query function described in part 1.

Once all tickets have been sold out or deadline has passed, anyone may call the following function to trigger the distribution of prizes:

*fn determine\_winner(lottery\_name: Vec&lt;u8&gt;)*  

When called, if at least one of the end conditions (tickets sold out or deadline passed) has been fulfilled, the lottery will randomly pick the winning tickets and distribute the prizes, according to the *prize\_distribution* set up at the start.

The storage is then cleaned, and another lottery with the same name may be started at any point in the future.
