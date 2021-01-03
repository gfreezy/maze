use rand::prelude::SliceRandom;
use std::collections::HashSet;
use std::fmt;

type Position = (isize, isize);

#[derive(PartialEq, Debug)]
struct Cell {
    pub pos: Position,
    pub north: Option<Position>,
    pub south: Option<Position>,
    pub west: Option<Position>,
    pub east: Option<Position>,
    links: HashSet<Position>,
}

impl Cell {
    fn new(pos: Position) -> Self {
        Cell {
            pos,
            north: None,
            south: None,
            west: None,
            east: None,
            links: HashSet::new(),
        }
    }

    fn link(&mut self, position: Position) {
        self.links.insert(position);
    }

    fn unlink(&mut self, pos: &Position) {
        self.links.remove(pos);
    }

    fn links(&self) -> impl Iterator<Item = &Position> {
        self.links.iter()
    }

    fn linked(&self, pos: &Position) -> bool {
        self.links.contains(pos)
    }

    pub fn linked_optional(&self, pos: Option<&Position>) -> bool {
        if let Some(pos) = pos {
            self.links.contains(pos)
        } else {
            false
        }
    }

    fn neighbors(&self) -> Vec<Position> {
        let mut neighbors = vec![];
        if let Some(n) = self.north {
            neighbors.push(n);
        }
        if let Some(n) = self.south {
            neighbors.push(n);
        }
        if let Some(n) = self.east {
            neighbors.push(n);
        }
        if let Some(n) = self.west {
            neighbors.push(n);
        }
        neighbors
    }

    pub fn clear(&mut self) {
        self.links.clear();
    }
}

#[derive(PartialEq, Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    columns: usize,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        let cells = (0..rows)
            .into_iter()
            .map(|y| {
                (0..columns)
                    .into_iter()
                    .map(|x| Cell::new((x as isize, y as isize)))
                    .collect()
            })
            .collect();
        let mut grid = Grid {
            cells,
            rows,
            columns,
        };
        grid.configure_cells();
        grid
    }

    fn get_cell_mut(&mut self, pos: &Position) -> Option<&mut Cell> {
        let (x, y) = *pos;
        let row = self.cells.get_mut(y as usize)?;
        row.get_mut(x as usize)
    }

    fn get_cell(&self, pos: &Position) -> Option<&Cell> {
        let (x, y) = *pos;
        let row = self.cells.get(y as usize)?;
        row.get(x as usize)
    }

    pub fn regenerate(&mut self) {
        for cell in self.each_cell_mut() {
            cell.clear();
        }
    }

    fn configure_cells(&mut self) {
        let rows = self.rows as isize;
        let columns = self.columns as isize;
        for cell in self.each_cell_mut() {
            let (x, y) = cell.pos;
            cell.north = if y > 0 {
                Some((x as isize, (y - 1) as isize))
            } else {
                None
            };
            cell.south = if y < rows - 1 {
                Some((x as isize, (y + 1) as isize))
            } else {
                None
            };
            cell.east = if x < columns - 1 {
                Some(((x + 1) as isize, y as isize))
            } else {
                None
            };
            cell.west = if x > 0 {
                Some(((x - 1) as isize, y as isize))
            } else {
                None
            };
        }
    }

    fn random_cell(&mut self) -> &mut Cell {
        let mut rng = rand::thread_rng();
        let row = self.cells.choose_mut(&mut rng).expect("empty cells");
        row.choose_mut(&mut rng).expect("empty column")
    }

    fn each_row(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.cells.iter()
    }

    fn each_cell(&self) -> impl Iterator<Item = &Cell> {
        self.each_row().flat_map(|row| row.iter())
    }

    fn each_cell_do(&self, f: impl FnMut(&Cell)) {
        self.each_row().flat_map(|row| row.iter()).for_each(f)
    }

    fn each_cell_mut_do(&mut self, f: impl FnMut(&mut Cell)) {
        self.each_row_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(f)
    }

    fn each_row_mut(&mut self) -> impl Iterator<Item = &mut Vec<Cell>> {
        self.cells.iter_mut()
    }

    fn each_cell_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.each_row_mut().flat_map(|row| row.iter_mut())
    }

    pub fn iter_position(&self) -> impl Iterator<Item = Position> + '_ {
        self.each_cell().map(|cell| cell.pos.clone())
    }

    pub fn link_cell(&mut self, from: Position, to: Position, bidi: bool) {
        let cell = self.get_cell_mut(&from).unwrap();
        cell.link(to);
        if bidi {
            let cell = self.get_cell_mut(&to).unwrap();
            cell.link(from);
        }
    }

    pub fn unlink_cell(&mut self, from: Position, to: Position, bidi: bool) {
        let cell = self.get_cell_mut(&from).unwrap();
        cell.unlink(&to);
        if bidi {
            let cell = self.get_cell_mut(&to).unwrap();
            cell.unlink(&from);
        }
    }

    pub fn north_of_cell(&self, pos: Position) -> Option<Position> {
        self.get_cell(&pos)?.north
    }

    pub fn south_of_cell(&self, pos: Position) -> Option<Position> {
        self.get_cell(&pos)?.south
    }

    pub fn east_of_cell(&self, pos: Position) -> Option<Position> {
        self.get_cell(&pos)?.east
    }

    pub fn west_of_cell(&self, pos: Position) -> Option<Position> {
        self.get_cell(&pos)?.west
    }

    pub fn iter(&self) -> GridIter {
        GridIter::new(self.rows, self.columns)
    }

    pub fn sprite_for_cell(&self, pos: Position) -> Option<u8> {
        let cell = self.get_cell(&pos)?;
        let mut sprite: u8 = 0;
        if cell.west.is_none() {
            sprite |= 0b0001;
        }
        if cell.east.is_none() {
            sprite |= 0b0100;
        }
        if cell.north.is_none() {
            sprite |= 0b1000;
        }
        if cell.south.is_none() {
            sprite |= 0b0010;
        }

        if !cell.linked_optional(cell.east.as_ref()) {
            sprite |= 0b0100;
        }
        if !cell.linked_optional(cell.south.as_ref()) {
            sprite |= 0b0010;
        }
        Some(sprite)
    }
}

pub struct GridIter {
    rows: isize,
    columns: isize,
    y: isize,
    x: isize,
}

impl GridIter {
    pub fn new(rows: usize, columns: usize) -> Self {
        GridIter {
            rows: rows as isize,
            columns: columns as isize,
            y: 0,
            x: 0,
        }
    }
}
impl Iterator for GridIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.rows {
            return None;
        }
        let pos = (self.x, self.y);
        self.x += 1;
        if self.x >= self.columns {
            self.x = 0;
            self.y += 1;
        }
        Some(pos)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "+")?;
        for _ in 0..self.columns {
            write!(f, "---+")?;
        }
        write!(f, "\n")?;

        for row in self.each_row() {
            let mut top = "|".to_string();
            let mut bottom = "+".to_string();

            for cell in row.iter() {
                let east_boundary = if cell.linked_optional(cell.east.as_ref()) {
                    " "
                } else {
                    "|"
                };
                top.push_str("   ");
                top.push_str(east_boundary);

                let south_boundary = if cell.linked_optional(cell.south.as_ref()) {
                    "   "
                } else {
                    "---"
                };
                bottom.push_str(south_boundary);
                bottom.push_str("+");
            }

            writeln!(f, "{}", top)?;
            writeln!(f, "{}", bottom)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::{Grid, GridIter};
    use crate::test_utils::{check, check_debug};
    use expect_test::expect;

    #[test]
    fn test_grid_iter() {
        let iter = GridIter::new(10, 8);
        let ret = iter.into_iter().collect::<Vec<_>>();
        check_debug(
            ret,
            expect![[r#"
                [
                    (
                        0,
                        0,
                    ),
                    (
                        1,
                        0,
                    ),
                    (
                        2,
                        0,
                    ),
                    (
                        3,
                        0,
                    ),
                    (
                        4,
                        0,
                    ),
                    (
                        5,
                        0,
                    ),
                    (
                        6,
                        0,
                    ),
                    (
                        7,
                        0,
                    ),
                    (
                        0,
                        1,
                    ),
                    (
                        1,
                        1,
                    ),
                    (
                        2,
                        1,
                    ),
                    (
                        3,
                        1,
                    ),
                    (
                        4,
                        1,
                    ),
                    (
                        5,
                        1,
                    ),
                    (
                        6,
                        1,
                    ),
                    (
                        7,
                        1,
                    ),
                    (
                        0,
                        2,
                    ),
                    (
                        1,
                        2,
                    ),
                    (
                        2,
                        2,
                    ),
                    (
                        3,
                        2,
                    ),
                    (
                        4,
                        2,
                    ),
                    (
                        5,
                        2,
                    ),
                    (
                        6,
                        2,
                    ),
                    (
                        7,
                        2,
                    ),
                    (
                        0,
                        3,
                    ),
                    (
                        1,
                        3,
                    ),
                    (
                        2,
                        3,
                    ),
                    (
                        3,
                        3,
                    ),
                    (
                        4,
                        3,
                    ),
                    (
                        5,
                        3,
                    ),
                    (
                        6,
                        3,
                    ),
                    (
                        7,
                        3,
                    ),
                    (
                        0,
                        4,
                    ),
                    (
                        1,
                        4,
                    ),
                    (
                        2,
                        4,
                    ),
                    (
                        3,
                        4,
                    ),
                    (
                        4,
                        4,
                    ),
                    (
                        5,
                        4,
                    ),
                    (
                        6,
                        4,
                    ),
                    (
                        7,
                        4,
                    ),
                    (
                        0,
                        5,
                    ),
                    (
                        1,
                        5,
                    ),
                    (
                        2,
                        5,
                    ),
                    (
                        3,
                        5,
                    ),
                    (
                        4,
                        5,
                    ),
                    (
                        5,
                        5,
                    ),
                    (
                        6,
                        5,
                    ),
                    (
                        7,
                        5,
                    ),
                    (
                        0,
                        6,
                    ),
                    (
                        1,
                        6,
                    ),
                    (
                        2,
                        6,
                    ),
                    (
                        3,
                        6,
                    ),
                    (
                        4,
                        6,
                    ),
                    (
                        5,
                        6,
                    ),
                    (
                        6,
                        6,
                    ),
                    (
                        7,
                        6,
                    ),
                    (
                        0,
                        7,
                    ),
                    (
                        1,
                        7,
                    ),
                    (
                        2,
                        7,
                    ),
                    (
                        3,
                        7,
                    ),
                    (
                        4,
                        7,
                    ),
                    (
                        5,
                        7,
                    ),
                    (
                        6,
                        7,
                    ),
                    (
                        7,
                        7,
                    ),
                    (
                        0,
                        8,
                    ),
                    (
                        1,
                        8,
                    ),
                    (
                        2,
                        8,
                    ),
                    (
                        3,
                        8,
                    ),
                    (
                        4,
                        8,
                    ),
                    (
                        5,
                        8,
                    ),
                    (
                        6,
                        8,
                    ),
                    (
                        7,
                        8,
                    ),
                    (
                        0,
                        9,
                    ),
                    (
                        1,
                        9,
                    ),
                    (
                        2,
                        9,
                    ),
                    (
                        3,
                        9,
                    ),
                    (
                        4,
                        9,
                    ),
                    (
                        5,
                        9,
                    ),
                    (
                        6,
                        9,
                    ),
                    (
                        7,
                        9,
                    ),
                ]
            "#]],
        )
    }

    #[test]
    fn test_display_grid() {
        let mut grid = Grid::new(10, 10);
        grid.link_cell((1, 1), (1, 2), true);
        check(
            &grid,
            expect![[r#"
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+   +---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
                |   |   |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
            "#]],
        );
        grid.link_cell((9, 9), (8, 9), true);
        check(&grid, expect![[r#"
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+   +---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |   |   |
            +---+---+---+---+---+---+---+---+---+---+
            |   |   |   |   |   |   |   |   |       |
            +---+---+---+---+---+---+---+---+---+---+
        "#]]);
    }

    #[test]
    fn test_north_east_south_west() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.get_cell(&(0, 0)).unwrap().north, None);
        assert_eq!(grid.get_cell(&(1, 0)).unwrap().north, None);
        assert_eq!(grid.get_cell(&(1, 1)).unwrap().north, Some((1, 0)));
        assert_eq!(grid.get_cell(&(2, 3)).unwrap().north, Some((2, 2)));
        assert_eq!(grid.get_cell(&(2, 3)).unwrap().east, Some((3, 3)));
        assert_eq!(grid.get_cell(&(9, 3)).unwrap().east, None);
        assert_eq!(grid.get_cell(&(0, 3)).unwrap().west, None);
        assert_eq!(grid.get_cell(&(9, 9)).unwrap().east, None);
        assert_eq!(grid.get_cell(&(9, 9)).unwrap().north, Some((9, 8)));
        assert_eq!(grid.get_cell(&(9, 9)).unwrap().east, None);
        assert_eq!(grid.get_cell(&(9, 9)).unwrap().west, Some((8, 9)));
    }

    #[test]
    fn test_cell_sprite() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.sprite_for_cell((0, 0)), Some(0b1111));
        assert_eq!(grid.sprite_for_cell((0, 1)), Some(0b0111));
        assert_eq!(grid.sprite_for_cell((1, 1)), Some(0b0110));
    }
}
