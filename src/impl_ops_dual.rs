#[cfg(feature = "implicit-clone")]
use crate::ToOwning;
use crate::{CompatibleWith, Dual, OwningMode, ROAble, RWAble, Scalar, RO, RW};
use std::ops;

//
//
// Macros
// ======
//
//

macro_rules! check_same_ndiffs {
    ($x : ident , $y : ident) => {
        assert_eq!(
            $x.ndiffs(),
            $y.ndiffs(),
            "Duals have different numbers of diffs: {} =/= {}.",
            $x.ndiffs(),
            $y.ndiffs()
        );
    };
}

// Derive multiple implementations of the ops from the XAssign<&Dual<_,_>> for Dual<_,RW> one
macro_rules! derive_ops {
    ($opsname : ident, $opsassignname : ident, $fn_name:ident, $fnassign_name : ident) => {
        impl<L, R, M, F> ops::$opsassignname<Dual<R, M, F>> for Dual<L, RW, F>
        where
            M: OwningMode,
            L: RWAble<F>,
            R: ROAble<F>,
            R: CompatibleWith<M, F>,
            F: Scalar,
        {
            fn $fnassign_name(&mut self, rhs: Dual<R, M, F>) {
                ops::$opsassignname::$fnassign_name(self, &rhs)
            }
        }

        impl<L, R, M, F> ops::$opsname<Dual<R, M, F>> for Dual<L, RW, F>
        where
            M: OwningMode,
            L: RWAble<F>,
            R: ROAble<F>,
            R: CompatibleWith<M, F>,
            F: Scalar,
        {
            type Output = Self;
            fn $fn_name(mut self, rhs: Dual<R, M, F>) -> Self {
                ops::$opsassignname::$fnassign_name(&mut self, &rhs);
                self
            }
        }
        impl<L, R, M, F> ops::$opsname<&Dual<R, M, F>> for Dual<L, RW, F>
        where
            M: OwningMode,
            L: RWAble<F>,
            R: ROAble<F>,
            R: CompatibleWith<M, F>,
            F: Scalar,
        {
            type Output = Self;
            fn $fn_name(mut self, rhs: &Dual<R, M, F>) -> Self {
                ops::$opsassignname::$fnassign_name(&mut self, rhs);
                self
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, R, F> ops::$opsname<Dual<R, RO, F>> for Dual<L, RO, F>
        where
            L: ToOwning<F>,
            R: ROAble<F>,
            F: Scalar,
        {
            type Output = Dual<L::Owning, RW, F>;
            fn $fn_name(self, rhs: Dual<R, RO, F>) -> Dual<L::Owning, RW, F> {
                let mut res = self.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, &rhs);
                res
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, R, MR, F> ops::$opsname<&Dual<R, MR, F>> for Dual<L, RO, F>
        where
            L: ToOwning<F>,
            MR: OwningMode,
            R: ROAble<F>,
            R: CompatibleWith<MR, F>,
            F: Scalar,
        {
            type Output = Dual<L::Owning, RW, F>;
            fn $fn_name(self, rhs: &Dual<R, MR, F>) -> Self::Output {
                let mut res = self.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, rhs);
                res
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, R, ML, F> ops::$opsname<Dual<R, RO, F>> for &Dual<L, ML, F>
        where
            ML: OwningMode,
            L: ToOwning<F>,
            L: CompatibleWith<ML, F>,
            R: ROAble<F>,
            F: Scalar,
        {
            type Output = Dual<L::Owning, RW, F>;
            fn $fn_name(self, rhs: Dual<R, RO, F>) -> Self::Output {
                let mut res = self.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, &rhs);
                res
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, R, MR, ML, F> ops::$opsname<&Dual<R, MR, F>> for &Dual<L, ML, F>
        where
            MR: OwningMode,
            ML: OwningMode,
            L: ToOwning<F>,
            L: CompatibleWith<ML, F>,
            R: ROAble<F>,
            R: CompatibleWith<MR, F>,
            F: Scalar,
        {
            type Output = Dual<L::Owning, RW, F>;
            fn $fn_name(self, rhs: &Dual<R, MR, F>) -> Self::Output {
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

        impl<L, R, F> ops::$opsname<Dual<R, RW, F>> for Dual<L, RO, F>
        where
            L: ROAble<F>,
            R: RWAble<F>,
            F: Scalar,
        {
            type Output = Dual<R, RW, F>;
            fn $fn_name(self, mut rhs: Dual<R, RW, F>) -> Dual<R, RW, F> {
                ops::$opsassignname::$fnassign_name(&mut rhs, &self);
                rhs
            }
        }

        impl<L, R, F, M> ops::$opsname<Dual<R, RW, F>> for &Dual<L, M, F>
        where
            M: OwningMode,
            L: ROAble<F>,
            L: CompatibleWith<M, F>,
            R: RWAble<F>,
            F: Scalar,
        {
            type Output = Dual<R, RW, F>;
            fn $fn_name(self, mut rhs: Dual<R, RW, F>) -> Dual<R, RW, F> {
                ops::$opsassignname::$fnassign_name(&mut rhs, self);
                rhs
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

impl<L, R, M, F> ops::AddAssign<&Dual<R, M, F>> for Dual<L, RW, F>
where
    M: OwningMode,
    L: RWAble<F>,
    R: ROAble<F>,
    R: CompatibleWith<M, F>,
    F: Scalar,
{
    fn add_assign(&mut self, rhs: &Dual<R, M, F>) {
        check_same_ndiffs!(self, rhs);
        self.as_slice_mut()
            .iter_mut()
            .zip(rhs.as_slice())
            .for_each(|(ds, dr)| *ds += dr);
    }
}

derive_ops_commut!(Add, AddAssign, add, add_assign);

impl<L, R, M, F> ops::DivAssign<&Dual<R, M, F>> for Dual<L, RW, F>
where
    M: OwningMode,
    L: RWAble<F>,
    R: ROAble<F>,
    R: CompatibleWith<M, F>,
    F: Scalar,
{
    fn div_assign(&mut self, rhs: &Dual<R, M, F>) {
        check_same_ndiffs!(self, rhs);
        let vs = self.val();
        let vr = rhs.val();
        *self.val_mut() /= vr;
        self.diffs_mut()
            .iter_mut()
            .zip(rhs.diffs())
            .for_each(|(ds, dr)| *ds = (*ds - *dr * vs / vr) / vr);
    }
}

impl<L, R, F> ops::Div<Dual<R, RW, F>> for Dual<L, RO, F>
where
    L: ROAble<F>,
    R: RWAble<F>,
    F: Scalar,
{
    type Output = Dual<R, RW, F>;
    fn div(self, rhs: Dual<R, RW, F>) -> Dual<R, RW, F> {
        &self / rhs
    }
}

impl<L, R, ML, F> ops::Div<Dual<R, RW, F>> for &Dual<L, ML, F>
where
    ML: OwningMode,
    L: ROAble<F>,
    L: CompatibleWith<ML, F>,
    R: RWAble<F>,
    F: Scalar,
{
    type Output = Dual<R, RW, F>;
    fn div(self, mut rhs: Dual<R, RW, F>) -> Dual<R, RW, F> {
        check_same_ndiffs!(self, rhs);
        let vs = self.val();
        let vr = rhs.val();
        *rhs.val_mut() = vs / vr;
        self.diffs()
            .iter()
            .zip(rhs.diffs_mut())
            .for_each(|(ds, dr)| *dr = (*ds - *dr * vs / vr) / vr);
        rhs
    }
}

derive_ops!(Div, DivAssign, div, div_assign);

impl<L, R, M, F> ops::MulAssign<&Dual<R, M, F>> for Dual<L, RW, F>
where
    M: OwningMode,
    L: RWAble<F>,
    R: ROAble<F>,
    R: CompatibleWith<M, F>,
    F: Scalar,
{
    fn mul_assign(&mut self, rhs: &Dual<R, M, F>) {
        check_same_ndiffs!(self, rhs);
        let vs = self.val();
        let vr = rhs.val();
        *self.val_mut() *= vr;
        self.diffs_mut()
            .iter_mut()
            .zip(rhs.diffs())
            .for_each(|(ds, dr)| *ds = vs * dr + vr * *ds);
    }
}

derive_ops_commut!(Mul, MulAssign, mul, mul_assign);

impl<L, R, M, F> ops::SubAssign<&Dual<R, M, F>> for Dual<L, RW, F>
where
    M: OwningMode,
    L: RWAble<F>,
    R: ROAble<F>,
    R: CompatibleWith<M, F>,
    F: Scalar,
{
    fn sub_assign(&mut self, rhs: &Dual<R, M, F>) {
        check_same_ndiffs!(self, rhs);
        self.as_slice_mut()
            .iter_mut()
            .zip(rhs.as_slice())
            .for_each(|(ds, dr)| *ds -= dr);
    }
}

impl<L, R, F> ops::Sub<Dual<R, RW, F>> for Dual<L, RO, F>
where
    L: ROAble<F>,
    R: RWAble<F>,
    F: Scalar,
{
    type Output = Dual<R, RW, F>;
    fn sub(self, rhs: Dual<R, RW, F>) -> Dual<R, RW, F> {
        &self - rhs
    }
}

impl<L, R, ML, F> ops::Sub<Dual<R, RW, F>> for &Dual<L, ML, F>
where
    L: ROAble<F>,
    ML: OwningMode,
    L: CompatibleWith<ML, F>,
    R: RWAble<F>,
    F: Scalar,
{
    type Output = Dual<R, RW, F>;
    fn sub(self, mut rhs: Dual<R, RW, F>) -> Dual<R, RW, F> {
        check_same_ndiffs!(self, rhs);
        self.as_slice()
            .iter()
            .zip(rhs.as_slice_mut())
            .for_each(|(ds, dr)| *dr = *ds - *dr);
        rhs
    }
}

derive_ops!(Sub, SubAssign, sub, sub_assign);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instanciations::vecf64::Owning;

    fn generate_pair() -> (Dual<Vec<f64>, RW, f64>, Dual<Vec<f64>, RW, f64>) {
        let mut y = Owning::constant(42., 3);
        let mut x = Owning::constant(42., 3);
        x.diffs_mut()[0] = 17.;
        y.diffs_mut()[1] = -1.;
        x.diffs_mut()[2] = -7.;
        y.diffs_mut()[2] = 13.;
        (x, y)
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_diff_panic() {
        let y = Owning::constant(42., 3);
        let yv = y.view();
        let x = Owning::constant(42., 2);
        x + yv;
    }

    #[test]
    fn test_diff_add_mul() {
        let mut x = Owning::constant(42., 2);
        let mut y = Owning::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        let res = (x + &y) * y;
        assert_eq!(
            res,
            Owning::from(vec![(42. + 17.) * 17., 17., 2. * 17. + 42.])
        );
    }

    #[test]
    fn test_diff_div() {
        let mut x = Owning::constant(42., 2);
        let mut y = Owning::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        let res = x / y;
        assert_eq!(
            res,
            Owning::from(vec![42. / 17., 1. / 17., -42. / (17. * 17.)])
        );
    }

    #[test]
    fn test_diff_div_inv() {
        let (x, y) = generate_pair();
        let res1 = x.clone() / y.view();
        let res2 = x * y.inv();
        assert!(res1.is_close(&res2, 1e-8));
    }

    #[test]
    fn test_powf() {
        let (x, y) = generate_pair();
        let res1 = x.clone() * x.view();
        let res2 = x.powf(2.);
        assert!(res1.is_close(&res2, 1e-8));
        let res3 = y.clone().inv();
        let res4 = y.powf(-1.);
        assert!(res3.is_close(&res4, 1e-8));
    }

    #[test]
    fn test_diff_subneg() {
        let mut x = Owning::constant(42., 2);
        let mut y = Owning::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() - y.view(), x + (-y))
    }

    #[test]
    fn test_powd() {
        let x: Dual<_, RW, f64> = Owning::from(vec![3., 1.]);
        assert!(x.clone().powdual(x).is_close(
            &Dual::<_, RW, f64>::from(vec![27., 27. * (3_f64.ln() + 1.)]),
            1e-8
        ))
    }
}
