# Elrond smart contract module - Governance

This is a standard smart contract module, that when added to a smart contract offers governance features:
- proposing actions
- voting/downvoting a certain proposal
- after a voting period, either putting the action in a queue (if it reached quorum), or canceling

Voting can only be done by depositing a certain token, decided upon first time setup.  

The module provides the following configurable parameters:  
- `quorum` - the minimum number of (`votes` minus `downvotes`) at the end of voting period  
- `minTokenBalanceForProposing` - Minimum numbers of tokens the proposer has to deposit. These automatically count as `votes` as well  
- `maxActionsPerProposal` - Maximum number of actions (transfers and/or smart contract calls) that a proposal may have  
- `votingDelayInBlocks` - Number of blocks to wait after a block is proposed before being able to vote/downvote that proposal
- `votingPeriodInBlocks` - Number of blocks the voting period lasts (voting delay does not count towards this)  
- `lockTimeAfterVotingEndsInBlocks` - Number of blocks to wait before a successful proposal can be executed  

The module also provides events for most actions that happen: 
- `proposalCreated` - triggers when a proposal is created. Also provoides all the relevant information, like proposer, actions etc.  
- `voteCast` - user voted on a proposal  
- `downvoteCast` - user downvoted a proposal  
- `proposalCanceled`, `proposalQueued` and `proposalExecuted` - provides the ID of the specific proposal  
- `userDeposit` - a user deposited some tokens needed for a future payable action  

Please note that although the main contract can modify the module's storage directly, it is not recommended to do so, as that defeats the whole purpose of having governance. These parameters should only be modified through actions.
