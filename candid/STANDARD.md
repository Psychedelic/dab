# DAB registries standard

Every registry that intends to be added to DAB's router canister and be used under the lable of DAB, should follow DAB registries standard. **No contradiction** should be found between the registry's candid and the standard. The standard candid is located [here](standard.did).

## Types

There are two enumerations and one structure that should be implemented in a registry:

- [Canister Metadata Struct (record)](standard.did#L13): This struct defines what information you store in your registry about every canister. The canister metadata **can include other fields that are not included in the registries standard**. Use the `details` field with a vector of data if needed.
- [Error Enum (variant)](standard.did#L21): This variant contains the list of possible errors that a registry might return as response.
- [Response Enum (variant)](standard.did#L28): This variant covers the two possible outcomes of an operation: success or failure. The success of an operation will be determined by the `Ok` response and the failure by the `Err` response which will have a value of `Error` variant.

## Methods

Registries will contain different methods, including but not limited to `name`, `get`, `add`, and `remove` methods. Other methods are optional and they can be designed by the developer.

- [Name() -> (text) query](standard.did#L35): This method returns the name of the registry.
- [get(principal) -> (opt metadata) query](standard.did#L37): This method returns the metadata of the requested principal if it has already been added to the registry.
- [add(principal, metadata) -> (response)](standard.did#L38): This method adds a new canister to the registry with its metadata. Only canister admins should be able to call this method.
- [remove(principal) -> (response)](standard.did#L39): This method removes the metadata of a principal from the registry.

## The template registry canister

The template registry canister is a sample canister written in rust that follows the DAB registries standard. The canister directory is located [here](../template_registry). You can take inspiration from the template registry canister.
