pub use core::ffi::c_void;

/// `typedef unsigned int GLenum;`
pub type GLenum = core::ffi::c_uint;

/// `typedef unsigned char GLboolean;`
pub type GLboolean = core::ffi::c_uchar;

/// `typedef unsigned int GLbitfield;`
pub type GLbitfield = core::ffi::c_uint;

/// `typedef void GLvoid;`
pub type GLvoid = c_void;

/// `typedef khronos_int8_t GLbyte;`
pub type GLbyte = i8;

/// `typedef khronos_uint8_t GLubyte;`
pub type GLubyte = u8;

/// `typedef khronos_int16_t GLshort;`
pub type GLshort = i16;

/// `typedef khronos_uint16_t GLushort;`
pub type GLushort = u16;

/// `typedef int GLint;`
pub type GLint = core::ffi::c_int;

/// `typedef unsigned int GLuint;`
pub type GLuint = core::ffi::c_uint;

/// `typedef khronos_int32_t GLclampx;`
pub type GLclampx = i32;

/// `typedef int GLsizei;`
pub type GLsizei = core::ffi::c_int;

/// `typedef khronos_float_t GLfloat;`
pub type GLfloat = core::ffi::c_float;

/// `typedef khronos_float_t GLclampf;`
pub type GLclampf = core::ffi::c_float;

/// `typedef double GLdouble;`
pub type GLdouble = core::ffi::c_double;

/// `typedef double GLclampd;`
pub type GLclampd = core::ffi::c_double;

/// `typedef void *GLeglClientBufferEXT;`
pub type GLeglClientBufferEXT = *mut c_void;

/// `typedef void * GLeglImageOES;`
pub type GLeglImageOES = *mut c_void;

/// `typedef char GLchar;`
pub type GLchar = core::ffi::c_char;

/// `typedef char GLcharARB;`
pub type GLcharARB = core::ffi::c_char;

/// ```c
/// #ifdef __APPLE__
/// typedef void *GLhandleARB;
/// #else
/// typedef unsigned int GLhandleARB;
/// #endif
/// ````
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub type GLhandleARB = *mut c_void;

/// ```c
/// #ifdef __APPLE__
/// typedef void *GLhandleARB;
/// #else
/// typedef unsigned int GLhandleARB;
/// #endif
/// ````
#[cfg(not(any(target_os = "ios", target_os = "macos")))]
pub type GLhandleARB = GLuint;

/// `typedef khronos_uint16_t GLhalf;`
pub type GLhalf = u16;

/// `typedef khronos_uint16_t GLhalfARB;`
pub type GLhalfARB = u16;

/// `typedef khronos_int32_t GLfixed;`
pub type GLfixed = i32;

/// `typedef khronos_intptr_t GLintptr;`
pub type GLintptr = isize;

/// `typedef khronos_intptr_t GLintptrARB;`
pub type GLintptrARB = isize;

/// `typedef khronos_ssize_t GLsizeiptr;`
pub type GLsizeiptr = isize;

/// `typedef khronos_ssize_t GLsizeiptrARB;`
pub type GLsizeiptrARB = isize;

/// `typedef khronos_int64_t GLint64;`
pub type GLint64 = i64;

/// `typedef khronos_int64_t GLint64EXT;`
pub type GLint64EXT = i64;

/// `typedef khronos_uint64_t GLuint64;`
pub type GLuint64 = u64;

/// `typedef khronos_uint64_t GLuint64EXT;`
pub type GLuint64EXT = u64;

/// `typedef struct __GLsync * GLsync;`
pub type GLsync = *mut c_void;

/// `struct _cl_context;`
#[allow(non_camel_case_types)]
pub type _cl_context = c_void;

/// `struct _cl_event;`
#[allow(non_camel_case_types)]
pub type _cl_event = c_void;

/// ```c
/// typedef void (*GLDEBUGPROC)(
///   GLenum source,
///   GLenum type,
///   GLuint id,
///   GLenum severity,
///   GLsizei length,
///   const GLchar *message,
///   const void *userParam
/// );
/// ```
pub type GLDEBUGPROC = Option<
  unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void),
>;

/// ```c
/// typedef void (*GLDEBUGPROCARB)(
///   GLenum source,
///   GLenum type,
///   GLuint id,
///   GLenum severity,
///   GLsizei length,
///   const GLchar *message,
///   const void *userParam
/// );
/// ````
pub type GLDEBUGPROCARB = Option<
  unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void),
>;

/// ```c
/// typedef void (*GLDEBUGPROCKHR)(
///   GLenum source,
///   GLenum type,
///   GLuint id,
///   GLenum severity,
///   GLsizei length,
///   const GLchar *message,
///   const void *userParam
/// );
/// ```
pub type GLDEBUGPROCKHR = Option<
  unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void),
>;

/// ```c
/// typedef void (*GLDEBUGPROCAMD)(
///   GLuint id,
///   GLenum category,
///   GLenum severity,
///   GLsizei length,
///   const GLchar *message,
///   void *userParam
/// );
/// ```
pub type GLDEBUGPROCAMD =
  Option<unsafe extern "system" fn(GLuint, GLenum, GLenum, GLsizei, *const GLchar, *mut c_void)>;

/// `typedef unsigned short GLhalfNV;`
pub type GLhalfNV = core::ffi::c_ushort;

/// `typedef GLintptr GLvdpauSurfaceNV;`
pub type GLvdpauSurfaceNV = isize;

/// `typedef void (*GLVULKANPROCNV)(void);`
pub type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;
