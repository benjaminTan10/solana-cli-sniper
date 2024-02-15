# Running the project

You can run the project using either Bash or Powershell.

## Bash

```bash
export RPC_ENDPOINT="wss://mainnet.helius-rpc.com/?api-key=d9e80c44-bc75-4139-8cc7-084cefe506c7"
export RUST_LOG="info"
cargo run
```

## Powershell

```bash
$env:RPC_ENDPOINT="wss://mainnet.helius-rpc.com/?api-key=d9e80c44-bc75-4139-8cc7-084cefe506c7"
$env:RUST_LOG="info"
cargo run
```