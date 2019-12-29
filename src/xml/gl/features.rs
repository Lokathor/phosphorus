use super::*;

/// An item that the feature requires when it's being targeted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRequirement {
  /// A type alias
  #[allow(missing_docs)]
  Type { name: String },
  /// An enum
  #[allow(missing_docs)]
  Enum { name: String },
  /// A command
  #[allow(missing_docs)]
  Command { name: String },
}

/// An item to mark as removed when you target this feature level.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRemoval {
  /// A type alias
  #[allow(missing_docs)]
  Type { name: String },
  /// An enum
  #[allow(missing_docs)]
  Enum { name: String },
  /// A command
  #[allow(missing_docs)]
  Command { name: String },
}

/// A feature is a particular version of the GL API (eg, OpenGL 3.3, GL ES 3.1,
/// etc)
#[derive(Debug, Default, Clone)]
pub struct Feature {
  /// The API category (Gl, Gles, etc)
  pub api: String,
  /// The unique name for this API ("GL_VERSION_1_0", etc)
  pub name: String,
  /// The API number (eg: 1.0)
  pub number: String,
  /// The requirements that this API has added compared to the last API level.
  pub requirements: HashMap<Option<Profile>, HashSet<FeatureRequirement>>,
  /// The removals that should be performed compared to the last API level.
  pub removals: HashMap<Option<Profile>, HashSet<FeatureRemoval>>,
}

/// Gets all Feature info out of the XML Iterator.
#[must_use]
pub fn pull_feature(
  it: &mut XmlIterator<'_>,
  feature: &mut Feature,
) -> Option<()> {
  loop {
    match it.next()? {
      EndTag { name: "feature" } => return Some(()),
      StartTag { name: "require", attrs } => {
        let mut profile = None;
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "comment" => (),
            "profile" => match v {
              "core" => profile = Some(Profile::Core),
              "compatibility" => profile = Some(Profile::Compatibility),
              "common" => (), /* counts as None for our purposes */
              _ => panic!("unknown `require` attrs: {}", attrs),
            },
            _ => panic!("unknown `require` attrs: {}", attrs),
          }
        }
        let temp =
          feature.requirements.entry(profile).or_insert(HashSet::new());
        'gather_require: loop {
          match it.next()? {
            EndTag { name: "require" } => break 'gather_require,
            EmptyTag { name: "type", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown `type` attrs: {}", attrs),
              };
              temp.insert(FeatureRequirement::Type { name });
            }
            EmptyTag { name: "enum", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown `enum` attrs: {}", attrs),
              };
              temp.insert(FeatureRequirement::Enum { name });
            }
            EmptyTag { name: "command", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown `command` attrs: {}", attrs),
              };
              temp.insert(FeatureRequirement::Command { name });
            }
            other => panic!("unhandled feature/require> {:?}", other),
          }
        }
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
        let mut profile = None;
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "comment" => (),
            "profile" => match v {
              "core" => profile = Some(Profile::Core),
              "compatibility" => profile = Some(Profile::Compatibility),
              _ => panic!("unknown `require` attrs: {}", attrs),
            },
            _ => panic!("unknown `remove` attrs: {}", attrs),
          }
        }
        let temp = feature.removals.entry(profile).or_insert(HashSet::new());
        'gather_remove: loop {
          match it.next()? {
            EndTag { name: "remove" } => break 'gather_remove,
            EmptyTag { name: "type", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown attrs> {:?}", attrs),
              };
              temp.insert(FeatureRemoval::Type { name });
            }
            EmptyTag { name: "enum", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown attrs> {:?}", attrs),
              };
              temp.insert(FeatureRemoval::Enum { name });
            }
            EmptyTag { name: "command", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", v) => v.to_owned(),
                _ => panic!("unknown attrs> {:?}", attrs),
              };
              temp.insert(FeatureRemoval::Command { name });
            }
            other => panic!("unhandled feature/require> {:?}", other),
          }
        }
      }
      other => panic!("unhandled feature> {:?}", other),
    }
  }
}
