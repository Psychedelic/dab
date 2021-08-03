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
printf "\n%s Deploying canisters on IC\n" "$action" 
dfx deploy

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

tput setab 1
printf "\nTESTING THE PROFILE CANISTER\n"
tput sgr0

# Step 1. Setting up our profile.
printf "\n%s Setting up our display name\n" "${action}"
dfx canister call profile set_display_name "(\"Peter Parker\")"

printf "\n%s Setting up our description\n" "${action}"
dfx canister call profile set_description "(\"Your friendly neighbourhood Spider-Man\")"

printf "\n%s Setting up our emoji\n" "${action}"
dfx canister call profile set_emoji "(\"🕷\")"

printf "\n%s Setting up our avatar\n" "${action}"
dfx canister call profile set_avatar "(\"https://upload.wikimedia.org/wikipedia/en/2/21/Web_of_Spider-Man_Vol_1_129-1.png\")"

printf "\n%s Setting up our banner\n" "${action}"
dfx canister call profile set_banner "(\"https://www.nme.com/wp-content/uploads/2020/09/Spider-Man-Suit.jpg\")"

printf "\n%s Getting our profile\n" "${action}"
dfx canister call profile get_profile "(\"null\")"

printf "\n%s Updating our profile with the set_profile method\n" "${action}"
dfx canister call profile set_profile "(record {display_name= \"Barry Allen\"; description= \"The fastest man alive.\"; emoji= \"⚡️\"; avatar= \"https://upload.wikimedia.org/wikipedia/en/3/3b/Flash_%28Barry_Allen_circa_2019%29.png\"; banner= \"https://static3.cbrimages.com/wordpress/wp-content/uploads/2020/07/the-flash-featured.jpg\"; version= 5})"

printf "\n%s Getting our profile\n" "${action}"
dfx canister call profile get_profile "(\"null\")"

# Step n. Stopping the DFX replica
printf "\n%s Stopping DFX.\n" "${action}"
dfx stop
sleep 1

printf "\n%s Exiting.\n" "${action}"
exit
