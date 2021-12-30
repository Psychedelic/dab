use ic_kit::candid::CandidType;

#[derive(CandidType)]
pub enum Error {
    CanisterAlreadyExists,
    BadParameters,
    NonExistantCanister,
    NotAuthorized
}