use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::{
    Event::{Input, Loop},
    Input::{Button, Move},
    Loop::{Render, Update},
    Motion::MouseCursor,
};

use crate::renderable::Renderable;
use crate::state::State;

fn make_window() -> GlutinWindow {
    WindowSettings::new("NEAT Experiments", (1200, 675))
        .samples(4)
        .build()
        .expect("Unable to create window")
}

pub fn start(state: &mut State) {
    // Create a new window
    let mut window = make_window();
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    // Start the event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        match event {
            Loop(loop_event) => match loop_event {
                Render(render_args) => {
                    let (width, height) = (render_args.window_size[0], render_args.window_size[1]);

                    gl.draw(render_args.viewport(), |mut ctx, gl| {
                        state.render(&mut ctx, gl, 0.0, 0.0, width, height);
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
