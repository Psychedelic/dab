use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;

#[query]
fn name() -> String {
    String::from("NFT Registry Canister")
}
