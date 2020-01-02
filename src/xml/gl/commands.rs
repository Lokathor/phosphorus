use super::*;

/// An argument to a command.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Param {
  /// Param type.
  pub(crate) ptype: String,
  /// Param name.
  pub(crate) name: String,
  /// Param group (for enums).
  ///
  /// If there's a group declared for a non-enum param there's probably
  /// something weird going on.
  pub(crate) group: Option<String>,
  /// Param length (for pointers).
  ///
  /// If a length is declared on a non-pointer param there's probably something
  /// weird going on.
  pub len: Option<String>,
}
impl Param {
  fn rust_param_type(&self) -> String {
    if self.ptype == "GLenum" || self.ptype == "GLbitfield" {
      if let Some(s) = self.group.as_ref() {
        s.to_owned()
      } else {
        self.ptype.to_owned()
      }
    } else if self.ptype.contains('*') {
      if self.ptype == "const GLchar *const*"
        || self.ptype == "const GLchar*const*"
      {
        return "*const *const GLchar".to_owned();
      } else if self.ptype == "const void *const*" {
        return "*const *const c_void".to_owned();
      } else if self.ptype == "void **" {
        return "*mut *mut c_void".to_owned();
      }
      let star_count = self.ptype.chars().filter(|c| *c == '*').count();
      assert_eq!(star_count, 1, "unhandled: {}", self.ptype);
      // TODO: handle special pointer types for known lengths!
      let mut out = String::new();
      out.push('*');
      if self.ptype.contains("const") {
        out.push_str("const ");
        let term = if self.ptype.contains("void") {
          "c_void "
        } else {
          self.ptype.split_whitespace().nth(1).unwrap()
        };
        out.push_str(&term[..term.len() - 1]);
      } else {
        out.push_str("mut ");
        let term =
          if self.ptype.contains("void") { "c_void " } else { &self.ptype };
        out.push_str(&term[..term.len() - 1]);
      }
      out
    } else {
      self.ptype.clone()
    }
  }
}

/// A single command's info.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Command {
  /// Just the name, for easy searching.
  pub name: String,
  /// proto-return type
  pub(crate) return_type: Option<String>,
  /// The function's C prototype
  pub(crate) full_proto: String,
  /// The group that the prototype return value belongs to, if any.
  pub(crate) return_group: Option<String>,
  /// The function's arguments
  pub params: Vec<Param>,
  /// You can find this command under some alternate name.
  ///
  /// Generally the "main" version won't have an alias entry, and then one or
  /// more versions (that were extensions before being stabilized) will all
  /// mark themselves as being an alias of the main version.
  pub(crate) alias: Option<String>,
  /// This command has an equivalent "vector" version that just takes a single
  /// pointer instead of however many separate params.
  pub(crate) vecequiv: Option<String>,
}
impl Command {
  fn rust_fn_type(&self) -> String {
    let mut out = String::new();
    out.push_str("unsafe extern \"system\" fn(");
    for param in self.params.iter() {
      out.push_str(&param.rust_param_type());
      out.push_str(", ");
    }
    if self.params.len() > 0 {
      out.pop();
      out.pop();
    }
    out.push_str(")");
    if let Some(s) = self.return_type.as_ref() {
      out.push_str(" -> ");
      out.push_str(s);
    }
    out
  }

  fn param_names(&self) -> String {
    let mut out = String::new();
    for param in self.params.iter() {
      if param.name == "type" {
        out.push_str("type_");
      } else if param.name == "ref" {
        out.push_str("ref_");
      } else {
        out.push_str(&param.name);
      }
      out.push_str(", ");
    }
    if self.params.len() > 0 {
      out.pop();
      out.pop();
    }
    out
  }

  fn param_names_and_types(&self) -> String {
    let mut out = String::new();
    for param in self.params.iter() {
      if param.name == "type" {
        out.push_str("type_");
      } else if param.name == "ref" {
        out.push_str("ref_");
      } else {
        out.push_str(&param.name);
      }
      out.push_str(": ");
      out.push_str(&param.rust_param_type());
      out.push_str(", ");
    }
    if self.params.len() > 0 {
      out.pop();
      out.pop();
    }
    out
  }
}

/// Displayer for a global version of a given Command.
pub struct GlobalCommand<'c> {
  /// The Command to display.
  pub command: &'c Command,
}
impl core::fmt::Display for GlobalCommand<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let the_name =
      if f.alternate() { &self.command.name[2..] } else { &self.command.name };
    let rust_fn_type = self.command.rust_fn_type();
    let type_alias_name = format!("{}_t", the_name);
    let ptr_name = format!("{}_p", the_name);
    let str_name = format!("{}_str", the_name);
    let param_names = self.command.param_names();
    let param_names_and_types = self.command.param_names_and_types();
    let return_tag = if let Some(r_type) = self.command.return_type.as_ref() {
      format!("-> {}", r_type)
    } else {
      String::new()
    };
    let get_error = if f.alternate() { "GetError" } else { "glGetError" };
    let no_error = if f.alternate() { "NO_ERROR" } else { "GL_NO_ERROR" };
    let mut error_format_string = String::new();
    for param in self.command.params.iter() {
      if param.ptype == "GLenum" {
        error_format_string.push_str("0x{:X}, ");
      } else {
        error_format_string.push_str("{:?}, ");
      }
    }
    if self.command.params.len() > 0 {
      error_format_string.pop();
      error_format_string.pop();
    }
    let error_check = if self.command.name != "glGetError" {
      format!("#[cfg(feature = \"error_checks\")] {{ let err = {get_error}(); if err != {no_error} {{ std::println!(\"{{}}({error_format_string}) error: {{err_name}}\", &{str_name}[..{str_name}.len()-1], {param_names}{param_comma} err_name = error_name_for(err)); }} }}", get_error = get_error, no_error = no_error, param_names = param_names, str_name = str_name, error_format_string = error_format_string, param_comma = if self.command.params.len() > 0 { ", " } else { "" })
    } else {
      String::new()
    };
    write!(
      f,
      "type {type_alias_name} = {rust_fn_type};",
      type_alias_name = type_alias_name,
      rust_fn_type = rust_fn_type
    )?;
    write!(
      f,
      " static {ptr_name}: AtomicPtr<c_void> = AtomicPtr::new(null_mut());",
      ptr_name = ptr_name
    )?;
    write!(
      f,
      " const {str_name}: &str = \"{full_name}\\0\";",
      str_name = str_name,
      full_name = self.command.name
    )?;
    write!(
      f,
      " #[inline] #[doc(hidden)] pub unsafe fn load_{the_name}(loader: &mut dyn FnMut(*const c_char) -> *mut c_void) {{ {ptr_name}.store(call_loader(loader, {str_name}.as_bytes()), Ordering::SeqCst) }}",
      the_name = the_name,
      ptr_name = ptr_name,
      str_name = str_name,
    )?;
    write!(
      f,
      " #[inline] #[doc(hidden)] pub fn {the_name}_is_loaded() -> bool {{ {ptr_name}.load(Ordering::Relaxed).is_null().not() }}",
      the_name = the_name,
      ptr_name = ptr_name,
    )?;
    // TODO: Spec links on all commands.
    // TODO: doc the lengths of pointers.
    // TODO: mention alternates
    // TODO: mention vector forms.
    write!(f, " #[inline] pub unsafe fn {the_name}({param_names_and_types}){return_tag} {{ 
      #[cfg(feature = \"trace_calls\")] {{ std::println!(\"calling {{}}\", &{str_name}[..{str_name}.len()-1]); }} let p = {ptr_name}.load(Ordering::Relaxed); if p.is_null() {{ panic!(\"{{}} not loaded\", &{str_name}[..{str_name}.len()-1]) }} let out = transmute::<*mut c_void, {type_alias_name}>(p)({param_names}); {error_check} out }}", the_name = the_name, ptr_name = ptr_name, type_alias_name = type_alias_name, str_name = str_name, param_names_and_types = param_names_and_types, return_tag = return_tag, param_names = param_names, error_check = error_check)?;
    Ok(())
  }
}

/// All the commands.
#[derive(Debug, Default, Clone)]
pub struct Commands(pub(crate) Vec<Command>);

/// Grab all the commands out of the XML.
#[must_use]
pub fn pull_commands(
  it: &mut XmlIterator<'_>,
  commands: &mut Commands,
) -> Option<()> {
  loop {
    match it.next()? {
      EndTag { name: "commands" } => return Some(()),
      StartTag { name: "command", attrs } => {
        for (k, _v) in AttributeIterator::new(attrs) {
          match k {
            "comment" => (),
            _ => panic!("unknown `command` attrs: {}", attrs),
          }
        }
        let mut command = Command::default();
        let mut seen_proto = false;
        'gather_command: loop {
          match it.next()? {
            EndTag { name: "command" } => break 'gather_command,
            StartTag { name: "proto", attrs } if !seen_proto => {
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "group" => command.return_group = Some(v.to_owned()),
                  _ => panic!("unknown `proto` attrs: {}", attrs),
                }
              }
              'gather_proto: loop {
                match it.next()? {
                  EndTag { name: "proto" } => break 'gather_proto,
                  StartTag { name: "ptype", .. } => {
                    match it.next()? {
                      Text(text) => {
                        command.return_type = Some(text.to_owned());
                        command.full_proto.push_str(text);
                        command.full_proto.push(' ');
                      }
                      other => panic!("unexpected> {:?}", other),
                    }
                    match it.next()? {
                      EndTag { name: "ptype" } => (),
                      other => panic!("unexpected> {:?}", other),
                    }
                  }
                  Text(text) => command.full_proto.push_str(text),
                  StartTag { name: "name", .. } => {
                    match it.next()? {
                      Text(text) => {
                        command.name.push_str(text);
                        command.full_proto.push_str(text);
                      }
                      other => panic!("unexpected> {:?}", other),
                    }
                    match it.next()? {
                      EndTag { name: "name" } => (),
                      other => panic!("unexpected> {:?}", other),
                    }
                  }
                  other => panic!("unexpected> {:?}", other),
                }
              }
              seen_proto = true;
            }
            StartTag { name: "param", attrs } => {
              let mut group = None;
              let mut len = None;
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "group" => group = Some(v.to_owned()),
                  "len" => len = Some(v.to_owned()),
                  other => panic!("unexpected> {:?}", other),
                }
              }
              // ptype
              let mut ptype = String::new();
              'gather_ptype: loop {
                match it.next()? {
                  StartTag { name: "name", .. } => break 'gather_ptype,
                  StartTag { name: "ptype", .. } => continue,
                  EndTag { name: "ptype" } => continue,
                  Text(text) => ptype.push_str(text),
                  other => panic!("unexpected> {:?}", other),
                }
              }
              // name
              let name = match it.next()? {
                Text(text) => text.to_owned(),
                other => panic!("unexpected> {:?}", other),
              };
              match it.next()? {
                EndTag { name: "name" } => (),
                other => panic!("unexpected> {:?}", other),
              }
              match it.next()? {
                EndTag { name: "param" } => (),
                Text("[2]") => {
                  // we special case this, it's used in exactly one place ever,
                  // the `baseAndCount` param of `glPathGlyphIndexRangeNV`.
                  ptype.push_str("*");
                  assert_eq!(len, None);
                  len = Some("2".to_string());
                  match it.next()? {
                    EndTag { name: "param" } => (),
                    other => panic!("unexpected> {:?}", other),
                  }
                }
                other => panic!("unexpected> {:?}", other),
              }
              command.params.push(Param { ptype, name, group, len });
            }
            EmptyTag { name: "glx", attrs } => {
              // TODO: Not sure what the `glx` stuff is all about, we should
              // probably look into the meaning and maybe do something with the
              // information.
              let _ = attrs;
            }
            EmptyTag { name: "alias", attrs } => {
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "name" => {
                    assert_eq!(command.alias, None);
                    command.alias = Some(v.to_owned());
                  }
                  other => panic!("unexpected> {:?}", other),
                }
              }
            }
            EmptyTag { name: "vecequiv", attrs } => {
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "name" => command.vecequiv = Some(v.to_owned()),
                  other => panic!("unexpected> {:?}", other),
                }
              }
            }
            other => panic!("unexpected> {:?}", other),
          }
        }
        commands.0.push(command);
      }
      other => panic!("unexpected> {:?}", other),
    }
  }
}
