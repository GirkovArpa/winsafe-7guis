use std::fs;
use std::path::Path;

fn main() {
  const RES: &str = "resources.res";
  let mut dotlib = String::from(RES);
  dotlib.push_str(".lib");
  if Path::new(RES).exists() {
    fs::copy(RES, &dotlib).unwrap();
  }
  println!("cargo:rustc-link-lib=dylib={}", RES);
}
