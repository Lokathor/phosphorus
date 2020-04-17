use super::*;

#[derive(Debug, Clone, Default)]
pub struct Types {
  pub type_entries: Vec<TypeEntry>,
}
impl Types {
  pub fn make_types_module_string(&self) -> String {
    let mut out = String::new();
    writeln_str!(out, "// Note: The C types must already be in scope!");
    writeln_str!(out, "// Import them from winapi, libc, wherever.");
    writeln_str!(out, "pub use types::*;");
    writeln_str!(out, "pub mod types {{");
    writeln_str!(out, "  use super::*;");
    for type_entry in self.type_entries.iter() {
      for line in type_entry.static_str().lines() {
        writeln_str!(out, "  {}", line);
      }
    }
    writeln_str!(out, "}}");
    out
  }
}

pub(crate) fn parse_types<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
) -> Types {
  let mut types = Types::default();
  loop {
    match iter.next().unwrap() {
      EndTag { name: "types" } => return types,
      StartTag { name: "type", attrs } => {
        types.type_entries.push(parse_type(iter, attrs));
      }
      unknown => panic!("parse_types:{:?}", unknown),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct TypeEntry {
  pub attrs: HashMap<String, String>,
  pub text: String,
}

impl TypeEntry {
  pub fn static_str(&self) -> &'static str {
    match self.text.as_str() {
      "#include &lt;KHR/khrplatform.h&gt;" => "",
      "typedef unsigned int GLenum;" => "pub type GLenum = c_uint;",
      "typedef unsigned char GLboolean;" => "pub type GLboolean = c_uchar;",
      "typedef unsigned int GLbitfield;" => "pub type GLbitfield = c_uint;",
      "typedef void GLvoid;" => "pub type GLvoid = c_void;",
      "typedef khronos_int8_t GLbyte;" => "pub type GLbyte = i8;",
      "typedef khronos_uint8_t GLubyte;" => "pub type GLubyte = u8;",
      "typedef khronos_int16_t GLshort;" => "pub type GLshort = i16;",
      "typedef khronos_uint16_t GLushort;" => "pub type GLushort = u16;",
      "typedef int GLint;" => "pub type GLint = c_int;",
      "typedef unsigned int GLuint;" => "pub type GLuint = c_uint;",
      "typedef khronos_int32_t GLclampx;" => "pub type GLclampx = i32;",
      "typedef int GLsizei;" => "pub type GLsizei = c_int;",
      "typedef khronos_float_t GLfloat;" => "pub type GLfloat = c_float;",
      "typedef khronos_float_t GLclampf;" => "pub type GLclampf = c_float;",
      "typedef double GLdouble;" => "pub type GLdouble = c_double;",
      "typedef double GLclampd;" => "pub type GLclampd = c_double;",
      "typedef void *GLeglClientBufferEXT;" => {
        "pub type GLeglClientBufferEXT = *mut c_void;"
      }
      "typedef void *GLeglImageOES;" => "pub type GLeglImageOES = *mut c_void;",
      "typedef char GLchar;" => "pub type GLchar = c_char;",
      "typedef char GLcharARB;" => "pub type GLcharARB = c_char;",
      "#ifdef __APPLE__\ntypedef void *GLhandleARB;\n#else\ntypedef unsigned int GLhandleARB;\n#endif" => {
        "#[cfg(any(target_os=\"macos\", target_os=\"ios\"))]\npub type GLhandleARB = *mut c_void;\n#[cfg(not(any(target_os=\"macos\", target_os=\"ios\")))]\npub type GLhandleARB = c_uint;"
      }
      "typedef khronos_uint16_t GLhalf;" => "pub type GLhalf = u16;",
      "typedef khronos_uint16_t GLhalfARB;" => "pub type GLhalfARB = u16;",
      "typedef khronos_int32_t GLfixed;" => "pub type GLfixed = i32;",
      "typedef khronos_intptr_t GLintptr;" => "pub type GLintptr = isize;",
      "typedef khronos_intptr_t GLintptrARB;" => "pub type GLintptrARB = isize;",
      "typedef khronos_ssize_t GLsizeiptr;" => "pub type GLsizeiptr = isize;",
      "typedef khronos_ssize_t GLsizeiptrARB;" => "pub type GLsizeiptrARB = isize;",
      "typedef khronos_int64_t GLint64;" => "pub type GLint64 = i64;",
      "typedef khronos_int64_t GLint64EXT;" => "pub type GLint64EXT = i64;",
      "typedef khronos_uint64_t GLuint64;" => "pub type GLuint64 = u64;",
      "typedef khronos_uint64_t GLuint64EXT;" => "pub type GLuint64EXT = u64;",
      "typedef struct __GLsync *GLsync;" => "pub struct __GLsync { _unused: u8 };\npub type pub GLsync = *mut __GLsync;",
      "struct _cl_context;" => "pub struct _cl_context { _unused: u8 };",
      "struct _cl_event;" => "pub struct _cl_event; { _unused: u8 };",
      "typedef void ( *GLDEBUGPROC)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);" => r#"pub type GLDEBUGPROC = Option<extern "system" fn(GLenum,GLenum,GLuint,GLenum,GLsizei,*const GLchar,*const c_void)>;"#,
      "typedef void ( *GLDEBUGPROCARB)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);" => r#"pub type GLDEBUGPROCARB = Option<extern "system" fn(GLenum,GLenum,GLuint,GLenum,GLsizei,*const GLchar,*const c_void)>;"#,
      "typedef void ( *GLDEBUGPROCKHR)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);" => r#"pub type GLDEBUGPROCARB = Option<extern "system" fn(GLenum,GLenum,GLuint,GLenum,GLsizei,*const GLchar,*const c_void)>;"#,
      "typedef void ( *GLDEBUGPROCAMD)(GLuint id,GLenum category,GLenum severity,GLsizei length,const GLchar *message,void *userParam);" => r#"pub type GLDEBUGPROCAMD = Option<extern "system" fn(GLuint,GLenum,GLenum,GLsizei,*const GLchar,*mut c_void)>;"#,
      "typedef unsigned short GLhalfNV;" => "pub type GLhalfNV = c_ushort;",
      "typedef GLintptr GLvdpauSurfaceNV;" => "pub type GLvdpauSurfaceNV = GLintptr;",
      "typedef void ( *GLVULKANPROCNV)(void);" => r#"pub type GLVULKANPROCNV = Option<unsafe "system" fn()>;"#,
      unknown => panic!("TypeEntry::make_type_entry_string>{}", unknown),
    }
  }
}

fn parse_type<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &str,
) -> TypeEntry {
  let mut type_entry = TypeEntry::default();
  type_entry.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => return type_entry,
      Text(t) => type_entry.text.push_str(t),
      StartTag { name: "name", attrs: "" } => (),
      EndTag { name: "name" } => (),
      EmptyTag { name: "apientry", attrs: "" } => (),
      unknown => panic!("parse_type:{:?}", unknown),
    }
  }
}
