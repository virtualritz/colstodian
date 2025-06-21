use colstodian::basic_encodings::*;
use colstodian::*;
use glam::Vec3;

/// Demonstrates mathematical operations with colors.
fn main() {
    println!("=== Color Mathematics ===");

    // Create linear colors for math operations.
    let red = Color::linear_srgb(1.0, 0.0, 0.0);
    let green = Color::linear_srgb(0.0, 1.0, 0.0);
    let blue = Color::linear_srgb(0.0, 0.0, 1.0);

    println!("Red:   {:?}", red);
    println!("Green: {:?}", green);
    println!("Blue:  {:?}", blue);

    // Basic arithmetic.
    println!("\n=== Addition ===");
    let yellow = red + green;
    let magenta = red + blue;
    let cyan = green + blue;
    let white = red + green + blue;

    println!("Red + Green = Yellow:  {:?}", yellow);
    println!("Red + Blue = Magenta:  {:?}", magenta);
    println!("Green + Blue = Cyan:   {:?}", cyan);
    println!("All colors = White:    {:?}", white);

    // Scalar operations.
    println!("\n=== Scalar Operations ===");
    let dim_red = red * 0.5;
    let bright_green = green * 2.0;
    let half_blue = blue / 2.0;

    println!("Dim red (0.5x):     {:?}", dim_red);
    println!("Bright green (2x):  {:?}", bright_green);
    println!("Half blue (/2):     {:?}", half_blue);

    // Vector operations.
    println!("\n=== Vector Operations ===");
    let base_color = Color::linear_srgb(0.8, 0.6, 0.4);
    let tint = Vec3::new(1.2, 1.0, 0.8); // Warm tint.
    let tinted = base_color * tint;

    println!("Base color: {:?}", base_color);
    println!("Tint:       {:?}", tint);
    println!("Tinted:     {:?}", tinted);

    // Lighting simulation.
    println!("\n=== Lighting Simulation ===");
    let object_color = Color::linear_srgb(0.7, 0.3, 0.1); // Orange object.
    let light_color = Color::linear_srgb(1.0, 0.9, 0.8); // Warm light.
    let ambient = 0.2;
    let direct = 0.8;

    // Combine ambient and direct lighting.
    let ambient_component = object_color * ambient;
    let direct_component = object_color * direct;

    // Scale light color with direct component.
    let red_light = light_color.r * direct_component.r;
    let green_light = light_color.g * direct_component.g;
    let blue_light = light_color.b * direct_component.b;
    let lit_direct = Color::linear_srgb(red_light, green_light, blue_light);

    let lit_color = ambient_component + lit_direct;

    println!("Object color: {:?}", object_color);
    println!("Light color:  {:?}", light_color);
    println!("Final color:  {:?}", lit_color);

    // Convert back to sRGB for display.
    let display_color = lit_color.convert::<SrgbU8>();
    println!("Display RGB:  {:?}", display_color);
}
