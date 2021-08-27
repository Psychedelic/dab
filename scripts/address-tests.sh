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
dfx deploy address

tput setab 1
printf "\nTESTING THE PRIVATE ADDRESS BOOK\n"
tput sgr0

# Step 2. Adding Address Book's principal ID to the private address book using add_address method
printf "\n%s Adding address' principal ID to the BTree Map\n" "${action}"
AddressBook=$(dfx canister id address)
dfx canister call address add_address "(\"address_book\", principal \"$AddressBook\")"

# Step 3. Getting the principal ID associated with the name "address_book" using get_address method
printf "\n%s Checking if address_book has been added to the map\n" "${action}"
dfx canister call address get_address "(\"address_book\")"

# Step 4. Adding another canister so that we can check the get_all method.
printf "\n%s Adding another address and calling the get_all method.\n" "${action}"
dfx canister call address add_address "(\"XTC\", principal \"aanaa-xaaaa-aaaah-aaeiq-cai\")"
dfx canister call address get_all

# Step 5. Creating another identity and adding addresses.
printf "\n%s Creating a new identity to make another private address book.\n" "${action}"
dfx identity new jack
dfx identity use jack

# Step 6. Asking DAB for another user's private address book.
printf "\n%s Checking if our new identity (Jack) can access other user's private data.\n" "${action}"
dfx canister call address get_address "(\"address_book\")"

# Step 7. Adding a new address to Jack's address book.
printf "\n%s Adding profile's address to Jack's address book.\n" "${action}"
profile=$(dfx canister id profile)
dfx canister call address add_address "(\"profile\", principal \"$profile\")"
dfx canister call address get_address "(\"profile\")"

# Step 8. Switching back to the other user and removing XTC
printf "\n%s Switching back to the previous user and removing the XTC canister address.\n" "${action}"
dfx identity use default
dfx canister call address remove_address "(\"XTC\")"
dfx canister call address get_all

# Step n. Stopping the DFX replica
printf "\n%s Stopping DFX.\n" "${action}"
dfx stop
sleep 1

printf "\n%s Exiting.\n" "${action}"
exit
