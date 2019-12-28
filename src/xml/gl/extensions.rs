use super::*;

/// A block of requirements for an extension.
#[derive(Debug, Default, Clone)]
pub struct ExtensionRequirement {
  /// The API that this block applies to (None = all)
  pub api: Option<String>,
  /// The profile that this block applies to (None = all)
  pub profile: Option<String>,
  /// The items required.
  pub items: Vec<FeatureRequirement>,
}

/// An extension of the baseline for a given API.
#[derive(Debug, Default, Clone)]
pub struct Extension {
  /// Extension name
  pub name: String,
  /// What APIs can support this extension.
  pub supported: String,
  /// The requirements blocks of this extension.
  ///
  /// Different blocks can have different API and Profile situations, such as
  /// requiring some items all the time and then _more_ items in a specific API
  /// or profile.
  pub requirements: Vec<ExtensionRequirement>,
}

/// The list of all extensions.
#[derive(Debug, Default, Clone)]
pub struct Extensions(pub(crate) Vec<Extension>);

/// Grab all extensions out of the XML iterator.
#[must_use]
pub fn pull_extensions(
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
        let mut requirements: Vec<ExtensionRequirement> = vec![];
        'gather_extension: loop {
          match it.next()? {
            EndTag { name: "extension" } => break 'gather_extension,
            StartTag { name: "require", attrs } => {
              let mut api = None;
              let mut profile = None;
              let mut items = vec![];
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
                    items.push(FeatureRequirement::Type { name });
                  }
                  EmptyTag { name: "enum", attrs } => {
                    let name = match AttributeIterator::new(attrs).next() {
                      Some(("name", v)) => v.to_owned(),
                      _ => panic!("unexpected attrs: {}", attrs),
                    };
                    items.push(FeatureRequirement::Enum { name });
                  }
                  EmptyTag { name: "command", attrs } => {
                    let name = match AttributeIterator::new(attrs).next() {
                      Some(("name", v)) => v.to_owned(),
                      _ => panic!("unexpected attrs: {}", attrs),
                    };
                    items.push(FeatureRequirement::Command { name });
                  }
                  other => panic!(
                    "unhandled in extensions/extension/require: {:?}",
                    other
                  ),
                }
              }
              requirements.push(ExtensionRequirement { api, profile, items });
            }
            other => panic!("unhandled in extensions/extension: {:?}", other),
          }
        }
        extensions.0.push(Extension { name, supported, requirements });
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
        extensions.0.push(Extension { name, supported, requirements });
      }
      other => panic!("unhandled in extensions: {:?}", other),
    }
  }
}
