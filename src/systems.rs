//! A number of systems that can be used to synchronize and trace components.

use legion::systems::{Builder, SystemBuilder};

use net_sync::synchronisation::{NetworkCommand, NetworkMessage};

use crate::{
    resources::RegisteredComponentsResource,
    systems::tcp::{tcp_client_receive_system, tcp_client_sent_system},
};

pub mod tcp;

pub trait BuilderExt {
    fn add_server_systems(self) -> Builder;
    fn add_client_systems(self) -> Builder;
    fn add_tcp_server_systems<
        //        C: CompressionStrategy + 'static,
        ServerToClientMessage: NetworkMessage,
        ClientToServerMessage: NetworkMessage,
        ClientToServerCommand: NetworkCommand,
    >(
        self,
    ) -> Builder;
    fn add_tcp_client_systems<
        //        C: CompressionStrategy + 'static,
        ServerToClientMessage: NetworkMessage,
        ClientToServerMessage: NetworkMessage,
        ClientToServerCommand: NetworkCommand,
    >(
        self,
    ) -> Builder;
}

impl BuilderExt for Builder {
    fn add_server_systems(self) -> Builder {
        self
    }

    fn add_client_systems(self) -> Builder {
        self
    }

    fn add_tcp_server_systems<
        //        C: CompressionStrategy + 'static,
        ServerToClientMessage: NetworkMessage,
        ClientToServerMessage: NetworkMessage,
        ClientToServerCommand: NetworkCommand,
    >(
        self,
    ) -> Builder {
        let builder = tcp::tcp_connection_listener::<
            ServerToClientMessage,
            ClientToServerMessage,
            ClientToServerCommand,
        >(self);

        let builder = tcp::tcp_server_receive_system::<
            //            C,
            ServerToClientMessage,
            ClientToServerMessage,
            ClientToServerCommand,
        >(builder);

        let builder = tcp::tcp_server_sent_system::<
            //            C,
            ServerToClientMessage,
            ClientToServerMessage,
            ClientToServerCommand,
        >(builder);

        builder
    }

    fn add_tcp_client_systems<
        //        C: CompressionStrategy + 'static,
        ServerToClientMessage: NetworkMessage,
        ClientToServerMessage: NetworkMessage,
        ClientToServerCommand: NetworkCommand,
    >(
        self,
    ) -> Builder {
        let builder = tcp_client_sent_system::<
            //            C,
            ServerToClientMessage,
            ClientToServerMessage,
            ClientToServerCommand,
        >(self);

        let builder = tcp_client_receive_system::<
            //            C,
            ServerToClientMessage,
            ClientToServerMessage,
            ClientToServerCommand,
        >(builder);

        builder
    }
}

pub trait SystemBuilderExt {
    fn read_registered_components(self) -> SystemBuilder;
    fn write_registered_components(self) -> SystemBuilder;
}

impl SystemBuilderExt for SystemBuilder {
    fn read_registered_components(self) -> SystemBuilder {
        let mut builder = self;
        for component in RegisteredComponentsResource::new().slice_with_uid().iter() {
            builder = component.1.grand_read_access(builder);
        }
        builder
    }

    fn write_registered_components(self) -> SystemBuilder {
        let mut builder = self;
        for component in RegisteredComponentsResource::new().slice_with_uid().iter() {
            builder = component.1.grand_write_access(builder);
        }
        builder
    }
}
