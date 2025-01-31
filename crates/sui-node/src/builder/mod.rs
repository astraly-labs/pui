use crate::SuiNode;
use crate::SuiNodeHandle;

use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use sui_config::Config;
use sui_config::NodeConfig;
use sui_core::runtime::SuiRuntimes;
use sui_exex::BoxedLaunchExEx;
use sui_exex::ExExContext;
use sui_types::sunfish::SparseStatePredicates;
use sui_types::supported_protocol_versions::SupportedProtocolVersions;
use tracing::info;

pub struct NodeBuilder {
    pub config: Option<NodeConfig>,
    exexes: Vec<(String, Box<dyn BoxedLaunchExEx>)>,
}

impl NodeBuilder {
    pub fn new() -> Self {
        NodeBuilder {
            config: None,
            exexes: vec![],
        }
    }

    pub fn with_config(mut self, config_path: PathBuf) -> Self {
        let mut cfg = NodeConfig::load(config_path).unwrap();
        assert!(
            cfg.supported_protocol_versions.is_none(),
            "supported_protocol_versions cannot be read from the config file"
        );
        cfg.supported_protocol_versions = Some(SupportedProtocolVersions::SYSTEM_DEFAULT);

        self.config = Some(cfg);
        self
    }

    pub fn with_exex<F, Fut, E>(mut self, name: &str, exex: F) -> Self
    where
        F: FnOnce(ExExContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<E>> + Send + 'static,
        E: Future<Output = anyhow::Result<()>> + Send + 'static,
    {
        self.exexes.push((String::from(name), Box::new(exex)));
        self
    }

    pub fn and_override_sparse_config(
        mut self,
        sparse_state_config: SparseStatePredicates,
    ) -> Self {
        if let Some(ref mut config) = self.config {
            if let Some(ref mut state_sync_config) = config.p2p_config.state_sync {
                state_sync_config.sparse_state_predicates = Some(sparse_state_config);
            }
        }
        self
    }

    pub async fn launch(self) -> anyhow::Result<(SuiNodeHandle, Arc<SuiRuntimes>)> {
        info!("Starting Sui Node...");
        let cfg = self
            .config
            .as_ref()
            .expect("NodeBuilder: Config was not provided");
        let runtimes = Arc::new(SuiRuntimes::new(&cfg));
        let rpc_runtime = runtimes.json_rpc.handle().clone();
        let registry_service = mysten_metrics::start_prometheus_server(cfg.metrics_address);

        let node = SuiNode::start(
            cfg.clone(),
            registry_service,
            Some(rpc_runtime),
            self.exexes,
        )
        .await?;

        Ok((node.into(), runtimes))
    }
}
