//! Parses `gl.xml` and then prints out Rust source that can bind to it.

use phosphorus::{ApiGroup, GlApiSelection, GlProfile, GlRegistry};

fn main() {
  const USAGE: &str = "Illegal Arg Count. Usage: phosphorus <filename> <api> <major> <minor> <profile> [comma,separated,extensions,if,any]";
  let args: Vec<_> = std::env::args_os().collect();
  if args.len() != 6 && args.len() != 7 {
    panic!("Should be either 6 or 7 args. {}", USAGE);
  }
  let filename = &args[1];
  let api = match args[2].to_str().unwrap() {
    "gl" => ApiGroup::Gl,
    "gles1" => ApiGroup::Gles1,
    "gles2" => ApiGroup::Gles2,
    "glsc2" => ApiGroup::Glsc2,
    _ => panic!("illegal api name, pick from {{gl,gles1,gles2,glsc2}}"),
  };
  let major: i32 = args[3].to_str().unwrap().parse().unwrap();
  let minor: i32 = args[4].to_str().unwrap().parse().unwrap();
  let profile = match args[5].to_str().unwrap() {
    "core" => GlProfile::Core,
    "compatibility" => GlProfile::Compatibility,
    _ => panic!("illegal profile name, pick from {{core,compatibility}}"),
  };
  let extensions: Vec<&str> = if args.len() == 7 {
    args[6]
      .to_str()
      .unwrap()
      .split(',')
      .map(str::trim)
      .filter(|s| !s.is_empty())
      .collect()
  } else {
    Vec::new()
  };
  
  if cfg!(debug_assertions) {
    eprintln!("Reading `{}`", filename.to_str().unwrap());
  }
  let gl_xml = std::fs::read_to_string(&filename).unwrap();
  
  if cfg!(debug_assertions) {
    eprintln!("Parsing the registry.");
  }
  let registry = GlRegistry::from_gl_xml_str(&gl_xml);
  
  if cfg!(debug_assertions) {
    eprintln!("Selecting the correct API.");
  }
  let selection = GlApiSelection::new_from_registry_api_extensions(
    &registry,
    api,
    (major, minor),
    profile,
    &extensions,
  );
  
  if cfg!(debug_assertions) {
    eprintln!("Printing.");
  }
  println!("{}", selection);
}
