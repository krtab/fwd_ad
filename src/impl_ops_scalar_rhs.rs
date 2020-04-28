#[cfg(feature = "implicit-clone")]
use crate::{CompatibleWith, RO};
use crate::{Dual, RW, Scalar};
use std::borrow::*;
use std::ops;

//
//
// Macros
// ======
//
//

// Derive multiple implementations of the ops from the XAssign<f64> for Dual<_,RW> one
macro_rules! derive_ops {
    ($opsname : ident, $opsassignname : ident, $fn_name:ident, $fnassign_name : ident) => {
        impl<L, F> ops::$opsname<F> for Dual<L, RW, F>
        where
            L: BorrowMut<[F]>,
            F: Scalar
        {
            type Output = Self;
            fn $fn_name(mut self, rhs: F) -> Self {
                ops::$opsassignname::$fnassign_name(&mut self, rhs);
                self
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, F> ops::$opsname<F> for Dual<L, RO, F>
        where
            L: CompatibleWith<RO, F>,
            F: Scalar
        {
            type Output = Dual<Vec<F>, RW, F>;
            fn $fn_name(self, rhs: F) -> Dual<Vec<F>, RW, F> {
                let mut res = self.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, rhs);
                res
            }
        }
    };
}

macro_rules! derive_ops_commut {
    ($opsname : ident, $opsassignname : ident, $fn_name:ident, $fnassign_name : ident) => {
        derive_ops!($opsname, $opsassignname, $fn_name, $fnassign_name);

        impl<R> ops::$opsname<Dual<R, RW, f64>> for f64
        where
            R: BorrowMut<[f64]>,
            f64: Scalar
        {
            type Output = Dual<R, RW, f64>;
            fn $fn_name(self, mut rhs: Dual<R, RW, f64>) -> Dual<R, RW, f64> {
                ops::$opsassignname::$fnassign_name(&mut rhs, self);
                rhs
            }
        }

        impl<R> ops::$opsname<Dual<R, RW, f32>> for f32
        where
            R: BorrowMut<[f32]>,
        {
            type Output = Dual<R, RW, f32>;
            fn $fn_name(self, mut rhs: Dual<R, RW, f32>) -> Dual<R, RW, f32> {
                ops::$opsassignname::$fnassign_name(&mut rhs, self);
                rhs
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<R> ops::$opsname<Dual<R, RO, f64>> for f64
        where
            R: CompatibleWith<RO, f64>,
            f64: Scalar
        {
            type Output = Dual<Vec<f64>, RW, f64>;
            fn $fn_name(self, rhs: Dual<R, RO, f64>) -> Dual<Vec<f64>, RW, f64> {
                let mut res = rhs.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, rhs);
                res
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<R> ops::$opsname<Dual<R, RO, f32>> for f32
        where
            R: CompatibleWith<RO, f32>,
            f32: Scalar
        {
            type Output = Dual<Vec<f32>, RW, f32>;
            fn $fn_name(self, rhs: Dual<R, RO, f32>) -> Dual<Vec<f32>, RW, f32> {
                let mut res = rhs.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, rhs);
                res
            }
        }
    };
}

//
//
// Ops Implementations
// ===================
//
//

impl<S, F> ops::AddAssign<F> for Dual<S, RW, F>
where
    S: BorrowMut<[F]>,
    F: Scalar
{
    fn add_assign(&mut self, rhs: F) {
        *self.val_mut() += rhs;
    }
}
derive_ops_commut!(Add, AddAssign, add, add_assign);

impl<S, F> ops::DivAssign<F> for Dual<S, RW, F>
where
    S: BorrowMut<[F]>,
    F: Scalar
{
    fn div_assign(&mut self, rhs: F) {
        self.as_slice_mut().iter_mut().for_each(|ds| *ds /= rhs);
    }
}
derive_ops!(Div, DivAssign, div, div_assign);

impl<S, F> ops::MulAssign<F> for Dual<S, RW, F>
where
    S: BorrowMut<[F]>,
    F: Scalar
{
    fn mul_assign(&mut self, rhs: F) {
        self.as_slice_mut().iter_mut().for_each(|ds| *ds *= rhs);
    }
}
derive_ops_commut!(Mul, MulAssign, mul, mul_assign);

impl<S, F> ops::SubAssign<F> for Dual<S, RW, F>
where
    S: BorrowMut<[F]>,
    F: Scalar
{
    fn sub_assign(&mut self, rhs: F) {
        *self.val_mut() -= rhs;
    }
}
derive_ops!(Sub, SubAssign, sub, sub_assign);
impl<S> ops::Sub<Dual<S, RW, f64>> for f64
where
    S: BorrowMut<[f64]>,
    f64: Scalar
{
    type Output = Dual<S, RW, f64>;
    fn sub(self, mut rhs: Dual<S, RW, f64>) -> Dual<S, RW, f64> {
        *rhs.val_mut() = self - rhs.val();
        for d in rhs.diffs_mut() {
            *d = -*d;
        }
        rhs
    }
}
impl<S> ops::Sub<Dual<S, RW, f32>> for f32
where
    S: BorrowMut<[f32]>,
    f32: Scalar
{
    type Output = Dual<S, RW, f32>;
    fn sub(self, mut rhs: Dual<S, RW, f32>) -> Dual<S, RW, f32> {
        *rhs.val_mut() = self - rhs.val();
        for d in rhs.diffs_mut() {
            *d = -*d;
        }
        rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_rhs_add_mul() {
        let mut x = Dual::constant(42., 2);
        let y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 0.;
        x.diffs_mut()[1] = 1.;
        assert_eq!((x.clone() + &y) * y, (x + 17.) * 17.);
    }

    #[test]
    fn test_scalar_rhs_div() {
        let mut x = Dual::constant(42., 2);
        let y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 0.;
        x.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() / y, x / 17.);
    }

    #[test]
    fn test_diff_subneg() {
        let mut x = Dual::constant(42., 2);
        let y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 0.;
        x.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() - y, x - 17.)
    }

    #[test]
    fn test_diff_subneg2() {
        let mut x = Dual::constant(42., 2);
        let y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 0.;
        x.diffs_mut()[1] = 1.;
        assert_eq!(y - x.clone(), 17. - x)
    }
}
