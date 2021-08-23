#!/bin/sh

redBG=$(tput setab 10)
blackFG=$(tput setaf 0)
reset=$(tput sgr0)
action=$(echo "${redBG}${blackFG}"[ACTION]"${reset}")

# Step 0. Restarting DFX
clear
printf "\n%s Restarting DFX\n" "$action"
cd ..
dfx stop & rm -rf .dfx & dfx start --background --clean --emulator

# Step 1. Deploying canisters on IC
printf "\n%s Deploying the canister on IC\n" "$action" 
dfx deploy registry

tput setab 1
printf "\nTESTING THE REGISTRY\n"
tput sgr0

# Step 1. Adding a new canister
printf "\n%s Adding a new canister to the registry\n" "${action}"
dfx canister call registry add_canister "(\"XTC\", record {principal_id= principal \"aanaa-xaaaa-aaaah-aaeiq-cai\"; description= \"The Cycles Token (XTC) is Dank's first product.\"; url= \"https://dank.ooo\"; idl= null; version= 0})"

printf "\n%s Asking for XTC's info\n" "${action}"
dfx canister call registry get_info "(\"XTC\")"

printf "\n%s Updating the description\n" "${action}"
dfx canister call registry set_description "(\"XTC\", \"The Cycles Token (XTC) is a cycles ledger canister that provides users with a “wrapped/tokenized” version of cycles (XTC) that can be held with just a Principal ID (no need for a Cycles Wallet), and that also includes all the same developer features and functions (calls) as the Cycles Wallet (built into the XTC token itself).\")"

printf "\n%s Updating the URL\n" "${action}"
dfx canister call registry set_url "(\"XTC\", \"https://docs.dank.ooo/\")"

printf "\n%s Updating the IDL\n" "${action}"
dfx canister call registry set_idl "(\"XTC\", \"Not IDL, but the method works.\")"

printf "\n%s Asking for XTC's info\n" "${action}"
dfx canister call registry get_info "(\"XTC\")"

# Step n. Stopping the DFX replica
printf "\n%s Stopping DFX.\n" "${action}"
dfx stop
sleep 1

printf "\n%s Exiting.\n" "${action}"
exit