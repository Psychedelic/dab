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
| get_info           | This method returns the metadata associated with the given canister principal IDs                     |


## How to use them?

In this section we take a look at the methods that the registry canister offers. Let's start by learning the structure of the metadata.

Registry canister stores the following information about every canister that has been added to it:

- Name
- Description
- Front-end URL
- Logo URL
- Version of the metadata

The version of the metadata helps developers identify new updates and changes to the metadata. Version increments by one, every time the metadata receives an update.