# Release

NOTE: these steps were done on Ubuntu 21.04

## Source

```sh
git clone git@github.com:Psychedelic/dab.git
git checkout main
git pull
```

## Setup

Skip this section if you have already setup dependencies for deploying the canister.

If you are missing any packages:

```sh
apt-get update -y
apt-get install -y libssl-dev pkg-config
```

If you haven't installed the wasm32 target, install it with

```sh
rustup target add wasm32-unknown-unknown
```

If you haven't installed the IC CDK optimizer, install it with the following. Run this from a directory where there is no Cargo.toml:

```sh
cargo install ic-cdk-optimizer
```

## Prepare

If cargo.lock file has been changed, run

```sh
cargo update
```

Build and test

```sh
node build.js
cargo test
```

## Deploy

```sh
dfx canister --network=ic --no-wallet install nft --mode=upgrade
```

## Update the registry

```sh
cd scripts
chmod +x add.js
./add.js path/to/your/csv/file
```
