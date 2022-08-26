use macroquad::{window::Conf, text::load_ttf_font};

use crate::{renderable::{Renderable, RenderArgs}, updatable::Updatable};

/// Make config information available to main for constructing a window
pub fn window_config() -> Conf {
    Conf {
        window_title: "NEAT Experiments".to_string(),
        window_width: 960,
        window_height: 540,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        icon: None, // TODO
        ..Default::default()
    }
}

pub async fn start<A: Renderable + Updatable>(app: &mut A) {
    // Prepare and preload assets
    // TODO assets loading screen
    let bg_color = macroquad::color::Color::from_rgba(0, 0, 0, 255);
    let font = load_ttf_font("assets/fonts/OpenSans-Regular.ttf").await.unwrap();
    let render_args = RenderArgs::new(font);
    
    loop {
        // Update the app
        let dt = macroquad::time::get_frame_time() as f64;
        app.update(dt);

        // Clear the screen between frames
        macroquad::window::clear_background(bg_color);

        // Render the app
        app.render(&render_args, 0.0, 0.0, macroquad::window::screen_width() as f64, macroquad::window::screen_height() as f64);

        // Await the next frame
        macroquad::window::next_frame().await
    }
}
