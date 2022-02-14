![](https://storageapi.fleek.co/nicopoggi-team-bucket/dab-gh%20(1).png)

# DAB

[![Fleek](https://img.shields.io/badge/Made%20by-Fleek-blue)](https://fleek.co/)
[![Discord](https://img.shields.io/badge/Discord-Channel-blue)](https://discord.gg/yVEcEzmrgm)

## 👀 Overview

> An Internet Computer open internet service for data.

All the data an IC app needs to make a seamless experience, accessible directly on the IC. [DAB](https://dab.ooo/) is an open internet service that keeps track of and provides accessibility to verified and non-verified registries on the Internet Computer.

### 🛣️ DAB's Roadmap (Order is not set)

* [x] NFT List
* [x] Canister List
* [X] Token List
* [ ] Dapp List

---

## 🛠 Creating Your Own Registry

Anyone is able to create their own DAB registry by following the [DAB Registry Standard](https://github.com/Psychedelic/dab/blob/278f25c20ad426c58f8d97dfa352c20dfb9999de/candid/STANDARD.md). Any additional logic you'd like to add to your registry is up to your own discretion but it is imperative that the Registry Standard is implemented without fault. If not, your canister will not work with services built for DAB registries such as [DAB-js](https://github.com/Psychedelic/DAB-js).

If you're unsure of how an implementation of the DAB Registry Standard would look like, we have created an example implementation in rust that we are calling [the Template Registry](https://github.com/Psychedelic/dab/tree/main/template_registry).

All newly created registries are unverified. To request verification of your registry, submit [this form](https://dab-ooo.typeform.com/to/m8qBZHWu?typeform-source=admin.typeform.com).

---

## #️⃣ V1.0.0 - DAB's Current Verified Registries.

In v1.0.0, DAB has three registries that developers can integrate with, or submit items to:

- The NFT List (auto-surface NFTs in apps and multi-standard support).
- The Canister List (associate metadata to Canister IDs and auto-surface it in UIs)
- The Token List (auto-surface Tokens in apps and multi-standard support).

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

## 💠 The Token List

The Token Registry will work exactly like the NFT List. Any Token can get listed on this open registry, regardless of its standard (DIP20, EXT to start), adding metadata for UIs to surface (name, symbol, image, Canister ID, standard…)

Then UIs, apps, and DeFi experiences can consume this list & integrate it using DAB-js to integrate and auto-surface and support all tokens on the list for your users (showing their balance, allowing them to interact with them for example to make transfers), as well as anyone that’s added in the future, without having to do per-token or per-standard integrations.

- [Read our getting started guide](https://docs.dab.ooo/token-list/getting-started/)
- [DAB-js library - Repository](https://github.com/psychedelic/dab-js)

###  Making New Submissions
You can see the current listed tokens on our website. **Want to submit a new Token to the list? Use the form below.**

- [View the current Token List📜](registries/tokens/list.json)
- [Submit a new Token to the list 📫](https://dab-ooo.typeform.com/token-list)


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

For V1.0.0, the review process for submissions is manual and done by the DAB core team; in the future we will automate the process, and migrate to a community-governed and trustless system.

**We are exploring an automated way of adding Canister IDs and their metadata to the registry**. The main issues are confirming the controller is the one submitting it, and then adding a verification layer to avoid duplicates/phishing/impersonation.

### 🧰 Start Integrating DAB's Registries into your App

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
