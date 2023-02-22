#![feature(async_fn_in_trait)]

pub mod transport;
pub mod peer;
pub mod topology;

use std::collections::HashMap;

use peer::PeerIdentity;
use transport::{Transport, Connection};
use topology::{Topology, AddressedPacket};

pub struct FogMesh<'a> {
    transports: Vec<Box<&'a dyn Transport>>,
    connections: HashMap<PeerIdentity, &'a dyn Connection>
}

impl<'a> FogMesh<'a> {
    pub fn new() -> FogMesh<'a> {
        FogMesh {
            transports: Vec::new(),
            connections: HashMap::new()
        }
    }

    pub fn add_transport(&mut self, transport: &'a dyn Transport) {
        self.transports.push(Box::new(transport));
    }

    pub fn connect(&mut self, peer: PeerIdentity) -> &'a dyn Connection {
        let conn = self.transports[0].direct_connection_to(peer);
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::transport::DummyTransport;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn single_peer() {
        let mut mesh = FogMesh::new();
        let dummy_peer = PeerIdentity::new("dummy1".to_string());
        let dummy_transport = DummyTransport::new();

        mesh.add_transport(&dummy_transport);
        let conn = mesh.connect(dummy_peer);

        assert_eq!(conn.peer_identity().id, "dummy1");
    }
}