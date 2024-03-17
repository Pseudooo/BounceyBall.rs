use graphics::ellipse::circle;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::ball::Ball;

pub struct App {
        pub gl: GlGraphics,
        pub gravity: f64,
        pub ball: Ball
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let ball = &self.ball;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            let transform = c.transform;
            let x = circle(ball.x, ball.y, ball.radius);
            circle_arc(ball.colour, 10.0, 0.0, 6.28, x, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {

        let ball_radius = self.ball.radius + 10.0;
        const BOUNCE_DAMPING: f64 = 1.0;

        if self.ball.y + ball_radius > 500.0 {
            self.ball.vel.j = -self.ball.vel.j * BOUNCE_DAMPING;
            self.ball.y = 500.0 - ball_radius;
        } else if self.ball.y - ball_radius < 0.0 {
            self.ball.vel.j = -self.ball.vel.j * BOUNCE_DAMPING;
            self.ball.y = ball_radius;
        }
        self.ball.vel.j += self.gravity * args.dt;
        self.ball.y += distance(self.ball.vel.j, self.gravity, args.dt);

        if self.ball.x + ball_radius > 500.0 {
            self.ball.vel.i = -self.ball.vel.i * BOUNCE_DAMPING;
            self.ball.x = 500.0 - ball_radius;
        } else if self.ball.x - ball_radius < 0.0 {
            self.ball.vel.i = -self.ball.vel.i * BOUNCE_DAMPING;
            self.ball.x = ball_radius;
        }
        self.ball.x += self.ball.vel.i * args.dt;
    }
}


fn distance(v: f64, a: f64, t: f64) -> f64 {
    (v * t) - (0.5 * a * t * t)
}