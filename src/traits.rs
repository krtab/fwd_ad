//! All user implementable traits needed to use `Dual` with uncommon container or scalar types are located here.

mod scalars;
pub use scalars::Scalar;
mod containers;
pub use containers::*;
