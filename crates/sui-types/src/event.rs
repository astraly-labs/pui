// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::ensure;
use move_core_types::account_address::AccountAddress;
use move_core_types::annotated_value::MoveDatatypeLayout;
use move_core_types::annotated_value::MoveValue;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;
use serde_with::Bytes;
use serde_with::DisplayFromStr;

use crate::base_types::{ObjectID, SuiAddress, TransactionDigest};
use crate::error::{SuiError, SuiResult};
use crate::object::bounded_visitor::BoundedVisitor;
use crate::sui_serde::BigInt;
use crate::sui_serde::Readable;
use crate::sui_serde::SuiStructTag;
use crate::SUI_SYSTEM_ADDRESS;

/// A universal Sui event type encapsulating different types of events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    pub timestamp: u64,
    /// Transaction digest of associated transaction
    pub tx_digest: TransactionDigest,
    /// Consecutive per-tx counter assigned to this event.
    pub event_num: u64,
    /// Specific event type
    pub event: Event,
    /// Move event's json value
    pub parsed_json: Value,
}
/// Unique ID of a Sui Event, the ID is a combination of transaction digest and event seq number.
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EventID {
    pub tx_digest: TransactionDigest,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub event_seq: u64,
}

impl From<(TransactionDigest, u64)> for EventID {
    fn from((tx_digest_num, event_seq_number): (TransactionDigest, u64)) -> Self {
        Self {
            tx_digest: tx_digest_num as TransactionDigest,
            event_seq: event_seq_number,
        }
    }
}

impl From<EventID> for String {
    fn from(id: EventID) -> Self {
        format!("{:?}:{}", id.tx_digest, id.event_seq)
    }
}

impl TryFrom<String> for EventID {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let values = value.split(':').collect::<Vec<_>>();
        ensure!(values.len() == 2, "Malformed EventID : {value}");
        Ok((
            TransactionDigest::from_str(values[0])?,
            u64::from_str(values[1])?,
        )
            .into())
    }
}

impl EventEnvelope {
    pub fn new(
        timestamp: u64,
        tx_digest: TransactionDigest,
        event_num: u64,
        event: Event,
        move_struct_json_value: Value,
    ) -> Self {
        Self {
            timestamp,
            tx_digest,
            event_num,
            event,
            parsed_json: move_struct_json_value,
        }
    }
}

/// Specific type of event
#[serde_as]
#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub package_id: ObjectID,
    pub transaction_module: Identifier,
    pub sender: SuiAddress,
    pub type_: StructTag,
    #[serde_as(as = "Bytes")]
    pub contents: Vec<u8>,
}

impl Event {
    pub fn new(
        package_id: &AccountAddress,
        module: &IdentStr,
        sender: SuiAddress,
        type_: StructTag,
        contents: Vec<u8>,
    ) -> Self {
        Self {
            package_id: ObjectID::from(*package_id),
            transaction_module: Identifier::from(module),
            sender,
            type_,
            contents,
        }
    }
    pub fn move_event_to_move_value(
        contents: &[u8],
        layout: MoveDatatypeLayout,
    ) -> SuiResult<MoveValue> {
        BoundedVisitor::deserialize_value(contents, &layout.into_layout()).map_err(|e| {
            SuiError::ObjectSerializationError {
                error: e.to_string(),
            }
        })
    }

    pub fn is_system_epoch_info_event(&self) -> bool {
        self.type_.address == SUI_SYSTEM_ADDRESS
            && self.type_.module.as_ident_str() == ident_str!("sui_system_state_inner")
            && self.type_.name.as_ident_str() == ident_str!("SystemEpochInfoEvent")
    }
}

impl Event {
    pub fn random_for_testing() -> Self {
        Self {
            package_id: ObjectID::random(),
            transaction_module: Identifier::new("test").unwrap(),
            sender: AccountAddress::random().into(),
            type_: StructTag {
                address: AccountAddress::random(),
                module: Identifier::new("test").unwrap(),
                name: Identifier::new("test").unwrap(),
                type_params: vec![],
            },
            contents: vec![],
        }
    }
}

// Event emitted in move code `fun advance_epoch`
#[derive(Serialize, Deserialize, Default)]
pub struct SystemEpochInfoEvent {
    pub epoch: u64,
    pub protocol_version: u64,
    pub reference_gas_price: u64,
    pub total_stake: u64,
    pub storage_fund_reinvestment: u64,
    pub storage_charge: u64,
    pub storage_rebate: u64,
    pub storage_fund_balance: u64,
    pub stake_subsidy_amount: u64,
    pub total_gas_fees: u64,
    pub total_stake_rewards_distributed: u64,
    pub leftover_storage_fund_inflow: u64,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum EventFilter {
    /// Return all events.
    All([Box<EventFilter>; 0]),

    /// Return events that match any of the given filters. Only supported on event subscriptions.
    Any(Vec<EventFilter>),

    /// Query by sender address.
    Sender(SuiAddress),

    /// Return events emitted by the given transaction.
    Transaction(
        ///digest of the transaction, as base-64 encoded string
        TransactionDigest,
    ),
    /// Return events emitted in a specified Move module.
    /// If the event is defined in Module A but emitted in a tx with Module B,
    /// query `MoveModule` by module B returns the event.
    /// Query `MoveEventModule` by module A returns the event too.
    MoveModule {
        /// the Move package ID
        package: ObjectID,
        /// the module name
        #[schemars(with = "String")]
        #[serde_as(as = "DisplayFromStr")]
        module: Identifier,
    },
    /// Return events with the given Move event struct name (struct tag).
    /// For example, if the event is defined in `0xabcd::MyModule`, and named
    /// `Foo`, then the struct tag is `0xabcd::MyModule::Foo`.
    MoveEventType(
        #[schemars(with = "String")]
        #[serde_as(as = "SuiStructTag")]
        StructTag,
    ),
    /// Return events with the given Move module name where the event struct is defined.
    /// If the event is defined in Module A but emitted in a tx with Module B,
    /// query `MoveEventModule` by module A returns the event.
    /// Query `MoveModule` by module B returns the event too.
    MoveEventModule {
        /// the Move package ID
        package: ObjectID,
        /// the module name
        #[schemars(with = "String")]
        #[serde_as(as = "DisplayFromStr")]
        module: Identifier,
    },
    /// Return events emitted in [start_time, end_time] interval
    #[serde(rename_all = "camelCase")]
    TimeRange {
        /// left endpoint of time interval, milliseconds since epoch, inclusive
        #[schemars(with = "BigInt<u64>")]
        #[serde_as(as = "BigInt<u64>")]
        start_time: u64,
        /// right endpoint of time interval, milliseconds since epoch, exclusive
        #[schemars(with = "BigInt<u64>")]
        #[serde_as(as = "BigInt<u64>")]
        end_time: u64,
    },
    /// Return events where the parsed_json matches any of the given values.
    AnyValue(Vec<Value>),
    /// Return events where the parsed_json matches all of the given values.
    AllValues(Vec<Value>),
}

pub trait Filter<T> {
    fn matches(&self, item: &T) -> bool;
}
