![](https://storageapi.fleek.co/nicopoggi-team-bucket/dab-gh%20(1).png)

# DAB

[![Fleek](https://img.shields.io/badge/Made%20by-Fleek-blue)](https://fleek.co/)
[![Discord](https://img.shields.io/badge/Discord-Channel-blue)](https://discord.gg/yVEcEzmrgm)

## 👀 Overview

> An Internet Computer open internet service for data.

All the data an IC app needs to make a seamless experience, accessible directly on the IC. [DAB](https://dab.ooo/) is an open internet service for NFT, Token, Canister, and Dapp registries.

### 🛣️ DAB's Roadmap (Order is not set)

* [x] NFT List
* [x] Canister List
* [ ] Token List
* [ ] Dapp List

---

## #️⃣ V0.2.0 - DAB's Current Registries.

In v0.2.0, DAB has two registries that developers can integrate with, or submit items to:

- The NFT List (auto-surface NFTs in apps and multi-standard support).
- The Canister List (associate metadata to Canister IDs and auto-surface it in UIs)

## 🎨 The NFT List

**The NFT list** DAB provides a list of NFTs that apps & developers can **consume to surface new NFTs as they are listed in DAB, instead of manually adding them one by one**.

DAB's NFT list is **standard agnostic** and through the DAB-js library, developers can easily integrate and make calls to any NFT collection on the list regardless of their NFT standard interface (EXT, Departure Labs, etc.), **because in its library DAB wraps all standards into a common javascript interface**.

* [**View the NFT List Canister Source Code**](registries/nft/)

### 🖌️ Current NFT List & Making New Submissions

You can see the current listed NFT collections in the link below. **Want to submit a new NFT collection to the list? Use the form below.**

* [**View the current NFT Collection List📜**](registries/nft/list.json)
* [**Submit a new NFT to the list 📫**](https://dab-ooo.typeform.com/nft-list)

The review process for submissions is currently manual and done by the DAB core team; in the future we will automate the process, and migrate to a community-governed and trustless system.

### 🧰 Start Integrating DAB's NFT List into your App

To interact with DAB's services you need to use the DAB-js library. Read our documentation or visit the DAB-js repository to get started.

* [**Read our documentation**](https://docs.dab.ooo)
* [**DAB-js library - Repository**](https://github.com/psychedelic/dab-js)


## 🛢️ The Canister List

![](https://storageapi.fleek.co/fleek-team-bucket/canregistry.png)

**The Canister List** is a canister registry where you can associate Canister IDs to a metadata profile (name, front-end URL, description, logo...) to make them more discoverable by UIs. 

Apps that show Canister IDs in their UIs/apps can **integrate to the Canister List** to check if that Canister ID has associated metadata, and display it for their users to see in a more descriptive and human-readable way.

- It helps make Canister ID human-readable and identifiable.
- It helps give users information to judge whether to trust a canister or not
- It can help in the future to identify duplicates or impersonations.

[**View the Canister Registry Source Code**](registries/canister_registry/)

### 🖌️ Submitting/Adding a Canister ID to the Canister List

Want to submit a new Canister ID to the registry to associate metadat to it, and have integrated apps auto-surface it? Use the form below.

* [**Submit a new Canister to the list 📫**](https://dab-ooo.typeform.com/canister-list)

For V0.2.0, the review process for submissions is manual and done by the DAB core team; in the future we will automate the process, and migrate to a community-governed and trustless system.

**We are exploring an automated way of adding Canister IDs and their metadata to the registry**. The main issues are confirming the controller is the one submitting it, and then adding a verification layer to avoid duplicates/phishing/impersonation.

### 🧰 Start Integrating DAB's Canister List into your App

To interact with DAB's services you need to use the DAB-js library. Read our documentation or visit the DAB-js repository to get started.

* [**Read our documentation**](https://docs.dab.ooo)
* [**DAB-js library - Repository**](https://github.com/psychedelic/dab-js)
---

## ⛱️ Sandbox

In the Sandbox directory, you can find in-development projects, such as the canister directory, and other data registries that the DAB team is exploring. Developers are welcome to review and provide feedback.

* [Address book](sandbox/address/README.md)
* [Profile metadata](sandbox/profile/README.md)

---

## License

DAB © Fleek LLC 2021 - [License (GPL-3.0)](https://github.com/Psychedelic/dab/blob/main/LICENSE)
