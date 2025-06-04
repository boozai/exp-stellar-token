Bare minimum SEP-41 compliant token

## Prerequisites
- Rust toolchain
- Install Soroban CLI:
  ```bash
  cargo install soroban-cli --locked
  ```
  Successful installation ends with output similar to:
  ```
  Installed package `soroban-cli v22.8.1` (executables `soroban`, `stellar`)
  ```
- Add the required WASM targets:
  ```bash
  rustup target add wasm32-unknown-unknown wasm32v1-none
  ```

## Building
Use `make build` to compile the contract:
```bash
make build
```
This generates `opt/token.wasm` with a summary like:
```
Wasm Hash: 01a40c7606e2af07d78dd442a0613e2df146ebc2aa456f48d470c944069249e1
✅ Build Complete
```

## Deployment attempts
Set the default network then deploy using the provided account:
```bash
stellar network use testnet
stellar contract deploy --wasm opt/token.wasm --alias contract \
  --source-account GBLMF4AFEAJKUTEJQJVAMLHJ3UDBKRFOF4EY4462CXHFUYZGYVLJTNL7
```
Due to lack of network connectivity the deployment fails with:
```
error: Networking or low-level protocol error: HTTP error: error trying to connect: tcp connect error: Network is unreachable (os error 101)
```
As deployment did not succeed, no contract address was produced.

## Deployment account
All deployment fees were intended to be paid from account:
```
Public key: GBLMF4AFEAJKUTEJQJVAMLHJ3UDBKRFOF4EY4462CXHFUYZGYVLJTNL7
```
