use crate::{neat::environment::Environment, neat::predictor::Predictor};

pub struct EnvironmentXor;

impl Environment<2, 1> for EnvironmentXor {
    fn evaluate_predictors<const N: usize, P: Predictor<2, 1>>(
        &mut self,
        population: &[&P; N],
    ) -> [f64; N] {
        // TODO *Consider* using MaybeUninit to avoid filling with zeros here
        let mut scores: [f64; N] = [0.0; N];

        for (i, predictor) in population.iter().enumerate() {
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
            score = (score / training_data.len() as f64).max(0.0);

            scores[i] = score;
        }

        scores
    }
}

impl EnvironmentXor {
    pub fn new() -> Self {
        EnvironmentXor {}
    }
}
