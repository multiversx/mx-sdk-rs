# MultiversX smart contract module - Governance

This is a standard smart contract module, that when added to a smart contract offers governance features:
- proposing actions
- voting/downvoting a particular proposal
- after a voting period, either putting the action in a queue (if it reached quorum) or canceling

Voting can only be done by depositing a specific token defined in the initial setup.

## Configuration

Initial configuration is done through the `init_governance_module` function. Usually, this should be called from the main contract's `#[init]` function.

Arguments for the init function:

- `governance_token_id` - the token that will be used for voting
- `quorum` - the minimum number of (`votes` minus `downvotes`) at the end of the voting period
- `min_token_balance_for_proposal` - Minimum numbers of tokens the proposer has to deposit. These automatically count as `votes` as well
- `voting_delay_in_blocks` - Number of blocks to wait after a block is proposed before being able to vote/downvote that proposal
- `voting_period_in_blocks` - Number of blocks the voting period lasts (voting delay does not count towards this)
- `lock_time_after_voting_ends_in_blocks` - Number of blocks to wait before a successful proposal can be executed

All of the above parameters execpt the `governance_token_id` can be changed later through proposals.

The module also provides events for most actions that happen:
- `proposalCreated` - triggers when a proposal is created. It also provides all the relevant information, like proposer, actions, etc.
- `voteCast` - user voted on a proposal
- `downvoteCast` - user downvoted a proposal
- `proposalCanceled`, `proposalQueued` and `proposalExecuted` - provides the ID of the specific proposal
- `userDeposit` - a user deposited some tokens needed for a future payable action

Please note that although the main contract can modify the module's storage directly, it is not recommended to do so, as that defeats the whole purpose of having governance. These parameters should only be modified through actions.

## Proposing actions

Proposing actions is done through the `propose` endpoint. An action has the following format:
    - gas limit for action execution
    - destination address
    - a vector of ESDT transfers, in the form of `ManagedVec<EsdTokenPayment>`
    - endpoint to be called on the destination
    - a vector of arguments for the endpoint, in the form of `ManagedVec<ManagedBuffer>`

A maximum of `MAX_GOVERNANCE_PROPOSAL_ACTIONS` may be proposed at a time. All actions are bundled into a single proposal, and a `proposal_id` is returned. This ID is further used for interacting with the proposal for the purpose of voting, downvoting, etc.

Additionally, a minimum of `min_token_balance_for_proposal` governance tokens must be deposited at proposal time.

Examples of actions that can be proposed:
- transfering ESDT tokens to user accounts
- calling other smart contracts, with or without sending tokens as well
- calling the goverance contract itself, for the purpose of changing configurable parameters

## Voting/Downvoting

After a period of `voting_delay_in_blocks` blocks, in which governance members can evaluate the proposal, the voting/downvoting period starts.

To express their desire for the proposal to be executed, governance members should deposit governance tokens through the `vote` endpoint. If they do not wish for it to executed, they should use the `downvote` endpoint.

This period lasts an amount of blocks equal to `voting_period_in_blocks`.

## Executing proposals

Once the voting period ends, proposals have to be queued, after which they're locked for another `lock_time_after_voting_ends_in_blocks` blocks. Then, they can be executed, which will launch all the proposed actions.

After the execution, the governance tokens can be withdrawn by the voters and downvoters, to further use them for other proposals.
