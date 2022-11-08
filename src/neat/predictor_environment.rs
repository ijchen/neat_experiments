use super::predictor::Predictor;

pub trait PredictorEnvironment {
    /// The input type to a predictor in the environment
    type Input;
    /// The output type from the predictor
    type Output;
    /// The type of a predictor score
    // TODO Associated type defaults are currently unstable
    // #29661 https://github.com/rust-lang/rust/issues/29661
    // type PredictorScore = f64;
    type PredictorScore;

    /// Evaluates a population of predictors, and returns an index-aligned
    /// array with the relative fitness of each predictor.
    /// It is the responsibility of the implementor of this function to ensure
    /// that the score at some index in the output corresponds to the score of
    /// the predictor at the same index of the input population
    fn evaluate_predictors<P: Predictor<Self::Input, Self::Output>>(
        &mut self,
        population: &[&P],
    ) -> Vec<Self::PredictorScore>;
}
