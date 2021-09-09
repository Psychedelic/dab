#!/bin/sh

dfx canister call nft add "(record {principal_id= principal \"aanaa-xaaaa-aaaah-aaeiq-cai\"; name= \"xtc\"; standard= \"icpunks\"})"
dfx canister call nft get_all
dfx canister call nft remove "(\"xtc\")"
dfx canister call nft get_all