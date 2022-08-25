use macroquad::prelude::{KeyCode, is_key_pressed};

use crate::{renderable::{Renderable, RenderArgs}, updatable::Updatable};

/// A simple example app for demonstration purposes
pub struct ExampleApp {
    elapsed: f64
}

impl Renderable for ExampleApp {
    fn render(&self, _args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        // Background
        let fill = Color::from_rgba(20, 20, 20, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);

        // Very important random line
        let fill = Color::from_rgba(0, 0, 255, 255);
        draw_line(600.0, 400.0, 100.0, 200.0, 8.0, fill);

        // Circle at the mouse position
        let fill = Color::from_rgba(255, 255, 0, 255);
        draw_circle(mouse_position().0, mouse_position().1, 15.0, fill);

        // Text rendering
        let fill = Color::from_rgba(255, 255, 255, 255);
        let elapsed_text = format!("Elapsed time: {:.2}s", self.elapsed);
        let padding = 20.0;
        draw_text(&elapsed_text, padding, height as f32 - padding, 24.0, fill);
    }
}

impl Updatable for ExampleApp {
    fn update(&mut self, dt: f64) {
        self.elapsed += dt;

        // Reset the elapsed time if the user presses space, because why not
        if is_key_pressed(KeyCode::Space) {
            self.elapsed = 0.0;
        }
    }
}

impl ExampleApp {
    pub fn new() -> Self {
        ExampleApp {
            elapsed: 0.0
        }
    }
}