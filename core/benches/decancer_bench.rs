use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use decancer::{cure, cure_char};
use rand::{thread_rng, Rng};

fn cure_random(c: &mut Criterion) {
  c.bench_function("cure_random", |b| {
    b.iter_batched(
      || {
        thread_rng()
          .sample_iter::<char, _>(rand::distributions::Standard)
          .take(100)
          .collect::<String>()
      },
      |key| cure(&key),
      BatchSize::SmallInput,
    )
  });
}

fn cure_char_random(c: &mut Criterion) {
  c.bench_function("cure_char_random", |b| {
    b.iter_batched(
      || {
        thread_rng()
          .sample_iter::<char, _>(rand::distributions::Standard)
          .next()
          .unwrap()
      },
      cure_char,
      BatchSize::SmallInput,
    )
  });
}

fn cure_speed(c: &mut Criterion) {
  let input = String::from("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  c.bench_function("cure_speed", |b| b.iter(|| cure(&input)));
}

fn cure_char_speed(c: &mut Criterion) {
  let input = '✅';

  c.bench_function("cure_char_speed", |b| b.iter(|| cure_char(input)));
}

criterion_group!(
  benches,
  cure_random,
  cure_char_random,
  cure_speed,
  cure_char_speed
);
criterion_main!(benches);
