#![feature(external_doc)]
#![doc(include = "../Readme.md")]

use core::marker::PhantomData;
use std::borrow::*;
use std::ops;
use num_traits;

pub mod owning_markers; 

#[doc(inline)]
pub use owning_markers::CompatibleWith;
pub use owning_markers::{OwningMode, RO, RW};

pub trait Scalar : num_traits::real::Real + num_traits::NumAssignOps + num_traits::NumAssignRef + num_traits::NumRef {
    const ZERO : Self;
    const ONE : Self;
    const LN_OF2 : Self;
}

impl Scalar for f64 
{
    const ZERO : Self = 0.;
    const ONE : Self = 1.;
    const LN_OF2 : Self = std::f64::consts::LN_2;
}

impl Scalar for f32 
{
    const ZERO : Self = 0.;
    const ONE : Self = 1.;
    const LN_OF2 : Self = std::f32::consts::LN_2;
}

/// The struct implementing dual numbers.
///
/// It is parametrized by a type <T> which stands for either a borrowed or an owned container,
/// and derefences to `[f64]`.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Dual<T, M, F>
where
    M: OwningMode,
    T: CompatibleWith<M,F>,
    F: Scalar
{
    content: T,
    om: M,
    ph_f : PhantomData<F>
}

impl<T, M, F> From<T> for Dual<T, M, F>
where
    M: OwningMode,
    T: CompatibleWith<M, F>,
    F: Scalar
{
    fn from(x: T) -> Self {
        Dual {
            content: x,
            om: M::default(),
            ph_f: PhantomData
        }
    }
}

impl<F> Dual<Vec<F>, RW, F> 
where
    F: Scalar,
{
    /// Generates a dual number backed by a Vec<F> with value `value` and `ndiffs`
    /// differentials, set to 0.
    pub fn constant(value: F, ndiffs: usize) -> Self {
        let mut res = Dual::from(vec![F::ZERO; ndiffs + 1]);
        res.content[0] = value;
        res
    }
}

/// Implementations for Duals that do not necessarily own their content.
impl<T, M, F> Dual<T, M, F>
where
    M: OwningMode,
    T: Borrow<[F]>,
    T: CompatibleWith<M, F>,
    F: Scalar
{
    /// Returns the content as a slice.
    pub fn as_slice(&self) -> &[F] {
        self.content.borrow()
    }

    // TODO Implement as borrow::ToOwned
    /// Clone the borrowed content, so that the resulting Dual
    /// owns its content.
    pub fn to_owning(&self) -> Dual<Vec<F>, RW, F> {
        Dual::from(self.as_slice().to_owned())
    }

    /// Returns the value of the dual.
    pub fn val(&self) -> F {
        self.as_slice()[0]
    }

    /// Returns a slice of the differentials
    pub fn diffs(&self) -> &[F] {
        &self.as_slice()[1..]
    }

    /// Return the number of differentials
    pub fn ndiffs(&self) -> usize {
        self.as_slice().len() - 1
    }

    /// Allows comparing to duals by checking whether they are elementwise within `atol` of each other.
    pub fn is_close<S, M2>(&self, b: &Dual<S, M2, F>, atol: F) -> bool
    where
        M2: OwningMode,
        S: Borrow<[F]>,
        S: CompatibleWith<M2, F>,
    {
        self.as_slice()
            .iter()
            .zip(b.as_slice())
            .all(|(xs, xb)| (*xs - *xb).abs() <= atol)
    }

    /// Returns a non-owning Dual backed by the same container as self.
    pub fn view<'a, S: ?Sized>(&'a self) -> Dual<&'a S, RO, F>
    where
        T: Borrow<S>,
        &'a S: CompatibleWith<RO, F>,
    {
        Dual::from(self.content.borrow())
    }
}

/// Methods for Duals that own their content
impl<T, F> Dual<T, RW, F>
where
    T: BorrowMut<[F]>,
    T: CompatibleWith<RW, F>,
    F : Scalar,
{
    /// Returns a mutable slice
    pub fn as_slice_mut(&mut self) -> &mut [F] {
        self.content.borrow_mut()
    }

    /// Return a mutable reference to the value
    pub fn val_mut(&mut self) -> &mut F {
        unsafe { self.as_slice_mut().get_unchecked_mut(0) }
    }

    /// Return a mutable slice of the differentials
    pub fn diffs_mut(&mut self) -> &mut [F] {
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
            *x *= F::LN_OF2 * expval;
        }
        self
    }

    /// Returns base^self.
    pub fn exp_base(mut self, base: F) -> Self {
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
        *self.val_mut() = F::ONE / vs;
        self.diffs_mut().iter_mut().for_each(|ds| *ds *= -F::ONE / svs);
        self
    }

    /// Returns self^exp.
    pub fn powf(mut self, exp: F) -> Self {
        let vs = self.val();
        *self.val_mut() = vs.powf(exp);
        self.diffs_mut()
            .iter_mut()
            .for_each(|ds| *ds *= exp * vs.powf(exp - F::ONE));
        self
    }

    /// Returns self^exp.
    pub fn powdual<S, M2>(mut self, exp: Dual<S, M2, F>) -> Self
    where
        M2: OwningMode,
        S: Borrow<[F]>,
        S: CompatibleWith<M2, F>,
    {
        let vs = self.val();
        if vs == F::ZERO {
            for ds in self.diffs_mut() {
                *ds = F::ZERO
            }
            return self;
        }
        let ve = exp.val();
        *self.val_mut() = vs.powf(ve);
        self.diffs_mut()
            .iter_mut()
            .zip(exp.diffs())
            .for_each(|(ds, de)| *ds = vs.powf(ve - F::ONE) * (vs * de * vs.ln() + ve * *ds));
        self
    }

    pub fn abs(mut self) -> Self {
        if self.val() < F::ZERO {
            self *= -F::ONE;
        };
        self
    }
}

impl<T, F> ops::Neg for Dual<T, RW, F>
where
    T: BorrowMut<[F]>, //Implies Compatibility with RW
    T: CompatibleWith<RW, F>,
    F : Scalar
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
            pub fn $fname(&self,$($param : $ptype),*) -> Dual<Vec<F>, RW, F> {
                    let res = self.to_owning();
                    res.$fname($($param),*)
            }
        }
    }

    impl<T, F> Dual<T, RO, F>
    where
        T: Borrow<[F]>,
        T: CompatibleWith<RO, F>,
        F: Scalar,
    {
        clone_impl!(exp());
        clone_impl!(exp2());
        clone_impl!(exp_base(base: F));
        clone_impl!(ln());
        clone_impl!(inv());
        clone_impl!(powf(exp: F));
        clone_impl!(abs());

        pub fn powdual<S, M2>(self, exp: Dual<S, M2, F>) -> Dual<Vec<F>, RW, F>
        where
            M2: OwningMode,
            S: Borrow<[F]>,
            S: CompatibleWith<M2, F>,
        {
            let res = self.to_owning();
            res.powdual(exp)
        }
    }

    impl<T, F> ops::Neg for Dual<T, RO, F>
    where
        T: Borrow<[F]>,
        T: CompatibleWith<RO, F>,
        F: Scalar
    {
        type Output = Dual<Vec<F>, RW, F>;
        fn neg(self) -> Dual<Vec<F>, RW, F> {
            let res = self.to_owning();
            -res
        }
    }
}

mod generate_duals;
mod impl_ops_dual;
mod impl_ops_scalar_rhs;

pub mod instanciations {


    pub mod vecf64 {
        use super::super::*;
        pub type Owning = Dual<Vec<f64>,RW,f64>;
        pub type View<'a> = Dual<&'a [f64], RO, f64>;
    }

    pub mod vecf32 {
        use super::super::*;
        pub type Owning = Dual<Vec<f32>, RW, f32>;
        pub type View<'a> = Dual<&'a [f32], RO, f32>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate() -> Dual<Vec<f64>, RW, f64> {
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
