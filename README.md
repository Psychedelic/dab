# DAB

> An open internet service for user profile information

## Overview

DAB is an IC open internet service to support dapp's that want to integrate user's self-sovereign profile data. DAB is a decentralized application on the Internet Computer.

DAB will allow user's to set and edit publicly available profile information (e.g. 'display name', 'emoji', 'avatar url') against their IC principal id. Dapp developers will be able to access this information on behalf of their users and integrate for improved UX.

> DAB is currently a reference implementation looking for community feedback

DAB is in its early stages, Fleek is seeking to add profile capabilities to [plug](https://github.com/psychedelic/plug), but believes the community more generally would find shared profiles useful in dapp development, and so are seeking early community feedback.

## Roadmap

* [x] open source proposed API
* [ ] deploy v0.1.0 test canister to mainnet
* [ ] plug integration

## How to run

To run this canister you need to have `dfx` and `node` installed.

``` bash
$ dfx start --background
$ dfx deploy
$ dfx canister call dab name
("DAB")
```

## How to interact with DAB

Interacting with DAB is an easy process. You can find the complete guide [here](https://github.com/Psychedelic/dab/blob/main/dab/README.md).
