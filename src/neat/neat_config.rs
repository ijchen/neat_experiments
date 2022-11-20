use super::score_normalizer::ScoreNormalizationStrategy;

/// Configuration settings for a NEAT population
#[derive(Clone, Debug)]
pub struct NeatConfig {
    /// The number of genomes in the population
    population_size: usize,

    /// The strategy for normalizing the scores of the population
    score_normalization_strategy: ScoreNormalizationStrategy,
}

impl Default for NeatConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            score_normalization_strategy: ScoreNormalizationStrategy::LerpMinMax,
        }
    }
}

impl NeatConfig {
    /// Updates the population_size setting and returns self
    pub fn with_population_size(mut self, population_size: usize) -> Self {
        self.population_size = population_size;

        self
    }

    /// Returns the population size
    pub fn population_size(&self) -> usize {
        self.population_size
    }

    /// Updates the score_normalization_strategy setting and returns self
    pub fn with_score_normalization_strategy(
        mut self,
        score_normalization_strategy: ScoreNormalizationStrategy,
    ) -> Self {
        self.score_normalization_strategy = score_normalization_strategy;

        self
    }

    /// Returns the score normalization strategy
    pub fn score_normalization_strategy(&self) -> ScoreNormalizationStrategy {
        self.score_normalization_strategy
    }
}
