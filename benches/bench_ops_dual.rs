#[macro_use]
extern crate criterion;

use criterion::{Criterion, BatchSize};

use smol_ad::Dual;

fn bench(c: &mut Criterion) {
    let mut x = Dual::constant(42., 15);
    let mut y = Dual::constant(17., 15);
    x.diffs_mut()[0] = 1.;
    y.diffs_mut()[1] = 1.;
    c.bench_function("add",
        move |b| {
        b.iter_batched(
            || (x.clone(),y.clone()),
            move |(x2, y2)|  x2 + &y2,
            BatchSize::SmallInput
        )
    });
    let mut x = Dual::constant(42., 15);
    let mut y = Dual::constant(17., 15);
    x.diffs_mut()[0] = 1.;
    y.diffs_mut()[1] = 1.;
    c.bench_function("mul",
        move |b| {
        b.iter_batched(
            || (x.clone(),y.clone()),
            move |(x2, y2)|  x2 * &y2,
            BatchSize::SmallInput
        )
    });
}

criterion_group!(benches,bench);
criterion_main!(benches);
