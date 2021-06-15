use ic_cdk::api;
use ic_cdk::export::candid::{CandidType, Nat};
use ic_cdk::export::Principal;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub struct CanisterSettings {
    pub controller: Option<Principal>,
    pub compute_allocation: Option<Nat>,
    pub memory_allocation: Option<Nat>,
    pub freezing_threshold: Option<Nat>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CreateCanisterArgs {
    pub cycles: u64,
    pub settings: CanisterSettings,
}

#[derive(CandidType, Deserialize)]
pub struct CreateResult {
    pub canister_id: Principal,
}

pub async fn create_canister_call(args: CreateCanisterArgs) -> Result<CreateResult, String> {
    #[derive(CandidType)]
    struct In {
        settings: Option<CanisterSettings>,
    }

    let in_arg = In {
        settings: Some(args.settings),
    };

    let (create_result,): (CreateResult,) = match api::call::call_with_payment(
        Principal::management_canister(),
        "create_canister",
        (in_arg,),
        args.cycles as i64,
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };

    Ok(create_result)
}

pub async fn install_wasm(canister_id: &Principal, wasm_module: Vec<u8>) -> Result<(), String> {
    // Install Wasm
    #[derive(CandidType, Deserialize)]
    enum InstallMode {
        #[serde(rename = "install")]
        Install,
        #[serde(rename = "reinstall")]
        Reinstall,
        #[serde(rename = "upgrade")]
        Upgrade,
    }

    #[derive(CandidType, Deserialize)]
    struct CanisterInstall {
        mode: InstallMode,
        canister_id: Principal,
        #[serde(with = "serde_bytes")]
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
    }

    let install_config = CanisterInstall {
        mode: InstallMode::Install,
        canister_id: canister_id.clone(),
        wasm_module: wasm_module.clone(),
        arg: b" ".to_vec(),
    };

    match api::call::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };

    Ok(())
}
