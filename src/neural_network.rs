pub struct NeuralNetwork {
    input_count: usize,
    output_count: usize,
}

impl NeuralNetwork {
    pub fn input_count(&self) -> usize {
        self.input_count
    }

    pub fn output_count(&self) -> usize {
        self.output_count
    }

    pub fn predict(&self, inputs: Vec<f64>) -> Vec<f64> {
        debug_assert!(inputs.len() == self.input_count());

        todo!()
    }
}
