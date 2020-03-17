//! A crate implementing forward auto-differentiation, via dual numbers.

use std::borrow::*;
use std::ops;

/// The struct implementing dual numbers.
///
/// It is parametrized by a type <T> which stands for either a borrowed or an owned container,
/// and derefences to `[f64]`.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Dual<T: Borrow<[f64]>>(pub T);

impl Dual<Vec<f64>> {
    /// Generates a dual number backed by a Vec<f64> with value `value` and `ndiffs`
    /// differentials, set to 0.
    pub fn constant(value: f64, ndiffs: usize) -> Dual<Vec<f64>> {
        let mut res = Dual(vec![0.; ndiffs + 1]);
        res.0[0] = value;
        res
    }
}

// impl<S,T> From<Dual<T>> for Dual<S>
// where
//     S: From<T>
// {
//     fn from(dualt : Dual<T>) -> Dual<S> {
//         Dual(From::from(dualt.0))
//     }
// }

/// Implementations for Duals that do not necessarily own their content.
impl<T> Dual<T>
where
    T: Borrow<[f64]>,
{
    /// Clone the borrowed content, so that the resulting Dual
    /// owns its content.
    pub fn to_owning(&self) -> Dual<Vec<f64>> {
        Dual(self.as_slice().to_owned())
    }

    /// Returns the content as a slice.
    pub fn as_slice(&self) -> &[f64] {
        self.0.borrow()
    }

    /// Returns the value of the dual.
    pub fn val(&self) -> f64 {
        self.as_slice()[0]
    }

    /// Returns a slice of the differentials
    pub fn diffs(&self) -> &[f64] {
        &self.as_slice()[1..]
    }

    /// Return the number of differentials
    pub fn ndiffs(&self) -> usize {
        self.as_slice().len() - 1
    }

    /// Allows comparing to duals by checking whether they are elementwise within `atol` of each other.
    pub fn is_close<S>(&self, b: &Dual<S>, atol: f64) -> bool
    where
        S: Borrow<[f64]>,
    {
        self.as_slice()
            .iter()
            .zip(b.as_slice())
            .all(|(xs, xb)| (*xs - *xb).abs() <= atol)
    }

    /// Returns a non-owning Dual backed by the same container as self.
    pub fn view(&self) -> Dual<&[f64]> {
        Dual(self.as_slice())
    }
}

/// Methods for Duals that own their content
impl<T> Dual<T>
where
    T: BorrowMut<[f64]>,
{
    /// Returns a mutable slice
    pub fn as_slice_mut(&mut self) -> &mut [f64] {
        self.0.borrow_mut()
    }

    /// Return a mutable reference to the value
    pub fn val_mut(&mut self) -> &mut f64 {
        unsafe { self.as_slice_mut().get_unchecked_mut(0) }
    }

    /// Return a mutable slice of the differentials
    pub fn diffs_mut(&mut self) -> &mut [f64] {
        &mut self.as_slice_mut()[1..]
    }

    /// Returns the e^self.
    pub fn exp(mut self) -> Dual<T> {
        let expval = self.val().exp();
        *self.val_mut() = expval;
        for x in self.diffs_mut() {
            *x *= expval;
        }
        self
    }

    /// Returns ln(self).
    pub fn ln(mut self) -> Dual<T> {
        let val = self.val();
        *self.val_mut() = val.ln();
        for x in self.diffs_mut() {
            *x /= val;
        }
        self
    }

    /// Returns 1/self.
    pub fn inv(mut self) -> Dual<T> {
        let vs = self.val();
        let svs = vs * vs;
        *self.val_mut() = 1. / vs;
        self.diffs_mut().iter_mut().for_each(|ds| *ds *= -1. / svs);
        self
    }

    /// Returns self^exp.
    pub fn powf(mut self, exp: f64) -> Dual<T> {
        let vs = self.val();
        *self.val_mut() = vs.powf(exp);
        self.diffs_mut()
            .iter_mut()
            .for_each(|ds| *ds *= exp * vs.powf(exp - 1.));
        self
    }

    /// Returns self^exp.
    pub fn powdual<S>(mut self, exp: Dual<S>) -> Dual<T>
    where
        S: Borrow<[f64]>,
    {
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

mod impl_ops_dual;
mod impl_ops_scalar_rhs;

impl<T> ops::Neg for Dual<T>
where
    T: BorrowMut<[f64]>,
{
    type Output = Dual<T>;
    fn neg(mut self) -> Dual<T> {
        for x in self.as_slice_mut() {
            *x = ops::Neg::neg(*x);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate() -> Dual<Vec<f64>> {
        let mut x = Dual::constant(42., 3);
        x.diffs_mut()[0] = 17.;
        x.diffs_mut()[2] = -7.;
        x
    }

    #[test]
    fn test_constant() {
        let x = Dual::constant(42., 2);
        assert_eq!(x, Dual(vec![42., 0., 0.]));
    }

    #[test]
    fn test_size() {
        let x = Dual::constant(0., 42);
        assert_eq!(x.ndiffs(), 42);
    }

    #[test]
    fn test_neg() {
        let x = generate();
        assert_eq!(-(-x.clone()), x);
    }

    #[test]
    fn test_ln_exp() {
        let x = generate();
        assert!(x.clone().is_close(&x.clone().exp().ln(), 1e-8));
        assert!(x.clone().is_close(&x.clone().ln().exp(), 1e-8));
    }
}
