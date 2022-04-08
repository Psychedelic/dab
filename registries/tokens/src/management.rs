use ic_kit::Principal;
use ic_kit::ic;
use ic_kit::macros::*;

use crate::common_types::OperationError;

pub struct Admins(pub Vec<Principal>);

impl Default for Admins {
    fn default() -> Self {
        panic!()
    }
}

pub fn is_admin(account: &Principal) -> bool {
    ic::get::<Admins>().0.contains(account)
}

#[update]
pub fn add_admin(new_admin: Principal) -> Result<(), OperationError> {
    if is_admin(&ic::caller()) {
        ic::get_mut::<Admins>().0.push(new_admin);
        return Ok(());
    }
    Err(OperationError::NotAuthorized)
}