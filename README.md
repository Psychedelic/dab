# Dfinity Address Book (Dab)

## Overview

The Dfinity Address Book (also known as the Dab) is a decentralized application and canister based on the Internet Computer.

What Dab does is not much different from a classic phone book. With either, you can look up for different addresses and the names associated with them. Dab acts as a phone book for canisters based on the Internet Computer. Each canister has a name and a principal ID, it is Dab’s responsibility to store them and return the ID when a call is made for the name.

For storing the information, Big Map is used. Big Map is a very minimalistic insert/update only auto-scalable key-value storage for the I.C.

## Methods

| Method Name    | Parameters                     | Return Type |
| :------------- | :----------                    | :--------   |
| name           | ___________                    | String      |
| add_canister   | key: String, value: Principal  | ()          |
| get_canister   | key: String | Principal        | Principal   |

## How to run

To run this canister you need to have `dfx` and `node` installed.

``` bash
dfx start --background
dfx deploy

dfx canister call dab name
-> returns the name: ("Dfinity Address Book")

dfx canister call dab add_canister (dank, principal "principal") -> ()
-> returns nothing: ()

dfx canister call dab get_canister (dank)
-> returns the principal ID associated with the key: ("principal")
```
