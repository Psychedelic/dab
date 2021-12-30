use crate::common_types::*;
use ic_kit::*;
use ic_kit::macros::*;

#[derive(Default)]
pub struct Admins(pub Vec<Principal>);

#[init]
fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

pub fn is_admin(account: &Principal) -> bool {
    ic::get::<Admins>().0.contains(account)
}

#[update]
fn set_admin(account: Principal) -> Result<(), Error> {
    if is_admin(&ic::caller()) {
        ic::get_mut::<Admins>().0.push(account);
        return Ok(());
    }
    Err(Error::NotAuthorized)
}