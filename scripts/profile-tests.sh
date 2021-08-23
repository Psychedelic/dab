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
dfx deploy profile

tput setab 1
printf "\nTESTING THE PRIVATE ADDRESS BOOK\n"
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