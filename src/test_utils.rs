use expect_test::Expect;
use std::fmt::Debug;

pub(crate) fn check_debug(actual: impl Debug, expect: Expect) {
    expect.assert_debug_eq(&actual);
}

pub(crate) fn check(actual: &dyn ToString, expect: Expect) {
    expect.assert_eq(&actual.to_string());
}
