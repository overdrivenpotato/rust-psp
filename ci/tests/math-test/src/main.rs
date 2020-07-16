#![no_std]
#![no_main]

use psp::math;
use psp::test_runner::TestRunner;

psp::module!("math_test", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    let mut test_runner = TestRunner::new_file_runner();
    test_runner.start();
    test_runner.check_value_equality(&[
        ("cos_2.5", test_cos(2.5), -0.8011436),
        ("cos_0", test_cos(0.0), 1.0),
        ("cos_pi", test_cos(psp::sys::GU_PI), -1.0),
    ]);
    test_runner.finish();
}

fn test_cos(num: f32) -> f32 {
    unsafe { math::cosf32(num) }
}
