#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use magnesium::{XmlElement::*, *};
use std::collections::HashMap;
use tinyvec::*;

fn main() {
  let file_bytes = std::fs::read_to_string("xml/gl.xml").unwrap();
  let mut indent = 0;
  let mut iter = &mut ElementIterator::new(&file_bytes)
    .filter_map(skip_comments)
    .filter_map(skip_empty_text_elements);
  //
  let registry = match iter.next().unwrap() {
    StartTag { name: "registry", attrs: "" } => parse_registry(iter),
    unknown => panic!("{:?}", unknown),
  };
}

fn hashmap_from_attrs(attrs: &str) -> HashMap<String, String> {
  TagAttributeIterator::new(attrs)
    .map(|ta| (ta.key.to_string(), ta.value.to_string()))
    .collect()
}

#[derive(Debug, Clone, Default)]
pub struct Registry {
  types: Option<Types>,
  groups: Option<Groups>,
  enumerants: Vec<Enumerants>,
  features: Vec<Feature>,
  commands: Option<Commands>,
  extensions: Option<Extensions>,
}

fn parse_registry<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
) -> Registry {
  let mut registry = Registry::default();
  loop {
    match iter.next().unwrap() {
      EndTag { name: "registry" } => return registry,
      StartTag { name: "comment", attrs: "" } => parse_comment(iter),
      StartTag { name: "types", attrs: "" } => {
        assert!(registry.types.is_none());
        registry.types = Some(parse_types(iter))
      }
      StartTag { name: "groups", attrs: "" } => {
        assert!(registry.groups.is_none());
        registry.groups = Some(parse_groups(iter))
      }
      StartTag { name: "enums", attrs } => {
        registry.enumerants.push(parse_enums(iter, attrs));
      }
      EmptyTag { name: "enums", attrs } => (),
      StartTag { name: "commands", attrs } => {
        assert!(registry.commands.is_none());
        registry.commands = Some(parse_commands(iter, attrs))
      }
      StartTag { name: "feature", attrs } => {
        registry.features.push(parse_feature(iter, attrs));
      }
      StartTag { name: "extensions", attrs: "" } => {
        assert!(registry.extensions.is_none());
        registry.extensions = Some(parse_extensions(iter));
      }

      unknown => println!("parse_registry:{:?}", unknown),
    }
  }
}

fn parse_comment<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  loop {
    match iter.next().unwrap() {
      EndTag { name: "comment" } => return,
      Text(t) => (),
      unknown => println!("parse_comment:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Types {
  type_entries: Vec<TypeEntry>,
}

fn parse_types<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) -> Types {
  let mut types = Types::default();
  loop {
    match iter.next().unwrap() {
      EndTag { name: "types" } => return types,
      StartTag { name: "type", attrs } => {
        types.type_entries.push(parse_type(iter, attrs));
      }
      unknown => println!("parse_types:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct TypeEntry {
  attrs: HashMap<String, String>,
  text: String,
}

fn parse_type<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> TypeEntry {
  let mut type_entry = TypeEntry::default();
  type_entry.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => return type_entry,
      Text(t) => type_entry.text.push_str(t),
      StartTag { name: "name", attrs: "" } => (),
      EndTag { name: "name" } => (),
      EmptyTag { name: "apientry", attrs: "" } => (),
      unknown => println!("parse_type:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Groups {
  groups: Vec<Group>,
}

fn parse_groups<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) -> Groups {
  let mut groups = Groups::default();
  loop {
    match iter.next().unwrap() {
      EndTag { name: "groups" } => return groups,
      StartTag { name: "group", attrs } => {
        groups.groups.push(parse_group(iter, attrs));
      }
      EmptyTag { name: "group", attrs } => continue,
      unknown => println!("parse_groups:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Group {
  attrs: HashMap<String, String>,
  enumerants: Vec<String>,
}

fn parse_group<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Group {
  let mut group = Group::default();
  group.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "group" } => return group,
      EmptyTag { name: "enum", attrs } => {
        let hm = hashmap_from_attrs(attrs);
        group.enumerants.push(hm["name"].to_string());
      }
      unknown => println!("parse_group:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Enumerants {
  attrs: HashMap<String, String>,
  enumerants: Vec<Enumerant>,
}

#[derive(Debug, Clone, Default)]
pub struct Enumerant {
  attrs: HashMap<String, String>,
}

fn parse_enums<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Enumerants {
  let mut enumerants = Enumerants::default();
  enumerants.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "enums" } => return enumerants,
      EmptyTag { name: "enum", attrs } => {
        let attrs = hashmap_from_attrs(attrs);
        enumerants.enumerants.push(Enumerant { attrs });
      }
      EmptyTag { name: "unused", attrs } => (),
      unknown => println!("parse_enums:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Commands {
  attrs: HashMap<String, String>,
  commands: Vec<Command>,
}

fn parse_commands<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Commands {
  let mut commands = Commands::default();
  commands.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "commands" } => return commands,
      StartTag { name: "command", attrs } => {
        commands.commands.push(parse_command(iter, attrs));
      }
      unknown => println!("parse_commands:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Command {
  attrs: HashMap<String, String>,
  prototype: Prototype,
  params: Vec<Param>,
  glx_attrs: HashMap<String, String>,
  alias: String,
  vec_equivalent: String,
}

fn parse_command<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Command {
  let mut command = Command::default();
  command.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "command" } => return command,
      StartTag { name: "proto", attrs } => {
        command.prototype = parse_proto(iter, attrs);
      }
      StartTag { name: "param", attrs } => {
        command.params.push(parse_param(iter, attrs));
      }
      EmptyTag { name: "glx", attrs } => {
        command.glx_attrs = hashmap_from_attrs(attrs);
      }
      EmptyTag { name: "alias", attrs } => {
        command.alias = hashmap_from_attrs(attrs)["name"].to_string();
      }
      EmptyTag { name: "vecequiv", attrs } => {
        command.alias = hashmap_from_attrs(attrs)["name"].to_string();
      }
      unknown => println!("parse_command:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Prototype {
  attrs: HashMap<String, String>,
  text: String,
}

fn parse_proto<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Prototype {
  let mut prototype = Prototype::default();
  prototype.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "proto" } => return prototype,
      Text(t) => prototype.text.push_str(t),
      StartTag { name: "ptype", attrs: "" } => (),
      EndTag { name: "ptype" } => (),
      StartTag { name: "name", attrs: "" } => (),
      EndTag { name: "name" } => (),
      unknown => println!("parse_proto:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Param {
  attrs: HashMap<String, String>,
  text: String,
}

fn parse_param<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Param {
  let mut param = Param::default();
  param.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "param" } => return param,
      Text(t) => param.text.push_str(t),
      StartTag { name: "ptype", attrs: "" } => (),
      EndTag { name: "ptype" } => (),
      StartTag { name: "name", attrs: "" } => (),
      EndTag { name: "name" } => (),
      unknown => println!("parse_param:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Feature {
  attrs: HashMap<String, String>,
  requirements: Vec<Requirements>,
  removals: Vec<Removals>,
}

fn parse_feature<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Feature {
  let mut feature = Feature::default();
  feature.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "feature" } => return feature,
      StartTag { name: "require", attrs } => {
        feature.requirements.push(parse_require(iter, attrs));
      }
      EmptyTag { name: "require", attrs } => continue,
      StartTag { name: "remove", attrs } => {
        feature.removals.push(parse_remove(iter, attrs));
      }
      unknown => println!("parse_feature:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Requirements {
  attrs: HashMap<String, String>,
  types: Vec<String>,
  enumerants: Vec<String>,
  commands: Vec<String>,
}

fn parse_require<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Requirements {
  let mut requirements = Requirements::default();
  requirements.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "require" } => return requirements,
      EmptyTag { name: "type", attrs } => {
        requirements.types.push(hashmap_from_attrs(attrs)["name"].to_string());
      }
      EmptyTag { name: "enum", attrs } => {
        requirements
          .enumerants
          .push(hashmap_from_attrs(attrs)["name"].to_string());
      }
      EmptyTag { name: "command", attrs } => {
        requirements
          .commands
          .push(hashmap_from_attrs(attrs)["name"].to_string());
      }
      unknown => println!("parse_require:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Removals {
  attrs: HashMap<String, String>,
  types: Vec<String>,
  enumerants: Vec<String>,
  commands: Vec<String>,
}

fn parse_remove<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Removals {
  let mut removals = Removals::default();
  removals.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "remove" } => return removals,
      EmptyTag { name: "type", attrs } => {
        removals.types.push(hashmap_from_attrs(attrs)["name"].to_string());
      }
      EmptyTag { name: "enum", attrs } => {
        removals.enumerants.push(hashmap_from_attrs(attrs)["name"].to_string());
      }
      EmptyTag { name: "command", attrs } => {
        removals.commands.push(hashmap_from_attrs(attrs)["name"].to_string());
      }
      unknown => println!("parse_remove:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extensions {
  extensions: Vec<Extension>,
}

fn parse_extensions<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
) -> Extensions {
  let mut extensions = Extensions::default();
  loop {
    match iter.next().unwrap() {
      EndTag { name: "extensions" } => return extensions,
      unknown => println!("parse_extensions:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extension;
