//! A module containing marker types used to indicated whether a `Dual` can write or not in its content.
//!
//! These marker types are empty struct, only deriving common traits.

use super::view_and_owning_traits::{ROAble, RWAble};

/// A type used to indicate read-only capability
///
/// An empty struct, only deriving common traits. There isn't anything really interesting to see here.
#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, Default)]
pub struct RO;

/// A type used to indicate read-write capability
///
/// An empty struct, only deriving common traits. There isn't anything Scalarly interesting to see here.
#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, Default)]
pub struct RW;

/// A trait regrouping owning mode markers
///
/// This trait is [sealed](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed)
/// and cannot be implemented outside this crate.
pub trait OwningMode: private::Sealed + Default {}
impl OwningMode for RO {}
impl OwningMode for RW {}

/// A trait to indicate whether a given container type is compatible with a given RO/RW marker.
pub trait CompatibleWith<OM: OwningMode, F> {}
/// Being read-only means containers only need to have the capability to borrow their content, not necessarily mutably.
impl<F, T: ROAble<F> + ?Sized> CompatibleWith<RO, F> for T {}
/// Being read-write means containers need to be able to mutably borrow their content.
impl<F, T: RWAble<F> + ?Sized> CompatibleWith<RW, F> for T {}

mod private {
    use super::*;

    pub trait Sealed {}
    impl Sealed for RO {}
    impl Sealed for RW {}
}
