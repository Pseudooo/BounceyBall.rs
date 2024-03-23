use graphics::color::PURPLE;
use graphics::ellipse::circle;
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::{RenderArgs, UpdateArgs};

use crate::ball::Ball;
use crate::vector::Vector;

pub struct App {
        pub gl: GlGraphics,
        pub fps: i32,
        pub last_mouse_pos: Vector,
        pub gravity: f64,
        pub ball: Ball,
        pub win_size: [f64; 2]
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let ball = &self.ball;
        let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new())
            .unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            text(PURPLE, 14, &self.fps.to_string(), &mut glyph_cache, c.transform.trans(10.0, 20.0), gl).unwrap();
            clear(WHITE, gl);

            let transform = c.transform;
            let x = circle(ball.x, ball.y, ball.radius);
            circle_arc(ball.colour, 10.0, 0.0, 6.28, x, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.fps = (1.0 / args.dt).trunc() as i32;

        let ball_radius = self.ball.radius + 10.0;
        let dx = self.ball.vel.i * args.dt;
        let dy = distance(self.ball.vel.j, self.gravity, args.dt);
        let [win_width, win_height] = self.win_size;

        const BOUNCE_DAMPING: f64 = 0.9;

        if self.ball.x + ball_radius + dx > win_width {
            self.ball.vel.i = -self.ball.vel.i * BOUNCE_DAMPING;
            self.ball.x = win_width - ball_radius;
        } else if self.ball.x - ball_radius < 0.0 {
            self.ball.vel.i = -self.ball.vel.i * BOUNCE_DAMPING;
            self.ball.x = ball_radius;
        } else {
            self.ball.x += dx;
        }

        if self.ball.y + ball_radius + dy > win_height {
            self.ball.vel.j = -self.ball.vel.j * BOUNCE_DAMPING;
            self.ball.y = win_height - ball_radius;
        } else if self.ball.y - ball_radius < 0.0 {
            self.ball.vel.j = -self.ball.vel.j * BOUNCE_DAMPING;
            self.ball.y = ball_radius;
        } else {
            self.ball.y += dy;
        }
        self.ball.vel.j += self.gravity * args.dt;
    }

    pub fn mouse_push(&mut self) {
        const ATTRACT_FORCE_SCALE: f64 = 0.2;
        let force = Vector {
            i: ATTRACT_FORCE_SCALE * (self.last_mouse_pos.i - self.ball.x),
            j: ATTRACT_FORCE_SCALE * (self.last_mouse_pos.j - self.ball.y),
        };

        self.ball.vel.i += force.i;
        self.ball.vel.j += force.j;
    }
}


fn distance(v: f64, a: f64, t: f64) -> f64 {
    (v * t) - (0.5 * a * t * t)
}