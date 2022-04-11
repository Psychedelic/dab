# 🛢️ Canister Registry/List 

![](https://storageapi.fleek.co/fleek-team-bucket/canregistry.png)

**The Canister List** is a canister registry where you can associate Canister IDs to a metadata profile (name, front-end URL, description, logo...); apps can then integrate to DAB to query this data in their UIs and show it to make Canister IDs more human-readable.

### 🖌️ Submitting/Adding a Canister ID to the Canister List

Want to submit a new Canister ID to the registry to associate metadat to it, and have integrated apps auto-surface it? Use the form below.

* [**Submit a new Canister to the list 📫**](https://dab-ooo.typeform.com/canister-list)
  

You can interact with the registry canister using the methods it provides. You can find all of the methods and their usage in the
shell script located [here](https://github.com/Psychedelic/dab/blob/main/scripts/registry-tests.sh).

## 🧰 Interaction guide

As a developer that is looking to integrate DAB into an app/UI, your main point of interaction should be to use the [DAB-js library](https://github.com/psychedelic/dab-js). You can [read our documentation](https://docs.dab.ooo) to get detailed examples on how to do so.

### Registry Canister Methods

The registry canister has two public methods. You can find the details of these methods in the [candid file](https://github.com/Psychedelic/dab/blob/main/candid/registry.did).

| Method Name        | Description                                                                                           |
| -----------        | -----------                                                                                           |
| name               | This method return the name of the canister for health-check                                          |
| get                | This method returns the metadata associated with the given canister principal IDs                     |
| get_all            | This method returns all of the information stored in the registry                                     |



## How to use them?

In this section we take a look at the methods that the registry canister offers. Let's start by learning the structure of the metadata.

The version of the metadata helps developers identify new updates and changes to the metadata. Version increments by one, every time the metadata receives an update.

The registry canister's principal ID on the mainnet is `curr3-vaaaa-aaaah-abbdq-cai`. Let's check if we have the right canister:

```sh
$ dfx canister --network=ic call curr3-vaaaa-aaaah-abbdq-cai name
("Canister Registry")
```

Now that we are sure of the canister's principal ID, let's ask the registry for XTC canister's metadata:

```sh
$ dfx canister --network=ic call curr3-vaaaa-aaaah-abbdq-cai get "(principal \"aanaa-xaaaa-aaaah-aaeiq-cai\")"
(
  vec {
    opt record {
      url = "https://dank.ooo/xtc/";
      name = "XTC";
      description = "Cycles Token (XTC) is a token that allows users or developers to hold cycles with just a Principal ID, and send, trade, or develop canisters with them.";
      version = 0 : nat32;
      logo_url = "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/XTC.svg";
    };
  },
)
```

### NOTE:

The `details` field in this canister contains the **category** of the entry canister: `Vec<(String, DetailValue::Text(String))>`
