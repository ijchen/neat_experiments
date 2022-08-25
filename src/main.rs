use macroquad::window::Conf;
use simple_example_app::SimpleExampleApp;

mod renderable;
mod updatable;
mod ui;
mod simple_example_app;

fn window_config() -> Conf {
    crate::ui::window_config()
}

#[macroquad::main(window_config)]
async fn main() {
    let mut app = SimpleExampleApp::new();

    ui::start(&mut app).await;
}
