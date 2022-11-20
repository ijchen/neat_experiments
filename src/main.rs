mod frontend;
mod neat;
mod xor;

use macroquad::window::Conf;
use xor::xor_app::XorApp;

fn window_config() -> Conf {
    frontend::ui::window_config()
}

// TODO make sure I'm handling zero populations okay (no panics or anything)
#[macroquad::main(window_config)]
async fn main() {
    let mut app = XorApp::new();

    frontend::ui::start(&mut app).await;
}
