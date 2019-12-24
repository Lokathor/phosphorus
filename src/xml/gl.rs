//! Types and functions to interact with `gl.xml`

use super::{XmlElement::*, *};

/*

TODO:

Use an error type instead of Option<()> all over?

A way to write the GL types out as Rust type aliases.

A way to write out all the GL Enums.

A way to write out the commands.

A way to query the API levels available.

A way to query the extensions available.

Remove all of the `..` ignoring of stuff in matches. We should always have
expected the input style or we should reject the file and refuse to parse it
(because it's added some totally new attribute which might be important but
that we're not handling)

*/

mod types;
pub use types::*;

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
        pull_enums(it, &mut registry.enums)?;
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

#[derive(Debug, Default, Clone)]
struct Group {
  name: String,
  enums: Vec<String>,
}
#[derive(Debug, Default, Clone)]
struct Groups(Vec<Group>);
#[must_use]
fn pull_groups(it: &mut XmlIterator<'_>, groups: &mut Groups) -> Option<()> {
  loop {
    match it.next()? {
      EndTag { name: "groups" } => return Some(()),
      StartTag { name: "group", attrs } => {
        let name = match AttributeIterator::new(attrs).next()? {
          ("name", name) => name.to_owned(),
          other => panic!("unexpected> {:?}", other),
        };
        let mut enums = vec![];
        'pull_group_enums: loop {
          match it.next()? {
            EndTag { name: "group" } => break 'pull_group_enums,
            EmptyTag { name: "enum", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", name) => name.to_owned(),
                other => panic!("unexpected> {:?}", other),
              };
              enums.push(name);
            }
            other => panic!("unexpected> {:?}", other),
          }
        }
        groups.0.push(Group { name, enums });
      }
      EmptyTag { name: "group", attrs } => {
        // Note: This happens in all of two places, `DataType` and
        // `FfdMaskSGIX`, and they both have a comment saying what the "real"
        // group is supposed to be. So, we can probably special case this I
        // guess. However, we don't really use the groups that much right now
        // anyway so it's not an immediate priority. For now we just record that
        // we saw them and call it good.
        let name = match AttributeIterator::new(attrs).next()? {
          ("name", name) => name.to_owned(),
          other => panic!("unexpected> {:?}", other),
        };
        groups.0.push(Group { name, enums: vec![] });
      }
      other => panic!("unexpected> {:?}", other),
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct EnumKey {
  /// Enum name
  name: String,
  /// Enum availability (`None` == always)
  api: Option<String>,
}
#[derive(Debug, Default, Clone)]
struct Enums(HashMap<EnumKey, u64>);
#[must_use]
fn pull_enums(it: &mut XmlIterator<'_>, enums: &mut Enums) -> Option<()> {
  loop {
    match it.next()? {
      EndTag { name: "enums" } => return Some(()),
      EmptyTag { name: "enum", attrs } => {
        let mut name = None;
        let mut value = None;
        let mut alias = None;
        let mut api = None;
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "name" => name = Some(v),
            "value" => value = Some(v),
            "alias" => alias = Some(v),
            "comment" => (),
            "type" => (),
            "api" => api = Some(v.to_owned()),
            _ => panic!("unexpected key> {}; {}", k, attrs),
          }
        }
        let name = name.unwrap().to_owned();
        let value = value.unwrap();
        let value: u64 = if value.contains('x') || value.contains('X') {
          // hex numbers we have to slice off the `Ox` ourselves
          u64::from_str_radix(&value[2..], 16).unwrap()
        } else if value.contains('-') {
          // Some values are given as a negative value, so we parse as `i32`
          // then cast to `u32`, which will preserve the bit pattern.
          i64::from_str_radix(&value, 10).unwrap() as u64
        } else {
          u64::from_str_radix(&value, 10).unwrap()
        };
        let key = EnumKey { name, api: api.clone() };
        if enums.0.contains_key(&key) {
          let old = *enums.0.get(&key).unwrap();
          let new = value;
          if old != new {
            panic!(
              "key overwrite: key: {:?}, old: {:?}, new: {:?}",
              key, old, new
            );
          }
        } else {
          enums.0.insert(key, value);
        }
        if let Some(alias) = alias {
          let name = alias.to_owned();
          let key = EnumKey { name, api: api.clone() };
          if enums.0.contains_key(&key) {
            let old = *enums.0.get(&key).unwrap();
            let new = value;
            if old != new {
              panic!(
                "key overwrite: key: {:?}, old: {:?}, new: {:?}",
                key, old, new
              );
            }
          } else {
            enums.0.insert(key, value);
          }
        }
      }
      EmptyTag { name: "unused", attrs } => {
        // TODO: We should check if the `unused` tag is somehow used despite the
        // name, because programming is that bad at naming sometimes.
        let _ = attrs;
      }
      other => panic!("unexpected> {:?}", other),
    }
  }
}

#[derive(Debug, Default, Clone)]
struct Param {
  /// Param type.
  ptype: String,
  /// Param name.
  name: String,
  /// Param group (for enums).
  ///
  /// If there's a group declared for a non-enum param there's probably
  /// something weird going on.
  group: Option<String>,
  /// Param length (for pointers).
  ///
  /// If a length is declared on a non-pointer param there's probably something
  /// weird going on.
  len: Option<String>,
}
#[derive(Debug, Default, Clone)]
struct Command {
  /// The function's C prototype
  proto: String,
  /// The group that the prototype return value belongs to, if any.
  proto_group: Option<String>,
  /// The function's arguments
  params: Vec<Param>,
  /// You can find this command under some alternate name.
  ///
  /// Generally the "main" version won't have an alias entry, and then one or
  /// more versions (that were extensions before being stabilized) will all
  /// mark themselves as being an alias of the main version.
  alias: Option<String>,
  /// This command has an equivalent "vector" version that just takes a single
  /// pointer instead of however many separate params.
  vecequiv: Option<String>,
}
#[derive(Debug, Default, Clone)]
struct Commands(Vec<Command>);
#[must_use]
fn pull_commands(
  it: &mut XmlIterator<'_>,
  commands: &mut Commands,
) -> Option<()> {
  loop {
    match it.next()? {
      EndTag { name: "commands" } => return Some(()),
      StartTag { name: "command", attrs } => {
        for (k, _v) in AttributeIterator::new(attrs) {
          match k {
            "comment" => (),
            _ => panic!("unknown `command` attrs: {}", attrs),
          }
        }
        let mut command = Command::default();
        let mut seen_proto = false;
        'gather_command: loop {
          match it.next()? {
            EndTag { name: "command" } => break 'gather_command,
            StartTag { name: "proto", attrs } if !seen_proto => {
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "group" => command.proto_group = Some(v.to_owned()),
                  _ => panic!("unknown `proto` attrs: {}", attrs),
                }
              }
              'gather_proto: loop {
                match it.next()? {
                  EndTag { name: "proto" } => break 'gather_proto,
                  Text(text) => command.proto.push_str(text),
                  StartTag { name: "name", .. } => continue,
                  EndTag { name: "name" } => continue,
                  StartTag { name: "ptype", .. } => continue,
                  EndTag { name: "ptype" } => continue,
                  other => panic!("unexpected> {:?}", other),
                }
              }
              seen_proto = true;
            }
            StartTag { name: "param", attrs } => {
              let mut group = None;
              let mut len = None;
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "group" => group = Some(v.to_owned()),
                  "len" => len = Some(v.to_owned()),
                  other => panic!("unexpected> {:?}", other),
                }
              }
              // ptype
              let mut ptype = String::new();
              'gather_ptype: loop {
                match it.next()? {
                  StartTag { name: "name", .. } => break 'gather_ptype,
                  StartTag { name: "ptype", .. } => continue,
                  EndTag { name: "ptype" } => continue,
                  Text(text) => ptype.push_str(text),
                  other => panic!("unexpected> {:?}", other),
                }
              }
              // name
              let name = match it.next()? {
                Text(text) => text.to_owned(),
                other => panic!("unexpected> {:?}", other),
              };
              match it.next()? {
                EndTag { name: "name" } => (),
                other => panic!("unexpected> {:?}", other),
              }
              match it.next()? {
                EndTag { name: "param" } => (),
                Text("[2]") => {
                  // we special case this, it's used in exactly one place ever,
                  // the `baseAndCount` param of `glPathGlyphIndexRangeNV`.
                  ptype.push_str("*");
                  assert_eq!(len, None);
                  len = Some("2".to_string());
                  match it.next()? {
                    EndTag { name: "param" } => (),
                    other => panic!("unexpected> {:?}", other),
                  }
                }
                other => panic!("unexpected> {:?}", other),
              }
              command.params.push(Param { ptype, name, group, len });
            }
            EmptyTag { name: "glx", attrs } => {
              // TODO: Not sure what the `glx` stuff is all about, we should
              // probably look into the meaning and maybe do something with the
              // information.
              let _ = attrs;
            }
            EmptyTag { name: "alias", attrs } => {
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "name" => {
                    assert_eq!(command.alias, None);
                    command.alias = Some(v.to_owned());
                  }
                  other => panic!("unexpected> {:?}", other),
                }
              }
            }
            EmptyTag { name: "vecequiv", attrs } => {
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "name" => command.vecequiv = Some(v.to_owned()),
                  other => panic!("unexpected> {:?}", other),
                }
              }
            }
            other => panic!("unexpected> {:?}", other),
          }
        }
        commands.0.push(command);
      }
      other => panic!("unexpected> {:?}", other),
    }
  }
}

#[derive(Debug, Clone)]
enum FeatureRequirement {
  Type { name: String },
  Enum { name: String },
  Command { name: String },
}
#[derive(Debug, Clone)]
enum FeatureRemoval {
  Type { name: String },
  Enum { name: String },
  Command { name: String },
}
#[derive(Debug, Default, Clone)]
struct Feature {
  api: String,
  name: String,
  number: String,
  requirements: HashMap<String, Vec<FeatureRequirement>>,
  removals: HashMap<String, Vec<FeatureRemoval>>,
}
#[must_use]
fn pull_feature(it: &mut XmlIterator<'_>, feature: &mut Feature) -> Option<()> {
  use hashbrown::hash_map::Entry;
  loop {
    match it.next()? {
      EndTag { name: "feature" } => return Some(()),
      StartTag { name: "require", attrs } => {
        let mut profile = String::new();
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "comment" => (),
            "profile" => profile = v.to_owned(),
            _ => panic!("unknown `require` attrs: {}", attrs),
          }
        }
        let mut temp = vec![];
        'gather_require: loop {
          match it.next()? {
            EndTag { name: "require" } => break 'gather_require,
            EmptyTag { name: "type", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown `type` attrs: {}", attrs),
              };
              temp.push(FeatureRequirement::Type { name });
            }
            EmptyTag { name: "enum", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown `enum` attrs: {}", attrs),
              };
              temp.push(FeatureRequirement::Enum { name });
            }
            EmptyTag { name: "command", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown `command` attrs: {}", attrs),
              };
              temp.push(FeatureRequirement::Command { name });
            }
            other => panic!("unhandled feature/require> {:?}", other),
          }
        }
        match feature.requirements.entry(profile) {
          Entry::Occupied(mut oe) => oe.get_mut().append(&mut temp),
          Entry::Vacant(ve) => {
            ve.insert(temp);
          }
        };
      }
      EmptyTag { name: "require", attrs } => {
        for (k, _v) in AttributeIterator::new(attrs) {
          match k {
            "comment" => (),
            _ => panic!("unknown `require` attrs: {}", attrs),
          }
        }
      }
      StartTag { name: "remove", attrs } => {
        let mut profile = String::new();
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "comment" => (),
            "profile" => profile = v.to_owned(),
            _ => panic!("unknown `remove` attrs: {}", attrs),
          }
        }
        let mut temp = vec![];
        'gather_remove: loop {
          match it.next()? {
            EndTag { name: "remove" } => break 'gather_remove,
            EmptyTag { name: "type", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown attrs> {:?}", attrs),
              };
              temp.push(FeatureRemoval::Type { name });
            }
            EmptyTag { name: "enum", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown attrs> {:?}", attrs),
              };
              temp.push(FeatureRemoval::Enum { name });
            }
            EmptyTag { name: "command", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown attrs> {:?}", attrs),
              };
              temp.push(FeatureRemoval::Command { name });
            }
            other => panic!("unhandled feature/require> {:?}", other),
          }
        }
        match feature.removals.entry(profile) {
          Entry::Occupied(mut oe) => oe.get_mut().append(&mut temp),
          Entry::Vacant(ve) => {
            ve.insert(temp);
          }
        };
      }
      other => panic!("unhandled feature> {:?}", other),
    }
  }
}

#[derive(Debug, Default, Clone)]
struct Extension {
  name: String,
  supported: String,
  requirements: Vec<FeatureRequirement>,
  api: Option<String>,
  profile: Option<String>,
}
#[derive(Debug, Default, Clone)]
struct Extensions(Vec<Extension>);
#[must_use]
fn pull_extensions(
  it: &mut XmlIterator<'_>,
  extensions: &mut Extensions,
) -> Option<()> {
  loop {
    match it.next()? {
      EndTag { name: "extensions" } => return Some(()),
      StartTag { name: "extension", attrs } => {
        let mut name = None;
        let mut supported = None;
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "name" => name = Some(v.to_owned()),
            "supported" => supported = Some(v.to_owned()),
            "comment" => (),
            _ => panic!("unknown extension attr: {}", attrs),
          }
        }
        let name = name.unwrap();
        let supported = supported.unwrap();
        let mut requirements = vec![];
        let mut api = None;
        let mut profile = None;
        'gather_extension: loop {
          match it.next()? {
            EndTag { name: "extension" } => break 'gather_extension,
            StartTag { name: "require", attrs } => {
              for (k, v) in AttributeIterator::new(attrs) {
                match k {
                  "comment" => (),
                  "api" => api = Some(v.to_owned()),
                  "profile" => profile = Some(v.to_owned()),
                  _ => panic!("unknown `require` attrs: {}", attrs),
                }
              }
              'gather_require: loop {
                match it.next()? {
                  EndTag { name: "require" } => break 'gather_require,
                  EmptyTag { name: "type", attrs } => {
                    let name = match AttributeIterator::new(attrs).next() {
                      Some(("name", v)) => v.to_owned(),
                      _ => panic!("unexpected attrs: {}", attrs),
                    };
                    requirements.push(FeatureRequirement::Type { name });
                  }
                  EmptyTag { name: "enum", attrs } => {
                    let name = match AttributeIterator::new(attrs).next() {
                      Some(("name", v)) => v.to_owned(),
                      _ => panic!("unexpected attrs: {}", attrs),
                    };
                    requirements.push(FeatureRequirement::Enum { name });
                  }
                  EmptyTag { name: "command", attrs } => {
                    let name = match AttributeIterator::new(attrs).next() {
                      Some(("name", v)) => v.to_owned(),
                      _ => panic!("unexpected attrs: {}", attrs),
                    };
                    requirements.push(FeatureRequirement::Command { name });
                  }
                  other => panic!(
                    "unhandled in extensions/extension/require: {:?}",
                    other
                  ),
                }
              }
            }
            other => panic!("unhandled in extensions/extension: {:?}", other),
          }
        }
        extensions.0.push(Extension {
          name,
          supported,
          requirements,
          api,
          profile,
        });
      }
      EmptyTag { name: "extension", attrs } => {
        let mut name = None;
        let mut supported = None;
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "name" => name = Some(v.to_owned()),
            "supported" => supported = Some(v.to_owned()),
            _ => panic!("unknown `extension` attr: {}", attrs),
          }
        }
        let name = name.unwrap();
        let supported = supported.unwrap();
        let requirements = vec![];
        let api = None;
        let profile = None;
        extensions.0.push(Extension {
          name,
          supported,
          requirements,
          api,
          profile,
        });
      }
      other => panic!("unhandled in extensions: {:?}", other),
    }
  }
}
