use std::sync::Arc;
use sui_types::{
    base_types::SuiAddress,
    effects::TransactionEffects,
    sunfish::SparseStatePredicates,
    transaction::{TransactionDataAPI, TransactionKind, VerifiedTransaction},
};

pub fn matches_sparse_predicates(
    tx: &Arc<VerifiedTransaction>,
    effects: &TransactionEffects,
    predicates: &SparseStatePredicates,
) -> bool {
    match tx.data().intent_message().value.kind() {
        TransactionKind::ProgrammableTransaction(_) => {
            // 1. Filter based on tx sender
            (match_addresses(tx, predicates)
            // 2. Filter based on tx events
                && match_events(tx, effects, predicates)
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
    _tx: &Arc<VerifiedTransaction>,
    _effects: &TransactionEffects,
    _predicates: &SparseStatePredicates,
) -> bool {
    true
}

// TODO(sunfish): Implement a package-based filter.
fn match_package(
    _tx: &Arc<VerifiedTransaction>,
    _effects: &TransactionEffects,
    _predicates: &SparseStatePredicates,
) -> bool {
    true
}
