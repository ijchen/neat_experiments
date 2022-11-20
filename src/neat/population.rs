use rand::Rng;

use crate::neat::implementation_config::{self, CrossoverStrategy};

use super::{
    genome::Genome, neat_config::NeatConfig, node_id::NodeIDGenerator,
    score_normalizer::ScoreNormalizationStrategy,
};

/// Represents a population of genomes from the NEAT algorithm
#[derive(Debug)]
pub struct Population {
    input_count: usize,
    output_count: usize,
    score_normalization_strategy: ScoreNormalizationStrategy,

    node_id_generator: NodeIDGenerator,
    generation: u32,
    gene_pool: Vec<Genome>,
}

impl Population {
    /// Constructs a new Population with the given population size
    pub fn new(input_count: usize, output_count: usize, config: &NeatConfig) -> Self {
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

        let default_genome: Genome = Genome::new(&input_node_ids, &output_node_ids);
        Self {
            input_count,
            output_count,
            score_normalization_strategy: config.score_normalization_strategy(),

            node_id_generator,
            generation: 0,
            gene_pool: vec![default_genome; config.population_size()],
        }
    }

    /// Get the genomes in the population
    pub fn get_genomes(&self) -> Vec<&Genome> {
        self.gene_pool.iter().collect()
    }

    /// Evolve the population based on scores for each member of the population
    pub fn evolve(&mut self, scores: &[f64]) {
        // Ensure we have the same number of scores as members of the population
        let pop_size = self.gene_pool.len();
        assert_eq!(scores.len(), pop_size);

        // Normalize scores
        let scores: Vec<f64> = self.score_normalization_strategy.normalize(scores);

        // Generate members of the next generation
        let mut next_generation: Vec<Genome> = match implementation_config::EVOLUTION_STRATEGY {
            CrossoverStrategy::AsexualProportional => {
                let mut next_generation = Vec::with_capacity(pop_size);

                let score_sum = scores.iter().copied().reduce(|a, b| a + b).unwrap();

                for _ in 0..pop_size {
                    let mut index = 0;
                    let mut n = rand::thread_rng().gen_range(0.0..score_sum);
                    while n > scores[index] {
                        n -= scores[index];
                        index += 1;
                    }

                    next_generation.push(self.gene_pool[index].clone());
                }

                next_generation
            }
            CrossoverStrategy::NoEvolution => self.gene_pool.clone(),
        };
        assert_eq!(next_generation.len(), pop_size);

        // Mutate the population
        for genome in next_generation.iter_mut() {
            genome.mutate();
        }

        // Replace the current population with the next generation
        self.gene_pool = next_generation;

        // Increment the generation
        self.generation += 1;
    }

    /// Returns the current generation number
    pub fn generation(&self) -> u32 {
        self.generation
    }
}
