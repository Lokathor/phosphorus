use super::*;

#[derive(Debug, Clone, Default)]
pub struct Groups {
  pub groups: Vec<Group>,
}
impl Groups {
  pub fn make_groups_module_string(&self) -> String {
    let mut out = String::new();
    writeln_str!(out, "pub use groups::*;");
    writeln_str!(out, "pub mod groups {{");
    writeln_str!(out, "  use super::*;");
    for group in self.groups.iter() {
      for line in group.make_string().lines() {
        writeln_str!(out, "  {}", line);
      }
    }
    writeln_str!(out, "}}");
    out
  }
}

pub(crate) fn parse_groups<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
) -> Groups {
  let mut groups = Groups::default();
  loop {
    match iter.next().unwrap() {
      EndTag { name: "groups" } => return groups,
      StartTag { name: "group", attrs } => {
        groups.groups.push(parse_group(iter, attrs));
      }
      EmptyTag { name: "group", attrs } => drop(attrs),
      unknown => panic!("parse_groups:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Group {
  pub attrs: HashMap<String, String>,
  pub enumerants: Vec<String>,
}
impl Group {
  pub fn make_string(&self) -> String {
    let mut out = String::new();
    out.push_str("/// Sub-group of [`GLenum`] values.\n");
    out.push_str("/// \n");
    out.push_str("/// Group Members: ");
    for (i, enumerant_name) in self.enumerants.iter().enumerate() {
      if i != 0 {
        out.push_str(", ");
      }
      out.push('`');
      out.push_str(enumerant_name);
      out.push('`');
    }
    out.push('\n');
    out.push_str("pub type ");
    out.push_str(&self.attrs["name"]);
    out.push_str(" = GLenum;");
    out
  }
}

pub(crate) fn parse_group<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> Group {
  let mut group = Group::default();
  group.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "group" } => return group,
      EmptyTag { name: "enum", attrs } => {
        let hm = hashmap_from_attrs(attrs);
        group.enumerants.push(hm["name"].to_string());
      }
      unknown => panic!("parse_group:{:?}", unknown),
    }
  }
}
