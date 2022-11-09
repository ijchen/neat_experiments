use super::node_gene::ActivationFunction;

// Defaults
pub const DEFAULT_POPULATION_SIZE: usize = 10;
pub const DEFAULT_WEIGHT: f64 = 0.0;
pub const DEFAULT_CONNECTION_ENABLED: bool = false;
pub const DEFAULT_INPUT_BIAS: f64 = 0.0;
pub const DEFAULT_HIDDEN_BIAS: f64 = 0.0;
pub const DEFAULT_OUTPUT_BIAS: f64 = 0.0;
pub const DEFAULT_INPUT_ACTIVATION_FUNCTION: ActivationFunction = ActivationFunction::Identity;
pub const DEFAULT_HIDDEN_ACTIVATION_FUNCTION: ActivationFunction = ActivationFunction::Identity;
pub const DEFAULT_OUTPUT_ACTIVATION_FUNCTION: ActivationFunction = ActivationFunction::Identity;

// Evolution
#[allow(dead_code)] // TODO
pub enum CrossoverStrategy {
    AsexualProportional,
    NoEvolution,
}
pub const EVOLUTION_STRATEGY: CrossoverStrategy = CrossoverStrategy::AsexualProportional;

// Score normalization
#[allow(dead_code)] // TODO
pub enum ScoreNormalizationStrategy {
    NoNormalization,
    MapMinMaxToZeroOne,
}
pub const SCORE_NORMALIZATION_STRATEGY: ScoreNormalizationStrategy =
    ScoreNormalizationStrategy::MapMinMaxToZeroOne;

// Mutation
#[allow(dead_code)] // TODO
pub enum MutationStrategy {
    NoMutation,
    SimpleRandomMutation,
}
pub const MUTATION_STRATEGY: MutationStrategy = MutationStrategy::SimpleRandomMutation;
