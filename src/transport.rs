use std::sync::{Arc, Mutex};

use crate::peer::PeerIdentity;

use async_trait::async_trait;

#[async_trait]
pub trait Connection {
    fn send(&mut self, data: &[u8]);
    async fn receive(&mut self) -> Vec<u8>;
    fn peer_identity(&self) -> PeerIdentity;
    fn connection_identity(&self) -> String;
}

pub trait Transport {
    fn direct_connection_to(&self, peer: PeerIdentity) -> Box<dyn Connection>;
}

#[derive(Clone)]
pub struct DummyConnection {
    inbound: Arc<Mutex<Vec<Vec<u8>>>>,
    outbound: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl DummyConnection {
    pub fn new_pair() -> (DummyConnection, DummyConnection) {
        let inbound = Arc::new(Mutex::new(Vec::new()));
        let outbound = Arc::new(Mutex::new(Vec::new()));
        (
            DummyConnection {
                inbound: outbound.clone(),
                outbound: inbound.clone(),
            },
            DummyConnection {
                inbound: inbound.clone(),
                outbound: outbound.clone(),
            },
        )
    }
}

#[async_trait]
impl Connection for DummyConnection {
    fn send(&mut self, data: &[u8]) {
        self.outbound.lock().unwrap().push(data.to_vec());
    }

    async fn receive(&mut self) -> Vec<u8> {
        self.inbound.lock().unwrap().pop().unwrap()
    }

    fn peer_identity(&self) -> PeerIdentity {
        PeerIdentity {
            id: "dummy".to_string(),
        }
    }

    fn connection_identity(&self) -> String {
        "dummy".to_string()
    }
}

pub struct DummyTransport {
    conn1: DummyConnection,
    conn2: DummyConnection,
}

impl DummyTransport {
    pub fn new() -> DummyTransport {
        let (conn1, conn2) = DummyConnection::new_pair();
        DummyTransport { conn1, conn2 }
    }
}

impl Transport for DummyTransport {
    fn direct_connection_to(&self, peer: PeerIdentity) -> Box<dyn Connection> {
        if peer.id == "dummy1" {
            Box::new(self.conn1.clone())
        } else {
            Box::new(self.conn2.clone())
        }
    }
}