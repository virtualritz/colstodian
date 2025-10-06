use approx::relative_eq;
use colstodian::basic_encodings::*;
use colstodian::details::encodings::{EncodedSrgbF32, Oklab};
use colstodian::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn srgb_u8_to_linear_round_trip(r in 5u8..=250, g in 5u8..=250, b in 5u8..=250) {
        let original = Color::srgb_u8(r, g, b);
        let linear = original.convert::<LinearSrgb>();
        let back = linear.convert::<SrgbU8>();

        // Allow for minor rounding errors due to precision loss.
        // Avoid extreme values near 0 and 255 where precision issues are more common.
        prop_assert!(original.r.abs_diff(back.r) <= 2);
        prop_assert!(original.g.abs_diff(back.g) <= 2);
        prop_assert!(original.b.abs_diff(back.b) <= 2);
    }

    #[test]
    fn srgb_f32_to_linear_round_trip(
        r in 0.05f32..=0.95,
        g in 0.05f32..=0.95,
        b in 0.05f32..=0.95
    ) {
        let original = Color::srgb_f32(r, g, b);
        let linear = original.convert::<LinearSrgb>();
        let back = linear.convert::<EncodedSrgbF32>();

        // Avoid extreme values near 0 and 1 where precision issues are more common.
        prop_assert!(relative_eq!(back.r, original.r, epsilon = 0.01));
        prop_assert!(relative_eq!(back.g, original.g, epsilon = 0.01));
        prop_assert!(relative_eq!(back.b, original.b, epsilon = 0.01));
    }

    #[test]
    fn linear_to_oklab_round_trip(
        r in 0.0f32..=1.0,
        g in 0.0f32..=1.0,
        b in 0.0f32..=1.0
    ) {
        let original = Color::linear_srgb(r, g, b);
        let oklab = original.convert::<Oklab>();
        let back = oklab.convert::<LinearSrgb>();

        // Oklab conversion may have larger tolerances due to complex transforms.
        prop_assert!(relative_eq!(back.r, original.r, epsilon = 0.01));
        prop_assert!(relative_eq!(back.g, original.g, epsilon = 0.01));
        prop_assert!(relative_eq!(back.b, original.b, epsilon = 0.01));
    }

    #[test]
    fn alpha_preservation_through_conversions(
        r in 0u8..=255,
        g in 0u8..=255,
        b in 0u8..=255,
        a in 0u8..=255
    ) {
        let original = Color::srgba_u8(r, g, b, a);
        let linear = original.convert::<LinearSrgba>();
        let back = linear.convert::<SrgbaU8>();

        // Alpha should be preserved exactly through linear conversions.
        prop_assert_eq!(original.a, back.a);
    }

    #[test]
    fn linear_color_math_properties(
        r1 in 0.0f32..=1.0,
        g1 in 0.0f32..=1.0,
        b1 in 0.0f32..=1.0,
        r2 in 0.0f32..=1.0,
        g2 in 0.0f32..=1.0,
        b2 in 0.0f32..=1.0
    ) {
        let color1 = Color::linear_srgb(r1, g1, b1);
        let color2 = Color::linear_srgb(r2, g2, b2);

        // Addition is commutative.
        let sum1 = color1 + color2;
        let sum2 = color2 + color1;
        prop_assert!(relative_eq!(sum1.r, sum2.r, epsilon = 0.001));
        prop_assert!(relative_eq!(sum1.g, sum2.g, epsilon = 0.001));
        prop_assert!(relative_eq!(sum1.b, sum2.b, epsilon = 0.001));

        // Subtraction is inverse of addition.
        let diff = color1 - color2;
        let back = diff + color2;
        prop_assert!(relative_eq!(back.r, color1.r, epsilon = 0.001));
        prop_assert!(relative_eq!(back.g, color1.g, epsilon = 0.001));
        prop_assert!(relative_eq!(back.b, color1.b, epsilon = 0.001));
    }

    #[test]
    fn perceptual_blend_bounds(
        r1 in 0u8..=255,
        g1 in 0u8..=255,
        b1 in 0u8..=255,
        r2 in 0u8..=255,
        g2 in 0u8..=255,
        b2 in 0u8..=255,
        t in 0.0f32..=1.0
    ) {
        let color1 = Color::srgb_u8(r1, g1, b1).convert::<Oklab>();
        let color2 = Color::srgb_u8(r2, g2, b2).convert::<Oklab>();

        let blend = color1.perceptual_blend(color2, t);

        // Blended lightness should be between the two input lightnesses.
        let min_l = color1.l.min(color2.l);
        let max_l = color1.l.max(color2.l);
        prop_assert!(blend.l >= min_l - 0.01); // Small tolerance for floating point.
        prop_assert!(blend.l <= max_l + 0.01);

        // At t=0, should be very close to color1.
        if t < 0.01 {
            let blend_zero = color1.perceptual_blend(color2, 0.0);
            prop_assert!(relative_eq!(blend_zero.l, color1.l, epsilon = 0.001));
        }

        // At t=1, should be very close to color2.
        if t > 0.99 {
            let blend_one = color1.perceptual_blend(color2, 1.0);
            prop_assert!(relative_eq!(blend_one.l, color2.l, epsilon = 0.001));
        }
    }
}
