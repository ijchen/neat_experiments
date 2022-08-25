use macroquad::window::Conf;
use xor_app::XorApp;

mod renderable;
mod updatable;
mod ui;
mod simple_example_app;
mod xor_app;

fn window_config() -> Conf {
    crate::ui::window_config()
}

#[macroquad::main(window_config)]
async fn main() {
    let mut app = XorApp::new();

    ui::start(&mut app).await;
}
