mod frontend;
mod neat;
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
