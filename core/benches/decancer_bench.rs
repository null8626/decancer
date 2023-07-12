use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use decancer::{cure, cure_char};
use rand::random;

/// Tests the speed of curing a short constant `String`.
fn cure_short(c: &mut Criterion) {
  let input = String::from("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  c.bench_function("cure short input", |b| b.iter(|| cure(&input)));
}

/// Tests the speed of curing a random input `String` with 500 characters.
fn cure_random(c: &mut Criterion) {
  c.bench_function("cure random input", |b| {
    b.iter_batched(
      || String::from_iter([random::<char>(); 500].iter()),
      |key| cure(&key),
      BatchSize::SmallInput,
    )
  });
}

/// Tests the speed of curing indvidual random characters using `cure_char`.
fn cure_char_random(c: &mut Criterion) {
  c.bench_function("cure_char random input", |b| {
    b.iter_batched(|| random::<char>(), cure_char, BatchSize::SmallInput)
  });
}

criterion_group!(benches, cure_short, cure_random, cure_char_random,);
criterion_main!(benches);
