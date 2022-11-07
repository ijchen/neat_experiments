use super::{connection_gene::ConnectionGene, predictor::Predictor};

/// Represents one genome in a NEAT population
pub struct Genome<const INPUTS: usize, const OUTPUTS: usize> {
    genes: Vec<ConnectionGene>,
}

impl<const INPUTS: usize, const OUTPUTS: usize> Predictor<INPUTS, OUTPUTS>
    for Genome<INPUTS, OUTPUTS>
{
    fn predict(&self, inputs: &[f64; INPUTS]) -> [f64; OUTPUTS] {
        _ = (inputs, &self.genes);
        todo!();
    }
}

impl<const INPUTS: usize, const OUTPUTS: usize> Genome<INPUTS, OUTPUTS> {
    fn mutate(&mut self) {
        todo!();
        /*
         * Types of connections:
         *   - Add connection
         *     - A single new connection gene with a random weight is added
         *       connecting two previously unconnected nodes
         *   - Add node
         *     - An existing connection is split and a new node is placed where
         *       the old connection used to be. The old connection is disabled
         *       and two new connections are added to the genome. The new
         *       connection leading into the new node receives a weight of 1.0,
         *       and the new connection leading out receives the same weight as
         *       the old connection
         */
    }
}
