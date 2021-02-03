use magnesium::{XmlElement::*, *};

// TODO: format a struct loader

// TODO: format a global loader

/// Requires a source of C types to be in scope.
pub const CORE_TYPES: &str = r#"
#![allow(bad_style)]
fn main() {}
type c_int = i32;
type c_uint = u32;
type c_uchar = u8;
type c_float = f32;
type c_double = f64;
type c_ushort = u16;

// Note(Lokathor): Alias this internally, but the rest of the world can just see `c_void`.
pub(crate) type void = core::ffi::c_void;

/// A GL enumeration value.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GlEnum(pub c_uint);
impl core::fmt::Debug for GlEnum {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "GlEnum(0x{:X})", self.0)
  }
}
impl Clone for GlEnum {
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for GlEnum { }

/// A GL bitfield value.
///
/// You can mix around the bits of these values using standard bitwise ops.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GlBitfield(pub c_uint);
impl core::fmt::Debug for GlBitfield {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "GlBitfield(0b{:b})", self.0)
  }
}
impl Clone for GlBitfield {
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for GlBitfield { }
impl core::ops::BitAnd for GlBitfield {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self::Output {
    Self(self.0 & rhs.0)
  }
}
impl core::ops::BitAndAssign for GlBitfield {
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}
impl core::ops::BitOr for GlBitfield {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
impl core::ops::BitOrAssign for GlBitfield {
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}
impl core::ops::BitXor for GlBitfield {
  type Output = Self;
  fn bitxor(self, rhs: Self) -> Self::Output {
    Self(self.0 ^ rhs.0)
  }
}
impl core::ops::BitXorAssign for GlBitfield {
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}
impl core::ops::Not for GlBitfield {
  type Output = Self;
  fn not(self) -> Self::Output {
    Self(!self.0)
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLhandleARB(
  #[cfg(any(target_os = "ios", target_os="macos"))]
  pub *mut void,
  #[cfg(not(any(target_os = "ios", target_os="macos")))]
  pub c_uint
);

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLeglClientBufferEXT(pub *mut void);

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLeglImageOES(pub *mut void);

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLsync(pub *mut void);

#[repr(transparent)]
pub struct _cl_context(pub void);

#[repr(transparent)]
pub struct _cl_event(pub void);

pub type GLDEBUGPROC = Option<unsafe extern "system" fn(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const void)>;
pub type GLDEBUGPROCARB = GLDEBUGPROC;
pub type GLDEBUGPROCKHR = GLDEBUGPROC;
pub type GLDEBUGPROCAMD = Option<unsafe extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut void)>;
pub type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;

/// alias for `GlEnum` (rust style) to also be `GLenum` (GL style).
pub type GLenum = GlEnum;

/// alias for `GlBitfield` (rust style) to also be `GLbitfield` (GL style).
pub type GLbitfield = GlBitfield;

pub type GLboolean = c_uchar;
pub type GLvoid = core::ffi::c_void;
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint = c_int;
pub type GLuint = c_uint;
pub type GLclampx = i32;
pub type GLsizei = c_int;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLdouble = c_double;
pub type GLclampd = c_double;
pub type GLchar = u8;
pub type GLcharARB = u8;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLfixed = i32;
pub type GLintptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptr = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64 = i64;
pub type GLint64EXT = i64;
pub type GLuint64 = u64;
pub type GLuint64EXT = u64;
pub type GLhalfNV = c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

"#;

pub fn fmt_core_types(s: &mut String) {
  s.push_str(CORE_TYPES)
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct Command<'s> {
  pub ret_type: &'s str,
  pub ret_group: Option<&'s str>,
  pub ret_class: Option<&'s str>,
  pub comment: Option<&'s str>,
  pub name: &'s str,
  pub alias: Option<&'s str>,
  pub params: Vec<CommandParam<'s>>,
}

#[derive(Debug, Default)]
pub struct CommandParam<'s> {
  pub group: Option<&'s str>,
  pub class: Option<&'s str>,
  pub len: Option<&'s str>,
  pub ptype: &'s str,
  pub name: &'s str,
  pub is_const: bool,
  pub is_ptr: bool,
}
impl<'s> CommandParam<'s> {
  fn fmt_rust_type(&self, s: &mut String) -> core::fmt::Result {
    todo!()
  }
}

#[derive(Debug, Default)]
pub struct Feature<'s> {
  pub api: &'s str,
  pub name: &'s str,
  pub number: &'s str,
  pub commands_added: Vec<&'s str>,
  pub commands_removed: Vec<&'s str>,
}

#[derive(Debug, Default)]
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
