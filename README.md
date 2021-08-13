![](https://storageapi.fleek.co/fleek-team-bucket/Dank/dabcover.png)
# DAB

[![Fleek](https://img.shields.io/badge/Made%20by-Fleek-blue)](https://fleek.co/)
[![Discord](https://img.shields.io/badge/Discord-Channel-blue)](https://discord.gg/yVEcEzmrgm)

## Overview

> An IC open internet service for user profile information

DAB is an IC open internet service to support dapp's that want to integrate user's self-sovereign profile data. DAB is a decentralized application on the [Internet Computer](https://dfinity.org).

DAB will allow user's to set and edit publicly available profile information (e.g. 'display name', 'emoji', 'avatar url') against their IC principal id. Dapp developers will be able to access this information on behalf of their users and integrate for improved UX.

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
