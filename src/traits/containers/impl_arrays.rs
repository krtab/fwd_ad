use super::*;

macro_rules! impl_viewable_array {
    ($n:literal) => {
        reimpl_To_Owned!(<T>, [T;$n]);

        impl<F> ToView<F> for [F; $n] {
            type ViewType = [F; $n];
            fn view(&self) -> &[F; $n] {
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
