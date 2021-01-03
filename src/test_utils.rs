use expect_test::Expect;
use std::fmt::Debug;

pub(crate) fn check_debug(actual: impl Debug, expect: Expect) {
    expect.assert_debug_eq(&actual);
}

pub(crate) fn check(actual: &dyn ToString, expect: Expect) {
    expect.assert_eq(&actual.to_string());
}

macro_rules! check_alg {
    ($alg: ident, $expected: expr) => {{
        use rand::SeedableRng;

        let mut grid = $crate::grid::Grid::new(10, 10);
        let mut rng = rand::prelude::StdRng::seed_from_u64(1);

        $alg(&mut grid, &mut rng);
        $crate::test_utils::check(&grid, $expected);
    }};
}
