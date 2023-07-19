# Multisig Smart Contract (MSC)

## Abstract
Cryptocurrencies can be one of the safest ways to store and manage wealth and value. By safeguarding a short list of words, the so-called seed or recovery phrase, anyone can protect thousands or millions of dollars in wealth and rest assured that no hacker or government can take it from them. In practice, it’s never so easy.

One problem is that single-signature addresses rely on protecting a single private key.

A better solution would use for example 2-of-3 multisig (or any combination of M-of-N for M ≤ N) quorum consisting of three separate private keys, held by three separate people or entities, and requiring any two to sign. This provides both security and redundancy since compromising any one key/person does not break the quorum: if one key is stolen or lost, the other two keyholders can sweep funds to another address (protected by a new quorum) by mutually signing a transaction moving the funds.

As an example, let us imagine the following scenario. An institution launches a stablecoin. For safety, it is required that 3 out of 5 designated addresses sign any mint or burn transaction. Alice deploys the multisig SC. She adds Bob, Charlie, Dave and Eve as signers to the contract and sets the quorum to a minimum number of signers to 3. A quorum of signatures is also required to add or remove signers after the initial deployment. If for some reason, Eve’s account is compromised, Alice proposes removing Eve’s address from the signers’ board. Charlie and Dave sign, causing Eve’s address to be removed. There are only 4 addresses now registered in the contract. By the same process, signers could add 2 more addresses to their ranks, and increase the required quorum signatures from 3 to 4.

Thus, essentially the multisig SC (we will refer to it, from now on, as MSC) enables multiple parties to sign or approve an action that takes place - typically a requirement for certain wallets, accounts, and smart contracts to prevent a rogue or hacked individual from performing detrimental actions.

## Multisig transaction flow
On-chain multisig wallets are made possible by the fact that smart contracts can call other smart contracts. To execute a multisig transaction the flow would be:

* A proposer or board member proposes an action.
* The proposed action receives an unique id/hash.
* N board members are notified (off-chain) to review the action with the specific id/hash.
* M out of N board members sign and approve the action.
* Any proposer or board member “performs the action”.

## Design guidelines
 
The required guidelines are:
* **No external contracts.** Calling methods of other contracts from within the methods of your own MSC is an amazing feature but should not be required for our simple use case. This also avoids exposing us to bugs. Because any arbitrarily complex function call can be executed, the MSC functions exactly as a standard wallet, but requires multiple signatures.

* **No libraries.** Extending the last guideline, our contract has no upstream dependencies other than itself. This minimizes the chance of us misunderstanding or misusing some piece of library code. It also forces us to stay simple and eases auditing and eventually formal verification.

* **Minimal internal state.** Complex applications can be built inside of MultiversX smart contracts. Storing minimal internal state allows our contract’s code to be simpler, and to be written in a more functional style, which is easier to test and reason about.

* **Uses cold-storage.** The proposer which creates an action or spends from the contract has no special rights or access to the MSC. Authorization is handled by directly signing messages by the board members’ wallets that can be hardware wallets (Trezor; Ledger, etc.) or software wallets.

* **Complete end-to-end testing.** The contract itself is exhaustively unit tested, audited and formally verified.

## Roles
* **Deployer** - This is the address that deploys the MSC. By default this address is also the owner of the SC, but the owner can be changed later if required, as this is by default supported by the MultiversX protocol itself. This is the address that initially set up the configuration of the SC: board members, quorum, etc. It is important to mention that at deployment a very important configuration parameter is the option to allow the SC to be upgradeable or not. It is recommended for most use cases the SC to be non-upgradeable. Leaving the SC upgradable will give the owner of the SC the possibility to upgrade the SC and bypass the board, defeating the purpose of a MSC. If keeping the SC upgradeable is desired, a possible approach would be to make the owner another MSC, and both SCs could maintain the same board, so an upgrade action would need the approval of the board.

* **Owner** - The deployer is initially the owner of the MSC, but if desired can be changed later by the current owner to a different owner. If the SC is upgradeable, the owner can also upgrade the SC.

* **Board and quorum** - Multiple addresses need to be previously registered in the MSC, forming its board. A board member needs to be specifically registered as a board member, meaning for example that the owner or deployer of the MSC is not automatically a board member as well. Board members can vote on every action that the MSC performs. Signing a proposed action means the board members agree. Customarily, not all board members will need to sign every action; the MSC will configure how many signatures will be necessary for an action to be performed, the quorum. For instance, such a contract could have 5 board members, but a quorum of 3 would be enough to perform any action (M-of-N or in this case 3-of-5).

* **Proposer** - The proposer is an address whitelisted in the MSC that can propose any action. An action can be any transaction; for example: send 10 eGLD to the treasury, mint more ESDT, etc. All board members are proposers by default but non-board members can be added as well to the list of whitelisted proposers. The proposers can only propose actions that then need to be approved and signed by the board members. The board member that proposes an action doesn’t need to sign it anymore; it is considered signed.

## Functionality
The MSC should be able to perform most tasks that a regular account is able to perform. It should also be as general as possible. This means that it should operate with a generic concept of “Action”, that the board needs to sign before being performed. Actions can interact with the MSC itself (let's call them **internal actions**) or with external addresses or other SC (**external actions**).

External actions have one and only one function, which is to send the action as a transaction whose sender is the MSC. Because any arbitrarily complex function call can be executed, the MSC functions exactly as a standard wallet, but requires multiple signatures.

The types of internal actions should be the following:

* Add a new member to the board.
* Remove a member from the board. This is only allowed if the new board size remains larger than the number of required signatures (quorum). Otherwise a new member needs to be added first.
* Change the quorum: the required number of signatures. Restriction: 1 <= quorum <= board size.
* Add a proposer.
* Remove a proposer.
* Change multisig contract owner (might be relevant for upgrading the MSC).
* Pay functions - by default we recommend the MSC to not be set up as a payable SC and any deposit or send transaction of eGLD or ESDT towards the MSC will need to call the desired pay function (if a transaction is not a call to these 2 functions then it is rejected immediately and the value is sent back to original sender): Deposit and/or Send. By making the MSC not a payable MSC we reduce the risk of users sending into the MSC funds that then are locked in the MSC or need to be manually send back to the user (in case of a mistake). By making the MSC not a payable MSC it also means that any deposit or send transaction needs to explicitly call the deposit or send function of the MSC.

Any external and internal action will follow these steps and process:

* **Propose action:** this will generate an action id. The action id is unique.
* **View action:** the board members need to see the action proposed before they approve it.
* **Sign action:** board members are allowed to sign. We might add an expiration date until board members can sign (until block x…).
* **Un-sign action:** board members are allowed to un-sign, i.e. to remove their signature from an action. Actions with 0 signatures are cleared from storage. This is to allow mistakes to be cleared.
* **Perform action (by id/hash)** - can be activated by proposers or board members. It is successful only if enough signatures are present from the board members. Whoever calls “perform action” needs to provide any eGLD required by the target, as well as to pay for gas. If there is a move balance kind of action, who calls the action pays the gas and the amount to be moved is taken from MSC balance. But the gas is always taken from the balance of the one who creates the "perform action" transaction.

Also the following view functions will be available:
* **Count pending Actions:** returns the number of existing Actions.
* **List latest N pending Actions:** provides hashes of the latest N pending Actions, most recent being 0 and oldest being N-1. Usually called in tandem with Count.

## Initializing the MSC

There are 2 ways to do it:
* Provide all board member addresses and the number of required signatures directly in the constructor.
* Deployer deploys with just herself on the board and required signatures = 1. Then adds all other N-1 signers and sets required signatures to M. This works, but requires many transactions, so the constructor-only approach might be preferred.

MSC is a deployable SC written in Rust and compiled in WASM.

## Conclusion

Multisig accounts are a critical safety feature for all users of the MultiversX ecosystem. Decentralised applications will rely heavily upon multisig security.
