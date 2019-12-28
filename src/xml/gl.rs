//! Types and functions to interact with `gl.xml`

use super::{XmlElement::*, *};

/*

TODO:

A way to write out all the GL Enums.

A way to write out the commands.

A way to query and use the API levels available.

A way to query and use the extensions available.

Remove all of the `..` ignoring of stuff in matches. We should always have
expected the input style or we should reject the file and refuse to parse it
(because it's added some totally new attribute which might be important but
that we're not handling)

Use an error type instead of Option<()> all over?

*/

mod enums;
pub use enums::*;

mod groups;
pub use groups::*;

mod types;
pub use types::*;

mod commands;
pub use commands::*;

mod features;
pub use features::*;

mod extensions;
pub use extensions::*;

/// The output of parsing `gl.xml`.
///
/// A "registry" (the file's term) is a collection of types, enums, functions,
/// etc that describe the various GL APIs.
#[derive(Debug, Default, Clone)]
pub struct GlRegistry {
  types: Types,
  groups: Groups,
  enums: Enums,
  commands: Commands,
  feature_list: Vec<Feature>,
  extensions: Extensions,
}
impl GlRegistry {
  /// Inspect the types of the Registry
  pub fn types(&self) -> &[Type] {
    &self.types.0
  }
  /// Inspect the groups of the Registry
  pub fn groups(&self) -> &HashMap<String, HashSet<String>> {
    &self.groups.0
  }
  /// Inspect the enums of the Registry
  pub fn enums(&self) -> &HashMap<EnumKey, EnumValue> {
    &self.enums.0
  }
  /// All possible commands from all Features and Extensions
  pub fn commands(&self) -> &[Command] {
    &self.commands.0
  }
  /// All possible Features
  pub fn features(&self) -> &[Feature] {
    &self.feature_list
  }
  /// All possible Extensions
  pub fn extensions(&self) -> &[Extension] {
    &self.extensions.0
  }

  /// Make a new `GlRegistry` from the `gl.xml` file contents.
  pub fn from_gl_xml(whole_file: &str) -> Option<Self> {
    let body = drop_declaration(whole_file);
    let mut registry = None;
    let mut it = XmlIterator::new(body);
    while let Some(elem) = it.next() {
      match elem {
        StartTag { name: "registry", attrs: "" } => {
          registry = pull_registry(&mut it);
        }
        other => panic!("unexpected: {:?}", other),
      }
    }
    registry
  }
}

#[must_use]
fn pull_registry(it: &mut XmlIterator<'_>) -> Option<GlRegistry> {
  let mut registry = GlRegistry::default();
  loop {
    match it.next()? {
      EndTag { name: "registry" } => return Some(registry),
      StartTag { name: "comment", attrs: "" } => 'grab_comment: loop {
        match it.next()? {
          EndTag { name: "comment" } => break 'grab_comment,
          Text(_) => continue,
          other => panic!("unexpected>{:?}", other),
        }
      },
      StartTag { name: "types", attrs: "" } => {
        pull_types(it, &mut registry.types)?;
      }
      StartTag { name: "groups", attrs: "" } => {
        pull_groups(it, &mut registry.groups)?;
      }
      StartTag { name: "enums", attrs } => {
        let mut is_bitmask = false;
        let mut group = None;
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "namespace" => (),
            "group" => group = Some(v.to_owned()),
            "type" => is_bitmask = v == "bitmask",
            "comment" => (),
            "vendor" => (),
            "start" => (),
            "end" => (),
            _ => panic!("unknown `enums` attributes: {}", attrs),
          }
        }
        let group: Option<&mut HashSet<String>> = if let Some(name) = group {
          Some(registry.groups.0.entry(name).or_insert(HashSet::new()))
        } else {
          None
        };
        pull_enums(it, &mut registry.enums, is_bitmask, group)?;
      }
      EmptyTag { name: "enums", attrs } => {
        for (k, _v) in AttributeIterator::new(attrs) {
          // LATER: do we want to handle any of these?
          match k {
            "namespace" => (),
            "group" => (),
            "type" => (),
            "comment" => (),
            "vendor" => (),
            "start" => (),
            "end" => (),
            _ => panic!("unknown `enums` attributes: {}", attrs),
          }
        }
      }
      StartTag { name: "commands", attrs } => {
        for (k, _v) in AttributeIterator::new(attrs) {
          match k {
            "namespace" => (),
            _ => panic!("unknown `commands` attributes: {}", attrs),
          }
        }
        pull_commands(it, &mut registry.commands)?;
      }
      StartTag { name: "feature", attrs } => {
        registry.feature_list.push(Feature::default());
        let feat = registry.feature_list.last_mut().unwrap();
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "api" => feat.api = v.to_owned(),
            "name" => feat.name = v.to_owned(),
            "number" => feat.number = v.to_owned(),
            _ => panic!("unexpected feature attributes: {}", attrs),
          }
        }
        pull_feature(it, feat)?;
      }
      StartTag { name: "extensions", attrs } => {
        for (k, _v) in AttributeIterator::new(attrs) {
          match k {
            _ => panic!("unknown `extensions` attrs: {}", attrs),
          }
        }
        pull_extensions(it, &mut registry.extensions)?;
      }
      elem => panic!("unhandled registry element: {:?}", elem),
    }
  }
}
