use crate::{renderable::Renderable, updatable::Updatable};

pub struct SimpleExampleApp {
    elapsed: f64
}

impl Renderable for SimpleExampleApp {
    fn render(&self, _x: f64, _y: f64, _width: f64, height: f64) {
        use macroquad::prelude::*;

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(mouse_position().0, mouse_position().1, 15.0, YELLOW);
        draw_text("Hello, world!", 20.0, 40.0, 30.0, Color::from_rgba(255, 0, 0, 255));

        let elapsed_text = format!("Elapsed time: {:.2}s", self.elapsed);
        draw_text(&elapsed_text, 20.0, height as f32 - 20.0, 24.0, Color::from_rgba(255, 255, 255, 255));
    }
}

impl Updatable for SimpleExampleApp {
    fn update(&mut self, dt: f64) {
        self.elapsed += dt;
    }
}

impl SimpleExampleApp {
    pub fn new() -> Self {
        SimpleExampleApp {
            elapsed: 0.0
        }
    }
}