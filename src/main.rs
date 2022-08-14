use crate::state::State;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game_xor;
mod neural_network;
mod environment;
mod state;
mod ui;

fn main() {
    let mut state = State::new(80.0, 60.0, (255, 0, 255));

    ui::start(&mut state);
}
