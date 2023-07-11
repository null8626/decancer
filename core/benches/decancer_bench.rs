use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use decancer::{cure, cure_char};
use rand::{thread_rng, Rng};

fn cure_short(c: &mut Criterion) {
    let input = String::from("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");
  
    c.bench_function("cure short input", |b| b.iter(|| cure(&input)));
  }

fn cure_random(c: &mut Criterion) {
  c.bench_function("cure random input", |b| {
    b.iter_batched(
      || {
        thread_rng()
          .sample_iter::<char, _>(rand::distributions::Standard)
          .take(500)
          .collect::<String>()
      },
      |key| cure(&key),
      BatchSize::SmallInput,
    )
  });
}

fn cure_char_random(c: &mut Criterion) {
  c.bench_function("cure_char random input", |b| {
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

criterion_group!(
  benches,
  cure_short,
  cure_random,
  cure_char_random,
);
criterion_main!(benches);
