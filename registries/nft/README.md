# 🎨 NFT Registry Canister

![](https://storageapi.fleek.co/nicopoggi-team-bucket/dab-gh-nft.png)

The NFT registry canister provides a list of different NFT collections on the Internet Computer. Their names, standards they support and Principal IDs are part of the information.

## Current NFT List & New Submissions 📜

The current list of NFT collections that are live and queryable in the NFT List canister can be found in this directory. Want to make a new submissions to the list? Use the form below:

- [View the current NFT Collection List📜](registries\nft\Cargo.toml)
- [Submit a new NFT to the list 📫](https://dab-ooo.typeform.com/nft-list)

---

## 🧰 Interaction guide

As a developer that is looking to integrate DAB into an app/UI, your main point of interaction should be to use the [DAB-js library](https://github.com/psychedelic/dab-js), that also provides a standard wrapper to faciliate the integration to NFTs that use different standards. You can [read our documentation](https://docs.dab.ooo) to get detailed examples on how to do so.

This interaction guide is **not end-user oriented**, and focuses on reviewing all the exposed methods of this canister, usable in DFX, which the DAB team initially will use to maintain the canister and add new registries.

**This canister currently has four methods:**

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