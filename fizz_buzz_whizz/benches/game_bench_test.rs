use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fizz_buzz_whizz;

fn fizz_buzz_whizz_bench(c: &mut Criterion) {
    c.bench_function("fizz_buzz_whizz_bench", |b| b.iter(|| fizz_buzz_whizz::play(black_box(119))));
}

criterion_group!(benches, fizz_buzz_whizz_bench);
criterion_main!(benches);