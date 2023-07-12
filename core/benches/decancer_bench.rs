use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::random;

fn cure(c: &mut Criterion) {
  let input = String::from("vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣");

  c.bench_function("cure", |b| b.iter(|| decancer::cure(&input)));
}

fn cure_char(c: &mut Criterion) {
  c.bench_function("cure_char", |b| {
<<<<<<< HEAD
    b.iter_batched(random::<char>, decancer::cure_char, BatchSize::SmallInput)
  });
}

criterion_group!(benches, cure, cure_char);
=======
    b.iter_batched(
      || random::<char>(),
      decancer::cure_char,
      BatchSize::SmallInput,
    )
  });
}

criterion_group!(benches, cure, cure_char,);
>>>>>>> 84f2ee062d4e00e3a78ab285f1d381f706f09858
criterion_main!(benches);
