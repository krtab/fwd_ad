use crate::Dual;
use std::ops;

impl ops::AddAssign<&Dual> for Dual {
    fn add_assign(&mut self, rhs: &Dual)  {
        self.0
            .iter_mut()
            .zip(&rhs.0)
            .for_each(|(ds, dr)| *ds += dr);
    }
}

impl ops::Add<&Dual> for Dual {
    type Output = Dual;
    fn add(mut self, rhs: &Dual) -> Dual {
        self += rhs;
        self
    }
}

impl ops::DivAssign<&Dual> for Dual {
    fn div_assign(&mut self, rhs: &Dual)  {
        let vs = self.val();
        let vr = rhs.val();
        *self.val_mut() /= vr;
        self.diffs_mut()
            .iter_mut()
            .zip(rhs.diffs())
            .for_each(|(ds, dr)| *ds = (*ds - dr * vs / vr) / vr);
    }
}

impl ops::Div<&Dual> for Dual {
    type Output = Dual;
    fn div(mut self, rhs: &Dual) -> Dual {
        self /= rhs;
        self
    }
}

impl ops::MulAssign<&Dual> for Dual {
    fn mul_assign(&mut self, rhs: &Dual)  {
        let vs = self.val();
        let vr = rhs.val();
        *self.val_mut() *= vr;
        self.diffs_mut()
            .iter_mut()
            .zip(rhs.diffs())
            .for_each(|(ds, dr)| *ds = vs * dr + vr * *ds);
    }
}

impl ops::Mul<&Dual> for Dual {
    type Output = Dual;
    fn mul(mut self, rhs: &Dual) -> Dual {
        self *= rhs;
        self
    }
}

impl ops::SubAssign<&Dual> for Dual {
    fn sub_assign(&mut self, rhs: &Dual)  {
        self.0
            .iter_mut()
            .zip(&rhs.0)
            .for_each(|(ds, dr)| *ds -= dr);
    }
}

impl ops::Sub<&Dual> for Dual {
    type Output = Dual;
    fn sub(mut self, rhs: &Dual) -> Dual {
        self -= rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_add_mul() {
        let mut x = Dual::constant(42., 2);
        let mut y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        let res = (x + &y) * &y;
        assert_eq!(res, Dual(vec![(42. + 17.) * 17., 17., 2. * 17. + 42.]));
    }

    #[test]
    fn test_diff_div() {
        let mut x = Dual::constant(42., 2);
        let mut y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        let res = x / &y;
        assert_eq!(res, Dual(vec![42. / 17., 1. / 17., -42. / (17. * 17.)]));
    }

    #[test]
    fn test_diff_subneg() {
        let mut x = Dual::constant(42., 2);
        let mut y = Dual::constant(17., 2);
        x.diffs_mut()[0] = 1.;
        y.diffs_mut()[1] = 1.;
        assert_eq!(x.clone() - &y, x + &(-y))
    }
}
