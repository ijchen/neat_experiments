use crate::{
    can_crossover::CanCrossover, neural_network_neuron::NeuralNetworkNeuron, predictor::Predictor,
};

pub struct NeuralNetwork {
    input_count: usize,
    output_count: usize,
    layers: Vec<Vec<NeuralNetworkNeuron>>,
}

impl Predictor for NeuralNetwork {
    fn input_count(&self) -> usize {
        self.input_count
    }

    fn output_count(&self) -> usize {
        self.output_count
    }

    fn predict(&self, inputs: &Vec<f64>) -> Vec<f64> {
        debug_assert!(inputs.len() == self.input_count());

        let mut last_activations = inputs.clone();

        for layer in &self.layers {
            let mut new_last_activations = vec![];

            for neuron in layer {
                new_last_activations.push(neuron.activate(&last_activations));
            }

            last_activations = new_last_activations;
        }

        last_activations
    }
}
impl CanCrossover for NeuralNetwork {
    fn crossover(&self, other: &Self) -> Self {
        debug_assert!(self.input_count == other.input_count);
        debug_assert!(self.output_count == other.output_count);
        debug_assert!(self.layers.len() == other.layers.len());
        for i in 0..self.layers.len() {
            debug_assert!(self.layers[i].len() == other.layers[i].len());
        }

        let mut new_layers: Vec<Vec<NeuralNetworkNeuron>> = vec![];
        for i in 0..self.layers.len() {
            let mut layer: Vec<NeuralNetworkNeuron> = vec![];

            for j in 0..self.layers[i].len() {
                let new_neuron = self.layers[i][j].crossover(&other.layers[i][j]);

                layer.push(new_neuron);
            }

            new_layers.push(layer);
        }

        NeuralNetwork {
            input_count: self.input_count,
            output_count: self.output_count,
            layers: new_layers,
        }
    }
}

impl NeuralNetwork {
    pub fn new(input_count: usize, output_count: usize, layer_sizes: Vec<usize>) -> Self {
        let mut layers = vec![];
        let mut prev_layer_size = input_count;
        for size in layer_sizes {
            layers.push(vec![NeuralNetworkNeuron::new(prev_layer_size); size]);
            prev_layer_size = size;
        }
        layers.push(vec![
            NeuralNetworkNeuron::new(prev_layer_size);
            output_count
        ]);

        NeuralNetwork {
            input_count,
            output_count,
            layers,
        }
    }
}
