use graphics::{glyph_cache, CharacterCache, Context, Text, Transformed};
use opengl_graphics::{GlGraphics, TextureSettings};
use piston::{Button, ButtonArgs, ButtonState, MouseButton};

use crate::{
    environment::Environment, game_xor::GameXor, genetic_population::GeneticPopulation,
    neural_network::NeuralNetwork, renderable::Renderable,
};

/*
```

// let best = population.get_best().clone();
// best.log();

for generation in 0..=100000 {
    if generation % 1000 == 0 {
        let best = population.get_best().clone();
        println!(
            "correct: 0.0 | prediction: {}",
            best.predict(&vec![0.0, 0.0])[0]
        );
        println!(
            "correct: 1.0 | prediction: {}",
            best.predict(&vec![0.0, 1.0])[0]
        );
        println!(
            "correct: 1.0 | prediction: {}",
            best.predict(&vec![1.0, 0.0])[0]
        );
        println!(
            "correct: 0.0 | prediction: {}",
            best.predict(&vec![1.0, 1.0])[0]
        );
        let score = GameXor::new().evaluate_predictors(&vec![best])[0];
        println!("Generation: {generation} | Score: {score}");
        println!();
    }

    population.advance_generation();
}

// let best = population.get_best().clone();
// best.log();
```
*/

pub struct State {
    population: GeneticPopulation<NeuralNetwork, GameXor>,
}

impl Renderable for State {
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
        let rect = graphics::rectangle::rectangle_by_corners(0.0, 0.0, width, height);
        graphics::rectangle(fill, rect, ctx.transform, gl);

        // Draw the environment
        let env_x = x;
        let env_y = y;
        let env_w = f64::min(height, 2.0 / 3.0 * width);
        let env_h = height;
        self.population
            .environment()
            .render(ctx, gl, env_x, env_y, env_w, env_h);

        // Draw the population
        let pop_x = env_x + env_w;
        let pop_y = env_y;
        let pop_w = width - env_w;
        let pop_h = height / 2.0;
        self.render_info_pane(ctx, gl, pop_x, pop_y, pop_w, pop_h);

        // Draw the infomation pane
        let info_x = env_x + env_w;
        let info_y = pop_y + pop_h;
        let info_w = width - env_w;
        let info_h = height - pop_h;
        self.render_info_pane(ctx, gl, info_x, info_y, info_w, info_h);
    }
}

impl State {
    pub fn new() -> Self {
        const POPULATION_SIZE: usize = 50;

        let environment = GameXor::new();

        let mut nns = vec![];
        for _ in 0..POPULATION_SIZE {
            nns.push(NeuralNetwork::new(
                environment.input_count(),
                environment.output_count(),
                vec![4],
            ));
        }
        let population = GeneticPopulation::new(nns, environment);

        State { population }
    }

    pub fn tick(&mut self, _dt: f64) {
        // Does nothing, for now
    }

    fn render_info_pane(
        &self,
        ctx: &mut Context,
        gl: &mut GlGraphics,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        // Draw the background
        let fill: [f32; 4] = [
            rand::Rng::gen_range(&mut rand::thread_rng(), 0.0..=1.0),
            rand::Rng::gen_range(&mut rand::thread_rng(), 0.0..=1.0),
            rand::Rng::gen_range(&mut rand::thread_rng(), 0.0..=1.0),
            1.0,
        ];
        let rect = graphics::rectangle::rectangle_by_corners(x, y, x + width, y + height);
        graphics::rectangle(fill, rect, ctx.transform, gl);

        // Temporary text
        let fill: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let mut glyphs = glyph_cache::rusttype::GlyphCache::new(
            "assets/fonts/OpenSans-Regular.ttf",
            (),
            TextureSettings::new(),
        )
        .unwrap();
        Text::new_color(fill, 127)
            .draw(
                "Hello, world!",
                &mut glyphs,
                &ctx.draw_state,
                ctx.transform.trans(x + width / 2.0, y + height / 2.0),
                gl,
            )
            .unwrap();
    }

    pub fn event_mouse_pos(&mut self, _x: f64, _y: f64) {
        // Does nothing, for now
    }

    pub fn event_button(&mut self, event: &ButtonArgs) {
        // Mouse button
        if let Button::Mouse(mouse_button) = event.button {
            match mouse_button {
                MouseButton::Left => match event.state {
                    ButtonState::Press => {}
                    ButtonState::Release => {}
                },
                MouseButton::Right => match event.state {
                    ButtonState::Press => {}
                    ButtonState::Release => {}
                },
                _ => {}
            }
        }
    }
}
