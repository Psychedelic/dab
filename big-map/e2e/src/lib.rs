use big_map;
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;

struct Data(big_map::BigMap<u32, u32>);
impl Default for Data {
    fn default() -> Self {
        Self(big_map::BigMap::new(5, 2 * 1024 * 1024))
    }
}

#[update]
async fn set(key: u32, value: u32) {
    let data = storage::get_mut::<Data>();
    data.0.insert(key, value).await;
}

#[query]
async fn get(key: u32) -> Option<u32> {
    let data = storage::get::<Data>();
    data.0.get(key).await
}
