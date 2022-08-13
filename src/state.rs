use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonArgs, ButtonState, MouseButton};

pub struct State {
    pos: (f64, f64),
    size: (f64, f64),
    color: (u8, u8, u8),
    bg: (u8, u8, u8),
}

impl State {
    pub fn new(rect_w: f64, rect_h: f64, color: (u8, u8, u8)) -> Self {
        State {
            pos: (0.0, 0.0),
            size: (rect_w, rect_h),
            color: color,
            bg: (0, 0, 0),
        }
    }

    pub fn tick(&mut self, _dt: f64) {
        // Does nothing, for now
    }

    pub fn render(&mut self, ctx: &mut Context, gl: &mut GlGraphics, _width: f64, _height: f64) {
        // Draw the background
        let fill: [f32; 4] = [
            self.bg.0 as f32 / 255.0,
            self.bg.1 as f32 / 255.0,
            self.bg.2 as f32 / 255.0,
            1.0,
        ];
        graphics::clear(fill, gl);

        // Draw the box
        let fill: [f32; 4] = [
            self.color.0 as f32 / 255.0,
            self.color.1 as f32 / 255.0,
            self.color.2 as f32 / 255.0,
            1.0,
        ];
        let square = graphics::rectangle::rectangle_by_corners(
            self.pos.0 - self.size.0 as f64 / 2.0,
            self.pos.1 - self.size.1 as f64 / 2.0,
            self.pos.0 + self.size.0 as f64 / 2.0,
            self.pos.1 + self.size.1 as f64 / 2.0,
        );
        graphics::rectangle(fill, square, ctx.transform, gl);
    }

    pub fn event_mouse_pos(&mut self, x: f64, y: f64) {
        self.pos = (x, y);
    }

    pub fn event_button(&mut self, event: &ButtonArgs) {
        // Mouse button
        if let Button::Mouse(mouse_button) = event.button {
            match mouse_button {
                MouseButton::Left => {
                    if event.state == ButtonState::Press {
                        self.bg = (10, 20, 50);
                    } else {
                        self.bg = (0, 0, 0);
                    }
                }
                _ => {}
            }
        }
    }
}
