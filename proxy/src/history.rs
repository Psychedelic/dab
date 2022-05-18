use crate::common_types::*;
use ic_kit::Principal;
use ic_kit::*;

#[derive(Default)]
pub struct History(pub Vec<Event>);

impl History {
    pub fn archive(&mut self) -> Vec<Event> {
        let map = std::mem::replace(&mut self.0, Vec::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<Event>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn store_event(&mut self, event: Event) {
        self.0.push(event);
    }

    pub fn store_addition_event(&mut self, registry: Principal, metadata: &CanisterMetadata) {
        let event = Event::Addition {
            time: ic::time(),
            by: ic::caller(),
            registry: registry,
            metadata: metadata.clone(),
        };

        self.store_event(event);
    }

    pub fn store_deletion_event(&mut self, registry: Principal, canister: Principal) {
        let event = Event::Deletion {
            time: ic::time(),
            by: ic::caller(),
            registry,
            canister,
        };

        self.store_event(event);
    }

    pub fn store_trusted_source_addition_event(&mut self, trusted_source: AddTrustedSourceInput) {
        let event = Event::TrustedSourceAddition {
            time: ic::time(),
            by: ic::caller(),
            trusted_source: trusted_source.principal_id,
            accessible_registries: trusted_source.accessible_registries.clone(),
        };

        self.store_event(event);
    }

    pub fn store_trusted_source_deletion_event(&mut self, trusted_source: Principal) {
        let event = Event::TrustedSourceDeletion {
            time: ic::time(),
            by: ic::caller(),
            trusted_source,
        };

        self.store_event(event);
    }

    pub fn get_all(&self) -> Vec<Event> {
        self.0.clone()
    }
}
