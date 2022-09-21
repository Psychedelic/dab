# Proxy Canister
The gateway for trusted sources to manage DAB registries!
Mainnet canister id: `tdc6j-iiaaa-aaaah-abj2q-cai`

## Examples

### Add an NFT entry to DAB nft registry

`dfx canister --network=ic call tdc6j-iiaaa-aaaah-abj2q-cai add "(principal \"ctqxp-yyaaa-aaaah-abbda-cai\", record { name = \"NFT collection name\"; description = \"NFT collection description\"; thumbnail = \"NFT collection thumbnail\"; principal_id = principal \"\"; details = vec { record { \"standard\"; variant = { Text = \"DIP721\" } } } })"`