use macroquad::window::Conf;
use example_app::ExampleApp;

mod renderable;
mod updatable;
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
mod example_app;

fn window_config() -> Conf {
    crate::ui::window_config()
}

#[macroquad::main(window_config)]
async fn main() {
    let mut app = ExampleApp::new();

    ui::start(&mut app).await;
}
