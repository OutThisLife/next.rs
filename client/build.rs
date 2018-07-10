use std::env;
use std::process::Command;

fn main() {
  if let Ok(_) = env::var("COMPILING_UNDER_CARGO_WEB") {
    Command::new("cargo-web build");
  }

  println!("cargo:rerun-if-changed=\"src/lib.rs\"");
}
