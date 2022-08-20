use graphics::Context;
use opengl_graphics::GlGraphics;

pub trait Renderable {
    fn render(
        &self,
        ctx: &mut Context,
        gl: &mut GlGraphics,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );
}
