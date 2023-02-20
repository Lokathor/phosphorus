use magnesium::*;
use phosphorus::*;

const XML: &str = include_str!("../gl.xml");

fn main() {
  let mut iter = ElementIterator::new(XML)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("registry", ""));
  let registry = Registry::from_iter(&mut iter);
  println!("{registry:#?}");

  /*
  for feature in registry.features.iter() {
    let name = feature.name;
    let api = feature.api;
    let number = feature.number;
    println!("Feature {name}, ({api}-{number})");
  }
  */
}

pub fn print_fn_type_aliases(commands: &[Command]) {
  for command in commands {
    println!("pub(crate) type {}_t = {};", command.name, command.get_fn_ty());
  }
}
