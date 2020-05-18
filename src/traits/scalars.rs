use num_traits::*;

/// Indicates that a type can be used as a scalar.
///
/// Implemented for `f32` and `f64`.
pub trait Scalar: NumAssignOps + NumRef + NumAssignRef + real::Real + float::FloatConst {}

impl<F> Scalar for F where F: NumAssignOps + NumRef + NumAssignRef + real::Real + float::FloatConst {}
