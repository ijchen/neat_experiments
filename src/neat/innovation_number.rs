#[derive(Copy, Clone, Debug)]
pub struct InnovationNumber(u64);

/// A generator for unique `InnovationNumber`s
pub struct InnovationNumberGenerator {
    curr: u64,
}

impl InnovationNumberGenerator {
    /// Constructs a new InnovationNumberGenerator
    pub fn new() -> Self {
        Self { curr: 0 }
    }

    /// Returns a new unique InnovationNumber that has never been returned by
    /// this generator before
    pub fn next(&mut self) -> InnovationNumber {
        let innovation_number = InnovationNumber(self.curr);
        self.curr += 1;
        innovation_number
    }
}
