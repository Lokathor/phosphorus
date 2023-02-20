#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
pub type GLenum = u32;
pub type GLbitfield = u32;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub type GLhandleARB = *mut void;
#[cfg(not(any(target_os = "ios", target_os = "macos")))]
pub type GLhandleARB = GLuint;
pub type GLeglClientBufferEXT = *mut void;
pub type GLeglImageOES = *mut void;
pub type GLsync = *mut void;
pub type _cl_context = void;
pub type _cl_event = void;
pub type GLDEBUGPROC = Option<
  unsafe extern "system" fn(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *const void,
  ),
>;
pub type GLDEBUGPROCARB = GLDEBUGPROC;
pub type GLDEBUGPROCKHR = GLDEBUGPROC;
pub type GLDEBUGPROCAMD = Option<
  unsafe extern "system" fn(
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut void,
  ),
>;
pub type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;
pub type GLboolean = u32;
pub type GLbyte = i8;
pub type GLcharARB = u8;
pub type GLclampd = f64;
pub type GLclampf = f32;
pub type GLclampx = i32;
pub type GLdouble = f64;
pub type GLfixed = i32;
pub type GLfloat = f32;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLhalfNV = u16;
pub type GLint = i32;
pub type GLint64 = i64;
pub type GLint64EXT = i64;
pub type GLintptr = isize;
pub type GLintptrARB = isize;
pub type GLshort = i16;
pub type GLsizei = u32;
pub type GLsizeiptr = isize;
pub type GLsizeiptrARB = isize;
pub type GLubyte = u8;
pub type GLuint = u32;
pub type GLuint64 = u64;
pub type GLuint64EXT = u64;
pub type GLushort = u16;
pub type GLvdpauSurfaceNV = GLintptr;
pub type void = core::ffi::c_void;
pub type GLchar = u8;
