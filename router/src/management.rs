use crate::common_types::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(Default)]
pub struct Admins(pub Vec<Principal>);

#[init]
pub fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

pub fn is_admin(account: &Principal) -> bool {
    ic::get::<Admins>().0.contains(account)
}

#[update]
fn set_admin(account: Principal) -> Result<(), OperationError> {
    if is_admin(&ic::caller()) {
        ic::get_mut::<Admins>().0.push(account);
        return Ok(());
    }
    Err(OperationError::NotAuthorized)
}
