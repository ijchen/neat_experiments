/// Arguments for rendering something. Exactly what these are will depend on the
/// graphics backend - this struct is so that users of this code don't have to
/// worry about exactly what is being passed around behind the scenes for rendering.
pub struct RenderArgs {}

impl RenderArgs {
    /// Returns a new instance of RenderArgs with default settings
    pub fn new() -> Self {
        RenderArgs {}
    }
}

/// A trait for the ability to render an object
pub trait Renderable {
    /// Renders the object at the given position
    fn render(&self, args: &RenderArgs, x: f64, y: f64, width: f64, height: f64);
}
