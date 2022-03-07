# Interaction Guide

Interacting with the address book canister and the methods it offers is a fairly easy and efficient process. Currently the private address book canister offers its users a private address book for storing different principal IDs and the names associated with them. You can find all of the methods and their usage in the
shell script located [here](https://github.com/Psychedelic/dab/blob/main/scripts/method-tests.sh).

## Private Address Book Methods

The address book canister has four methods:

| Method Name      | Description                                                                                                  |
| -----------      | -----------                                                                                                  |
| name             | Returns the name of the cansiter.                                                                            |
| get_all          | Returns all of the addresses from the address book.                             |
| remove_address   | Removes the address associated with the canister name from the address book. Returns `Ok(())` if successful. |
| add              | Adds a new address to the registry and returns `Ok(())` if the call is successful.                           |

### How to use them?

First, we add a new address to our private address book with the `add` method:

```bash
$ dfx canister call address add "(record { name= \"XTC\"; description= opt \"Dank's ledger\"; emoji= opt \"🚀\"; principal_id= principal \"aanaa-xaaaa-aaaah-aaeiq-cai\"}}"
(variant { Ok = null })
```

Now we can use the `get_all` method and ask the canister to return all of the addresses that are associated with our principal id:

```bash
$ dfx canister call address get_all
(
  record {
    principal_id = principal "aanaa-xaaaa-aaaah-aaeiq-cai";
    name = "XTC";
    emoji= opt "🚀";
    description = opt "Dank's ledger";
  },
)
```

Okay, but since this canister is not deployed to the mainnet yet, we should remove it from the address book. To remove an address we use the `remove` method:

```bash
$ dfx canister call address remove "(\"XTC\")"
(variant { Ok = null })
```

Voilà! We have used `add`, `remove`, and `get_all`! And since the `name` method only returns the name of the canister and doesn't actually do an operation, we are not going to call it here.
