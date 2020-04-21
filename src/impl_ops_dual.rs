use crate::{Dual, RO, RW, CompatibleWith};
use std::borrow::*;
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

        impl<L, R, M> ops::$opsassignname<Dual<R, M>> for Dual<L, RW> 
        where
            L: BorrowMut<[f64]>,
            R: CompatibleWith<M>,
        {
            fn $fnassign_name(&mut self, rhs: Dual<R, M>) {
                ops::$opsassignname::$fnassign_name(self,&rhs)
            }
        }

        impl<L, R, M> ops::$opsname<Dual<R, M>> for Dual<L, RW>
        where
            L: BorrowMut<[f64]>,
            R: CompatibleWith<M>,
        {
            type Output = Self;
            fn $fn_name(mut self, rhs: Dual<R, M>) -> Self {
                ops::$opsassignname::$fnassign_name(&mut self, &rhs);
                self
            }
        }
        impl<L, R, M> ops::$opsname<&Dual<R, M>> for Dual<L, RW>
        where
            L: BorrowMut<[f64]>,
            R: CompatibleWith<M>,
        {
            type Output = Self;
            fn $fn_name(mut self, rhs: &Dual<R, M>) -> Self {
                ops::$opsassignname::$fnassign_name(&mut self, rhs);
                self
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, R> ops::$opsname<Dual<R, RO>> for Dual<L, RO>
        where
            L : CompatibleWith<RO>,
            R : CompatibleWith<RO>,
        {
            type Output = Dual<Vec<f64>, RW>;
            fn $fn_name(self, rhs: Dual<R,RO>) -> Dual<Vec<f64>,RW> {
                let mut res = self.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, &rhs);
                res
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, R, MR> ops::$opsname<&Dual<R, MR>> for Dual<L, RO>
        where
            L : CompatibleWith<RO>,
            R : CompatibleWith<MR>,
        {
            type Output = Dual<Vec<f64>, RW>;
            fn $fn_name(self, rhs: &Dual<R,MR>) -> Dual<Vec<f64>,RW> {
                let mut res = self.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, rhs);
                res
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, R, ML> ops::$opsname<Dual<R, RO>> for &Dual<L, ML>
        where
            L : CompatibleWith<ML>,
            R : CompatibleWith<RO>,
        {
            type Output = Dual<Vec<f64>, RW>;
            fn $fn_name(self, rhs: Dual<R,RO>) -> Dual<Vec<f64>,RW> {
                let mut res = self.to_owning();
                ops::$opsassignname::$fnassign_name(&mut res, &rhs);
                res
            }
        }

        #[cfg(feature = "implicit-clone")]
        impl<L, R, MR, ML> ops::$opsname<&Dual<R, MR>> for &Dual<L, ML>
        where
            L : CompatibleWith<ML>,
            R : CompatibleWith<MR>,
        {
            type Output = Dual<Vec<f64>, RW>;
            fn $fn_name(self, rhs: &Dual<R,MR>) -> Dual<Vec<f64>,RW> {
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

        impl<L, R> ops::$opsname<Dual<R, RW>> for Dual<L, RO>
        where
            L: CompatibleWith<RO>,
            R: BorrowMut<[f64]>,
        {
            type Output = Dual<R, RW>;
            fn $fn_name(self, mut rhs: Dual<R, RW>) -> Dual<R, RW> {
                ops::$opsassignname::$fnassign_name(&mut rhs, &self);
                rhs
            }
        }

        impl<L, R> ops::$opsname<Dual<R, RW>> for &Dual<L, RO>
        where
            L: CompatibleWith<RO>,
            R: BorrowMut<[f64]>,
        {
            type Output = Dual<R, RW>;
            fn $fn_name(self, mut rhs: Dual<R, RW>) -> Dual<R, RW> {
                ops::$opsassignname::$fnassign_name(&mut rhs, self);
                rhs
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

impl<L, R, M> ops::AddAssign<&Dual<R, M>> for Dual<L, RW>
where
    L: BorrowMut<[f64]>,
    R: CompatibleWith<M>,
{
    fn add_assign(&mut self, rhs: &Dual<R, M>) {
        check_same_ndiffs!(self, rhs);
        self.as_slice_mut()
            .iter_mut()
            .zip(rhs.as_slice())
            .for_each(|(ds, dr)| *ds += dr);
    }
}

derive_ops_commut!(Add, AddAssign, add, add_assign);



impl<L, R, M> ops::DivAssign<&Dual<R, M>> for Dual<L, RW>
where
    L: BorrowMut<[f64]>,
    R: CompatibleWith<M>,
{
    fn div_assign(&mut self, rhs: &Dual<R, M>) {
        check_same_ndiffs!(self, rhs);
        let vs = self.val();
        let vr = rhs.val();
        *self.val_mut() /= vr;
        self.diffs_mut()
            .iter_mut()
            .zip(rhs.diffs())
            .for_each(|(ds, dr)| *ds = (*ds - dr * vs / vr) / vr);
    }
}

impl<L, R> ops::Div<Dual<R, RW>> for Dual<L, RO>
where
    L: CompatibleWith<RO>,
    R: BorrowMut<[f64]>,
{
    type Output = Dual<R,RW>;
    fn div(self, mut rhs: Dual<R, RW>) -> Dual<R,RW>{
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

impl<L, R, M> ops::MulAssign<&Dual<R, M>> for Dual<L, RW>
where
    L: BorrowMut<[f64]>,
    R: CompatibleWith<M>,
{
    fn mul_assign(&mut self, rhs: &Dual<R, M>) {
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

impl<L, R, M> ops::SubAssign<&Dual<R, M>> for Dual<L, RW>
where
    L: BorrowMut<[f64]>,
    R: CompatibleWith<M>,
{
    fn sub_assign(&mut self, rhs: &Dual<R, M>) {
        check_same_ndiffs!(self, rhs);
        self.as_slice_mut()
            .iter_mut()
            .zip(rhs.as_slice())
            .for_each(|(ds, dr)| *ds -= dr);
    }
}

derive_ops!(Sub, SubAssign, sub, sub_assign);


#[cfg(test)]
mod tests {
    use super::*;

    fn generate_pair() -> (Dual<Vec<f64>, RW>, Dual<Vec<f64>, RW>) {
        let mut y = Dual::constant(42., 3);
        let mut x = Dual::constant(42., 3);
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
        let y = Dual::constant(42., 3);
        let yv = y.view();
        let x = Dual::constant(42., 2);
        x + yv;
        let x = Dual::constant(42., 2);
        x * yv;
        let x = Dual::constant(42., 2);
        x / yv;
        let x = Dual::constant(42., 2);
        x - yv;
    }

    #[test]
    fn test_diff_add_mul() {
        let mut x = Dual::constant(42., 2);
        let mut y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        let res = (x + &y) * y;
        assert_eq!(res, Dual::from(vec![(42. + 17.) * 17., 17., 2. * 17. + 42.]));
    }

    #[test]
    fn test_diff_div() {
        let mut x = Dual::constant(42., 2);
        let mut y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        let res = x / y;
        assert_eq!(res, Dual::from(vec![42. / 17., 1. / 17., -42. / (17. * 17.)]));
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
        let mut x = Dual::constant(42., 2);
        let mut y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() - y.view(), x + (-y))
    }

    #[test]
    fn test_powd() {
        let x : Dual<_,RW> = Dual::from(vec![3., 1.]);
        assert!(x
            .clone()
            .powdual(x)
            .is_close(&Dual::<_, RW>::from(vec![27., 27. * (3_f64.ln() + 1.)]), 1e-8))
    }
}
