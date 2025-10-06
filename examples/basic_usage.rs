use colstodian::basic_encodings::*;
use colstodian::details::encodings::EncodedSrgbF32;
use colstodian::*;

/// Basic color creation and conversion examples.
fn main() {
    println!("=== Basic Color Usage ===");

    // Create colors in different ways.
    let color_u8 = Color::srgb_u8(255, 128, 64);
    let color_f32 = Color::srgb_f32(1.0, 0.5, 0.25);
    let color_linear = Color::linear_srgb(0.8, 0.4, 0.1);

    println!("sRGB U8:     {:?}", color_u8);
    println!("sRGB F32:    {:?}", color_f32);
    println!("Linear sRGB: {:?}", color_linear);

    // Convert between encodings.
    println!("\n=== Color Conversions ===");

    let u8_to_f32 = color_u8.convert::<EncodedSrgbF32>();
    let f32_to_linear = color_f32.convert::<LinearSrgb>();
    let linear_to_u8 = color_linear.convert::<SrgbU8>();

    println!("U8 -> F32:       {:?}", u8_to_f32);
    println!("F32 -> Linear:   {:?}", f32_to_linear);
    println!("Linear -> U8:    {:?}", linear_to_u8);

    // Access individual components.
    println!("\n=== Component Access ===");
    println!("Red component:   {}", color_f32.r);
    println!("Green component: {}", color_f32.g);
    println!("Blue component:  {}", color_f32.b);

    // Work with alpha.
    println!("\n=== Alpha Channels ===");
    let rgba_color = Color::srgba_u8(255, 128, 64, 200);
    println!("RGBA color: {:?}", rgba_color);
    println!("Alpha: {}", rgba_color.a);

    let rgba_linear = rgba_color.convert::<LinearSrgba>();
    println!("Linear RGBA: {:?}", rgba_linear);
}
