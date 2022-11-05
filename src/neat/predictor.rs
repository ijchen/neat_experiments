/// TODO doc
pub trait Predictor<const INPUTS: usize, const OUTPUTS: usize> {
    /// Makes a prediction from a given input
    fn predict(&self, inputs: &[f64; INPUTS]) -> [f64; OUTPUTS];
}
