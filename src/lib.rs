use std::borrow::*;
use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Dual<T>(pub T);

impl Dual<Vec<f64>> {
    pub fn constant(v: f64, ndiffs: usize) -> Dual<Vec<f64>> {
        let mut res = Dual(vec![0.; ndiffs + 1]);
        res.0[0] = v;
        res
    }

    pub fn to_vec(self) -> Vec<f64> {
        self.0
    }
}

impl<S> Dual<S>
where
    S : Borrow<[f64]>
    {
        pub fn to_owning(&self) -> Dual<Vec<f64>> {
            Dual(self.as_slice().to_owned())
        }

        pub fn to_owning_default<T>(&self) -> Dual<T>
        where
            T : std::default::Default,
            T : BorrowMut<[f64]>
        {
            let mut res = Dual(std::default::Default::default());
            res.as_slice_mut().copy_from_slice(&self.as_slice());
            res

        }
    }

impl<T> Dual<T>
where
    T: Borrow<[f64]>,
{
    pub fn as_slice(&self) -> &[f64] {
        self.0.borrow()
    }

    pub fn val(&self) -> f64 {
        self.as_slice()[0]
    }

    pub fn diffs(&self) -> &[f64] {
        &self.as_slice()[1..]
    }

    pub fn ndiffs(&self) -> usize {
        self.as_slice().len() - 1
    }

    pub fn is_close<S>(&self, b: &Dual<S>, atol: f64) -> bool
    where
        S: Borrow<[f64]>,
    {
        self.as_slice()
            .iter()
            .zip(b.as_slice())
            .all(|(xs, xb)| (*xs - *xb).abs() <= atol)
    }

    pub fn view(&self) -> Dual<&[f64]> {
        Dual(self.as_slice())
    }
}

impl<T> Dual<T>
where
    T: BorrowMut<[f64]>,
{
    pub fn as_slice_mut(&mut self) -> &mut [f64] {
        self.0.borrow_mut()
    }

    pub fn val_mut(&mut self) -> &mut f64 {
        unsafe { self.as_slice_mut().get_unchecked_mut(0) }
    }

    pub fn diffs_mut(&mut self) -> &mut [f64] {
        &mut self.as_slice_mut()[1..]
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
}
