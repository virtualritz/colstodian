use approx::assert_relative_eq;
use colstodian::{Color, basic_encodings::*, details::encodings::*};

#[test]
fn srgb_u8_to_f32_conversion() {
    let u8_color = Color::srgb_u8(255, 0, 0);
    let f32_color = u8_color.convert::<SrgbF32>();

    assert_relative_eq!(f32_color.r, 1.0, epsilon = 0.001);
    assert_relative_eq!(f32_color.g, 0.0, epsilon = 0.001);
    assert_relative_eq!(f32_color.b, 0.0, epsilon = 0.001);
}

#[test]
fn srgb_f32_to_u8_conversion() {
    let f32_color = Color::srgb_f32(1.0, 0.0, 0.0);
    let u8_color = f32_color.convert::<SrgbU8>();

    assert_eq!(u8_color.r, 255);
    assert_eq!(u8_color.g, 0);
    assert_eq!(u8_color.b, 0);
}

#[test]
fn srgb_to_linear_conversion() {
    let srgb_color = Color::srgb_f32(1.0, 1.0, 1.0);
    let linear_color = srgb_color.convert::<LinearSrgb>();

    // sRGB white should convert to linear white.
    assert_relative_eq!(linear_color.r, 1.0, epsilon = 0.001);
    assert_relative_eq!(linear_color.g, 1.0, epsilon = 0.001);
    assert_relative_eq!(linear_color.b, 1.0, epsilon = 0.001);
}

#[test]
fn linear_to_srgb_conversion() {
    let linear_color = Color::linear_srgb(1.0, 1.0, 1.0);
    let srgb_color = linear_color.convert::<SrgbF32>();

    // Linear white should convert to sRGB white.
    assert_relative_eq!(srgb_color.r, 1.0, epsilon = 0.001);
    assert_relative_eq!(srgb_color.g, 1.0, epsilon = 0.001);
    assert_relative_eq!(srgb_color.b, 1.0, epsilon = 0.001);
}

#[test]
fn alpha_preservation() {
    let rgba_color = Color::srgba_u8(255, 128, 64, 200);
    let linear_rgba = rgba_color.convert::<LinearSrgba>();
    let back_to_srgba = linear_rgba.convert::<SrgbaU8>();

    assert_eq!(back_to_srgba.a, 200);
}

#[test]
fn extremes_conversion() {
    // Test black.
    let black_u8 = Color::srgb_u8(0, 0, 0);
    let black_linear = black_u8.convert::<LinearSrgb>();
    assert_relative_eq!(black_linear.r, 0.0, epsilon = 0.001);
    assert_relative_eq!(black_linear.g, 0.0, epsilon = 0.001);
    assert_relative_eq!(black_linear.b, 0.0, epsilon = 0.001);

    // Test white.
    let white_u8 = Color::srgb_u8(255, 255, 255);
    let white_linear = white_u8.convert::<LinearSrgb>();
    assert_relative_eq!(white_linear.r, 1.0, epsilon = 0.001);
    assert_relative_eq!(white_linear.g, 1.0, epsilon = 0.001);
    assert_relative_eq!(white_linear.b, 1.0, epsilon = 0.001);
}
