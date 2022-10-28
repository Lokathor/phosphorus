#![allow(clippy::from_str_radix_10)]
#![allow(clippy::field_reassign_with_default)]

use magnesium::{
  skip_comments, skip_empty_text_elements, trim_text, ElementIterator,
  TagAttributeIterator,
  XmlElement::{self, *},
};

fn main() {
  let mut args_list: Vec<String> = std::env::args().collect();

  let file_name = if args_list.len() == 2 {
    args_list.pop().unwrap()
  } else {
    println!("USAGE: phosphorus [gl.xml]");
    return;
  };
  println!("Reading File: {file_name:?}");
  let gl_xml = std::fs::read_to_string(file_name.as_str()).unwrap();
  let mut elem_iter = ElementIterator::new(&gl_xml)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  let registry = match elem_iter.next().unwrap() {
    StartTag { name: "registry", attrs: "" } => Registry::new(&mut elem_iter),
    _ => panic!("couldn't read the `registry` element"),
  };
  println!("registry: {registry:#?}");
}

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub enumerations: Vec<Enumeration>,
}
impl Registry {
  pub fn new<'a>(mut elem_iter: impl Iterator<Item = XmlElement<'a>>) -> Self {
    let mut reg = Self::default();
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "registry" } => return reg,
        StartTag { name: "comment", attrs: "" } => {
          // for some goofy reason, gl.xml has comments in matched comment tags
          // as well as in the usual `<!--` and `-->`
          burn_until("comment", &mut elem_iter)
        }
        StartTag { name: "types", attrs: "" } => {
          // We won't bother to read the `types` tree at all. It's badly
          // formatted because it contains literal C code and junk. Instead, see
          // the `gl_core_types` module.
          burn_until("types", &mut elem_iter)
        }
        EmptyTag { name: "enums", attrs: _ } => {
          // we don't need to keep empty `enums` instances, since they don't
          // hold any enum values.
        }
        StartTag { name: "enums", attrs } => 'enums: loop {
          let is_bitmask = TagAttributeIterator::new(attrs)
            .any(|attr| attr.key == "type" && attr.value == "bitmask");
          match elem_iter.next().unwrap() {
            EndTag { name: "enums" } => break 'enums,
            EmptyTag { name: "unused", attrs: _ } => (),
            EmptyTag { name: "enum", attrs } => {
              let mut enumeration = Enumeration::default();
              enumeration.is_bitmask = is_bitmask;
              for attr in TagAttributeIterator::new(attrs) {
                match attr.key {
                  "value" => enumeration.value = attr.value.to_string(),
                  "name" => enumeration.name = attr.value.to_string(),
                  "group" => enumeration.group = attr.value.to_string(),
                  "alias" => enumeration.alias = Some(attr.value.to_string()),
                  "type" => enumeration.type_ = Some(attr.value.to_string()),
                  "api" => enumeration.api = Some(attr.value.to_string()),
                  "comment" => (),
                  other => panic!("{other:?}"),
                }
              }
              reg.enumerations.push(enumeration);
            }
            other => panic!("{other:?}"),
          }
        },
        StartTag { name: "commands", attrs: r#"namespace="GL""# } => {
          burn_until("commands", &mut elem_iter)
        }
        StartTag { name: "feature", attrs } => {
          println!("feature_attrs: {attrs:?}");
          burn_until("feature", &mut elem_iter)
        }
        StartTag { name: "extensions", attrs: "" } => {
          burn_until("extensions", &mut elem_iter)
        }
        other => {
          println!("UNKNOWN: Registry: {other:?}");
          return reg;
        }
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Enumeration {
  pub value: String,
  pub name: String,
  pub group: String,
  pub alias: Option<String>,
  pub type_: Option<String>,
  pub api: Option<String>,
  pub is_bitmask: bool,
}

fn burn_until<'a>(
  end: &str, elem_iter: &mut impl Iterator<Item = XmlElement<'a>>,
) {
  loop {
    if let EndTag { name } = elem_iter.next().unwrap() {
      if end == name {
        return;
      }
    }
  }
}
