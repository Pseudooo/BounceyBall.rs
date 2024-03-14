

    use graphics::ellipse::circle;
    use opengl_graphics::GlGraphics;
    use piston::{RenderArgs, UpdateArgs};

    pub struct App {
        pub gl: GlGraphics,
        pub ball: Ball
    }

    pub struct Ball {
        pub x: f64,
        pub y: f64,
        pub radius: f64,
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
            })
        }

        pub fn update(&mut self, args: &UpdateArgs) {
            self.ball.y += 10.0 * args.dt;
        }
    }
