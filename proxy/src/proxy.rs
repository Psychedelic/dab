use ic_kit::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;

use crate::common_types::*;
use crate::history::*;
use crate::management::*;
use crate::trusted_sources::*;

#[init]
pub fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

#[query]
pub fn name() -> String {
    String::from("Proxy Canister")
}

#[update]
pub fn add_trusted_source(trusted_source: AddTrustedSourceInput) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let aux = trusted_source.clone();

    let db = ic::get_mut::<TrustedSources>();
    db.add(trusted_source)?;

    ic::get_mut::<History>().store_trusted_source_addition_event(aux);

    return Ok(());
}

#[query]
pub fn get_trusted_source(principal_id: Principal) -> Option<&'static TrustedSource> {
    let db = ic::get_mut::<TrustedSources>();
    return db.get(&principal_id);
}

#[query]
pub fn get_trusted_sources() -> Vec<&'static TrustedSource> {
    let db = ic::get_mut::<TrustedSources>();
    return db.get_all();
}

#[update]
pub fn remove_trusted_source(principal_id: Principal) -> Result<(), OperationError> {
    if is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<TrustedSources>();

    db.remove(&principal_id)?;

    ic::get_mut::<History>().store_trusted_source_deletion_event(principal_id);

    return Ok(());
}

#[update]
pub async fn add(
    canister_id: Principal,
    metadata: AddCanisterMetadataInput,
) -> Result<(), OperationError> {
    if !ic::get::<TrustedSources>().has_access_to_registry(&ic::caller(), &canister_id) {
        return Err(OperationError::NotAuthorized);
    }

    let add_registry_input = CanisterMetadata {
        name: metadata.name,
        description: metadata.description,
        thumbnail: metadata.thumbnail,
        frontend: metadata.frontend,
        principal_id: metadata.principal_id,
        details: metadata.details.clone(),
        submitter: ic::caller(),
        last_updated_by: ic::caller(),
        last_updated_at: ic::time(),
    };

    let _add_response: (Option<String>,) =
        ic::call(canister_id, "add", (add_registry_input.clone(),))
            .await
            .unwrap();

    ic::get_mut::<History>().store_addition_event(canister_id, &add_registry_input);

    return Ok(());
}

#[update]
pub async fn remove(canister_id: Principal, registry_id: Principal) -> Result<(), OperationError> {
    if !ic::get::<TrustedSources>().has_access_to_registry(&ic::caller(), &canister_id) {
        return Err(OperationError::NotAuthorized);
    }

    let _remove_response: (Option<String>,) = ic::call(canister_id, "remove", (registry_id,))
        .await
        .unwrap();

    ic::get_mut::<History>().store_deletion_event(canister_id, registry_id);
    return Ok(());
}

#[update]
pub async fn get_all(canister_id: Principal) -> Vec<CanisterMetadata> {
    let get_all_response: (Vec<CanisterMetadata>,) =
        ic::call(canister_id, "get_all", ()).await.unwrap();
    return get_all_response.0;
}

#[update]
pub async fn get(canister_id: Principal, registry_id: Principal) -> CanisterMetadata {
    let get_response: (CanisterMetadata,) =
        ic::call(canister_id, "get", (registry_id,)).await.unwrap();
    return get_response.0;
}

#[query]
pub fn get_history() -> Vec<Event> {
    ic::get::<History>().get_all()
}
