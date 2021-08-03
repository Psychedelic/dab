# Interaction Guide

Interacting with the address book canister and the methods it offers is a fairly easy and efficient process. Currently the private address book canister offers its users a private address book
for storing different canister principal IDs and the names associated with them. You can find all of the methods and their usage in the
shell script located [here](https://github.com/Psychedelic/dab/blob/main/scripts/method-tests.sh).

## Private Address Book Methods

The private address book has four methods:

| Method Name      | Description                                                                                                |
| -----------      | -----------                                                                                                |
| add_address      | Adds a new address to users address book. Returns `true` if successful.                                    |
| get_address      | Returns the principal ID associated with the canister name from the address book.                          |
| get_all          | Returns all of the addresses from the address book with the total count of them.                           |
| remove_address   | Removes the address associated with the canister name from the address book. Returns `true` if successful. |

### How to use them?

Let's jump to the commandline and check these methods out. First, we add a new address to our private address book with the `add_address` command:

```bash
$ dfx canister call address add_address "(\"XTC\", principal \"aanaa-xaaaa-aaaah-aaeiq-cai\")"
(true)
```

Now we can use the `get_address` method and ask the canister to return the address associated with name `XTC`:

```bash
$ dfx canister call address get_address "(\"XTC\")"
(
  record {
    canister_id = opt principal "aanaa-xaaaa-aaaah-aaeiq-cai";
    canister_name = "XTC";
  },
)
```

Everything seems right! Let's add another address and use the `get_all` method to ask the canister for all the addresses we have added:

```bash
$ dfx canister call address add_address "(\"address_book\", principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
(true)
$ dfx canister call address get_all
(
  record {
    list = vec {
      record {
        record {
          principal "YOUR-PRINCIPAL-ID";
          "address_book";
        };
        principal "rrkah-fqaaa-aaaaa-aaaaq-cai";
      };
      record {
        record {
          principal "YOUR-PRINCIPAL-ID";
          "XTC";
        };
        principal "aanaa-xaaaa-aaaah-aaeiq-cai";
      };
    };
    total_addresses = 2;
  },
)
```

Okay, but since this canister is not deployed to the mainnet yet, we should remove it from the address book. To remove an address we use the `remove_address` method:

```bash
$ dfx canister call address remove_address "(\"address_book\")"
(true)
```

Voilà! Now let's say after sometime we find out XTC's canister ID has changed. What should we do then? In that scenario, the address should be updated. We use the same `add_address` method that we used before for that purpose:

```bash
$ dfx canister call address add_address "(\"XTC\", principal \"NEW-PRINCIPAL\")"
(true)
$ dfx canister call address get_address "(\"XTC\")"
(
  record {
    canister_id = opt principal "NEW-PRINCIPAL";
    canister_name = "XTC";
  },
)
```
