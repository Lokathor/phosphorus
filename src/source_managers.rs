use super::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandTypeEntry {
  pub name: String,
  pub comments: String,
  pub type_declaration: String,
}

pub fn get_command_type_entries() -> Vec<CommandTypeEntry> {
  let mut entries = Vec::new();
  let mut current_entry = CommandTypeEntry::default();
  for line in include_str!("gl_command_types.rs").lines().skip(1) {
    if line.starts_with("///") || line.starts_with('#') {
      current_entry.comments.push_str(line);
      current_entry.comments.push('\n');
    } else if line.starts_with("//") {
      continue;
    } else if line.is_empty() {
      if !current_entry.name.is_empty() {
        entries.push(current_entry);
        current_entry = CommandTypeEntry::default();
      } else {
        continue;
      }
    } else {
      let ty_name = line["pub type ".len()..].split(' ').next().unwrap();
      current_entry.name = ty_name[..ty_name.len() - 2].to_string();
      current_entry.type_declaration = line.to_string();
    }
  }
  if !current_entry.name.is_empty() {
    entries.push(current_entry);
  }
  entries
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumerationEntry {
  pub name: String,
  pub comments: String,
  pub enum_declaration: String,
}

pub fn get_enumeration_entries() -> Vec<EnumerationEntry> {
  let mut entries = Vec::new();
  let mut current_entry = EnumerationEntry::default();
  for line in include_str!("gl_enumerations.rs").lines().skip(1) {
    if line.starts_with("///") || line.starts_with('#') {
      current_entry.comments.push_str(line);
      current_entry.comments.push('\n');
    } else if line.starts_with("//") {
      continue;
    } else if line.is_empty() {
      if !current_entry.name.is_empty() {
        entries.push(current_entry);
        current_entry = EnumerationEntry::default();
      } else {
        continue;
      }
    } else {
      let ty_name = line["pub const ".len()..].split(':').next().unwrap();
      current_entry.name = ty_name[..ty_name.len()].to_string();
      current_entry.enum_declaration = line.to_string();
    }
  }
  if !current_entry.name.is_empty() {
    entries.push(current_entry);
  }
  entries
}
