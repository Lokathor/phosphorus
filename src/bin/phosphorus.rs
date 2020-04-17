#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

fn main() {
  let file_content = std::fs::read_to_string("xml/gl.xml").unwrap();
  let registry = phosphorus::Registry::from_xml(&file_content);
  //println!("{}", registry.types.make_types_module_string());
  //println!("{}", registry.groups.make_groups_module_string());
  // TODO
  println!("{:#?}", registry.enumerants);
  //println!("{}", registry.features.?);
  //println!("{}", registry.commands.?);
  //println!("{}", registry.extensions.?);
}
