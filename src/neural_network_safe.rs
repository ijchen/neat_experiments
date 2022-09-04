use macroquad::{
    prelude::Color,
    shapes::{draw_circle, draw_line},
};
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap; 


use crate::{
    can_crossover::CanCrossover,
    can_mutate::CanMutate,
    neural_network_neuron::{ConnectionGene, NeuralNetworkActivationFun, NeuralNetworkNeuron},
    predictor::Predictor,
    renderable::Renderable,
};
#[derive(Clone)]

pub struct Genome {
    connection_genes: HashMap<usize, ConnectionGene>,
    max_innovation: usize, 
    fitness_score: Option<f64>, //confirm this is the right type... may need to incorporate predictor
}

impl CanCrossover for Genome {
    fn crossover(&self, other: &Self) -> Self {
        let excess_index = min(self.max_innovation, other.max_innovation); 

        let new_connections: HashMap<usize, ConnectionGene> = HashMap::new();

        //Loops through possible innovation numbers up until the highest innovation number for 
        //the parent whose max innovation number is lower (i.e., up until the start of any excess genes)
        //If both parents have gene, one is randomly chosen to give to offspring. If only one parent has it, child 
        //gets gene from that parent
        for i in 1..excess_index {
            if self.connection_genes.contains_key(&i) && other.connection_genes.contains_key(&i){

                //choose randomly between parents for which version of gene to pass 
                let inherited_gene = {
                    match rand::thread_rng().gen_bool(0.5){
                        0 => self.connection_genes[&i], 
                        1 => other.connection_genes[&i],
                    } }; 
                
                new_connections.insert(i, inherited_gene);     
            }
            else if self.connection_genes.contains_key(&i) {
                new_connections.insert(i, self.connection_genes[&i]); 
            }
            else if other.connection_genes.contains_key(&i) {
                new_connections.insert(i, other.connection_genes[&i]); 
            }
        }

        //Deals with excess genes (this logic could defintely be cleaned and tightened)
        if (self.max_innovation > other.max_innovation) && (self.fitness_score >= other.fitness_score) {
            for i in excess_index..self.max_innovation {
                new_connections.insert(i, self.connection_genes[&i]); 
            }
        }
        else if (other.max_innovation > self.max_innovation) && (other.fitness_score >= self.fitness_score){
            for i in excess_index..other.max_innovation {
                new_connections.insert(i, other.connection_genes[&i]); 
            }
        }

        Genome {
            connection_genes: new_connections,
            max_innovation: max(self.max_innovation, other.max_innovation), 
            fitness_score: None, //confirm this is the right type... may need to incorporate predictor
        }
      
        }
    }

impl CanMutate for Genome {
    fn mutate(&mut self) {
        //if connection doesn't exist, innov # must increase
        todo!();
    }
}
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

    fn predict(&self, inputs: &[f64]) -> Vec<f64> {
        assert!(inputs.len() == self.input_count());

        let mut last_activations = inputs.to_vec();

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
        assert!(self.input_count == other.input_count);
        assert!(self.output_count == other.output_count);
        assert!(self.layers.len() == other.layers.len());
        for i in 0..self.layers.len() {
            assert!(self.layers[i].len() == other.layers[i].len());
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

impl CanMutate for NeuralNetwork {
    fn mutate(&mut self) {
        for layer in &mut self.layers {
            for neuron in layer.iter_mut() {
                neuron.mutate();
            }
        }
    }
}

impl Renderable for NeuralNetwork {
    fn render(
        &self,
        _args: &crate::renderable::RenderArgs,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        // TODO Profile and optimize, visually clean up code, improve readability
        fn map(x: f64, a1: f64, b1: f64, a2: f64, b2: f64) -> f64 {
            assert!(b1 != a1);

            (x - a1) / (b1 - a1) * (b2 - a2) + a2
        }

        fn lerp_color(c1: (u8, u8, u8), c2: (u8, u8, u8), frac: f64) -> (u8, u8, u8) {
            assert!((0.0..=1.0).contains(&frac));

            (
                (c1.0 as f64 * frac + c2.0 as f64 * (1.0 - frac)).round() as u8,
                (c1.1 as f64 * frac + c2.1 as f64 * (1.0 - frac)).round() as u8,
                (c1.2 as f64 * frac + c2.2 as f64 * (1.0 - frac)).round() as u8,
            )
        }
        const RED: (u8, u8, u8) = (255, 0, 0);
        const GREEN: (u8, u8, u8) = (0, 255, 0);

        // Calculate useful information for various parts of rendering the network
        let layer_dist = width / (self.layers.len() + 2) as f64;
        let max_neuron_count = self.layers.iter().map(|layer| layer.len()).max().unwrap() as f64;
        let neuron_dist: f64 = height / (max_neuron_count + 1.0);
        let min_network_weight = self
            .layers
            .iter()
            .flatten()
            .flat_map(|neuron| neuron.weights())
            .copied()
            .reduce(f64::min)
            .unwrap();
        let max_network_weight = self
            .layers
            .iter()
            .flatten()
            .flat_map(|neuron| neuron.weights())
            .copied()
            .reduce(f64::max)
            .unwrap();
        let min_network_bias = self
            .layers
            .iter()
            .flatten()
            .map(|neuron| neuron.bias())
            .copied()
            .reduce(f64::min)
            .unwrap();
        let max_network_bias = self
            .layers
            .iter()
            .flatten()
            .map(|neuron| neuron.bias())
            .copied()
            .reduce(f64::max)
            .unwrap();
        let max_weight_abs = f64::max(min_network_weight.abs(), max_network_weight.abs());
        let max_bias_abs = f64::max(min_network_bias.abs(), max_network_bias.abs());

        // Render connections
        for (layer_index, layer) in self.layers.iter().enumerate() {
            let layer_x = x + (layer_index as f64 + 2.0) * layer_dist;
            let layer_neuron_count = layer.len();
            for (neuron_index, neuron) in layer.iter().enumerate() {
                let neuron_y = y
                    + (neuron_index as f64
                        + 1.0
                        + (max_neuron_count - layer_neuron_count as f64) / 2.0)
                        * neuron_dist;
                let prev_x = layer_x - layer_dist;

                // Connections to previous neurons
                for (weight_index, weight) in neuron.weights().iter().enumerate() {
                    let prev_y = y
                        + (weight_index as f64
                            + 1.0
                            + (max_neuron_count - neuron.weights().len() as f64) / 2.0)
                            * neuron_dist;

                    let thickness: f64 = map(
                        weight.abs(),
                        0.0,
                        max_weight_abs,
                        0.0,
                        f64::min(width, height) / 50.0,
                    );
                    let color_frac = map(*weight, -max_weight_abs, max_weight_abs, 0.0, 1.0);
                    let color = lerp_color(GREEN, RED, color_frac);
                    draw_line(
                        prev_x as f32,
                        prev_y as f32,
                        layer_x as f32,
                        neuron_y as f32,
                        thickness as f32,
                        Color::from_rgba(color.0, color.1, color.2, 255),
                    )
                }
            }
        }

        // Render input layer nodes
        let node_rad: f64 = f64::min(width, height) / 30.0;
        let input_layer_x = x + layer_dist;
        let input_layer_neuron_count = self.input_count;
        for neuron_index in 0..input_layer_neuron_count {
            let neuron_y = y
                + (neuron_index as f64
                    + 1.0
                    + (max_neuron_count - input_layer_neuron_count as f64) / 2.0)
                    * neuron_dist;
            draw_circle(
                input_layer_x as f32,
                neuron_y as f32,
                node_rad as f32,
                Color::from_rgba(127, 127, 127, 255),
            );
        }

        // Hidden/output layer nodes
        for (layer_index, layer) in self.layers.iter().enumerate() {
            let layer_x = x + (layer_index as f64 + 2.0) * layer_dist;
            let layer_neuron_count = layer.len();
            for (neuron_index, neuron) in layer.iter().enumerate() {
                let neuron_y = y
                    + (neuron_index as f64
                        + 1.0
                        + (max_neuron_count - layer_neuron_count as f64) / 2.0)
                        * neuron_dist;

                // Neuron
                let color_frac = map(*neuron.bias(), -max_bias_abs, max_bias_abs, 0.0, 1.0);
                let color = lerp_color(GREEN, RED, color_frac);
                draw_circle(
                    layer_x as f32,
                    neuron_y as f32,
                    node_rad as f32,
                    Color::from_rgba(color.0, color.1, color.2, 255),
                );
            }
        }
    }
}

impl NeuralNetwork {
    pub fn new(input_count: usize, output_count: usize, layer_sizes: Vec<usize>) -> Self {
        let mut layers = vec![];
        let mut prev_layer_size = input_count;
        for size in layer_sizes {
            layers.push(vec![
                NeuralNetworkNeuron::new(
                    prev_layer_size,
                    NeuralNetworkActivationFun::TanH
                );
                size
            ]);
            prev_layer_size = size;
        }
        layers.push(vec![
            NeuralNetworkNeuron::new(
                prev_layer_size,
                NeuralNetworkActivationFun::Identity
            );
            output_count
        ]);

        NeuralNetwork {
            input_count,
            output_count,
            layers,
        }
    }
}

impl Genome {
    //only call if new innov added 
    fn update_max_innovation(&mut self){
        self.max_innovation = self.connection_genes.keys().max()
    }
}
