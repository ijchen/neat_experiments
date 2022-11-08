use super::node_gene::ActivationFunction;

// Defaults
pub const DEFAULT_POPULATION_SIZE: usize = 100;
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
pub enum EvolutionStrategy {
    ClassicalNeat,
    SimpleProportional,
    NoEvolution,
}
pub const EVOLUTION_STRATEGY: EvolutionStrategy = EvolutionStrategy::NoEvolution;
