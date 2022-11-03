#![allow(clippy::from_str_radix_10)]
#![allow(clippy::field_reassign_with_default)]

use core::fmt::Write;
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
  eprintln!("{args:?}");

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
  let mut output_enumeration_names = Vec::new();
  let mut output_command_names = Vec::new();
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
      output_enumeration_names.extend_from_slice(&requirement.enumerations);
      output_command_names.extend_from_slice(&requirement.commands);
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
          output_enumeration_names.iter().position(|e| e == enumeration)
        {
          output_enumeration_names.remove(index);
        }
      }
      for command in removal.commands.iter() {
        if let Some(index) =
          output_command_names.iter().position(|e| e == command)
        {
          output_command_names.remove(index);
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
      output_enumeration_names.extend_from_slice(&requirement.enumerations);
      output_command_names.extend_from_slice(&requirement.commands);
    }
  }
  output_enumeration_names.sort();
  output_command_names.sort();

  eprintln!(
    "Found {} enumerations and {} commands.",
    output_enumeration_names.len(),
    output_command_names.len()
  );

  let mut output_enumerations = Vec::new();
  let mut output_commands = Vec::new();
  for output_enumeration_name in output_enumeration_names {
    if let Some(enumeration) = registry.enumerations.iter().find(|e| {
      e.name == output_enumeration_name
        && (match e.api.as_ref() {
          Some(e_api) => e_api.as_str() == args.api,
          None => true,
        })
    }) {
      output_enumerations.push(enumeration.clone());
    } else {
      println!("Registry error: `{}` is part of the requested feature, but not defined in the registry.",
      output_enumeration_name);
      return;
    }
  }
  for output_command_name in output_command_names {
    if let Some(command) =
      registry.commands.iter().find(|c| c.name == output_command_name)
    {
      output_commands.push(command.clone());
    } else {
      println!("Registry error: `{}` is part of the requested feature, but not defined in the registry.",
      output_command_name);
      return;
    }
  }
  output_enumerations.dedup_by_key(|e| e.name.clone());
  output_commands.dedup_by_key(|c| c.name.clone());

  println!("{}", phosphorus::GL_CORE_TYPES);
  output_enumerations.iter().for_each(|e| e.print_rust_const());
  output_commands.iter().for_each(|c| c.print_global_wrapper_fn());
  print_global_loader(&output_commands);
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

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Enumeration {
  pub name: String,
  pub value: String,
  pub group: Option<String>,
  pub alias: Option<String>,
  pub type_: Option<String>,
  pub api: Option<String>,
  pub is_bitmask: bool,
}
impl Enumeration {
  pub fn print_rust_const(&self) {
    if let Some(group) = self.group.as_ref() {
      let s = if group.contains(',') { "s" } else { "" };
      print!("/// * Group{s}: ");
      for (i, entry) in group.split(',').enumerate() {
        if i != 0 {
          print!(", ");
        }
        print!("`{entry}`");
      }
      println!();
    }
    if self.name.chars().any(|c| c.is_ascii_lowercase()) {
      println!("#[allow(non_upper_case_globals)]");
    }
    let name = &self.name;
    let type_ = match self.type_.as_deref() {
      None => {
        if self.value.starts_with('-') {
          "i32"
        } else {
          "u32"
        }
      }
      Some("u") => "u32",
      Some("ull") => "u64",
      Some(other) => panic!("unknown enumeration type: `{other}`"),
    };
    let expr = &self.value;
    println!("pub const {name}: {type_} = {expr};");
  }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Command {
  pub name: String,
  pub return_ty: String,
  pub return_group: Option<String>,
  pub return_class: Option<String>,
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
            Text("void *") => command.return_ty = "*mut void".to_string(),
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
                param.param_ty = "*const void".to_string();
              }
              Text("const void **") => {
                param.param_ty = "*const *mut void".to_string();
              }
              Text("void *") => {
                param.param_ty = "*mut void".to_string();
              }
              Text("void **") => {
                param.param_ty = "*mut *mut void".to_string();
              }
              Text("const void *const*") => {
                param.param_ty = "*const *const void".to_string();
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

  pub(crate) fn print_global_wrapper_fn(&self) {
    let name = &self.name;
    let static_name = format!("{name}_p");
    let ret_ty = &self.return_ty;
    let mut arg_name_and_type_list = String::new();
    let mut fn_ptr_ty = "unsafe extern \"system\" fn(".to_string();
    let mut arg_name_list = String::new();
    for param in self.params.iter() {
      let param_name = match param.name.as_str() {
        "type" => "type_",
        "ref" => "ref_",
        other => other,
      };
      let param_ty = &param.param_ty;
      write!(arg_name_and_type_list, "{param_name}: {param_ty}, ").ok();
      write!(fn_ptr_ty, "{},", param.param_ty).ok();
      write!(arg_name_list, "{param_name}, ").ok();
    }
    write!(fn_ptr_ty, ")->{ret_ty}").ok();
    println!(
      "
      static {static_name}: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
      #[inline]
      pub unsafe fn {name}({arg_name_and_type_list}) -> {ret_ty} {{
        let u: usize = {static_name}.load(core::sync::atomic::Ordering::Acquire);
        assert!(u != 0);
        let _func_p: {fn_ptr_ty} = unsafe {{ core::mem::transmute(u) }};
        _func_p({arg_name_list})
      }}"
    );
  }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
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

fn print_global_loader(commands: &[Command]) {
  let mut command_info = String::new();
  for command in commands.iter() {
    let name = &command.name;
    writeln!(command_info, "(\"{name}\\0\", &{name}_p),").ok();
  }
  println!(
    "
    pub unsafe fn load_gl_functions(load_fn: &dyn Fn(*const u8) -> *const void) -> Result<(), &'static str> {{
      let command_info = [{command_info}];
      for (command_name, command_p) in command_info.iter() {{
        let addr: usize = match load_fn(command_name.as_ptr()) as usize {{
          0 | 1 | 2 | 3 | usize::MAX => {{
            return Err(&command_name[..command_name.len()-1]);
          }}
          other => other
        }};
        command_p.store(addr, core::sync::atomic::Ordering::Release);
      }}
      Ok( () )
    }}
    "
  );
}
