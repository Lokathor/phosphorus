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
  println!("parsed args: {args:?}");

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

  // These are *just the names* of all the things we want to output. Once we've
  // got the full list of things to output, we'll have to look up the rest of
  // the info over in the registry.
  let mut output_enumerations = Vec::new();
  let mut output_commands = Vec::new();
  let mut feature_name_and_number_match = false;
  for feature in api_entries.iter() {
    for requirement in feature.requirements.iter() {
      if let Some(api) = requirement.api.as_ref() {
        if api.as_str() != args.api.as_str() {
          continue;
        }
      }
      if let Some(profile) = requirement.profile.as_ref() {
        if profile.as_str() != "core" {
          continue;
        }
      }
      output_enumerations.extend_from_slice(&requirement.enumerations);
      output_commands.extend_from_slice(&requirement.commands);
    }

    for removal in feature.removals.iter() {
      if let Some(api) = removal.api.as_ref() {
        if api.as_str() != args.api.as_str() {
          continue;
        }
      }
      if let Some(profile) = removal.profile.as_ref() {
        if profile.as_str() != "core" {
          continue;
        }
      }
      for enumeration in removal.enumerations.iter() {
        if let Some(index) =
          output_enumerations.iter().position(|e| e == enumeration)
        {
          output_enumerations.remove(index);
        }
      }
      for command in removal.commands.iter() {
        if let Some(index) = output_commands.iter().position(|e| e == command) {
          output_commands.remove(index);
        }
      }
    }

    if feature.name == args.name && feature.number == args.number {
      feature_name_and_number_match = true;
      break;
    }
  }
  if !feature_name_and_number_match {
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
    for requirement in extension.requirements.iter() {
      if let Some(api) = requirement.api.as_ref() {
        if api.as_str() != args.api.as_str() {
          continue;
        }
      }
      if let Some(profile) = requirement.profile.as_ref() {
        if profile.as_str() != "core" {
          continue;
        }
      }
      output_enumerations.extend_from_slice(&requirement.enumerations);
      output_commands.extend_from_slice(&requirement.commands);
    }
  }
  output_enumerations.sort();
  output_commands.sort();

  println!(
    "Found {} enumerations and {} commands.",
    output_enumerations.len(),
    output_commands.len()
  );

  println!("{:?}", output_enumerations);
  println!("{:?}", output_commands);
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
pub struct Requirement {
  api: Option<String>,
  profile: Option<String>,
  enumerations: Vec<String>,
  commands: Vec<String>,
}
impl Requirement {
  pub fn new<'a>(
    elem_iter: &mut impl Iterator<Item = XmlElement<'a>>, attrs: &str,
  ) -> Self {
    let mut requirement = Requirement::default();
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
pub struct Removal {
  api: Option<String>,
  profile: Option<String>,
  enumerations: Vec<String>,
  commands: Vec<String>,
}
impl Removal {
  pub fn new<'a>(
    elem_iter: &mut impl Iterator<Item = XmlElement<'a>>, attrs: &str,
  ) -> Self {
    let mut removal = Removal::default();
    for attr in TagAttributeIterator::new(attrs) {
      match attr.key {
        "comment" => (),
        "api" => removal.api = Some(attr.value.to_string()),
        "profile" => removal.profile = Some(attr.value.to_string()),
        other => panic!("{other:?}"),
      }
    }
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "remove" } => return removal,
        EmptyTag { name: "enum", attrs } => {
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "name" => removal.enumerations.push(attr.value.to_string()),
              "comment" => (),
              other => panic!("{other:?}"),
            }
          }
        }
        EmptyTag { name: "command", attrs } => {
          for attr in TagAttributeIterator::new(attrs) {
            match attr.key {
              "name" => removal.commands.push(attr.value.to_string()),
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
pub struct Feature {
  pub api: String,
  pub name: String,
  pub number: String,
  pub requirements: Vec<Requirement>,
  pub removals: Vec<Removal>,
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
          feature.requirements.push(Requirement::new(elem_iter, attrs));
        }
        StartTag { name: "remove", attrs } => {
          feature.removals.push(Removal::new(elem_iter, attrs));
        }
        other => panic!("{other:?}"),
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub name: String,
  pub supported: String,
  pub requirements: Vec<Requirement>,
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
        StartTag { name: "require", attrs } => {
          extension.requirements.push(Requirement::new(elem_iter, attrs))
        }
        other => panic!("{other:?}"),
      }
    }
  }
}
