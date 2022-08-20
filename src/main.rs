use state::State;

extern crate gfx_device_gl;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

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
