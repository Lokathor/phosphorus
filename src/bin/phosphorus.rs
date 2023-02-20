#![allow(unused_imports)]

use std::collections::{hash_map::Entry, HashMap, HashSet};

use magnesium::*;
use phosphorus::*;

const XML: &str = include_str!("../../gl.xml");

fn main() {
  #[allow(unused)]
  let Registry { types, enum_lists, commands, features, extensions } = {
    let mut iter = ElementIterator::new(XML)
      .filter_map(skip_comments)
      .map(trim_text)
      .filter_map(skip_empty_text_elements);
    assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("registry", ""));
    Registry::from_iter(&mut iter)
  };
  drop(types);
  drop(enum_lists);
  //
  let mut gl_command_names: Vec<StaticStr> = Vec::new();
  let mut gles2_command_names: Vec<StaticStr> = Vec::new();
  for Feature { name: _, api, number: _, requirements, removals } in features.iter() {
    let command_names = match *api {
      "gl" => &mut gl_command_names,
      "gles2" => &mut gles2_command_names,
      _ => continue,
    };
    for requirement in requirements.iter() {
      command_names.extend(requirement.commands.iter().copied());
    }
    for removal in removals.iter() {
      for removed_command in removal.commands.iter() {
        if let Some(pos) = command_names.iter().position(|c| c == removed_command) {
          command_names.remove(pos);
        }
      }
    }
  }
  gl_command_names.sort();
  gl_command_names.dedup();
  gles2_command_names.sort();
  gles2_command_names.dedup();
  //
  let combo_commands: Vec<Command> = gl_command_names
    .into_iter()
    .chain(gles2_command_names.into_iter())
    .map(|name| commands.iter().find(|c| c.name == name).cloned().unwrap())
    .collect();
  //
  println!("// listing {} commands from both GL and GLES", combo_commands.len());
  //print_fn_type_aliases(&combo_commands);
  print_struct_declaration(&combo_commands);
}

pub fn print_fn_type_aliases(commands: &[Command]) {
  for command in commands {
    println!(
      "#[allow(non_camel_case_types)] pub(crate) type {}_t = {};",
      command.name,
      command.get_fn_ty()
    );
  }
}

pub fn print_struct_declaration(commands: &[Command]) {
  println!("pub struct GlFns {{");
  for command in commands.iter() {
    let field_name = command.name;
    println!("  {field_name}: Option<{field_name}_t>,");
  }
  println!("}}");
}

pub fn print_enum_listings(enum_lists: &[EnumList]) {
  let mut map = HashMap::new();
  for enum_list in enum_lists.iter() {
    let list_ty = if enum_list.ty == "bitmask" { "GLbitfield" } else { "GLenum" };
    for entry @ EnumEntry { name, group, alias, .. } in enum_list.enums.iter() {
      if *group == "SpecialNumbers" || *name == phosphorus::FORBIDDEN_ENUMERATION {
        continue;
      }
      match map.entry(name) {
        Entry::Occupied(oe) => {
          eprintln!("already defined: {oe:?}");
        }
        Entry::Vacant(ve) => {
          ve.insert(EnumEntry { ty: list_ty, ..entry.clone() });
        }
      }
      if !alias.is_empty() {
        match map.entry(name) {
          Entry::Occupied(oe) => {
            assert_eq!(oe.get().value, entry.value);
          }
          Entry::Vacant(ve) => {
            ve.insert(EnumEntry { ty: list_ty, ..entry.clone() });
          }
        }
      }
    }
  }
  let mut list: Vec<_> = map.into_iter().collect();
  list.sort_by_key(|x| x.0);
  for EnumEntry { name, value, group, comment: _, alias, ty, api: _ } in
    list.iter().map(|x| x.1.clone())
  {
    if !group.is_empty() {
      let group_items: Vec<_> = group.split(',').collect();
      if group_items.len() == 1 {
        println!("/// * Group: `{group}`");
      } else {
        print!("/// * Groups: ");
        for (i, group_item) in group_items.iter().enumerate() {
          if i != 0 {
            print!(", ");
          }
          print!("`{group_item}`");
        }
        println!();
      }
    }
    if !alias.is_empty() {
      println!("/// * Alias: [`{alias}`]");
    }
    if name.chars().any(char::is_lowercase) {
      println!("#[allow(non_upper_case_globals)]");
    }
    if let Some(was_neg) = value.strip_prefix('-') {
      println!("pub const {name}: {ty} = ({was_neg} as {ty}).wrapping_neg();");
    } else {
      println!("pub const {name}: {ty} = {value};");
    }
    println!();
  }
}
