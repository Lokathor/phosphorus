use super::*;

/// All the Groups.
#[derive(Debug, Default, Clone)]
pub struct Groups(pub(crate) HashMap<String, HashSet<String>>);

/// Allows properly displaying an enum key/value pair.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct GroupDisplay<'a> {
  pub name: &'a String,
  pub entries: &'a HashSet<String>,
}
impl core::fmt::Display for GroupDisplay<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "#[doc=\"{}\\n", self.name)?;
    let mut enums_v: Vec<&String> = self.entries.iter().collect();
    enums_v.sort();
    for enu in enums_v {
      write!(f, "* `{}`\\n", enu)?;
    }
    write!(f, "\"]")?;
    write!(f, " pub type {} = GLenum;", self.name)
  }
}

/// Extracts all the Group definitions from the iterator.
#[must_use]
pub fn pull_groups(
  it: &mut XmlIterator<'_>,
  groups: &mut Groups,
) -> Option<()> {
  loop {
    match it.next()? {
      EndTag { name: "groups" } => return Some(()),
      StartTag { name: "group", attrs } => {
        let name = match AttributeIterator::new(attrs).next()? {
          ("name", name) => name.to_owned(),
          other => panic!("unexpected> {:?}", other),
        };
        let mut enums = HashSet::new();
        'pull_group_enums: loop {
          match it.next()? {
            EndTag { name: "group" } => break 'pull_group_enums,
            EmptyTag { name: "enum", attrs } => {
              let name = match AttributeIterator::new(attrs).next()? {
                ("name", name) => name.to_owned(),
                other => panic!("unexpected> {:?}", other),
              };
              enums.insert(name);
            }
            other => panic!("unexpected> {:?}", other),
          }
        }
        groups.0.insert(name, enums);
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
        groups.0.insert(name, HashSet::new());
      }
      other => panic!("unexpected> {:?}", other),
    }
  }
}
