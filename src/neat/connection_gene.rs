use crate::neat::node_id::NodeID;

use super::innovation_number::InnovationNumber;

#[derive(Clone, Debug)]
pub struct ConnectionGene {
    innovation_number: InnovationNumber,
    from_node: NodeID,
    to_node: NodeID,
    weight: f64,
    enabled: bool,
}

impl ConnectionGene {
    /// Constructs a new ConnectionGene
    pub fn new(
        innovation_number: InnovationNumber,
        from_node: NodeID,
        to_node: NodeID,
        weight: f64,
        enabled: bool,
    ) -> Self {
        Self {
            innovation_number,
            from_node,
            to_node,
            weight,
            enabled,
        }
    }

    /// Returns the NodeID of the node this ConnectionGene is coming from
    pub fn get_from_node(&self) -> NodeID {
        self.from_node
    }

    /// Returns the NodeID of the node this ConnectionGene is going to
    pub fn get_to_node(&self) -> NodeID {
        self.to_node
    }

    /// Returns the weight of this ConnectionGene
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Returns whether or not this ConnectionGene is enabled
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Sets the weight of this ConnectionGene
    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }

    /// Toggles whether or not this ConnectionGene is enabled
    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }
}
