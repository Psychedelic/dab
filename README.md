![](https://storageapi.fleek.co/nicopoggi-team-bucket/dab-gh%20(1).png)
# DAB

[![Fleek](https://img.shields.io/badge/Made%20by-Fleek-blue)](https://fleek.co/)
[![Discord](https://img.shields.io/badge/Discord-Channel-blue)](https://discord.gg/yVEcEzmrgm)

## 👀 Overview

> An Internet Computer open internet service for data.

All the data an IC app needs to make seamless experiences, accessible directly on the IC. [DAB](https://dab.ooo/) is an open internet service for NFT, Token, Canister, and Dapp registries.

### 🛣️ DAB's Roadmap (Tentative)

* [x] NFT List
* [ ] Token List
* [ ] Canister List (partially developed)
* [ ] Dapp List

---

## 🎨 V0.1.0 - The NFT List

![](https://storageapi.fleek.co/nicopoggi-team-bucket/dab-gh-nft.png)

In its first iteration, we have released DAB's first registry, **the NFT list** DAB provides a list of NFTs that apps & developers can **consume to surface new NFTs as they are listed in DAB, instead of manually adding them one by one**.

DAB's NFT list is **standard agnostic** and through the DAB-js library, developers can easily integrate and make calls to any NFT collection on the list regardless of their NFT standard interface (EXT, Departure Labs, etc.), **because in its library DAB wraps all standards into a common javascript interface**.

- [**View the NFT List Canister Source Code**](registries\nft\README.md)

### 🖌️ Current NFT List & Making New Submissions
You can see the current listed NFT collections in the link below. **Want to submit a new NFT collection to the list? Use the form below.**

- [**View the current NFT Collection List📜**](registries\nft\Cargo.toml)
- [**Submit a new NFT to the list 📫**](https://dab-ooo.typeform.com/nft-list)

For V0.1.0, the review process for submissions is manual and done by the DAB core team; in the future we will automate the process, and migrate to a community-governed and trustless system.

### 🧰 Start Integrating DAB's NFT List into your App
To interact with DAB's services you need to use the DAB-js library. Read our documentation or visit the DAB-js repository to get started.

- [**Read our documentation**](https://docs.dab.ooo)
- [**DAB-js library - Repository**](https://github.com/psychedelic/dab-js)

----
## ⛱️ Sandbox

In the Sandbox directory, you can find in-development projects, such as the canister directory, and other data registries that the DAB team is exploring. Developers are welcome to review and provide feedback.

- [Address book](sandbox\address\README.md)
- [Profile metadata](sandbox\profile\README.md)
- [Canister registry](sandbox\registry\README.md)

----

## License

DAB © Fleek LLC 2021 - [License (GPL-3.0)](https://github.com/Psychedelic/dab/blob/main/LICENSE)
