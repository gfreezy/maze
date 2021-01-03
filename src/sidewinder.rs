use crate::grid::Grid;
use rand::prelude::SliceRandom;
use rand::Rng;

pub fn sidewinder<T: Rng>(grid: &mut Grid, rng: &mut T) {
    let mut run = vec![];
    for row in grid.iter_rows() {
        run.clear();
        for pos in row {
            run.push(pos);
            let at_eastern_boundary = grid.east_of_cell(pos).is_none();
            let at_northern_boundary = grid.north_of_cell(pos).is_none();

            let should_close_out =
                at_eastern_boundary || (!at_northern_boundary && rng.gen::<bool>());
            if should_close_out {
                if let Some(member) = run.choose(rng).cloned() {
                    grid.link_cell_to_north(member);
                }
                run.clear();
            } else {
                grid.link_cell_to_east(pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sidewinder;
    use expect_test::expect;

    #[test]
    fn test_sidewinder() {
        check_alg!(sidewinder, expect![[r#"
            +---+---+---+---+---+---+---+---+---+---+
            |                                       |
            +   +   +---+   +---+---+---+---+   +   +
            |   |       |           |           |   |
            +   +---+   +---+---+   +---+   +---+   +
            |   |               |   |       |       |
            +---+   +---+   +   +---+---+---+   +   +
            |       |       |   |               |   |
            +   +   +   +   +---+---+   +---+---+   +
            |   |   |   |       |       |           |
            +   +   +---+   +   +---+---+   +   +   +
            |   |       |   |           |   |   |   |
            +   +   +---+---+   +---+---+---+   +---+
            |   |   |               |               |
            +   +---+   +---+   +   +---+---+---+   +
            |       |       |   |   |               |
            +   +   +   +   +---+---+   +---+---+   +
            |   |   |   |   |                   |   |
            +---+   +---+   +   +---+---+---+---+   +
            |       |       |           |           |
            +---+---+---+---+---+---+---+---+---+---+
        "#]]);
    }
}
