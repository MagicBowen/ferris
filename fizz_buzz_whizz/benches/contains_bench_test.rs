use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn contains_digit(number: u32, digit: u32) -> bool {
    number
        .to_string()
        .chars()
        .any(|c| c.to_digit(10) == Some(digit))
}

fn contains_digit_loop(number: u32, digit: u32) -> bool {
    let mut n = number;
    while n > 0 {
        if n % 10 == digit {
            return true;
        }
        n /= 10;
    }
    false
}

fn contains_digit_bench(c: &mut Criterion) {
    c.bench_function("contains_digit", |b| {
        b.iter(|| contains_digit(black_box(12345), black_box(5)))
    });
}

fn contains_digit_loop_bench(c: &mut Criterion) {
    c.bench_function("contains_digit", |b| {
        b.iter(|| contains_digit_loop(black_box(12345), black_box(5)))
    });
}

criterion_group!(benches, contains_digit_bench, contains_digit_loop_bench);
criterion_main!(benches);
