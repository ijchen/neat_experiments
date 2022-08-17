use genetic_population::GeneticPopulation;
use neural_network_neuron::NeuralNetworkActivationFun;
use predictor::Predictor;

use crate::{environment::Environment, game_xor::GameXor, state::State};

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod can_crossover;
mod can_mutate;
mod environment;
mod game_xor;
mod genetic_population;
mod neural_network;
mod neural_network_neuron;
mod predictor;
mod renderable;
mod state;
mod ui;

fn main() {
    let mut state = State::new();

    ui::start(&mut state);
}
