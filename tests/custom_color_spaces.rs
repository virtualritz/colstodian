//! Tests for custom color space support.

use colstodian::Color;
use colstodian::basic_encodings::{LinearSrgb, SrgbU8};
use colstodian::custom::{CustomColorSpace, DynamicColor, RgbPrimaries, WhitePoint};
use glam::Vec3;

#[test]
fn test_custom_color_space_round_trip() {
    // Create a custom color space with known primaries.
    let space = CustomColorSpace::from_primaries_d65(
        [0.64, 0.33], // Red - same as sRGB.
        [0.30, 0.60], // Green - same as sRGB.
        [0.15, 0.06], // Blue - same as sRGB.
    );

    // This should be equivalent to sRGB primaries.
    assert_eq!(space.primaries, RgbPrimaries::Bt709);
    assert_eq!(space.white_point, WhitePoint::D65);

    // Test round-trip conversion.
    let original = Vec3::new(0.5, 0.7, 0.3);
    let to_srgb = space.to_linear_srgb(original);
    let back = space.from_linear_srgb(to_srgb);

    // Check components are preserved.
    assert!((original.x - back.x).abs() < 0.0001);
    assert!((original.y - back.y).abs() < 0.0001);
    assert!((original.z - back.z).abs() < 0.0001);
}

#[test]
fn test_dynamic_color_to_linear_srgb() {
    let space = CustomColorSpace::from_primaries_d65([0.64, 0.33], [0.30, 0.60], [0.15, 0.06]);

    let dynamic = DynamicColor::new(0.5, 0.7, 0.3, space);
    let linear: Color<LinearSrgb> = dynamic.to_color();

    // Since we're using sRGB primaries, the values should be the same.
    let repr = linear.repr;
    assert!((repr.x - 0.5).abs() < 0.0001);
    assert!((repr.y - 0.7).abs() < 0.0001);
    assert!((repr.z - 0.3).abs() < 0.0001);
}

#[test]
fn test_dynamic_color_with_wide_gamut() {
    // Create a wide gamut color space (similar to Adobe RGB).
    let space = CustomColorSpace::from_primaries_d65(
        [0.64, 0.33],
        [0.21, 0.71], // Wider green primary.
        [0.15, 0.06],
    );

    // Create a color that's outside sRGB gamut in the green channel.
    let dynamic = DynamicColor::new(0.3, 0.95, 0.2, space);

    // Convert to linear sRGB - this will map the wide gamut color.
    let linear: Color<LinearSrgb> = dynamic.to_color();

    // The color should be converted, but exact values depend on the primaries.
    assert!(linear.repr.x >= 0.0);
    assert!(linear.repr.y >= 0.0);
    assert!(linear.repr.z >= 0.0);
}

#[test]
fn test_from_dynamic_color_variants() {
    let space = CustomColorSpace::default(); // Uses sRGB primaries.
    let dynamic = DynamicColor::new(0.5, 0.7, 0.3, space);

    // Test conversion to various types.
    let _linear: Color<LinearSrgb> = dynamic.into();
    let _srgb_u8: Color<SrgbU8> = dynamic.into();
}

#[test]
fn test_color_to_custom_rgb() {
    // Create a custom space.
    let space = CustomColorSpace::from_primaries_d65([0.64, 0.33], [0.30, 0.60], [0.15, 0.06]);

    // Create a color in sRGB.
    let srgb = Color::srgb_u8(128, 200, 100);

    // Convert to custom space (which is the same as sRGB in this case).
    let (r, g, b) = srgb.to_custom_rgb(&space);

    // Values should be roughly preserved since we're using sRGB primaries.
    assert!(r >= 0.0 && r <= 1.0);
    assert!(g >= 0.0 && g <= 1.0);
    assert!(b >= 0.0 && b <= 1.0);
}

#[test]
fn test_xyz_intermediate_preserves_wide_gamut() {
    // Create a very wide gamut color space.
    let wide_space = CustomColorSpace::from_primaries_and_white_point(
        [0.7347, 0.2653],  // ACES AP0 red.
        [0.0000, 1.0000],  // ACES AP0 green.
        [0.0001, -0.0770], // ACES AP0 blue.
        0.3127,
        0.3290, // D65 white point.
    );

    // Create a highly saturated green that would be clipped in sRGB.
    let dynamic = DynamicColor::new(0.0, 1.0, 0.0, wide_space);

    // Convert through the system - should preserve the wide gamut via XYZ.
    let linear: Color<LinearSrgb> = dynamic.to_color();

    // The green channel will be mapped but should still be high.
    assert!(linear.repr.y > 0.5); // Should have significant green component.
}
