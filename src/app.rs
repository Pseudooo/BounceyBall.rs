use graphics::ellipse::circle;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

pub struct App {
        pub gl: GlGraphics,
        pub gravity: f64,
        pub ball: Ball
    }

    pub struct Ball {
        pub x: f64,
        pub y: f64,
        pub radius: f64,
        pub vel: f64,
        pub colour: [f32; 4],
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
            // +10 because of the thickness of the circle arc
            if self.ball.y + self.ball.radius + 10.0 < 500.0 {
                self.ball.vel += self.gravity * args.dt;
            } else {
                self.ball.vel = -self.ball.vel;
            }
            self.ball.y += (self.ball.vel * args.dt) - (0.5 * self.gravity * args.dt * args.dt);
        }
    }
