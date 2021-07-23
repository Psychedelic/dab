use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use ic_cdk::*;

type Key = (Principal, String);

#[query]
fn name() -> String {
    String::from("Dfinity Address Book")
}

#[update]
async fn get_canister(key: String) -> Option<Principal> {
    let pointer: Key = (caller(), key);
    // let data = storage::get::<Data>();
    // return data.0.get(pointer).await;
}

#[update]
async fn add_canister(key: String, value: Principal) {
    let pointer: Key = (caller(), key);
    // let data = storage::get_mut::<Data>();
    // data.0.insert(pointer, value).await;
}

#[update]
async fn remove_canister(key: String) {
    let pointer: Key = (caller(), key);
}

#[update]
async fn remove_all() {
}

#[update]
async fn get_all() {
}
