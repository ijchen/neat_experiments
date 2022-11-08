/// TODO doc
pub trait Predictor<Input, Output> {
    /// Makes a prediction from a given input
    fn predict(&self, inputs: Input) -> Output;
}
