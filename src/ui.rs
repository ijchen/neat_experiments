use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, MouseCursorEvent};

use crate::state::State;

fn make_window() -> GlutinWindow {
    WindowSettings::new("NEAT Experiments", (800, 600))
        .samples(4)
        .build()
        .expect("Unable to create Glutin window")
}

pub fn start(state: &mut State) {
    // Create a new window and backend graphics library instance
    let mut window = make_window();
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    // Start the event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        // Tick
        if let Some(args) = event.update_args() {
            state.tick(args.dt * 1000.0);
        }

        // Render
        if let Some(args) = event.render_args() {
            let (width, height) = (args.window_size[0], args.window_size[1]);

            gl.draw(args.viewport(), |mut ctx, mut gl| {
                state.render(&mut ctx, &mut gl, width, height);
            });
        }

        // Mouse position
        if let Some(args) = event.mouse_cursor_args() {
            state.event_mouse_pos(args[0], args[1]);
        }

        // Button
        if let Some(args) = event.button_args() {
            state.event_button(&args);
        }
    }
}
