#![no_std]
#![no_main]

use psp::test_runner::TestRunner;

mod bmp_screenshot_test;
mod math_test;
mod vram_test;

psp::module!("ci_tests", 1, 1);

fn psp_main() {
    let tests = &[
        vram_test::test_main,
        math_test::test_main,
        bmp_screenshot_test::test_main,
    ];

    let mut runner = TestRunner::new_file_runner();
    runner.start_run();

    for test in tests {
        runner.run(test);
    }

    runner.finish_run();
}
