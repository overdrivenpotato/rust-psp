use psp::math;
use psp::test_runner::TestRunner;

pub fn test_main(test_runner: &mut TestRunner) {
    test_runner.check_list(&[
        ("cos_2.5", test_cos(2.5), -0.8011436),
        ("cos_0", test_cos(0.0), 1.0),
        ("cos_pi", test_cos(psp::sys::GU_PI), -1.0),
    ]);
}

fn test_cos(num: f32) -> f32 {
    unsafe { math::cosf32(num) }
}
