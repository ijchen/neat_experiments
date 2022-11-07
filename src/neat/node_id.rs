#[derive(Copy, Clone, Debug)]
pub struct NodeID(u64);

/// A generator for unique `NodeID`s
pub struct NodeIDGenerator {
    curr: u64,
}

impl NodeIDGenerator {
    /// Constructs a new NodeIDGenerator
    pub fn new() -> Self {
        Self { curr: 0 }
    }

    /// Returns a new unique NodeID that has never been returned by this
    /// generator before
    pub fn next(&mut self) -> NodeID {
        let innovation_number = NodeID(self.curr);
        self.curr += 1;
        innovation_number
    }
}
