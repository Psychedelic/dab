# DAB's Router Canister

The router canister is a canister registry that contains the list of all DAB registries.

### NOTE:
The `details` field in this canister contains the **verified** status of the entry: `Vec<(String, DetailValue::True)> || Vec<(String, DetailValue::False)>`