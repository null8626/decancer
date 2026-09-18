use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
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

#[cfg(feature = "leetspeak")]
fn leetspeak(c: &mut Criterion) {
  c.bench_function("leetspeak", |b| {
    b.iter(|| {
      let options = decancer::Options::default().retain_capitalization();
      let cured = decancer::cure(r"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣 wWiIiIIttHh l133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdfl133t5p3/-\|<asdfasdf", options).unwrap();

      assert_eq!(cured, "very funny text with leetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdfleetspeakasdfasdf");
    });
  });
}

#[cfg(feature = "leetspeak")]
criterion_group!(benches, cure, cure_char, leetspeak);

#[cfg(not(feature = "leetspeak"))]
criterion_group!(benches, cure, cure_char);

criterion_main!(benches);
