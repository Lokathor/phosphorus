#![allow(clippy::cognitive_complexity)]

//! `phosphorus` CLI program.
//!
//! Currently just a demo program to test with!

fn main() {
  let bytes_vec = std::fs::read("xml/gl.xml").unwrap();
  let string = String::from_utf8(bytes_vec).unwrap();

  let registry = phosphorus::GlRegistry::from_gl_xml(&string).unwrap();

  //println!("registry> {:#?}", registry);

  // println!("// types\n");
  // println!("pub use types::*;");
  // println!("pub mod types {{");
  // println!("  use super::*;");
  // for t in registry.types() {
  //   if t.name() != "khrplatform" {
  //     println!("  {}", t);
  //   }
  // }
  // println!("}}");

  // println!("\n// enums\n");
  // println!("pub use consts::*;");
  // println!("pub mod consts {{");
  // println!("  use super::*;");
  // for (key, value) in registry.enums() {
  //   println!("  {}", phosphorus::EnumDisplay { key, value });
  // }
  // println!("}}");

  // println!("\n// groups\n");
  // println!("pub use groups::*;");
  // println!("pub mod groups {{");
  // println!("  use super::*;");
  // for (name, entries) in registry.groups() {
  //   print!("  #[doc=\"{}\\n\"]", name);
  //   let mut enums_v: Vec<&String> = entries.iter().collect();
  //   enums_v.sort();
  //   for enu in enums_v {
  //     print!(" #[doc=\"* `{}`\\n\"]", enu);
  //   }
  //   println!(" pub type {} = GLenum;", name);
  // }
  // println!("}}");

  // println!("\n\n");
  // for feature in registry.features() {
  //   println!("// feature name: {}", feature.name);
  // }
  for t in
    phosphorus::generators::available_extensions(&registry, "GL_VERSION_3_3")
      .unwrap()
  {
    println!("{:?}", t);
  }
}
