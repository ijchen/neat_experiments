use crate::neat::implementation_config::{self, EvolutionStrategy};

use super::{genome::Genome, node_id::NodeIDGenerator};

/// Represents a population of genomes from the NEAT algorithm
#[derive(Debug)]
pub struct Population {
    input_count: usize,
    output_count: usize,
    generation: u32,
    gene_pool: Vec<Genome>,
    node_id_generator: NodeIDGenerator,
}

impl Population {
    /// Constructs a new Population with the given population size
    pub fn new(population_size: usize, input_count: usize, output_count: usize) -> Self {
        let mut node_id_generator = NodeIDGenerator::new();

        // Generate input node IDs
        let mut input_node_ids = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            input_node_ids.push(node_id_generator.next());
        }
        // Generate output node IDs
        let mut output_node_ids = Vec::with_capacity(output_count);
        for _ in 0..output_count {
            output_node_ids.push(node_id_generator.next());
        }

        let default_genome: Genome = Genome::new(input_node_ids, output_node_ids);
        Self {
            input_count,
            output_count,
            generation: 0,
            gene_pool: vec![default_genome; population_size],
            node_id_generator,
        }
    }

    /// Get the genomes in the population
    pub fn get_genomes(&self) -> Vec<&Genome> {
        self.gene_pool.iter().collect()
    }

    /// Evolve the population based on scores for each member of the population
    pub fn evolve(&mut self, scores: &[f64]) {
        // Ensure we have the same number of scores as members of the population
        assert_eq!(scores.len(), self.gene_pool.len());

        // Evolve the population
        match implementation_config::EVOLUTION_STRATEGY {
            EvolutionStrategy::ClassicalNeat => {
                todo!();
            }
            EvolutionStrategy::SimpleProportional => {
                todo!();
            }
            EvolutionStrategy::NoEvolution => {
                // Not much to do here...
            }
        }

        // Increment the generation
        self.generation += 1;
    }

    /// Returns the current generation number
    pub fn generation(&self) -> u32 {
        self.generation
    }
}
