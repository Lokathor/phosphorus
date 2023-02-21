#![allow(unused_imports)]

use core::fmt::Write;
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
    let list = match *api {
      "gl" => &mut gl_command_names,
      "gles2" => &mut gles2_command_names,
      _ => continue,
    };
    for requirement in requirements.iter() {
      list.extend(requirement.commands.iter().copied());
    }
    for removal in removals.iter() {
      for removed_command in removal.commands.iter() {
        if let Some(pos) = list.iter().position(|c| c == removed_command) {
          list.remove(pos);
        }
      }
    }
  }
  //

  // IF YOU WANT TO SUPPORT COMMANDS FROM OTHER EXTENSIONS ADD THEM TO THE LIST
  // IN THIS FILTER RULE HERE.
  for Extension { require_lists, .. } in
    extensions.iter().filter(|x| ["GL_KHR_debug"].contains(&x.name))
  {
    for RequireList { api, commands, .. } in require_lists.iter() {
      let list = match *api {
        "gl" => &mut gl_command_names,
        "gles2" => &mut gles2_command_names,
        _ => continue,
      };
      eprintln!("Adding: {commands:?}");
      list.extend(commands.iter().copied());
    }
  }

  //
  let mut combo_names: Vec<StaticStr> =
    gl_command_names.into_iter().chain(gles2_command_names.into_iter()).collect();
  combo_names.sort();
  combo_names.dedup();
  //
  let combo_commands: Vec<Command> = combo_names
    .into_iter()
    .map(|name| commands.iter().find(|c| c.name == name).cloned().unwrap())
    .collect();

  println!("#![allow(nonstandard_style)]");
  print_struct_declaration(&combo_commands);
  print_struct_loader_method(&combo_commands);
  print_struct_gl_methods(&combo_commands);
  print_fn_type_aliases(&combo_commands);
  print_fn_loaded_checker(&combo_commands);
}

const FN_LOADED_METHOD_MACRO: &str = "
macro_rules! mk_load_checker_method {
  ($full_name:ident, $short_name:ident) => {
    #[inline]
    #[must_use]
    pub fn $short_name(&self) -> bool {
      {
        self.0.$full_name.is_some()
      }
    }
  };
}
";

fn print_fn_loaded_checker(commands: &[Command]) {
  println!("impl GlFns {{");
  println!("  pub fn has_loaded(&self) -> FnLoadedChecker<'_> {{ FnLoadedChecker(self) }}");
  println!("}}");
  println!("{FN_LOADED_METHOD_MACRO}");
  println!("pub struct FnLoadedChecker<'g>(&'g GlFns);");
  println!("impl FnLoadedChecker<'_> {{");
  for command in commands.iter() {
    let full_name = command.name;
    let short_name = full_name.strip_prefix("gl").unwrap();
    println!("mk_load_checker_method!({full_name}, {short_name});");
  }
  println!("}}");
}

pub fn print_struct_loader_method(commands: &[Command]) {
  println!("impl GlFns {{");
  println!("  #[inline]#[must_use]");
  println!("  pub fn new_boxed() -> Box<Self> {{");
  println!("    // this struct is usually *hundreds* of `usize` big,");
  println!("    // so we do this heap dance to avoid stack strain.");
  println!(
    "    assert!(core::mem::size_of::<Self>() % core::mem::size_of::<Option<fn()>>() == 0);"
  );
  println!("    let len = core::mem::size_of::<Self>() / core::mem::size_of::<Option<fn()>>();");
  println!("    let v: Vec<Option<fn()>> = vec![None; len];");
  println!("    #[allow(clippy::type_complexity)]");
  println!("    let b: Box<[Option<fn()>]> = v.into_boxed_slice();");
  println!("    let ptr_slice: *mut [Option<fn()>] = Box::leak(b);");
  println!("    let ptr_self: *mut Self = ptr_slice as *mut Self;");
  println!("    let box_self: Box<Self> = unsafe {{ Box::from_raw(ptr_self) }};");
  println!("    box_self");
  println!("  }}");
  println!("#[allow(clippy::missing_safety_doc)]");
  println!(
    "  pub unsafe fn load(&mut self, mut loader: impl FnMut(*const u8) -> *const core::ffi::c_void) {{"
  );
  println!("    use core::mem::transmute;");
  for command in commands.iter() {
    let full_name = command.name;
    println!("    self.{full_name} = unsafe {{ transmute(loader(\"{full_name}\\0\".as_ptr())) }};");
  }
  println!("  }}");
  println!("}}");
  println!();
  println!(
    "/// This 'literal' value might help if you wanna make a GlFns in a `static mut` or something."
  );
  println!("#[allow(unused)] #[doc(hidden)]");
  println!("pub const BLANK_GL_FNS: GlFns = GlFns {{");
  for command in commands.iter() {
    let full_name = command.name;
    println!("  {full_name}: None,");
  }
  println!("}};");
}

pub fn print_fn_type_aliases(commands: &[Command]) {
  println!("use fn_ty_aliases::*;");
  println!("mod fn_ty_aliases {{");
  println!("  use super::*;");
  for command in commands {
    println!("  pub type {}_t = {};", command.name, command.get_fn_ty());
  }
  println!("}}");
}

pub fn print_struct_declaration(commands: &[Command]) {
  println!("use super::type_alias::*;");
  println!("/// Holds fn pointers for {} GL functions", commands.len());
  println!("#[repr(C)]");
  println!("pub struct GlFns {{");
  for command in commands.iter() {
    let field_name = command.name;
    println!("  {field_name}: Option<{field_name}_t>,");
  }
  println!("}}");
}

const COLD_PANIC_SRC: &str = "
#[cold]
#[doc(hidden)]
#[inline(never)]
#[cfg_attr(feature = \"track_caller\", track_caller)]
fn cold_panic(msg: &str) -> ! {
  panic!(\"Called a GL fn that wasn't loaded: {msg}\");
}
";

const WRAPPER_METHOD_MACRO: &str = "
macro_rules! mk_wrapper_method {
  ($full_name:ident, $short_name:ident, [$($arg_name:ident : $arg_ty:ty,)*] -> $ret_ty:ty) => {
    #[inline]
    #[allow(nonstandard_style)]
    #[allow(clippy::unused_unit)]
    #[allow(clippy::needless_return)]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::missing_safety_doc)]
    #[cfg_attr(feature = \"track_caller\", track_caller)]
    pub unsafe fn $short_name(&self, $($arg_name : $arg_ty),*) -> $ret_ty {
      if let Some(f) = self.$full_name {
        return unsafe { f($($arg_name),*) };
      } else {
        cold_panic(stringify!($full_name));
      }
    }
  };
}
";

pub fn print_struct_gl_methods(commands: &[Command]) {
  println!("{COLD_PANIC_SRC}");
  println!("{WRAPPER_METHOD_MACRO}");
  println!("impl GlFns {{");
  for command in commands.iter() {
    let short_name = command.name.strip_prefix("gl").unwrap();
    let mut args_and_tys = String::new();
    let mut args = String::new();
    for param in command.params.iter() {
      let name = param.name;
      let ty_string = format_type_and_variant(param.ty, param.ty_variant);
      write!(args_and_tys, "{name}: {ty_string}, ").ok();
      write!(args, "{name}, ").ok();
    }
    let ret_ty = format_type_and_variant(command.return_ty, command.return_ty_variant);
    let full_name = command.name;
    println!("  mk_wrapper_method!({full_name}, {short_name}, [{args_and_tys}] -> {ret_ty});");
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
