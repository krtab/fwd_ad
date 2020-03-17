use crate::Dual;
use std::borrow::*;
use std::ops;

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

impl<L, R> ops::AddAssign<Dual<R>> for Dual<L>
where
    L: BorrowMut<[f64]>,
    R: Borrow<[f64]>,
{
    fn add_assign(&mut self, rhs: Dual<R>) {
        check_same_ndiffs!(self, rhs);
        self.as_slice_mut()
            .iter_mut()
            .zip(rhs.as_slice())
            .for_each(|(ds, dr)| *ds += dr);
    }
}

impl<L, R> ops::Add<Dual<R>> for Dual<L>
where
    L: BorrowMut<[f64]>,
    R: Borrow<[f64]>,
{
    type Output = Self;
    fn add(mut self, rhs: Dual<R>) -> Self {
        self += rhs;
        self
    }
}

impl<S, R> ops::DivAssign<Dual<R>> for Dual<S>
where
    S: BorrowMut<[f64]>,
    R: Borrow<[f64]>,
{
    fn div_assign(&mut self, rhs: Dual<R>) {
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

impl<S, R> ops::Div<Dual<R>> for Dual<S>
where
    S: BorrowMut<[f64]>,
    R: Borrow<[f64]>,
{
    type Output = Dual<S>;
    fn div(mut self, rhs: Dual<R>) -> Dual<S> {
        self /= rhs;
        self
    }
}

impl<S, R> ops::MulAssign<Dual<R>> for Dual<S>
where
    S: BorrowMut<[f64]>,
    R: Borrow<[f64]>,
{
    fn mul_assign(&mut self, rhs: Dual<R>) {
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

impl<S, R> ops::Mul<Dual<R>> for Dual<S>
where
    S: BorrowMut<[f64]>,
    R: Borrow<[f64]>,
{
    type Output = Dual<S>;
    fn mul(mut self, rhs: Dual<R>) -> Dual<S> {
        self *= rhs;
        self
    }
}

impl<S, R> ops::SubAssign<Dual<R>> for Dual<S>
where
    S: BorrowMut<[f64]>,
    R: Borrow<[f64]>,
{
    fn sub_assign(&mut self, rhs: Dual<R>) {
        check_same_ndiffs!(self, rhs);
        self.as_slice_mut()
            .iter_mut()
            .zip(rhs.as_slice())
            .for_each(|(ds, dr)| *ds -= dr);
    }
}

impl<S, R> ops::Sub<Dual<R>> for Dual<S>
where
    S: BorrowMut<[f64]>,
    R: Borrow<[f64]>,
{
    type Output = Dual<S>;
    fn sub(mut self, rhs: Dual<R>) -> Dual<S> {
        self -= rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_pair() -> (Dual<Vec<f64>>, Dual<Vec<f64>>) {
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
        let res = (x + y.view()) * y;
        assert_eq!(res, Dual(vec![(42. + 17.) * 17., 17., 2. * 17. + 42.]));
    }

    #[test]
    fn test_diff_div() {
        let mut x = Dual::constant(42., 2);
        let mut y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        let res = x / y;
        assert_eq!(res, Dual(vec![42. / 17., 1. / 17., -42. / (17. * 17.)]));
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
        let x = Dual(vec![3., 1.]);
        assert!(x
            .clone()
            .powdual(x)
            .is_close(&Dual(vec![27., 27. * (3_f64.ln() + 1.)]), 1e-8))
    }
}
