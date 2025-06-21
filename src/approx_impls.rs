// Approx trait implementations for testing
use crate::component_structs::*;
use crate::traits::*;
use crate::*;
use approx::{AbsDiffEq, RelativeEq};

impl<E: ColorEncoding> AbsDiffEq for Color<E>
where
    E::Repr: PartialEq + Copy,
    E::ComponentStruct: AbsDiffEq,
{
    type Epsilon = <E::ComponentStruct as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        E::ComponentStruct::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        (**self).abs_diff_eq(&**other, epsilon)
    }
}

impl<E: ColorEncoding> RelativeEq for Color<E>
where
    E::Repr: PartialEq + Copy,
    E::ComponentStruct: RelativeEq,
{
    fn default_max_relative() -> Self::Epsilon {
        E::ComponentStruct::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        (**self).relative_eq(&**other, epsilon, max_relative)
    }
}

// Implement for component structs
impl AbsDiffEq for Rgb<f32> {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.r.abs_diff_eq(&other.r, epsilon)
            && self.g.abs_diff_eq(&other.g, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
    }
}

impl RelativeEq for Rgb<f32> {
    fn default_max_relative() -> Self::Epsilon {
        f32::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.r.relative_eq(&other.r, epsilon, max_relative)
            && self.g.relative_eq(&other.g, epsilon, max_relative)
            && self.b.relative_eq(&other.b, epsilon, max_relative)
    }
}

impl AbsDiffEq for Rgba<f32> {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.r.abs_diff_eq(&other.r, epsilon)
            && self.g.abs_diff_eq(&other.g, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
            && self.a.abs_diff_eq(&other.a, epsilon)
    }
}

impl RelativeEq for Rgba<f32> {
    fn default_max_relative() -> Self::Epsilon {
        f32::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.r.relative_eq(&other.r, epsilon, max_relative)
            && self.g.relative_eq(&other.g, epsilon, max_relative)
            && self.b.relative_eq(&other.b, epsilon, max_relative)
            && self.a.relative_eq(&other.a, epsilon, max_relative)
    }
}

impl AbsDiffEq for Rgb<u8> {
    type Epsilon = u8;

    fn default_epsilon() -> Self::Epsilon {
        0
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let r_diff = (self.r as i32 - other.r as i32).unsigned_abs() as u8;
        let g_diff = (self.g as i32 - other.g as i32).unsigned_abs() as u8;
        let b_diff = (self.b as i32 - other.b as i32).unsigned_abs() as u8;
        r_diff <= epsilon && g_diff <= epsilon && b_diff <= epsilon
    }
}

impl RelativeEq for Rgb<u8> {
    fn default_max_relative() -> Self::Epsilon {
        0
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        _max_relative: Self::Epsilon,
    ) -> bool {
        self.abs_diff_eq(other, epsilon)
    }
}

impl AbsDiffEq for Rgba<u8> {
    type Epsilon = u8;

    fn default_epsilon() -> Self::Epsilon {
        0
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let r_diff = (self.r as i32 - other.r as i32).unsigned_abs() as u8;
        let g_diff = (self.g as i32 - other.g as i32).unsigned_abs() as u8;
        let b_diff = (self.b as i32 - other.b as i32).unsigned_abs() as u8;
        let a_diff = (self.a as i32 - other.a as i32).unsigned_abs() as u8;
        r_diff <= epsilon && g_diff <= epsilon && b_diff <= epsilon && a_diff <= epsilon
    }
}

impl RelativeEq for Rgba<u8> {
    fn default_max_relative() -> Self::Epsilon {
        0
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        _max_relative: Self::Epsilon,
    ) -> bool {
        self.abs_diff_eq(other, epsilon)
    }
}

impl AbsDiffEq for Lab<f32> {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.l.abs_diff_eq(&other.l, epsilon)
            && self.a.abs_diff_eq(&other.a, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
    }
}

impl RelativeEq for Lab<f32> {
    fn default_max_relative() -> Self::Epsilon {
        f32::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.l.relative_eq(&other.l, epsilon, max_relative)
            && self.a.relative_eq(&other.a, epsilon, max_relative)
            && self.b.relative_eq(&other.b, epsilon, max_relative)
    }
}
