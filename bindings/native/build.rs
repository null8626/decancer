#[cfg(windows)]
fn embed() {
  println!("cargo:rerun-if-changed=decancer.rc");

  let version = env!("CARGO_PKG_VERSION");
  let version_digits = version.split('.').collect::<Vec<_>>();

  let rc_version_major = format!("DECANCER_RC_VERSION_MAJOR={}", version_digits[0]);
  let rc_version_minor = format!("DECANCER_RC_VERSION_MINOR={}", version_digits[1]);
  let rc_version_patch = format!("DECANCER_RC_VERSION_PATCH={}", version_digits[2]);
  let rc_version = format!("DECANCER_RC_VERSION=\"{version}\"");

  embed_resource::compile(
    "decancer.rc",
    [
      &rc_version_major,
      &rc_version_minor,
      &rc_version_patch,
      &rc_version,
    ],
  );
}

fn main() {
  #[cfg(windows)]
  embed();
}
