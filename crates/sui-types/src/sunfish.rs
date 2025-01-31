use serde::{Deserialize, Serialize};

use crate::{base_types::SuiAddress, event::EventFilter};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SparseStatePredicates {
    pub addresses: Option<Vec<SuiAddress>>,
    // TODO(sunfish): Implement the events filter - see filter_content_by_events
    pub events: Option<Vec<EventFilter>>,
    // TODO(sunfish): Implement the packages filter
    pub packages: Option<Vec<SuiAddress>>,
}

impl SparseStatePredicates {
    /// Returns true if the given address matches the sparse state predicates
    pub fn matches_address(&self, address: &SuiAddress) -> bool {
        if let Some(allowed_addresses) = &self.addresses {
            allowed_addresses.contains(address)
        } else {
            true
        }
    }
}
