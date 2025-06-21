use colstodian::basic_encodings::*;
use colstodian::*;
use glam::Vec3;

/// Simulates a complete game graphics workflow with realistic color operations.
fn main() {
    println!("=== Game Graphics Color Workflow ===");

    // 1. Asset colors from artists (typically provided as hex/RGB values).
    let character_colors = GameCharacterColors {
        skin: Color::srgb_u8(220, 180, 140),
        hair: Color::srgb_u8(101, 67, 33),
        shirt: Color::srgb_u8(45, 85, 135),
        pants: Color::srgb_u8(65, 65, 65),
    };

    println!("Original character colors:");
    println!("  Skin:  {:?}", character_colors.skin);
    println!("  Hair:  {:?}", character_colors.hair);
    println!("  Shirt: {:?}", character_colors.shirt);
    println!("  Pants: {:?}", character_colors.pants);

    // 2. Convert to linear space for lighting calculations.
    let linear_colors = character_colors.to_linear();

    // 3. Apply environmental lighting.
    println!("\n=== Environmental Lighting ===");

    let lighting_scenarios = [
        ("Daylight", Vec3::new(1.0, 1.0, 1.0), 0.8),
        ("Sunset", Vec3::new(1.2, 0.8, 0.6), 0.6),
        ("Moonlight", Vec3::new(0.7, 0.8, 1.1), 0.3),
        ("Fire", Vec3::new(1.5, 0.7, 0.4), 0.7),
        ("Underwater", Vec3::new(0.6, 0.9, 1.2), 0.4),
    ];

    for (scenario_name, light_color, intensity) in lighting_scenarios {
        println!("\n{} lighting:", scenario_name);

        let lit_colors = apply_lighting(&linear_colors, light_color, intensity);
        let display_colors = lit_colors.to_srgb_u8();

        println!("  Skin:  {:?}", display_colors.skin);
        println!("  Hair:  {:?}", display_colors.hair);
        println!("  Shirt: {:?}", display_colors.shirt);
        println!("  Pants: {:?}", display_colors.pants);
    }

    // 4. Demonstrate post-processing effects.
    println!("\n=== Post-Processing Effects ===");

    let base_scene_color = Color::linear_srgb(0.5, 0.7, 0.9); // Sky blue scene.

    // Simulate various post-processing effects.
    let effects = [
        ("Original", Vec3::ONE, 1.0, 0.0),
        ("Desaturated", Vec3::new(0.5, 0.5, 0.5), 1.0, 0.0),
        ("Warm Filter", Vec3::new(1.2, 1.0, 0.8), 1.1, 0.0),
        ("Cold Filter", Vec3::new(0.8, 1.0, 1.2), 1.0, 0.0),
        ("High Contrast", Vec3::ONE, 1.3, 0.0),
        ("Faded", Vec3::ONE, 0.8, 0.2),
    ];

    for (effect_name, color_mult, contrast, fade) in effects {
        let mut processed = base_scene_color * color_mult * contrast;

        // Apply fade by blending with gray.
        if fade > 0.0 {
            let gray = Color::linear_srgb(0.5, 0.5, 0.5);
            processed = blend_linear(processed, gray, fade);
        }

        let result = processed.convert::<SrgbU8>();
        println!("{:12}: {:?}", effect_name, result);
    }

    // 5. UI color theming.
    println!("\n=== UI Color Theming ===");

    let ui_theme = create_ui_theme(Color::srgb_u8(100, 150, 255)); // Blue theme.

    println!("UI Theme (Blue base):");
    println!("  Primary:      {:?}", ui_theme.primary);
    println!("  Primary Alt:  {:?}", ui_theme.primary_alt);
    println!("  Secondary:    {:?}", ui_theme.secondary);
    println!("  Background:   {:?}", ui_theme.background);
    println!("  Text:         {:?}", ui_theme.text);
    println!("  Accent:       {:?}", ui_theme.accent);
}

#[derive(Debug, Clone)]
struct GameCharacterColors {
    skin: Color<SrgbU8>,
    hair: Color<SrgbU8>,
    shirt: Color<SrgbU8>,
    pants: Color<SrgbU8>,
}

#[derive(Debug, Clone)]
struct LinearCharacterColors {
    skin: Color<LinearSrgb>,
    hair: Color<LinearSrgb>,
    shirt: Color<LinearSrgb>,
    pants: Color<LinearSrgb>,
}

impl GameCharacterColors {
    fn to_linear(&self) -> LinearCharacterColors {
        LinearCharacterColors {
            skin: self.skin.convert(),
            hair: self.hair.convert(),
            shirt: self.shirt.convert(),
            pants: self.pants.convert(),
        }
    }
}

impl LinearCharacterColors {
    fn to_srgb_u8(&self) -> GameCharacterColors {
        GameCharacterColors {
            skin: self.skin.convert(),
            hair: self.hair.convert(),
            shirt: self.shirt.convert(),
            pants: self.pants.convert(),
        }
    }
}

fn apply_lighting(
    colors: &LinearCharacterColors,
    light_color: Vec3,
    intensity: f32,
) -> LinearCharacterColors {
    let ambient = 0.2;
    let direct = intensity;

    LinearCharacterColors {
        skin: colors.skin * (ambient + direct * light_color),
        hair: colors.hair * (ambient + direct * light_color),
        shirt: colors.shirt * (ambient + direct * light_color),
        pants: colors.pants * (ambient + direct * light_color),
    }
}

fn blend_linear(color1: Color<LinearSrgb>, color2: Color<LinearSrgb>, t: f32) -> Color<LinearSrgb> {
    color1 * (1.0 - t) + color2 * t
}

#[derive(Debug)]
struct UITheme {
    primary: Color<SrgbU8>,
    primary_alt: Color<SrgbU8>,
    secondary: Color<SrgbU8>,
    background: Color<SrgbU8>,
    text: Color<SrgbU8>,
    accent: Color<SrgbU8>,
}

fn create_ui_theme(base_color: Color<SrgbU8>) -> UITheme {
    use colstodian::details::encodings::Oklab;

    let base_oklab = base_color.convert::<Oklab>();
    let white_oklab = Color::srgb_u8(255, 255, 255).convert::<Oklab>();
    let black_oklab = Color::srgb_u8(0, 0, 0).convert::<Oklab>();

    UITheme {
        primary: base_color,
        primary_alt: base_oklab.perceptual_blend(white_oklab, 0.3).convert(),
        secondary: base_oklab.perceptual_blend(black_oklab, 0.2).convert(),
        background: Color::srgb_u8(250, 250, 250),
        text: Color::srgb_u8(33, 33, 33),
        accent: base_oklab
            .perceptual_blend(
                Color::srgb_u8(255, 150, 0).convert(), // Orange.
                0.3,
            )
            .convert(),
    }
}
