use crate::execution_cache::TransactionCacheRead;
use std::sync::Arc;
use sui_types::{
    base_types::SuiAddress,
    effects::{TransactionEffects, TransactionEffectsAPI},
    sunfish::SparseStatePredicates,
    transaction::{TransactionDataAPI, TransactionKind, VerifiedTransaction},
};

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
                && match_events(transaction_cache_reader, effects, predicates)
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

/// Check if the transaction provided matches the events predicates.
fn match_events(
    transaction_cache_reader: &Arc<dyn TransactionCacheRead>,
    effects: &TransactionEffects,
    predicates: &SparseStatePredicates,
) -> bool {
    // If there are no event filters, we just return true
    let Some(ref _event_filters) = predicates.events else {
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
        // TODO: match on the event and return early if one matches
        tracing::info!("Event found: {:?}", event);
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

// ========== Old Event Filtering ==========

// pub fn tx_matches_event_filters<'r>(
//     layout_resolver: &mut Box<dyn LayoutResolver + 'r>,
//     cache_reader: &dyn TransactionCacheRead,
//     tx: &ExecutionData,
//     event_filters: &[EventFilter],
// ) -> bool {
//     let TransactionData::V1(d) = &tx.transaction.data().intent_message().value;

//     // Skip filtering for non-programmable transactions and zero address
//     !matches!(d.kind, TransactionKind::ProgrammableTransaction(_)) || d.sender == SuiAddress::ZERO ||
//     // Event filtering logic
//     tx.effects.events_digest()
//         .and_then(|digest| cache_reader.get_events(&digest))
//         .map_or(false, |events| {
//             events.data.iter().enumerate().any(|(i, event)| {
//                 let Ok(layout) = layout_resolver.get_annotated_layout(&event.type_) else {
//                     return false;
//                 };

//                 SuiEvent::try_from(
//                     event.clone(),
//                     tx.transaction.digest().clone(),
//                     i.try_into().unwrap(),
//                     None,
//                     layout,
//                 )
//                 .map(|sui_event| event_filters.iter().all(|f| f.matches(&sui_event)))
//                 .unwrap_or(false)
//             })
//         })
// }
