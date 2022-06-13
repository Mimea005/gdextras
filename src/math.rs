use std::ops::Range;
use num_traits::Float;


trait Interpolation
    where Self: Float
{

    fn lerp(self, b: Self, t: Self) -> Self;

    fn lerp_inv(self, a: Self, b: Self) -> Self;

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
