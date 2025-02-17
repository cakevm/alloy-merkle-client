# Alloy Merkle Auction Client

This crate contains structs for the [Searchers API](https://docs.merkle.io/private-pool/searchers/bid-on-transactions) of [merkle.io](https://merkle.io) with [Alloy](https://github.com/alloy-rs/alloy). Additionally, it allows to convert from a merkle transaction to an alloy transaction.

## Remarks
### Why name it client when only dto's are provided? 
I guess at some point I will implement the client as well.

### There is `merkel-sdk-rs` already, why another crate?
The [merkel-sdk-rs](https://github.com/merkle3/merkle-sdk-rs) still uses `ethers` and seems not to be maintained.

## Usage
See `subscribe_merkle_auction.rs` in [examples](./examples) for a full usage examples. Use whatever WebSocket library you want to connect to the merkle.io API.

## Acknowledgements
Many thanks to the [merkle.io](https://merkle.io)  to provide such an API. And many thanks to the [alloy-rs](https://github.com/alloy-rs) team.

## License
This project is licensed under the [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).