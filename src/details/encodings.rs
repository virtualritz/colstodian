use crate::Color;
use crate::component_structs::*;
use crate::linear_spaces;
use crate::reprs::*;
use crate::traits::*;

use glam::Vec3;
use glam::Vec4;
use glam::Vec4Swizzles;
use kolor::details::color::WhitePoint;
use kolor::details::transform;

#[inline(always)]
fn u8_to_f32(x: u8) -> f32 {
    x as f32 / 255.0
}

#[inline(always)]
fn f32_to_u8(x: f32) -> u8 {
    (x.clamp(0.0, 1.0) * 255.0).round() as u8
}

#[doc = include_str!("descriptions/srgb_u8.md")]
pub struct EncodedSrgbU8;

impl Color<EncodedSrgbU8> {
    /// Create a [`Color`] in the [`EncodedSrgbU8`] encoding.
    ///
    /// **If you don't know what you're doing and you have RGB values from a
    /// color picker that vary from `0-255`, use this.**
    ///
    /// If you're not sure, see [the `EncodedSrgbU8` encoding docs][EncodedSrgbU8] for more
    /// info.
    #[inline(always)]
    pub const fn encoded_srgb_u8(r: u8, g: u8, b: u8) -> Self {
        Color::from_repr([r, g, b])
    }

    /// Alias for backward compatibility.
    #[inline(always)]
    pub const fn srgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::encoded_srgb_u8(r, g, b)
    }
}

impl ColorEncoding for EncodedSrgbU8 {
    type ComponentStruct = Rgb<u8>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = U8Repr;

    const NAME: &'static str = "EncodedSrgbU8";

    #[inline]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let [x, y, z] = repr;
        let raw_electro = Vec3::new(u8_to_f32(x), u8_to_f32(y), u8_to_f32(z));
        let optical = transform::srgb_eotf(raw_electro, WhitePoint::D65);
        (optical, 1.0)
    }

    #[inline]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        let electro = transform::srgb_oetf(raw, WhitePoint::D65);

        [
            f32_to_u8(electro.x),
            f32_to_u8(electro.y),
            f32_to_u8(electro.z),
        ]
    }
}

impl ConvertFrom<EncodedSrgbF32> for EncodedSrgbU8 {}
impl ConvertFrom<EncodedSrgbaU8> for EncodedSrgbU8 {}
impl ConvertFrom<EncodedSrgbaF32> for EncodedSrgbU8 {}
impl ConvertFrom<EncodedSrgbaPremultipliedU8> for EncodedSrgbU8 {}
impl ConvertFrom<Srgb> for EncodedSrgbU8 {}
impl ConvertFrom<Srgba> for EncodedSrgbU8 {}
impl ConvertFrom<SrgbaPremultiplied> for EncodedSrgbU8 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for EncodedSrgbU8 {}

#[doc = include_str!("descriptions/srgb_f32.md")]
pub struct EncodedSrgbF32;

impl Color<EncodedSrgbF32> {
    /// Create a [`Color`] in the [`EncodedSrgbF32`] encoding.
    ///
    /// **If you don't know what you're doing and you have RGB values from a
    /// color picker that vary from `0.0..=1.0`, use this.**
    ///
    /// If you're not sure, see [the `EncodedSrgbF32` encoding docs][EncodedSrgbF32] for more
    /// info.
    #[inline(always)]
    pub const fn encoded_srgb_f32(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }

    /// Alias for backward compatibility.
    #[inline(always)]
    pub const fn srgb_f32(r: f32, g: f32, b: f32) -> Self {
        Self::encoded_srgb_f32(r, g, b)
    }
}

impl ColorEncoding for EncodedSrgbF32 {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32Repr;

    const NAME: &'static str = "EncodedSrgbF32";

    #[inline]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let optical = transform::srgb_eotf(repr, WhitePoint::D65);
        (optical, 1.0)
    }

    #[inline]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        transform::srgb_oetf(raw, WhitePoint::D65)
    }
}

impl ConvertFrom<EncodedSrgbU8> for EncodedSrgbF32 {}
impl ConvertFrom<EncodedSrgbaU8> for EncodedSrgbF32 {}
impl ConvertFrom<EncodedSrgbaF32> for EncodedSrgbF32 {}
impl ConvertFrom<EncodedSrgbaPremultipliedU8> for EncodedSrgbF32 {}
impl ConvertFrom<Srgb> for EncodedSrgbF32 {}
impl ConvertFrom<Srgba> for EncodedSrgbF32 {}
impl ConvertFrom<SrgbaPremultiplied> for EncodedSrgbF32 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for EncodedSrgbF32 {}

#[doc = include_str!("descriptions/srgba_u8.md")]
pub struct EncodedSrgbaU8;

impl Color<EncodedSrgbaU8> {
    /// Create a [`Color`] in the [`EncodedSrgbaU8`] encoding.
    ///
    /// **If you don't know what you're doing and you have RGBA values from a
    /// color picker that vary from `0-255`, use this.**
    ///
    /// If you're not sure, see [the `EncodedSrgbaU8` encoding docs][EncodedSrgbaU8] for more
    /// info.
    #[inline(always)]
    pub const fn encoded_srgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_repr([r, g, b, a])
    }

    /// Alias for backward compatibility.
    #[inline(always)]
    pub const fn srgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::encoded_srgba_u8(r, g, b, a)
    }
}

impl ColorEncoding for EncodedSrgbaU8 {
    type ComponentStruct = Rgba<u8>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = U8aRepr;

    const NAME: &'static str = "EncodedSrgbaU8";

    #[inline]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let [x, y, z, a] = repr;
        let raw_electro = Vec3::new(u8_to_f32(x), u8_to_f32(y), u8_to_f32(z));
        let optical = transform::srgb_eotf(raw_electro, WhitePoint::D65);
        let a = u8_to_f32(a);
        (optical, a)
    }

    #[inline]
    fn dst_transform_raw(raw: glam::Vec3, alpha: f32) -> Self::Repr {
        let electro = transform::srgb_oetf(raw, WhitePoint::D65);

        [
            f32_to_u8(electro.x),
            f32_to_u8(electro.y),
            f32_to_u8(electro.z),
            f32_to_u8(alpha),
        ]
    }
}

impl ConvertFrom<EncodedSrgbU8> for EncodedSrgbaU8 {}
impl ConvertFrom<EncodedSrgbF32> for EncodedSrgbaU8 {}
impl ConvertFrom<EncodedSrgbaF32> for EncodedSrgbaU8 {}
impl ConvertFrom<EncodedSrgbaPremultipliedU8> for EncodedSrgbaU8 {}
impl ConvertFrom<Srgb> for EncodedSrgbaU8 {}
impl ConvertFrom<Srgba> for EncodedSrgbaU8 {}
impl ConvertFrom<SrgbaPremultiplied> for EncodedSrgbaU8 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for EncodedSrgbaU8 {}

#[doc = include_str!("descriptions/srgba_f32.md")]
pub struct EncodedSrgbaF32;

impl Color<EncodedSrgbaF32> {
    /// Create a [`Color`] in the [`EncodedSrgbaF32`] encoding.
    ///
    /// **If you don't know what you're doing and you have RGB values from a
    /// color picker that vary from `0.0..=1.0`, use this.**
    ///
    /// If you're not sure, see [the `EncodedSrgbaF32` encoding docs][EncodedSrgbaF32] for
    /// more info.
    #[inline(always)]
    pub const fn encoded_srgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::from_repr(Vec4::new(r, g, b, a))
    }

    /// Alias for backward compatibility.
    #[inline(always)]
    pub const fn srgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::encoded_srgba_f32(r, g, b, a)
    }
}

impl ColorEncoding for EncodedSrgbaF32 {
    type ComponentStruct = Rgba<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32aRepr;

    const NAME: &'static str = "EncodedSrgbaF32";

    #[inline]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let optical = transform::srgb_eotf(repr.xyz(), WhitePoint::D65);
        (optical, repr.w)
    }

    #[inline]
    fn dst_transform_raw(raw: glam::Vec3, alpha: f32) -> Self::Repr {
        let electro = transform::srgb_oetf(raw, WhitePoint::D65);
        electro.extend(alpha)
    }
}

impl ConvertFrom<EncodedSrgbU8> for EncodedSrgbaF32 {}
impl ConvertFrom<EncodedSrgbaU8> for EncodedSrgbaF32 {}
impl ConvertFrom<EncodedSrgbF32> for EncodedSrgbaF32 {}
impl ConvertFrom<EncodedSrgbaPremultipliedU8> for EncodedSrgbaF32 {}
impl ConvertFrom<Srgb> for EncodedSrgbaF32 {}
impl ConvertFrom<Srgba> for EncodedSrgbaF32 {}
impl ConvertFrom<SrgbaPremultiplied> for EncodedSrgbaF32 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for EncodedSrgbaF32 {}

/// The fully-encoded form of the sRGB color encoding standard, with
/// *premultiplied* alpha component.
///
/// Premultiplied means that the color components are already multiplied by the
/// alpha component. Such multiplication happens *before* the sRGB OETF is
/// applied.
///
/// This is not a common way for humans to specify colors directly, but is a
/// moderately common way to encode textures before uploading them to the GPU or
/// otherwise using them in a rendering pipeline.
///
/// This color encoding is defined as the strict sRGB color encoding standard,
/// with OETF applied and encoded into 8 bits per component. The alpha component
/// is linearly encoded into 8 bits, i.e. the sRGB OETF is not applied.
pub struct EncodedSrgbaPremultipliedU8;

impl ColorEncoding for EncodedSrgbaPremultipliedU8 {
    type ComponentStruct = Rgba<u8>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = U8aRepr;

    const NAME: &'static str = "EncodedSrgbaPremultipliedU8";

    #[inline]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let [x, y, z, a] = repr;
        let raw_electro = Vec3::new(u8_to_f32(x), u8_to_f32(y), u8_to_f32(z));
        let optical = transform::srgb_eotf(raw_electro, WhitePoint::D65);
        let a = u8_to_f32(a);
        let separated = optical / a;
        (separated, a)
    }

    #[inline]
    fn dst_transform_raw(raw: glam::Vec3, alpha: f32) -> Self::Repr {
        let premultiplied = raw * alpha;
        let electro = transform::srgb_oetf(premultiplied, WhitePoint::D65);

        [
            f32_to_u8(electro.x),
            f32_to_u8(electro.y),
            f32_to_u8(electro.z),
            f32_to_u8(alpha),
        ]
    }
}

impl ConvertFrom<EncodedSrgbU8> for EncodedSrgbaPremultipliedU8 {}
impl ConvertFrom<EncodedSrgbF32> for EncodedSrgbaPremultipliedU8 {}
impl ConvertFrom<EncodedSrgbaF32> for EncodedSrgbaPremultipliedU8 {}
impl ConvertFrom<EncodedSrgbaU8> for EncodedSrgbaPremultipliedU8 {}
impl ConvertFrom<Srgb> for EncodedSrgbaPremultipliedU8 {}
impl ConvertFrom<Srgba> for EncodedSrgbaPremultipliedU8 {}
impl ConvertFrom<SrgbaPremultiplied> for EncodedSrgbaPremultipliedU8 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for EncodedSrgbaPremultipliedU8 {}

impl AlphaOver for EncodedSrgbaPremultipliedU8 {
    fn composite(over: Color<Self>, under: Color<Self>) -> Color<Self> {
        let over = over.convert::<SrgbaPremultiplied>();
        let under = under.convert::<SrgbaPremultiplied>();
        let comp = over.alpha_over(under);
        comp.convert::<Self>()
    }
}

/// The linear form of the sRGB color encoding standard.
///
/// This is a moderately rare way to specify color values.
///
/// If you have three f32s which are *not* directly related to the u8 form, or
/// you otherwise know should be "linear rgb" values, then this is the encoding
/// you have. If you instead have four values with an alpha component where the
/// alpha component varies independently of the color components, you have
/// [`Srgba`] values. If you have four values with an alpha component and
/// the rgb components are modified directly when the alpha component changes as
/// well, you have [`SrgbaPremultiplied`] values.
pub struct Srgb;

impl Color<Srgb> {
    /// Create a [`Color`] in the [`Srgb`] encoding.
    ///
    /// If you're not sure, you should probably use [`Color::encoded_srgb_f32`] instead.
    /// See [the `Srgb` encoding docs][Srgb] for more info.
    #[inline(always)]
    pub fn srgb(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }

    /// Alias for backward compatibility.
    #[inline(always)]
    pub fn linear_srgb(r: f32, g: f32, b: f32) -> Self {
        Self::srgb(r, g, b)
    }
}

impl ColorEncoding for Srgb {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32Repr;

    const NAME: &'static str = "Srgb";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        raw
    }
}

impl ConvertFrom<EncodedSrgbU8> for Srgb {}
impl ConvertFrom<EncodedSrgbF32> for Srgb {}
impl ConvertFrom<EncodedSrgbaU8> for Srgb {}
impl ConvertFrom<EncodedSrgbaF32> for Srgb {}
impl ConvertFrom<EncodedSrgbaPremultipliedU8> for Srgb {}
impl ConvertFrom<Srgba> for Srgb {}
impl ConvertFrom<SrgbaPremultiplied> for Srgb {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for Srgb {}

impl WorkingEncoding for Srgb {}

/// The linear form of the sRGB color encoding standard with a separate alpha
/// component.
///
/// This is a moderately common way to specify color values.
///
/// If you have four f32s which are *not* directly related to the u8 form, or
/// you otherwise know should be "linear rgb" values, and the alpha component
/// varies independently of the color componewnts, then this is the encoding you
/// have. If you instead have three values, you have [`Srgb`] values.
/// If you have four values with an alpha component and the rgb components are
/// modified directly when the alpha component changes as well, you have
/// [`SrgbaPremultiplied`] values.
pub struct Srgba;

impl Color<Srgba> {
    /// Create a [`Color`] in the [`Srgba`] encoding.
    ///
    /// If you're not sure, you should probably use [`Color::encoded_srgba_f32`]
    /// instead. See [the `Srgba` encoding docs][Srgba] for more
    /// info.
    #[inline(always)]
    pub fn srgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::from_repr(Vec4::new(r, g, b, a))
    }

    /// Alias for backward compatibility.
    #[inline(always)]
    pub fn linear_srgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::srgba(r, g, b, a)
    }
}

impl ColorEncoding for Srgba {
    type ComponentStruct = Rgba<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32aRepr;

    const NAME: &'static str = "Srgba";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr.xyz(), repr.w)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, alpha: f32) -> Self::Repr {
        raw.extend(alpha)
    }
}

impl ConvertFrom<EncodedSrgbU8> for Srgba {}
impl ConvertFrom<EncodedSrgbF32> for Srgba {}
impl ConvertFrom<EncodedSrgbaU8> for Srgba {}
impl ConvertFrom<EncodedSrgbaF32> for Srgba {}
impl ConvertFrom<EncodedSrgbaPremultipliedU8> for Srgba {}
impl ConvertFrom<Srgb> for Srgba {}
impl ConvertFrom<SrgbaPremultiplied> for Srgba {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for Srgba {}

impl WorkingEncoding for Srgba {}

impl AlphaOver for Srgba {
    fn composite(over: Color<Self>, under: Color<Self>) -> Color<Self> {
        let over = over.convert::<SrgbaPremultiplied>();
        let under = under.convert::<SrgbaPremultiplied>();
        let comp = over.alpha_over(under);
        comp.convert::<Self>()
    }
}

/// The linear form of the sRGB color encoding standard with a *premultiplied*
/// alpha component.
///
/// "Premultiplied" alpha means that the value of the color components has been
/// multiplied by the alpha component. This operation is unintuitive when
/// specifying color values, but it is the "most correct" way to store color
/// values with an alpha component when performing operations like blending and
/// compositing on them.
///
/// This is a relatively rare way to specify color values.
///
/// If you have four f32s which are *not* directly related to the u8 form, or
/// you otherwise know should be "linear rgb" values, and the alpha component
/// varies independently of the color componewnts, then this is the encoding you
/// have. If you instead have three values, you have [`Srgb`] values.
/// If you have four values with an alpha component and the rgb components are
/// modified directly when the alpha component changes as well, you have
/// [`SrgbaPremultiplied`] values.
pub struct SrgbaPremultiplied;

impl Color<SrgbaPremultiplied> {
    /// Create a [`Color`] in the [`SrgbaPremultiplied`] encoding.
    ///
    /// "Premultiplied" alpha means that the value of the color components has
    /// been multiplied by the alpha component. This operation is
    /// unintuitive when specifying color values, but it is the
    /// "most correct" way to store color values with an alpha component when
    /// performing operations like blending and compositing on them.
    ///
    /// If you're not sure, see [the `Srgba` encoding docs][Srgba]
    /// for more info.
    #[inline(always)]
    pub fn srgba_premultiplied(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::from_repr(Vec4::new(r, g, b, a))
    }

    /// Alias for backward compatibility.
    #[inline(always)]
    pub fn linear_srgba_premultiplied(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::srgba_premultiplied(r, g, b, a)
    }
}

impl ColorEncoding for SrgbaPremultiplied {
    type ComponentStruct = Rgba<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32aRepr;

    const NAME: &'static str = "SrgbaPremultiplied";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let separated = repr.xyz() / repr.w;
        (separated, repr.w)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, alpha: f32) -> Self::Repr {
        let premultiplied = raw * alpha;
        premultiplied.extend(alpha)
    }
}

impl ConvertFrom<EncodedSrgbU8> for SrgbaPremultiplied {}
impl ConvertFrom<EncodedSrgbF32> for SrgbaPremultiplied {}
impl ConvertFrom<EncodedSrgbaU8> for SrgbaPremultiplied {}
impl ConvertFrom<EncodedSrgbaF32> for SrgbaPremultiplied {}
impl ConvertFrom<EncodedSrgbaPremultipliedU8> for SrgbaPremultiplied {}
impl ConvertFrom<Srgba> for SrgbaPremultiplied {}
impl ConvertFrom<Srgb> for SrgbaPremultiplied {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for SrgbaPremultiplied {}

impl AlphaOver for SrgbaPremultiplied {
    #[inline]
    fn composite(over: Color<Self>, under: Color<Self>) -> Color<Self> {
        Color::from_repr(over.repr + under.repr * (1.0 - over.repr.w))
    }
}

/// A 32-bit-per-component version of the Oklab perceptually-uniform color
/// space.
pub struct Oklab;

impl Color<Oklab> {
    /// Create a [`Color`] in the [`Oklab`] color encoding.
    ///
    /// This is fairly rare, it would be more common to specify colors in
    /// another color encoding like [`EncodedSrgbU8`] and then convert them to
    /// [`Oklab`] to blend them together.
    #[inline(always)]
    pub fn oklab(l: f32, a: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(l, a, b))
    }
}

impl ColorEncoding for Oklab {
    type ComponentStruct = Lab<f32>;
    type LinearSpace = linear_spaces::CieXYZ;
    type Repr = F32Repr;

    const NAME: &'static str = "Oklab";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let xyz = transform::ok_lab_to_xyz(repr, WhitePoint::D65);
        (xyz, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        transform::xyz_to_ok_lab(raw, WhitePoint::D65)
    }
}

impl ConvertFrom<EncodedSrgbU8> for Oklab {}
impl ConvertFrom<EncodedSrgbF32> for Oklab {}
impl ConvertFrom<EncodedSrgbaU8> for Oklab {}
impl ConvertFrom<EncodedSrgbaF32> for Oklab {}
impl ConvertFrom<EncodedSrgbaPremultipliedU8> for Oklab {}
impl ConvertFrom<Srgb> for Oklab {}
impl ConvertFrom<Srgba> for Oklab {}
impl ConvertFrom<SrgbaPremultiplied> for Oklab {}

impl WorkingEncoding for Oklab {}
impl PerceptualEncoding for Oklab {}

// Transform functions for Adobe RGB and ProPhoto RGB

/// Adobe RGB OETF (gamma encoding).
/// Uses a simple gamma of 563/256 (2.19921875).
#[inline(always)]
fn adobe_rgb_oetf(color: Vec3) -> Vec3 {
    const GAMMA: f32 = 256.0 / 563.0; // 1/2.19921875
    color.powf(GAMMA)
}

/// Adobe RGB EOTF (gamma decoding).
/// Inverse of adobe_rgb_oetf.
#[inline(always)]
fn adobe_rgb_eotf(color: Vec3) -> Vec3 {
    const GAMMA: f32 = 563.0 / 256.0; // 2.19921875
    color.powf(GAMMA)
}

/// ProPhoto RGB OETF (gamma encoding).
/// Similar to sRGB but with different parameters.
#[inline(always)]
fn prophoto_rgb_oetf(color: Vec3) -> Vec3 {
    const ET: f32 = 1.0 / 512.0; // Threshold
    let lower = color * 16.0;
    let higher = color.powf(1.0 / 1.8);
    Vec3::select(color.cmple(Vec3::splat(ET)), lower, higher)
}

/// ProPhoto RGB EOTF (gamma decoding).
/// Inverse of prophoto_rgb_oetf.
#[inline(always)]
fn prophoto_rgb_eotf(color: Vec3) -> Vec3 {
    const ET2: f32 = 16.0 / 512.0; // Encoded threshold
    let lower = color / 16.0;
    let higher = color.powf(1.8);
    Vec3::select(color.cmple(Vec3::splat(ET2)), lower, higher)
}

/// Linear Adobe RGB color space.
pub struct AdobeRgb;

impl Color<AdobeRgb> {
    /// Create a [`Color`] in the [`AdobeRgb`] linear color space.
    #[inline(always)]
    pub fn adobe_rgb(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }

    /// Backward compatibility alias for adobe_rgb.
    #[inline(always)]
    pub fn linear_adobe_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::adobe_rgb(r, g, b)
    }
}

impl ColorEncoding for AdobeRgb {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::AdobeRgb;
    type Repr = F32Repr;

    const NAME: &'static str = "AdobeRgb";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        raw
    }
}

impl WorkingEncoding for AdobeRgb {}

/// Linear ProPhoto RGB color space.
pub struct ProPhotoRgb;

impl Color<ProPhotoRgb> {
    /// Create a [`Color`] in the [`ProPhotoRgb`] linear color space.
    #[inline(always)]
    pub fn prophoto_rgb(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }

    /// Backward compatibility alias for prophoto_rgb.
    #[inline(always)]
    pub fn linear_prophoto_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::prophoto_rgb(r, g, b)
    }
}

impl ColorEncoding for ProPhotoRgb {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::ProPhotoRgb;
    type Repr = F32Repr;

    const NAME: &'static str = "ProPhotoRgb";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        raw
    }
}

impl WorkingEncoding for ProPhotoRgb {}

/// Non-linear Adobe RGB U8 encoding.
pub struct EncodedAdobeRgbU8;

impl Color<EncodedAdobeRgbU8> {
    /// Create a [`Color`] in the [`EncodedAdobeRgbU8`] encoding.
    #[inline(always)]
    pub fn encoded_adobe_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Color::from_repr([r, g, b])
    }

    /// Backward compatibility alias for encoded_adobe_rgb_u8.
    #[inline(always)]
    pub fn adobe_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::encoded_adobe_rgb_u8(r, g, b)
    }
}

impl ColorEncoding for EncodedAdobeRgbU8 {
    type ComponentStruct = Rgb<u8>;
    type LinearSpace = linear_spaces::AdobeRgb;
    type Repr = U8Repr;

    const NAME: &'static str = "EncodedAdobeRgbU8";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let [x, y, z] = repr;
        let raw_electro = Vec3::new(u8_to_f32(x), u8_to_f32(y), u8_to_f32(z));
        let optical = adobe_rgb_eotf(raw_electro);
        (optical, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        let electro = adobe_rgb_oetf(raw);
        [
            f32_to_u8(electro.x),
            f32_to_u8(electro.y),
            f32_to_u8(electro.z),
        ]
    }
}

/// Non-linear ProPhoto RGB U8 encoding.
pub struct EncodedProPhotoRgbU8;

impl Color<EncodedProPhotoRgbU8> {
    /// Create a [`Color`] in the [`EncodedProPhotoRgbU8`] encoding.
    #[inline(always)]
    pub fn encoded_prophoto_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Color::from_repr([r, g, b])
    }

    /// Backward compatibility alias for encoded_prophoto_rgb_u8.
    #[inline(always)]
    pub fn prophoto_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::encoded_prophoto_rgb_u8(r, g, b)
    }
}

impl ColorEncoding for EncodedProPhotoRgbU8 {
    type ComponentStruct = Rgb<u8>;
    type LinearSpace = linear_spaces::ProPhotoRgb;
    type Repr = U8Repr;

    const NAME: &'static str = "EncodedProPhotoRgbU8";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        let [x, y, z] = repr;
        let raw_electro = Vec3::new(u8_to_f32(x), u8_to_f32(y), u8_to_f32(z));
        let optical = prophoto_rgb_eotf(raw_electro);
        (optical, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        let electro = prophoto_rgb_oetf(raw);
        [
            f32_to_u8(electro.x),
            f32_to_u8(electro.y),
            f32_to_u8(electro.z),
        ]
    }
}
/// Linear Display P3 color space.
pub struct DisplayP3;

impl Color<DisplayP3> {
    /// Create a [`Color`] in the [`DisplayP3`] linear color space.
    #[inline(always)]
    pub fn display_p3(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }
}

impl ColorEncoding for DisplayP3 {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::DisplayP3;
    type Repr = F32Repr;

    const NAME: &'static str = "DisplayP3";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        raw
    }
}

impl WorkingEncoding for DisplayP3 {}

/// Linear ACEScg color space.
pub struct AcesCg;

impl Color<AcesCg> {
    /// Create a [`Color`] in the [`AcesCg`] linear color space.
    #[inline(always)]
    pub fn aces_cg(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }
}

impl ColorEncoding for AcesCg {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::AcesCg;
    type Repr = F32Repr;

    const NAME: &'static str = "AcesCg";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        raw
    }
}

impl WorkingEncoding for AcesCg {}

/// Linear ACES 2065 color space.
pub struct Aces2065;

impl Color<Aces2065> {
    /// Create a [`Color`] in the [`Aces2065`] linear color space.
    #[inline(always)]
    pub fn aces_2065(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }
}

impl ColorEncoding for Aces2065 {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::Aces2065;
    type Repr = F32Repr;

    const NAME: &'static str = "Aces2065";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        raw
    }
}

impl WorkingEncoding for Aces2065 {}

/// Linear BT.2020 color space.
pub struct Bt2020;

impl Color<Bt2020> {
    /// Create a [`Color`] in the [`Bt2020`] linear color space.
    #[inline(always)]
    pub fn bt2020(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }
}

impl ColorEncoding for Bt2020 {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::Bt2020;
    type Repr = F32Repr;

    const NAME: &'static str = "Bt2020";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        raw
    }
}

impl WorkingEncoding for Bt2020 {}
