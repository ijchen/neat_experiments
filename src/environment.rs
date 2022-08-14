use crate::neural_network::NeuralNetwork;

pub trait Environment {
    /// Returns the number of inputs given to a neural network
    fn input_count(&self) -> usize;

    /// Returns the expected number of outputs from a neural network
    fn output_count(&self) -> usize;

    /// Evaluates a population of neural networks, and returns an index-aligned
    /// vector with the relative fitness of each network.
    /// It is the responsibility of the implementor of this function to ensure
    /// that the length of the returned vector of fitness scores is the same as
    /// the length of the given population vector, and that the score at some
    /// index is for the neural network at the same index of the population vector.
    /// Scores should all be >= 0
    fn evaluate_networks(&mut self, population: &Vec<NeuralNetwork>) -> Vec<f64>;
}
