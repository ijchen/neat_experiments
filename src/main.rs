mod can_crossover;
mod can_mutate;
mod environment;
mod environment_xor;
mod frontend;
mod genetic_population;
mod neural_network;
mod neural_network_neuron;
mod predictor;
mod xor;

use macroquad::window::Conf;
use xor::xor_app::XorApp;

fn window_config() -> Conf {
    frontend::ui::window_config()
}

#[macroquad::main(window_config)]
async fn main() {
    let mut app = XorApp::new();

    frontend::ui::start(&mut app).await;
}
