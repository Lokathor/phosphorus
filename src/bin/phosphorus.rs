//! Parses `gl.xml` and then prints out Rust source that can bind to it.

use magnesium::{XmlElement::*, *};

fn main() {
  let gl_xml =
    std::fs::read_to_string("target/gl.xml").expect("Couldn't read gl.xml");

  let registry = GlRegistry::from_iter(iter);
  let selection = GlApiSelection::new_from_registry_api_extensions(
    &registry,
    ApiGroup::Gl,
    (4, 6),
    GlProfile::Core,
    &[
      "GL_EXT_texture_filter_anisotropic",
      "GL_ARB_draw_buffers_blend",
      "GL_ARB_program_interface_query",
    ],
  );
  println!("{}", selection);
}
