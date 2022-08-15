pub trait Predictor {
    /// Returns the number of inputs given to the predictor
    fn input_count(&self) -> usize;

    /// Returns the expected number of outputs from the predictor
    fn output_count(&self) -> usize;

    /// Makes a prediction from a given input
    /// It is the responsibility of the implementor of this method to ensure
    /// that the length of the returned vector of predictions is the same as
    /// the expected number of outputs from the predictor
    fn predict(&self, inputs: Vec<f64>) -> Vec<f64>;
}
