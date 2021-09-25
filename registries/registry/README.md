# Registry Canister Interaction Guide

You can interact with the registry canister using the methods it provides. You can find all of the methods and their usage in the
shell script located [here](https://github.com/Psychedelic/dab/blob/main/scripts/registry-tests.sh).

## Registry Canister Methods

The registry canister has three public methods. You can find the details of these methods in the [candid file](https://github.com/Psychedelic/dab/blob/main/candid/registry.did).

| Method Name        | Description                                                                                           |
| -----------        | -----------                                                                                           |
| name               | This method return the name of the canister for health-check                                          |
| get_info           | This method returns the metadata associated with the given canister principal IDs                     |
| update_canister    | This method makes an inter-canister call to the principal ID passed and updates the canister metadata |

## How to use them?

In this section we take a look at the methods that the registry canister offers and try to add our canister to the registry. Let's start by learning the structure of the metadata.

Registry canister stores the following information about every canister that has been added to it:

- Name
- Description
- Front-end URL
- Logo URL
- Version of the metadata

The version of the metadata helps developers identify new updates and changes to the metadata. Version increments by one, every time the metadata receives an update. Let's add the XTC canister to the registry as an example to see how it works. For that to happen we need to add a snippet of code to our canister. This snippet returns the metadata of the canister for the time being. The reason we ask you to add this code is that DFX currently doesn't let us find out who is the controller of each canister so it is necessary to verify with this method.

The method is named `dab_registry` and you can just copy and paste it in to your code:

```rust
pub struct InputCanisterMetadata {
    name: String,
    description: String,
    url: String,
    logo_url: String,
}

fn dab_registry() -> InputCanisterMetadata {
  return InputCanisterMetadata {
    name: String::from("Canister Name"),
    description: String::from("Canister Description"),
    url: String::from("https://frontend_url.com"),
    logo_url: String::from("https://logo_url.com"),
    };
}
```

Notice that we didn't include the version field. That's because the registry itself controls the version and you do not have to worry about that! Next step is adding this snippet to your candid and after that we will call the registry and the registry takes care of the rest:

```candid
type input_canister_metadata = record {
    name: text;
    description: text;
    url: text;
    logo_url: text;
};

service : {
  "dab_registry" : () -> (input_canister_metadata); 
}
```

Let's add the canister to the registry using the `update_canister` method:

```sh
dfx canister --network=ic call qxtlu-aiaaa-aaaah-aaupq-cai update_canister "(principal \"YOUR_CANISTER_ID\")" 
```

Now that we have added this canister to the registry, we can ask for its metadata:

```sh
$ dfx canister --network=ic call qxtlu-aiaaa-aaaah-aaupq-cai get_info "(vec {principal \"YOUR_CANISTER_ID\"})"
(
  opt record {
    name = "Canister Name";
    description = "Canister Description";
    url = "https://frontend_url.com";
    logo_url = "https://logo_url.com";
    version = 1;
  },
)
```

It works! We have added our canister and updated its information. You can also ask for other canisters' metadata with passing their principal ID along:

```sh
dfx canister --network=ic call qxtlu-aiaaa-aaaah-aaupq-cai get_info "(vec {principal \"CANISTER_ID_ONE\"; principal \"CANISTER_ID_TWO\"})"
```

