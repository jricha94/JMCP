use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Isotope {
    szaid: String,
    mat: u32,
    izaw: [(u32, f64); 16],
    nxs: [u32; 16],
    jxs: [u32; 32],
    xxs: Vec<f64>,
    pub mt_numbers: Vec<u32>,
}

impl fmt::Display for Isotope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Isotope {}, MAT Number {}", &self.szaid, &self.mat)
    }
}

impl Error for Isotope {}

impl Isotope {
    pub fn new(path_to_ace: String) -> Result<Self, Box<dyn Error>> {
        let f = File::open(path_to_ace)?;
        let f = BufReader::new(f);

        let mut szaid: Option<String> = None;
        let mut mat: Option<u32> = None;
        let mut izaw: [(u32, f64); 16] = [(0u32, 0f64); 16];
        let mut nxs: [u32; 16] = [0; 16];
        let mut jxs: [u32; 32] = [0; 32];
        let mut xxs: Vec<f64> = Vec::new();
        let mut mt_numbers: Vec<u32> = Vec::new();

        for (i, line) in f.lines().enumerate() {
            match i {
                0 => {
                    let l = line.unwrap();
                    let l: Vec<&str> = l.split_whitespace().collect();
                    szaid = Some(String::from(l[1]));
                }

                3 => {
                    let l = line.unwrap();
                    let mut l: Vec<&str> = l.split_whitespace().collect();
                    mat = Some(l.pop().unwrap().replace("mat", "").parse::<u32>().unwrap());
                }

                4 | 5 | 6 | 7 => {
                    let l = line.unwrap();
                    let l: Vec<&str> = l.split_whitespace().collect();
                    for j in 0..4 {
                        izaw[j + (i - 4) * 4] = (
                            l[j * 2].parse::<u32>().unwrap(),
                            l[j * 2 + 1].parse::<f64>().unwrap(),
                        )
                    }
                }

                8 | 9 => {
                    let l = line.unwrap();
                    let l: Vec<&str> = l.split_whitespace().collect();
                    for j in 0..8 {
                        nxs[j + (i - 8) * 8] = l[j].parse::<u32>().unwrap();
                    }
                }

                10 | 11 | 12 | 13 => {
                    let l = line.unwrap();
                    let l: Vec<&str> = l.split_whitespace().collect();
                    for j in 0..8 {
                        jxs[j + (i - 10) * 8] = l[j].parse::<u32>().unwrap();
                    }
                }

                _ if i > 13 => {
                    let l = line.unwrap();
                    for val in l.split_whitespace() {
                        xxs.push(val.parse::<f64>().unwrap())
                    }
                }

                _ => continue,
            }
        }

        let szaid = szaid.unwrap();
        let mat = mat.unwrap();

        let mt_index: usize = (jxs[2] - 1) as usize;
        let n_mt: usize = (nxs[3] - 1) as usize;
        for i in (mt_index)..(mt_index + n_mt) {
            mt_numbers.push(xxs[i] as u32);
        }

        Ok(Self {
            szaid,
            mat,
            izaw,
            nxs,
            jxs,
            xxs,
            mt_numbers,
        })
    }

    pub fn total_xs(&self, energy: f64) -> Option<f64> {
        let xxs_index = (self.jxs[0] - 1) as usize;
        let ene_pts = self.nxs[2] as usize;

        for i in xxs_index..(xxs_index + ene_pts) {
            if self.xxs[i] == energy {
                return Some(self.xxs[xxs_index + ene_pts + i]);
            } else if self.xxs[i] < energy && self.xxs[i + 1] > energy {
                return Some(self.xxs[xxs_index + ene_pts + i]);
            }
        }

        None
    }

    pub fn dis_xs(&self, energy: f64) -> Option<f64> {
        let xxs_index = (self.jxs[0] - 1) as usize;
        let ene_pts = self.nxs[2] as usize;

        for i in xxs_index..(xxs_index + ene_pts) {
            if self.xxs[i] == energy {
                return Some(self.xxs[xxs_index + ene_pts * 2 + i]);
            } else if self.xxs[i] < energy && self.xxs[i + 1] > energy {
                return Some(self.xxs[xxs_index + ene_pts * 2 + i]);
            }
        }

        None
    }

    pub fn elastic_xs(&self, energy: f64) -> Option<f64> {
        let xxs_index = (self.jxs[0] - 1) as usize;
        let ene_pts = self.nxs[2] as usize;

        for i in xxs_index..(xxs_index + ene_pts) {
            if self.xxs[i] == energy {
                return Some(self.xxs[xxs_index + ene_pts * 3 + i]);
            } else if self.xxs[i] < energy && self.xxs[i + 1] > energy {
                return Some(self.xxs[xxs_index + ene_pts * 3 + i]);
            }
        }

        None
    }

    pub fn avg_heating(&self, energy: f64) -> Option<f64> {
        let xxs_index = (self.jxs[0] - 1) as usize;
        let ene_pts = self.nxs[2] as usize;

        for i in xxs_index..(xxs_index + ene_pts) {
            if self.xxs[i] == energy {
                return Some(self.xxs[xxs_index + ene_pts * 4 + i]);
            } else if self.xxs[i] < energy && self.xxs[i + 1] > energy {
                return Some(self.xxs[xxs_index + ene_pts * 4 + i]);
            }
        }

        None
    }

    pub fn nu(&self, energy: f64) -> Option<f64> {
        if self.jxs[1] == 0 {
            // No nu values
            return None;
        } else if self.jxs[1] > 0 {
            // Prompt or total nu given (but not both)
            let nu_index = self.jxs[1] as usize;
            match self.xxs[nu_index] as u8 {
                1 => {
                    return None;
                }
                2 => {
                    let regions = self.xxs[nu_index + 1] as usize;
                    if regions > 0 {
                        let mut interp_params: Vec<f64> = Vec::new();
                        let mut interp_scheme: Vec<f64> = Vec::new();
                        for i in (nu_index + 2)..(nu_index + 2 + regions) {
                            interp_params.push(self.xxs[i]);
                        }
                        for i in (nu_index + 2 + regions)..(nu_index + 2 + 2 * regions) {
                            interp_scheme.push(self.xxs[i]);
                        }
                    }

                    let ene_pts = self.xxs[nu_index + 2 + (2 * regions)] as usize;

                    for i in
                        (nu_index + 3 + (2 * regions))..(nu_index + 3 + (2 * regions) + ene_pts)
                    {
                        if self.xxs[i] == energy {
                            return Some(self.xxs[ene_pts + i]);
                        } else if self.xxs[i] < energy && self.xxs[i + 1] > energy {
                            return Some(self.xxs[ene_pts + i]);
                        }
                    }
                }
                _ => {
                    return None;
                }
            }

            return None;
        } else {
            return None;
        }
    }

    // pub fn delayed_nu(&self, energy: f64) -> Option<f64> {

    pub fn q_value(&self, mt: u32) -> Option<f64> {
        let q_index: usize = (self.jxs[3] - 1) as usize;

        for (i, num) in self.mt_numbers.iter().enumerate() {
            if *num == mt {
                return Some(self.xxs[q_index + i]);
            }
        }

        None
    }

    pub fn n_release(&self, mt: u32) -> Option<i32> {
        let q_index: usize = (self.jxs[4] - 1) as usize;

        for (i, num) in self.mt_numbers.iter().enumerate() {
            if *num == mt {
                return Some(self.xxs[q_index + i] as i32);
            }
        }

        None
    }

    pub fn mt_xs(&self, mt: u32, e: f64) -> Option<f64> {
        let lsig_index: usize = (self.jxs[5] - 1) as usize;

        for (i, num) in self.mt_numbers.iter().enumerate() {
            if *num == mt {
                let location: usize = self.xxs[lsig_index + i] as usize;
                println!("{}", location);
            }
        }

        Some(7.0)
    }
}

pub struct Material {
    pub name: String,
    pub iso: Vec<Isotope>,
}
