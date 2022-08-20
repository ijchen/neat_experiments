use gfx_device_gl::Device;
use graphics::Context;
use piston_window::{G2d, Glyphs};

pub trait Renderable {
    fn render(
        &self,
        glyphs: &mut Glyphs,
        device: &mut Device,
        ctx: &mut Context,
        gl: &mut G2d,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );
}
