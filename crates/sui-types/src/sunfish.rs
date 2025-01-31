use serde::{Deserialize, Serialize};

use crate::{base_types::SuiAddress, event::EventFilter};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SparseStatePredicates {
    pub addresses: Option<Vec<SuiAddress>>,
    pub events: Option<Vec<EventFilter>>,
    pub packages: Option<Vec<SuiAddress>>,
}
