use ic_kit::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;
use std::collections::HashMap;

use crate::common_types::*;
use crate::management::*;

#[derive(Default)]
pub struct TrustedSources(HashMap<Principal, TrustedSource>);

impl TrustedSources {
    pub fn archive(&mut self) -> Vec<(Principal, TrustedSource)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, TrustedSource)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(&mut self, trusted_source: AddTrustedSourceInput) -> Result<(), OperationError> {
        let new_trusted_source = TrustedSource {
            added_by: ic::caller(),
            principal_id: trusted_source.principal_id,
            accessible_registries: trusted_source.accessible_registries,
            last_call: 0,
        };

        self.0
            .insert(trusted_source.principal_id, new_trusted_source);

        return Ok(());
    }

    pub fn get(&self, principal_id: &Principal) -> Option<&TrustedSource> {
        self.0.get(principal_id)
    }

    pub fn get_all(&self) -> Vec<&TrustedSource> {
        self.0.values().collect()
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if !self.0.contains_key(principal_id) {
            return Err(OperationError::NonExistentItem);
        }

        self.0.remove(principal_id);
        return Ok(());
    }

    pub fn has_access_to_registry(
        &self,
        principal_id: &Principal,
        registry_principal_id: &Principal,
    ) -> bool {
        match self.0.get(principal_id) {
            Some(v) if v.accessible_registries.contains(registry_principal_id) => true,
            _ => false,
        }
    }
}
