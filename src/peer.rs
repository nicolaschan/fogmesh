#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PeerIdentity {
    pub id: String,
}

impl PeerIdentity {
    pub fn new(id: String) -> PeerIdentity {
        PeerIdentity { id }
    }
}