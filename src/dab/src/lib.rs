use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use ic_cdk::*;
use big_map;

struct Data(big_map::BigMap<String, Option<Principal>>);
impl Default for Data {
    fn default() -> Self {
        Self(big_map::BigMap::new(5, 2 * 1024 * 1024))
    }
}

#[update]
fn name() {
    String::from("Dfinity Address Book");
}

#[update]
async fn add_canister(key: String, value: Option<Principal>) -> () {
    let data = storage::get_mut::<Data>();
    data.0.insert(key, value).await;
}

#[query]
async fn get_canister(key: String) -> Option<Principal> {
    let data = storage::get::<Data>();
    return data.0.get(key).await;
}