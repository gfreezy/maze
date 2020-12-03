use rand;

pub type Cell = u8;

pub const SIZE: usize = 4;

#[derive(Debug)]
pub struct Maze {
    cells: [[Cell; SIZE]; SIZE],
}

impl Maze {
    pub fn new() -> Self {
        Maze {
            cells: [[0b0000; SIZE]; SIZE],
        }
    }

    pub fn regenerate(&mut self) {
        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if (x, y) == (SIZE - 1, 0) {
                    *cell = 0b1100;
                } else if x == SIZE - 1 {
                    *cell = 0b0100;
                } else if y == 0 {
                    *cell = 0b1000;
                } else {
                    let to_carve_right: bool = rand::random();
                    if to_carve_right {
                        *cell = 0b1000;
                    } else {
                        *cell = 0b0100;
                    }
                }

                if x == 0 {
                    *cell |= 0b0001;
                }

                if y == SIZE - 1 {
                    *cell |= 0b0010;
                }
            }
        }
    }

    pub fn iter_row(&self) -> impl Iterator<Item = &[Cell; SIZE]> {
        self.cells.iter()
    }
}
