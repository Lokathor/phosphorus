#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
type GLenum = u32;
type GLbitfield = u32;
#[cfg(any(target_os = "ios", target_os = "macos"))]
type GLhandleARB = *mut void;
#[cfg(not(any(target_os = "ios", target_os = "macos")))]
type GLhandleARB = GLuint;
type GLeglClientBufferEXT = *mut void;
type GLeglImageOES = *mut void;
type GLsync = *mut void;
type _cl_context = void;
type _cl_event = void;
type GLDEBUGPROC = Option<
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
type GLDEBUGPROCARB = GLDEBUGPROC;
type GLDEBUGPROCKHR = GLDEBUGPROC;
type GLDEBUGPROCAMD = Option<
  unsafe extern "system" fn(
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut void,
  ),
>;
type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;
type GLboolean = bool;
type GLbyte = i8;
type GLcharARB = u8;
type GLclampd = f64;
type GLclampf = f32;
type GLclampx = i32;
type GLdouble = f64;
type GLfixed = i32;
type GLfloat = f32;
type GLhalf = u16;
type GLhalfARB = u16;
type GLhalfNV = u16;
type GLint = i32;
type GLint64 = i64;
type GLint64EXT = i64;
type GLintptr = isize;
type GLintptrARB = isize;
type GLshort = i16;
type GLsizei = u32;
type GLsizeiptr = isize;
type GLsizeiptrARB = isize;
type GLubyte = u8;
type GLuint = u32;
type GLuint64 = u64;
type GLuint64EXT = u64;
type GLushort = u16;
type GLvdpauSurfaceNV = GLintptr;
type void = core::ffi::c_void;
type GLchar = u8;
