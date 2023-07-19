# Digital Cash Contract

The basic idea of MultiversX Digital Cash is that ONE link can hold information (ESDT tokens, EGLD) on chain, this link can be sent from one person to another, there is no need to hold any wallet to receive, consume and send it forward to another person.

# Usage

## Covering up payment fees & funding

The contract allows funding any number of tokens in 1 call within a address under a valability if the fee cost was covered.

In order to fund one should first call `deposit_fees` depositing the fee funds as `eGLD` within the contract. Only after, if the fees cover the transfer of the certain number of tokens, it is possible to deposit the funds, making them available for claiming or forwarding.

`fund` after making sure everything is ok on the fee aspect will set up the `deposit` storage increasing the number of tokens to transfer by the number of tokens paid to the endpoint and set the expiration date by the number of rounds specified within the `valability` parameter.

The fees are unique per address and only cover one instance of transfer, either if it is a `claim` or a `forward`, per number of tokens transfered. Only by making one of these actions will consume the fee funds following to to refund the rest of the fees to the depositor.

## Claiming funds

Claiming the funds require the signature parameter to be valid. Next the round will be checked to ve greater than the `expiration_round` within the deposit. Once these requirement was fulfilled the funds will be sent to the caller and the remaining of the fee funds sent back to the depositor.

## Withdrawing funds

If the valability of a deposit has expired it can no longer be claimed. Anyone on this point can call `withdraw` making the funds go back to the depositor together with the unused fee funds.

## Forwarding funds

Funds cam be forwarded to another address using the signature, but the forwarded address requires to have the fees covered. This actions will also consume the funds from the initial address.

After the forward in case of a withdraw the funds will go to the `depositor_address` set within the `forwarded_address` deposit storage.
