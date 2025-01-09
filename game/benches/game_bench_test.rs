use criterion::{black_box, criterion_group, criterion_main, Criterion};
use game;

fn game_bench(c: &mut Criterion) {
    c.bench_function("game_bench", |b| {
        b.iter(|| game::play_result(black_box(119)))
    });
}

criterion_group!(benches, game_bench);
criterion_main!(benches);
