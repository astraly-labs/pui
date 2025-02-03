use anyhow::Result;
use mysten_metrics::monitored_mpsc::{self, UnboundedReceiver};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
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
    #[allow(unused)]
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

#[derive(Debug, thiserror::Error)]
pub enum PollError<E> {
    #[error("Future completed unexpectedly")]
    UnexpectedComplete,
    #[error("Future failed with error: {0}")]
    Failed(E),
}

/// A trait to allow polling a future exactly once
#[async_trait::async_trait]
pub trait PollOnce {
    type Error;
    /// Poll the future exactly once. Returns Ok(()) if the future is still pending,
    /// and Err if the future completed unexpectedly or with an error.
    async fn poll_once(&mut self) -> Result<(), PollError<Self::Error>>;
}

#[async_trait::async_trait]
impl<F, E> PollOnce for F
where
    F: Future<Output = Result<(), E>> + Unpin + Send,
    E: Send,
{
    type Error = E;

    async fn poll_once(&mut self) -> Result<(), PollError<Self::Error>> {
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        match Pin::new(self).poll(&mut cx) {
            Poll::Ready(Ok(_)) => Err(PollError::UnexpectedComplete),
            Poll::Ready(Err(e)) => Err(PollError::Failed(e)),
            Poll::Pending => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_context_creation() {
        let _ = test_exex_context().await.unwrap();
    }
}
