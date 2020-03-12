//! A number of resources that can be used to synchronize and trace components.

pub use self::{
    buffer::{BufferResource, ReceiveBufferResource, SentBufferResource},
    component::RegisteredComponentsResource,
    event::EventResource,
    packer::Packer,
    tick::TickResource,
    track::TrackResource,
};
use crate::{
    resources::tcp::{TcpClientResource, TcpListenerResource},
    tracking::SerializationStrategy,
};
use legion::prelude::{Entity, Resources};
use net_sync::{compression::CompressionStrategy, uid::UidAllocator};
use std::{
    collections::{vec_deque::Iter, VecDeque},
    net::{SocketAddr, TcpListener},
};

mod buffer;
mod component;
mod event;
mod packer;
mod tick;
mod track;

pub mod tcp;

pub trait ResourcesExt {
    fn insert_server_resources<
        S: SerializationStrategy + 'static,
        C: CompressionStrategy + 'static,
    >(
        &mut self,
        serialization: S,
        compression: C,
    );
    fn insert_client_resources<
        S: SerializationStrategy + 'static,
        C: CompressionStrategy + 'static,
    >(
        &mut self,
        serialization: S,
        compression: C,
    );
    fn insert_required<S: SerializationStrategy + 'static, C: CompressionStrategy + 'static>(
        &mut self,
        serialization: S,
        compression: C,
    );

    fn insert_tcp_client_resources(&mut self, addr: SocketAddr);
    fn insert_tcp_listener_resources(&mut self, listener: TcpListener);
}

impl ResourcesExt for Resources {
    fn insert_server_resources<
        S: SerializationStrategy + 'static,
        C: CompressionStrategy + 'static,
    >(
        &mut self,
        serialization: S,
        compression: C,
    ) {
        self.insert(TrackResource::new());
        self.insert(ReceiveBufferResource::default());
        self.insert(RegisteredComponentsResource::new());
        self.insert(BufferResource::from_capacity(1500));
        self.insert(RemovedEntities::new());
        self.insert_required(serialization, compression);
    }

    fn insert_client_resources<
        S: SerializationStrategy + 'static,
        C: CompressionStrategy + 'static,
    >(
        &mut self,
        serialization: S,
        compression: C,
    ) {
        self.insert(SentBufferResource::new());
        self.insert_required(serialization, compression);
    }

    fn insert_required<S: SerializationStrategy + 'static, C: CompressionStrategy + 'static>(
        &mut self,
        __serialization: S,
        __compression: C,
    ) {
        self.insert(RegisteredComponentsResource::new());
        self.insert(Packer::<S, C>::default());
        self.insert(UidAllocator::<Entity>::new())
    }

    fn insert_tcp_client_resources(&mut self, addr: SocketAddr) {
        self.insert(TcpClientResource::new(addr).unwrap());
    }

    fn insert_tcp_listener_resources(&mut self, listener: TcpListener) {
        self.insert(TcpListenerResource::new(Some(listener)));
    }
}

pub struct RemovedEntities {
    removed: VecDeque<Entity>,
}

impl RemovedEntities {
    pub fn new() -> RemovedEntities {
        RemovedEntities {
            removed: VecDeque::new(),
        }
    }

    pub fn add(&mut self, entity: Entity) {
        self.removed.push_back(entity);
    }

    pub fn iter(&self) -> Iter<Entity> {
        self.removed.iter()
    }
}
