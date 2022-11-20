use crate::{neat::predictor::Predictor, neat::predictor_environment::PredictorEnvironment};

pub struct EnvironmentXor;

impl PredictorEnvironment for EnvironmentXor {
    type Input = Vec<f64>;
    type Output = Vec<f64>;
    type PredictorScore = f64;

    fn evaluate_predictors<P: Predictor<Self::Input, Self::Output>>(
        &mut self,
        population: &[&P],
    ) -> Vec<f64> {
        population
            .iter()
            .map(|predictor| {
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
                    // TODO the .to_vec() is because I can't figure out how to
                    // make Self::Input be a reference (weird lifetime stuff)
                    score += 1.0
                        - (predictor.predict(datum.0.to_vec())[0] - datum.1)
                            .abs()
                            .clamp(0.0, 1.0);
                }

                (score / training_data.len() as f64).clamp(0.0, 1.0)
            })
            .collect()
    }
}

impl EnvironmentXor {
    pub fn new() -> Self {
        EnvironmentXor {}
    }
}
