use chrono::{DateTime, Utc};
use std::sync::atomic::AtomicI64;
use std::sync::Arc;

use crate::actor::system::ActorSystem;
use crate::actor::LocalActorRef;
use crate::remote::actor::{
    RemoteClientRegistry, RemoteHandler, RemoteRegistry, RemoteSystemConfig,
};
use crate::remote::cluster::builder::client::ClusterClientBuilder;
use crate::remote::cluster::builder::worker::ClusterWorkerBuilder;
use crate::remote::cluster::discovery::NodeDiscovery;
use crate::remote::heartbeat::Heartbeat;
use crate::remote::stream::mediator::StreamMediator;
use crate::remote::system::builder::RemoteActorSystemBuilder;

pub mod actor;
pub mod builder;
pub mod cluster;
pub mod raft;
pub mod rpc;

use crate::remote::raft::RaftSystem;
pub use actor::*;
pub use cluster::*;
pub use rpc::*;

#[derive(Clone)]
pub struct RemoteActorSystem {
    inner: Arc<RemoteSystemCore>,
}

pub type NodeId = u64;
pub type AtomicNodeId = AtomicI64;

#[derive(Clone)]
pub struct RemoteSystemCore {
    node_id: NodeId,
    inner: ActorSystem,
    started_at: DateTime<Utc>,
    handler_ref: Arc<parking_lot::Mutex<RemoteHandler>>,
    registry_ref: LocalActorRef<RemoteRegistry>,
    clients_ref: LocalActorRef<RemoteClientRegistry>,
    discovery_ref: LocalActorRef<NodeDiscovery>,
    heartbeat_ref: LocalActorRef<Heartbeat>,
    mediator_ref: Option<LocalActorRef<StreamMediator>>,
    config: Arc<RemoteSystemConfig>,
    raft: Option<Arc<RaftSystem>>,
    current_leader: Arc<AtomicNodeId>,
}

impl RemoteActorSystem {
    pub fn builder() -> RemoteActorSystemBuilder {
        RemoteActorSystemBuilder::new()
    }

    pub fn cluster_worker(self) -> ClusterWorkerBuilder {
        ClusterWorkerBuilder::new(self)
    }

    pub fn cluster_client(self) -> ClusterClientBuilder {
        ClusterClientBuilder::new(self)
    }

    pub fn config(&self) -> &RemoteSystemConfig {
        &self.inner.config
    }

    pub fn node_tag(&self) -> &str {
        self.inner.config.node_tag()
    }

    pub fn node_id(&self) -> NodeId {
        self.inner.node_id
    }

    pub fn started_at(&self) -> &DateTime<Utc> {
        &self.inner.started_at
    }

    pub fn heartbeat(&self) -> &LocalActorRef<Heartbeat> {
        &self.inner.heartbeat_ref
    }

    pub fn client_registry(&self) -> &LocalActorRef<RemoteClientRegistry> {
        &self.inner.clients_ref
    }

    pub fn node_discovery(&self) -> &LocalActorRef<NodeDiscovery> {
        &self.inner.discovery_ref
    }

    pub fn stream_mediator(&self) -> Option<&LocalActorRef<StreamMediator>> {
        self.inner.mediator_ref.as_ref()
    }

    pub fn actor_system(&self) -> &ActorSystem {
        &self.inner.actor_system()
    }
}

impl RemoteSystemCore {
    pub fn actor_system(&self) -> &ActorSystem {
        &self.inner
    }
}
