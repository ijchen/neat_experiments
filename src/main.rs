use predictor::Predictor;

use crate::state::State;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod can_crossover;
mod environment;
mod game_xor;
mod neural_network;
mod neural_network_neuron;
mod predictor;
mod state;
mod ui;

fn main() {
    let mut state = State::new(80.0, 60.0, (255, 0, 255));

    let nn = neural_network::NeuralNetwork::new(2, 1, vec![4, 3]);
    let prediction = nn.predict(&vec![0.0, 1.0]);
    println!("Prediction: {prediction:?}");

    ui::start(&mut state);
}
