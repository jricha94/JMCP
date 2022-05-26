


pub struct Cell {
    material: Material,
    surfs: Vec<Box<dyn Surface>>,
    adj: Vec<Box<Cell>>,
}

pub struct Geometry {
    cells: Vec<Box<Cell>>,
}
