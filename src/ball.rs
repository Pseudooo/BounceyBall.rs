use crate::vector::Vector;

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub vel: Vector,
    pub colour: [f32; 4],
}

