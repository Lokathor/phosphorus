#![allow(clippy::from_str_radix_10)]
#![allow(clippy::field_reassign_with_default)]

use std::path::PathBuf;

use clap::Parser;
use magnesium::{
  skip_comments, skip_empty_text_elements, trim_text, ElementIterator,
  TagAttributeIterator,
  XmlElement::{self, *},
};

#[derive(Parser, Debug)]
struct Args {
  /// gl.xml file
  #[arg(long)]
  xml: PathBuf,

  #[arg(long)]
  api: String,

  #[arg(long)]
  name: String,

  #[arg(long)]
  number: String,

  /// List of extensions to add
  #[arg(long)]
  ext: Vec<String>,
}

macro_rules! list_features_and_return {
  ($features:expr) => {{
    println!("Available features include:");
    for f in $features.iter() {
      let api = &f.api;
      let name = &f.name;
      let number = &f.number;
      println!("* api:{api}, name:{name}, number:{number}");
    }
    return;
  }};
}

fn main() {
  let args = Args::parse();
  println!("parsed args as: {args:?}");

  let gl_xml = std::fs::read_to_string(&args.xml).unwrap();
  let mut elem_iter = ElementIterator::new(&gl_xml)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  let registry = match elem_iter.next().unwrap() {
    StartTag { name: "registry", attrs: "" } => Registry::new(&mut elem_iter),
    _ => panic!("couldn't read the `registry` element"),
  };

  let api_entries: Vec<&Feature> =
    registry.features.iter().filter(|f| f.api == args.api).collect();
  if api_entries.is_empty() {
    println!("No features found with that API.");
    list_features_and_return!(registry.features);
  }
  println!(
    "Matching APIs: {:?}",
    api_entries.iter().map(|f| f.name.as_str()).collect::<Vec<_>>()
  );

  let mut total_enumerations = Vec::new();
  let mut total_commands = Vec::new();
  let mut success = false;
  for feature in api_entries.iter() {
    total_enumerations.extend_from_slice(&feature.required_enumerations);
    total_commands.extend_from_slice(&feature.required_commands);
    if !feature.removed_commands.is_empty()
      || !feature.removed_enumerations.is_empty()
    {
      println!("TODO: PROCESS THE API REMOVALS");
    }

    if feature.name == args.name && feature.number == args.number {
      success = true;
      break;
    }
  }
  if !success {
    println!("Could not find an exact feature match!");
    list_features_and_return!(registry.features);
  }

  for ex in args.ext.iter() {
    let extension: &Extension =
      match registry.extensions.iter().find(|x| &x.name == ex) {
        Some(x) => x,
        None => {
          println!("Requested extension `{ex}` is not available.");
          return;
        }
      };
    if !extension.supported.contains(&args.api) {
      println!(
        "Found extension `{ex}`, but it isn't supported by API `{}`.",
        args.api
      );
      return;
    }
    // TODO: add the debug extension
  }

  println!(
    "Found {} enumerations and {} commands.",
    total_enumerations.len(),
    total_commands.len()
  );
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

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub enumerations: Vec<Enumeration>,
  pub commands: Vec<Command>,
  pub features: Vec<Feature>,
  pub extensions: Vec<Extension>,
}
impl Registry {
  pub fn new<'a>(mut elem_iter: impl Iterator<Item = XmlElement<'a>>) -> Self {
    let mut registry = Self::default();
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "registry" } => return registry,
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
                  "group" => enumeration.group = Some(attr.value.to_string()),
                  "alias" => enumeration.alias = Some(attr.value.to_string()),
                  "type" => enumeration.type_ = Some(attr.value.to_string()),
                  "api" => enumeration.api = Some(attr.value.to_string()),
                  "comment" => (),
                  other => panic!("{other:?}"),
                }
              }
              registry.enumerations.push(enumeration);
            }
            other => panic!("{other:?}"),
          }
        },
        StartTag { name: "commands", attrs: r#"namespace="GL""# } => {
          'commands: loop {
            match elem_iter.next().unwrap() {
              EndTag { name: "commands" } => {
                break 'commands;
              }
              StartTag { name: "command", attrs } => {
                registry.commands.push(Command::new(&mut elem_iter, attrs))
              }
              other => panic!("{other:?}"),
            }
          }
        }
        StartTag { name: "feature", attrs } => {
          registry.features.push(Feature::new(&mut elem_iter, attrs))
        }
        StartTag { name: "extensions", attrs: "" } => 'extensions: loop {
          match elem_iter.next().unwrap() {
            EndTag { name: "extensions" } => break 'extensions,
            EmptyTag { name: "extension", attrs } => {
              let mut extension = Extension::default();
              for attr in TagAttributeIterator::new(attrs) {
                match attr.key {
                  "name" => extension.name = attr.value.to_string(),
                  "supported" => extension.supported = attr.value.to_string(),
                  other => panic!("{other:?}"),
                }
              }
              registry.extensions.push(extension);
            }
            StartTag { name: "extension", attrs } => {
              registry.extensions.push(Extension::new(&mut elem_iter, attrs))
            }
            other => panic!("{other:?}"),
          }
        },
        other => panic!("{other:?}"),
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Enumeration {
  pub value: String,
  pub name: String,
  pub group: Option<String>,
  pub alias: Option<String>,
  pub type_: Option<String>,
  pub api: Option<String>,
  pub is_bitmask: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Command {
  pub return_ty: String,
  pub return_group: Option<String>,
  pub return_class: Option<String>,
  pub name: String,
  pub params: Vec<Param>,
  /// This command is an alias of another command, named here.
  pub alias: Option<String>,
}
impl Command {
  pub fn new<'a>(
    elem_iter: &mut impl Iterator<Item = XmlElement<'a>>, attrs: &str,
  ) -> Self {
    let mut command = Command::default();
    for attr in TagAttributeIterator::new(attrs) {
      if attr.key == "comment" {
        continue;
      }
      println!("command_attr: {attr:?}");
    }
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "command" } => {
          return command;
        }
        StartTag { name: "proto", attrs } => {
          // proto attrs
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "group" => command.return_group = Some(attr.value.to_string()),
              "class" => command.return_class = Some(attr.value.to_string()),
              other => panic!("{other:?}"),
            }
          }
          // return type
          match elem_iter.next().unwrap() {
            Text("void") => command.return_ty = "()".to_string(),
            Text("void *") => command.return_ty = "*mut c_void".to_string(),
            Text("const") => {
              assert_eq!(
                elem_iter.next().unwrap(),
                StartTag { name: "ptype", attrs: "" }
              );
              let t = match elem_iter.next().unwrap() {
                Text(t) => t.to_string(),
                other => panic!("{other:?}"),
              };
              assert_eq!(elem_iter.next().unwrap(), EndTag { name: "ptype" });
              assert_eq!(elem_iter.next().unwrap(), Text("*"));
              command.return_ty = format!("*const {t}");
            }
            StartTag { name: "ptype", attrs: "" } => {
              match elem_iter.next().unwrap() {
                Text(t) => command.return_ty = t.to_string(),
                other => panic!("{other:?}"),
              }
              assert_eq!(elem_iter.next().unwrap(), EndTag { name: "ptype" });
            }
            other => panic!("{other:?}"),
          }
          // name
          match elem_iter.next().unwrap() {
            StartTag { name: "name", attrs: "" } => {
              match elem_iter.next().unwrap() {
                Text(t) => command.name = t.to_string(),
                other => panic!("{other:?}"),
              }
              assert_eq!(elem_iter.next().unwrap(), EndTag { name: "name" });
            }
            other => panic!("{other:?}"),
          }
          assert_eq!(elem_iter.next().unwrap(), EndTag { name: "proto" });
        }
        StartTag { name: "param", attrs } => {
          let mut param = Param::default();
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "group" => param.group = Some(attr.value.to_string()),
              "class" => param.class = Some(attr.value.to_string()),
              "len" => param.len = Some(attr.value.to_string()),
              other => panic!("{other:?}"),
            }
          }
          'param: loop {
            match elem_iter.next().unwrap() {
              EndTag { name: "param" } => {
                command.params.push(param);
                break 'param;
              }
              StartTag { name: "ptype", attrs: "" } => {
                match elem_iter.next().unwrap() {
                  Text(t) => param.param_ty = t.to_string(),
                  other => panic!("{other:?}"),
                }
                assert_eq!(elem_iter.next().unwrap(), EndTag { name: "ptype" });
              }
              Text("const") => {
                assert_eq!(
                  elem_iter.next().unwrap(),
                  StartTag { name: "ptype", attrs: "" }
                );
                let t = match elem_iter.next().unwrap() {
                  Text(t) => t.to_string(),
                  other => panic!("{other:?}"),
                };
                assert_eq!(elem_iter.next().unwrap(), EndTag { name: "ptype" });
                match elem_iter.next().unwrap() {
                  Text("*") => param.param_ty = format!("*const {t}"),
                  Text("*const*") => {
                    param.param_ty = format!("*const *const {t}")
                  }
                  Text("**") => param.param_ty = format!("*const *mut {t}"),
                  other => panic!("{other:?}"),
                }
              }
              Text("const void *") => {
                param.param_ty = "*const c_void".to_string();
              }
              Text("const void **") => {
                param.param_ty = "*const *mut c_void".to_string();
              }
              Text("void *") => {
                param.param_ty = "*mut c_void".to_string();
              }
              Text("void **") => {
                param.param_ty = "*mut *mut c_void".to_string();
              }
              Text("const void *const*") => {
                param.param_ty = "*const *const c_void".to_string();
              }
              Text("*") => {
                param.param_ty = format!("*mut {}", param.param_ty);
              }
              StartTag { name: "name", attrs: "" } => {
                match elem_iter.next().unwrap() {
                  Text(t) => param.name = t.to_string(),
                  other => panic!("{other:?}"),
                }
                assert_eq!(elem_iter.next().unwrap(), EndTag { name: "name" });
              }
              other => panic!("{other:?}, Command: {command:?}"),
            }
          }
        }
        EmptyTag { name: "glx", .. } => (),
        EmptyTag { name: "vecequiv", .. } => (),
        EmptyTag { name: "alias", attrs } => {
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "name" => command.alias = Some(attr.value.to_string()),
              other => panic!("{other:?}"),
            }
          }
        }
        other => panic!("{other:?}"),
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Param {
  pub param_ty: String,
  pub name: String,
  pub group: Option<String>,
  pub class: Option<String>,
  pub len: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum Profile {
  Core,
  Compatibility,
}

#[derive(Debug, Clone, Default)]
pub struct Feature {
  pub api: String,
  pub name: String,
  pub number: String,
  /// (name, profile)
  pub required_enumerations: Vec<(String, Option<Profile>)>,
  /// (name, profile)
  pub required_commands: Vec<(String, Option<Profile>)>,
  /// (name, profile)
  pub removed_enumerations: Vec<(String, Option<Profile>)>,
  /// (name, profile)
  pub removed_commands: Vec<(String, Option<Profile>)>,
}
impl Feature {
  pub fn new<'a>(
    elem_iter: &mut impl Iterator<Item = XmlElement<'a>>, attrs: &str,
  ) -> Self {
    let mut feature = Feature::default();
    for attr in TagAttributeIterator::new(attrs) {
      match attr.key {
        "api" => feature.api = attr.value.to_string(),
        "name" => feature.name = attr.value.to_string(),
        "number" => feature.number = attr.value.to_string(),
        other => panic!("{other:?}"),
      }
    }
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "feature" } => return feature,
        EmptyTag { name: "require", .. } => (),
        StartTag { name: "require", attrs } => {
          let mut profile = None;
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "comment" => (),
              "profile" => match attr.value {
                "core" => profile = Some(Profile::Core),
                "compatibility" => profile = Some(Profile::Compatibility),
                "common" => (),
                other => panic!("{other:?}"),
              },
              other => panic!("{other:?}"),
            }
          }
          'require: loop {
            match elem_iter.next().unwrap() {
              EndTag { name: "require" } => break 'require,
              EmptyTag { name: "type", .. } => continue,
              EmptyTag { name: "enum", attrs } => {
                for attr in TagAttributeIterator::new(attrs) {
                  match attr.key {
                    "name" => feature
                      .required_enumerations
                      .push((attr.value.to_string(), profile)),
                    "comment" => (),
                    other => panic!("{other:?}"),
                  }
                }
              }
              EmptyTag { name: "command", attrs } => {
                for attr in TagAttributeIterator::new(attrs) {
                  match attr.key {
                    "name" => feature
                      .required_commands
                      .push((attr.value.to_string(), profile)),
                    other => panic!("{other:?}"),
                  }
                }
              }
              other => panic!("{other:?}"),
            }
          }
        }
        StartTag { name: "remove", attrs } => {
          let mut profile = None;
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "comment" => (),
              "profile" => match attr.value {
                "core" => profile = Some(Profile::Core),
                "compatibility" => profile = Some(Profile::Compatibility),
                "common" => (),
                other => panic!("{other:?}"),
              },
              other => panic!("{other:?}, Feature: {:?}", feature.name),
            }
          }
          'remove: loop {
            match elem_iter.next().unwrap() {
              EndTag { name: "remove" } => break 'remove,
              EmptyTag { name: "type", .. } => continue,
              EmptyTag { name: "enum", attrs } => {
                for attr in TagAttributeIterator::new(attrs) {
                  match attr.key {
                    "name" => feature
                      .removed_enumerations
                      .push((attr.value.to_string(), profile)),
                    other => panic!("{other:?}"),
                  }
                }
              }
              EmptyTag { name: "command", attrs } => {
                for attr in TagAttributeIterator::new(attrs) {
                  match attr.key {
                    "name" => feature
                      .removed_commands
                      .push((attr.value.to_string(), profile)),
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
}

#[derive(Debug, Clone, Default)]
pub struct ExtensionRequirement {
  api: Option<String>,
  profile: Option<String>,
  enumerations: Vec<String>,
  commands: Vec<String>,
}
impl ExtensionRequirement {
  pub fn new<'a>(
    elem_iter: &mut impl Iterator<Item = XmlElement<'a>>, attrs: &str,
  ) -> Self {
    let mut requirement = ExtensionRequirement::default();
    for attr in TagAttributeIterator::new(attrs) {
      match attr.key {
        "comment" => (),
        "api" => requirement.api = Some(attr.value.to_string()),
        "profile" => requirement.profile = Some(attr.value.to_string()),
        other => panic!("{other:?}"),
      }
    }
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "require" } => return requirement,
        EmptyTag { name: "enum", attrs } => {
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "name" => requirement.enumerations.push(attr.value.to_string()),
              "comment" => (),
              other => panic!("{other:?}"),
            }
          }
        }
        EmptyTag { name: "command", attrs } => {
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "name" => requirement.commands.push(attr.value.to_string()),
              "comment" => (),
              other => panic!("{other:?}"),
            }
          }
        }
        EmptyTag { name: "type", attrs: _ } => (),
        other => panic!("{other:?}"),
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub name: String,
  pub supported: String,
  pub requirements: Vec<ExtensionRequirement>,
}
impl Extension {
  /// works for StartTag, not EmptyTag
  pub fn new<'a>(
    elem_iter: &mut impl Iterator<Item = XmlElement<'a>>, attrs: &str,
  ) -> Self {
    let mut extension = Extension::default();
    for attr in TagAttributeIterator::new(attrs) {
      match attr.key {
        "name" => extension.name = attr.value.to_string(),
        "supported" => extension.supported = attr.value.to_string(),
        "comment" => (),
        other => panic!("{other:?}"),
      }
    }
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "extension" } => return extension,
        StartTag { name: "require", attrs } => extension
          .requirements
          .push(ExtensionRequirement::new(elem_iter, attrs)),
        other => panic!("{other:?}"),
      }
    }
  }
}
