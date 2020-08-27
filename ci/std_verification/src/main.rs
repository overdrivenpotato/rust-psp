#![feature(restricted_std)]
#![no_main]

use psp::test_runner::TestRunner;

mod time_test;

psp::module!("std_verification", 1, 1);

fn psp_main() {
    let tests = &[
        time_test::test_main,
    ];

    let mut runner = TestRunner::new_dprintln_runner();
    runner.start_run();

    for test in tests {
        runner.run(test);
    }

    runner.finish_run();
}
