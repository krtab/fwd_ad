use crate::Dual;
use std::borrow::*;
use std::ops;

impl<S> ops::AddAssign<f64> for Dual<S>
where
    S: BorrowMut<[f64]>,
{
    fn add_assign(&mut self, rhs: f64) {
        *self.val_mut() += rhs;
    }
}

impl<S> ops::Add<f64> for Dual<S>
where
    S: BorrowMut<[f64]>,
{
    type Output = Dual<S>;
    fn add(mut self, rhs: f64) -> Dual<S> {
        self += rhs;
        self
    }
}

impl<S> ops::DivAssign<f64> for Dual<S>
where
    S: BorrowMut<[f64]>,
{
    fn div_assign(&mut self, rhs: f64) {
        self.as_slice_mut().iter_mut().for_each(|ds| *ds /= rhs);
    }
}

impl<S> ops::Div<f64> for Dual<S>
where
    S: BorrowMut<[f64]>,
{
    type Output = Dual<S>;
    fn div(mut self, rhs: f64) -> Dual<S> {
        self /= rhs;
        self
    }
}

impl<S> ops::MulAssign<f64> for Dual<S>
where
    S: BorrowMut<[f64]>,
{
    fn mul_assign(&mut self, rhs: f64) {
        self.as_slice_mut().iter_mut().for_each(|ds| *ds *= rhs);
    }
}

impl<S> ops::Mul<f64> for Dual<S>
where
    S: BorrowMut<[f64]>,
{
    type Output = Dual<S>;
    fn mul(mut self, rhs: f64) -> Dual<S> {
        self *= rhs;
        self
    }
}

impl<S> ops::SubAssign<f64> for Dual<S>
where
    S: BorrowMut<[f64]>,
{
    fn sub_assign(&mut self, rhs: f64) {
        *self.val_mut() -= rhs;
    }
}

impl<S> ops::Sub<f64> for Dual<S>
where
    S: BorrowMut<[f64]>,
{
    type Output = Dual<S>;
    fn sub(mut self, rhs: f64) -> Dual<S> {
        self -= rhs;
        self
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
        assert_eq!((x.clone() + y.view()) * y.view(), (x + 17.) * 17.);
    }

    #[test]
    fn test_scalar_rhs_div() {
        let mut x = Dual::constant(42., 2);
        let y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 0.;
        x.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() / y.view(), x / 17.);
    }

    #[test]
    fn test_diff_subneg() {
        let mut x = Dual::constant(42., 2);
        let y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 0.;
        x.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() - y.view(), x - 17.)
    }
}
