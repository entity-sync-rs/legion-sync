//! A number of systems that can be used to synchronize and trace components.

use legion::systems::schedule::Builder;

use crate::{systems::tcp::tcp_sent_system, tracking::SerializationStrategy};
use net_sync::compression::CompressionStrategy;

mod insert;
pub mod tcp;
mod track;

pub use self::{insert::insert_received_entities_system, track::track_modifications_system};

pub trait SchedulerExt {
    fn add_server_systems(self) -> Builder;
    fn add_client_systems(self) -> Builder;
    fn add_required_systems(self) -> Builder;
    fn add_tcp_listener_systems<
        S: SerializationStrategy + 'static,
        C: CompressionStrategy + 'static,
    >(
        self,
    ) -> Builder;
    fn add_tcp_client_systems<
        S: SerializationStrategy + 'static,
        C: CompressionStrategy + 'static,
    >(
        self,
    ) -> Builder;
}

impl SchedulerExt for Builder {
    fn add_server_systems(self) -> Builder {
        self.add_system(insert_received_entities_system())
    }

    fn add_client_systems(self) -> Builder {
        self.add_system(track_modifications_system())
    }

    fn add_required_systems(self) -> Builder {
        // TODO: future use
        self
    }

    fn add_tcp_listener_systems<
        S: SerializationStrategy + 'static,
        C: CompressionStrategy + 'static,
    >(
        self,
    ) -> Builder {
        self.add_system(tcp::tcp_connection_listener())
            .add_system(tcp::tcp_receive_system::<S, C>())
    }

    fn add_tcp_client_systems<
        S: SerializationStrategy + 'static,
        C: CompressionStrategy + 'static,
    >(
        self,
    ) -> Builder {
        self.add_system(tcp_sent_system::<S, C>())
    }
}
