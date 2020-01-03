//! Module for stuff that lets you select a feature and spit out some code.

use super::*;

/// Lets you list the extensions that are compatible with a feature name.
pub fn available_extensions<'r>(
  reg: &'r GlRegistry,
  name: &str,
) -> Result<impl Iterator<Item = &'r Extension>, ()> {
  match reg.features().iter().find(|f| f.name == name) {
    None => Err(()),
    Some(feature) => Ok(reg.extensions().iter().filter(move |ex| {
      ex.supported.split('|').any(|support| support == feature.api.as_str())
    })),
  }
}

fn feature_layers(name: &str) -> &[&'static str] {
  const GL: [&str; 19] = [
    "GL_VERSION_1_0",
    "GL_VERSION_1_1",
    "GL_VERSION_1_2",
    "GL_VERSION_1_3",
    "GL_VERSION_1_4",
    "GL_VERSION_1_5",
    "GL_VERSION_2_0",
    "GL_VERSION_2_1",
    "GL_VERSION_3_0",
    "GL_VERSION_3_1",
    "GL_VERSION_3_2",
    "GL_VERSION_3_3",
    "GL_VERSION_4_0",
    "GL_VERSION_4_1",
    "GL_VERSION_4_2",
    "GL_VERSION_4_3",
    "GL_VERSION_4_4",
    "GL_VERSION_4_5",
    "GL_VERSION_4_6",
  ];

  const GLES1: [&str; 1] = ["GL_VERSION_ES_CM_1_0"];

  const GLES2: [&str; 4] = [
    "GL_ES_VERSION_2_0",
    "GL_ES_VERSION_3_0",
    "GL_ES_VERSION_3_1",
    "GL_ES_VERSION_3_2",
  ];

  const GLSC2: [&str; 1] = ["GL_SC_VERSION_2_0"];

  match name {
    // gl
    "GL_VERSION_1_0" => &GL[..1],
    "GL_VERSION_1_1" => &GL[..2],
    "GL_VERSION_1_2" => &GL[..3],
    "GL_VERSION_1_3" => &GL[..4],
    "GL_VERSION_1_4" => &GL[..5],
    "GL_VERSION_1_5" => &GL[..6],
    "GL_VERSION_2_0" => &GL[..7],
    "GL_VERSION_2_1" => &GL[..8],
    "GL_VERSION_3_0" => &GL[..9],
    "GL_VERSION_3_1" => &GL[..10],
    "GL_VERSION_3_2" => &GL[..11],
    "GL_VERSION_3_3" => &GL[..12],
    "GL_VERSION_4_0" => &GL[..13],
    "GL_VERSION_4_1" => &GL[..14],
    "GL_VERSION_4_2" => &GL[..15],
    "GL_VERSION_4_3" => &GL[..16],
    "GL_VERSION_4_4" => &GL[..17],
    "GL_VERSION_4_5" => &GL[..18],
    "GL_VERSION_4_6" => &GL[..19],

    // gles1
    "GL_VERSION_ES_CM_1_0" => &GLES1[..1],

    // gles2
    "GL_ES_VERSION_2_0" => &GLES2[..1],
    "GL_ES_VERSION_3_0" => &GLES2[..2],
    "GL_ES_VERSION_3_1" => &GLES2[..3],
    "GL_ES_VERSION_3_2" => &GLES2[..4],

    // glsc2
    "GL_SC_VERSION_2_0" => &GLSC2[..1],

    // have to pick one of the real ones!
    _ => &[],
  }
}

/// Lets you prep the output.
#[derive(Debug, Clone)]
pub struct Output {
  /// Feature name
  pub name: String,
  /// Profile selected
  pub profile: Profile,
  /// Extensions selected,
  pub extensions: Vec<String>,
  /// All type aliases required
  pub types: HashSet<Type>,
  /// All enums required.
  ///
  /// Display with `EnumDisplay`
  pub enums: HashMap<String, EnumValue>,
  /// All groups required
  ///
  /// Display with `GroupDisplay`
  pub groups: HashMap<String, HashSet<String>>,
  /// All commands required.
  pub commands: HashSet<Command>,
}

impl Output {
  /// Fills in a new output struct.
  pub fn new(
    reg: &GlRegistry,
    name: &str,
    profile: Profile,
    extensions: &[&str],
  ) -> Self {
    let mut out = Output {
      name: name.to_string(),
      profile,
      extensions: extensions.iter().map(|s| (*s).to_string()).collect(),
      types: HashSet::new(),
      enums: HashMap::new(),
      groups: HashMap::new(),
      commands: HashSet::new(),
    };

    let api: String =
      format!("{:?}", ApiCategory::from_name(name)).to_lowercase();

    for feat_name in feature_layers(name) {
      let feature =
        reg.features().iter().find(|f| &f.name == feat_name).unwrap();
      for (req_k, req_vals) in feature.requirements.iter() {
        // Note: we allow `None` to match our selected profile too!
        if let Some(p) = req_k {
          if *p != profile {
            continue;
          }
        }
        for req in req_vals {
          match req {
            FeatureRequirement::Type { name } => {
              let t =
                reg.types().iter().find(|t| &t.name == name).unwrap().clone();
              out.types.insert(t);
            }
            FeatureRequirement::Enum { name } => {
              let the_api_key =
                EnumKey { name: name.clone(), api: Some(api.clone()) };
              let the_none_key = EnumKey { name: name.clone(), api: None };
              let (k, v) = reg
                .enums()
                .iter()
                .find(|(k, _)| k == &&the_api_key || k == &&the_none_key)
                .unwrap();
              out.enums.insert(k.name.clone(), *v);
            }
            FeatureRequirement::Command { name } => {
              let c = reg
                .commands()
                .iter()
                .find(|c| &c.name == name)
                .unwrap()
                .clone();
              out.commands.insert(c);
            }
          }
        }
      }
      for (rem_k, rem_vals) in feature.removals.iter() {
        // Note: we allow `None` to match our selected profile too!
        if let Some(p) = rem_k {
          if *p != profile {
            continue;
          }
        }
        for rem in rem_vals {
          match rem {
            FeatureRemoval::Type { name } => {
              let t = reg.types().iter().find(|t| &t.name == name).unwrap();
              out.types.remove(t);
            }
            FeatureRemoval::Enum { name } => {
              out.enums.remove(name);
            }
            FeatureRemoval::Command { name } => {
              let c = reg.commands().iter().find(|c| &c.name == name).unwrap();
              out.commands.remove(c);
            }
          }
        }
      }
    }

    for extension_name in extensions {
      let extension = reg
        .extensions()
        .iter()
        .find(|ex| {
          &ex.name == extension_name
            && ex.supported.split('|').any(|s| s == api)
        })
        .unwrap();
      for req in extension.requirements.iter() {
        // if an API is specified but it's not our API, skip
        if req.api.as_ref().map(|s| s != &api).unwrap_or(false) {
          continue;
        }
        // if a profile is specified but it's not our profile, skip
        if req.profile.as_ref().map(|s| s != &api).unwrap_or(false) {
          continue;
        }
        for item in req.items.iter() {
          match item {
            FeatureRequirement::Type { name } => {
              let t =
                reg.types().iter().find(|t| &t.name == name).unwrap().clone();
              out.types.insert(t);
            }
            FeatureRequirement::Enum { name } => {
              let the_api_key =
                EnumKey { name: name.clone(), api: Some(api.clone()) };
              let the_none_key = EnumKey { name: name.clone(), api: None };
              let (k, v) = reg
                .enums()
                .iter()
                .find(|(k, _)| k == &&the_api_key || k == &&the_none_key)
                .unwrap();
              out.enums.insert(k.name.clone(), *v);
            }
            FeatureRequirement::Command { name } => {
              let c = reg
                .commands()
                .iter()
                .find(|c| &c.name == name)
                .unwrap()
                .clone();
              out.commands.insert(c);
            }
          }
        }
      }
    }

    for cmd in out.commands.iter() {
      use hashbrown::hash_map::Entry;
      if let Some(ref ty) = cmd.return_type {
        let t = reg.types().iter().find(|t| &t.name == ty).unwrap().clone();
        out.types.insert(t);
      }
      if let Some(ref group) = cmd.return_group {
        match out.groups.entry(group.to_owned()) {
          Entry::Occupied(_) => (),
          Entry::Vacant(ve) => {
            if let Some(gr_set) = reg.groups().get(group) {
              // adding groups into the output is best effort! There are many
              // groups "required" that aren't defined in `gl.xml`.
              ve.insert(gr_set.clone());
            }
          }
        }
      }
      for param in cmd.params.iter() {
        // This is just an approximation so we can pull in all the right types,
        // it doesn't have to parse very properly.
        let ty = if param.ptype.contains("void") {
          "GLvoid"
        } else {
          let mut t = param.ptype.as_str();
          if t.starts_with("const ") {
            t = &t[6..];
          }
          if t.contains('*') {
            t = t.split('*').next().unwrap()
          }
          t
        };
        let t = reg.types().iter().find(|t| t.name == ty).unwrap().clone();
        out.types.insert(t);
        if let Some(ref group) = param.group {
          match out.groups.entry(group.to_owned()) {
            Entry::Occupied(_) => (),
            Entry::Vacant(ve) => {
              if let Some(gr_set) = reg.groups().get(group) {
                // adding groups into the output is best effort! There are many
                // groups "required" that aren't defined in `gl.xml`.
                ve.insert(gr_set.clone());
              }
            }
          }
        }
      }
    }

    out
  }
}

/// The standard generator printer.
pub struct StandardGenerator<'o> {
  /// The output database to display
  pub output: &'o Output,
}
impl core::fmt::Display for StandardGenerator<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let output = self.output;
    let version = env!("CARGO_PKG_VERSION");

    writeln!(f, "#![cfg_attr(not(any(feature = \"trace_calls\", feature = \"error_checks\")), no_std)]")?;
    writeln!(f, "#![allow(bad_style)]")?;
    writeln!(f, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(f, "#![allow(clippy::missing_safety_doc)]")?;
    writeln!(f, "#![allow(clippy::let_unit_value)]")?;
    writeln!(f, "#![allow(clippy::let_and_return)]")?;
    writeln!(f, "#![allow(clippy::too_many_arguments)]")?;
    writeln!(f, "#![allow(clippy::many_single_char_names)]")?;

    writeln!(f)?;
    writeln!(f, "/*")?;
    writeln!(f, "Bindings for GL.")?;
    writeln!(f)?;
    writeln!(f, "* API: {}", output.name)?;
    writeln!(f, "* profile: {}", output.profile)?;
    writeln!(f, "* extensions: {:?}", output.extensions)?;
    writeln!(f)?;
    writeln!(f, "generated by phosphorus-{}", version)?;
    writeln!(f, "*/")?;
    writeln!(f, "fn main() {{ }}")?;

    writeln!(f)?;
    writeln!(
      f,
      "// provides C types, replace with your own source if you like."
    )?;
    writeln!(f, "pub use chlorine::*;")?;

    {
      writeln!(f)?;
      writeln!(f, "pub use types::*;")?;
      writeln!(f, "pub mod types {{")?;
      writeln!(f, "  //! Type aliases used by GL.")?;
      writeln!(f)?;
      writeln!(
        f,
        "  // Note: A source of C types must be in scope! winapi, libc, etc."
      )?;
      writeln!(f, "  use super::*;")?;
      let mut type_v: Vec<&Type> = output.types.iter().collect();
      type_v.sort();
      for t_ref in type_v {
        if f.alternate() {
          writeln!(f, "  {:#}", t_ref)?;
        } else {
          writeln!(f, "  {}", t_ref)?;
        }
      }
      writeln!(f, "}}")?;
    }

    {
      writeln!(f)?;
      writeln!(f, "pub use groups::*;")?;
      writeln!(f, "pub mod groups {{")?;
      writeln!(f, "  //! Various aliases for `GLenum`.")?;
      writeln!(f, "  //!")?;
      writeln!(f, "  //! This helps function signatures be more clear about what `GLenum` values")?;
      writeln!(f, "  //! they allow.")?;
      writeln!(f, "  //!")?;
      writeln!(f, "  //! **Important**: The groups info is **not** necessarily correct! Khronos")?;
      writeln!(f, "  //! considers it to be low-value metadata, and so they don't keep the groups")?;
      writeln!(f, "  //! info up to date with all API levels and Extension alterations. That is why")?;
      writeln!(f, "  //! it's presented only as an alias, because you might need to pass other")?;
      writeln!(f, "  //! `GLenum` values based on your exact use case.")?;
      writeln!(f, "  use super::*;")?;
      let mut group_v: Vec<String> = output
        .groups
        .iter()
        .map(|(name, entries)| format!("{}", GroupDisplay { name, entries }))
        .collect();
      group_v.sort();
      for group_string in group_v {
        writeln!(f, "  {}", group_string)?;
      }
      writeln!(f, "}}")?;
    }

    {
      writeln!(f)?;
      writeln!(f, "pub use enums::*;")?;
      writeln!(f, "pub mod enums {{")?;
      writeln!(f, "  //! All the enumerated constants.")?;
      writeln!(f, "  use super::*;")?;
      let mut enums_v: Vec<String> = output
        .enums
        .iter()
        .map(|(name, value)| {
          let key = &EnumKey { name: name.to_owned(), api: None };
          if f.alternate() {
            format!("{:#}", EnumDisplay { key, value })
          } else {
            format!("{}", EnumDisplay { key, value })
          }
        })
        .collect();
      enums_v.sort();
      for enum_string in enums_v {
        writeln!(f, "  {}", enum_string)?;
      }
      writeln!(f, "}}")?;
    }

    // Elements common to both loaders
    {
      writeln!(f)?;
      writeln!(
        f,
        r#"#[cfg(any(feature="global_loader", feature="struct_loader"))]"#
      )?;
      writeln!(f, "pub(crate) use loader_common::*;")?;
      writeln!(
        f,
        r#"#[cfg(any(feature="global_loader", feature="struct_loader"))]"#
      )?;
      writeln!(f, "pub mod loader_common {{")?;
      writeln!(f, "  //! Elements common to both loaders.")?;
      writeln!(f, "  use super::*;")?;

      // TODO: make the function type aliases and also the function name strings
      // be part of the common module.

      writeln!(f)?;
      writeln!(f, "  /// Function pointer sanity check.")?;
      writeln!(f, "  ///")?;
      writeln!(f, "  /// * Null pointers (0) are bad.")?;
      writeln!(
        f,
        "  /// * Sometimes windows will return non-null error values."
      )?;
      writeln!(
        f,
        "  ///   * Known non-null error values include 1, 2, 3, and -1."
      )?;
      writeln!(f, "  pub(crate) fn fn_ptr_ok(p: *const c_void) -> bool {{")?;
      writeln!(f, "    let p_u = p as usize;")?;
      writeln!(f, "    (p_u >= 8) && (p_u != usize::max_value())")?;
      writeln!(f, "  }}")?;

      writeln!(f)?;
      writeln!(f, "  pub(crate) fn call_loader(")?;
      writeln!(f, "    loader: &mut dyn FnMut(*const c_char) -> *mut c_void,")?;
      writeln!(f, "    name: &[u8],")?;
      writeln!(f, "  ) -> *mut c_void {{")?;
      writeln!(f, "    debug_assert!(*name.last().unwrap() == 0_u8);")?;
      writeln!(f, "    let p = loader(name.as_ptr() as *const c_char);")?;
      writeln!(f, "    if fn_ptr_ok(p) {{")?;
      writeln!(f, "      p")?;
      writeln!(f, "    }} else {{")?;
      writeln!(f, "      core::ptr::null_mut()")?;
      writeln!(f, "    }}")?;
      writeln!(f, "  }}")?;

      let enum_prefix = if f.alternate() { "" } else { "GL_" };
      writeln!(f)?;
      writeln!(f, "  #[cfg(feature=\"error_checks\")]")?;
      writeln!(f, "  use std::borrow::Cow;")?;
      writeln!(f, "  #[cfg(feature=\"error_checks\")]")?;
      writeln!(
        f,
        "  pub(crate) fn error_name_for(err: GLenum) -> Cow<'static, str> {{"
      )?;
      writeln!(f, "    match err {{")?;
      for short_name in &[
        "INVALID_ENUM",
        "INVALID_VALUE",
        "INVALID_OPERATION",
        "INVALID_FRAMEBUFFER_OPERATION",
        "OUT_OF_MEMORY",
        "STACK_UNDERFLOW",
        "STACK_OVERFLOW",
      ] {
        let gl_name = format!("GL_{}", short_name);
        let prefixed_name = format!("{}{}", enum_prefix, short_name);
        if output.enums.keys().any(|k| k == &gl_name) {
          writeln!(
            f,
            "      {prefixed_name} => Cow::Borrowed(\"{prefixed_name}\"),",
            prefixed_name = prefixed_name
          )?;
        }
      }
      writeln!(f, "      _ => Cow::Owned(format!(\"0x{{:X}}\", err)),")?;
      writeln!(f, "    }}")?;
      writeln!(f, "  }}")?;

      writeln!(f)?;

      // end of module
      writeln!(f, "}}")?;
    }

    {
      let fn_prefix = if f.alternate() { "" } else { "gl" };
      writeln!(f)?;
      writeln!(f, r#"#[cfg(feature="global_loader")]"#)?;
      writeln!(f, "pub use global_loader::*;")?;
      writeln!(f, r#"#[cfg(feature="global_loader")]"#)?;
      writeln!(f, "pub mod global_loader {{")?;
      writeln!(f, "  use super::*;")?;
      writeln!(f, "  use core::ptr::null_mut;")?;
      writeln!(f, "  use core::mem::transmute;")?;
      writeln!(f, "  use core::ops::Not;")?;
      writeln!(f, "  use core::sync::atomic::{{AtomicPtr, Ordering}};")?;
      writeln!(f)?;
      writeln!(f, "  #[doc = \"Loads all the global GL functions.\\n\\nGiven a function from C string pointer to GL function pointer, loads all the GL functions.\\n\\n## Safety\\nThis can check for nulls and other common error values, but otherwise mostly trusts whatever pointer is returned.\"]")?;
      writeln!(f, "  pub unsafe fn global_load<F>(mut f: F)")?;
      writeln!(f, "    where F: FnMut(*const c_char) -> *mut c_void")?;
      writeln!(f, "  {{")?;
      for command in output.commands.iter() {
        let name = &command.name[2..];
        writeln!(
          f,
          "    load_{fn_prefix}{name}(&mut f);",
          name = name,
          fn_prefix = fn_prefix
        )?;
      }
      writeln!(f, "  }}")?;
      let mut command_v: Vec<String> = output
        .commands
        .iter()
        .map(|command| format!("{}", GlobalCommand { command }))
        .collect();
      command_v.sort();
      for command_string in command_v {
        writeln!(f)?;
        writeln!(f, "  {}", command_string)?;
      }
      writeln!(f, "}}")?;
      // TODO: output the commands as global functions.
    }

    {
      writeln!(f)?;
      writeln!(f, r#"#[cfg(feature="struct_loader")]"#)?;
      writeln!(f, "pub use struct_loader::*;")?;
      writeln!(f, r#"#[cfg(feature="struct_loader")]"#)?;
      writeln!(f, "pub mod struct_loader {{")?;
      writeln!(f, "  use super::*;")?;
      writeln!(f, "  // TODO!")?;
      writeln!(f, "}}")?;
      // TODO: output the commands as struct functions.
    }
    Ok(())
  }
}
