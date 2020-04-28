// Use gradient descent to optimize Rosenbrok's function
// =====================================================

extern crate smolad;
use smolad::*;

// The factor by which we will descend along the gradient.
// Rosenbrock function is pretty steep so its quite small.
const ALPHA: f64 = 1e-3;

fn main() {
    // Create two duals with two derivatives each, as well as
    // closures getdx and getdy to get their corresponding derivative
    let mut x: Dual<[f64; 3], RW, f64> = Dual::from([0., 1., 0.]);
    let mut y: Dual<[f64; 3], RW, f64> = Dual::from([0., 0., 1.]);
    for _ in 0..10000 {
        // x and y will be consummed below, we need to store their value;
        let xval: f64 = x.val();
        let yval: f64 = y.val();
        // The Rosenbrock function itself
        let res = (x.clone() - 1.).powf(2.) + 100. * (y - x.powf(2.)).powf(2.);
        println!(
            "At x={}, y={}, the rosenbrock function is {}",
            xval,
            yval,
            res.val()
        );

        // We generate two new duals containing the new variables.
        // We could save one allocation by directly modifying x and y but it would be less user friendly.
        let dresdx = res.diffs()[0];
        let dresdy = res.diffs()[1];
        *x.val_mut() -= ALPHA * dresdx;
        x.diffs_mut().copy_from_slice(&[1., 0.]);
        *y.val_mut() -= ALPHA * dresdy;
        y.diffs_mut().copy_from_slice(&[0., 1.]);
    }
}
