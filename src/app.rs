use graphics::color::GREEN;
use graphics::ellipse::circle;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::ball::Ball;
use crate::vector::Vector;

pub struct App {
        pub gl: GlGraphics,
        pub last_mouse_pos: [f64; 2],
        pub gravity: f64,
        pub balls: Vec<Ball>,
        pub win_size: [f64; 2]
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let balls = &self.balls;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            for ball in balls {
                let transform = c.transform;
                let x = circle(ball.x, ball.y, ball.radius);
                circle_arc(ball.colour, 10.0, 0.0, 6.28, x, transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        const BOUNCE_DAMPING: f64 = 0.9;
        for ball in &mut self.balls {
            let force = Vector::new(0.0, self.gravity);
            ball.tick(force, args.dt, self.win_size);
        }
    }

    pub fn mouse_push(&mut self) {
        let [x, y] = self.last_mouse_pos;

        self.balls.push(Ball {
            colour: GREEN,
            x, y,
            radius: 10.0,
            vel: Vector::new(0.0, 0.0),
        })

    }
}