# Registry Canister Interaction Guide

You can interact with the registry canister using the methods it provides. You can find all of the methods and their usage in the
shell script located [here](https://github.com/Psychedelic/dab/blob/main/scripts/registry-tests.sh).

## Registry Canister Methods

The profile canister has seven methods. You can find the details of these methods in the [candid file](https://github.com/Psychedelic/dab/blob/main/candid/profile.did).

| Method Name        | Description                                                                                          |
| -----------        | -----------                                                                                          |
| get_info           | This method returns the metadata associated with the given canister name                             |
| add_canister       | This method adds a new canister with its metadata to the registry                                    |
| set_url            | This method updates the front-end URL of the canister.                                               |
| set_description    | This method updates the description associated with the canister.                                    |
| set_idl            | This method updates the IDL of the canister.                                                         |
| set_logo           | This method updates the link to the logo of the canister.                                            |

## How to use them?

In this section we take a look at the methods that the registry canister offers and try to add a new canister to the registry. Let's start by learning the structure of the metadata.

Registry canister stores the following information about every canister that has been added to it:
- Principal ID
- Description
- Front-end URL
- IDL
- Logo URL
- Version of the metadata

The version of the metadata helps you identify new updates and changes to the metadata. Version increments by one every time the metadata receives an update. Let's add the XTC canister to the registry as an example to see how it works. Remember that you have to be a controller of the canister if you want to apply changes to its metadata in the registry.

First we add our canister to the registry using the `add_canister` method. This method two arguments: the name of the canister and its metadata:

```sh
dfx canister call registry add_canister "(\"XTC\", record {principal_id= principal \"aanaa-xaaaa-aaaah-aaeiq-cai\"; description= \"The Cycles Token (XTC) is Dank's first product.\"; url= \"https://dank.ooo\"; idl= null; logo_url= \"https://github.com/Psychedelic/dank\"; version= 0})"

```

Notice that we left the version of the canister to be zero. That's the initial version and with every new update it increments by one. Let's try to change the description of the canister:

```sh
dfx canister call registry set_description "(\"XTC\", \"The Cycles Token (XTC) is a cycles ledger canister that provides users with a “wrapped/tokenized” version of cycles (XTC) that can be held with just a Principal ID (no need for a Cycles Wallet), and that also includes all the same developer features and functions (calls) as the Cycles Wallet (built into the XTC token itself).\")"
```

You might have noticed that the length of this new paragraph is much more than the previous description. Registry canister has a limit for the maximum length of descriptions and it is set to be 1200 characters.

Now if we decide to ask the registry for the information associated with the "XTC" canister, we should use the `get_info` method:

```sh
$ dfx canister call registry get_info "(\"XTC\")"
(
  opt record {
    idl = opt "Not IDL, but the method works.";
    url = opt "https://dank.ooo/";
    description = opt "The Cycles Token (XTC) is a cycles ledger canister that provides users with a “wrapped/tokenized” version of cycles (XTC) that can be held with just a Principal ID (no need for a Cycles Wallet), and that also includes all the same developer features and functions (calls) as the Cycles Wallet (built into the XTC token itself).";
    version = 1;
    logo_url = opt "https://github.com/Psychedelic/dank";
    principal_id = principal "aanaa-xaaaa-aaaah-aaeiq-cai";
  },
)
```

It works! We added our canister and updated its information with only two steps. Really, it's that easy!
