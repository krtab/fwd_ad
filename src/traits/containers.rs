use std::convert::{AsMut, AsRef};

/// A trait implemented by types which can provide read access to their content of scalars `F`.
///
/// Implemented for all types that are `std::convert::AsRef<[F]>`.
pub trait ROAble<F> {
    fn ro(&self) -> &[F];
}

impl<F, T> ROAble<F> for T
where
    T: AsRef<[F]>,
{
    fn ro(&self) -> &[F] {
        self.as_ref()
    }
}

/// A trait implemented by types which can provide write access to their content of scalars `F`.
///
/// All types that are `RWAble<F>` must be `ROAble<F>`.
///
/// Implemented for all types that are `std::convert::AsMut<[F]>` and `ROAble<F>`.
pub trait RWAble<F>: ROAble<F> {
    fn rw(&mut self) -> &mut [F];
}

impl<F, T> RWAble<F> for T
where
    T: AsMut<[F]>,
    T: ROAble<F>,
{
    fn rw(&mut self) -> &mut [F] {
        self.as_mut()
    }
}

/// A trait used to indicate the canonical view type of a given type.
pub trait ToView<F>: ROAble<F> {
    type ViewType: ?Sized;
    fn view(&self) -> &Self::ViewType;
}

impl<F> ToView<F> for Vec<F> {
    type ViewType = [F];
    fn view(&self) -> &[F] {
        &self
    }
}

impl<T: ?Sized, F> ToView<F> for &T
where
    T: ROAble<F>,
    T: AsRef<[F]>,
{
    type ViewType = T;
    fn view(&self) -> &T {
        self
    }
}

/// A trait used to indicate the canonical owning associated with a given type.
///
/// Any type that is `std::borrow::ToOwned` is `ToOwning`.
pub trait ToOwning<F>: ROAble<F> {
    type Owning: RWAble<F>;
    fn to_owning(&self) -> Self::Owning;
}

macro_rules! reimpl_To_Owned {
    (<$($gen:tt),*>, $t:ty) => {

        impl<$($gen),*,F> ToOwning<F> for $t
        where
            $t: ToOwned,
            <$t as ToOwned>::Owned : RWAble<F>,
            $t: ROAble<F>
        {
           type Owning = <$t as ToOwned>::Owned;
           fn to_owning(&self) -> Self::Owning {
               self.to_owned()
           }
        }

    };
}
// reimpl_To_Owned!(<T>, T);
reimpl_To_Owned!(<T>, Vec<T>);
reimpl_To_Owned!(<T>, [T]);
reimpl_To_Owned!(<'a, T>, &'a [T]);

// impl<F> ToOwning<F> for &[F]
// where
//     F: Clone,
// {
//     type Owning = Vec<F>;
//     fn to_owning(&self) -> Self::Owning {
//         self.to_vec()
//     }
// }

mod impl_arrays;
