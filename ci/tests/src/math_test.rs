use core::f32::{consts::PI, EPSILON, NAN};
use psp::math;
use psp::test_runner::TestRunner;

pub fn test_main(test_runner: &mut TestRunner) {
    test_runner.check_list(&[
        ("cos_0", test_cos(0.0), 1.0),
        ("cos_pi", test_cos(PI), -1.0),
        ("sin_0", test_sin(0.0), 0.0),
        ("sin_2.5", test_sin(2.5), 0.5984721),
        ("fmodf", test_fmodf(-10.0, 3.0), -1.0),
        ("fminf", test_fminf(-10.0, 3.0), -10.0),
        ("fminf_NAN", test_fminf(-10.0, NAN), -10.0),
        ("fmaxf", test_fmaxf(-10.0, 3.0), 3.0),
        ("fmaxf_NAN", test_fmaxf(NAN, 3.0), 3.0),
    ]);
    let cos_2_5 = test_cos(2.5) + 0.8011436;
    test_runner.check_true(
        "cos_2.5",
        cos_2_5 < (EPSILON * 2.0) && cos_2_5 > -(EPSILON * 2.0),
    );
    let almost_zero = test_sin(PI);
    test_runner.check_true("sin_pi", almost_zero < EPSILON && almost_zero > -EPSILON);
    let fmodf_0 = test_fmodf(1.0, 0.0);
    test_runner.check_true("fmodf_0", fmodf_0.is_nan());
}

fn test_cos(num: f32) -> f32 {
    unsafe { math::cosf(num) }
}

fn test_sin(num: f32) -> f32 {
    unsafe { math::sinf(num) }
}

fn test_fminf(x: f32, y: f32) -> f32 {
    unsafe { math::fminf(x, y) }
}

fn test_fmaxf(x: f32, y: f32) -> f32 {
    unsafe { math::fmaxf(x, y) }
}

fn test_fmodf(x: f32, y: f32) -> f32 {
    math::fmodf(x, y)
}
