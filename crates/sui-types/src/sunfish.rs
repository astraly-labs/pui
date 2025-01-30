use serde::{Deserialize, Serialize};

use crate::{base_types::SuiAddress, event::EventFilter};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SparseStatePredicates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<SuiAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EventFilter>>,
    // TODO(sunfish): Implement the packages filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<SuiAddress>>,
}
