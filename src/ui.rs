use macroquad::window::Conf;

use crate::{renderable::{Renderable, RenderArgs}, updatable::Updatable, event::EventHandler};

pub fn window_config() -> Conf {
    Conf {
        window_title: "NEAT Experiments".to_string(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        icon: None, // TODO
        ..Default::default()
    }
}

pub async fn start<A: Renderable + Updatable + EventHandler>(app: &mut A) {
    // Background color for clearing the screen between frames
    let bg_color = macroquad::color::Color::from_rgba(0, 0, 0, 255);

    loop {
        // Handle events
        // TODO

        // Update the app
        let dt = macroquad::time::get_frame_time() as f64;
        app.update(dt);

        // Clear the screen between frames
        macroquad::window::clear_background(bg_color);

        // Render the app
        app.render(RenderArgs::new(), 0.0, 0.0, macroquad::window::screen_width() as f64, macroquad::window::screen_height() as f64);

        // Await the next frame
        macroquad::window::next_frame().await
    }
}
