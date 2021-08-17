# Elrond smart contract module for holding attributes of NFTs/SFTs inside the SC storage.

This contract helps with handling the token attributes inside the SC storage, rather than placing it inside the token, thus potentially increasing the state of the blockchain.
This module is useful only to a contract handles (creates, burns, etc) tokens that contain attributes that only make sense inside the SC (ex. metadata).

Usually, SC handles tokens as described above at a number of one or two, or maximum 10. This module can handle as many as (u8::MAX - 1) different tokenIDs. Can be easily modified to handle more than that, but since it's not the 99% use cases, it's not the default impl.
Should not be used with a Token that its attributes are actually of direct importance to the user. Lile Bytes that represent images that the user actually cares about.

It offers:
* a set of methods to efficiently handle (set, read, update, get, is_empty) attributes for multiple Token IDs and multiple nonces.
