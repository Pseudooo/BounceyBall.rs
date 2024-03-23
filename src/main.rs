extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Button, ButtonState, Event, Input, MouseButton, MouseCursorEvent, Window as OtherWindow};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use crate::app::App;

mod app;
mod vector;
mod ball;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Bouncy Ball", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        last_mouse_pos: [0.0, 0.0],
        gravity: 3500.0,
        balls: Vec::new(),
        win_size: [500.0, 500.0]
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        app.win_size = [window.size().width, window.size().height];

        if let Some(mouse_args) = e.mouse_cursor_args() {
            app.last_mouse_pos = mouse_args;
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        match e {
            Event::Input(ref input, ..) => match input {
                Input::Button(button) => match button.state {
                    ButtonState::Press => match button.button {
                        Button::Mouse(MouseButton::Left) => {
                            app.mouse_push();
                        }
                        _ => ()
                    },
                    _ => ()
                }
                _ => ()
            }
            _ => ()
        }
    }
}
