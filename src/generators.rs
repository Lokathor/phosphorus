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

pub struct Output {
  types: HashSet<Type>,
  enums: HashMap<String, EnumValue>,
  groups: HashMap<String, HashSet<String>>,
  commands: HashSet<Command>,
}
impl Output {
  pub fn new(
    reg: &GlRegistry,
    name: &str,
    profile: Profile,
    extensions: &[&str],
  ) -> Self {
    let mut out = Output {
      types: HashSet::new(),
      enums: HashMap::new(),
      groups: HashMap::new(),
      commands: HashSet::new(),
    };

    for feat_name in feature_layers(name) {
      let feature =
        reg.features().iter().find(|f| &f.name == feat_name).unwrap();
      
    }

    out
  }
}

// TODO: generate the lib.rs for an API level + extensions

// TODO: generate the global_loader.rs for an API level + extensions

// TODO: generate the struct_loader.rs for an API level + extensions
