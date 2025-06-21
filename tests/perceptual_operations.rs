use approx::assert_relative_eq;
use colstodian::{Color, basic_encodings::*, details::encodings::*};

#[test]
fn oklab_perceptual_blend() {
    let start = Color::srgb_u8(255, 0, 0); // Red.
    let end = Color::srgb_u8(0, 0, 255); // Blue.

    let start_oklab = start.convert::<Oklab>();
    let end_oklab = end.convert::<Oklab>();

    let mid_oklab = start_oklab.perceptual_blend(end_oklab, 0.5);
    let result_srgb = mid_oklab.convert::<SrgbU8>();

    // Computed expected value for 50% blend of red and blue in Oklab space
    // This gives a perceptually uniform midpoint between red and blue.
    let expected = Color::srgb_u8(140, 83, 162);

    assert_eq!(result_srgb, expected);
}

#[test]
fn oklab_blend_extremes() {
    let color = Color::srgb_u8(128, 64, 192);
    let oklab = color.convert::<Oklab>();

    // Blend with self at t=0 should return original.
    let blend_zero = oklab.perceptual_blend(oklab, 0.0);
    assert_relative_eq!(blend_zero, oklab, epsilon = 0.001);

    // Blend with self at t=1 should return original.
    let blend_one = oklab.perceptual_blend(oklab, 1.0);
    assert_relative_eq!(blend_one, oklab, epsilon = 0.001);
}

#[test]
fn oklab_blend_interpolation() {
    let start = Color::srgb_u8(0, 0, 0); // Black.
    let end = Color::srgb_u8(255, 255, 255); // White.

    let start_oklab = start.convert::<Oklab>();
    let end_oklab = end.convert::<Oklab>();

    // Test interpolation points.
    let quarter = start_oklab.perceptual_blend(end_oklab, 0.25);
    let half = start_oklab.perceptual_blend(end_oklab, 0.5);
    let three_quarter = start_oklab.perceptual_blend(end_oklab, 0.75);

    // Ensure lightness progresses monotonically.
    assert!(quarter.l >= start_oklab.l);
    assert!(half.l >= quarter.l);
    assert!(three_quarter.l >= half.l);
    assert!(end_oklab.l >= three_quarter.l);
}
