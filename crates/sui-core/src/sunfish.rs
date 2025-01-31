use std::{str::FromStr, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use sui_types::{
    base_types::SuiAddress,
    digests::TransactionDigest,
    effects::{TransactionEffects, TransactionEffectsAPI},
    event::event_matches_filters,
    id::ID,
    sunfish::SparseStatePredicates,
    transaction::{TransactionDataAPI, TransactionKind, VerifiedTransaction},
};

use crate::execution_cache::TransactionCacheRead;

pub fn matches_sparse_predicates(
    transaction_cache_reader: &Arc<dyn TransactionCacheRead>,
    tx: &Arc<VerifiedTransaction>,
    effects: &TransactionEffects,
    predicates: &SparseStatePredicates,
) -> bool {
    match tx.data().intent_message().value.kind() {
        TransactionKind::ProgrammableTransaction(_) => {
            // 1. Filter based on tx sender
            (match_addresses(tx, predicates)
            // 2. Filter based on tx events
                && match_events(transaction_cache_reader, tx, effects, predicates)
            // 3. Filter based on tx package
                && match_package(tx, effects, predicates)
            )
            // 4. Some programmable transactions are made by the address ZERO.
            // They must be included no matter the filter.
                || tx.sender_address() == SuiAddress::ZERO
        }
        // Include all non-programmable transactions
        _ => true,
    }
}

/// Check if the transaction provided matches the addresses predicates.
fn match_addresses(tx: &Arc<VerifiedTransaction>, predicates: &SparseStatePredicates) -> bool {
    if let Some(allowed_addresses) = &predicates.addresses {
        allowed_addresses.contains(&tx.sender_address())
    } else {
        true
    }
}

// For now, we did not find a way to decode arbitrary events.
// We have a list of events that we know - we define them at the end of this file
// so we're able to decode any Vec<u8> encountered.
/// Check if the transaction provided matches the events predicates.
fn match_events(
    transaction_cache_reader: &Arc<dyn TransactionCacheRead>,
    tx: &Arc<VerifiedTransaction>,
    effects: &TransactionEffects,
    predicates: &SparseStatePredicates,
) -> bool {
    // If there are no event filters, we just return true
    let Some(ref event_filters) = predicates.events else {
        return true;
    };

    // No events digest means no events to match
    let Some(events_digest) = effects.events_digest() else {
        return false;
    };

    // Get events from cache
    let Some(events) = transaction_cache_reader.get_events(events_digest) else {
        return false;
    };

    for event in events.data {
        let event_data = &event.contents;
        if let Some(event_data) = try_decode_event_to_json(event_data) {
            tracing::info!(
                "[SUNFISH] GOT EVENT {:?} FROM TX {}",
                &event_data,
                tx.digest()
            );

            if event_matches_filters(&event_filters, &event, &event_data, tx.digest()) {
                tracing::info!("[SUNFISH] ✅ MATCHED :D");
                return true;
            }
        }
    }

    false
}

// TODO(sunfish): Implement a package-based filter.
fn match_package(
    _tx: &Arc<VerifiedTransaction>,
    _effects: &TransactionEffects,
    _predicates: &SparseStatePredicates,
) -> bool {
    true
}

// ================ Pui Oracle Structs ================

// For now, we did not find a way to decode arbitrary events.
// We have a list of events that we know - we define them at the end of this file
// so we're able to decode any Vec<u8> encountered.

// TODO: Remove them once we have a find a way to decode arbitrary events bytes
// into their type.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceStorageCreatedEvent {
    pub storage_id: ID,
    pub publisher: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceUpdatedEvent {
    pub storage_id: ID,
    pub publisher: String,
    pub asset_type: String,
    pub pair_id: String,
    pub source: String,
    pub price: u128,
    pub decimals: u8,
    pub volume: u128,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistryCreatedEvent {
    pub owner: SuiAddress,
    pub registry: SuiAddress,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublisherRegisteredEvent {
    pub publisher_name: String,
    pub publisher_address: SuiAddress,
}

/// Try decoding the event data against all known event types and convert to JSON
fn try_decode_event_to_json(data: &[u8]) -> Option<Value> {
    if let Ok(evt) = bcs::from_bytes::<PriceStorageCreatedEvent>(data) {
        return serde_json::to_value(evt).ok();
    }
    if let Ok(evt) = bcs::from_bytes::<PriceUpdatedEvent>(data) {
        return serde_json::to_value(evt).ok();
    }
    if let Ok(evt) = bcs::from_bytes::<RegistryCreatedEvent>(data) {
        return serde_json::to_value(evt).ok();
    }
    if let Ok(evt) = bcs::from_bytes::<PublisherRegisteredEvent>(data) {
        return serde_json::to_value(evt).ok();
    }

    None
}
