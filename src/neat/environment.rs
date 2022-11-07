use super::predictor::Predictor;

pub trait Environment<const INPUTS: usize, const OUTPUTS: usize> {
    /// Evaluates a population of predictors, and returns an index-aligned
    /// array with the relative fitness of each predictor.
    /// It is the responsibility of the implementor of this function to ensure
    /// that the score at some index in the output corresponds to the score of
    /// the predictor at the same index of the input population
    fn evaluate_predictors<const N: usize, P: Predictor<INPUTS, OUTPUTS>>(
        &mut self,
        population: &[&P; N],
    ) -> [f64; N];
}
