//! Structs that act as bags of named components which [`Color`][crate::Color]s
//! of different color encodings may be `Deref`erenced to in order to gain more
//! appropriate dot syntax for that encoding.
//!
//! For example, a [`Color`][crate::Color] in the
//! [`SrgbU8`][crate::encodings::SrgbU8] color space can be `Deref`'d to
//! [`Rgb`], allowing you to do things like `color.r` or `color.g`.

use core::fmt;

use crate::reprs::*;
use crate::traits::ComponentStructFor;

// #[cfg(feature = "bytemuck")]
// macro_rules! impl_bytemuck {
//     ($($inner:ident),+) => {
//         $(
//             unsafe impl bytemuck::Zeroable for $inner {}
//             unsafe impl bytemuck::Pod for $inner {}

//             unsafe impl bytemuck::Zeroable for ColAlpha<$inner> {}
//             unsafe impl bytemuck::Pod for ColAlpha<$inner> {}
//         )+
//     }
// }

// #[cfg(feature = "bytemuck")]
// impl_bytemuck!(Rgb, ICtCp, Xyz, Lab, LCh);

/// A bag of components with names R, G, B. Some `Color`s with RGB color
/// encodings will `Deref`/`DerefMut` to this struct so that you can access
/// their components with dot-syntax.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

unsafe impl ComponentStructFor<U8Repr> for Rgb<u8> {
    fn cast(repr: &U8Repr) -> &Self {
        // SAFETY: [u8; 3] is guaranteed to have the same layout as Self
        unsafe { &*(repr as *const U8Repr as *const Self) }
    }

    fn cast_mut(repr: &mut U8Repr) -> &mut Self {
        // SAFETY: [u8; 3] is guaranteed to have the same layout as Self
        unsafe { &mut *(repr as *mut U8Repr as *mut Self) }
    }
}

unsafe impl ComponentStructFor<F32Repr> for Rgb<f32> {
    fn cast(repr: &F32Repr) -> &Self {
        // SAFETY: Vec3 is guaranteed to have the same layout as Self
        unsafe { &*(repr as *const F32Repr as *const Self) }
    }

    fn cast_mut(repr: &mut F32Repr) -> &mut Self {
        // SAFETY: Vec3 is guaranteed to have the same layout as Self
        unsafe { &mut *(repr as *mut F32Repr as *mut Self) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<T: fmt::Display> fmt::Display for Rgb<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R: {:.3}, G: {:.3}, B: {:.3}", self.r, self.g, self.b)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<T: fmt::Display> fmt::Debug for Rgb<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R: {}, G: {}, B: {}", self.r, self.g, self.b)
    }
}

/// A bag of components with names R, G, B, A. Some `Color`s with RGBA color
/// encodings will `Deref`/`DerefMut` to this struct so that you can access
/// their components with dot-syntax.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

unsafe impl ComponentStructFor<U8aRepr> for Rgba<u8> {
    fn cast(repr: &U8aRepr) -> &Self {
        // SAFETY: [u8; 4] is guaranteed to have the same layout as Self
        unsafe { &*(repr as *const U8aRepr as *const Self) }
    }

    fn cast_mut(repr: &mut U8aRepr) -> &mut Self {
        // SAFETY: [u8; 4] is guaranteed to have the same layout as Self
        unsafe { &mut *(repr as *mut U8aRepr as *mut Self) }
    }
}

unsafe impl ComponentStructFor<F32aRepr> for Rgba<f32> {
    fn cast(repr: &F32aRepr) -> &Self {
        // SAFETY: Vec4 is guaranteed to have the same layout as Self
        unsafe { &*(repr as *const F32aRepr as *const Self) }
    }

    fn cast_mut(repr: &mut F32aRepr) -> &mut Self {
        // SAFETY: Vec4 is guaranteed to have the same layout as Self
        unsafe { &mut *(repr as *mut F32aRepr as *mut Self) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<T: fmt::Display> fmt::Display for Rgba<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R: {:.3}, G: {:.3}, B: {:.3}, A: {:.3}",
            self.r, self.g, self.b, self.a
        )
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<T: fmt::Display> fmt::Debug for Rgba<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R: {}, G: {}, B: {}, A: {}",
            self.r, self.g, self.b, self.a
        )
    }
}

/// A bag of components with names L, A, B. Some `Color`s with Lab color
/// encodings will `Deref`/`DerefMut` to this struct so that you can access
/// their components with dot-syntax.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lab<T> {
    pub l: T,
    pub a: T,
    pub b: T,
}

unsafe impl ComponentStructFor<U8Repr> for Lab<u8> {
    fn cast(repr: &U8Repr) -> &Self {
        // SAFETY: [u8; 3] is guaranteed to have the same layout as Self
        unsafe { &*(repr as *const U8Repr as *const Self) }
    }

    fn cast_mut(repr: &mut U8Repr) -> &mut Self {
        // SAFETY: [u8; 3] is guaranteed to have the same layout as Self
        unsafe { &mut *(repr as *mut U8Repr as *mut Self) }
    }
}

unsafe impl ComponentStructFor<F32Repr> for Lab<f32> {
    fn cast(repr: &F32Repr) -> &Self {
        // SAFETY: Vec3 is guaranteed to have the same layout as Self
        unsafe { &*(repr as *const F32Repr as *const Self) }
    }

    fn cast_mut(repr: &mut F32Repr) -> &mut Self {
        // SAFETY: Vec3 is guaranteed to have the same layout as Self
        unsafe { &mut *(repr as *mut F32Repr as *mut Self) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<T: fmt::Display> fmt::Display for Lab<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L: {:.3}, a: {:.3}, b: {:.3}", self.l, self.a, self.b)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<T: fmt::Display> fmt::Debug for Lab<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L: {}, a: {}, b: {}", self.l, self.a, self.b)
    }
}
