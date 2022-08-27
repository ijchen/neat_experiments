use crate::{environment::Environment, predictor::Predictor};

pub struct EnvironmentXor {}

impl Environment for EnvironmentXor {
    fn input_count(&self) -> usize {
        2
    }

    fn output_count(&self) -> usize {
        1
    }

    fn evaluate_predictors<P: Predictor>(&mut self, population: &[&P]) -> Vec<f64> {
        let mut scores: Vec<f64> = vec![];

        for predictor in population {
            // Ensure the predictor and this game are on the same page
            // about how many inputs and outputs there should be
            assert!(predictor.input_count() == self.input_count());
            assert!(predictor.output_count() == self.output_count());

            // For something as simple as XOR, we can test all possible inputs
            // Overfitting is not a concern here, since the point isn't to generalize,
            // but to test the predictor's ability to learn non-linear functions
            let mut score = 0.0;
            let training_data = vec![
                // Standard XOR
                ([0.0, 0.0], 0.0),
                ([0.0, 1.0], 1.0),
                ([1.0, 0.0], 1.0),
                ([1.0, 1.0], 0.0),

                // Add this for a center point
                // ([0.5, 0.5], 0.5),

                // Edge points and corner reinforcement
                // ([0.0, 0.0], 0.0),
                // ([0.0, 1.0], 1.0),
                // ([1.0, 0.0], 1.0),
                // ([1.0, 1.0], 0.0),
                // ([0.0, 0.5], 0.5),
                // ([1.0, 0.5], 0.5),
                // ([0.5, 0.0], 0.5),
                // ([0.5, 1.0], 0.5),
            ];
            for datum in &training_data {
                score += 1.0 - (predictor.predict(&datum.0)[0] - datum.1).abs();
            }
            score = f64::max(score / training_data.len() as f64, 0.0);

            scores.push(score);
        }

        scores
    }
}

impl EnvironmentXor {
    pub fn new() -> Self {
        EnvironmentXor {}
    }
}
