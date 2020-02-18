use std::collections::hash_map::IterMut;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct TcpListenerResource {
    listener: Option<TcpListener>,
    streams: HashMap<SocketAddr, (bool, TcpStream)>,
}

impl TcpListenerResource {
    pub fn new(listener: Option<TcpListener>) -> Self {
        Self {
            listener,
            streams: HashMap::new(),
        }
    }

    /// Returns an immutable reference to the listener if there is one configured.
    pub fn get(&self) -> Option<&TcpListener> {
        self.listener.as_ref()
    }

    /// Returns a mutable reference to the listener if there is one configured.
    pub fn get_mut(&mut self) -> Option<&mut TcpListener> {
        self.listener.as_mut()
    }

    /// Sets the bound listener to the `TcpNetworkResource`.
    pub fn set_listener(&mut self, listener: TcpListener) {
        self.listener = Some(listener);
    }

    /// Drops the listener from the `TcpNetworkResource`.
    pub fn drop_listener(&mut self) {
        self.listener = None;
    }

    /// Returns a tuple of an active TcpStream and whether ot not that stream is active
    pub fn get_stream(&mut self, addr: SocketAddr) -> Option<&mut (bool, TcpStream)> {
        self.streams.get_mut(&addr)
    }

    pub fn add_stream(&mut self, addr: SocketAddr, stream: TcpStream) {
        self.streams.insert(addr, (true, stream));
    }

    /// Drops the stream with the given `SocketAddr`. This will be called when a peer seems to have
    /// been disconnected
    pub fn drop_stream(&mut self, addr: SocketAddr) -> Option<(bool, TcpStream)> {
        self.streams.remove(&addr)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, SocketAddr, (bool, TcpStream)> {
        self.streams.iter_mut()
    }
}