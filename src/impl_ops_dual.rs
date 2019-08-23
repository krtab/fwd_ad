use crate::Dual;
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

impl ops::AddAssign<&Dual> for Dual {
    fn add_assign(&mut self, rhs: &Dual) {
        check_same_ndiffs!(self, rhs);
        self.0.iter_mut().zip(&rhs.0).for_each(|(ds, dr)| *ds += dr);
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
    fn div_assign(&mut self, rhs: &Dual) {
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

impl ops::Div<&Dual> for Dual {
    type Output = Dual;
    fn div(mut self, rhs: &Dual) -> Dual {
        self /= rhs;
        self
    }
}

impl ops::MulAssign<&Dual> for Dual {
    fn mul_assign(&mut self, rhs: &Dual) {
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

impl ops::Mul<&Dual> for Dual {
    type Output = Dual;
    fn mul(mut self, rhs: &Dual) -> Dual {
        self *= rhs;
        self
    }
}

impl ops::SubAssign<&Dual> for Dual {
    fn sub_assign(&mut self, rhs: &Dual) {
        check_same_ndiffs!(self, rhs);
        self.0.iter_mut().zip(&rhs.0).for_each(|(ds, dr)| *ds -= dr);
    }
}

impl ops::Sub<&Dual> for Dual {
    type Output = Dual;
    fn sub(mut self, rhs: &Dual) -> Dual {
        self -= rhs;
        self
    }
}

impl Dual {
    pub fn inv(mut self) -> Dual {
        let vs = self.val();
        let svs = vs * vs;
        *self.val_mut() = 1. / vs;
        self.diffs_mut().iter_mut().for_each(|ds| *ds *= -1. / svs);
        self
    }

    pub fn powf(mut self, exp: f64) -> Dual {
        let vs = self.val();
        *self.val_mut() = vs.powf(exp);
        self.diffs_mut()
            .iter_mut()
            .for_each(|ds| *ds *= exp * vs.powf(exp - 1.));
        self
    }

    pub fn powdual(mut self, exp: &Dual) -> Dual {
        let vs = self.val();
        if vs == 0. {
            for ds in self.diffs_mut() {
                *ds = 0.
            }
            return self;
        }
        let ve = exp.val();
        *self.val_mut() = vs.powf(ve);
        self.diffs_mut()
            .iter_mut()
            .zip(exp.diffs())
            .for_each(|(ds, de)| *ds = vs.powf(ve - 1.) * (vs * de * vs.ln() + ve * *ds));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_pair() -> (Dual, Dual) {
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
        let x = Dual::constant(42., 2);
        x + &y;
        let x = Dual::constant(42., 2);
        x * &y;
        let x = Dual::constant(42., 2);
        x / &y;
        let x = Dual::constant(42., 2);
        x - &y;
    }

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
    fn test_diff_div_inv() {
        let (x, y) = generate_pair();
        let res1 = x.clone() / &y;
        let res2 = x * &y.inv();
        assert!(res1.is_close(&res2, 1e-8));
    }

    #[test]
    fn test_powf() {
        let (x, y) = generate_pair();
        let res1 = x.clone() * &x;
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
        assert_eq!(x.clone() - &y, x + &(-y))
    }

    #[test]
    fn test_powd() {
        let x = Dual(vec![3., 1.]);
        assert!(x
            .clone()
            .powdual(&x)
            .is_close(&Dual(vec![27., 27. * (3_f64.ln() + 1.)]), 1e-8))
    }
}
