


pub mod neutron;
pub mod surfaces;
// pub mod geometry;
pub mod material;



pub struct Pos {
    x: f64,
    y: f64,
    z: f64,
}


impl Pos {
    pub fn new(x: f64, y:f64 , z: f64) -> Self {
    Self {x, y, z}
    }
}


