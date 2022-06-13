use std::ops::Range;
use num_traits::Float;

/// Interpolate between two floats
pub trait Interpolation
    where Self: Float
{

    /// Linear interpolation between self and b by t amount
    fn lerp(self, b: Self, t: Self) -> Self;

    /// Get the interpolation amount between a and b
    fn lerp_inv(self, a: Self, b: Self) -> Self;

    /// remap self from range i to range o
    fn remap(self, i: Range<Self>, o: Range<Self>) -> Self {
        let t = self.lerp_inv(i.start, i.end);
        o.start.lerp(o.end, t)
    }
}

impl<F> Interpolation for F
    where F: Float
{
    fn lerp(self, b: Self, t: Self) -> Self {
        (F::one() - t) * self + b * t
    }

    fn lerp_inv(self, a: Self, b: Self) -> Self {
        (self - a) / (b - a)
    }
}
