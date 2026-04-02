use std::collections::HashSet;

pub struct NetworkPeer {
    pub node_id: String,
    peers: HashSet<String>,
    cached: HashSet<String>,
}

impl NetworkPeer {
    pub fn new(id: &str) -> Self {
        NetworkPeer {
            node_id: id.into(),
            peers: HashSet::new(),
            cached: HashSet::new(),
        }
    }

    pub fn add_peer(&mut self, peer: &str) {
        self.peers.insert(peer.into());
    }

    pub fn broadcast(&self, hash: &str) {
        for p in &self.peers {
            println!("NODE {} BROADCAST: {}", self.node_id, hash);
        }
    }
}
