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
mod state;
mod ui;

fn main() {
    // TODO
    if false {
        let mut state = State::new(80.0, 60.0, (255, 0, 255));

        ui::start(&mut state);
    }

    let mut nns = vec![];
    for _ in 0..1000 {
        nns.push(neural_network::NeuralNetwork::new(2, 1, vec![2]));
    }
    let mut population = GeneticPopulation::new(nns, GameXor::new());

    let best = population.get_best().clone();
    best.log();

    for generation in 0..20 {
        let best = population.get_best().clone();
        println!(
            "correct: 0.0 | prediction: {}",
            best.predict(&vec![0.0, 0.0])[0]
        );
        println!(
            "correct: 1.0 | prediction: {}",
            best.predict(&vec![0.0, 1.0])[0]
        );
        println!(
            "correct: 1.0 | prediction: {}",
            best.predict(&vec![1.0, 0.0])[0]
        );
        println!(
            "correct: 0.0 | prediction: {}",
            best.predict(&vec![1.0, 1.0])[0]
        );
        let score = GameXor::new().evaluate_predictors(&vec![best])[0];
        println!("Generation: {generation} | Score: {score}");
        println!();

        population.advance_generation();
    }

    let best = population.get_best().clone();
    best.log();
}
