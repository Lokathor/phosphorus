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

// TODO: ability to output Commands

// TODO: emit all output as a single file using internal modules so that users
// can break it up if they choose.
