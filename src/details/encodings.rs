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
pub struct SrgbU8;

impl Color<SrgbU8> {
    /// Create a [`Color`] in the [`SrgbU8`] encoding.
    ///
    /// **If you don't know what you're doing and you have RGB values from a
    /// color picker that vary from `0-255`, use this.**
    ///
    /// If you're not sure, see [the `SrgbU8` encoding docs][SrgbU8] for more
    /// info.
    #[inline(always)]
    pub const fn srgb_u8(r: u8, g: u8, b: u8) -> Self {
        Color::from_repr([r, g, b])
    }
}

impl ColorEncoding for SrgbU8 {
    type ComponentStruct = Rgb<u8>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = U8Repr;

    const NAME: &'static str = "SrgbU8";

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

impl ConvertFrom<SrgbF32> for SrgbU8 {}
impl ConvertFrom<SrgbaU8> for SrgbU8 {}
impl ConvertFrom<SrgbaF32> for SrgbU8 {}
impl ConvertFrom<SrgbaPremultipliedU8> for SrgbU8 {}
impl ConvertFrom<LinearSrgb> for SrgbU8 {}
impl ConvertFrom<LinearSrgba> for SrgbU8 {}
impl ConvertFrom<LinearSrgbaPremultiplied> for SrgbU8 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for SrgbU8 {}

#[doc = include_str!("descriptions/srgb_f32.md")]
pub struct SrgbF32;

impl Color<SrgbF32> {
    /// Create a [`Color`] in the [`SrgbF32`] encoding.
    ///
    /// **If you don't know what you're doing and you have RGB values from a
    /// color picker that vary from `0.0..=1.0`, use this.**
    ///
    /// If you're not sure, see [the `SrgbF32` encoding docs][SrgbF32] for more
    /// info.
    #[inline(always)]
    pub const fn srgb_f32(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }
}

impl ColorEncoding for SrgbF32 {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32Repr;

    const NAME: &'static str = "SrgbF32";

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

impl ConvertFrom<SrgbU8> for SrgbF32 {}
impl ConvertFrom<SrgbaU8> for SrgbF32 {}
impl ConvertFrom<SrgbaF32> for SrgbF32 {}
impl ConvertFrom<SrgbaPremultipliedU8> for SrgbF32 {}
impl ConvertFrom<LinearSrgb> for SrgbF32 {}
impl ConvertFrom<LinearSrgba> for SrgbF32 {}
impl ConvertFrom<LinearSrgbaPremultiplied> for SrgbF32 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for SrgbF32 {}

#[doc = include_str!("descriptions/srgba_u8.md")]
pub struct SrgbaU8;

impl Color<SrgbaU8> {
    /// Create a [`Color`] in the [`SrgbaU8`] encoding.
    ///
    /// **If you don't know what you're doing and you have RGBA values from a
    /// color picker that vary from `0-255`, use this.**
    ///
    /// If you're not sure, see [the `SrgbaU8` encoding docs][SrgbaU8] for more
    /// info.
    #[inline(always)]
    pub const fn srgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_repr([r, g, b, a])
    }
}

impl ColorEncoding for SrgbaU8 {
    type ComponentStruct = Rgba<u8>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = U8aRepr;

    const NAME: &'static str = "SrgbaU8";

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

impl ConvertFrom<SrgbU8> for SrgbaU8 {}
impl ConvertFrom<SrgbF32> for SrgbaU8 {}
impl ConvertFrom<SrgbaF32> for SrgbaU8 {}
impl ConvertFrom<SrgbaPremultipliedU8> for SrgbaU8 {}
impl ConvertFrom<LinearSrgb> for SrgbaU8 {}
impl ConvertFrom<LinearSrgba> for SrgbaU8 {}
impl ConvertFrom<LinearSrgbaPremultiplied> for SrgbaU8 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for SrgbaU8 {}

#[doc = include_str!("descriptions/srgba_f32.md")]
pub struct SrgbaF32;

impl Color<SrgbaF32> {
    /// Create a [`Color`] in the [`SrgbaF32`] encoding.
    ///
    /// **If you don't know what you're doing and you have RGB values from a
    /// color picker that vary from `0.0..=1.0`, use this.**
    ///
    /// If you're not sure, see [the `SrgbaF32` encoding docs][SrgbaF32] for
    /// more info.
    #[inline(always)]
    pub const fn srgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::from_repr(Vec4::new(r, g, b, a))
    }
}

impl ColorEncoding for SrgbaF32 {
    type ComponentStruct = Rgba<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32aRepr;

    const NAME: &'static str = "SrgbaF32";

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

impl ConvertFrom<SrgbU8> for SrgbaF32 {}
impl ConvertFrom<SrgbaU8> for SrgbaF32 {}
impl ConvertFrom<SrgbF32> for SrgbaF32 {}
impl ConvertFrom<SrgbaPremultipliedU8> for SrgbaF32 {}
impl ConvertFrom<LinearSrgb> for SrgbaF32 {}
impl ConvertFrom<LinearSrgba> for SrgbaF32 {}
impl ConvertFrom<LinearSrgbaPremultiplied> for SrgbaF32 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for SrgbaF32 {}

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
pub struct SrgbaPremultipliedU8;

impl ColorEncoding for SrgbaPremultipliedU8 {
    type ComponentStruct = Rgba<u8>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = U8aRepr;

    const NAME: &'static str = "SrgbaPremultipliedU8";

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

impl ConvertFrom<SrgbU8> for SrgbaPremultipliedU8 {}
impl ConvertFrom<SrgbF32> for SrgbaPremultipliedU8 {}
impl ConvertFrom<SrgbaF32> for SrgbaPremultipliedU8 {}
impl ConvertFrom<SrgbaU8> for SrgbaPremultipliedU8 {}
impl ConvertFrom<LinearSrgb> for SrgbaPremultipliedU8 {}
impl ConvertFrom<LinearSrgba> for SrgbaPremultipliedU8 {}
impl ConvertFrom<LinearSrgbaPremultiplied> for SrgbaPremultipliedU8 {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for SrgbaPremultipliedU8 {}

impl AlphaOver for SrgbaPremultipliedU8 {
    fn composite(over: Color<Self>, under: Color<Self>) -> Color<Self> {
        let over = over.convert::<LinearSrgbaPremultiplied>();
        let under = under.convert::<LinearSrgbaPremultiplied>();
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
/// [`LinearSrgba`] values. If you have four values with an alpha component and
/// the rgb components are modified directly when the alpha component changes as
/// well, you have [`LinearSrgbaPremultiplied`] values.
pub struct LinearSrgb;

impl Color<LinearSrgb> {
    /// Create a [`Color`] in the [`LinearSrgb`] encoding.
    ///
    /// If you're not sure, you should probably use [`Color::srgb_f32`] instead.
    /// See [the `LinearSrgb` encoding docs][LinearSrgb] for more info.
    #[inline(always)]
    pub fn linear_srgb(r: f32, g: f32, b: f32) -> Self {
        Color::from_repr(Vec3::new(r, g, b))
    }
}

impl ColorEncoding for LinearSrgb {
    type ComponentStruct = Rgb<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32Repr;

    const NAME: &'static str = "LinearSrgb";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr, 1.0)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, _: f32) -> Self::Repr {
        raw
    }
}

impl ConvertFrom<SrgbU8> for LinearSrgb {}
impl ConvertFrom<SrgbF32> for LinearSrgb {}
impl ConvertFrom<SrgbaU8> for LinearSrgb {}
impl ConvertFrom<SrgbaF32> for LinearSrgb {}
impl ConvertFrom<SrgbaPremultipliedU8> for LinearSrgb {}
impl ConvertFrom<LinearSrgba> for LinearSrgb {}
impl ConvertFrom<LinearSrgbaPremultiplied> for LinearSrgb {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for LinearSrgb {}

impl WorkingEncoding for LinearSrgb {}

/// The linear form of the sRGB color encoding standard with a separate alpha
/// component.
///
/// This is a moderately common way to specify color values.
///
/// If you have four f32s which are *not* directly related to the u8 form, or
/// you otherwise know should be "linear rgb" values, and the alpha component
/// varies independently of the color componewnts, then this is the encoding you
/// have. If you instead have three values, you have [`LinearSrgb`] values.
/// If you have four values with an alpha component and the rgb components are
/// modified directly when the alpha component changes as well, you have
/// [`LinearSrgbaPremultiplied`] values.
pub struct LinearSrgba;

impl Color<LinearSrgba> {
    /// Create a [`Color`] in the [`LinearSrgba`] encoding.
    ///
    /// If you're not sure, you should probably use [`Color::srgba_f32`]
    /// instead. See [the `LinearSrgba` encoding docs][LinearSrgba] for more
    /// info.
    #[inline(always)]
    pub fn linear_srgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::from_repr(Vec4::new(r, g, b, a))
    }
}

impl ColorEncoding for LinearSrgba {
    type ComponentStruct = Rgba<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32aRepr;

    const NAME: &'static str = "LinearSrgba";

    #[inline(always)]
    fn src_transform_raw(repr: Self::Repr) -> (glam::Vec3, f32) {
        (repr.xyz(), repr.w)
    }

    #[inline(always)]
    fn dst_transform_raw(raw: glam::Vec3, alpha: f32) -> Self::Repr {
        raw.extend(alpha)
    }
}

impl ConvertFrom<SrgbU8> for LinearSrgba {}
impl ConvertFrom<SrgbF32> for LinearSrgba {}
impl ConvertFrom<SrgbaU8> for LinearSrgba {}
impl ConvertFrom<SrgbaF32> for LinearSrgba {}
impl ConvertFrom<SrgbaPremultipliedU8> for LinearSrgba {}
impl ConvertFrom<LinearSrgb> for LinearSrgba {}
impl ConvertFrom<LinearSrgbaPremultiplied> for LinearSrgba {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for LinearSrgba {}

impl WorkingEncoding for LinearSrgba {}

impl AlphaOver for LinearSrgba {
    fn composite(over: Color<Self>, under: Color<Self>) -> Color<Self> {
        let over = over.convert::<LinearSrgbaPremultiplied>();
        let under = under.convert::<LinearSrgbaPremultiplied>();
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
/// have. If you instead have three values, you have [`LinearSrgb`] values.
/// If you have four values with an alpha component and the rgb components are
/// modified directly when the alpha component changes as well, you have
/// [`LinearSrgbaPremultiplied`] values.
pub struct LinearSrgbaPremultiplied;

impl Color<LinearSrgbaPremultiplied> {
    /// Create a [`Color`] in the [`LinearSrgbaPremultiplied`] encoding.
    ///
    /// "Premultiplied" alpha means that the value of the color components has
    /// been multiplied by the alpha component. This operation is
    /// unintuitive when specifying color values, but it is the
    /// "most correct" way to store color values with an alpha component when
    /// performing operations like blending and compositing on them.
    ///
    /// If you're not sure, see [the `LinearSrgba` encoding docs][LinearSrgba]
    /// for more info.
    #[inline(always)]
    pub fn linear_srgba_premultiplied(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::from_repr(Vec4::new(r, g, b, a))
    }
}

impl ColorEncoding for LinearSrgbaPremultiplied {
    type ComponentStruct = Rgba<f32>;
    type LinearSpace = linear_spaces::Srgb;
    type Repr = F32aRepr;

    const NAME: &'static str = "LinearSrgbaPremultiplied";

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

impl ConvertFrom<SrgbU8> for LinearSrgbaPremultiplied {}
impl ConvertFrom<SrgbF32> for LinearSrgbaPremultiplied {}
impl ConvertFrom<SrgbaU8> for LinearSrgbaPremultiplied {}
impl ConvertFrom<SrgbaF32> for LinearSrgbaPremultiplied {}
impl ConvertFrom<SrgbaPremultipliedU8> for LinearSrgbaPremultiplied {}
impl ConvertFrom<LinearSrgba> for LinearSrgbaPremultiplied {}
impl ConvertFrom<LinearSrgb> for LinearSrgbaPremultiplied {}
// TODO: oklab gamut clipping
impl ConvertFrom<Oklab> for LinearSrgbaPremultiplied {}

impl AlphaOver for LinearSrgbaPremultiplied {
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
    /// another color encoding like [`SrgbU8`] and then convert them to
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

impl ConvertFrom<SrgbU8> for Oklab {}
impl ConvertFrom<SrgbF32> for Oklab {}
impl ConvertFrom<SrgbaU8> for Oklab {}
impl ConvertFrom<SrgbaF32> for Oklab {}
impl ConvertFrom<SrgbaPremultipliedU8> for Oklab {}
impl ConvertFrom<LinearSrgb> for Oklab {}
impl ConvertFrom<LinearSrgba> for Oklab {}
impl ConvertFrom<LinearSrgbaPremultiplied> for Oklab {}

impl WorkingEncoding for Oklab {}
impl PerceptualEncoding for Oklab {}
