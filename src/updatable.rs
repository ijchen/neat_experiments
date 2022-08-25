/// A trait for the ability to update an object
/// 
/// Usually, this update modifies the state of the object based
/// on the passage of time, like in a physics simulation, countdown
/// timer, game, etc.
pub trait Updatable {
    /// Updates the object
    /// 
    /// dt is delta-time in seconds
    fn update(&mut self, dt: f64);
}