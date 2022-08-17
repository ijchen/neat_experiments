use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::{environment::Environment, predictor::Predictor, renderable::Renderable};

pub struct GameXor {}

impl Renderable for GameXor {
    fn render(
        &self,
        ctx: &mut Context,
        gl: &mut GlGraphics,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        // Draw the background
        let fill: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let rect = graphics::rectangle::rectangle_by_corners(x, y, x + width, y + height);
        graphics::rectangle(fill, rect, ctx.transform, gl);
    }
}

impl Environment for GameXor {
    fn input_count(&self) -> usize {
        2
    }

    fn output_count(&self) -> usize {
        1
    }

    fn evaluate_predictors<P: Predictor>(&mut self, population: &Vec<P>) -> Vec<f64> {
        let mut scores: Vec<f64> = vec![];

        for predictor in population {
            // Ensure the predictor and this game are on the same page
            // about how many inputs and outputs there should be
            debug_assert!(predictor.input_count() == self.input_count());
            debug_assert!(predictor.output_count() == self.output_count());

            // For something as simple as XOR, we can test all possible inputs
            // Overfitting is not a concern here, since the point isn't to generalize,
            // but to test the predictor's ability to learn non-linear functions
            let mut score = 0.0;
            score += 1.0 - (predictor.predict(&vec![0.0, 0.0])[0] - 0.0).abs();
            score += 1.0 - (predictor.predict(&vec![0.0, 1.0])[0] - 1.0).abs();
            score += 1.0 - (predictor.predict(&vec![1.0, 0.0])[0] - 1.0).abs();
            score += 1.0 - (predictor.predict(&vec![1.0, 1.0])[0] - 0.0).abs();
            score = f64::max(score / 4.0, 0.0);

            scores.push(score);
        }

        scores
    }
}

impl GameXor {
    pub fn new() -> Self {
        GameXor {}
    }
}
