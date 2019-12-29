#![allow(unused_labels)]
#![allow(clippy::unused_label)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::implicit_hasher)]
#![warn(missing_docs)]

//! `phosphorus` is a GL bindings generator crate.
//!
//! * **Status:** Experimental, do not use.

use hashbrown::*;

pub mod xml;
pub use xml::*;

pub mod generators;
pub use generators::*;

/// The broad categories of GL API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApiCategory {
  /// OpenGL
  Gl,
  /// OpenGL ES 1.0
  Gles1,
  /// OpenGL ES 2.0 and later (including 3.0!)
  Gles2,
  /// OpenGL Safety Critical 2.0
  Glsc2,
}
impl ApiCategory {
  /// Turn an API Feature Name into an ApiCategory, or panic.
  ///
  /// eg: "GL_VERSION_3_3" -> `ApiCategory::Gl`
  pub fn from_name(name: &str) -> Self {
    match name {
      "GL_SC_VERSION_2_0" => Self::Glsc2,
      "GL_ES_VERSION_3_2" => Self::Gles2,
      "GL_ES_VERSION_3_1" => Self::Gles2,
      "GL_ES_VERSION_3_0" => Self::Gles2,
      "GL_ES_VERSION_2_0" => Self::Gles2,
      "GL_VERSION_ES_CM_1_0" => Self::Gles1,
      "GL_VERSION_4_6" => Self::Gl,
      "GL_VERSION_4_5" => Self::Gl,
      "GL_VERSION_4_4" => Self::Gl,
      "GL_VERSION_4_3" => Self::Gl,
      "GL_VERSION_4_2" => Self::Gl,
      "GL_VERSION_4_1" => Self::Gl,
      "GL_VERSION_4_0" => Self::Gl,
      "GL_VERSION_3_3" => Self::Gl,
      "GL_VERSION_3_2" => Self::Gl,
      "GL_VERSION_3_1" => Self::Gl,
      "GL_VERSION_3_0" => Self::Gl,
      "GL_VERSION_2_1" => Self::Gl,
      "GL_VERSION_2_0" => Self::Gl,
      "GL_VERSION_1_5" => Self::Gl,
      "GL_VERSION_1_4" => Self::Gl,
      "GL_VERSION_1_3" => Self::Gl,
      "GL_VERSION_1_2" => Self::Gl,
      "GL_VERSION_1_1" => Self::Gl,
      "GL_VERSION_1_0" => Self::Gl,
      _ => panic!("Not an ApiCategory: {}", name),
    }
  }
}

/// The two profiles that you can use with GL (if that version uses profiles).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Profile {
  /// The core functionality only
  Core,
  /// Includes backwards compatibility
  Compatibility,
}

/// Gets the feature name for a given (category, major, minor) triple.
///
/// ## Failure
///
/// If the combination given isn't a valid feature combination then you will get
/// `None` back.
pub fn feature_name_for(
  category: ApiCategory,
  major: i32,
  minor: i32,
) -> Option<&'static str> {
  Some(match (category, major, minor) {
    (ApiCategory::Glsc2, 2, 0) => "GL_SC_VERSION_2_0",
    (ApiCategory::Gles2, 3, 2) => "GL_ES_VERSION_3_2",
    (ApiCategory::Gles2, 3, 1) => "GL_ES_VERSION_3_1",
    (ApiCategory::Gles2, 3, 0) => "GL_ES_VERSION_3_0",
    (ApiCategory::Gles2, 2, 0) => "GL_ES_VERSION_2_0",
    (ApiCategory::Gles1, 1, 0) => "GL_VERSION_ES_CM_1_0",
    (ApiCategory::Gl, 4, 6) => "GL_VERSION_4_6",
    (ApiCategory::Gl, 4, 5) => "GL_VERSION_4_5",
    (ApiCategory::Gl, 4, 4) => "GL_VERSION_4_4",
    (ApiCategory::Gl, 4, 3) => "GL_VERSION_4_3",
    (ApiCategory::Gl, 4, 2) => "GL_VERSION_4_2",
    (ApiCategory::Gl, 4, 1) => "GL_VERSION_4_1",
    (ApiCategory::Gl, 4, 0) => "GL_VERSION_4_0",
    (ApiCategory::Gl, 3, 3) => "GL_VERSION_3_3",
    (ApiCategory::Gl, 3, 2) => "GL_VERSION_3_2",
    (ApiCategory::Gl, 3, 1) => "GL_VERSION_3_1",
    (ApiCategory::Gl, 3, 0) => "GL_VERSION_3_0",
    (ApiCategory::Gl, 2, 1) => "GL_VERSION_2_1",
    (ApiCategory::Gl, 2, 0) => "GL_VERSION_2_0",
    (ApiCategory::Gl, 1, 5) => "GL_VERSION_1_5",
    (ApiCategory::Gl, 1, 4) => "GL_VERSION_1_4",
    (ApiCategory::Gl, 1, 3) => "GL_VERSION_1_3",
    (ApiCategory::Gl, 1, 2) => "GL_VERSION_1_2",
    (ApiCategory::Gl, 1, 1) => "GL_VERSION_1_1",
    (ApiCategory::Gl, 1, 0) => "GL_VERSION_1_0",
    _ => return None,
  })
}
