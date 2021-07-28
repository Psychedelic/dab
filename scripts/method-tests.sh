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

# Step n. Stopping the DFX replica
echo
echo "${action} Stopping DFX"
echo
dfx stop
wait $!

echo
echo "${action} Exiting..."
exit
