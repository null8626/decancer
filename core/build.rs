fn main() {
  println!("cargo:rerun-if-changed=bin/bidi.bin");
  println!("cargo:rerun-if-changed=bin/codepoints.bin");
}
