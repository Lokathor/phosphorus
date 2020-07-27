//! Parses `gl.xml` and then prints out Rust source that can bind to it.

use phosphorus::{ApiGroup, GlApiSelection, GlProfile, GlRegistry};

fn main() {
  let args: Vec<_> = std::env::args_os().collect();
  let filename = match args.as_slice() {
    [_, a] if a.to_str() == Some("--version") => {
      println!("phosphorus-{}", env!("CARGO_PKG_VERSION"));
      return;
    }
    [_, f] => {
      f
    }
    _otherwise => {
      eprintln!("Expected exactly 1 arg, the gl.xml filename. Got {}", args.len());
      for arg in args.iter() {
        eprintln!("{:?}", arg.to_str());
      }
      return;
    }
  };
  //
  if cfg!(debug_assertions) {
    eprintln!("Reading `{}`", filename.to_str().unwrap());
  }
  let gl_xml = std::fs::read_to_string(&filename).unwrap();
  //
  if cfg!(debug_assertions) {
    eprintln!("Parsing the registry.");
  }
  let registry = GlRegistry::from_gl_xml_str(&gl_xml);
  //
  if cfg!(debug_assertions) {
    eprintln!("Selecting API_A.");
  }
  let mut api_a = GlApiSelection::new_from_registry_api_extensions(
    &registry,
    ApiGroup::Gl,
    (4, 1),
    GlProfile::Core,
    &[],
  );
  //
  if cfg!(debug_assertions) {
    eprintln!("Selecting API_B.");
  }
  let mut api_b = GlApiSelection::new_from_registry_api_extensions(
    &registry,
    ApiGroup::Gles2,
    (3, 0),
    GlProfile::Core,
    &[],
  );
  //
  let mut out = GlApiSelection::default();
  //
  assert_eq!(api_a.gl_types, api_b.gl_types);
  out.gl_types = api_a.gl_types.clone();
  //
  let mut api_a_enum_keys: Vec<_> = api_a.gl_enums.iter().collect();
  api_a_enum_keys.sort_by_cached_key(|a|a.0.clone());
  for (k,gl_enum) in api_a_enum_keys {
    if api_b.gl_enums.contains_key(k) {
      out.gl_enums.insert(k.clone(), gl_enum.clone());
    } else {
      eprintln!("Enum not in {:?} {:?}: {:?}", api_b.api, api_b.version, gl_enum.name)
    }
  }
  //
  let mut api_a_command_keys: Vec<_> = api_a.gl_commands.iter().collect();
  api_a_command_keys.sort_by_cached_key(|a|a.0.clone());
  for (k,gl_command) in api_a_command_keys {
    if api_b.gl_commands.contains_key(k) {
      out.gl_commands.insert(k.clone(), gl_command.clone());
    } else {
      eprintln!("Command not in {:?} {:?}: {:?}", api_b.api, api_b.version, gl_command.name)
    }
  }
  //
  //
  //
  core::mem::swap(&mut api_a, &mut api_b);
  //
  //
  //
  let mut api_a_enum_keys: Vec<_> = api_a.gl_enums.iter().collect();
  api_a_enum_keys.sort_by_cached_key(|a|a.0.clone());
  for (k,gl_enum) in api_a_enum_keys {
    if api_b.gl_enums.contains_key(k) {
      out.gl_enums.insert(k.clone(), gl_enum.clone());
    } else {
      eprintln!("Enum not in {:?} {:?}: {:?}", api_b.api, api_b.version, gl_enum.name)
    }
  }
  //
  let mut api_a_command_keys: Vec<_> = api_a.gl_commands.iter().collect();
  api_a_command_keys.sort_by_cached_key(|a|a.0.clone());
  for (k,gl_command) in api_a_command_keys {
    if api_b.gl_commands.contains_key(k) {
      out.gl_commands.insert(k.clone(), gl_command.clone());
    } else {
      eprintln!("Command not in {:?} {:?}: {:?}", api_b.api, api_b.version, gl_command.name)
    }
  }
  //
  out.api = ApiGroup::Gl;
  //
  out.version = (3, 4);

  if cfg!(debug_assertions) {
    eprintln!("Printing.");
  }
  println!("{}", 0);
}






















