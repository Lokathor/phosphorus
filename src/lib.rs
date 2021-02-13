#![allow(bad_style)]
#![allow(unused_imports)]

//! "`phosphorus` helps make things that glow."
//!
//! Get it, because this makes GL bindings, "GLow", get it? Yeah, whatever,
//! crate names are all silly anyway.
//!
//! This crate generates bindings to the OpenGL and OpenGL ES APIs. However,
//! there's no *single* GL API, which rather complicates the process. You
//! generally have to load the appropriate function pointers dynamically, which
//! means that you need to deliberately store them somewhere once the program
//! starts, instead of having the linker just put the correct jumps directly
//! into your program.
//!
//! This leads to two basic styles of loader:
//! * "global", where there's a set of function pointers that are stored in
//!   global mutable memory. Any function can use GL from anywhere once the
//!   initialization has been completed.
//! * "struct", where a struct stores all of the pointers. To use GL within a
//!   function someone passes you a reference to a struct with all the
//!   appropriate methods.
//!
//! The basis for truth with GL is
//! [gl.xml](https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/master/xml/gl.xml).
//! The only problem with the XML is that it describes a C API, and so there's
//! very little type safety to prevent you from doing the wrong thing. This
//! crate attempts to *help* provide some better type safety with the bindings
//! it exposes by at least preventing you from accidentally mixing up `GLenum`,
//! `GLbitfield`, and plain integer values.

use chlorine::*;
use magnesium::{XmlElement::*, *};

use core::fmt::Write;

pub mod registry;
pub(crate) use registry::*;

pub mod gl_core_types;
pub(crate) use gl_core_types::*;

pub mod gl_enumerations;
pub(crate) use gl_enumerations::*;

pub mod gl_groups;
pub(crate) use gl_groups::*;

pub mod gl_command_types;
pub(crate) use gl_command_types::*;

pub mod gl_feature_deltas;
pub(crate) use gl_feature_deltas::*;

pub mod gl_extension_deltas;
pub(crate) use gl_extension_deltas::*;

//

pub mod source_managers;
pub(crate) use source_managers::*;

//

pub mod struct_gl46fns;
pub(crate) use struct_gl46fns::*;

pub mod global_opt_46fns;
pub(crate) use global_opt_46fns::*;

//

/// Creates code for a struct loader with the given name.
///
/// * `non_null_commands`: A list of commands the struct *must* support. Failure
///   to load any of these commands will cause the entire struct to fail
///   construction.
/// * `nullable_commands`: A list of commands that the struct *can* support.
///   Failure to load any of these will allow creation with the command missing.
///   For a command such as `glFooBar` use the `FooBar__is_loaded` method to
///   check if it's available.
///
/// The advantage of non_null commands over nullable commands is that the
/// non_null commands have one less branch per call. If you know for sure the
/// minimum API version that you're targeting (eg: "my program requires GL 3.3
/// or later") you should place those commands in the non_null list.
pub fn fmt_struct_loader(s: &mut String, struct_name: &str, non_null_commands: &[&str], nullable_commands: &[&str]) -> core::fmt::Result {
  let desired_command_type_entries: Vec<CommandTypeEntry> = get_command_type_entries().into_iter().filter(|entry| non_null_commands.contains(&&*entry.name) || nullable_commands.contains(&&*entry.name)).collect();

  writeln!(s, "// Note(Lokathor): _p for ptr, _t for type")?;
  writeln!(s)?;
  writeln!(s, "#[repr(C)]")?;
  writeln!(s, "pub struct {struct_name} {{", struct_name = struct_name)?;
  for cmd in non_null_commands.iter() {
    writeln!(s, "  {name}_p: {name}_t,", name = cmd)?;
  }
  for cmd in nullable_commands.iter() {
    writeln!(s, "  {name}_p: Option<{name}_t>,", name = cmd)?;
  }
  writeln!(s, "}}")?;
  writeln!(s)?;
  writeln!(s, "impl {struct_name} {{", struct_name = struct_name)?;
  writeln!(s, "  fn ptr_filter(p: *const c_void) -> Option<core::ptr::NonNull<c_void>> {{")?;
  writeln!(s, "    match p as usize {{")?;
  writeln!(s, "      // Note(Lokathor): wgl is known to sometimes give phony non-null pointer values.")?;
  writeln!(s, "      0 | 1 | 2 | 3 | usize::MAX => None,")?;
  writeln!(s, "      _ => unsafe {{ core::mem::transmute(p) }},")?;
  writeln!(s, "    }}")?;
  writeln!(s, "  }}")?;
  writeln!(s, "  #[cold]")?;
  writeln!(s, "  #[inline(never)]")?;
  writeln!(s, "  #[cfg_attr(feature=\"track_caller\", track_caller)]")?;
  writeln!(s, "  fn not_loaded(name: &str) -> ! {{")?;
  writeln!(s, "    panic!(\"Function Not Loaded: {{}}\", name);")?;
  writeln!(s, "  }}")?;
  writeln!(s)?;
  writeln!(s, "  /// Loads all GL functions from the loader given.")?;
  writeln!(s, "  /// ")?;
  writeln!(s, "  /// ## Failure")?;
  writeln!(s, "  /// This fails if any non-nullable function does not load.")?;
  writeln!(s, "  /// The error value will be the name of the first non-nullable function that doesn't load.")?;
  writeln!(s, "  /// ")?;
  writeln!(s, "  /// ## Safety")?;
  writeln!(s, "  /// * The \"Get Proc Address\" function you provide will always be given a pointer to the start of a null-terminated string containing the name of a GL function to load.")?;
  writeln!(s, "  /// * The \"Get Proc Address\" function given must always return accurate function pointer values, or null on failure.")?;
  writeln!(s, "  pub unsafe fn load_from(f: &dyn Fn(*const u8) -> *const c_void) -> Result<Self, &'static str> {{")?;
  writeln!(s, "    use core::mem::transmute;")?;
  writeln!(s, "    type nn_cv = core::ptr::NonNull<c_void>;")?;
  writeln!(s, "    // non-nullable loads")?;
  for cmd in non_null_commands.iter() {
    writeln!(s, "    let {name}_p = transmute::<nn_cv, {name}_t>(Self::ptr_filter(f(b\"{name}\\0\".as_ptr())).ok_or(\"{name}\")?);", name = cmd)?;
  }
  writeln!(s, "    // nullable loads")?;
  for cmd in nullable_commands.iter() {
    writeln!(s, "    let {name}_p = transmute::<Option<nn_cv>, Option<{name}_t>>(Self::ptr_filter(f(b\"{name}\\0\".as_ptr())));", name = cmd)?;
  }
  writeln!(s, "    // we're all good!")?;
  writeln!(s, "    Ok(Self {{")?;
  for cmd in non_null_commands.iter() {
    writeln!(s, "      {name}_p,", name = cmd)?;
  }
  for cmd in nullable_commands.iter() {
    writeln!(s, "      {name}_p,", name = cmd)?;
  }
  writeln!(s, "    }})")?;
  writeln!(s, "  }}")?;
  writeln!(s, "}}")?;
  writeln!(s)?;
  writeln!(s, "impl {struct_name} {{", struct_name = struct_name)?;
  for cmd in non_null_commands.iter().copied() {
    let entry = desired_command_type_entries.iter().find(|ent| ent.name == cmd).unwrap();
    let name = &entry.name;
    let unsafe_str = if entry.type_declaration.contains("unsafe") { "unsafe" } else { "" };
    let short_name = &entry.name[2..];
    let args_str = {
      let open_paren = entry.type_declaration.as_bytes().iter().copied().position(|b| b == b'(').unwrap();
      let close_paren = entry.type_declaration.as_bytes().iter().copied().position(|b| b == b')').unwrap();
      &entry.type_declaration[open_paren + 1..close_paren]
    };
    let ret_type = match entry.type_declaration.as_bytes().iter().copied().position(|b| b == b'>') {
      Some(position) => &entry.type_declaration[position + 1..entry.type_declaration.len() - 1],
      None => "",
    };
    let ret_arrow = if ret_type.is_empty() { "" } else { "->" };
    for docs_line in entry.comments.lines() {
      writeln!(s, "  {}", docs_line)?;
    }
    let args_names = {
      let mut s = String::new();
      for arg in args_str.split(',') {
        let arg_name = arg.split(':').next().unwrap();
        if arg_name.len() > 0 {
          write!(s, "{arg_name},", arg_name = arg_name)?;
        }
      }
      s
    };
    writeln!(s, "  pub {unsafe_str} fn {short_name}(&self, {args_str}){ret_arrow}{ret_type} {{", unsafe_str = unsafe_str, short_name = short_name, args_str = args_str, ret_arrow = ret_arrow, ret_type = ret_type)?;
    writeln!(s, "    (self.{name}_p)({args_names})", name = name, args_names = args_names)?;
    writeln!(s, "  }}")?;
  }
  //
  for cmd in nullable_commands.iter().copied() {
    let entry = desired_command_type_entries.iter().find(|ent| ent.name == cmd).unwrap();
    let name = &entry.name;
    let unsafe_str = if entry.type_declaration.contains("unsafe") { "unsafe" } else { "" };
    let short_name = &entry.name[2..];
    let args_str = {
      let open_paren = entry.type_declaration.as_bytes().iter().copied().position(|b| b == b'(').unwrap();
      let close_paren = entry.type_declaration.as_bytes().iter().copied().position(|b| b == b')').unwrap();
      &entry.type_declaration[open_paren + 1..close_paren]
    };
    let ret_type = match entry.type_declaration.as_bytes().iter().copied().position(|b| b == b'>') {
      Some(position) => &entry.type_declaration[position + 1..entry.type_declaration.len() - 1],
      None => "",
    };
    let ret_arrow = if ret_type.is_empty() { "" } else { "->" };
    for docs_line in entry.comments.lines() {
      writeln!(s, "  {}", docs_line)?;
    }
    let args_names = {
      let mut s = String::new();
      for arg in args_str.split(',') {
        let arg_name = arg.split(':').next().unwrap();
        if arg_name.len() > 0 {
          write!(s, "{arg_name},", arg_name = arg_name)?;
        }
      }
      s
    };
    writeln!(s, "  #[cfg_attr(feature=\"track_caller\", track_caller)]")?;
    writeln!(s, "  pub {unsafe_str} fn {short_name}(&self, {args_str}){ret_arrow}{ret_type} {{", unsafe_str = unsafe_str, short_name = short_name, args_str = args_str, ret_arrow = ret_arrow, ret_type = ret_type)?;
    writeln!(s, "    match self.{name}_p {{", name = name)?;
    writeln!(s, "      Some(f) => f({args_names}),", args_names = args_names)?;
    writeln!(s, "      None => Self::not_loaded(\"{name}\"),", name = name)?;
    writeln!(s, "    }}")?;
    writeln!(s, "  }}")?;
    writeln!(s, "  #[doc(hidden)]")?;
    writeln!(s, "  pub fn {short_name}_is_loaded(&self) -> bool {{", short_name = short_name)?;
    writeln!(s, "    self.{name}_p.is_some()", name = name)?;
    writeln!(s, "  }}")?;
  }
  writeln!(s, "}}")?;

  Ok(())
}

/// Generates code for a global GL loader.
///
/// Unlike with [`fmt_struct_loader`], the global loader only accepts a list of
/// nullable commands. Since GL itself might not have been initialized when you
/// call a given function, every function is potentially not loaded.
pub fn fmt_global_loader(s: &mut String, nullable_commands: &[&str]) -> core::fmt::Result {
  let desired_command_type_entries: Vec<CommandTypeEntry> = get_command_type_entries().into_iter().filter(|entry| nullable_commands.contains(&&*entry.name)).collect();

  writeln!(s, "#[repr(transparent)]")?;
  writeln!(s, "struct GlFnCell<T>(core::cell::UnsafeCell<Option<T>>);")?;
  writeln!(s)?;
  writeln!(s, "// Note(Lokathor): _p for ptr, _t for type")?;
  writeln!(s)?;
  writeln!(s, "// Note(Lokathor): This is a **lie**, and `GlFnCell` MUST remain private to this module for safety to be upheld.")?;
  writeln!(s, "unsafe impl<T> Sync for GlFnCell<T> {{}}")?;
  writeln!(s)?;
  writeln!(s, "fn gl_ptr_filter(p: *const c_void) -> Option<core::ptr::NonNull<c_void>> {{")?;
  writeln!(s, "  match p as usize {{")?;
  writeln!(s, "    // Note(Lokathor): wgl is known to sometimes give phony non-null pointer values.")?;
  writeln!(s, "    0 | 1 | 2 | 3 | usize::MAX => None,")?;
  writeln!(s, "    _ => unsafe {{ core::mem::transmute(p) }},")?;
  writeln!(s, "  }}")?;
  writeln!(s, "}}")?;
  writeln!(s)?;
  writeln!(s, "#[cold]")?;
  writeln!(s, "#[inline(never)]")?;
  writeln!(s, "#[cfg_attr(feature=\"track_caller\", track_caller)]")?;
  writeln!(s, "fn gl_not_loaded(name: &str) -> ! {{")?;
  writeln!(s, "  panic!(\"GL function not loaded: {{name}}\", name = name);")?;
  writeln!(s, "}}")?;
  writeln!(s)?;
  writeln!(s, "/// Loads function pointer for global-style GL usage.")?;
  writeln!(s, "/// ")?;
  writeln!(s, "/// Note: This function is effectively just a shortcut for calling each individual function pointer loader function individually.")?;
  writeln!(s, "/// The individual function pointer loaders are named `{{cmd}}_load_with`, though they are all `doc(hidden)` to avoid polluting the module docs.")?;
  writeln!(s, "/// ")?;
  writeln!(s, "/// ## Safety")?;
  writeln!(s, "/// * The \"Get Proc Address\" function you provide will always be given a pointer to the start of a null-terminated string containing the name of a GL function to load.")?;
  writeln!(s, "/// * The \"Get Proc Address\" function given must always return accurate function pointer values, or null on failure.")?;
  writeln!(s, "/// * This action alters non-atomic global memory (`static` UnsafeCell values), and it is not synchronized.")?;
  writeln!(s, "///   If your program is using GL in a multi-threaded way you **must** provide external synchronization of your own.")?;
  writeln!(s, "///   The best practice is to initialize global GL *before* spawning any other threads.")?;
  writeln!(s, "/// * This function is only safe to use if all GL contexts in your program will be able to share the same set of function pointers.")?;
  writeln!(s, "///   Here's some guidance, depending on platform:")?;
  writeln!(s, "///   * Windows: In practice, if two GL contexts come from the same vendor, and refer to the same GPU, and are for the same pixel format, then they will use identical functions.")?;
  writeln!(s, "///     See Also [the OpenGL wiki](https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions#Windows).")?;
  writeln!(s, "///   * Mac: While GL is increasingly less supported with each passing release of Mac, starting in 10.2 all GL symbols are weak-linked.")?;
  writeln!(s, "///     This means, implicitly, that a single set of symbols is valid for your whole program, regardless of a particular context.")?;
  writeln!(s, "///     You must still only call functions that are valid for your current context's API version and available extensions.")?;
  writeln!(s, "///   * Linux: On Linux the GL functions are not context specific.")?;
  writeln!(s, "///     You must still only call functions that are valid for your current context's API version and available extensions.")?;
  writeln!(s, "pub unsafe fn load_global_gl(f: &dyn Fn(*const u8) -> *const c_void) {{")?;
  //
  for cmd in nullable_commands.iter().copied() {
    writeln!(s, "  {cmd}_load_with(f);", cmd = cmd)?;
  }
  //
  writeln!(s, "}}")?;
  writeln!(s)?;
  writeln!(s, "/// Clears the global GL function settings.")?;
  writeln!(s, "/// ")?;
  writeln!(s, "/// ## Safety")?;
  writeln!(s, "/// * Similar to [`load_global_gl`].")?;
  writeln!(s, "///   Specifically, while it's safe to clear a function pointer's value (calling the function will just panic),")?;
  writeln!(s, "///   this is altering non-atomic global memory, so you **must** provide external synchronization of your own.")?;
  writeln!(s, "pub unsafe fn reset_global_gl() {{")?;
  //
  for cmd in nullable_commands.iter().copied() {
    writeln!(s, "  {cmd}_reset_ptr();", cmd = cmd)?;
  }
  //
  writeln!(s, "}}")?;
  writeln!(s)?;
  //
  for cmd in nullable_commands.iter().copied() {
    let entry = desired_command_type_entries.iter().find(|ent| ent.name == cmd).unwrap();
    let name = &entry.name;
    let unsafe_str = if entry.type_declaration.contains("unsafe") { "unsafe" } else { "" };
    let args_str = {
      let open_paren = entry.type_declaration.as_bytes().iter().copied().position(|b| b == b'(').unwrap();
      let close_paren = entry.type_declaration.as_bytes().iter().copied().position(|b| b == b')').unwrap();
      &entry.type_declaration[open_paren + 1..close_paren]
    };
    let ret_type = match entry.type_declaration.as_bytes().iter().copied().position(|b| b == b'>') {
      Some(position) => &entry.type_declaration[position + 1..entry.type_declaration.len() - 1],
      None => "",
    };
    let ret_arrow = if ret_type.is_empty() { "" } else { "->" };
    let args_names = {
      let mut s = String::new();
      for arg in args_str.split(',') {
        let arg_name = arg.split(':').next().unwrap();
        if arg_name.len() > 0 {
          write!(s, "{arg_name},", arg_name = arg_name)?;
        }
      }
      s
    };
    for docs_line in entry.comments.lines() {
      writeln!(s, "{}", docs_line)?;
    }
    writeln!(s, "#[cfg_attr(feature=\"track_caller\", track_caller)]")?;
    writeln!(s, "pub {unsafe_str} fn {name}({args_str}){ret_arrow}{ret_type} {{", name = name, unsafe_str = unsafe_str, args_str = args_str, ret_arrow = ret_arrow, ret_type = ret_type)?;
    writeln!(s, "  #[allow(unused_unsafe)]")?;
    writeln!(s, "  match unsafe {{ *{name}_p.0.get() }} {{", name = name)?;
    writeln!(s, "    Some({name}_inner) => {name}_inner({args_names}),", name = name, args_names = args_names)?;
    writeln!(s, "    None => gl_not_loaded(\"{name}\"),", name = name)?;
    writeln!(s, "  }}")?;
    writeln!(s, "}}")?;
    writeln!(s, "static {name}_p: GlFnCell<{name}_t> = GlFnCell(core::cell::UnsafeCell::new(None));", name = name)?;
    writeln!(s, "#[doc(hidden)]")?;
    writeln!(s, "pub fn {name}_is_loaded() -> bool {{", name = name)?;
    writeln!(s, "  unsafe {{ *{name}_p.0.get() }}.is_some()", name = name)?;
    writeln!(s, "}}")?;
    writeln!(s, "#[doc(hidden)]")?;
    writeln!(s, "pub unsafe fn {name}_load_with(f: &dyn Fn(*const u8) -> *const c_void) {{", name = name)?;
    writeln!(s, "  *{name}_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<{name}_t>>(gl_ptr_filter(f(b\"{name}\\0\".as_ptr())));", name = name)?;
    writeln!(s, "}}")?;
    writeln!(s, "#[doc(hidden)]")?;
    writeln!(s, "pub unsafe fn {name}_reset_ptr() {{", name = name)?;
    writeln!(s, "  *{name}_p.0.get() = None;", name = name)?;
    writeln!(s, "}}")?;
  }

  Ok(())
}
