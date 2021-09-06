# NFT Registry Canister

The NFT registry canister provides information about different NFT canisters on the Internet Computer. Their names, standards they support and principal IDs are part of the information.

## Interaction guide

This canister currently has four methods:

- name: returns the name of the NFT registry canister for health check purposes
- add: adds a new canister to the registry
- remove: removes a canister that has been added to the registry before
- get_all: returns a vector of all the canisters in the registry with their information

You can call these methods using DFX. Let's add a new canister to the registry:

```sh
$ dfx canister call nft name
("NFT Registry Canister")

$ dfx canister call nft add "(record {principal_id= principal \"principalAddress\"; name= \"ICPunks\"; standard= \"icpunks\"})"
("Operation was successful")

$ dfx canister call nft get_all
(
  vec {
    record {
      name = "xtc";
      principal_id = principal "aanaa-xaaaa-aaaah-aaeiq-cai";
      standard = "icpunks";
    };
  },
)

$ dfx canister call nft remove "(\"ICPunks\")"
("Operation was successful")

$ dfx canister call nft get_all
(vec {})
```

You can find the related script [here](https://github.com/Psychedelic/dab/blob/main/scripts/nft-tests.sh)