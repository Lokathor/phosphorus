#![allow(bad_style)]

// ignore this, it's just in the example folder so I need a `main`.
fn main() {}

// module "prelude" content.

use phosphorus::gl_groups::*;

// given a command with this type signature as input

/// [glAccum](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAccum.xhtml)
/// * `op` group: AccumOp
/// * `value` group: CoordF
pub type glAccum_t = extern "system" fn(op: AccumOp, value: f32);

// we'd get a loader struct like this

/// Struct for Open GL 1.0 functions
pub struct GlFns10 {
  glAccum_p: glAccum_t,
  // one field per command
}
// creation method
impl GlFns10 {
  /// Loads all functions.
  ///
  /// ## Errors
  /// If any function fails to load this will error. The returned error is the
  /// name of the first function that failed to load.
  pub fn load_from(_loader_fn: ()) -> Result<Self, &'static str> {
    todo!()
  }
}
// methods to call the gl functions
impl GlFns10 {
  pub fn glAccum(&self, op: AccumOp, value: f32) {
    (self.glAccum_p)(op, value)
  }

  // one call method per command
}
