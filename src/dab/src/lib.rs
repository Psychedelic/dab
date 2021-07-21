use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use ic_cdk::*;

type Key = (Principal, String);

/** struct Data(big_map::BigMap<Key, Principal>);

impl Default for Data {
    fn default() -> Self {
        Self(big_map::BigMap::new(5, 2 * 1024 * 1024))
    }
}

#[query]
fn name() -> String {
    String::from("Dfinity Address Book")
}

#[update]
async fn add_canister(key: String, value: Principal) {
    let pointer: Key = (caller(), key);
    let data = storage::get_mut::<Data>();
    data.0.insert(pointer, value).await;
}

#[update]
async fn get_canister(key: String) -> Option<Principal> {
    let pointer: Key = (caller(), key);
    let data = storage::get::<Data>();
    return data.0.get(pointer).await;
}

#[update]
async fn remove_canister(key: String) {

}

#[update]
async fn remove_all(key: String) {

}

#[update]
async fn get_all(key: String) {

}

**/