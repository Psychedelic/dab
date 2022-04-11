# 🎨 NFT Registry Canister

The NFT registry canister provides a list of different NFT collections on the Internet Computer. Their names, standards they support and Principal IDs are part of the information.

## Current NFT List & New Submissions 📜

The current list of NFT collections that are live and queryable in the NFT List canister can be found in this directory. Want to make a new submissions to the list? Use the form below:

- [View the current NFT Collection List📜](list.json)
- [Submit a new NFT to the list 📫](https://dab-ooo.typeform.com/nft-list)

---

## 🧰 Interaction guide

As a developer that is looking to integrate DAB into an app/UI, your main point of interaction should be to use the [DAB-js library](https://github.com/psychedelic/dab-js), that also provides a standard wrapper to faciliate the integration to NFTs that use different standards. You can [read our documentation](https://docs.dab.ooo) to get detailed examples on how to do so.

**The principal ID of the registry on the mainnet: `ctqxp-yyaaa-aaaah-abbda-cai`**

**This canister currently has three public methods:**

- name: returns the name of the NFT registry canister for health check purposes
- get: returns the metadata associated with the canister principal ID that is passed as an argument
- get_all: returns a vector of all the canisters in the registry with their information

Let's call the name method and do a health-check:

```sh
$ dfx canister --network=ic call ctqxp-yyaaa-aaaah-abbda-cai name
("NFT Registry Canister")
```

Now we can use the `get_canister` method to get the metadata of a canister in the registry (in this case XTC):

`$ dfx canister --network=ic call ctqxp-yyaaa-aaaah-abbda-cai get "(principal \"aanaa-xaaaa-aaaah-aaeiq-cai\")"`

Notice that we passed the principal ID of the canister as an argument. That is necessary for the registry to find the canister. If you don't have an special canister in your mind, you can use the `get_all` method to get every canister's information:

```sh
$ dfx canister --network=ic call ctqxp-yyaaa-aaaah-abbda-cai get_all
(
  vec {
    record {
      icon = "https://qcg3w-tyaaa-aaaah-qakea-cai.raw.ic0.app/Token/1";
      name = "Wrapped ICPunks";
      description = "ICPunks wrapped under the EXT standard. 10,000 randomly generated, unique collectible clowns with proof of ownership stored on the Internet Computer blockchain. Created as a reference to a meme comparing the Internet Computer token (ICP) with the Insane Clown Posse.";
      timestamp = 1_631_892_608_834_917_796 : nat64;
      principal_id = principal "bxdf4-baaaa-aaaah-qaruq-cai";
      standard = "EXT";
    };
    record {
      icon = "https://73xld-saaaa-aaaah-qbjya-cai.raw.ic0.app/?tokenid=tpx6i-sykor-uwiaa-aaaaa-b4ako-aaqca-aaaaz-a";
      name = "Wing";
      description = "An NFT photographic series created by the photographer @olisav ";
      timestamp = 1_631_892_635_257_688_785 : nat64;
      principal_id = principal "73xld-saaaa-aaaah-qbjya-cai";
      standard = "EXT";
    }; 
    .
    .
    .
    record { 
      icon = "https://e3izy-jiaaa-aaaah-qacbq-cai.raw.ic0.app/?tokenid=hancg-5ykor-uwiaa-aaaaa-b4aaq-maqca-aabuk-a";
      name = "Cronic Critters";
      description = "Cronics is a Play-to-earn NFT game being developed by ToniqLabs for the Internet Computer. Cronics  incorporates breeding mechanics, wearable NFTs and a p2e minigame ecosystem and more.";
      timestamp = 1_631_892_601_985_356_293 : nat64;
      principal_id = principal "e3izy-jiaaa-aaaah-qacbq-cai";
      standard = "EXT";
    };
  },
)
```

That's it for now! You can also find the related script to these commands [here](https://github.com/Psychedelic/dab/blob/main/scripts/nft-tests.sh)

### NOTE:

The `details` field in this canister contains the **standard** of the entry NFT canister: `Vec<(String, DetailValue::Text(String))>`
