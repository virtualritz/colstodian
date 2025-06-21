use colstodian::*;
use std::collections::HashMap;

#[test]
fn color_eq_trait() {
    let color1 = Color::srgb_u8(255, 128, 64);
    let color2 = Color::srgb_u8(255, 128, 64);
    let color3 = Color::srgb_u8(255, 128, 65);

    // Test Eq trait (requires PartialEq + reflexivity, symmetry, transitivity).
    assert!(color1 == color2);
    assert!(color1 != color3);

    // Test that we can use == and != operators.
    assert_eq!(color1, color2);
    assert_ne!(color1, color3);
}

#[test]
fn color_hash_trait() {
    use std::collections::HashSet;

    let color1 = Color::srgb_u8(255, 128, 64);
    let color2 = Color::srgb_u8(255, 128, 64);
    let color3 = Color::srgb_u8(255, 128, 65);

    // Test that we can use colors as HashMap keys.
    let mut color_map = HashMap::new();
    color_map.insert(color1, "red-orange");
    color_map.insert(color3, "slightly different red-orange");

    assert_eq!(color_map.get(&color1), Some(&"red-orange"));
    assert_eq!(color_map.get(&color2), Some(&"red-orange")); // Same color as color1.
    assert_eq!(
        color_map.get(&color3),
        Some(&"slightly different red-orange")
    );

    // Test that we can use colors in HashSet.
    let mut color_set = HashSet::new();
    color_set.insert(color1);
    color_set.insert(color2); // Should not add duplicate.
    color_set.insert(color3);

    assert_eq!(color_set.len(), 2); // Only 2 unique colors.
    assert!(color_set.contains(&color1));
    assert!(color_set.contains(&color2)); // Same as color1.
    assert!(color_set.contains(&color3));
}

#[test]
fn linear_color_equality_only() {
    // Note: Linear colors use floating-point representations (Vec3/Vec4)
    // which don't implement Eq and Hash due to floating-point precision issues.
    // We can only test PartialEq for these.

    let color1 = Color::linear_srgb(1.0, 0.5, 0.25);
    let color2 = Color::linear_srgb(1.0, 0.5, 0.25);
    let color3 = Color::linear_srgb(1.0, 0.5, 0.26);

    // Test PartialEq works.
    assert_eq!(color1, color2);
    assert_ne!(color1, color3);

    // Note: We cannot use these in HashMap/HashSet due to lack of Eq + Hash.
    // This is expected and correct behavior for floating-point color types.
}

#[test]
fn component_struct_traits() {
    use colstodian::details::component_structs::*;
    use std::collections::HashMap;

    let rgb1 = Rgb {
        r: 255u8,
        g: 128u8,
        b: 64u8,
    };
    let rgb2 = Rgb {
        r: 255u8,
        g: 128u8,
        b: 64u8,
    };
    let rgb3 = Rgb {
        r: 255u8,
        g: 128u8,
        b: 65u8,
    };

    // Test Eq trait.
    assert_eq!(rgb1, rgb2);
    assert_ne!(rgb1, rgb3);

    // Test Hash trait.
    let mut component_map = HashMap::new();
    component_map.insert(rgb1, "primary");
    component_map.insert(rgb3, "variant");

    assert_eq!(component_map.get(&rgb1), Some(&"primary"));
    assert_eq!(component_map.get(&rgb2), Some(&"primary")); // Same as rgb1.
    assert_eq!(component_map.get(&rgb3), Some(&"variant"));
}

#[test]
fn rgba_component_struct_traits() {
    use colstodian::details::component_structs::*;
    use std::collections::HashSet;

    let rgba1 = Rgba {
        r: 255u8,
        g: 128u8,
        b: 64u8,
        a: 200u8,
    };
    let rgba2 = Rgba {
        r: 255u8,
        g: 128u8,
        b: 64u8,
        a: 200u8,
    };
    let rgba3 = Rgba {
        r: 255u8,
        g: 128u8,
        b: 64u8,
        a: 201u8,
    };

    let mut rgba_set = HashSet::new();
    rgba_set.insert(rgba1);
    rgba_set.insert(rgba2); // Should not add duplicate.
    rgba_set.insert(rgba3);

    assert_eq!(rgba_set.len(), 2); // Only 2 unique RGBA values.
    assert!(rgba_set.contains(&rgba1));
    assert!(rgba_set.contains(&rgba2)); // Same as rgba1.
    assert!(rgba_set.contains(&rgba3));
}
