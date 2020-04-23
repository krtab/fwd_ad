
Forward auto-differentiation using dual numbers.

This crate allows you to easily write operations on [dual numbers](https://en.wikipedia.org/wiki/Dual_number) and do forward automatic differentiation.

# Examples

Detailled examples are available in the `examples/` directory, but some snippets are reproduced below.

## Rosenbrock function minimization

```rust
#[macro_use]
extern crate smolad;
use smolad::*;

// The factor by which we will descend along the gradient.
// Rosenbrock function is pretty steep so its quite small.
const ALPHA : f64 = 1e-3;

fn main() {
    // Create two duals with two derivatives each, as well as
    // closures getdx and getdy to get their corresponding derivative
    generate_duals!{
        x = 0.; @ getdx
        y = 0.; @ getdy
    }
    for _ in 0..10000 {
        let xval : f64 = x.val();
        let yval : f64 = y.val();

        let res = (x.clone() - 1.).powf(2.) + 100.*(y-x.powf(2.)).powf(2.);
        println!("At x={}, y={}, the rosenbrock function is {}",xval, yval, res.val());

        generate_duals!{
            newx = xval - ALPHA*getdx(res.view());
            newy = yval - ALPHA*getdy(res.view());
        }
        x = newx;
        y = newy;
    }
}
```


# Comparision with other (forward) AD rust libraries

| crate      | version | multi-variate | higher-order | last update |
|------------|--------:|:-------------:|:------------:|------------:|
| **smolad** |   0.1.0 |       ✔️       |      ❌       |  2020-04-23 |
| ad         |   0.1.0 |       ❌       |      ❌       |  2020-01-01 |
| autodiff   |   0.1.9 |       ❌       |      ❌       |  2019-11-07 |
| descent¹   |     0.3 |       ✔️       | (2nd order?) |  2018-12-10 |
| dual       |   0.2.0 |       ❌       |      ❌       |  2015-12-25 |
| dual_num   |   0.2.7 |       ❌       |      ❌       |  2019-04-03 |
| hyperdual² |   0.3.4 |       ✔️       |      ❌       |  2020-02-08 |
| peroxide   |  0.21.7 |       ❌       | (2nd order)  |  2020-04-21 |


1. `descent` Automatic differentiation seems promising but isn't very documented and is mixed-up with the IP-OPT interface
2. `hyperdual` has similar properties to smolad, except that all operations will allocate when `smolad` tries to reuse existing memory

 