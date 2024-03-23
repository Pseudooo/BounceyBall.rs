use crate::vector::Vector;

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub vel: Vector,
    pub colour: [f32; 4],
}

impl Ball {
    pub fn tick(&mut self, force: Vector, dt: f64, win_size: [f64; 2]) {
        const BOUNCE_DAMPING: f64 = 0.9;

        let radius_with_edge = self.radius + 10.0;
        let dx = Self::distance(self.vel.i, force.i, dt);
        let dy = Self::distance(self.vel.j, force.j, dt);
        let [win_width, win_height] = win_size;

        if self.x + radius_with_edge + dx > win_width {
            self.vel.i = -self.vel.i * BOUNCE_DAMPING;
            self.x = win_width - radius_with_edge;
        } else if self.x - radius_with_edge < 0.0 {
            self.vel.i = -self.vel.i * BOUNCE_DAMPING;
            self.x = radius_with_edge;
        } else {
            self.x += dx;
        }

        if self.y + radius_with_edge + dy > win_height {
            self.vel.j = -self.vel.j * BOUNCE_DAMPING;
            self.y = win_height - radius_with_edge;
        } else if self.y - radius_with_edge < 0.0 {
            self.vel.j = -self.vel.j * BOUNCE_DAMPING;
            self.y = radius_with_edge;
        } else {
            self.y += dy;
        }
        self.vel.j += force.j * dt;
    }

    fn distance(v: f64, a: f64, t: f64) -> f64 {
        (v * t) - (0.5 * a * t * t)
    }
}

