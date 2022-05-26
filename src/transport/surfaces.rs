
use crate::transport::Pos;
use crate::transport::neutron::Neutron;

pub trait Surface {
    fn is_inside(&self, p: Pos) -> bool;
    fn dist_to_surf(&self, n: Neutron) -> Option<f64>;
}


pub struct Plane {
    name: String,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}


impl Plane {
     pub fn new(name: String, a: f64, b:f64, c: f64, d:f64 ) -> Self {
        Self {name, a, b, c, d }
    }
}    


impl Surface for Plane {
    fn is_inside(&self, p: Pos) -> bool {
        let v = self.a * p.x + self.b * p.y + self.c * p.z + self.d;
        if v <= 0f64 {true} else {false}
    }

    fn dist_to_surf(&self, n: Neutron) -> Option<f64> {
        let bot = self.a * n.angle.mu * n.angle.theta.sin()
                + self.b * (1f64 - (n.angle.mu * n.angle.mu)).sqrt() * n.angle.theta.sin()
                + self.c * n.angle.theta.cos();

        let top = -1f64 * (self.a * n.pos.x + self.b * n.pos.y + self.c * n.pos.z + self.d);
        let res = top/bot;
        if res < 0f64 || res == f64::INFINITY {
            None
        } else {
            Some(res)
        }
    }
}


// pub struct Circle {
//     pos: Pos,
//     rad: f64,
// }

// impl Surface for Circle {
//     fn is_inside(p: Pos) -> bool {

//     }

// }

// pub struct Cylinder {
//     circle: Circle,
//     top: Plane,
//     bottom: Plane,
// }

// pub struct Rectangle {

// }

// pub struct RectangularPrism {

// }
