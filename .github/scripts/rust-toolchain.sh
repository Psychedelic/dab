#!/bin/bash

set -e

mkdir -p /github/home/.cargo

ln -s "/root/.cargo/bin" /github/home/.cargo

ls -la /github/home

rustup install nightly
rustup default nightly

rustup target add wasm32-unknown-unknown

apt-get update -y -o Acquire::https::deb.nodesource.com::Verify-Peer=false
apt-get install -y libssl-dev pkg-config