use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::{
    Event::{Input, Loop},
    Input::{Button, Move},
    Loop::{Render, Update},
    Motion::MouseCursor,
};
use piston_window::PistonWindow;

use crate::state::State;

fn make_window() -> PistonWindow {
    WindowSettings::new("NEAT Experiments", (1200, 675))
        .samples(4)
        .build()
        .expect("Unable to create window")
}

pub fn start(state: &mut State) {
    // Create a new window
    let mut window = make_window();

    // TODO Refactor this so it's not passing a random font around everywhere
    let mut glyphs = window
        .load_font("assets/fonts/OpenSans-Regular.ttf")
        .unwrap();

    // Start the event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        match event {
            Loop(loop_event) => match loop_event {
                Render(render_args) => {
                    let (width, height) = (render_args.window_size[0], render_args.window_size[1]);

                    window.draw_2d(&event, |mut ctx, mut gl, mut device| {
                        state.render(&mut glyphs, &mut device, &mut ctx, gl, width, height);
                    });
                }
                Update(update_args) => {
                    state.tick(update_args.dt * 1000.0);
                }
                _ => {}
            },
            Input(input_event, _) => match input_event {
                Button(button_args) => state.event_button(&button_args),
                Move(motion) => match motion {
                    MouseCursor([x, y]) => state.event_mouse_pos(x, y),
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}
