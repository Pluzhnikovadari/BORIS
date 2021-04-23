# substrate-api-client

<p align="center">
<img src=./web3_foundation_grants_badge_black.svg width = 400>
</p>

substrate-api-client a library written in Rust for connecting to the substrate's RPC interface via WebSockets allowing to

* Compose extrinsics, send them and subscribe to updates (synchronously).
* Watch events and execute code upon events.

## Prerequisites

In order to build the substrate-api-client and the examples, Rust and the wasm target are needed. For Linux:

    curl https://sh.rustup.rs -sSf | sh

    rustup default nightly
    rustup target add wasm32-unknown-unknown --toolchain nightly
    cargo install --git https://github.com/alexcrichton/wasm-gc

For more information, please refer to the [substrate](https://github.com/paritytech/substrate) repository.


## Examples

To run an example, clone the `substrate-api-client` repository and run the desired example directly with the cargo command:

```bash
    git clone https://github.com/scs/substrate-api-client.git
    cd substrate-api-client
    cargo run --example example_get_storage
```

Set the output verbosity by prepending `RUST_LOG=info` or `RUST_LOG=debug`.

The following examples can be found in the [examples](/src/examples) folder:

* [example_event_callback](/src/examples/example_event_callback.rs): Subscribe and react on events.
* [example_generic_extrinsic](/src/examples/example_generic_extrinsic.rs): Compose an extrinsic for any call in any module by supplying the module and call name as strings.




## Acknowledgements

The development of substrate-api-client is financed by [web3 foundation](https://web3.foundation/)'s grant programme.

We also thank the teams at

* [Parity Technologies](https://www.parity.io/) for building [substrate](https://github.com/paritytech/substrate) and supporting us during development.
