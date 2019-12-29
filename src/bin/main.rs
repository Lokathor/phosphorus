#![allow(clippy::cognitive_complexity)]

//! `phosphorus` CLI program.
//!
//! Currently just a demo program to test with!

use phosphorus::*;

fn main() {
  let bytes_vec = std::fs::read("xml/gl.xml").unwrap();
  let string = String::from_utf8(bytes_vec).unwrap();

  let reg = GlRegistry::from_gl_xml(&string).unwrap();

  let output = Output::new(
    &reg,
    feature_name_for(ApiCategory::Gl, 3, 3).unwrap(),
    Profile::Core,
    &[],
  );

  println!("{:#?}", output);
}
