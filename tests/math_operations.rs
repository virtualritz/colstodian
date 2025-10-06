use approx::assert_relative_eq;
use colstodian::*;
use glam::Vec3;

#[test]
fn linear_color_addition() {
    let color1 = Color::linear_srgb(0.2, 0.3, 0.4);
    let color2 = Color::linear_srgb(0.1, 0.2, 0.1);
    let result = color1 + color2;

    assert_relative_eq!(result.r, 0.3, epsilon = 0.001);
    assert_relative_eq!(result.g, 0.5, epsilon = 0.001);
    assert_relative_eq!(result.b, 0.5, epsilon = 0.001);
}

#[test]
fn linear_color_subtraction() {
    let color1 = Color::linear_srgb(0.5, 0.7, 0.9);
    let color2 = Color::linear_srgb(0.1, 0.2, 0.3);
    let result = color1 - color2;

    assert_relative_eq!(result.r, 0.4, epsilon = 0.001);
    assert_relative_eq!(result.g, 0.5, epsilon = 0.001);
    assert_relative_eq!(result.b, 0.6, epsilon = 0.001);
}

#[test]
fn linear_color_scalar_multiplication() {
    let color = Color::linear_srgb(0.4, 0.6, 0.8);
    let result = color * 0.5;

    assert_relative_eq!(result.r, 0.2, epsilon = 0.001);
    assert_relative_eq!(result.g, 0.3, epsilon = 0.001);
    assert_relative_eq!(result.b, 0.4, epsilon = 0.001);
}

#[test]
fn linear_color_vec3_multiplication() {
    let color = Color::linear_srgb(0.4, 0.6, 0.8);
    let multiplier = Vec3::new(0.5, 2.0, 0.25);
    let result = color * multiplier;

    assert_relative_eq!(result.r, 0.2, epsilon = 0.001);
    assert_relative_eq!(result.g, 1.2, epsilon = 0.001);
    assert_relative_eq!(result.b, 0.2, epsilon = 0.001);
}

#[test]
fn linear_color_division() {
    let color = Color::linear_srgb(0.4, 0.6, 0.8);
    let result = color / 2.0;

    assert_relative_eq!(result.r, 0.2, epsilon = 0.001);
    assert_relative_eq!(result.g, 0.3, epsilon = 0.001);
    assert_relative_eq!(result.b, 0.4, epsilon = 0.001);
}

#[test]
fn assignment_operations() {
    let mut color = Color::linear_srgb(0.4, 0.6, 0.8);

    color += Color::linear_srgb(0.1, 0.1, 0.1);
    assert_relative_eq!(color.r, 0.5, epsilon = 0.001);
    assert_relative_eq!(color.g, 0.7, epsilon = 0.001);
    assert_relative_eq!(color.b, 0.9, epsilon = 0.001);

    color -= Color::linear_srgb(0.2, 0.2, 0.2);
    assert_relative_eq!(color.r, 0.3, epsilon = 0.001);
    assert_relative_eq!(color.g, 0.5, epsilon = 0.001);
    assert_relative_eq!(color.b, 0.7, epsilon = 0.001);

    color *= 2.0;
    assert_relative_eq!(color.r, 0.6, epsilon = 0.001);
    assert_relative_eq!(color.g, 1.0, epsilon = 0.001);
    assert_relative_eq!(color.b, 1.4, epsilon = 0.001);

    color /= Vec3::new(2.0, 2.0, 2.0);
    assert_relative_eq!(color.r, 0.3, epsilon = 0.001);
    assert_relative_eq!(color.g, 0.5, epsilon = 0.001);
    assert_relative_eq!(color.b, 0.7, epsilon = 0.001);
}
