// Number of "pixels" (out of 127 in each direction)
// to ignore in the center of the analog stick, because
// it's very common to have the stick rest slighly off-center.
const DEADZONE: i32 = 10;

// The maximum number of pixels to move in a single tick,
// essentially the "mouse sensitivity" of the analog stick.
const MAX_SPEED: i32 = 4;

// Carve a number of rings around our deadzone equal to MAX_SPEED.
const SPEED_MODIFIER: i32 = 127 / MAX_SPEED;

// Convert the analog stick position to a number of pixels to move
// in the direction it is being held.
//
// First, we need to treat 127, 127 as 0,0 on a Cartesian plane.
// We receive coordinates from the PSP control stick like this:
//
// +--------------------+
// |0,0            255,0|
// |                    |
// |                    |
// |                    |
// |      127,127       |
// |                    |
// |                    |
// |                    |
// |0,255        255,255|
// +--------------------+
//
// So we subtract 127 to make our values look like this:
//
// +--------------------+
// |-127,-127   127,-127|
// |                    |
// |                    |
// |                    |
// |         0,0        |
// |                    |
// |                    |
// |                    |
// |-127,127     127,127|
// +--------------------+
//
// Then, we carve out a "deadzone" around 0,0 where inputs are equal to zero.
// So, for a DEADZONE of 8, we would have:
//
// +--------------------+
// |                    |
// |                    |
// |    -8,-8   8,-8    |
// |      +------+      |
// |      |      |      |
// |      |      |      |
// |      +------+      |
// |    -8,8    8,8     |
// |                    |
// |                    |
// +--------------------+
//
// Now, we use MAX_SPEED as the number of "rings" around 0,0.
// So for a MAX_SPEED value of 3, this would look like:
//
//          For Y:            -    For X:    +
//  +--------------------+ +--------------------+
//  |33333333333333333333| |33221100000000112233|
// -|22222222222222222222| |33221100000000112233|
//  |11111111111111111111| |33221100000000112233|
//  |000000+------+000000| |332211+------+112233|
//  |000000|      |000000| |332211|      |112233|
//  |000000|      |000000| |332211|      |112233|
//  |000000+------+000000| |332211+------+112233|
//  |11111111111111111111| |33221100000000112233|
// +|22222222222222222222| |33221100000000112233|
//  |33333333333333333333| |33221100000000112233|
//  +--------------------+ +--------------------+
//
// And for a MAX_SPEED of 6, something like this:
//
//          For Y:            -    For X:    +
//  +--------------------+ +--------------------+
//  |66666666666666666666| |65432100000000123456|
// -|44444444444444444444| |65432100000000123456|
//  |22222222222222222222| |65432100000000123456|
//  |000000+------+000000| |654321+------+123456|
//  |000000|      |000000| |654321|      |123456|
//  |000000|      |000000| |654321|      |123456|
//  |000000+------+000000| |654321+------+123456|
//  |22222222222222222222| |65432100000000123456|
// +|44444444444444444444| |65432100000000123456|
//  |66666666666666666666| |65432100000000123456|
//  +--------------------+ +--------------------+
//
// Or, visualized differently for a MAX_SPEED of 8:
//
// +--------------------+
// |                    |
// |                    |
// |        0,-1        |
// |      +------+      |
// |      | 0,0  |1,0   |
// |      |      |      |
// |      +------+      |
// |                    |
// |  -4,4              |
// |                 8,8|
// +--------------------+
//
pub fn convert_analog_to_delta_with_sensitivity_deadzone(raw_val: u8) -> i32 {
    let delta_val = (raw_val as i32) - 127;

    // Zero out a "deadzone" around 0,0 to adjust for joysticks that sit off-center.
    let distance_without_deadzone = if delta_val > -DEADZONE && delta_val < DEADZONE {
        0
    } else if delta_val < -DEADZONE {
        delta_val + DEADZONE
    } else {
        delta_val - DEADZONE
    };

    distance_without_deadzone / SPEED_MODIFIER
}
