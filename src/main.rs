use macroquad::window::Conf;
use example_app::ExampleApp;

mod renderable;
mod updatable;
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
