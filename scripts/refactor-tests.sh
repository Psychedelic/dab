#!/bin/sh

redBG=$(tput setab 10)
blackFG=$(tput setaf 0)
reset=$(tput sgr0)
action=$(echo "${redBG}${blackFG}"[ACTION]"${reset}")

# Step 0. Restarting DFX
clear
printf "\n%s Restarting DFX\n" "$action"
cd ..
ps -ef | grep /usr/bin/dfx | grep -v grep | awk '{print $2}' | xargs kill & rm -rf .dfx & dfx start --background --clean --emulator

# Step 1. Deploying canisters on IC
printf "\n%s Deploying the latest version of all canisters on the local replica\n" "$action" 
printf "\n%s NFT Canister: \n" "${action}"
dfx deploy nft --no-wallet
printf "\n%s Registry Canister: \n" "${action}"
dfx deploy registry --no-wallet
printf "\n%s Tokens Canister: \n" "${action}"
dfx deploy tokens --no-wallet
printf "\n%s Router Canister: \n" "${action}"
dfx deploy router --no-wallet
printf "\n%s Template Registry Canister: \n" "${action}"
dfx deploy template_registry --no-wallet

printf "\n%s Doing health-check on all canisters\n" "${action}"
NFT=$(dfx canister id nft)
REGISTRY=$(dfx canister id registry)
TOKENS=$(dfx canister id tokens)
ROUTER=$(dfx canister id router)
TEMPLATE=$(dfx canister id template_registry)
dfx canister call "$NFT" name
dfx canister call "$REGISTRY" name
dfx canister call "$TOKENS" name
dfx canister call "$ROUTER" name
dfx canister call "$TEMPLATE" name

printf "\n%s Adding new entries to all of the canisters\n" "${action}"
printf "\n%s NFT Canister: \n" "${action}"
dfx canister call "$NFT" add "(record {name= \"Test NFT\"; description= \"Description test\"; thumbnail= \"https://fleek.co\"; frontend= opt \"https://fleek.ooo\"; principal_id= principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\"; details= vec { record {\"standard\"; variant { Text= \"DIP721\" } } } } )"
printf "\n%s Registry Canister: \n" "${action}"
dfx canister call "$REGISTRY" add "(record {name= \"Test Canister\"; description= \"Description test\"; thumbnail= \"https://fleek.co\"; frontend= opt \"https://fleek.ooo\"; principal_id= principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\"; details= vec { record {\"category\"; variant { Text= \"service\" } } } } )"
printf "\n%s Tokens Canister: \n" "${action}"
dfx canister call "$TOKENS" add "(record {name= \"Test Tokens Canister\"; description= \"Description test\"; thumbnail= \"https://fleek.co\"; frontend= opt \"https://fleek.ooo\"; principal_id= principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\"; details= vec { record {\"symbol\"; variant { Text= \"XTC\" } }; record {\"standard\"; variant { Text= \"DIP721\" } }; record {\"total_supply\"; variant { U64= 10000 } }; record {\"verified\"; variant { True } } } } )"
printf "\n%s Router Canister: \n" "${action}"
dfx canister call "$ROUTER" add "(record {name= \"Test Registry\"; description= \"Description test\"; thumbnail= \"https://fleek.co\"; frontend= opt \"https://fleek.ooo\"; principal_id= principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\"; details= vec { record {\"verified\"; variant { True } } } } )"
printf "\n%s Template Registry Canister: \n" "${action}"
dfx canister call "$TEMPLATE" add "(record {name= \"Example Canister\"; description= \"Description test\"; thumbnail= \"https://fleek.co\"; frontend= opt \"https://fleek.ooo\"; principal_id= principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\"; details= vec {} } )"

printf "\n%s Getting entries from all of the canisters\n" "${action}"
printf "\n%s NFT Canister: \n" "${action}"
dfx canister call "$NFT" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Registry Canister: \n" "${action}"
dfx canister call "$REGISTRY" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Tokens Canister: \n" "${action}"
dfx canister call "$TOKENS" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Router Canister: \n" "${action}"
dfx canister call "$ROUTER" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Template Registry Canister: \n" "${action}"
dfx canister call "$TEMPLATE" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"

printf "\n%s Removing entries from all of the canisters\n" "${action}"
printf "\n%s NFT Canister: \n" "${action}"
dfx canister call "$NFT" remove "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Registry Canister: \n" "${action}"
dfx canister call "$REGISTRY" remove "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Tokens Canister: \n" "${action}"
dfx canister call "$TOKENS" remove "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Router Canister: \n" "${action}"
dfx canister call "$ROUTER" remove "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Template Registry Canister: \n" "${action}"
dfx canister call "$TEMPLATE" remove "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"

printf "\n%s Getting entries from all of the canisters\n" "${action}"
printf "\n%s NFT Canister [EXPECT NULL]: \n" "${action}"
dfx canister call "$NFT" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Registry Canister [EXPECT NULL]: \n" "${action}"
dfx canister call "$REGISTRY" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Tokens Canister [EXPECT NULL]: \n" "${action}"
dfx canister call "$TOKENS" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Router Canister [EXPECT NULL]: \n" "${action}"
dfx canister call "$ROUTER" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"
printf "\n%s Template Registry Canister [EXPECT NULL]: \n" "${action}"
dfx canister call "$TEMPLATE" get "(principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\")"

