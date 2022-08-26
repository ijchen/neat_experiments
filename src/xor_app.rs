use crate::{renderable::{Renderable, RenderArgs}, updatable::Updatable, genetic_population::GeneticPopulation, neural_network::NeuralNetwork, environment_xor::EnvironmentXor, environment::Environment, predictor::Predictor};

pub struct XorApp {
    elapsed: f64,
    population: GeneticPopulation<NeuralNetwork, EnvironmentXor>
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

        // Draw the population
        let pop_x = env_x + env_w;
        let pop_y = env_y;
        let pop_w = width - env_w;
        let pop_h = height / 2.0;
        self.render_population(args, pop_x, pop_y, pop_w, pop_h);

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
        const MAX_TIME: f64 = 1.0 / 30.0;

        self.elapsed += dt;

        // If we're falling behind, skip generations to maintain FPS
        if self.elapsed >= MAX_TIME {
            let skipped_generations = ((self.elapsed - MAX_TIME) / SECONDS_PER_GENERATION).ceil() as u32;
            self.elapsed -= skipped_generations as f64 * SECONDS_PER_GENERATION;
            eprintln!("Can't keep up! Skipping {skipped_generations} generations");
        }

        // Advance the generation based on how much time has passed
        while self.elapsed >= SECONDS_PER_GENERATION {
            self.elapsed -= SECONDS_PER_GENERATION;

            self.population.advance_generation();
        }
    }
}

impl XorApp {
    pub fn new() -> Self {
        let environment = EnvironmentXor::new();
        let mut population = vec![];
        const POPULATION_SIZE: usize = 100;
        for _ in 0..POPULATION_SIZE {
            let nn = NeuralNetwork::new(environment.input_count(), environment.output_count(), vec![4]);
            
            population.push(nn);
        }

        let genetic_population: GeneticPopulation<NeuralNetwork, EnvironmentXor> = GeneticPopulation::new(population, environment);

        XorApp {
            elapsed: 0.0,
            population: genetic_population
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
        if let Some(best) = self.population.get_prev_best() {
            let resolution = 100;
            let cell_w = width / resolution as f64;
            let cell_h = height / resolution as f64;
            for row in 0..resolution {
                for col in 0..resolution {
                    let coord_x = col as f64 / resolution as f64 + 1.0 / (2.0 * resolution as f64);
                    let coord_y = row as f64 / resolution as f64 + 1.0 / (2.0 * resolution as f64);
                    let coord_y = 1.0 - coord_y; // Invert y (graphics coordinates go top-to-bottom, unlike typical cartesian coordinates)
                    // let brightness = coord_y * coord_x + (1.0 - coord_y) * (1.0 - coord_x);
                    let brightness = best.predict(&[coord_x, coord_y])[0].clamp(0.0, 1.0);
                    let color = Color::from_rgba(
                        (brightness * 255.0).round() as u8,
                        (brightness * 255.0).round() as u8,
                        (brightness * 255.0).round() as u8,
                        255
                    );

                    let rect_x = x + cell_w * col as f64;
                    let rect_y = y + cell_h * row as f64;

                    draw_rectangle(rect_x as f32, rect_y as f32, cell_w as f32, cell_h as f32, color);
                }
            }
        };
    }

    fn render_population(&self, _args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        // Background
        let fill = Color::from_rgba(255, 127, 0, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);
    }

    fn render_info_pane(&self, args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        // Background
        let fill = Color::from_rgba(0, 0, 0, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);

        // Generation text
        let elapsed_text = format!("Generation: {}", self.population.generation());
        let padding = width as f32 / 25.0;
        let font_size = f64::max(8.0, width / 20.0) as f32;
        let text_params = TextParams {
            font: args.font,
            font_size: font_size.round() as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::from_rgba(255, 255, 255, 255),
        };
        draw_text_ex(&elapsed_text, x as f32 + padding, y as f32 + height as f32 - padding, text_params);
        
        // Fitness text
        if let Some(best) = self.population.get_prev_best() {
            let score = EnvironmentXor::new().evaluate_predictors(&[best])[0];
            let elapsed_text = format!("Fitness: {score:.4}");
            let padding = width as f32 / 25.0;
            let font_size = f64::max(8.0, width / 20.0) as f32;
            let text_params = TextParams {
                font: args.font,
                font_size: font_size.round() as u16,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                color: Color::from_rgba(255, 255, 255, 255),
            };
            draw_text_ex(&elapsed_text, x as f32 + padding, y as f32 + height as f32 - padding * 2.0 - font_size as f32, text_params);
        };
    }
}