use macroquad::text::Font;

/// Arguments for rendering something. Exactly what these are will depend on the
/// graphics backend - this struct is so that users of this code don't have to
/// worry about exactly what is being passed around behind the scenes for rendering.
pub struct RenderArgs {
    pub font: Font,
}

impl RenderArgs {
    /// RenderArgs constructor
    pub fn new(font: Font) -> Self {
        RenderArgs { font }
    }
}

/// A trait representing the ability for something to render itself
pub trait Renderable {
    /// Renders itself at the given position
    fn render(&self, args: &RenderArgs, x: f64, y: f64, width: f64, height: f64);
}
