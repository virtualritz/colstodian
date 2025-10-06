//! Support for custom color spaces with user-defined primaries and white points.
//!
//! This module allows working with colors in arbitrary RGB color spaces that are
//! not predefined in the library. This is useful for:
//!
//! - Working with wide gamut displays.
//! - Processing images from cameras with custom color profiles.
//! - Converting between application-specific color spaces.
//! - Implementing color management for specialized hardware.
//!
//! # Examples
//!
//! ```
//! use colstodian::custom::{CustomColorSpace, DynamicColor};
//! use colstodian::{Color, basic_encodings::LinearSrgb};
//! use glam::Vec3;
//!
//! // Define a wide gamut color space (similar to Adobe RGB).
//! let wide_gamut = CustomColorSpace::from_primaries_d65(
//!     [0.64, 0.33],  // Red primary.
//!     [0.21, 0.71],  // Green primary (wider than sRGB).
//!     [0.15, 0.06],  // Blue primary.
//! );
//!
//! // Create a color in the wide gamut space.
//! let dynamic = DynamicColor::new(0.5, 0.9, 0.3, wide_gamut);
//!
//! // Convert to standard color types for processing.
//! let linear: Color<LinearSrgb> = dynamic.to_color();
//! ```

use crate::details::encodings::{EncodedSrgbF32, EncodedSrgbU8, Srgb, Srgba};
use crate::details::linear_spaces::Srgb as SrgbLinearSpace;
use crate::details::traits::{ConvertFrom, LinearColorSpace, LinearConvertFromRaw};
use crate::{Color, ColorEncoding};

use glam::Vec3;
use kolor::details::conversion::LinearColorConversion;

// Re-export types that are part of our public API.
pub use kolor::details::color::{RgbPrimaries, WhitePoint};

/// A custom color space specification with user-defined primaries and white point.
///
/// # Examples
///
/// ```
/// use colstodian::custom::{CustomColorSpace, DynamicColor, RgbPrimaries, WhitePoint};
/// use colstodian::Color;
/// use colstodian::basic_encodings::LinearSrgb;
/// use glam::Vec3;
///
/// // Define a custom color space with specific primaries and white point.
/// let space = CustomColorSpace {
///     primaries: RgbPrimaries::from_rgb_xy(
///         [0.64, 0.33],  // Red primary (CIE xy).
///         [0.30, 0.60],  // Green primary (CIE xy).
///         [0.15, 0.06],  // Blue primary (CIE xy).
///     ),
///     white_point: WhitePoint::D65,
/// };
///
/// // Create a dynamic color in this space.
/// let dynamic = DynamicColor {
///     value: Vec3::new(0.5, 0.7, 0.3),
///     space,
/// };
///
/// // Convert to a standard Color type.
/// let color: Color<LinearSrgb> = dynamic.into();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustomColorSpace {
    /// The RGB primaries for this color space.
    pub primaries: RgbPrimaries,
    /// The white point for this color space.
    pub white_point: WhitePoint,
}

impl Default for CustomColorSpace {
    fn default() -> Self {
        Self {
            primaries: RgbPrimaries::Bt709, // sRGB primaries.
            white_point: WhitePoint::D65,
        }
    }
}

impl CustomColorSpace {
    /// Create a custom color space from CIE xy chromaticity coordinates.
    ///
    /// This will automatically detect if the values match any known standard
    /// primaries and white points, using the standard enum variants if so.
    pub fn from_primaries_and_white_point(
        r_xy: [f32; 2],
        g_xy: [f32; 2],
        b_xy: [f32; 2],
        white_x: f32,
        white_y: f32,
    ) -> Self {
        Self {
            primaries: RgbPrimaries::from_rgb_xy(r_xy, g_xy, b_xy),
            white_point: WhitePoint::from_xy(white_x, white_y),
        }
    }

    /// Create a custom color space with custom primaries but standard D65 white point.
    pub fn from_primaries_d65(r_xy: [f32; 2], g_xy: [f32; 2], b_xy: [f32; 2]) -> Self {
        Self {
            primaries: RgbPrimaries::from_rgb_xy(r_xy, g_xy, b_xy),
            white_point: WhitePoint::D65,
        }
    }

    /// Create a custom color space with custom primaries but standard D50 white point.
    pub fn from_primaries_d50(r_xy: [f32; 2], g_xy: [f32; 2], b_xy: [f32; 2]) -> Self {
        Self {
            primaries: RgbPrimaries::from_rgb_xy(r_xy, g_xy, b_xy),
            white_point: WhitePoint::D50,
        }
    }

    /// Convert a color from this custom space to CIE XYZ.
    pub fn to_xyz(&self, color: Vec3) -> Vec3 {
        let conversion = LinearColorConversion::new(
            kolor::ColorSpace::new(self.primaries, self.white_point, None),
            kolor::ColorSpace::new(RgbPrimaries::CieXyz, self.white_point, None),
        );
        conversion.convert(color)
    }

    /// Convert a color from CIE XYZ to this custom space.
    pub fn from_xyz(&self, color: Vec3) -> Vec3 {
        let conversion = LinearColorConversion::new(
            kolor::ColorSpace::new(RgbPrimaries::CieXyz, self.white_point, None),
            kolor::ColorSpace::new(self.primaries, self.white_point, None),
        );
        conversion.convert(color)
    }

    /// Convert a color from this custom space to linear sRGB.
    /// Note: This may lose colors outside the sRGB gamut.
    pub fn to_linear_srgb(&self, color: Vec3) -> Vec3 {
        let conversion = LinearColorConversion::new(
            kolor::ColorSpace::new(self.primaries, self.white_point, None),
            kolor::ColorSpace::new(RgbPrimaries::Bt709, WhitePoint::D65, None),
        );
        conversion.convert(color)
    }

    /// Convert a color from linear sRGB to this custom space.
    pub fn from_linear_srgb(&self, color: Vec3) -> Vec3 {
        let conversion = LinearColorConversion::new(
            kolor::ColorSpace::new(RgbPrimaries::Bt709, WhitePoint::D65, None),
            kolor::ColorSpace::new(self.primaries, self.white_point, None),
        );
        conversion.convert(color)
    }
}

/// A color with runtime-defined color space.
///
/// This type allows working with colors in arbitrary color spaces that are not
/// known at compile time. It can be converted to any standard [`Color`] type
/// using `From`/`Into`.
///
/// # Examples
///
/// ```
/// use colstodian::custom::{CustomColorSpace, DynamicColor};
/// use colstodian::Color;
/// use colstodian::basic_encodings::{LinearSrgb, SrgbU8};
/// use glam::Vec3;
///
/// let space = CustomColorSpace::from_primaries_d65(
///     [0.64, 0.33],
///     [0.30, 0.60],
///     [0.15, 0.06],
/// );
///
/// let dynamic = DynamicColor {
///     value: Vec3::new(0.5, 0.7, 0.3),
///     space,
/// };
///
/// // Convert to different Color types.
/// let linear: Color<LinearSrgb> = dynamic.into();
/// let srgb: Color<SrgbU8> = dynamic.into();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DynamicColor {
    /// The color values in the custom color space.
    pub value: Vec3,
    /// The custom color space specification.
    pub space: CustomColorSpace,
}

impl DynamicColor {
    /// Create a new dynamic color.
    pub fn new(r: f32, g: f32, b: f32, space: CustomColorSpace) -> Self {
        Self {
            value: Vec3::new(r, g, b),
            space,
        }
    }

    /// Convert this dynamic color to linear sRGB values.
    fn to_linear_srgb(self) -> Vec3 {
        self.space.to_linear_srgb(self.value)
    }

    /// Convert to any Color type using the most direct conversion path.
    ///
    /// This method uses kolor's conversion infrastructure directly, which
    /// automatically routes through CIE XYZ as an intermediate space when needed,
    /// preserving the full gamut of the source color space.
    ///
    /// # Examples
    ///
    /// ```
    /// use colstodian::custom::{CustomColorSpace, DynamicColor};
    /// use colstodian::Color;
    /// use colstodian::basic_encodings::{LinearSrgb, SrgbU8};
    /// use colstodian::details::encodings::Oklab;
    /// use glam::Vec3;
    ///
    /// let space = CustomColorSpace::from_primaries_d65(
    ///     [0.64, 0.33],
    ///     [0.30, 0.60],
    ///     [0.15, 0.06],
    /// );
    ///
    /// let dynamic = DynamicColor {
    ///     value: Vec3::new(0.5, 0.7, 0.3),
    ///     space,
    /// };
    ///
    /// // Convert to different Color types.
    /// let linear: Color<LinearSrgb> = dynamic.to_color();
    /// let srgb: Color<SrgbU8> = dynamic.to_color();
    /// let oklab: Color<Oklab> = dynamic.to_color();
    /// ```
    pub fn to_color<E>(&self) -> Color<E>
    where
        E: ColorEncoding,
        E::LinearSpace: crate::details::traits::LinearColorSpace,
    {
        use kolor::ColorSpace;
        use kolor::details::conversion::LinearColorConversion;

        // Get the target encoding's linear space primaries and white point.
        let dst_primaries = E::LinearSpace::primaries();
        let dst_white = E::LinearSpace::white_point();

        // Create kolor color spaces.
        let src_space = ColorSpace::new(self.space.primaries, self.space.white_point, None);
        let dst_linear_space = ColorSpace::new(dst_primaries, dst_white, None);

        // Convert using kolor's optimal path (through XYZ if needed).
        let conversion = LinearColorConversion::new(src_space, dst_linear_space);
        let linear_value = conversion.convert(self.value);

        // Apply the target encoding's transform.
        let encoded = E::dst_transform_raw(linear_value, 1.0);
        Color::from_repr(encoded)
    }
}

// Implement From<DynamicColor> for various Color types.

impl From<DynamicColor> for Color<Srgb> {
    fn from(dynamic: DynamicColor) -> Self {
        let srgb = dynamic.to_linear_srgb();
        Color::srgb(srgb.x, srgb.y, srgb.z)
    }
}

impl From<DynamicColor> for Color<Srgba> {
    fn from(dynamic: DynamicColor) -> Self {
        let srgb = dynamic.to_linear_srgb();
        Color::srgba(srgb.x, srgb.y, srgb.z, 1.0)
    }
}

impl From<DynamicColor> for Color<EncodedSrgbU8> {
    fn from(dynamic: DynamicColor) -> Self {
        // First convert to linear sRGB.
        let linear_srgb: Color<Srgb> = dynamic.into();
        // Then use the standard conversion to EncodedSrgbU8.
        linear_srgb.convert()
    }
}

impl From<DynamicColor> for Color<EncodedSrgbF32> {
    fn from(dynamic: DynamicColor) -> Self {
        // First convert to linear sRGB.
        let linear_srgb: Color<Srgb> = dynamic.into();
        // Then use the standard conversion to EncodedSrgbF32.
        linear_srgb.convert()
    }
}

// Note: We can't implement a generic From<DynamicColor> for all Color<E> types
// because it would conflict with the specific implementations above.
// Users can convert via Srgb as an intermediate step if needed.

// Extension methods for Color to work with custom color spaces.

impl Color<Srgb> {
    /// Create a Color from values in a custom color space.
    ///
    /// # Examples
    ///
    /// ```
    /// use colstodian::Color;
    /// use colstodian::basic_encodings::LinearSrgb;
    /// use colstodian::custom::CustomColorSpace;
    /// use colstodian::details::encodings::Srgb;
    ///
    /// let space = CustomColorSpace::from_primaries_d65(
    ///     [0.64, 0.33],
    ///     [0.30, 0.60],
    ///     [0.15, 0.06],
    /// );
    ///
    /// let color = Color::<Srgb>::from_custom(&space, 0.5, 0.7, 0.3);
    /// ```
    pub fn from_custom(space: &CustomColorSpace, r: f32, g: f32, b: f32) -> Self {
        let srgb = space.to_linear_srgb(Vec3::new(r, g, b));
        Color::srgb(srgb.x, srgb.y, srgb.z)
    }
}

impl<E: ColorEncoding> Color<E>
where
    Srgb: ConvertFrom<E>,
    SrgbLinearSpace: LinearConvertFromRaw<<E as ColorEncoding>::LinearSpace>,
{
    /// Convert this color to RGB values in a custom color space.
    ///
    /// # Examples
    ///
    /// ```
    /// use colstodian::Color;
    /// use colstodian::basic_encodings::EncodedSrgbU8;
    /// use colstodian::custom::CustomColorSpace;
    ///
    /// let space = CustomColorSpace::from_primaries_d65(
    ///     [0.64, 0.33],
    ///     [0.30, 0.60],
    ///     [0.15, 0.06],
    /// );
    ///
    /// let color = Color::srgb_u8(128, 200, 100);
    /// let (r, g, b) = color.to_custom_rgb(&space);
    /// ```
    pub fn to_custom_rgb(&self, space: &CustomColorSpace) -> (f32, f32, f32) {
        // First convert to linear sRGB.
        let linear_srgb: Color<Srgb> = self.convert();
        let repr = linear_srgb.repr;

        // Then convert to custom space.
        let custom = space.from_linear_srgb(repr);
        (custom.x, custom.y, custom.z)
    }
}
