#![allow(bad_style)]
#![allow(unused_imports)]

use chlorine::*;
use magnesium::{XmlElement::*, *};

use core::fmt::Write;

pub mod gl_core_types;
pub(crate) use gl_core_types::*;

pub mod gl_enumerations;
pub(crate) use gl_enumerations::*;

pub mod gl_groups;
pub(crate) use gl_groups::*;

#[rustfmt::skip]
pub mod gl_command_types;
pub(crate) use gl_command_types::*;

pub mod gl_feature_lists;
pub(crate) use gl_feature_lists::*;

pub mod gl_extension_lists;
pub(crate) use gl_extension_lists::*;

pub mod struct_gl46fns;
pub(crate) use struct_gl46fns::*;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Enumeration<'s> {
  pub name: &'s str,
  pub value: &'s str,
  pub group: Option<&'s str>,
  pub comment: Option<&'s str>,
  pub alias: Option<&'s str>,
  pub type_: Option<&'s str>,
  pub api: Option<&'s str>,
  pub is_bitmask: bool,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Command<'s> {
  pub name: &'s str,
  pub ret_type: &'s str,
  pub ret_group: Option<&'s str>,
  pub ret_class: Option<&'s str>,
  /// comment about XML content, not a useful comment to GL users.
  pub comment: Option<&'s str>,
  pub alias: Option<&'s str>,
  pub params: Vec<CommandParam<'s>>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandParam<'s> {
  pub name: &'s str,
  pub group: Option<&'s str>,
  pub class: Option<&'s str>,
  pub len: Option<&'s str>,
  pub ptype: &'s str,
  pub is_const: bool,
  pub is_ptr: bool,
}
impl<'s> CommandParam<'s> {
  pub fn get_len_usize(&self) -> Option<usize> {
    if let Some(len) = self.len.as_ref() {
      match len.parse::<usize>() {
        Ok(count) => Some(count),
        Err(_) => {
          if len.len() > 3 {
            match len[1..len.len() - 1].parse::<usize>() {
              Ok(count) => Some(count),
              Err(_) => None,
            }
          } else {
            None
          }
        }
      }
    } else {
      None
    }
  }

  pub fn get_len_str(&self) -> Option<&'s str> {
    if self.get_len_usize().is_some() {
      None
    } else if let Some(len) = self.len.as_ref() {
      Some(len)
    } else {
      None
    }
  }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Feature<'s> {
  pub api: &'s str,
  pub name: &'s str,
  pub number: &'s str,
  pub commands_added: Vec<&'s str>,
  pub commands_removed: Vec<&'s str>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Extension<'s> {
  pub name: &'s str,
  pub supported: &'s str,
  pub commands_added: Vec<&'s str>,
}

#[derive(Debug, Default)]
pub struct Registry<'s> {
  pub enumerations: Vec<Enumeration<'s>>,
  pub commands: Vec<Command<'s>>,
  pub features: Vec<Feature<'s>>,
  pub extensions: Vec<Extension<'s>>,
}
impl<'s> Registry<'s> {
  pub fn from_element_iterator(iter: &mut impl Iterator<Item = XmlElement<'s>>) -> Self {
    let mut registry = Registry::default();

    assert_eq!(("registry", ""), iter.next().unwrap().unwrap_start_tag());
    loop {
      match iter.next().unwrap() {
        EndTag { name: "registry" } => break,
        StartTag { name: "comment", attrs: "" } => burn_to_comment_end(iter),
        StartTag { name: "types", attrs: "" } => loop {
          // Note(Lokathor): We don't parse the types of the file, we just
          // assume that they essentially won't change over time.
          match iter.next().unwrap() {
            EndTag { name: "types" } => break,
            _ => continue,
          }
        },
        StartTag { name: "groups", attrs: "" } => loop {
          // Note(Lokathor): This part of the XML will be removed soon. The same
          // info is available on each enum declaration.
          match iter.next().unwrap() {
            EndTag { name: "groups" } => break,
            _ => continue,
          }
        },
        StartTag { name: "enums", attrs } => loop {
          let is_bitmask = TagAttributeIterator::new(attrs).find(|t| t.key == "type").map(|t| t.value == "bitmask").unwrap_or(false);
          match iter.next().unwrap() {
            EndTag { name: "enums" } => break,
            EmptyTag { name: "enum", attrs } => {
              let mut entry = Enumeration::default();
              entry.is_bitmask = is_bitmask;
              for tag_attr in TagAttributeIterator::new(attrs) {
                match tag_attr.key {
                  "name" => entry.name = tag_attr.value,
                  "value" => entry.value = tag_attr.value,
                  "group" => entry.group = Some(tag_attr.value),
                  "comment" => entry.comment = Some(tag_attr.value),
                  "alias" => entry.alias = Some(tag_attr.value),
                  "type" => entry.type_ = Some(tag_attr.value),
                  "api" => entry.api = Some(tag_attr.value),
                  _ => eprintln!("unexpected enum attr: {:?}", tag_attr),
                }
              }
              registry.enumerations.push(entry);
            }
            EmptyTag { name: "unused", attrs: _ } => (),
            other => panic!("unknown: {:?}", other),
          }
        },
        EmptyTag { name: "enums", attrs: _ } => (),
        StartTag { name: "commands", attrs: _ } => loop {
          match iter.next().unwrap() {
            EndTag { name: "commands" } => break,
            StartTag { name: "command", attrs } => {
              let mut command = Command::default();
              for tag_attr in TagAttributeIterator::new(attrs) {
                match tag_attr.key {
                  "comment" => command.comment = Some(tag_attr.value),
                  _ => eprintln!("unexpected command attr: {:?}", tag_attr),
                }
              }
              // prototype (sometimes the attrs will say a group on a return type)
              let (name, proto_attrs) = iter.next().unwrap().unwrap_start_tag();
              assert_eq!(name, "proto");
              for tag_attr in TagAttributeIterator::new(proto_attrs) {
                match tag_attr.key {
                  "group" => command.ret_group = Some(tag_attr.value),
                  "class" => command.ret_class = Some(tag_attr.value),
                  _ => eprintln!("unexpected command proto attr: {:?}", tag_attr),
                }
              }
              // get the return type
              match iter.next().unwrap() {
                Text("const") => {
                  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("ptype", ""));
                  command.ret_type = iter.next().unwrap().unwrap_text();
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "ptype");
                  assert_eq!(iter.next().unwrap().unwrap_text(), "*");
                }
                Text(t) => command.ret_type = t,
                StartTag { name: "ptype", attrs: "" } => {
                  command.ret_type = iter.next().unwrap().unwrap_text();
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "ptype");
                }
                other => panic!("unknown: {:?}", other),
              }
              // get the name
              assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
              command.name = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "proto");
              // args
              'gather_param: loop {
                match iter.next().unwrap() {
                  EndTag { name: "command" } => break,
                  StartTag { name: "param", attrs } => {
                    let mut param = CommandParam::default();
                    for tag_attr in TagAttributeIterator::new(attrs) {
                      match tag_attr.key {
                        "group" => param.group = Some(tag_attr.value),
                        "class" => param.class = Some(tag_attr.value),
                        "len" => param.len = Some(tag_attr.value),
                        _ => eprintln!("unexpected command param attr: {:?}", tag_attr),
                      }
                    }
                    // we *might* see a const before the ptype
                    match iter.next().unwrap() {
                      Text("const") => {
                        param.is_const = true;
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("ptype", ""));
                      }
                      Text("const void *") => {
                        param.is_const = true;
                        param.is_ptr = true;
                        param.ptype = "void";
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                        param.name = iter.next().unwrap().unwrap_text();
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
                        command.params.push(param);
                        continue 'gather_param;
                      }
                      Text("void *") => {
                        param.is_ptr = true;
                        param.ptype = "void";
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                        param.name = iter.next().unwrap().unwrap_text();
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
                        command.params.push(param);
                        continue 'gather_param;
                      }
                      Text("void **") => {
                        param.is_ptr = true;
                        param.ptype = "*mut void";
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                        param.name = iter.next().unwrap().unwrap_text();
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
                        command.params.push(param);
                        continue 'gather_param;
                      }
                      Text("const void **") => {
                        param.is_const = true;
                        param.is_ptr = true;
                        param.ptype = "*mut void";
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                        param.name = iter.next().unwrap().unwrap_text();
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
                        command.params.push(param);
                        continue 'gather_param;
                      }
                      Text("const void *const*") => {
                        param.is_const = true;
                        param.is_ptr = true;
                        param.ptype = "*const void";
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                        param.name = iter.next().unwrap().unwrap_text();
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
                        command.params.push(param);
                        continue 'gather_param;
                      }
                      StartTag { name: "ptype", attrs: "" } => (),
                      other => panic!("unknown: {:?}", other),
                    }
                    param.ptype = iter.next().unwrap().unwrap_text();
                    assert_eq!(iter.next().unwrap().unwrap_end_tag(), "ptype");
                    // we *might* see a pointer star
                    match iter.next().unwrap() {
                      Text("*") => {
                        param.is_ptr = true;
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                      }
                      Text("**") => {
                        param.is_ptr = true;
                        match param.ptype {
                          "GLchar" => param.ptype = "*mut GLchar",
                          "GLboolean" => param.ptype = "*mut GLboolean",
                          "GLcharARB" => param.ptype = "*mut GLcharARB",
                          other => panic!("unknown: {:?}", other),
                        };
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                      }
                      Text("*const*") => {
                        assert_eq!(param.ptype, "GLchar");
                        param.is_ptr = true;
                        param.ptype = "*const GLchar";
                        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                      }
                      StartTag { name: "name", attrs: "" } => (),
                      other => panic!("unknown: {:?}", other),
                    }
                    param.name = iter.next().unwrap().unwrap_text();
                    assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                    match iter.next().unwrap() {
                      EndTag { name: "param" } => (),
                      Text(t) => {
                        param.len = Some(t);
                        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
                      }
                      other => panic!("unknown: {:?}", other),
                    }
                    command.params.push(param);
                  }
                  EmptyTag { name: "glx", attrs: _ } => (),
                  EmptyTag { name: "alias", attrs } => {
                    for tag_attr in TagAttributeIterator::new(attrs) {
                      match tag_attr.key {
                        "name" => command.alias = Some(tag_attr.value),
                        _ => eprintln!("unexpected command alias attr: {:?}", tag_attr),
                      }
                    }
                  }
                  EmptyTag { name: "vecequiv", attrs: _ } => (/* TODO */),
                  other => panic!("unknown: {:?}", other),
                }
              }
              registry.commands.push(command);
            }
            other => panic!("unknown: {:?}", other),
          }
        },
        StartTag { name: "feature", attrs } => {
          let mut feature = Feature::default();
          'gather_feature: loop {
            for tag_attr in TagAttributeIterator::new(attrs) {
              match tag_attr.key {
                "api" => feature.api = tag_attr.value,
                "name" => feature.name = tag_attr.value,
                "number" => feature.number = tag_attr.value,
                _ => eprintln!("unexpected feature attr: {:?}", tag_attr),
              }
            }
            match iter.next().unwrap() {
              EndTag { name: "feature" } => {
                registry.features.push(feature);
                break 'gather_feature;
              }
              StartTag { name: "require", attrs: _ } => 'gather_require: loop {
                match iter.next().unwrap() {
                  EndTag { name: "require" } => break 'gather_require,
                  EmptyTag { name: "type", attrs: _ } => (),
                  EmptyTag { name: "enum", attrs: _ } => (),
                  EmptyTag { name: "command", attrs } => {
                    for tag_attr in TagAttributeIterator::new(attrs) {
                      match tag_attr.key {
                        "name" => {
                          feature.commands_added.push(tag_attr.value);
                        }
                        _ => eprintln!("unexpected feature require command attr: {:?}", tag_attr),
                      }
                    }
                  }
                  other => panic!("unknown: {:?}", other),
                }
              },
              EmptyTag { name: "require", attrs: _ } => (),
              StartTag { name: "remove", attrs: _ } => loop {
                match iter.next().unwrap() {
                  EndTag { name: "remove" } => break,
                  EmptyTag { name: "type", attrs: _ } => (),
                  EmptyTag { name: "enum", attrs: _ } => (),
                  EmptyTag { name: "command", attrs } => {
                    for tag_attr in TagAttributeIterator::new(attrs) {
                      match tag_attr.key {
                        "name" => feature.commands_removed.push(tag_attr.value),
                        _ => eprintln!("unexpected feature remove command attr: {:?}", tag_attr),
                      }
                    }
                  }
                  other => panic!("unknown: {:?}", other),
                }
              },
              other => panic!("unknown: {:?}", other),
            }
          }
        }
        StartTag { name: "extensions", attrs: "" } => loop {
          match iter.next().unwrap() {
            EndTag { name: "extensions" } => break,
            EmptyTag { name: "extension", attrs: _ } => (),
            StartTag { name: "extension", attrs } => {
              let mut extension = Extension::default();
              for tag_attr in TagAttributeIterator::new(attrs) {
                match tag_attr.key {
                  "name" => extension.name = tag_attr.value,
                  "supported" => extension.supported = tag_attr.value,
                  "comment" => (),
                  _ => eprintln!("unexpected extension attr: {:?}", tag_attr),
                }
              }
              loop {
                match iter.next().unwrap() {
                  EndTag { name: "extension" } => break,
                  StartTag { name: "require", attrs: _ } => loop {
                    match iter.next().unwrap() {
                      EndTag { name: "require" } => break,
                      EmptyTag { name: "type", attrs: _ } => (),
                      EmptyTag { name: "enum", attrs: _ } => (),
                      EmptyTag { name: "command", attrs } => {
                        for tag_attr in TagAttributeIterator::new(attrs) {
                          match tag_attr.key {
                            "name" => extension.commands_added.push(tag_attr.value),
                            _ => eprintln!("({}) unexpected extension require command attr: {:?}", extension.name, tag_attr),
                          }
                        }
                      }
                      other => panic!("unknown: {:?}", other),
                    }
                  },
                  other => panic!("unknown: {:?}", other),
                }
              }
              if extension.commands_added.len() > 0 {
                registry.extensions.push(extension);
              }
            }
            other => panic!("unknown: {:?}", other),
          }
        },
        other => panic!("unknown: {:?}", other),
      }
    }

    registry.enumerations.sort();
    registry.commands.sort();
    registry.features.sort();
    for feature in registry.features.iter_mut() {
      feature.commands_added.sort();
      feature.commands_removed.sort();
    }
    registry.extensions.sort();
    for extension in registry.extensions.iter_mut() {
      extension.commands_added.sort();
    }
    registry
  }

  pub fn fmt_enumerations(&self, s: &mut String) -> core::fmt::Result {
    use core::fmt::Write;
    for enumeration in self.enumerations.iter() {
      if let Some(api) = enumeration.api.as_ref() {
        match *api {
          // Note(Lokathor): This one enum, of all enums, is defined to have a
          // different value in OpenGL than the value in OpenGLES. It's not a
          // super big deal since it's an extension-only enum. Still, we need to
          // do this special handling.
          "gles2" => {
            writeln!(s, "/// For the OpenGL ES version of `EXT_separate_shader_objects`.")?;
            writeln!(s, "#[doc(alias = \"GL_ACTIVE_PROGRAM_EXT\")]")?;
            writeln!(s, "pub const GL_ACTIVE_PROGRAM_EXT_ES: GlEnum = GlEnum(0x8259);")?;
          }
          "gl" => {
            writeln!(s, "/// For the OpenGL version of `EXT_separate_shader_objects`.")?;
            writeln!(s, "#[doc(alias = \"GL_ACTIVE_PROGRAM_EXT\")]")?;
            writeln!(s, "pub const GL_ACTIVE_PROGRAM_EXT_GL: GlEnum = GlEnum(0x8259);")?;
          }
          other => panic!("unknown: {:?}", other),
        }
      } else {
        if let Some(comment) = enumeration.comment.as_ref() {
          writeln!(s, "/// {}", comment)?;
        }
        if let Some(group) = enumeration.group.as_ref() {
          write!(s, "/// * Group: ")?;
          for (i, g) in group.split(',').enumerate() {
            write!(s, "{}[`{}`]", if i != 0 { ", " } else { "" }, g)?;
          }
          writeln!(s)?;
        }
        let e_ty = match enumeration.type_ {
          Some("u") => "c_uint",
          Some("ull") => "u64",
          Some(t) => panic!("unknown type_: {}", t),
          None => {
            if enumeration.is_bitmask {
              "GlBitfield"
            } else {
              "GlEnum"
            }
          }
        };
        let (ctor, o_paren, c_paren) = if e_ty == "c_uint" || e_ty == "u64" { ("", "", "") } else { (e_ty, "(", ")") };
        if let Some(alias) = enumeration.alias.as_ref() {
          let needs_prefix = if alias.starts_with("GL_") { "" } else { "GL_" };
          writeln!(s, "/// * Alias Of: [`{}`]", alias)?;
          writeln!(s, "pub const {name}: {e_ty} = {needs_prefix}{alias};", name = enumeration.name, e_ty = e_ty, alias = alias, needs_prefix = needs_prefix)?;
        } else {
          let max_str = if enumeration.value.as_bytes()[0] == b'-' { "u32::MAX" } else { "" };
          writeln!(s, "pub const {name}: {e_ty} = {ctor}{o_paren}{max}{val}{c_paren};", name = enumeration.name, e_ty = e_ty, val = enumeration.value, max = max_str, ctor = ctor, o_paren = o_paren, c_paren = c_paren)?;
        }
      }
      writeln!(s)?;
    }
    Ok(())
  }

  pub fn fmt_groups(&self, s: &mut String) -> core::fmt::Result {
    use std::collections::HashMap;
    let mut g: HashMap<&'s str, Vec<&'s str>> = HashMap::new();
    //
    for enumeration in self.enumerations.iter() {
      for group in enumeration.group.as_ref().unwrap_or(&"").split(',') {
        if group == "" {
          continue;
        }
        g.entry(group).or_insert(Vec::new()).push(enumeration.name);
      }
    }
    //
    let mut group_list: Vec<&'s str> = g.keys().copied().collect();
    group_list.sort();
    for group in group_list.iter() {
      g.entry(group).or_insert(Vec::new()).sort();
      for group_entry in g.entry(group).or_insert(Vec::new()).iter() {
        writeln!(s, "/// * [`{}`]", group_entry)?;
      }
      writeln!(s, "pub type {} = GlEnum;", group)?;
      writeln!(s, "")?;
    }
    //
    Ok(())
  }

  pub fn fmt_feature_lists(&self, s: &mut String) -> core::fmt::Result {
    for api_prefix in ["GL_VERSION", "GL_VERSION_ES_CM", "GL_ES_VERSION", "GL_SC_VERSION"].iter() {
      let mut out = Vec::new();
      for feature in self.features.iter().filter(|f| f.name.starts_with(api_prefix)) {
        if *api_prefix == "GL_VERSION" && feature.name.starts_with("GL_VERSION_ES_CM") {
          continue;
        }
        out.extend(feature.commands_added.iter());
        if feature.commands_removed.len() > 0 {
          out.retain(|e| !feature.commands_removed.contains(e));
        }
        out.sort();
        out.dedup();
        //
        writeln!(s, "pub const {name}: [&str; {len}] = {list:#?};", name = feature.name, len = out.len(), list = out)?;
        writeln!(s)?;
      }
    }
    Ok(())
  }

  pub fn fmt_extension_lists(&self, s: &mut String) -> core::fmt::Result {
    for extension in self.extensions.iter() {
      writeln!(s, "pub const {name}_COMMANDS: &[&str] = &{list:#?};", name = extension.name, list = extension.commands_added)?;
      writeln!(s)?;
    }
    Ok(())
  }

  pub fn fmt_command_types(&self, s: &mut String) -> core::fmt::Result {
    for command in self.commands.iter() {
      writeln!(s, "/// [{name}](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/{name}.xhtml)", name = command.name)?;
      for param in command.params.iter() {
        if let Some(group) = param.group.as_ref() {
          writeln!(s, "/// * `{param}` group: {group}", param = param.name, group = group)?;
        }
        if let Some(class) = param.class.as_ref() {
          writeln!(s, "/// * `{param}` class: {class}", param = param.name, class = class)?;
        }
        if let Some(len_str) = param.get_len_str() {
          writeln!(s, "/// * `{param}` len: {len_str}", param = param.name, len_str = len_str)?;
        }
      }
      writeln!(s, "pub type {name}_t = unsafe extern \"system\" fn(", name = command.name)?;
      for param in command.params.iter() {
        write!(
          s,
          "  {param_name}: ",
          param_name = match param.name {
            "ref" => "ref_",
            "type" => "type_",
            "box" => "box_",
            "in" => "in_",
            "struct _cl_context" => "_cl_context",
            "struct _cl_event" => "_cl_event",
            other => other,
          }
        )?;
        if param.is_ptr {
          write!(s, "*{} ", if param.is_const { "const" } else { "mut" })?;
        }
        let base_ty = match param.ptype {
          "struct _cl_context" => "_cl_context",
          "struct _cl_event" => "_cl_event",
          "GLenum" => param.group.unwrap_or("GLenum"),
          other => other,
        };
        if let Some(len_usize) = param.get_len_usize() {
          if len_usize > 1 {
            write!(s, "[{base_ty}; {len_usize}]", base_ty = base_ty, len_usize = len_usize)?
          } else {
            write!(s, "{base_ty}", base_ty = base_ty)?
          }
        } else {
          write!(s, "{base_ty}", base_ty = base_ty)?;
        }
        writeln!(s, ",")?;
      }
      write!(s, ")")?;
      if command.ret_type != "void" {
        write!(
          s,
          " -> {}",
          match command.ret_type {
            "void *" => "*mut void",
            "GLenum" => command.ret_group.unwrap_or("GLenum"),
            other => other,
          }
        )?;
      }
      writeln!(s, ";")?;
      writeln!(s)?;
    }
    Ok(())
  }
}

fn burn_to_comment_end<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  loop {
    match iter.next().unwrap() {
      EndTag { name: "comment" } => return,
      Text(_) => continue,
      other => panic!("unknown: {:?}", other),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct CommandTypeEntry {
  pub name: String,
  pub comments: String,
  pub type_declaration: String,
}

pub fn get_command_type_entries() -> Vec<CommandTypeEntry> {
  let mut entries = Vec::new();
  let mut current_entry = CommandTypeEntry::default();
  for line in include_str!("gl_command_types.rs").lines().skip(1) {
    if line.starts_with("// ") {
      continue;
    } else if line.is_empty() {
      if !current_entry.name.is_empty() {
        entries.push(current_entry);
        current_entry = CommandTypeEntry::default();
      } else {
        continue;
      }
    } else if line.starts_with("///") {
      current_entry.comments.push_str(line);
      current_entry.comments.push('\n');
    } else {
      let ty_name = line["pub type ".len()..].split(' ').next().unwrap();
      current_entry.name = ty_name[..ty_name.len() - 2].to_string();
      current_entry.type_declaration = line.to_string();
    }
  }
  if !current_entry.name.is_empty() {
    entries.push(current_entry);
  }
  entries
}

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
  writeln!(s, "      // wgl is known to sometimes give phony non-null pointer values.")?;
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
  writeln!(s, "  /// ## Safety")?;
  writeln!(s, "  /// * The \"Get Proc Address\" function will always be given a pointer to the start of a null-terminated string containing the name of a GL function to load.")?;
  writeln!(s, "  /// * The \"Get Proc Address\" function given must always return accurate function pointer values, or null on failure.")?;
  writeln!(s, "  pub unsafe fn load_from<F:Fn(*const u8) -> *const c_void>(f: F) -> Result<Self, &'static str> {{")?;
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
  }
  writeln!(s, "}}")?;

  Ok(())
}
