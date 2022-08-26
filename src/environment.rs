use crate::predictor::Predictor;

pub trait Environment {
    /// Returns the number of inputs given to a predictor
    fn input_count(&self) -> usize;

    /// Returns the expected number of outputs from a predictor
    fn output_count(&self) -> usize;

    /// Evaluates a population of predictors, and returns an index-aligned
    /// vector with the relative fitness of each predictor.
    /// It is the responsibility of the implementor of this function to ensure
    /// that the length of the returned vector of fitness scores is the same as
    /// the length of the given population vector, and that the score at some
    /// index is for the predictor at the same index of the population vector
    /// Scores should all be >= 0
    fn evaluate_predictors<P: Predictor>(&mut self, population: &[&P]) -> Vec<f64>;
}
