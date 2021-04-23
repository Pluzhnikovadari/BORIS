# Substrate

## Description
This project emplement interaction between two Quagga nodes with using blockchain.
Before interaction you shoud disable route acceptance between nodes but make their interaction enabled and build route map by yourself.

## Build and Run project
To build this project you shoud follow this steps:
1) Setting up your nodes in "substrate-node" folder
Following the instructions in substrate-node's README build your project and run this command:
For the first node:
```shell
./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --rpc-methods Unsafe \
```

For the second node:
```shell
./target/release/node-template \
   --base-path /tmp/node01 \
   --chain ./customSpecRaw.json \
   --port 30333 \
   --ws-port 9945 \
   --rpc-port 9933 \
   --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
   --validator \
   --rpc-methods Unsafe \
   --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

And then set up your keys sa in [tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/customchain)

2) Then set up api-client module in "substrate-api-client" folder. 
Here you need to interact only thith second node and run event-listener module using this command
```shell
cargo +nightly-2020-10-01 run --example example_event_callback
```

3) The last step is to run interraction script from main Substrate folder
For the first node:
```shell
sudo python3 interact.py alice
```
For the second node:
```shell
sudo python3 interact.py alice
```

## Result
Now you can add new paths in your Quagga and the interaction between your two nodes will take place via blockchain!
