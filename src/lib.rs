#![allow(clippy::should_implement_trait)]

use magnesium::{XmlElement::*, *};

pub type StaticStr = &'static str;

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
    let mut s = TypeEntry::default();
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
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}
