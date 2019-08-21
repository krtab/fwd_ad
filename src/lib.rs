use std::ops;

#[derive(PartialEq, Debug, Clone)]
pub struct Dual(Vec<f64>);

impl Dual {
    pub fn constant(v: f64, ndiffs: usize) -> Dual {
        let mut res = Dual(vec![0.; ndiffs + 1]);
        res.0[0] = v;
        res
    }

    pub fn val(&self) -> f64 {
        self.0[0]
    }

    pub fn val_mut(&mut self) -> &mut f64 {
        &mut self.0[0]
    }

    pub fn diffs(&self) -> &[f64] {
        &self.0[1..]
    }

    pub fn diffs_mut(&mut self) -> &mut [f64] {
        &mut self.0[1..]
    }

    pub fn ndiffs(&self) -> usize {
        self.0.len() - 1
    }
}

mod impl_ops_dual;
mod impl_ops_scalar_rhs;

impl ops::Neg for Dual {
    type Output = Dual;
    fn neg(mut self) -> Dual {
        for x in &mut self.0 {
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
        let y = Dual::constant(17., 2);
        let res = (x + &y) * &y;
        assert_eq!(res, Dual(vec![(42. + 17.) * 17., 0., 0.]));
    }

    #[test]
    fn test_size() {
        let x = Dual::constant(0., 42);
        assert_eq!(x.ndiffs(), 42);
    }
}
