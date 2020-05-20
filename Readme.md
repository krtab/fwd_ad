Fwd:AD a crate for Forward Auto-Differentiation
===============================================

![CI Pipeline](https://gitlab.inria.fr/InBio/Public/fwd_ad/badges/master/pipeline.svg?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/fwd_ad?style=flat-square)

This crate allows you to easily write operations on [dual numbers](https://en.wikipedia.org/wiki/Dual_number) and do forward automatic differentiation. It empowers its user to write auto-differentiation code with minimal allocations.

## Key selling-points

 1. **Clone-free** by default. Fwd:AD will never clone memory in its functions (except `to_owning()`) and `std::ops` implementations, leveraging Rust's ownership system to ensure correctness memory-wise, and leaving it up to the user to be explicit as to when cloning should happen.
 2. **Automatic cloning** on demand. If passed the `implicit-clone` feature, Fwd:AD will implicitly clone `Dual`s when needed. Deciding whether to clone or not is entirely done via the type-system, and hence at compile time.
 3. **Generic in memory location**: Fwd:AD's structs are generic over a container type, allowing them to be backed by any container of your choice: `Vec` to rely on the heap, arrays if you're more of a stack-person, or other. For example, it can be used with `&mut [f64]` to allow an FFI API that won't need to copy memory at its frontier.

## Examples

Detailled examples are available in the `examples/` directory, but some snippets are reproduced below.

### Rosenbrock function minimization

```rust
extern crate fwd_ad;
use fwd_ad::*;

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

# Short tutorial

Fdw:AD's main type is the `Dual<Container, OM, F>` struct. This struct is parametrized by three types, which are:
 1. `Container` a type indicating what "container" is used to store the struct content. Typical examples include `Vec<F>`, `[F; n]`, `&mut [F]`, or `&[F]`.
 2. `OM` an "owning mode" which is one of two possibilities: `RW` for "read-write", indicating that the content of the Dual is write-able and hence can be reused during computations and `RO` indicating that it is read-only.
 3. `F` is the scalar type, typically `f32` or `f64`, but you chan choose to use something different.

A `Dual` wraps its container, which must be "read-able as an `[F]`". The first item of this slice of scalars is the dual's actual value and the next ones are the derivative with respect to the successive variables.

To alleviate the burden of writting out long type names, canonical pairs of owning/view duals are defined in the `instanciations` module.

## Fwd:AD Traits

Fwd:AD relies on several traits to be generic enough. Traits a user may need to implement are located in the `traits` module.

 - `ROAble` (resp. `RWAble`) are traits that should be implemented by containers which are able to read (resp. write) their content. All container types must implement `ROAble`. These traits are similar to `AsRef`/`AsMut` from `core` and a blanket implementation is provided.
 - `ToView` and `ToOwning` are traits that are used to defined correspondances of canonical "owning" (which can be `RW`) and "view" (which only have `RO` capacity) containers.
 - `Scalar` is the trait representing scalar numbers, it is merely a supertrait for various traits of `num_traits`, so these are what you should seek to implement. 

Caveat: because you can't implement external traits on external types you may find yourself limited in using duals with an uncommon container or scalar type. If so, please contact the maintainer of this crate. 


# Comparision with other (forward) AD rust libraries

The last-update column represent the last time the corresponding crate was checked. Crates may have evolved since.

| crate      | version | multi-variate | higher-order | last update |
|------------|--------:|:-------------:|:------------:|------------:|
| **Fwd:AD** |   0.1.0 |       ✔️       |      ❌       |  2020-04-29 |
| ad         |   0.1.0 |       ❌       |      ❌       |  2020-01-01 |
| autodiff   |   0.1.9 |       ❌       |      ❌       |  2019-11-07 |
| descent¹   |     0.3 |       ✔️       | (2nd order?) |  2018-12-10 |
| dual       |   0.2.0 |       ❌       |      ❌       |  2015-12-25 |
| dual_num   |   0.2.7 |       ❌       |      ❌       |  2019-04-03 |
| hyperdual² |   0.3.4 |       ✔️       |      ❌       |  2020-02-08 |
| peroxide   |  0.21.7 |       ❌       | (2nd order)  |  2020-04-21 |


1. `descent` Automatic differentiation seems promising but isn't very documented and is mixed-up with the IP-OPT interface
2. `hyperdual` has similar properties to Fwd:AD, except that all operations will allocate when Fwd:AD tries to reuse existing memory

# Acknowledgments

Fwd:AD is being developped during my PhD in the [InBio](https://research.pasteur.fr/en/team/inbio/) research team, a joint research initiative by [Inria](https://www.inria.fr/en) and [Institut Pasteur](https://www.pasteur.fr/en).