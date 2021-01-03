use crate::grid::Grid;
use rand::prelude::SliceRandom;
use rand::Rng;

pub fn binary_tree<T: Rng>(grid: &mut Grid, rng: &mut T) {
    let mut neighbors = vec![];
    for pos in grid.iter() {
        neighbors.clear();
        if let Some(north) = grid.north_of_cell(pos) {
            neighbors.push(north);
        }
        if let Some(east) = grid.east_of_cell(pos) {
            neighbors.push(east);
        }
        let neighbor = neighbors.choose(rng);
        if let Some(neighbor) = neighbor.cloned() {
            grid.link_cell(pos, neighbor, true);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::binary_tree;
    use expect_test::expect;

    #[test]
    fn test_binary_tree() {
        check_alg!(
            binary_tree,
            expect![[r#"
                +---+---+---+---+---+---+---+---+---+---+
                |                                       |
                +   +   +---+---+---+---+   +   +   +   +
                |   |   |                   |   |   |   |
                +---+---+---+   +---+---+---+   +---+   +
                |               |               |       |
                +---+---+---+---+   +---+   +---+---+   +
                |                   |       |           |
                +   +---+   +---+---+   +---+   +---+   +
                |   |       |           |       |       |
                +---+   +---+---+---+---+   +   +---+   +
                |       |                   |   |       |
                +   +---+---+   +---+   +---+   +---+   +
                |   |           |       |       |       |
                +---+---+---+   +   +   +   +---+---+   +
                |               |   |   |   |           |
                +---+   +   +   +---+---+---+---+   +   +
                |       |   |   |                   |   |
                +---+   +   +   +   +   +   +   +   +   +
                |       |   |   |   |   |   |   |   |   |
                +---+---+---+---+---+---+---+---+---+---+
            "#]]
        )
    }
}
