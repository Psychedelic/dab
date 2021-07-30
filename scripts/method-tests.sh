#!/bin/sh

redBG=$(tput setab 1)
reset=$(tput sgr0)
action=$(echo "${redBG}"[ACTION]"${reset}")

# Step 0. Restarting DFX
clear
echo
echo "${action} Restarting DFX"
echo
cd ..
dfx stop & rm -rf .dfx & dfx start --background --clean

# Step 1. Deploying DAB on IC
echo
echo "${action} Deploying DAB on IC"
echo
dfx deploy

# Step 2. Adding DAB's principal ID to the private address book using add_address method
echo
echo "${action} Adding DAB's principal ID to the BTree Map"
echo
DAB=$(dfx canister id dab)
dfx canister call dab add_address "(\"DAB\", principal \"$DAB\")"

# Step 3. Getting the principal ID associated with the name "DAB" using get_address method
echo
echo "${action} Checking if DAB has been added to the map"
echo
dfx canister call dab get_address "(\"DAB\")"

# Step 4. Adding another canister so that we can check the get_all method.
echo
echo "${action} Adding another address and calling the get_all method."
echo
dfx canister call dab add_address "(\"XTC\", principal \"aanaa-xaaaa-aaaah-aaeiq-cai\")"
dfx canister call dab get_all

# Step 5. Creating another identity and adding addresses.
echo
echo "${action} Creating a new identity to make another private address book."
echo
dfx identity new jack
dfx identity use jack

# Step 6. Asking DAB for another user's private address book.
echo
echo "${action} Checking if our new identity (Jack) can access other user's private data."
echo
dfx canister call dab get_address "(\"DAB\")"

# Step 7. Adding a new address to Jack's address book.
echo
echo "${action} Adding DAB's address to Jack's address book."
echo
dfx canister call dab add_address "(\"DAB\", principal \"$DAB\")"
dfx canister call dab get_address "(\"DAB\")"

# Step 8. Switching back to the other user and removing XTC
echo
echo "${action} Switching back to the previous user and removing the XTC canister address."
echo
dfx identity use default
dfx canister call dab remove_address "(\"XTC\")"
dfx canister call dab get_all

# Step n. Stopping the DFX replica
echo
echo "${action} Stopping DFX"
echo
dfx stop
wait $!

echo
echo "${action} Exiting..."
exit
