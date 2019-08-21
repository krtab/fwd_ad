use crate::Dual;
use std::ops;

impl ops::AddAssign<f64> for Dual {
    fn add_assign(&mut self, rhs: f64) -> () {
        *self.val_mut() += rhs;
    }
}

impl ops::Add<f64> for Dual {
    type Output = Dual;
    fn add(mut self, rhs: f64) -> Dual {
        self += rhs;
        self
    }
}

impl ops::DivAssign<f64> for Dual {
    fn div_assign(&mut self, rhs: f64) -> () {
        self.0.iter_mut().for_each(|ds| *ds /= rhs);
    }
}

impl ops::Div<f64> for Dual {
    type Output = Dual;
    fn div(mut self, rhs: f64) -> Dual {
        self /= rhs;
        self
    }
}

impl ops::MulAssign<f64> for Dual {
    fn mul_assign(&mut self, rhs: f64) -> () {
        self.0.iter_mut().for_each(|ds| *ds *= rhs);
    }
}

impl ops::Mul<f64> for Dual {
    type Output = Dual;
    fn mul(mut self, rhs: f64) -> Dual {
        self *= rhs;
        self
    }
}

impl ops::SubAssign<f64> for Dual {
    fn sub_assign(&mut self, rhs: f64) -> () {
        *self.val_mut() -= rhs;
    }
}

impl ops::Sub<f64> for Dual {
    type Output = Dual;
    fn sub(mut self, rhs: f64) -> Dual {
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
        assert_eq!((x.clone() + &y) * &y, (x + 17.) * 17.);
    }

    #[test]
    fn test_scalar_rhs_div() {
        let mut x = Dual::constant(42., 2);
        let y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 0.;
        x.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() / &y, x / 17.);
    }

    #[test]
    fn test_diff_subneg() {
        let mut x = Dual::constant(42., 2);
        let y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 0.;
        x.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() - &y, x - 17.)
    }
}
