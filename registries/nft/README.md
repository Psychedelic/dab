# 🎨 NFT Registry Canister

![Top Photo](https://storageapi.fleek.co/nicopoggi-team-bucket/dab-gh-nft.png)

The NFT registry canister provides a list of different NFT collections on the Internet Computer. Their names, standards they support and Principal IDs are part of the information.

## Current NFT List & New Submissions 📜

The current list of NFT collections that are live and queryable in the NFT List canister can be found in this directory. Want to make a new submissions to the list? Use the form below:

- [View the current NFT Collection List📜](list.json)
- [Submit a new NFT to the list 📫](https://dab-ooo.typeform.com/nft-list)

---

## 🧰 Interaction guide

As a developer that is looking to integrate DAB into an app/UI, your main point of interaction should be to use the [DAB-js library](https://github.com/psychedelic/dab-js), that also provides a standard wrapper to faciliate the integration to NFTs that use different standards. You can [read our documentation](https://docs.dab.ooo) to get detailed examples on how to do so.

**This canister currently has three public methods:**

- name: returns the name of the NFT registry canister for health check purposes
- get_canister: returns the metadata associated with the canister name that is passed as an argument
- get_all: returns a vector of all the canisters in the registry with their information

There are also other methods such as `add`, `edit` and `remove` but these methods are not accessible to the users. These methods are used by the Psychedelic team to maintain the canister and its information.

Let's call the name method and do a health-check:

```sh
$ dfx canister call replace_principal_id name
("NFT Registry Canister")
```

Now we can use the `get_canister` method to get the metadata of a canister in the registry (in this case ICPunks):

```sh
$ dfx canister call replace_principal_id get_canister "(\"ICPunks\")"
(
  opt record {
    name = "ICPunks";
    id = principal "qcg3w-tyaaa-aaaah-qakea-cai";
    standard = "ICPunks";
  },
)
```

Notice that we passed the name of the canister as an argument. That is necessary for the registry to find the canister. If you don't have an special canister in your mind, you can use the `get_all` method to get every canister's information:

```sh
$ dfx canister call replace_principal_id get_all
(
  vec {
    record {
      name = "ICPunks";
      principal_id = principal "qcg3w-tyaaa-aaaah-qakea-cai";
      standard = "ICPunks";
    };
    record {
      name = "ICP News";
      principal_id = principal "uzhxd-ziaaa-aaaah-qanaq-cai";
      standard = "EXT";
    };
    record {
      name = "ICPuzzle";
      principal_id = principal "owuqd-dyaaa-aaaah-qapxq-cai";
      standard = "EXT";
    };
    record {
      name = "Starverse";
      principal_id = principal "nbg4r-saaaa-aaaah-qap7a-cai";
      standard = "EXT";
    };
    record {
      name = "Cronic Wearables";
      principal_id = principal "tde7l-3qaaa-aaaah-qansa-cai";
      standard = "EXT";
    };
    record {
      name = "Cronic Critters";
      principal_id = principal "e3izy-jiaaa-aaaah-qacbq-cai";
      standard = "EXT";
    };
    record {
      name = "Wrapped ICPunks";
      principal_id = principal "bxdf4-baaaa-aaaah-qaruq-cai";
      standard = "EXT";
    };
    record {
      name = "IC Drip";
      principal_id = principal "3db6u-aiaaa-aaaah-qbjbq-cai";
      standard = "EXT";
    };
    record {
      name = "Wing";
      principal_id = principal "73xld-saaaa-aaaah-qbjya-cai";
      standard = "EXT";
    };
    record {
      name = "ICMojis";
      principal_id = principal "gevsk-tqaaa-aaaah-qaoca-cai";
      standard = "EXT";
    };
  },
)
```

That's it for now! You can also find the related script to these commands [here](https://github.com/Psychedelic/dab/blob/main/scripts/nft-tests.sh)
