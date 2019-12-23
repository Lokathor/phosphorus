#![allow(clippy::cognitive_complexity)]

//! `phosphorus` CLI program.
//!
//! Currently just a demo program to test with!

fn main() {
  let bytes_vec = std::fs::read("xml/gl.xml").unwrap();
  let string = String::from_utf8(bytes_vec).unwrap();

  println!(
    "registry> {:#?}",
    phosphorus::GlRegistry::from_gl_xml(&string).unwrap()
  );
}
