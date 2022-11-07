use super::{genome::Genome, node_id::NodeIDGenerator};

/// Represents a population of genomes from the NEAT algorithm
pub struct Population<const INPUTS: usize, const OUTPUTS: usize> {
    gene_pool: Vec<Genome<INPUTS, OUTPUTS>>,
    node_id_generator: NodeIDGenerator,
}

impl<const INPUTS: usize, const OUTPUTS: usize> Population<INPUTS, OUTPUTS> {
    /// Constructs a new Population
    pub fn new() -> Self {
        Self {
            gene_pool: vec![],
            node_id_generator: NodeIDGenerator::new(),
        }
    }
}
