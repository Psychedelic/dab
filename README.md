![](https://storageapi.fleek.co/fleek-team-bucket/dab/dab-github.png)
# DAB

[![Fleek](https://img.shields.io/badge/Made%20by-Fleek-blue)](https://fleek.co/)
[![Discord](https://img.shields.io/badge/Discord-Channel-blue)](https://discord.gg/yVEcEzmrgm)

## Overview

> An Internet Computer open internet service for data.

DAB is an open internet service for interface-agnostic user profiles & data, human-readable Principal ID & Canister ID names & info, as well as canister, token & NFT registries & verification for quality assurance and reduced risk of scams in the ecosystem.

DAB is currently in development, and coming soon. The features above represent our vision for DAB, and we are starting with the implementations seen below. This V0, open sourced version is intended to showcase the concept to the community, and gather feedback.

[Visit DAB's website](https://dab.ooo/) to stay up to date with its release, and make early submissions to our Token, NFTs, and Dapp lists/registries.

### Roadmap

* [x] V1- User profiles / User Data (e.g name, avatar, owned canisters)
* [x] Address book registry (private contact list)
* [ ] Canister metadata registry (in development)
* [ ] Principal IDs, Canister ID metadata registry
* [ ] Canister, Token, NFTs, Dapps registries

---
## V0 - Profiles & Address Books

In its first iteration, DAB is an IC open internet service to support dapp's that want to integrate user's self-sovereign profile data. DAB is a decentralized application/open internet service on the [Internet Computer](https://dfinity.org).

DAB will allow user's to set and edit publicly available profile information (e.g. 'display name', 'emoji', 'avatar url') against their IC principal id. Dapp developers will be able to access this information on behalf of their users and integrate for improved UX.

 **One profile with all your user data, referenced by your Principal ID, that apps/UIs can automatically surface.** Users could log into any integrated IC app, and have their profile, contacts, owned canisters, and more automatically surfaced. No more app-specific data or profiles. It's universal, and blockchain wide.

> DAB is currently a **reference implementation** looking for community feedback

DAB is in its early stages, Fleek is seeking to add profile capabilities to [Plug](https://github.com/psychedelic/plug), but believes the community more generally would find shared profiles useful in dapp development, and so are seeking early community feedback.

### Goals

* A dapp developer should be able to use DAB with Plug to bootstrap the profile details section of their app. A user logging into a DAB enabled app will see their already configured profile details (e.g. display name).
* A secondary goal is to support the profile and address book functions to move between dapps + devices.

### Roadmap

* [x] open source proposed API
* [ ] deploy v0.1.0 test canister to mainnet
* [ ] plug integration

## Development

### How to run

The DAB reference implementation is in `rust`.

To run this canister you need to have `dfx` and `node` installed.

``` bash
$ dfx start --background
$ dfx deploy
$ dfx canister call address name
("Address Book")
$ dfx canister call profile name
("Profile Canister")
```

### How to interact with DAB

Currently DAB has two canisters: the private address book canister and the profile metadata canister. You can learn more about each of these canisters and their methods from their documentations:
- [The Private Address Book Documentation](https://github.com/Psychedelic/dab/tree/main/address)
- [The Profile Metadata Documentation](https://github.com/Psychedelic/dab/tree/main/profile)

----

## License

DAB © Fleek LLC 2021 - [License (GPL-3.0)](https://github.com/Psychedelic/dab/blob/main/LICENSE)
