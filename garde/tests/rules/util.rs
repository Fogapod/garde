#![allow(dead_code)]

use std::fmt::{Debug, Write};

use garde::Validate;
use owo_colors::OwoColorize;

pub fn check_ok<T: Validate + Debug>(cases: &[T], ctx: &T::Context) {
    let mut some_failed = false;
    for case in cases {
        if let Err(report) = case.validate_with(ctx) {
            eprintln!(
                "{} input: {case:?}, errors: [{}]",
                "FAIL".red(),
                report
                    .to_string()
                    .split('\n')
                    .collect::<Vec<_>>()
                    .join("; ")
            );
            some_failed = true;
        }
    }

    if some_failed {
        panic!("some cases failed, see error output");
    }
}

#[doc(hidden)]
pub fn __check_fail<T: Validate + Debug>(cases: &[T], ctx: &T::Context) -> String {
    let mut some_success = false;
    let mut snapshot = String::new();
    for case in cases {
        if let Err(report) = case.validate_with(ctx) {
            writeln!(&mut snapshot, "{case:#?}").unwrap();
            write!(&mut snapshot, "{report}").unwrap();
            writeln!(&mut snapshot).unwrap();
        } else {
            eprintln!("{} input: {case:?}", "SUCCESS".red());
            some_success = true;
        }
    }

    if some_success {
        panic!("some cases did not fail, see error output");
    }

    snapshot
}

#[doc(hidden)]
#[macro_export]
macro_rules! __check_fail {
    ($input:expr, $ctx:expr $(,)?) => {{
        let snapshot = $crate::rules::util::__check_fail($input, $ctx);
        ::insta::assert_snapshot!(snapshot);
    }};
}

pub use crate::__check_fail as check_fail;
