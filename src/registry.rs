use super::*;

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub types: Types,
  pub groups: Groups,
  pub enumerants: Vec<Enumerants>,
  pub features: Vec<Feature>,
  pub commands: Commands,
  pub extensions: Extensions,
}

impl Registry {
  /// Parses `gl.xml` content into a Registry.
  /// ## Panics
  /// * If the data can't be parsed this can panic.
  pub fn from_xml(file_content: &str) -> Self {
    let iter = &mut ElementIterator::new(&file_content)
      .filter_map(skip_comments)
      .filter_map(skip_empty_text_elements);
    match iter.next().unwrap() {
      StartTag { name: "registry", attrs: "" } => parse_registry(iter),
      unknown => panic!("{:?}", unknown),
    }
  }
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
        registry.types = parse_types(iter)
      }
      StartTag { name: "groups", attrs: "" } => {
        registry.groups = parse_groups(iter)
      }
      StartTag { name: "enums", attrs } => {
        registry.enumerants.push(parse_enums(iter, attrs));
      }
      EmptyTag { name: "enums", attrs } => drop(attrs),
      StartTag { name: "commands", attrs } => {
        registry.commands = parse_commands(iter, attrs)
      }
      StartTag { name: "feature", attrs } => {
        registry.features.push(parse_feature(iter, attrs));
      }
      StartTag { name: "extensions", attrs: "" } => {
        registry.extensions = parse_extensions(iter);
      }
      unknown => panic!("parse_registry:{:?}", unknown),
    }
  }
}

fn parse_comment<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  loop {
    match iter.next().unwrap() {
      EndTag { name: "comment" } => return,
      Text(t) => drop(t),
      unknown => panic!("parse_comment:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Enumerants {
  pub attrs: HashMap<String, String>,
  pub enumerants: Vec<Enumerant>,
}

#[derive(Debug, Clone, Default)]
pub struct Enumerant {
  pub attrs: HashMap<String, String>,
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
      EmptyTag { name: "unused", attrs } => drop(attrs),
      unknown => panic!("parse_enums:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Commands {
  pub attrs: HashMap<String, String>,
  pub commands: Vec<Command>,
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
      unknown => panic!("parse_commands:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Command {
  pub attrs: HashMap<String, String>,
  pub prototype: Prototype,
  pub params: Vec<Param>,
  pub glx_attrs: HashMap<String, String>,
  pub alias: String,
  pub vec_equivalent: String,
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
      unknown => panic!("parse_command:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Prototype {
  pub attrs: HashMap<String, String>,
  pub text: String,
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
      unknown => panic!("parse_proto:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Param {
  pub attrs: HashMap<String, String>,
  pub text: String,
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
      unknown => panic!("parse_param:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Feature {
  pub attrs: HashMap<String, String>,
  pub requirements: Vec<Requirements>,
  pub removals: Vec<Removals>,
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
      EmptyTag { name: "require", attrs } => drop(attrs),
      StartTag { name: "remove", attrs } => {
        feature.removals.push(parse_remove(iter, attrs));
      }
      unknown => panic!("parse_feature:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Requirements {
  pub attrs: HashMap<String, String>,
  pub types: Vec<String>,
  pub enumerants: Vec<String>,
  pub commands: Vec<String>,
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
      unknown => panic!("parse_require:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Removals {
  pub attrs: HashMap<String, String>,
  pub types: Vec<String>,
  pub enumerants: Vec<String>,
  pub commands: Vec<String>,
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
      unknown => panic!("parse_remove:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extensions {
  pub extensions: Vec<Extension>,
}

fn parse_extensions<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
) -> Extensions {
  let mut extensions = Extensions::default();
  loop {
    match iter.next().unwrap() {
      EndTag { name: "extensions" } => return extensions,
      StartTag { name: "extension", attrs } => {
        extensions.extensions.push(parse_extension(iter, attrs));
      }
      EmptyTag { name: "extension", attrs } => {
        extensions.extensions.push(Extension {
          attrs: hashmap_from_attrs(attrs),
          requirements: vec![],
        });
      }
      unknown => panic!("parse_extensions:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub attrs: HashMap<String, String>,
  pub requirements: Vec<Requirements>,
}

fn parse_extension<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Extension {
  let mut extension = Extension::default();
  extension.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "extension" } => return extension,
      StartTag { name: "require", attrs } => {
        extension.requirements.push(parse_require(iter, attrs));
      }
      unknown => panic!("parse_extension:{:?}", unknown),
    }
  }
}
