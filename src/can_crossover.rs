pub trait CanCrossover {
    /// Produces a new crossover of this and another of the same type.
    fn crossover(&self, other: &Self) -> Self;
}
