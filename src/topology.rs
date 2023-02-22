use std::collections::{HashMap, BTreeSet};

use crate::{peer::PeerIdentity, transport::Connection};

pub enum Destination {
    All,
    Group(Vec<PeerIdentity>),
}

pub struct AddressedPacket {
    destination: Destination,
    source: PeerIdentity,
    data: Vec<u8>,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct Route {
    peer: PeerIdentity,
    score: usize,
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

pub struct Topology {
    direct_connections: HashMap<PeerIdentity, Box<dyn Connection>>,
    routes: HashMap<PeerIdentity, BTreeSet<Route>>,
}

impl Topology {
    pub fn new() -> Topology {
        Topology {
            direct_connections: HashMap::new(),
            routes: HashMap::new(),
        }
    }

    pub fn add_direct_connection(&mut self, connection: Box<dyn Connection>) {
        self.direct_connections.insert(connection.peer_identity(), connection);
    }

    async fn send(&mut self, destination: Destination, data: Vec<u8>) {

    }

    fn best_route(&self, peer: PeerIdentity) -> Option<Route> {
        self.routes.get(&peer).and_then(|routes| routes.first().cloned())
    }
}