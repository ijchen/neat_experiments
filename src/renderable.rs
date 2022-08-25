/// A trait for the ability to render an object
pub trait Renderable {
    /// Renders the object at the given position
    fn render(&self, x: f64, y: f64, width: f64, height: f64);
}