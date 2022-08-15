use rand::Rng;

use crate::can_crossover::CanCrossover;

#[derive(Clone)]
pub struct NeuralNetworkNeuron {
    weights: Vec<f64>,
    bias: f64,
}

impl CanCrossover for NeuralNetworkNeuron {
    fn crossover(&self, other: &Self) -> Self {
        debug_assert!(self.weights.len() == other.weights.len());

        let mut rng = rand::thread_rng();

        let mut new_weights = vec![];
        for i in 0..self.weights.len() {
            let lerp_frac = rng.gen_range(0.0..1.0);
            let new_weight = self.weights[i] * lerp_frac + other.weights[i] * (1.0 - lerp_frac);

            new_weights.push(new_weight);
        }

        let lerp_frac = rng.gen_range(0.0..1.0);
        let new_bias = self.bias * lerp_frac + other.bias * (1.0 - lerp_frac);

        NeuralNetworkNeuron {
            weights: new_weights,
            bias: new_bias,
        }
    }
}

impl NeuralNetworkNeuron {
    pub fn new(weight_count: usize) -> Self {
        // TODO Be thoughtful about initial weights
        let mut rng = rand::thread_rng();

        let mut weights: Vec<f64> = vec![];
        for _ in 0..weight_count {
            weights.push(rng.gen_range(-2.0..2.0));
        }

        let bias = rng.gen_range(-2.0..2.0);

        NeuralNetworkNeuron { weights, bias }
    }

    fn activation_function(&self, x: f64) -> f64 {
        // // TODO Just hardcoded ReLU for now
        // f64::max(0.0, x)

        // TODO Just hardcoded TanH for now
        x.tanh()
    }

    pub fn activate(&self, inputs: &Vec<f64>) -> f64 {
        debug_assert!(inputs.len() == self.weights.len());

        let mut weighted_sum = 0.0;
        for i in 0..inputs.len() {
            weighted_sum += inputs[i] * self.weights[i];
        }

        let activation = self.activation_function(weighted_sum + self.bias);

        activation
    }
}
