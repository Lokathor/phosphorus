use super::*;

/// A GL type alias.
///
/// The `gl.xml` file doesn't actually handle when to include a type or not very
/// well, so we simply emit _all_ types regardless of feature selected.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Type {
  /// Just the name of the C type alias (for easy searching).
  name: String,
  /// The full text of the C `typedef` declaration.
  text: String,
}
impl core::fmt::Display for Type {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if self.text.starts_with("typedef") {
      let c_name = {
        let mut it = self.text.split_whitespace();
        assert_eq!(it.next(), Some("typedef"));
        match it.next().unwrap() {
          "void" => {
            let next = it.next().unwrap();
            if next.starts_with('*') {
              "*mut c_void"
            } else if next == "GLvoid;" {
              "c_void"
            } else {
              match self.text.as_str() {
                "typedef void (*GLDEBUGPROC)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);" => "Option<unsafe extern \"system\" fn(GLenum,GLenum,GLuint,GLenum,GLsizei,*const GLchar,*const c_void)>",
                "typedef void (*GLDEBUGPROCARB)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);" => "Option<unsafe extern \"system\" fn(GLenum,GLenum,GLuint,GLenum,GLsizei,*const GLchar,*const c_void)>",
                "typedef void (*GLDEBUGPROCKHR)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);"=> "Option<unsafe extern \"system\" fn(GLenum,GLenum,GLuint,GLenum,GLsizei,*const GLchar,*const c_void)>",
                "typedef void (*GLDEBUGPROCAMD)(GLuint id,GLenum category,GLenum severity,GLsizei length,const GLchar *message,void *userParam);" => "Option<unsafe extern \"system\" fn(GLuint,GLenum,GLenum,GLsizei,*const GLchar,*mut c_void)>",
                "typedef void (*GLVULKANPROCNV)(void);" => "Option<unsafe extern \"system\" fn()>",
                other => panic!("unexpected:{}", other),
              }
            }
          }
          "khronos_int8_t" => "i8",
          "khronos_uint8_t" => "u8",
          "khronos_int16_t" => "i16",
          "khronos_uint16_t" => "u16",
          "int" => "c_int",
          "khronos_int32_t" => "i32",
          "khronos_float_t" => "c_float",
          "double" => "c_double",
          "char" => "c_char",
          "khronos_intptr_t" => "isize",
          "khronos_ssize_t" => "isize",
          "khronos_int64_t" => "i64",
          "khronos_uint64_t" => "u64",
          "struct" => {
            assert_eq!(it.next(), Some("__GLsync"));
            assert_eq!(it.next(), Some("*GLsync;"));
            "*mut __GLsync; pub struct __GLsync(u8)"
          }
          "GLintptr" => "GLintptr",
          "unsigned" => match it.next().unwrap() {
            "int" => "c_uint",
            "char" => "c_uchar",
            "short" => "c_ushort",
            other => panic!("unexpected unsigned:{}", other),
          },
          other => panic!("unexpected:{}", other),
        }
      };
      if self.name == "GLDEBUGPROC" {
        write!(
          f,
          "#[doc=\"`fn(source, type, id, severity, length, message, userParam)`\"]"
        )?;
      }
      if self.name == "GLDEBUGPROCARB" {
        write!(
          f,
          "#[doc=\"`fn(source, type, id, severity, length, message, userParam)`\"]"
        )?;
      }
      if self.name == "GLDEBUGPROCKHR" {
        write!(
          f,
          "#[doc=\"`fn(source, type, id, severity, length, message, userParam)`\"]"
        )?;
      }
      if self.name == "GLDEBUGPROCAMD" {
        write!(
          f,
          "#[doc=\"`fn(id, category, severity, length, message, userParam)`\"]"
        )?;
      }
      write!(
        f,
        "pub type {r_name} = {c_name};",
        r_name = self.name,
        c_name = c_name,
      )
    } else {
      match self.name.as_str() {
        "khrplatform" => Ok(()),
        "struct _cl_context" => {
          write!(f, "pub struct _cl_context(u8);")
        }
        "struct _cl_event" => {
          write!(f, "pub struct _cl_event(u8);")
        }
        "GLhandleARB" => {
          write!(
            f,
            "#[cfg(any(target_os=\"mac_os\", target_os=\"ios\"))]pub type GLhandleARB = *mut c_void;#[cfg(not(any(target_os=\"mac_os\", target_os=\"ios\")))]pub type GLhandleARB = c_uint;"
          )
        }
        other => panic!("unexpected:{}", other),
      }
    }
  }
}
impl Type {
  /// The new alias name.
  pub fn name(&self) -> &str {
    &self.name
  }
  /// The full C definition of the alias.
  pub fn text(&self) -> &str {
    &self.text
  }
}

/// The [`Types`] extracted from `gl.xml`
#[derive(Debug, Default, Clone)]
pub struct Types(pub(crate) Vec<Type>);

/// Pull all the type definitions from an XmlIterator.
#[must_use]
pub fn pull_types(it: &mut XmlIterator<'_>, types: &mut Types) -> Option<()> {
  loop {
    match it.next()? {
      EndTag { name: "types" } => return Some(()),
      StartTag { name: "type", attrs } => {
        let mut t = Type::default();
        for (k, v) in AttributeIterator::new(attrs) {
          match k {
            "name" => t.name = v.to_owned(),
            "comment" => (),
            "requires" => (),
            _ => panic!("unexpected attrs: {}", attrs),
          }
        }
        'type_tag: loop {
          match it.next()? {
            EndTag { name: "type" } => {
              break 'type_tag;
            }
            Text(text) => t.text.push_str(text),
            EmptyTag { name: "apientry", attrs: "" } => (),
            StartTag { name: "name", .. } => {
              match it.next()? {
                Text(text) => {
                  t.name.push_str(text);
                  t.text.push_str(text);
                }
                other => panic!("unexpected>{:?}", other),
              }
              match it.next()? {
                EndTag { name: "name" } => (),
                other => panic!("unexpected>{:?}", other),
              }
            }
            other => panic!("unexpected>{:?}", other),
          }
        }
        types.0.push(t);
      }
      other => panic!("unexpected>{:?}", other),
    }
  }
}
