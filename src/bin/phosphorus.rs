#![allow(unused)]

use magnesium::*;
use phosphorus::{gl_extension_deltas::*, gl_feature_deltas::*, registry::*, source_managers::*, *};
use std::collections::{BTreeSet, HashMap};

fn main() {
  print_out_gl33_plus_extensions()
}

fn print_out_registry() {
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
  if cfg!(debug_assertions) {
    eprintln!("Enumerations Count: {}", registry.enumerations.len());
    eprintln!("Commands Count: {}", registry.commands.len());
    eprintln!("Features Count: {}", registry.features.len());
    eprintln!("Extensions Count: {}", registry.extensions.len());
  }
  let mut s = String::with_capacity(1024 * 1024 * 10);
  println!("{}", s);
}

fn print_out_gl33_plus_extensions() {
  // Read In The Registry
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

  // Prepare Our Api To Output
  let mut enums_needed = BTreeSet::new();
  let mut non_null_commands = BTreeSet::new();
  let mut nullable_commands = BTreeSet::new();
  for feature_delta in [GL_VERSION_1_0, GL_VERSION_1_1, GL_VERSION_1_2, GL_VERSION_1_3, GL_VERSION_1_4, GL_VERSION_1_5, GL_VERSION_2_0, GL_VERSION_2_1, GL_VERSION_3_0, GL_VERSION_3_1, GL_VERSION_3_2, GL_VERSION_3_3].iter() {
    for enum_name in feature_delta.enums_added.iter().copied() {
      enums_needed.insert(enum_name);
    }
    // core commands are required.
    for command_name in feature_delta.commands_added.iter().copied() {
      non_null_commands.insert(command_name);
    }
    // Only the 3.2 level *actually* removes stuff
    for enum_name in feature_delta.enums_removed.iter().copied() {
      enums_needed.remove(enum_name);
    }
    for command_name in feature_delta.commands_removed.iter().copied() {
      non_null_commands.remove(command_name);
    }
  }
  for extension_delta in [GL_KHR_debug_GL, GL_ARB_texture_filter_anisotropic, GL_ARB_pipeline_statistics_query].iter() {
    for enum_name in extension_delta.enums_added.iter().copied() {
      enums_needed.insert(enum_name);
    }
    // extension commands are nullable
    for command_name in extension_delta.commands_added.iter().copied() {
      nullable_commands.insert(command_name);
    }
  }
  let mut enumeration_entries = get_enumeration_entries();
  enumeration_entries.retain(|ee| enums_needed.contains(ee.name.as_str()));
  let mut command_type_entries = get_command_type_entries();
  command_type_entries.retain(|cte| non_null_commands.contains(cte.name.as_str()) || nullable_commands.contains(cte.name.as_str()));

  // Print All Enumeration Declarations
  for enum_entry in enumeration_entries.iter() {
    print!("{}", enum_entry.comments);
    println!("{}", enum_entry.enum_declaration);
    println!();
  }

  println!();
  println!("// // // // // // // // // // // // // // // // // // // // // // // // // //");
  println!();

  // Print All Group Declarations
  let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
  for enumeration in registry.enumerations.iter().filter(|e| enums_needed.contains(e.name)) {
    for group in enumeration.group.as_ref().unwrap_or(&"").split(',') {
      if group == "" {
        continue;
      }
      g.entry(group).or_insert(Vec::new()).push(enumeration.name);
    }
  }
  let mut group_list: Vec<&str> = g.keys().copied().filter(|g| command_type_entries.iter().any(|cte| cte.type_declaration.contains(g))).collect();
  group_list.sort();
  for group in group_list.iter() {
    g.entry(group).or_insert(Vec::new()).sort();
    println!("/// {}", group);
    for group_entry in g.entry(group).or_insert(Vec::new()).iter() {
      println!("/// * [`{}`]", group_entry);
    }
    println!("pub type {} = GLenum;", group);
    println!("");
  }

  println!();
  println!("// // // // // // // // // // // // // // // // // // // // // // // // // //");
  println!();

  // Print All Command Type Declarations
  for command_type_entry in command_type_entries.iter() {
    // these types are crate-private and the docs for each command appear on the
    // struct method.
    println!("pub(crate) {}", &command_type_entry.type_declaration["pub ".len()..]);
    println!();
  }

  println!();
  println!("// // // // // // // // // // // // // // // // // // // // // // // // // //");
  println!();
  // print command loader
  let non_null_commands: Vec<&str> = non_null_commands.iter().copied().collect();
  let nullable_commands: Vec<&str> = nullable_commands.iter().copied().collect();
  let mut s = String::with_capacity(10 * 1024 * 1024);
  fmt_struct_loader(&mut s, "GlFns", &non_null_commands, &nullable_commands);
  println!("{}", s);

  println!();
  println!("// // // // // // // // // // // // // // // // // // // // // // // // // //");
  println!();
  // print command loader
  let mut nullable_global_commands: Vec<&str> = nullable_commands.clone();
  nullable_global_commands.extend(non_null_commands);
  nullable_global_commands.sort();
  let mut s = String::with_capacity(10 * 1024 * 1024);
  fmt_global_loader(&mut s, &nullable_global_commands);
  println!("{}", s);
}

fn print_out_gl46_superbible() {
  // Read In The Registry
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

  // Prepare Our Api To Output
  let mut enums_needed = BTreeSet::new();
  let mut non_null_commands = BTreeSet::new();
  let mut nullable_commands = BTreeSet::new();
  for feature_delta in [GL_VERSION_1_0, GL_VERSION_1_1, GL_VERSION_1_2, GL_VERSION_1_3, GL_VERSION_1_4, GL_VERSION_1_5, GL_VERSION_2_0, GL_VERSION_2_1, GL_VERSION_3_0, GL_VERSION_3_1, GL_VERSION_3_2, GL_VERSION_3_3, GL_VERSION_4_0, GL_VERSION_4_1, GL_VERSION_4_2, GL_VERSION_4_3, GL_VERSION_4_4, GL_VERSION_4_5, GL_VERSION_4_6].iter() {
    for enum_name in feature_delta.enums_added.iter().copied() {
      enums_needed.insert(enum_name);
    }
    // core commands are required.
    for command_name in feature_delta.commands_added.iter().copied() {
      non_null_commands.insert(command_name);
    }
    // Only the 3.2 level *actually* removes stuff
    for enum_name in feature_delta.enums_removed.iter().copied() {
      enums_needed.remove(enum_name);
    }
    for command_name in feature_delta.commands_removed.iter().copied() {
      non_null_commands.remove(command_name);
    }
  }
  // anisotropic is just nifty, and doesn't take up any function pointers. The
  // rest of these are used by The OpenGL SuperBible 7e
  for extension_delta in [GL_ARB_texture_filter_anisotropic, GL_ARB_bindless_texture, GL_ARB_sparse_texture, GL_ARB_pipeline_statistics_query].iter() {
    for enum_name in extension_delta.enums_added.iter().copied() {
      enums_needed.insert(enum_name);
    }
    // extension commands are nullable
    for command_name in extension_delta.commands_added.iter().copied() {
      nullable_commands.insert(command_name);
    }
  }
  let mut enumeration_entries = get_enumeration_entries();
  enumeration_entries.retain(|ee| enums_needed.contains(ee.name.as_str()));
  let mut command_type_entries = get_command_type_entries();
  command_type_entries.retain(|cte| non_null_commands.contains(cte.name.as_str()) || nullable_commands.contains(cte.name.as_str()));

  // Print All Enumeration Declarations
  //for enum_entry in enumeration_entries.iter() {
  //  print!("{}", enum_entry.comments);
  //  println!("{}", enum_entry.enum_declaration);
  //  println!();
  //}

  println!();
  println!("// // // // // // // // // // // // // // // // // // // // // // // // // //");
  println!();

  // Print All Group Declarations
  let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
  for enumeration in registry.enumerations.iter().filter(|e| enums_needed.contains(e.name)) {
    for group in enumeration.group.as_ref().unwrap_or(&"").split(',') {
      if group == "" {
        continue;
      }
      g.entry(group).or_insert(Vec::new()).push(enumeration.name);
    }
  }
  let mut group_list: Vec<&str> = g.keys().copied().filter(|g| command_type_entries.iter().any(|cte| cte.type_declaration.contains(g))).collect();
  group_list.sort();
  //for group in group_list.iter() {
  //  g.entry(group).or_insert(Vec::new()).sort();
  //  println!("/// {}", group);
  //  for group_entry in g.entry(group).or_insert(Vec::new()).iter() {
  //    println!("/// * [`{}`]", group_entry);
  //  }
  //  println!("pub type {} = GLenum;", group);
  //  println!("");
  //}

  println!();
  println!("// // // // // // // // // // // // // // // // // // // // // // // // // //");
  println!();

  // Print All Command Type Declarations
  for command_type_entry in command_type_entries.iter() {
    // these types are crate-private and the docs for each command appear on the
    // struct method.
    println!("pub(crate) {}", &command_type_entry.type_declaration["pub ".len()..]);
    println!();
  }

  println!();
  println!("// // // // // // // // // // // // // // // // // // // // // // // // // //");
  println!();
  // print command loader
  let non_null_commands: Vec<&str> = non_null_commands.iter().copied().collect();
  let nullable_commands: Vec<&str> = nullable_commands.iter().copied().collect();
  let mut s = String::with_capacity(10 * 1024 * 1024);
  fmt_struct_loader(&mut s, "GlFns", &non_null_commands, &nullable_commands);
  println!("{}", s);
}
