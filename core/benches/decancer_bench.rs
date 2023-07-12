use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::random;

fn cure(c: &mut Criterion) {
  let input = String::from("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  c.bench_function("cure", |b| b.iter(|| decancer::cure(&input)));
}

fn cure_char(c: &mut Criterion) {
  c.bench_function("cure_char", |b| {
    b.iter_batched(
      || random::<char>(),
      decancer::cure_char,
      BatchSize::SmallInput,
    )
  });
}

criterion_group!(benches, cure, cure_char,);
criterion_main!(benches);
