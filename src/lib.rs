#![feature(external_doc)]
#![doc(include = "../Readme.md")]

use std::borrow::*;
use std::marker::PhantomData;
use std::ops;

/// A module containing marker types used to indicated whether a `Dual` can write or not in its content.
///
/// These marker types are empty struct, only deriving common traits.
pub mod owning_markers {
    use std::borrow::*;

    /// A type used to indicate read-only capability
    ///
    /// An empty struct, only deriving common traits. There isn't anything really interesting to see here.
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub struct RO;

    /// A type used to indicate read-write capability
    ///
    /// An empty struct, only deriving common traits. There isn't anything really interesting to see here.
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub struct RW;

    /// A trait to indicate whether a given container type is compatible with a given RO/RW marker.
    ///
    /// This trait is [sealed](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed)
    /// and cannot be implemented outside this crate.
    pub trait CompatibleWith<OM>: Borrow<[f64]> + private::Sealed<OM> {}
    impl<T: Borrow<[f64]>> CompatibleWith<RO> for T {}
    impl<T: BorrowMut<[f64]>> CompatibleWith<RW> for T {}

    mod private {
        use super::*;

        pub trait Sealed<OM> {}
        impl<T: Borrow<[f64]>> Sealed<RO> for T {}
        impl<T: BorrowMut<[f64]>> Sealed<RW> for T {}
    }
}

#[doc(inline)]
pub use owning_markers::CompatibleWith;
pub use owning_markers::{RO, RW};

/// The struct implementing dual numbers.
///
/// It is parametrized by a type <T> which stands for either a borrowed or an owned container,
/// and derefences to `[f64]`.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Dual<T, M>
where
    T: CompatibleWith<M>,
{
    content: T,
    ph_om: PhantomData<M>,
}

impl<T, M> From<T> for Dual<T, M>
where
    T: CompatibleWith<M>,
{
    fn from(x: T) -> Self {
        Dual {
            content: x,
            ph_om: PhantomData,
        }
    }
}

impl Dual<Vec<f64>, RW> {
    /// Generates a dual number backed by a Vec<f64> with value `value` and `ndiffs`
    /// differentials, set to 0.
    pub fn constant(value: f64, ndiffs: usize) -> Self {
        let mut res = Dual::from(vec![0.; ndiffs + 1]);
        res.content[0] = value;
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
impl<T, M> Dual<T, M>
where
    T: CompatibleWith<M>,
{
    /// Returns the content as a slice.
    pub fn as_slice(&self) -> &[f64] {
        self.content.borrow()
    }

    // TODO Implement as borrow::ToOwned
    /// Clone the borrowed content, so that the resulting Dual
    /// owns its content.
    pub fn to_owning(&self) -> Dual<Vec<f64>, RW> {
        Dual::from(self.as_slice().to_owned())
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
    pub fn is_close<S, M2>(&self, b: &Dual<S, M2>, atol: f64) -> bool
    where
        S: CompatibleWith<M2>,
    {
        self.as_slice()
            .iter()
            .zip(b.as_slice())
            .all(|(xs, xb)| (*xs - *xb).abs() <= atol)
    }

    /// Returns a non-owning Dual backed by the same container as self.
    pub fn view(&self) -> Dual<&[f64], RO> {
        Dual::from(self.as_slice())
    }
}

/// Methods for Duals that own their content
impl<T> Dual<T, RW>
where
    T: BorrowMut<[f64]>,
    T: CompatibleWith<RW>, //Implied by BorrowMut<[f64]>
{
    /// Returns a mutable slice
    pub fn as_slice_mut(&mut self) -> &mut [f64] {
        self.content.borrow_mut()
    }

    /// Return a mutable reference to the value
    pub fn val_mut(&mut self) -> &mut f64 {
        unsafe { self.as_slice_mut().get_unchecked_mut(0) }
    }

    /// Return a mutable slice of the differentials
    pub fn diffs_mut(&mut self) -> &mut [f64] {
        &mut self.as_slice_mut()[1..]
    }

    /// Returns e^self.
    pub fn exp(mut self) -> Self {
        let expval = self.val().exp();
        *self.val_mut() = expval;
        for x in self.diffs_mut() {
            *x *= expval;
        }
        self
    }

    /// Returns 2^self.
    pub fn exp2(mut self) -> Self {
        let expval = self.val().exp2();
        *self.val_mut() = expval;
        for x in self.diffs_mut() {
            *x *= 2_f64.ln() * expval;
        }
        self
    }

    /// Returns base^self.
    pub fn exp_base(mut self, base: f64) -> Self {
        let expval = base.powf(self.val());
        *self.val_mut() = expval;
        for x in self.diffs_mut() {
            *x *= base.ln() * expval;
        }
        self
    }

    /// Returns ln(self).
    pub fn ln(mut self) -> Self {
        let val = self.val();
        *self.val_mut() = val.ln();
        for x in self.diffs_mut() {
            *x /= val;
        }
        self
    }

    /// Returns 1/self.
    pub fn inv(mut self) -> Self {
        let vs = self.val();
        let svs = vs * vs;
        *self.val_mut() = 1. / vs;
        self.diffs_mut().iter_mut().for_each(|ds| *ds *= -1. / svs);
        self
    }

    /// Returns self^exp.
    pub fn powf(mut self, exp: f64) -> Self {
        let vs = self.val();
        *self.val_mut() = vs.powf(exp);
        self.diffs_mut()
            .iter_mut()
            .for_each(|ds| *ds *= exp * vs.powf(exp - 1.));
        self
    }

    /// Returns self^exp.
    pub fn powdual<S, M2>(mut self, exp: Dual<S, M2>) -> Self
    where
        S: CompatibleWith<M2>,
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

    pub fn abs(mut self) -> Self {
        if self.val() < 0. {
            self *= -1.;
        };
        self
    }
}

impl<T> ops::Neg for Dual<T, RW>
where
    T: BorrowMut<[f64]>, //Implies Compatibility with RW
{
    type Output = Self;
    fn neg(mut self) -> Self {
        for x in self.as_slice_mut() {
            *x = ops::Neg::neg(*x);
        }
        self
    }
}

// The feature gate is applied to a module because it is easier than applying it to each sub-item
#[cfg(feature = "implicit-clone")]
mod implicit_clone {
    use super::*;

    macro_rules! clone_impl {
        {$fname: ident($($param : ident : $ptype : ty),*)} => {
            pub fn $fname(&self,$($param : $ptype),*) -> Dual<Vec<f64>, RW> {
                    let res = self.to_owning();
                    res.$fname($($param),*)
            }
        }
    }

    impl<T> Dual<T, RO>
    where
        T: CompatibleWith<RO>,
    {
        clone_impl!(exp());
        clone_impl!(exp2());
        clone_impl!(exp_base(base: f64));
        clone_impl!(ln());
        clone_impl!(inv());
        clone_impl!(powf(exp: f64));
        clone_impl!(abs());

        pub fn powdual<S, M2>(self, exp: Dual<S, M2>) -> Dual<Vec<f64>, RW>
        where
            S: CompatibleWith<M2>,
        {
            let res = self.to_owning();
            res.powdual(exp)
        }
    }

    impl<T> ops::Neg for Dual<T, RO>
    where
        T: CompatibleWith<RO>,
    {
        type Output = Dual<Vec<f64>, RW>;
        fn neg(self) -> Dual<Vec<f64>, RW> {
            let res = self.to_owning();
            -res
        }
    }
}

mod generate_duals;
mod impl_ops_dual;
mod impl_ops_scalar_rhs;

#[cfg(test)]
mod tests {
    use super::*;

    fn generate() -> Dual<Vec<f64>, RW> {
        let mut x = Dual::constant(42., 3);
        x.diffs_mut()[0] = 17.;
        x.diffs_mut()[2] = -7.;
        x
    }

    #[test]
    fn test_constant() {
        let x = Dual::constant(42., 2);
        assert_eq!(x, Dual::from(vec![42., 0., 0.]));
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
