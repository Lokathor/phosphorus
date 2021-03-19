#![allow(dead_code)]

use super::*;

/// A GL enumeration value.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLenum(pub u32);
impl core::fmt::Debug for GLenum {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "GlEnum(0x{:X})", self.0)
  }
}
impl Clone for GLenum {
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for GLenum {}

/// A GL bitfield value.
///
/// You can mix around the bits of these values using standard bitwise ops.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLbitfield(pub u32);
impl core::fmt::Debug for GLbitfield {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "GlBitfield(0b{:b})", self.0)
  }
}
impl Clone for GLbitfield {
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for GLbitfield {}
impl core::ops::BitAnd for GLbitfield {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self::Output {
    Self(self.0 & rhs.0)
  }
}
impl core::ops::BitAndAssign for GLbitfield {
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}
impl core::ops::BitOr for GLbitfield {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
impl core::ops::BitOrAssign for GLbitfield {
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}
impl core::ops::BitXor for GLbitfield {
  type Output = Self;
  fn bitxor(self, rhs: Self) -> Self::Output {
    Self(self.0 ^ rhs.0)
  }
}
impl core::ops::BitXorAssign for GLbitfield {
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}
impl core::ops::Not for GLbitfield {
  type Output = Self;
  fn not(self) -> Self::Output {
    Self(!self.0)
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLhandleARB(#[cfg(any(target_os = "ios", target_os = "macos"))] pub *mut void, #[cfg(not(any(target_os = "ios", target_os = "macos")))] pub GLuint);

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLeglClientBufferEXT(pub *mut void);

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLeglImageOES(pub *mut void);

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLsync(pub *mut void);

#[repr(transparent)]
pub struct _cl_context(pub void);

#[repr(transparent)]
pub struct _cl_event(pub void);

// Note(Lokathor): These type aliases are useful to end users.

pub type GLDEBUGPROC = Option<unsafe extern "system" fn(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const void)>;
pub type GLDEBUGPROCARB = GLDEBUGPROC;
pub type GLDEBUGPROCKHR = GLDEBUGPROC;
pub type GLDEBUGPROCAMD = Option<unsafe extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut void)>;
pub type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;

// Note(Lokathor): These type aliases are *NOT* useful. Instead we will declare
// them just for use within the crate. Since they aren't world-visible, the
// outside world will show the base Rust type on function arguments.

pub(crate) type GLboolean = bool;
pub(crate) type GLbyte = i8;
pub(crate) type GLcharARB = u8;
pub(crate) type GLclampd = f64;
pub(crate) type GLclampf = f32;
pub(crate) type GLclampx = i32;
pub(crate) type GLdouble = f64;
pub(crate) type GLfixed = i32;
pub(crate) type GLfloat = f32;
pub(crate) type GLhalf = u16;
pub(crate) type GLhalfARB = u16;
pub(crate) type GLhalfNV = u16;
pub(crate) type GLint = i32;
pub(crate) type GLint64 = i64;
pub(crate) type GLint64EXT = i64;
pub(crate) type GLintptr = isize;
pub(crate) type GLintptrARB = isize;
pub(crate) type GLshort = i16;
pub(crate) type GLsizei = u32;
pub(crate) type GLsizeiptr = isize;
pub(crate) type GLsizeiptrARB = isize;
pub(crate) type GLubyte = u8;
pub(crate) type GLuint = u32;
pub(crate) type GLuint64 = u64;
pub(crate) type GLuint64EXT = u64;
pub(crate) type GLushort = u16;
pub(crate) type GLvdpauSurfaceNV = GLintptr;
//pub(crate) type GLvoid = void;
pub(crate) type void = core::ffi::c_void;

/// Note(Lokathor): Technically this should be `c_char` so that it's either `i8`
/// or `u8` depending on the OS. In practice the signed-ness doesn't matter.
/// Because Rust byte string literals (eg: `b"foo\0"`) are always `&[u8]`, we
/// just always use `u8`, so that we can do `b"foo\0".as_ptr()`.
pub(crate) type GLchar = u8;
