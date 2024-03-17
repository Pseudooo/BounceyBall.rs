extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::fmt::Debug;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Button, ButtonEvent, ButtonState, Event, Input, MouseButton, MouseCursorEvent};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use crate::app::App;
use crate::ball::Ball;
use crate::vector::Vector;

mod app;
mod vector;
mod ball;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let ball = Ball {
        x: 250.0,
        y: 250.0,
        vel: Vector::new(500.0, -25.0),
        radius: 10.0,
        colour: RED
    };

    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        last_mouse_pos: Vector::new(0.0, 0.0),
        gravity: 2750.0,
        ball,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(mouse_args) = e.mouse_cursor_args() {
            app.last_mouse_pos = Vector::new(mouse_args[0], mouse_args[1]);
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
                            let mouse_pos = &app.last_mouse_pos;
                            let ball = &mut app.ball;

                            const VEL_SCALE: f64 = 2.5;

                            ball.vel.i += VEL_SCALE * (mouse_pos.i - ball.x);
                            ball.vel.j += VEL_SCALE * (mouse_pos.j - ball.y);

                            println!("CLICK");
                        }
                        _ => ()
                    }
                    _ => ()
                }
                _ => ()
            }
            _ => ()
        }
    }
}
