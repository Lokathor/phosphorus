#![allow(unused_mut)]
#![allow(unused_labels)]
#![allow(clippy::drop_ref)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::should_implement_trait)]

//! THIS CRATE IS NOT CURRENTLY IN A FIT STATE TO BE EASILY USED IN A SINGLE
//! COMMAND LINE CALL.
//!
//! But it can output correct code if you're willing to comment and uncomment
//! what's printed by the `main` in the binary, and then pipe that output into
//! whatever files.
//!
//! * `type_alias` has all the GL types, you always need this.
//! * `special_numbers` contains a few constants that GL considers "special
//!   numbers", which aren't very special as far as I can tell.
//! * `enumerations` has all other GL constants. It just lists all constants for
//!   simplicity, no need to adjust the list based on API level or anything like
//!   that.
//! * `gl_fns` contains the `GlFns` struct, which lets you load GL functions and
//!   then call the functions via method calls on the struct. The struct is
//!   quite large (hundreds of `usize` big). You can make one on the stack with
//!   the `BLANK_GL_FNS` constant, but you are advised to make the struct
//!   directly on the heap with `GlFns::new_boxed()`. Either way, then call the
//!   `load` method with a loader closure to fill in the function pointers. To
//!   check if any fn has been loaded or not first call `has_loaded` to get a
//!   `FnLoadedChecker`, then call the name of the fn you want to check.

#[allow(dead_code)]
#[doc(hidden)]
pub mod type_alias;

#[allow(dead_code)]
#[doc(hidden)]
pub mod special_numbers;

#[allow(dead_code)]
#[doc(hidden)]
pub mod enumerations;

#[allow(dead_code)]
#[doc(hidden)]
pub mod gl_fns;

use core::fmt::Write;
use magnesium::{XmlElement::*, *};

pub type StaticStr = &'static str;

#[doc(hidden)]
#[allow(unused)]
pub const FORBIDDEN_ENUMERATION: StaticStr = "GL_ACTIVE_PROGRAM_EXT";
#[doc(hidden)]
#[allow(unused)]
pub const FORBIDDEN_EXT: StaticStr = "GL_EXT_separate_shader_objects";

macro_rules! assert_attrs_comment_only {
  ($attrs:expr) => {
    for TagAttribute { key, value } in TagAttributeIterator::new($attrs) {
      match key {
        "comment" => (),
        _ => panic!("{key:?} = {value:?}"),
      }
    }
  };
}

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub types: Vec<TypeEntry>,
  pub enum_lists: Vec<EnumList>,
  pub commands: Vec<Command>,
  pub features: Vec<Feature>,
  pub extensions: Vec<Extension>,
}
impl Registry {
  pub fn from_iter(iter: &mut impl Iterator<Item = XmlElement<'static>>) -> Self {
    let mut registry = Registry::default();
    loop {
      match iter.next().unwrap() {
        EndTag { name: "registry" } => return registry,
        StartTag { name: "comment", attrs: "" } => loop {
          if let EndTag { name: "comment" } = iter.next().unwrap() {
            break;
          }
        },
        StartTag { name: "types", attrs } => do_types(attrs, &mut registry.types, iter),
        StartTag { name: "enums", attrs } => do_enums(attrs, &mut registry.enum_lists, iter),
        EmptyTag { name: "enums", attrs } => registry.enum_lists.push(EnumList::from_attrs(attrs)),
        StartTag { name: "commands", attrs } => do_commands(attrs, &mut registry.commands, iter),
        StartTag { name: "feature", attrs } => do_feature(attrs, &mut registry.features, iter),
        StartTag { name: "extensions", attrs } => {
          do_extensions(attrs, &mut registry.extensions, iter)
        }
        other => panic!("{other:?}"),
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct TypeEntry {
  pub name: StaticStr,
  pub texts: Vec<StaticStr>,
  pub comment: StaticStr,
  pub requires: StaticStr,
}
impl TypeEntry {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "comment" => s.comment = value,
        "requires" => s.requires = value,
        other => panic!("{other:?}"),
      }
    }
    s
  }
}

fn do_types(
  attrs: StaticStr, types: &mut Vec<TypeEntry>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "types" } => return,
      StartTag { name: "type", attrs } => {
        let mut ty = TypeEntry::from_attrs(attrs);
        'ty: loop {
          match iter.next().unwrap() {
            EndTag { name: "type" } => {
              types.push(ty);
              break 'ty;
            }
            Text(t) => ty.texts.push(t),
            StartTag { name: "name", attrs: "" } => {
              assert!(ty.name.is_empty());
              ty.name = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
              ty.texts.push(ty.name);
            }
            EmptyTag { name: "apientry", attrs: "" } => (/*All fn types are 'system' fn*/),
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct EnumList {
  pub namespace: StaticStr,
  pub group: StaticStr,
  pub ty: StaticStr,
  pub comment: StaticStr,
  pub vendor: StaticStr,
  pub start: StaticStr,
  pub end: StaticStr,
  pub api: StaticStr,
  pub enums: Vec<EnumEntry>,
}
impl EnumList {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "namespace" => s.namespace = value,
        "group" => s.group = value,
        "type" => s.ty = value,
        "comment" => s.comment = value,
        "vendor" => s.vendor = value,
        "start" => s.start = value,
        "end" => s.end = value,
        "api" => s.api = value,
        other => panic!("{other:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct EnumEntry {
  pub name: StaticStr,
  pub value: StaticStr,
  pub group: StaticStr,
  pub comment: StaticStr,
  pub alias: StaticStr,
  pub ty: StaticStr,
  pub api: StaticStr,
}
impl EnumEntry {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "value" => s.value = value,
        "group" => s.group = value,
        "comment" => s.comment = value,
        "alias" => s.alias = value,
        "type" => s.ty = value,
        "api" => s.api = value,
        _ => panic!("{key:?} = {value:?}"),
      }
    }
    s
  }
}

fn do_enums(
  attrs: StaticStr, enum_lists: &mut Vec<EnumList>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut enum_list = EnumList::from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "enums" } => {
        enum_lists.push(enum_list);
        return;
      }
      EmptyTag { name: "enum", attrs } => {
        enum_list.enums.push(EnumEntry::from_attrs(attrs));
      }
      EmptyTag { name: "unused", attrs } => {
        drop(attrs);
      }
      other => panic!("{other:?}"),
    }
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TypeVariant {
  /// `T`
  #[default]
  Normal,
  /// `*const T`
  ConstPtr,
  /// `*mut T`
  MutPtr,
  /// `*mut *mut T`
  MutPtrMutPtr,
  /// `*const [T; N]`
  ConstArrayPtrLit(usize),
  /// `*const [T; NAMED_CONSTANT]`
  ConstArrayPtrNamed(StaticStr),
  /// `*mut *const T`
  MutPtrConstPtr,
  /// `*const *const T`
  ConstPtrConstPtr,
  /// `[T; N]`
  ArrayLit(usize),
  /// `[[T; A]; B]`
  ArrayOfArrayLit(usize, usize),
  /// `:N`
  BitfieldsLit(usize),
}

pub fn format_type_and_variant(ty: &str, ty_variant: TypeVariant) -> String {
  #[allow(clippy::useless_format)]
  match ty_variant {
    TypeVariant::Normal => format!("{ty}"),
    TypeVariant::ConstPtr => format!("*const {ty}"),
    TypeVariant::MutPtr => format!("*mut {ty}"),
    TypeVariant::MutPtrMutPtr => format!("*mut *mut {ty}"),
    TypeVariant::ConstArrayPtrLit(n) => format!("*const [{ty}; {n}]"),
    TypeVariant::ConstArrayPtrNamed(n) => format!("*const [{ty}; {n}]"),
    TypeVariant::MutPtrConstPtr => format!("*mut *const {ty}"),
    TypeVariant::ConstPtrConstPtr => format!("*const *const {ty}"),
    TypeVariant::ArrayLit(n) => format!("[{ty}; {n}]"),
    TypeVariant::ArrayOfArrayLit(a, b) => format!("[[{ty}; {a}]; {b}]"),
    TypeVariant::BitfieldsLit(n) => format!("{ty}{{:{n}}}"),
  }
}

#[derive(Clone, Default)]
pub struct Param {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub ty_variant: TypeVariant,
  pub group: StaticStr,
  pub class: StaticStr,
  pub len: StaticStr,
}
impl Param {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "group" => s.group = value,
        "class" => s.class = value,
        "len" => s.len = value,
        other => panic!("{other:?}"),
      }
    }
    s
  }

  pub fn get_name_and_ty(&self) -> String {
    let name = self.name;
    let ty_string = format_type_and_variant(self.ty, self.ty_variant);
    format!("{name}: {ty_string}")
  }
}
impl core::fmt::Debug for Param {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name_and_ty())?;
    if !self.group.is_empty() {
      write!(f, " /* group: {:?} */", self.group)?;
    }
    if !self.class.is_empty() {
      write!(f, " /* class: {:?} */", self.class)?;
    }
    if !self.len.is_empty() {
      write!(f, " /* len: {:?} */", self.len)?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone, Default)]
pub struct Command {
  pub name: StaticStr,
  pub return_ty: StaticStr,
  pub return_ty_variant: TypeVariant,
  pub return_group: StaticStr,
  pub return_class: StaticStr,
  pub params: Vec<Param>,
  pub alias: StaticStr,
  pub vec_equiv: StaticStr,
  pub comment: StaticStr,
}
impl Command {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "comment" => s.comment = value,
        other => panic!("{other:?}"),
      }
    }
    s
  }

  pub fn get_fn_ty(&self) -> String {
    let mut s = String::from("unsafe extern \"system\" fn(");
    for param in self.params.iter() {
      write!(s, "{},", param.get_name_and_ty()).ok();
    }
    s.push(')');
    if self.return_ty != "()" {
      s.push_str("->");
      s.push_str(&format_type_and_variant(self.return_ty, self.return_ty_variant));
    }
    s
  }
}

fn do_commands(
  attrs: StaticStr, commands: &mut Vec<Command>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
    match key {
      "namespace" => assert_eq!(value, "GL"),
      other => panic!("{other:?}"),
    }
  }
  'commands: loop {
    match iter.next().unwrap() {
      EndTag { name: "commands" } => return,
      StartTag { name: "command", attrs } => {
        let mut command = Command::from_attrs(attrs);
        'command: loop {
          match iter.next().unwrap() {
            EndTag { name: "command" } => {
              commands.push(command);
              break 'command;
            }
            StartTag { name: "proto", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "group" => command.return_group = value,
                  "class" => command.return_class = value,
                  other => panic!("{other:?}"),
                }
              }
              // return type
              match iter.next().unwrap() {
                Text("void") => command.return_ty = "()",
                Text("void *") => {
                  command.return_ty = "c_void";
                  command.return_ty_variant = TypeVariant::MutPtr;
                }
                Text("const") => {
                  command.return_ty_variant = TypeVariant::ConstPtr;
                  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("ptype", ""));
                  command.return_ty = iter.next().unwrap().unwrap_text();
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "ptype");
                }
                StartTag { name: "ptype", attrs: "" } => {
                  command.return_ty = iter.next().unwrap().unwrap_text();
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "ptype");
                }
                other => panic!("{other:?}"),
              }
              // fn name
              match iter.next().unwrap() {
                Text("*") => {
                  match command.return_ty_variant {
                    TypeVariant::ConstPtr => (),
                    TypeVariant::Normal => command.return_ty_variant = TypeVariant::MutPtr,
                    other => panic!("{other:?}"),
                  }
                  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                  command.name = iter.next().unwrap().unwrap_text();
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                }
                StartTag { name: "name", attrs: "" } => {
                  command.name = iter.next().unwrap().unwrap_text();
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                }
                other => panic!("{other:?}"),
              }
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "proto");
            }
            StartTag { name: "param", attrs } => {
              let mut param = Param::from_attrs(attrs);
              match iter.next().unwrap() {
                StartTag { name: "ptype", attrs: "" } => {
                  param.ty = iter.next().unwrap().unwrap_text();
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "ptype");
                }
                Text("const") => {
                  param.ty_variant = TypeVariant::ConstPtr;
                  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("ptype", ""));
                  param.ty = iter.next().unwrap().unwrap_text();
                  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "ptype");
                }
                Text("const void *") => {
                  param.ty = "c_void";
                  param.ty_variant = TypeVariant::ConstPtr;
                }
                Text("const void **") => {
                  param.ty = "c_void";
                  param.ty_variant = TypeVariant::MutPtrConstPtr;
                }
                Text("const void *const*") => {
                  param.ty = "c_void";
                  param.ty_variant = TypeVariant::ConstPtrConstPtr;
                }
                Text("void *") => {
                  param.ty = "c_void";
                  param.ty_variant = TypeVariant::MutPtr;
                }
                Text("void **") => {
                  param.ty = "c_void";
                  param.ty_variant = TypeVariant::MutPtrMutPtr;
                }
                other => panic!("{other:?}"),
              }
              match iter.next().unwrap() {
                Text("*") => {
                  match param.ty_variant {
                    TypeVariant::ConstPtr => (),
                    TypeVariant::Normal => param.ty_variant = TypeVariant::MutPtr,
                    other => panic!("{other:?}"),
                  }
                  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                }
                Text("**") => {
                  match param.ty_variant {
                    TypeVariant::Normal => param.ty_variant = TypeVariant::MutPtrMutPtr,
                    TypeVariant::ConstPtr => param.ty_variant = TypeVariant::MutPtrConstPtr,
                    other => panic!("{other:?}"),
                  }
                  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                }
                Text("*const*") => {
                  match param.ty_variant {
                    TypeVariant::ConstPtr => param.ty_variant = TypeVariant::ConstPtrConstPtr,
                    other => panic!("{other:?}"),
                  }
                  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
                }
                StartTag { name: "name", attrs: "" } => (),
                other => panic!("{other:?}"),
              }
              param.name = iter.next().unwrap().unwrap_text();
              if param.name == "type" {
                param.name = "ty";
              }
              if param.name == "ref" {
                param.name = "reference";
              }
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
              command.params.push(param);
            }
            EmptyTag { name: "alias", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => command.alias = value,
                  _ => panic!("{key:?} = {value:?}"),
                }
              }
            }
            EmptyTag { name: "vecequiv", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => command.vec_equiv = value,
                  _ => panic!("{key:?} = {value:?}"),
                }
              }
            }
            EmptyTag { name: "glx", attrs } => drop(attrs),
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureRequire {
  pub enums: Vec<StaticStr>,
  pub commands: Vec<StaticStr>,
  pub profile: StaticStr,
}
impl FeatureRequire {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "profile" => s.profile = value,
        "comment" => (),
        other => panic!("{other:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureRemove {
  pub enums: Vec<StaticStr>,
  pub commands: Vec<StaticStr>,
  pub profile: StaticStr,
}
impl FeatureRemove {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "profile" => s.profile = value,
        "comment" => (),
        other => panic!("{other:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Feature {
  pub name: StaticStr,
  pub api: StaticStr,
  pub number: StaticStr,
  pub requirements: Vec<FeatureRequire>,
  pub removals: Vec<FeatureRemove>,
}
impl Feature {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "api" => s.api = value,
        "number" => s.number = value,
        other => panic!("{other:?}"),
      }
    }
    s
  }
}

fn do_feature(
  attrs: StaticStr, features: &mut Vec<Feature>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut feature = Feature::from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "feature" } => {
        features.push(feature);
        return;
      }
      StartTag { name: "require", attrs } => {
        let mut require = FeatureRequire::from_attrs(attrs);
        'require: loop {
          match iter.next().unwrap() {
            EndTag { name: "require" } => {
              feature.requirements.push(require);
              break 'require;
            }
            EmptyTag { name: "type", attrs } => drop(attrs),
            EmptyTag { name: "enum", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => require.enums.push(value),
                  "comment" => (),
                  other => panic!("{other:?}"),
                }
              }
            }
            EmptyTag { name: "command", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => require.commands.push(value),
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        }
      }
      EmptyTag { name: "require", attrs } => drop(attrs),
      StartTag { name: "remove", attrs } => {
        let mut remove = FeatureRemove::from_attrs(attrs);
        'remove: loop {
          match iter.next().unwrap() {
            EndTag { name: "remove" } => {
              feature.removals.push(remove);
              break 'remove;
            }
            EmptyTag { name: "type", attrs } => drop(attrs),
            EmptyTag { name: "enum", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => remove.enums.push(value),
                  other => panic!("{other:?}"),
                }
              }
            }
            EmptyTag { name: "command", attrs } => {
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "name" => remove.commands.push(value),
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequireList {
  pub api: StaticStr,
  pub profile: StaticStr,
  pub enums: Vec<StaticStr>,
  pub commands: Vec<StaticStr>,
}
impl RequireList {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "api" => s.api = value,
        "profile" => s.profile = value,
        "comment" => (),
        other => panic!("{other:?}"),
      }
    }
    s
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub name: StaticStr,
  pub comment: StaticStr,
  pub supported: StaticStr,
  pub require_lists: Vec<RequireList>,
}
impl Extension {
  fn from_attrs(attrs: StaticStr) -> Self {
    let mut s = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => s.name = value,
        "supported" => s.supported = value,
        "comment" => s.comment = value,
        other => panic!("{other:?}"),
      }
    }
    s
  }
}

fn do_extensions(
  attrs: StaticStr, extensions: &mut Vec<Extension>,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "extensions" } => return,
      EmptyTag { name: "extension", attrs } => extensions.push(Extension::from_attrs(attrs)),
      StartTag { name: "extension", attrs } => {
        let mut extension = Extension::from_attrs(attrs);
        'extension: loop {
          match iter.next().unwrap() {
            EndTag { name: "extension" } => {
              extensions.push(extension);
              break 'extension;
            }
            StartTag { name: "require", attrs } => {
              let mut require_list = RequireList::from_attrs(attrs);
              'require: loop {
                match iter.next().unwrap() {
                  EndTag { name: "require" } => {
                    extension.require_lists.push(require_list);
                    break 'require;
                  }
                  EmptyTag { name: "enum", attrs } => {
                    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                      match key {
                        "name" => require_list.enums.push(value),
                        "comment" => (),
                        other => panic!("{other:?}"),
                      }
                    }
                  }
                  EmptyTag { name: "command", attrs } => {
                    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                      match key {
                        "name" => require_list.commands.push(value),
                        "comment" => (),
                        other => panic!("{other:?}"),
                      }
                    }
                  }
                  EmptyTag { name: "type", attrs } => drop(attrs),
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}
