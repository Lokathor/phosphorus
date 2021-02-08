#![allow(unused)]

use magnesium::*;
use phosphorus::{gl_extension_lists::*, gl_feature_lists::*, Registry, *};

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
  //registry.fmt_command_types(&mut s).unwrap();
  let extension_fns = {
    let mut v = Vec::new();
    for ext in [GL_ARB_debug_output_COMMANDS, GL_KHR_debug_COMMANDS].iter() {
      for cmd in ext.iter().copied() {
        if !GL_VERSION_4_6.contains(&cmd) {
          v.push(cmd);
        }
      }
    }
    v
  };
  s.push_str("use super::*;\n\n");
  fmt_global_loader(&mut s, "GlFns46", &GL_VERSION_4_6, &extension_fns).unwrap();
  println!("{}", s);
}
