use crate::{Dual, RW, RO, CompatibleWith};
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

        impl<L> ops::$opsname<f64> for Dual<L, RW>
        where
            L: BorrowMut<[f64]>,
        {
            type Output = Self;
            fn $fn_name(mut self, rhs: f64) -> Self {
                ops::$opsassignname::$fnassign_name(&mut self, rhs);
                self
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L> ops::$opsname<f64> for Dual<L, RO>
        where
            L : CompatibleWith<RO>,
        {
            type Output = Dual<Vec<f64>, RW>;
            fn $fn_name(self, rhs: f64) -> Dual<Vec<f64>,RW> {
                let mut res = self.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, rhs);
                res
            }
        }
    }
}

macro_rules! derive_ops_commut {
    ($opsname : ident, $opsassignname : ident, $fn_name:ident, $fnassign_name : ident) => {

        derive_ops!($opsname, $opsassignname, $fn_name, $fnassign_name);

        impl<R> ops::$opsname<Dual<R, RW>> for f64
        where
            R: BorrowMut<[f64]>,
        {
            type Output = Dual<R, RW>;
            fn $fn_name(self, mut rhs: Dual<R, RW>) -> Dual<R, RW> {
                ops::$opsassignname::$fnassign_name(&mut rhs, self);
                rhs
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<R> ops::$opsname<Dual<R,RO>> for f64
        where
            R : CompatibleWith<RO>,
        {
            type Output = Dual<Vec<f64>, RW>;
            fn $fn_name(self, rhs: Dual<R,RO>) -> Dual<Vec<f64>,RW> {
                let mut res = rhs.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, rhs);
                res
            }
        }
    }
}

//
//
// Ops Implementations
// ===================
//
//

impl<S> ops::AddAssign<f64> for Dual<S, RW>
where
    S: BorrowMut<[f64]>,
{
    fn add_assign(&mut self, rhs: f64) {
        *self.val_mut() += rhs;
    }
}
derive_ops_commut!(Add, AddAssign, add, add_assign);

impl<S> ops::DivAssign<f64> for Dual<S, RW>
where
    S: BorrowMut<[f64]>,
{
    fn div_assign(&mut self, rhs: f64) {
        self.as_slice_mut().iter_mut().for_each(|ds| *ds /= rhs);
    }
}
derive_ops!(Div,DivAssign,div,div_assign);

impl<S> ops::MulAssign<f64> for Dual<S, RW>
where
    S: BorrowMut<[f64]>,
{
    fn mul_assign(&mut self, rhs: f64) {
        self.as_slice_mut().iter_mut().for_each(|ds| *ds *= rhs);
    }
}
derive_ops_commut!(Mul, MulAssign, mul, mul_assign);

impl<S> ops::SubAssign<f64> for Dual<S, RW>
where
    S: BorrowMut<[f64]>,
{
    fn sub_assign(&mut self, rhs: f64) {
        *self.val_mut() -= rhs;
    }
}
derive_ops!(Sub, SubAssign, sub, sub_assign);
impl<S> ops::Sub<Dual<S,RW>> for f64
where
    S: BorrowMut<[f64]>,
{
    type Output = Dual<S,RW>;
    fn sub(self, mut rhs: Dual<S, RW>) -> Dual<S, RW> {
        *rhs.val_mut() = self - rhs.val();
        for d in rhs.diffs_mut() {
            *d = -*d;
        };
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
