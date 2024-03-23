
#[derive(Clone)]
pub struct Vector {
    pub i: f64,
    pub j: f64,
}

impl Vector
{
    pub fn new(i: f64, j: f64) -> Vector {
        Vector {
            i,
            j,
        }
    }
}
