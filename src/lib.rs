#![cfg_attr(
    all(doc, not(doctest)),
    feature(external_doc),
    doc(include = "../Readme.md")
)]

use core::marker::PhantomData;
use std::ops;

pub mod traits;
use traits::Scalar;
use traits::{ROAble, RWAble, ToOwning, ToView};

pub mod owning_markers;
pub use owning_markers::{CompatibleWith, OwningMode, RO, RW};

/// The struct implementing dual numbers.
///
/// It is parametrized by a type <T> which stands for either a borrowed or an owned container,
/// and derefences to `[f64]`.
///
/// # Creating Duals
/// ## From a already existing container
/// To create a Dual based on a container `c`, use `Dual::from(c)`.
/// See crate-level documentation for more information on how the container values are interpreted.
/// ## Create a constant (derivatives equal to zero) dual
/// A `constant` method is provided for RW `Dual`s backed by a `Vec` or an array (up to size 32).
/// To prevent cluttering, the array implementations documentation has been hidden.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Dual<T, M, F>
where
    M: OwningMode,
    T: CompatibleWith<M, F>,
    F: Scalar,
{
    content: T,
    om: M,
    ph_f: PhantomData<F>,
}

impl<T, M, F> From<T> for Dual<T, M, F>
where
    M: OwningMode,
    T: CompatibleWith<M, F>,
    F: Scalar,
{
    fn from(x: T) -> Self {
        Dual {
            content: x,
            om: M::default(),
            ph_f: PhantomData,
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
        let mut res = Dual::from(vec![F::zero(); ndiffs + 1]);
        res.content[0] = value;
        res
    }
}

mod array_constant_impl {
    use super::{Dual, Scalar, RW};
    macro_rules! impl_array {
        ($n:literal) => {
            #[doc(hidden)]
            impl<F> Dual<[F; $n], RW, F>
            where
                F: Scalar,
            {
                pub fn constant(value: F, ndiffs: usize) -> Self {
                    assert_eq!(ndiffs + 1, $n);
                    let mut res = Dual::from([F::zero(); $n]);
                    res.content[0] = value;
                    res
                }
            }
        };
    }
    impl_array!(1);
    impl_array!(2);
    impl_array!(3);
    impl_array!(4);
    impl_array!(5);
    impl_array!(6);
    impl_array!(7);
    impl_array!(8);
    impl_array!(9);
    impl_array!(10);
    impl_array!(11);
    impl_array!(12);
    impl_array!(13);
    impl_array!(14);
    impl_array!(15);
    impl_array!(16);
    impl_array!(17);
    impl_array!(18);
    impl_array!(19);
    impl_array!(20);
    impl_array!(21);
    impl_array!(22);
    impl_array!(23);
    impl_array!(24);
    impl_array!(25);
    impl_array!(26);
    impl_array!(27);
    impl_array!(28);
    impl_array!(29);
    impl_array!(30);
    impl_array!(31);
    impl_array!(32);
}

/// Implementations for Duals that do not necessarily own their content.
impl<T, M, F> Dual<T, M, F>
where
    M: OwningMode,
    T: ROAble<F>,
    T: CompatibleWith<M, F>,
    F: Scalar,
{
    /// Clone the borrowed content, so that the resulting Dual
    /// owns its content.
    pub fn to_owning(&self) -> Dual<T::Owning, RW, F>
    where
        T: ToOwning<F>,
    {
        Dual::from(self.content.to_owning())
    }

    /// Returns the content as a slice.
    ///
    /// ```
    /// # use fwd_ad::*;
    /// let d = Dual::<_,RW,f32>::from([17.,0.,0.]);
    /// assert_eq!(d.as_slice()[0], d.val());
    /// assert_eq!(&d.as_slice()[1..], d.diffs())
    /// ```
    pub fn as_slice(&self) -> &[F] {
        self.content.ro()
    }

    /// Returns the value of the dual.
    ///     
    /// ```
    /// # use fwd_ad::*;
    /// let d = Dual::<_,RW,f32>::from([17.,0.,0.]);
    /// assert_eq!(d.val(), 17.);
    /// ```
    pub fn val(&self) -> F {
        self.as_slice()[0]
    }

    /// Returns a slice of the differentials.
    ///     
    /// ```
    /// # use fwd_ad::*;
    /// let d = Dual::<_,RW,f32>::from([17.,1.,2.]);
    /// assert_eq!(d.diffs(), &[1.,2.]);
    /// ```
    pub fn diffs(&self) -> &[F] {
        &self.as_slice()[1..]
    }

    /// Return the number of differentials.
    ///     
    /// ```
    /// # use fwd_ad::*;
    /// let d = Dual::<_,RW,f32>::from([17.,1.,2.]);
    /// assert_eq!(d.ndiffs(), 2);
    /// ```
    pub fn ndiffs(&self) -> usize {
        self.as_slice().len() - 1
    }

    /// Allows comparing to duals by checking whether they are elementwise within `atol` of each other.
    ///     
    /// ```
    /// # use fwd_ad::*;
    /// let d1 = Dual::<_,RW,f64>::from([17.,1.,2.]);
    /// let d2 = Dual::<_,RW,f64>::from([17.+1e-10,1.-1e-10,2.]);
    /// assert_eq!(d1.is_close(&d2, 1e-9),true);
    /// assert_eq!(d1.is_close(&d2, 1e-11),false);
    /// ```
    pub fn is_close<S, M2>(&self, b: &Dual<S, M2, F>, atol: F) -> bool
    where
        M2: OwningMode,
        S: ROAble<F>,
        S: CompatibleWith<M2, F>,
    {
        self.as_slice()
            .iter()
            .zip(b.as_slice())
            .all(|(xs, xb)| (*xs - *xb).abs() <= atol)
    }

    /// Returns a non-owning Dual backed by the ViewType of self.
    ///     
    /// ```
    /// # use fwd_ad::*;
    /// let d1 = Dual::<[f64;3],RW,f64>::from([17.,1.,2.]);
    /// let d2 = Dual::<&[f64;3],RO,f64>::from(&[17.,1.,2.]);
    /// assert_eq!(d1.view(),d2);
    /// ```
    pub fn view<'a>(&'a self) -> Dual<&'a T::ViewType, RO, F>
    where
        T: ToView<F>,
        &'a T::ViewType: CompatibleWith<RO, F>,
    {
        Dual::from(self.content.view())
    }
}

/// Methods for Duals that own their content
impl<T, F> Dual<T, RW, F>
where
    T: RWAble<F>,
    F: Scalar,
{
    /// Returns the content a mutable slice.
    ///
    /// ```
    /// # use fwd_ad::*;
    /// let mut d = Dual::<_,RW,f32>::from([17.,0.,0.]);
    /// assert_eq!(&mut d.clone().as_slice_mut()[0], d.val_mut());
    /// assert_eq!(&d.clone().as_slice_mut()[1..], d.diffs_mut())
    /// ```
    pub fn as_slice_mut(&mut self) -> &mut [F] {
        self.content.rw()
    }

    /// Return a mutable reference to the value.
    ///
    /// ```
    /// # use fwd_ad::*;
    /// let mut d = Dual::<_,RW,f32>::from([17.,0.,0.]);
    /// *d.val_mut() = 42.;
    /// assert_eq!(d, Dual::<_,RW,f32>::from([42.,0.,0.]))
    /// ```
    pub fn val_mut(&mut self) -> &mut F {
        unsafe { self.as_slice_mut().get_unchecked_mut(0) }
    }

    /// Return a mutable slice of the differentials.
    ///
    /// ```
    /// # use fwd_ad::*;
    /// let mut d = Dual::<_,RW,f32>::from([17.,0.,0.]);
    /// d.diffs_mut()[0] = -1.;
    /// assert_eq!(d, Dual::<_,RW,f32>::from([17.,-1.,0.]))
    /// ```
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
            *x *= F::LN_2() * expval;
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
        *self.val_mut() = F::one() / vs;
        self.diffs_mut()
            .iter_mut()
            .for_each(|ds| *ds *= -F::one() / svs);
        self
    }

    /// Returns self^exp.
    pub fn powf(mut self, exp: F) -> Self {
        let vs = self.val();
        *self.val_mut() = vs.powf(exp);
        self.diffs_mut()
            .iter_mut()
            .for_each(|ds| *ds *= exp * vs.powf(exp - F::one()));
        self
    }

    /// Returns self^exp.
    pub fn powdual<S, M2>(mut self, exp: Dual<S, M2, F>) -> Self
    where
        M2: OwningMode,
        S: ROAble<F>,
        S: CompatibleWith<M2, F>,
    {
        let vs = self.val();
        if vs == F::zero() {
            for ds in self.diffs_mut() {
                *ds = F::zero()
            }
            return self;
        }
        let ve = exp.val();
        *self.val_mut() = vs.powf(ve);
        self.diffs_mut()
            .iter_mut()
            .zip(exp.diffs())
            .for_each(|(ds, de)| *ds = vs.powf(ve - F::one()) * (vs * de * vs.ln() + ve * *ds));
        self
    }

    pub fn abs(self) -> Self {
        let v = self.val();
        if v < F::zero() {
            -self
        } else {
            self
        }
    }
}

impl<T, F> ops::Neg for Dual<T, RW, F>
where
    T: RWAble<F>,
    F: Scalar,
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
            pub fn $fname(&self,$($param : $ptype),*) -> Dual<T::Owning, RW, F> {
                    let res = self.to_owning();
                    res.$fname($($param),*)
            }
        }
    }

    impl<T, F> Dual<T, RO, F>
    where
        T: ToOwning<F>,
        F: Scalar,
    {
        clone_impl!(exp());
        clone_impl!(exp2());
        clone_impl!(exp_base(base: F));
        clone_impl!(ln());
        clone_impl!(inv());
        clone_impl!(powf(exp: F));
        clone_impl!(abs());

        pub fn powdual<S, M2>(self, exp: Dual<S, M2, F>) -> Dual<T::Owning, RW, F>
        where
            M2: OwningMode,
            S: ROAble<F>,
            S: CompatibleWith<M2, F>,
        {
            let res = self.to_owning();
            res.powdual(exp)
        }
    }

    impl<T, F> ops::Neg for Dual<T, RO, F>
    where
        T: ToOwning<F>,
        F: Scalar,
    {
        type Output = Dual<T::Owning, RW, F>;
        fn neg(self) -> Self::Output {
            let res = self.to_owning();
            -res
        }
    }
}

mod generate_duals;
mod impl_ops_dual;
mod impl_ops_scalar_rhs;

pub mod instanciations;

#[cfg(test)]
mod tests {
    use super::instanciations::vecf64::Owning;
    use super::*;

    fn generate() -> Dual<Vec<f64>, RW, f64> {
        let mut x = Owning::constant(42., 3);
        x.diffs_mut()[0] = 17.;
        x.diffs_mut()[2] = -7.;
        x
    }

    #[test]
    fn test_constant() {
        let x = Owning::constant(42., 2);
        assert_eq!(x, Owning::from(vec![42., 0., 0.]));
    }

    #[test]
    fn test_size() {
        let x = Owning::constant(0., 42);
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
