use super::*;

/// An argument to a command.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Param {
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

/// A single command's info.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Command {
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

/// All the commands.
#[derive(Debug, Default, Clone)]
pub struct Commands(pub(crate) Vec<Command>);

/// Grab all the commands out of the XML.
#[must_use]
pub fn pull_commands(
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
