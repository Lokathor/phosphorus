#![allow(unused_imports)]

use magnesium::{XmlElement::*, *};

// first we put the module into our source tree so that RA will scan it for
// validity and such.
mod gl_core_types;
// but it's meant for use with our generator output, so what we really want is
// to have the module in string form.
pub const GL_CORE_TYPES: &str = include_str!("gl_core_types.rs");

pub mod temp;
