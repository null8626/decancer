use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::random;

fn cure(c: &mut Criterion) {
  c.bench_function("cure", |b| {
    b.iter(|| decancer::cure!("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣").unwrap());
  });
}

fn cure_char(c: &mut Criterion) {
  c.bench_function("cure_char", |b| {
    b.iter_batched(
      random::<char>,
      |character| decancer::cure_char!(character),
      BatchSize::SmallInput,
    );
  });
}

criterion_group!(benches, cure, cure_char);
criterion_main!(benches);
