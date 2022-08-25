use crate::{renderable::Renderable, updatable::Updatable, event::EventHandler};

pub struct SimpleExampleApp {}

impl Renderable for SimpleExampleApp {
    fn render(&self, _args: crate::renderable::RenderArgs, _x: f64, _y: f64, _width: f64, _height: f64) {
        use macroquad::prelude::*;

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("Hello, world!", 20.0, 40.0, 30.0, Color::from_rgba(255, 0, 0, 255));
    }
}

impl Updatable for SimpleExampleApp {
    fn update(&mut self, dt: f64) {
        // Does nothing for now
    }
}

impl EventHandler for SimpleExampleApp {
    fn handle_event(&mut self, event: &crate::event::Event) {
        // Does nothing for now
    }
}

impl SimpleExampleApp {
    pub fn new() -> Self {
        SimpleExampleApp {  }
    }
}