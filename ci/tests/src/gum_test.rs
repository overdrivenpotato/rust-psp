//! Regression tests for the sceGum matrix stack: a pop restores what its matching push saved.
//!
//! Only the translation column is compared, since it is enough to tell the cases apart and reads
//! better in a failure message than sixteen floats.

use psp::sys::{self, MatrixMode, ScePspFMatrix4, ScePspFVector3, ScePspFVector4};
use psp::test_runner::TestRunner;

fn zero_matrix() -> ScePspFMatrix4 {
    let zero = ScePspFVector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    ScePspFMatrix4 {
        x: zero,
        y: zero,
        z: zero,
        w: zero,
    }
}

fn translation() -> (f32, f32, f32) {
    let mut m = zero_matrix();
    unsafe { sys::sceGumStoreMatrix(&mut m) };
    (m.w.x, m.w.y, m.w.z)
}

fn translate(x: f32, y: f32, z: f32) {
    unsafe { sys::sceGumTranslate(&ScePspFVector3 { x, y, z }) };
}

/// The leading `sceGumLoadIdentity` is not redundant: it and `sceGumLoadMatrix` are the only entry
/// points that create the VFPU context, and `sceGumMatrixMode` on a cold one traps. See #189.
fn reset() {
    unsafe {
        sys::sceGumLoadIdentity();
        sys::sceGumMatrixMode(MatrixMode::Model);
        sys::sceGumLoadIdentity();
    }
}

pub fn test_main(test_runner: &mut TestRunner) {
    test_runner.check_list(&[
        (
            "gum_push_pop_restores_translation",
            push_pop_restores(),
            (1.0, 2.0, 3.0),
        ),
        (
            "gum_push_pop_nested_restores_translation",
            nested_push_pop_restores(),
            (1.0, 2.0, 3.0),
        ),
        (
            "gum_push_pop_survives_matrix_write",
            push_pop_survives_matrix_write(),
            (1.0, 2.0, 3.0),
        ),
        (
            "gum_push_leaves_current_matrix_alone",
            push_does_not_disturb_current(),
            (1.0, 2.0, 3.0),
        ),
    ]);
}

fn push_pop_restores() -> (f32, f32, f32) {
    reset();
    translate(1.0, 2.0, 3.0);
    unsafe { sys::sceGumPushMatrix() };
    translate(10.0, 20.0, 30.0);
    unsafe { sys::sceGumPopMatrix() };
    translation()
}

/// Two deep, so a stack that is off by one in either direction is caught.
fn nested_push_pop_restores() -> (f32, f32, f32) {
    reset();
    translate(1.0, 2.0, 3.0);
    unsafe { sys::sceGumPushMatrix() };
    translate(10.0, 20.0, 30.0);
    unsafe { sys::sceGumPushMatrix() };
    translate(100.0, 200.0, 300.0);
    unsafe { sys::sceGumPopMatrix() };
    unsafe { sys::sceGumPopMatrix() };
    translation()
}

/// A sync to `*CURRENT_MATRIX` between push and pop must not eat the saved copy. This is what a
/// draw does, and it rules out saving into the slot above the stack pointer. A mode switch stands
/// in for `sceGumUpdateMatrix`, which stores through the same instructions but ends in
/// `sceGuSetMatrix`, and this suite never brings the GU up.
fn push_pop_survives_matrix_write() -> (f32, f32, f32) {
    reset();
    translate(1.0, 2.0, 3.0);
    unsafe { sys::sceGumPushMatrix() };
    translate(10.0, 20.0, 30.0);
    unsafe {
        sys::sceGumMatrixMode(MatrixMode::View);
        sys::sceGumMatrixMode(MatrixMode::Model);
        sys::sceGumPopMatrix();
    }
    translation()
}

/// Pushing saves the matrix without disturbing it.
fn push_does_not_disturb_current() -> (f32, f32, f32) {
    reset();
    translate(1.0, 2.0, 3.0);
    unsafe { sys::sceGumPushMatrix() };
    let after_push = translation();
    unsafe { sys::sceGumPopMatrix() };
    after_push
}
