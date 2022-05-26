// TODO: Remove pubs which aren't needed


use rand::Rng;

use crate::transport::Pos;

pub struct Angle {
    pub mu: f64,
    pub theta: f64,
}

impl Angle {
    fn new() -> Self {
        let mu: f64 = (2f64 * rand::thread_rng().gen::<f64>()) - 1f64;
        let theta: f64 = 2f64 * std::f64::consts::PI * rand::thread_rng().gen::<f64>();
        Self { mu, theta }
    }
}

pub struct Neutron {
    pub pos: Pos,
    pub angle: Angle,
    pub energy: f64,
    pub tau: f64,
    // cell : Cell, // Cell which the neutron is in
} 


impl Neutron {
    pub fn new(pos: Pos) -> Self {
        let angle = Angle::new();
        let energy:f64 = 1f64; // TODO: change this
        let tau = -1f64 * rand::thread_rng().gen::<f64>().ln();

        Self { pos, angle, energy, tau }
    }
}
