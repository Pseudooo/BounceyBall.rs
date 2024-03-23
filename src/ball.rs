use crate::vector::Vector;

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub vel: Vector,
    pub colour: [f32; 4],
}

impl Ball {
    pub fn tick(&self, force: Vector, dt: f64, win_size: [f64; 2]) -> [Vector; 2] {
        const BOUNCE_DAMPING: f64 = 0.9;

        let radius_with_edge = self.radius + 10.0;
        let dx = Self::distance(self.vel.i, force.i, dt);
        let dy = Self::distance(self.vel.j, force.j, dt);
        let [win_width, win_height] = win_size;

        let mut final_pos = Vector::new(self.x, self.y);
        let mut final_vel = self.vel.clone();

        if self.x + radius_with_edge + dx > win_width {
            final_vel.i = -final_vel.i * BOUNCE_DAMPING;
            final_pos.i = win_width - radius_with_edge;
        } else if self.x - radius_with_edge < 0.0 {
            final_vel.i = -final_vel.i * BOUNCE_DAMPING;
            final_pos.i = radius_with_edge;
        } else {
            final_pos.i += dx;
        }
        final_vel.i += force.i * dt;

        if final_pos.j + radius_with_edge + dy > win_height {
            final_vel.j = -final_vel.j * BOUNCE_DAMPING;
            final_pos.j = win_height - radius_with_edge;
        } else if final_pos.j - radius_with_edge < 0.0 {
            final_vel.j = -final_vel.j * BOUNCE_DAMPING;
            final_pos.j = radius_with_edge;
        } else {
            final_pos.j += dy;
        }
        final_vel.j += force.j * dt;

        return [final_pos, final_vel];
    }

    fn distance(v: f64, a: f64, t: f64) -> f64 {
        (v * t) - (0.5 * a * t * t)
    }
}

