use ic_kit::candid::Principal;
use ic_kit::ic::*;

pub fn cover_check(canister: Principal) -> bool {
    let cover_id = Principal::from_text("iftvq-niaaa-aaaai-qasga-cai");
    let cover_response = match ic::call(cover_id, String::from("getVerificationByCanisterId"), ((),)).await {
        Ok((x,)) => x,
        Err((_code, msg)) => {
            return Err(Failure::InterCanisterCall(msg));
        }
    };
}
