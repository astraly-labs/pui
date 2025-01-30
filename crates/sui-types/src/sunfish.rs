use serde::{Deserialize, Serialize};

use crate::{base_types::SuiAddress, event::EventFilter};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SparseStatePredicates {
    pub addresses: Option<Vec<SuiAddress>>,
    pub events: Option<Vec<EventFilter>>,
    // TODO(sunfish): Implement the packages filter
    pub packages: Option<Vec<SuiAddress>>,
}

impl SparseStatePredicates {
    /// Filter transactions based on the sparse state predicates
    pub fn filter_transactions<T>(
        &self,
        transactions: &mut Vec<T>,
        tx_filter: impl Fn(&T) -> bool,
    ) {
        if let Some(addresses) = &self.addresses {
            transactions.retain(|tx| tx_filter(tx));
        }
    }

    /// Returns true if the given address matches the sparse state predicates
    pub fn matches_address(&self, address: &SuiAddress) -> bool {
        if let Some(addresses) = &self.addresses {
            addresses.contains(address)
        } else {
            true // If no address filter, match everything
        }
    }
}
