# Interaction Guide

Interacting with DAB and the method it offers is a fairly easy and efficient process. Currently DAB offers its users a private address book
for storing different canister principal IDs and the name associated with them. You can find all of the methods and their usage in the
shell script located [here](https://github.com/Psychedelic/dab/blob/main/scripts/method-tests.sh).

## Private Address Book Methods

The private address book has four methods:

- add_address(canister_name: String, canister_id: Principal): This method adds a new address to users address book. Returns `true` if successful.
- get_address(canister_name: String): This method returns the principal ID associated with the canister name from the address book.
- remove_address(canister_name: String): This method removes the address associated with the canister name from the address book. Returns `true` if successful.
- get_all(): This method returns all of the addresses from the address book with the total count of them.

### How to use them?

Let's jump to the commandline and check these methods out. First, we add a new address to our private address book with the `add_address` command:

```bash
$ dfx canister call dab add_address "(\"XTC\", principal \"aanaa-xaaaa-aaaah-aaeiq-cai\")"
(true)
```

Now we can use the `get_address` method and ask DAB to return the address associated with the cansiter name `XTC`:

```bash
$ dfx canister call dab get_address "(\"XTC\")"
(
  record {
    canister_id = opt principal "aanaa-xaaaa-aaaah-aaeiq-cai";
    address_exists = true;
    canister_name = "XTC";
  },
)
```

Everything seems right! Let's add another address and use the `get_all` method to ask DAB for all the addresses we have added:

```bash
$ dfx canister call dab add_address "(\"DAB\", principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
(true)
$ dfx canister call dab get_all
(
  record {
    list = vec {
      record {
        record {
          principal "YOUR-PRINCIPAL-ID";
          "DAB";
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

Okay, but since DAB is not deployed to the mainnet yet, we should remove it from the address book. To remove an address we use the `remove_address` method:

```bash
$ dfx canister call dab remove_address "(\"DAB\")"
(true)
```

Voilà! Now let's say after sometime we find out XTC's canister ID has changed. What should we do then? In that scenario, the address should be updated. We use the same `add_address` method that we used before for that purpose:

```bash
$ dfx canister call dab add_address "(\"XTC\", principal \"NEW-PRINCIPAL\")"
(true)
$ dfx canister call dab get_address "(\"XTC\")"
(
  record {
    canister_id = opt principal "NEW-PRINCIPAL";
    address_exists = true;
    canister_name = "XTC";
  },
)
```

## Profile Information Methods

The profile information methods are not yet implemented and this is just an introduction to the methods that will be added soon.

| Method Name        | Description                                                                                          |
| -----------        | -----------                                                                                          |
| get_public_profile | This method returns the public information of the profile associated with the principal ID provided. |
| set_display_name   | This method updates the display name of the caller.                                                  |
| set_description    | This method updates the biography of the caller.                                                     |
| set_emoji          | This method updates the emoji associated with the caller.                                            |
| set_avatar         | This method updates the link to the avatar of the caller.                                            |
