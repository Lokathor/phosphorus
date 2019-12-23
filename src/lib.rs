#![allow(unused_labels)]
#![allow(clippy::unused_label)]
#![allow(clippy::cognitive_complexity)]
#![warn(missing_docs)]

//! `phosphorus` is a GL bindings generator crate.
//!
//! * **Status:** Experimental, do not use.

use hashbrown::*;

pub mod xml;
pub use xml::*;

/// The broad categories of GL API
pub enum ApiCategory {
  /// OpenGL
  Gl,
  /// OpenGL ES 1.0
  Gles1,
  /// OpenGL ES 2.0 and later (including 3.0!)
  Gles2,
  /// OpenGL ES Safety Critical 2.0
  Glsc2,
}

/// The two profiles that you can use with GL (if that version uses profiles).
pub enum Profile {
  /// The core functionality only
  Core,
  /// Includes backwards compatability
  Compatability
}

