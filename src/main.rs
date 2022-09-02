use macroquad::window::Conf;
use xor_app::XorApp;

mod can_crossover;
mod can_mutate;
mod environment;
mod environment_xor;
mod genetic_population;
mod neural_network;
mod neural_network_neuron;
mod predictor;
mod renderable;
mod ui;
mod updatable;
mod xor_app;

fn window_config() -> Conf {
    crate::ui::window_config()
}

#[macroquad::main(window_config)]
async fn main() {
    let mut app = XorApp::new();

    ui::start(&mut app).await;
}
