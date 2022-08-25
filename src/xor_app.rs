use crate::{renderable::{Renderable, RenderArgs}, updatable::Updatable};

pub struct XorApp {
    elapsed: f64
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
        self.elapsed += dt;
    }
}

impl XorApp {
    pub fn new() -> Self {
        XorApp {
            elapsed: 0.0
        }
    }

    fn render_environment(&self, _args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;
        
        let fill = Color::from_rgba(255, 255, 255, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);
        // Transform x, y, width, and height so that we only work in a max-size centered square
        let (env_inner_x, env_inner_y, env_inner_w, env_inner_h) = {
            let side_len = f64::min(width, height);
            (
                x + (width - side_len) / 2.0,
                y + (height - side_len) / 2.0,
                side_len,
                side_len,
            )
        };
        let fill = Color::from_rgba(0, 0, 255, 255);
        draw_rectangle(env_inner_x as f32, env_inner_y as f32, env_inner_w as f32, env_inner_h as f32, fill);
    }

    fn render_population(&self, _args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        let fill = Color::from_rgba(255, 127, 0, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);
    }

    fn render_info_pane(&self, args: &RenderArgs, x: f64, y: f64, width: f64, height: f64) {
        use macroquad::prelude::*;

        let fill = Color::from_rgba(0, 0, 0, 255);
        draw_rectangle(x as f32, y as f32, width as f32, height as f32, fill);
        let elapsed_text = format!("Elapsed time: {:.2}s", self.elapsed);
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
    }
}