use anyhow::Result;
use mysten_metrics::monitored_mpsc::{self, UnboundedReceiver};
use std::sync::Arc;
use sui_network::state_sync::{Builder, Handle, UnstartedStateSync};
use sui_swarm_config::test_utils::{empty_contents, CommitteeFixture};
use sui_types::{messages_checkpoint::CheckpointSequenceNumber, storage::SharedInMemoryStore};
use tokio::sync::mpsc::{self, Sender};

use crate::{
    notification::{ExExNotification, ExExNotifications},
    ExExContext, ExExEvent,
};

/// Test handle for controlling and asserting on ExEx behavior
pub struct TestExExHandle {
    notification_sender: Sender<ExExNotification>,
    event_receiver: UnboundedReceiver<ExExEvent>,
    state_sync: Arc<Handle>,
}

impl TestExExHandle {
    /// Send a checkpoint synced notification
    pub async fn send_notification_checkpoint_synced(
        &self,
        checkpoint_number: CheckpointSequenceNumber,
    ) -> Result<()> {
        self.notification_sender
            .send(ExExNotification::CheckpointSynced { checkpoint_number })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send notification: {}", e))
    }

    /// Assert that no events have been emitted
    pub fn assert_events_empty(&mut self) {
        assert!(
            self.event_receiver.try_recv().is_err(),
            "Expected no events"
        );
    }

    /// Assert that a FinishedHeight event was emitted with the expected height
    pub fn assert_event_finished_height(
        &mut self,
        expected_height: CheckpointSequenceNumber,
    ) -> Result<()> {
        match self.event_receiver.try_recv() {
            Ok(ExExEvent::FinishedHeight(height)) => {
                assert_eq!(
                    height, expected_height,
                    "Unexpected height in FinishedHeight event"
                );
                Ok(())
            }
            Ok(other) => Err(anyhow::anyhow!(
                "Expected FinishedHeight event, got {:?}",
                other
            )),
            Err(e) => Err(anyhow::anyhow!("Failed to receive event: {}", e)),
        }
    }
}

/// Creates a test context for ExEx with all required dependencies and a handle for controlling it
pub async fn test_exex_context() -> Result<(ExExContext, TestExExHandle)> {
    let committee = CommitteeFixture::generate(rand::rngs::OsRng, 0, 4);
    let (ordered_checkpoints, _, _sequence_number_to_digest, _checkpoints) =
        committee.make_empty_checkpoints(2, None);
    let store = SharedInMemoryStore::default();
    store.inner_mut().insert_genesis_state(
        ordered_checkpoints.first().cloned().unwrap(),
        empty_contents(),
        committee.committee().to_owned(),
    );
    let (UnstartedStateSync { handle, .. }, _server) =
        Builder::new().store(store.clone()).build_internal();

    // Create channels
    let (notification_sender, notification_receiver) = mpsc::channel(100);
    let notifications = ExExNotifications::new(notification_receiver);
    let (events_tx, events_rx) = monitored_mpsc::unbounded_channel("test_exex_events");

    let context = ExExContext {
        store: Arc::new(store),
        state_sync_handle: handle.clone(),
        events: events_tx,
        notifications,
    };

    let handle = TestExExHandle {
        notification_sender,
        event_receiver: events_rx,
        state_sync: Arc::new(handle),
    };

    Ok((context, handle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_context_creation() {
        let _ = test_exex_context().await.unwrap();
    }
}
