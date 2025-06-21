//! An opinionated color management library for games and graphics.
//!
//! # Introduction
//!
//! Color is a really complex and fascinating topic, and I'd love to take you on
//! a little tour and show you how `colstodian` tries to help make it easier to
//! manage. But, if you really just want to get sh\*t done right now, here's the
//! basics:
//!
//! [`Color`] is a unified type representing a color in any [`ColorEncoding`].
//! The [`ColorEncoding`] defines a bunch of different properties about how the
//! color values are stored and what those values actually mean. For example,
//! [`Color<SrgbU8>`] is a color with red, green, and blue values that vary from
//! `0-255` and the meaning of those values is defined by the full sRGB color
//! encoding standard. The most common and standard color encodings are exposed
//! in the [`basic_encodings`] module.
//!
//! Many built-in color encodings expose constructor functions on the [`Color`]
//! type. See the docs for that type for a full list. The ones you are likely most interested in if you don't know much about color are:
//!
//! * [`Color::srgb_u8`]: If you have three `0-255` values, this is what you
//!   want.
//!
//! * [`Color::srgb_f32`]: If you have three `0.0..=1.0` values, this is
//!   probably what you want.
//!
//! * [`Color::linear_srgb`]: If you have three `0.0..=1.0` values that you know
//!   are "linear rgb", this is probably what you want.
//!
//! If you also have alpha (i.e. four values instead of three), then
//! [`Color::srgba_u8`], [`Color::srgba_f32`], and [`Color::linear_srgba`]
//! are the equivalents of the above with an alpha component.
//!
//! If you want to do math to a color (for example, adding two colors together
//! or multiplying one by a coefficient), you'll want to do so in a color
//! encoding that is conducive to that. Color encodings which have this property
//! implement the [`WorkingEncoding`][details::traits::WorkingEncoding] trait.
//! If a [`Color`] is not encoded in a working encoding, it will not implement
//! common math traits like addition, multiplication, etc.
//!
//! The most common [`WorkingEncoding`] is [`LinearSrgb`]. You can convert a
//! color you have created using any of the constructors above to [`LinearSrgb`]
//! by using the [`.convert::<E>()`][Color::convert] method.
//!
//! If you want to output a color into an image file, the most common color
//! encoding for most common image formats (and the one assumed by viewing
//! programs if a color profile is not embedded) is [`SrgbU8`]. You can convert
//! a color from a working encoding to [`SrgbU8`] for output again with the
//! [`.convert::<E>()`][Color::convert] method.
//!
//! ## Example
//!
//! Here we construct two colors in different ways, convert them both to
//! [`LinearSrgb`] to work with them, and then convert the result to [`SrgbU8`]
//! which can be passed on to be displayed in an image.
//!
//! ```
//! use colstodian::basic_encodings::{LinearSrgb, SrgbU8};
//! use colstodian::Color;
//!
//! let color1 = Color::srgb_u8(102, 54, 220);
//! let color2 = Color::srgb_f32(0.5, 0.8, 0.1);
//!
//! let color1_working = color1.convert::<LinearSrgb>();
//! let color2_working = color2.convert::<LinearSrgb>();
//!
//! let result_working = color1_working * 0.5 + color2_working;
//!
//! let output = result_working.convert::<SrgbU8>();
//!
//! assert_eq!(output, Color::srgb_u8(144, 207, 163));
//! ```
//!
//! ## Color Encoding Basics
//!
//! Much like how a 3d vector like a `glam::Vec3` could be used to describe any
//! of:
//!
//! * The motion vector of an object in meters per second.
//!
//! * The position of an object relative to a reference point in kilometers.
//!
//! * Three "wellness scores" for a character, which each axis representing how
//!   happy the character is about some aspect of their life.
//!
//! A bag of components that describes "a color" could actually be interpreted
//! in many different ways, and the end result of what those components mean is
//! very different.
//!
//! `colstodian` gathers all the information that defines how a color is
//! represented in data as well as what that data actually means into
//! representative types that implement the [`ColorEncoding`] trait.
//!
//! [`LinearSrgb`]: details::encodings::LinearSrgb
//! [`SrgbU8`]: details::encodings::SrgbU8
//!
//! ## Features
#![doc = document_features::document_features!()]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    //clippy::let_and_return, // it makes conversion code more explicit with naming
    unexpected_cfgs,
)]

/// Contains advanced usage details of the crate.
pub mod details {
    pub mod component_structs;

    /// Types representing different [`ColorEncoding`][traits::ColorEncoding]s.
    pub mod encodings;

    /// Contains the [`Color`][color::Color] type and helper functions.
    pub mod color;

    /// Types representing different
    /// [`LinearColorSpace`][traits::LinearColorSpace]s.
    #[rustfmt::skip]
    pub mod linear_spaces;

    /// The traits which form the backbone of this crate.
    pub mod traits;

    /// The underlying data representations ([`ColorRepr`][traits::ColorRepr]s)
    /// used by different [`ColorEncoding`][traits::ColorEncoding]s.
    pub mod reprs;
}

pub(crate) use details::*;

/// Contains a basic set of [`ColorEncoding`]s to get most people going.
///
/// These are all re-exported from inside the [`details::encodings`]
pub mod basic_encodings {
    #[doc(inline)]
    pub use crate::details::encodings::LinearSrgb;
    #[doc(inline)]
    pub use crate::details::encodings::LinearSrgba;
    #[doc(inline)]
    pub use crate::details::encodings::SrgbU8;
    #[doc(inline)]
    pub use crate::details::encodings::SrgbaU8;
}

#[doc(inline)]
pub use color::Color;

#[doc(inline)]
pub use traits::ColorEncoding;

#[doc(inline)]
pub use traits::WorkingEncoding;

#[cfg(any(test, feature = "approx"))]
mod approx_impls;

#[doc(inline)]
pub use traits::PerceptualEncoding;

/// Like [`Into`] but specialized for use with `colstodian` [`Color`] types.
///
/// This trait exists so that functions can accept colors in a variety of
/// encodings generically in an ergonomic fashion. [`ColorInto`] is blanket
/// implemented generically so that, if you have a function parameter `impl
/// ColorInto<Color<SomeEncoding>>`, a [`Color`] in any other encoding that is
/// able to [`.convert::<SomeEncoding>()`][Color::convert] can be passed into
/// that function as argument directly.
///
/// See [the docs of the `convert` method on `Color`][Color::convert] for more.
///
/// # Example
///
/// ```
/// # use colstodian::*;
/// # use colstodian::details::encodings::*;
/// # use approx::assert_relative_eq;
/// type MyColor = Color<LinearSrgb>;
///
/// fn test_fn(input: impl ColorInto<MyColor>) {
///     let input: MyColor = input.color_into();
///     let correct = Color::linear_srgb(0.14703, 0.42327, 0.22323);
///     assert_relative_eq!(input, correct, epsilon = 0.00001);
/// }
///
/// test_fn(Color::srgb_u8(107, 174, 130));
/// test_fn(Color::srgb_f32(0.41961, 0.68235, 0.5098));
/// ```
pub trait ColorInto<DstCol> {
    fn color_into(self) -> DstCol;
}

use details::traits::ConvertFrom;
use details::traits::LinearConvertFromRaw;

impl<SrcEnc, DstEnc> ColorInto<Color<DstEnc>> for Color<SrcEnc>
where
    SrcEnc: ColorEncoding,
    DstEnc: ColorEncoding + ConvertFrom<SrcEnc>,
    DstEnc::LinearSpace: LinearConvertFromRaw<SrcEnc::LinearSpace>,
{
    #[inline(always)]
    fn color_into(self) -> Color<DstEnc> {
        self.convert()
    }
}

