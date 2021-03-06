pub mod transport;
use crate::transport::neutron::*;
use crate::transport::Pos; //Neutron;
                           // use crate::transport::geometry::*;
use crate::transport::material::*;
use crate::transport::surfaces::*;

fn main() {
    // let mut mu:Vec<f64> = Vec::new();
    // let mut theta:Vec<f64> = Vec::new();
    // let mut tau:Vec<f64> = Vec::new();

    // for _ in 0..1000000 {
    //     let p = Pos::new(0f64, 0f64, 0f64);
    //     let n = Neutron::new(p);
    //     mu.push(n.angle.mu);
    //     theta.push(n.angle.theta);
    //     tau.push(n.tau)
    // }

    // println!("mu: {}, theta: {}, tau: {}",
    //     mu.iter().sum::<f64>() as f64 / mu.len() as f64,
    //     theta.iter().sum::<f64>() as f64 / theta.len() as f64,
    //     tau.iter().sum::<f64>() as f64 / tau.len() as f64);

    // let p = Plane::new(String::from("This"), 1f64, 0f64, 0f64, -5f64);
    // let pos = Pos::new(0f64, 0f64, 0f64);
    // let mut n = Neutron::new(pos);
    // n.angle.mu = 0f64;
    // n.angle.theta = std::f64::consts::PI / 2f64;

    // print!("{:?}\n", p.dist_to_surf(n));

    let path1 = String::from("/home/jarod/Projects/JMCP/xs/Lib80x/Lib80x/U/92235.800nc");
    let path2 = String::from("/home/jarod/Projects/JMCP/xs/Lib80x/Lib80x/H/1001.800nc");

    let h1 = Isotope::new(path2).unwrap();
    // println!("{}", h1.nu(1.0).unwrap_or_default());

    let u235 = Isotope::new(path1).unwrap();
    // for i in 1..15 {
    //     println!("nu: {}", u235.nu(i as f64).unwrap());
    // }

    // println!("{:?}", u235.q_value(5).unwrap());

    for mt in u235.mt_numbers.iter() {
        // println!("q: {}, n: {}", u235.q_value(*mt).unwrap(), u235.n_release(*mt).unwrap())
        println!("loc: {}", u235.mt_xs(*mt, 1.0).unwrap());
    }
}
