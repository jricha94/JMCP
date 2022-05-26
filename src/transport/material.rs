use std::fs::File; 
use std::io::{self, BufRead, BufReader}; 
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Isotope {
    szaid: String,
    mat: u32,
    izaw: [(u32,f64); 16],
    nxs: [u32; 16],
    jxs: [u32; 32],
    pub xxs: Vec<f64>,
}

impl Isotope {
    pub fn new(path_to_ace: String) -> Result<(Self), Box<dyn Error>> {
        let f = File::open(path_to_ace)?;
        let f = BufReader::new(f);

        let mut szaid: Option<String> = None;
        let mut mat: Option<u32> = None;
        let mut izaw: [(u32,f64); 16] = [(0u32, 0f64);16];
        let mut nxs: [u32; 16] = [0; 16];
        let mut jxs: [u32; 32] = [0; 32];
        let mut xxs: Vec<f64> = Vec::new();

        for (i,line) in f.lines().enumerate() {

            match i {
                0 => {
                    let l = line.unwrap();
                    let l: Vec<&str> = l.split_whitespace().collect();
                    szaid = Some(String::from(l[1]));
                },

                3 => {
                    let l = line.unwrap();
                    let mut l: Vec<&str> = l.split_whitespace().collect();
                    mat = Some(l.pop().unwrap().replace("mat", "").parse::<u32>().unwrap());
                },

                4|5|6|7 => {
                    let l = line.unwrap();
                    let l: Vec<&str> = l.split_whitespace().collect();
                    for j in 0..4 {
                        izaw[j + (i-4) * 4] = (l[j*2].parse::<u32>().unwrap(), l[j*2+1].parse::<f64>().unwrap())
                    }
                },

                8|9 => {
                    let l = line.unwrap();
                    let l: Vec<&str> = l.split_whitespace().collect();
                    for j in 0..8 {
                        nxs[j + (i-8) * 8] = l[j].parse::<u32>().unwrap();
                    }

                },

                10|11|12|13 => {
                    let l = line.unwrap();
                    let l: Vec<&str> = l.split_whitespace().collect();
                    for j in 0..8 {
                        jxs[j + (i-10) * 8] = l[j].parse::<u32>().unwrap();
                    }
                }

                _ if i > 13 => {
                    let l = line.unwrap();
                    //let l: Vec<&str> = l.split_whitespace().collect();
                    for val in l.split_whitespace() {
                        xxs.push(val.parse::<f64>().unwrap())
                    }
                }

                _ => continue,

            }

        }

        let szaid = szaid.unwrap();
        let mat = mat.unwrap();
        println!("szaid: {}, mat #: {}", szaid, mat);
        println!("izaw: {:?}", izaw);
        println!("nxs: {:?}", nxs);
        println!("jxs: {:?}", jxs);

        Ok(Self {szaid, mat, izaw, nxs, jxs, xxs})
        
    }
}

impl fmt::Display for Isotope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Isotope {}, MAT Number {}", &self.szaid, &self.mat)
    }
}

impl Error for Isotope {}

pub struct Material {
    pub name: String,
    pub iso: Vec<Isotope>
}
