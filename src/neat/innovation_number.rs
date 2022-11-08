use std::collections::{hash_map::Entry, HashMap};

use super::node_id::NodeID;

#[derive(Copy, Clone, Debug)]
pub struct InnovationNumber(u64);

/// A generator for unique `InnovationNumber`s based on a
#[derive(Clone, Debug)]
pub struct InnovationNumberGenerator {
    next: u64,
    map: HashMap<(NodeID, NodeID), InnovationNumber>,
}

impl InnovationNumberGenerator {
    /// Constructs a new InnovationNumberGenerator
    pub fn new() -> Self {
        Self {
            next: 0,
            map: HashMap::new(),
        }
    }

    /// Returns the InnovationNumber corresponding to the given endpoint pair,
    /// or a novel InnovationNumber if `.gen_from_endpoints(...)` has never been
    /// called with the given endpoint pair before
    pub fn gen_from_endpoints(&mut self, from_id: NodeID, to_id: NodeID) -> InnovationNumber {
        if let Entry::Vacant(entry) = self.map.entry((from_id, to_id)) {
            entry.insert(InnovationNumber(self.next));
            self.next += 1;
        }
        self.map[&(from_id, to_id)]
    }
}
