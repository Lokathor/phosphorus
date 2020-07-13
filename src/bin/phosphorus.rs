//! Parses `gl.xml` and then prints out Rust source that can bind to it.

use phosphorus::{ApiGroup, GlApiSelection, GlProfile, GlRegistry};

fn main() {
  let args: Vec<_> = std::env::args_os().collect();
  if args.len() != 7 {
    panic!("Usage: phosphorus [filename] [api] [major] [minor] [profile] [extensions,list]");
  }
  let filename = &args[1];
  let api = match args[2].to_str().unwrap() {
    "gl" => ApiGroup::Gl,
    "gles1" => ApiGroup::Gles1,
    "gles2" => ApiGroup::Gles2,
    "glsc2" => ApiGroup::Glsc2,
    _ => panic!("illegal api name, pick from [gl,gles1,gles2,glsc2]"),
  };
  let major: i32 = args[3].to_str().unwrap().parse().unwrap();
  let minor: i32 = args[4].to_str().unwrap().parse().unwrap();
  let profile = match args[5].to_str().unwrap() {
    "core" => GlProfile::Core,
    "compatibility" => GlProfile::Compatibility,
    _ => panic!("illegal profile name, pick from [core,compatibility]"),
  };
  let extensions: Vec<&str> =
    args[6].to_str().unwrap().split(",").map(str::trim).collect();
  let gl_xml = std::fs::read_to_string(&filename).unwrap();
  let registry = GlRegistry::from_gl_xml_str(&gl_xml);
  let selection = GlApiSelection::new_from_registry_api_extensions(
    &registry,
    api,
    (major, minor),
    profile,
    &extensions,
  );
  println!("{}", selection);
}
