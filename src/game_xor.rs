use crate::{environment::Environment, neural_network::NeuralNetwork};

pub struct GameXor {}

impl Environment for GameXor {
    /// Returns the number of inputs given to a neural network
    fn input_count(&self) -> usize {
        2
    }

    /// Returns the expected number of outputs from a neural network
    fn output_count(&self) -> usize {
        1
    }

    /// Evaluates a population of neural networks, and returns an index-aligned
    /// vector with the relative fitness of each network.
    /// It is the responsibility of the implementor of this function to ensure
    /// that the length of the returned vector of fitness scores is the same as
    /// the length of the given population vector, and that the score at some
    /// index is for the neural network at the same index of the population vector
    fn evaluate_networks(&mut self, population: &Vec<NeuralNetwork>) -> Vec<f64> {
        let mut scores: Vec<f64> = vec![];

        for nn in population {
            // Ensure the neural network and this game are on the same page
            // about how many inputs and outputs there should be
            debug_assert!(nn.input_count() == self.input_count());
            debug_assert!(nn.output_count() == self.output_count());

            // For something as simple as XOR, we can test all possible inputs
            // Overfitting is not a concern here, since the point isn't to generalize,
            // but to test the network's ability to learn non-linear functions
            let mut score = 0.0;
            score += 1.0 - (nn.predict(vec![0.0, 0.0])[0] - 0.0).abs();
            score += 1.0 - (nn.predict(vec![0.0, 1.0])[0] - 1.0).abs();
            score += 1.0 - (nn.predict(vec![1.0, 0.0])[0] - 1.0).abs();
            score += 1.0 - (nn.predict(vec![1.0, 1.0])[0] - 0.0).abs();
            score = f64::max(score, 0.0);

            scores.push(score);
        }

        scores
    }
}
