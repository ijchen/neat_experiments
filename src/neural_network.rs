use crate::predictor::Predictor;

pub struct NeuralNetwork {
    input_count: usize,
    output_count: usize,
}

impl Predictor for NeuralNetwork {
    fn input_count(&self) -> usize {
        self.input_count
    }

    fn output_count(&self) -> usize {
        self.output_count
    }

    fn predict(&self, inputs: Vec<f64>) -> Vec<f64> {
        debug_assert!(inputs.len() == self.input_count());

        todo!()
    }
}
