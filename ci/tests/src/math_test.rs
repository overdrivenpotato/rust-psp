use psp::math;
use psp::sys::GU_PI;
use psp::test_runner::TestRunner;

const EPSILON: f32 = 0.00001;

pub fn test_main(test_runner: &mut TestRunner) {
    test_runner.check_list(&[
        ("cos_2.5", test_cos(2.5), -0.8011436),
        ("cos_0", test_cos(0.0), 1.0),
        ("cos_pi", test_cos(GU_PI), -1.0),
        ("intrinsics_cos_pi", test_cos_intrinsic(GU_PI), -1.0),

        ("sin_2.5", test_sin(2.5), 0.5984721),
    ]);
    let almost_zero = test_sin_intrinsic(GU_PI);
    test_runner.check_true("intrinsics_sin_0", almost_zero < EPSILON && almost_zero > -EPSILON);
}

fn test_cos(num: f32) -> f32 {
    unsafe { math::cosf32(num) }
}

fn test_cos_intrinsic(num: f32) -> f32 {
    unsafe { core::intrinsics::cosf32(GU_PI) }
}

fn test_sin(num: f32) -> f32 {
    unsafe { math::sinf32(num) }
}

fn test_sin_intrinsic(num: f32) -> f32 {
    unsafe { core::intrinsics::sinf32(GU_PI) }
}
