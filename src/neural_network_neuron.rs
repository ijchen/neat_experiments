use rand::Rng;

use crate::{can_crossover::CanCrossover, can_mutate::CanMutate};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum NeuralNetworkActivationFun {
    Identity,
    Step,
    Sigmoid,
    TanH,
    ReLU,
    LeakyReLU(f64),
    Arctan,
    Sqnl,
}

#[derive(Clone, Debug)]
pub struct NeuralNetworkNeuron {
    weights: Vec<f64>,
    bias: f64,
    activation_function: NeuralNetworkActivationFun,
}

impl CanCrossover for NeuralNetworkNeuron {
    fn crossover(&self, other: &Self) -> Self {
        assert!(self.weights.len() == other.weights.len());

        let mut rng = rand::thread_rng();

        let mut new_weights = vec![];
        for i in 0..self.weights.len() {
            let lerp_frac = rng.gen_range(0.0..=1.0);
            let new_weight = self.weights[i] * (1.0 - lerp_frac) + other.weights[i] * lerp_frac;

            new_weights.push(new_weight);
        }

        let lerp_frac = rng.gen_range(0.0..=1.0);
        let new_bias = self.bias * (1.0 - lerp_frac) + other.bias * lerp_frac;

        let new_activation_function = if rng.gen() {
            self.activation_function.clone()
        } else {
            other.activation_function.clone()
        };

        NeuralNetworkNeuron {
            weights: new_weights,
            bias: new_bias,
            activation_function: new_activation_function,
        }
    }
}

impl CanMutate for NeuralNetworkNeuron {
    fn mutate(&mut self) {
        // TODO Be thoughtful about how mutation happens
        let mut rng = rand::thread_rng();
        const MUT_ODDS: f64 = 0.01;

        for i in 0..self.weights.len() {
            if rng.gen_bool(MUT_ODDS) {
                self.weights[i] += rng.gen_range(-0.5..0.5);
            }
        }
        if rng.gen_bool(MUT_ODDS) {
            self.bias += rng.gen_range(-0.5..0.5);
        }
    }
}

impl NeuralNetworkNeuron {
    pub fn new(weight_count: usize, activation_function: NeuralNetworkActivationFun) -> Self {
        // TODO Be thoughtful about initial weights
        let mut rng = rand::thread_rng();

        let mut weights: Vec<f64> = vec![];
        for _ in 0..weight_count {
            weights.push(rng.gen_range(-2.0..=2.0));
        }

        let bias = rng.gen_range(-2.0..=2.0);

        NeuralNetworkNeuron {
            weights,
            bias,
            activation_function,
        }
    }

    pub fn bias(&self) -> &f64 {
        &self.bias
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    fn apply_activation_function(&self, x: f64) -> f64 {
        match self.activation_function {
            NeuralNetworkActivationFun::Identity => x,
            NeuralNetworkActivationFun::Step => {
                if x > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            NeuralNetworkActivationFun::Sigmoid => ((-x).exp() + 1.0).recip(),
            NeuralNetworkActivationFun::TanH => x.tanh(),
            NeuralNetworkActivationFun::ReLU => f64::max(0.0, x),
            NeuralNetworkActivationFun::LeakyReLU(leak_factor) => f64::max(leak_factor * x, x),
            NeuralNetworkActivationFun::Arctan => x.atan(),
            NeuralNetworkActivationFun::Sqnl => {
                if x < -2.0 {
                    -1.0
                } else if x < 0.0 {
                    x + x * x / 4.0
                } else if x < 2.0 {
                    x - x * x / 4.0
                } else {
                    1.0
                }
            }
        }
    }

    pub fn activate(&self, inputs: &[f64]) -> f64 {
        assert!(inputs.len() == self.weights.len());

        let mut weighted_sum = 0.0;
        for (i, input) in inputs.iter().enumerate() {
            weighted_sum += input * self.weights[i];
        }

        self.apply_activation_function(weighted_sum + self.bias)
    }
}
