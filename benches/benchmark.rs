#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use prose::{FormatOpts, Reformatter};

fn bench_reformatting(c: &mut Criterion) {
    c.bench_function("analysis", |b| {
        let data = include_str!("../tests/data/inputs/plain.txt");
        let opts = FormatOpts::with_max_length(40);

        b.iter(|| Reformatter::new(&opts, black_box(data)));
    });
    c.bench_function("reformat", |b| {
        let data = include_str!("../tests/data/inputs/plain.txt");
        let opts = FormatOpts::with_max_length(40);
        let reformatter = black_box(Reformatter::new(&opts, data));

        b.iter(|| reformatter.reformatted());
    });
}

criterion_group!(benches, bench_reformatting);
criterion_main!(benches);
