//! Parses `gl.xml` and then prints out Rust source that can bind to it.

use phosphorus::{ApiGroup, GlApiSelection, GlProfile, GlRegistry};

fn main() {
  const USAGE: &str = "Usage: phosphorus <filename> <api> <major> <minor> <profile> [comma,separated,extensions,if,any]";
  let args: Vec<_> = std::env::args_os().collect();
  if args.len() == 2 && args[1].to_str() == Some("--version") {
    println!("phosphorus-{}", env!("CARGO_PKG_VERSION"));
    return;
  }
  if args.len() != 5 && args.len() != 6 && args.len() != 7 {
    panic!(
      "Illegal Arg Count: Should be either 5, 6 or 7 args, got {count}.\n{usage}",
      count = args.len(),
      usage = USAGE
    );
  }
  let filename = &args[1];
  let api = ApiGroup::from(args[2].to_str().unwrap());
  let major: i32 = args[3].to_str().unwrap().parse().unwrap();
  let minor: i32 = args[4].to_str().unwrap().parse().unwrap();
  let profile = args.get(5).map(|arg| GlProfile::from(arg.to_str().unwrap()));
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
