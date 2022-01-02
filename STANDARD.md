# DAB registries standard
[This candid](https://github.com/Psychedelic/dab/blob/main/standard.did) and this markdown file contain the information about DAB registries standard. It's essential that every registry follows this standard in order to be added to DAB's router canister.

## Types
There are three types that should be implemented in registries:

- [Canister Metadata Struct (record)](https://github.com/Psychedelic/dab/blob/fa4019d0fc3f723e5713fefd737f394dff8fcd29/standard.did#L13): What you are going to store and how you are going to store it, will be defined here. The metadata type illustrates what metadata fields the registry is going to have and how they are going to be presented.
- [Error Enum (variant)](https://github.com/Psychedelic/dab/blob/fa4019d0fc3f723e5713fefd737f394dff8fcd29/standard.did#L21): This variant contains the list of possible errors that the registry might return as a result.
- [Response Enum (variant)](https://github.com/Psychedelic/dab/blob/fa4019d0fc3f723e5713fefd737f394dff8fcd29/standard.did#L28): This variant covers the two possible outcomes of an operation: success or failure. The success of an operation will be determined by the `Ok` response and the failure by the `Err` response which will have a value of `Error` variant.
