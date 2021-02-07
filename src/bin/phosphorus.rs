#![allow(unused_mut)]
#![allow(unused_imports)]

use magnesium::*;
use phosphorus::{Registry, *};

fn main() {
  let args: Vec<String> = std::env::args().skip(1).collect();
  let gl_xml_filename = &args[0];
  if cfg!(debug_assertions) {
    eprintln!("Reading `{}`", gl_xml_filename);
  }
  let gl_xml_string = std::fs::read_to_string(gl_xml_filename).expect("Couldn't read gl.xml file!");
  if cfg!(debug_assertions) {
    eprintln!("Read {} bytes.", gl_xml_string.len());
  }
  let gl_xml_str = if gl_xml_string.chars().nth(0).unwrap() == '\u{feff}' {
    if cfg!(debug_assertions) {
      eprintln!("Byte Order Mark detected, removing.");
    }
    &gl_xml_string['\u{feff}'.len_utf8()..]
  } else {
    &gl_xml_string
  };

  let mut iter = ElementIterator::new(&gl_xml_str).filter_map(skip_comments).filter_map(skip_empty_text_elements).map(trim_text);
  let mut registry = Registry::from_element_iterator(&mut iter);
  registry.commands.sort();
  if cfg!(debug_assertions) {
    eprintln!("Enumerations Count: {}", registry.enumerations.len());
    eprintln!("Commands Count: {}", registry.commands.len());
    eprintln!("Features Count: {}", registry.features.len());
    eprintln!("Extensions Count: {}", registry.extensions.len());
  }
  let mut s = String::with_capacity(1024 * 1024 * 10);
  registry.fmt_feature_lists(&mut s).unwrap();
  println!("{}", s);
}
