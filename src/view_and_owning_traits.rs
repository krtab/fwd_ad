use std::borrow::{Borrow, BorrowMut};

pub trait ROAble<F> {
    fn ro(&self) -> &[F];
}
pub trait RWAble<F>: ROAble<F> {
    fn rw(&mut self) -> &mut [F];
}
pub trait ToView<F>: ROAble<F> {
    type ViewType: ?Sized;
    fn view(&self) -> &Self::ViewType;
}
pub trait ToOwning<F>: ROAble<F> {
    type Owning: RWAble<F>;
    fn to_owning(&self) -> Self::Owning;
}

macro_rules! reimpl_BorrowX_RXAble {
    (<$($gen:tt),*>, $t:ty) => {
        impl<F,$($gen),*> ROAble<F> for $t
        where
            $t: Borrow<[F]>
        {
            fn ro(&self) -> &[F] {
                Borrow::borrow(self)
            }
        }
        impl<F,$($gen),*> RWAble<F> for $t
        where
            $t: BorrowMut<[F]>
        {
            fn rw(&mut self) -> &mut [F] {
                BorrowMut::borrow_mut(self)
            }
        }

        impl<F,$($gen),*> ToOwning<F> for $t
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
reimpl_BorrowX_RXAble!(<T>, Vec<T>);
reimpl_BorrowX_RXAble!(<T>, [T]);

impl<T: ?Sized, F> ROAble<F> for &T
where
    T: ROAble<F>,
{
    fn ro(&self) -> &[F] {
        (*self).ro()
    }
}

impl<F> ToView<F> for Vec<F> {
    type ViewType = [F];
    fn view(&self) -> &[F] {
        &self
    }
}

impl<F> ToOwning<F> for &[F]
where
    F: Clone,
{
    type Owning = Vec<F>;
    fn to_owning(&self) -> Self::Owning {
        self.to_vec()
    }
}

impl<T: ?Sized, F> ToView<F> for &T
where
    T: ROAble<F>,
{
    type ViewType = T;
    fn view(&self) -> &T {
        &self
    }
}

macro_rules! impl_viewable_array {
    ($n:literal) => {
        reimpl_BorrowX_RXAble!(<T>, [T;$n]);

        impl<F> ToView<F> for [F;$n] {
            type ViewType = [F;$n];
            fn view(&self) -> &[F;$n] {
                &self
            }
        }

        impl<F> ToOwning<F> for &[F;$n]
        where
            F: Clone
        {
            type Owning = [F;$n];
            fn to_owning(&self) -> [F;$n] {
                (*self).clone()
            }
        }
    };
}

impl_viewable_array!(1);
impl_viewable_array!(2);
impl_viewable_array!(3);
impl_viewable_array!(4);
impl_viewable_array!(5);
impl_viewable_array!(6);
impl_viewable_array!(7);
impl_viewable_array!(8);
impl_viewable_array!(9);
impl_viewable_array!(10);
impl_viewable_array!(11);
impl_viewable_array!(12);
impl_viewable_array!(13);
impl_viewable_array!(14);
impl_viewable_array!(15);
impl_viewable_array!(16);
impl_viewable_array!(17);
impl_viewable_array!(18);
impl_viewable_array!(19);
impl_viewable_array!(20);
impl_viewable_array!(21);
impl_viewable_array!(22);
impl_viewable_array!(23);
impl_viewable_array!(24);
impl_viewable_array!(25);
impl_viewable_array!(26);
impl_viewable_array!(27);
impl_viewable_array!(28);
impl_viewable_array!(29);
impl_viewable_array!(30);
impl_viewable_array!(31);
impl_viewable_array!(32);
