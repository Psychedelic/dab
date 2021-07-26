# Decentralized Address Book (Dab)

## Overview

What Dab does is not much different from a classic phone book. With either, you can look up for different addresses and the names associated with them. Dab acts as a phone book for canisters based on the Internet Computer. Each canister has a name and a principal ID, it is Dab’s responsibility to store them and return the ID when a call is made for the name.

## How to run

To run this canister you need to have `dfx` installed.

``` bash
$ dfx start --background
$ dfx deploy

$ dfx canister call dab name
("Decentralised Address Book")
```
