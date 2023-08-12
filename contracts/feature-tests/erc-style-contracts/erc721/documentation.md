# Abstract

Non Fungible Token (NFT) implementation, using the MultiversX SC Framework. This is a very simplistic implementation where each token simply has an ID, an assigned owner, and optionally a "co-owner", also known as "approved account".

# Deployment

Deployment of the smart contract requires the number of initial available tokens. They will be created and their owner will be set to the account that performed the deploy transaction.

```
fn init(initial_minted: u64)
```

`initial_minted` - number of tokens to create

# Actions - Owner only

The owner may create additional tokens at any given point (also known as _minting_ new tokens). This is done by calling the following function:

```
fn mint(count: u64, new_token_owner: &Address)
```

`count` - number of tokens to create  
`new_token_owner` - the owner of those newly created tokens  

# Actions - Any account

NFT has three base operations:
- transfer
- approve account
- revoke approval

Let's start with the approve/revoke pair. The owner of a token may "share" ownership with another account by _approving_ that account. An approved account may transfer the token on behalf of the owner. Approving an account is done by calling the following function:

```
fn approve(token_id: u64, approved_address: &Address)
```

`token_id` - the id of the token the approval is done for  
`approve_address` - the address of the new "co-owner"  

Revoking the approval is done by calling, unsurprisingly, the _revoke_ function:

```
fn revoke(token_id: u64)
```

Since there can only be one co-owner, there is no need to supply the address when revoking.

Of course, only the owner of the token may call the two functions above.

Now, the transfer function:

```
fn transfer(token_id: u64, to: &Address)
```

`token_id` - the id of the token to be transferred  
`to` - the address of the account that will receive ownership of the token  

This function may be called by either the owner of the token or the approved account (co-owner). The token is then transferred to its new owner and any approval is revoked.

# Conclusion

That's it! This is just a simple example for the exciting concept of non fungible tokens. This can be extended to using names instead of ids for tokens or whatever else you can think of! If you want to read more about this concept, you can do some research on ERC721.
