use crate::{
    frontend::renderable::{RenderArgs, Renderable},
    frontend::updatable::Updatable,
    neat::{
        genome::Genome, implementation_config, neat_config::NeatConfig, population::Population,
        predictor::Predictor, predictor_environment::PredictorEnvironment,
    },
};

use super::environment_xor::EnvironmentXor;

pub struct XorApp {
    elapsed: f64,
    population: Population,
    environment: EnvironmentXor,
    best: (Genome, Option<f64>),
}

impl Renderable for XorApp {
    fn render(&self, args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        // Draw the background
        let fill = Color::from_rgba(255, 255, 255, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);

        // Draw the environment
        let env_x = x;
        let env_y = y;
        let env_w = f64::min(height, 2.0 / 3.0 * width);
        let env_h = height;
        self.render_environment(args, env_x, env_y, env_w, env_h);

        // Draw the model
        let pop_x = env_x + env_w;
        let pop_y = env_y;
        let pop_w = width - env_w;
        let pop_h = height / 2.0;
        self.render_model(args, pop_x, pop_y, pop_w, pop_h);

        // Draw the infomation pane
        let info_x = env_x + env_w;
        let info_y = pop_y + pop_h;
        let info_w = width - env_w;
        let info_h = height - pop_h;
        self.render_info_pane(args, info_x, info_y, info_w, info_h);
    }
}

impl Updatable for XorApp {
    fn update(&mut self, dt: f64) {
        const GENERATIONS_PER_SECOND: f64 = 100.0;
        const SECONDS_PER_GENERATION: f64 = 1.0 / GENERATIONS_PER_SECOND;
        const MAX_TIME: f64 = 1.0 / 30.0 + SECONDS_PER_GENERATION;

        self.elapsed += dt;

        // If we're falling behind, skip generations to maintain FPS
        if self.elapsed >= MAX_TIME {
            let skipped_generations =
                ((self.elapsed - MAX_TIME) / SECONDS_PER_GENERATION).ceil() as u32;
            self.elapsed -= skipped_generations as f64 * SECONDS_PER_GENERATION;
            eprintln!("Can't keep up! Skipping {skipped_generations} generations");
        }

        // Advance the generation based on how much time has passed
        while self.elapsed >= SECONDS_PER_GENERATION {
            self.elapsed -= SECONDS_PER_GENERATION;

            // Advance generation
            let genomes = self.population.get_genomes();
            let scores = self.environment.evaluate_predictors(&genomes);
            assert_eq!(genomes.len(), scores.len());
            let best = genomes
                .iter()
                .copied()
                .zip(scores.iter())
                .reduce(|accum, item| if item.1 > accum.1 { item } else { accum })
                .unwrap();
            self.best = (best.0.clone(), Some(*best.1));
            self.population.evolve(&scores);
        }
    }
}

impl XorApp {
    pub fn new() -> Self {
        let config = NeatConfig::default();
        let population = Population::new(2, 1, &config);
        let best = (population.get_genomes()[0].clone(), None);
        XorApp {
            elapsed: 0.0,
            population,
            environment: EnvironmentXor::new(),
            best,
        }
    }

    fn render_environment(&self, _args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        // Draw a white background
        let fill = Color::from_rgba(255, 255, 255, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);

        // Transform x, y, width, and height so that we only work in a max-size centered square
        let (x, y, width, height) = {
            let side_len = f64::min(width, height);
            (
                x + (width - side_len) / 2.0,
                y + (height - side_len) / 2.0,
                side_len,
                side_len,
            )
        };

        // Render the XOR field
        let resolution = 100;
        let cell_w = width / resolution as f64;
        let cell_h = height / resolution as f64;
        for row in 0..resolution {
            for col in 0..resolution {
                let coord_x = col as f64 / resolution as f64 + 1.0 / (2.0 * resolution as f64);
                let coord_y = row as f64 / resolution as f64 + 1.0 / (2.0 * resolution as f64);
                let coord_y = 1.0 - coord_y; // Invert y (graphics coordinates go top-to-bottom, unlike typical cartesian coordinates)

                // TODO
                // let brightness = (coord_x + coord_y - 2.0 * coord_x * coord_y).clamp(0.0, 1.0);
                let brightness = self.best.0.predict(vec![coord_x, coord_y])[0].clamp(0.0, 1.0);
                let color = Color::from_rgba(
                    (brightness * 255.0).round() as u8,
                    (brightness * 255.0).round() as u8,
                    (brightness * 255.0).round() as u8,
                    255,
                );

                let rect_x = x + cell_w * col as f64;
                let rect_y = y + cell_h * row as f64;

                draw_rectangle(
                    rect_x as f32,
                    rect_y as f32,
                    cell_w as f32,
                    cell_h as f32,
                    color,
                );
            }
        }
    }

    fn render_model(&self, _args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        // Background
        let fill = Color::from_rgba(0, 0, 0, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);

        // Render the neural network
        // TODO
    }

    fn render_info_pane(&self, args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        // Background
        let fill = Color::from_rgba(255, 255, 255, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);

        // Fitness text
        let score_text = match self.best.1 {
            Some(score) => format!("{score:.4}"),
            None => "Not evaluated".to_string(),
        };
        let fitness_text = format!("Fitness: {score_text}");
        let padding = width.min(height * 2.0) as f32 / 25.0;
        let font_size = (width.min(height * 2.0) / 20.0).max(8.0) as f32;
        let text_params = TextParams {
            font: args.font,
            font_size: font_size.round() as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::from_rgba(0, 0, 0, 255),
        };
        draw_text_ex(
            &fitness_text,
            x as f32 + padding,
            y as f32 + height as f32 - padding * 2.0 - font_size as f32,
            text_params,
        );

        // Generation text
        let elapsed_text = format!("Generation: {}", self.population.generation());
        let padding = width.min(height * 2.0) as f32 / 25.0;
        let font_size = (width.min(height * 2.0) / 20.0).max(8.0) as f32;
        let text_params = TextParams {
            font: args.font,
            font_size: font_size.round() as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::from_rgba(0, 0, 0, 255),
        };
        draw_text_ex(
            &elapsed_text,
            x as f32 + padding,
            y as f32 + height as f32 - padding,
            text_params,
        );
    }
}
