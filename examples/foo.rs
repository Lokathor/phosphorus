#![allow(bad_style)]
fn main() {}
type c_int = i32;
type c_uint = u32;
type c_uchar = u8;
type c_float = f32;
type c_double = f64;
type c_ushort = u16;

// Note(Lokathor): Alias this internally, but the rest of the world can just see
// `c_void`.
pub(crate) type void = core::ffi::c_void;

/// A GL enumeration value.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GlEnum(pub c_uint);
impl core::fmt::Debug for GlEnum {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "GlEnum(0x{:X})", self.0)
  }
}
impl Clone for GlEnum {
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for GlEnum {}

/// A GL bitfield value.
///
/// You can mix around the bits of these values using standard bitwise ops.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GlBitfield(pub c_uint);
impl core::fmt::Debug for GlBitfield {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "GlBitfield(0b{:b})", self.0)
  }
}
impl Clone for GlBitfield {
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for GlBitfield {}
impl core::ops::BitAnd for GlBitfield {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self::Output {
    Self(self.0 & rhs.0)
  }
}
impl core::ops::BitAndAssign for GlBitfield {
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}
impl core::ops::BitOr for GlBitfield {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
impl core::ops::BitOrAssign for GlBitfield {
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}
impl core::ops::BitXor for GlBitfield {
  type Output = Self;
  fn bitxor(self, rhs: Self) -> Self::Output {
    Self(self.0 ^ rhs.0)
  }
}
impl core::ops::BitXorAssign for GlBitfield {
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}
impl core::ops::Not for GlBitfield {
  type Output = Self;
  fn not(self) -> Self::Output {
    Self(!self.0)
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GLhandleARB(#[cfg(any(target_os = "ios", target_os = "macos"))] pub *mut void, #[cfg(not(any(target_os = "ios", target_os = "macos")))] pub c_uint);

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

pub type GLDEBUGPROC = Option<unsafe extern "system" fn(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const void)>;
pub type GLDEBUGPROCARB = GLDEBUGPROC;
pub type GLDEBUGPROCKHR = GLDEBUGPROC;
pub type GLDEBUGPROCAMD = Option<unsafe extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut void)>;
pub type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;

/// alias for `GlEnum` (rust style) to also be `GLenum` (GL style).
pub type GLenum = GlEnum;

/// alias for `GlBitfield` (rust style) to also be `GLbitfield` (GL style).
pub type GLbitfield = GlBitfield;

pub type GLboolean = c_uchar;
pub type GLvoid = core::ffi::c_void;
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint = c_int;
pub type GLuint = c_uint;
pub type GLclampx = i32;
pub type GLsizei = c_int;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLdouble = c_double;
pub type GLclampd = c_double;
pub type GLchar = u8;
pub type GLcharARB = u8;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLfixed = i32;
pub type GLintptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptr = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64 = i64;
pub type GLint64EXT = i64;
pub type GLuint64 = u64;
pub type GLuint64EXT = u64;
pub type GLhalfNV = c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

/// 
/// * Group: [`AttribMask`]
pub const GL_CURRENT_BIT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`AttribMask`]
pub const GL_POINT_BIT: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`AttribMask`]
pub const GL_LINE_BIT: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`AttribMask`]
pub const GL_POLYGON_BIT: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`AttribMask`]
pub const GL_POLYGON_STIPPLE_BIT: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`AttribMask`]
pub const GL_PIXEL_MODE_BIT: GlBitfield = GlBitfield(0x00000020);
/// 
/// * Group: [`AttribMask`]
pub const GL_LIGHTING_BIT: GlBitfield = GlBitfield(0x00000040);
/// 
/// * Group: [`AttribMask`]
pub const GL_FOG_BIT: GlBitfield = GlBitfield(0x00000080);
/// 
/// * Group: [`ClearBufferMask`], [`AttribMask`]
pub const GL_DEPTH_BUFFER_BIT: GlBitfield = GlBitfield(0x00000100);
/// 
/// * Group: [`ClearBufferMask`], [`AttribMask`]
pub const GL_ACCUM_BUFFER_BIT: GlBitfield = GlBitfield(0x00000200);
/// 
/// * Group: [`ClearBufferMask`], [`AttribMask`]
pub const GL_STENCIL_BUFFER_BIT: GlBitfield = GlBitfield(0x00000400);
/// 
/// * Group: [`AttribMask`]
pub const GL_VIEWPORT_BIT: GlBitfield = GlBitfield(0x00000800);
/// 
/// * Group: [`AttribMask`]
pub const GL_TRANSFORM_BIT: GlBitfield = GlBitfield(0x00001000);
/// 
/// * Group: [`AttribMask`]
pub const GL_ENABLE_BIT: GlBitfield = GlBitfield(0x00002000);
/// 
/// * Group: [`ClearBufferMask`], [`AttribMask`]
pub const GL_COLOR_BUFFER_BIT: GlBitfield = GlBitfield(0x00004000);
/// 
/// * Group: [`AttribMask`]
pub const GL_HINT_BIT: GlBitfield = GlBitfield(0x00008000);
/// 
/// * Group: [`AttribMask`]
pub const GL_EVAL_BIT: GlBitfield = GlBitfield(0x00010000);
/// 
/// * Group: [`AttribMask`]
pub const GL_LIST_BIT: GlBitfield = GlBitfield(0x00020000);
/// 
/// * Group: [`AttribMask`]
pub const GL_TEXTURE_BIT: GlBitfield = GlBitfield(0x00040000);
/// 
/// * Group: [`AttribMask`]
pub const GL_SCISSOR_BIT: GlBitfield = GlBitfield(0x00080000);
/// 
/// * Group: [`AttribMask`]
pub const GL_MULTISAMPLE_BIT: GlBitfield = GlBitfield(0x20000000);
/// 
/// * Group: [`AttribMask`]
pub const GL_MULTISAMPLE_BIT_ARB: GlBitfield = GlBitfield(0x20000000);
/// 
/// * Group: [`AttribMask`]
pub const GL_MULTISAMPLE_BIT_EXT: GlBitfield = GlBitfield(0x20000000);
/// 
/// * Group: [`AttribMask`]
pub const GL_MULTISAMPLE_BIT_3DFX: GlBitfield = GlBitfield(0x20000000);
/// Guaranteed to mark all attribute groups at once
/// * Group: [`AttribMask`]
pub const GL_ALL_ATTRIB_BITS: GlBitfield = GlBitfield(0xFFFFFFFF);
/// 
/// * Group: [`BufferStorageMask`]
pub const GL_DYNAMIC_STORAGE_BIT: GlBitfield = GlBitfield(0x0100);
/// 
/// * Group: [`BufferStorageMask`]
pub const GL_DYNAMIC_STORAGE_BIT_EXT: GlBitfield = GlBitfield(0x0100);
/// 
/// * Group: [`BufferStorageMask`]
pub const GL_CLIENT_STORAGE_BIT: GlBitfield = GlBitfield(0x0200);
/// 
/// * Group: [`BufferStorageMask`]
pub const GL_CLIENT_STORAGE_BIT_EXT: GlBitfield = GlBitfield(0x0200);
/// 
/// * Group: [`BufferStorageMask`]
pub const GL_SPARSE_STORAGE_BIT_ARB: GlBitfield = GlBitfield(0x0400);
/// 
/// * Group: [`BufferStorageMask`]
pub const GL_LGPU_SEPARATE_STORAGE_BIT_NVX: GlBitfield = GlBitfield(0x0800);
/// 
/// * Group: [`BufferStorageMask`]
pub const GL_PER_GPU_STORAGE_BIT_NV: GlBitfield = GlBitfield(0x0800);
/// 
/// * Group: [`BufferStorageMask`]
pub const GL_EXTERNAL_STORAGE_BIT_NVX: GlBitfield = GlBitfield(0x2000);
/// Collides with AttribMask bit GL_HINT_BIT. OK since this token is for OpenGL
/// ES 2, which doesn't have attribute groups.
/// * Group: [`ClearBufferMask`]
pub const GL_COVERAGE_BUFFER_BIT_NV: GlBitfield = GlBitfield(0x00008000);
/// 
/// * Group: [`ClientAttribMask`]
pub const GL_CLIENT_PIXEL_STORE_BIT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`ClientAttribMask`]
pub const GL_CLIENT_VERTEX_ARRAY_BIT: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`ClientAttribMask`]
pub const GL_CLIENT_ALL_ATTRIB_BITS: GlBitfield = GlBitfield(0xFFFFFFFF);
/// 
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_DEBUG_BIT_KHR: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT_ARB: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_NO_ERROR_BIT: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`ContextFlagMask`]
/// * Alias Of: [`GL_CONTEXT_FLAG_NO_ERROR_BIT`]
pub const GL_CONTEXT_FLAG_NO_ERROR_BIT_KHR: GlBitfield = GL_CONTEXT_FLAG_NO_ERROR_BIT;
/// 
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_PROTECTED_CONTENT_BIT_EXT: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`ContextProfileMask`]
pub const GL_CONTEXT_CORE_PROFILE_BIT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`ContextProfileMask`]
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_READ_BIT: GlBitfield = GlBitfield(0x0001);
/// 
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_READ_BIT_EXT: GlBitfield = GlBitfield(0x0001);
/// 
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_WRITE_BIT: GlBitfield = GlBitfield(0x0002);
/// 
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_WRITE_BIT_EXT: GlBitfield = GlBitfield(0x0002);
/// 
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_INVALIDATE_RANGE_BIT: GlBitfield = GlBitfield(0x0004);
/// 
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_INVALIDATE_RANGE_BIT_EXT: GlBitfield = GlBitfield(0x0004);
/// 
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_INVALIDATE_BUFFER_BIT: GlBitfield = GlBitfield(0x0008);
/// 
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_INVALIDATE_BUFFER_BIT_EXT: GlBitfield = GlBitfield(0x0008);
/// 
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_FLUSH_EXPLICIT_BIT: GlBitfield = GlBitfield(0x0010);
/// 
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_FLUSH_EXPLICIT_BIT_EXT: GlBitfield = GlBitfield(0x0010);
/// 
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_UNSYNCHRONIZED_BIT: GlBitfield = GlBitfield(0x0020);
/// 
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_UNSYNCHRONIZED_BIT_EXT: GlBitfield = GlBitfield(0x0020);
/// 
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_PERSISTENT_BIT: GlBitfield = GlBitfield(0x0040);
/// 
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_PERSISTENT_BIT_EXT: GlBitfield = GlBitfield(0x0040);
/// 
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_COHERENT_BIT: GlBitfield = GlBitfield(0x0080);
/// 
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_COHERENT_BIT_EXT: GlBitfield = GlBitfield(0x0080);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_ELEMENT_ARRAY_BARRIER_BIT: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_ELEMENT_ARRAY_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_UNIFORM_BARRIER_BIT: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_UNIFORM_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_TEXTURE_FETCH_BARRIER_BIT: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_TEXTURE_FETCH_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_SHADER_GLOBAL_ACCESS_BARRIER_BIT_NV: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GlBitfield = GlBitfield(0x00000020);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000020);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_COMMAND_BARRIER_BIT: GlBitfield = GlBitfield(0x00000040);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_COMMAND_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000040);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_PIXEL_BUFFER_BARRIER_BIT: GlBitfield = GlBitfield(0x00000080);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_PIXEL_BUFFER_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000080);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_TEXTURE_UPDATE_BARRIER_BIT: GlBitfield = GlBitfield(0x00000100);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_TEXTURE_UPDATE_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000100);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_BUFFER_UPDATE_BARRIER_BIT: GlBitfield = GlBitfield(0x00000200);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_BUFFER_UPDATE_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000200);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_FRAMEBUFFER_BARRIER_BIT: GlBitfield = GlBitfield(0x00000400);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_FRAMEBUFFER_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000400);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GlBitfield = GlBitfield(0x00000800);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00000800);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_ATOMIC_COUNTER_BARRIER_BIT: GlBitfield = GlBitfield(0x00001000);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_ATOMIC_COUNTER_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00001000);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_SHADER_STORAGE_BARRIER_BIT: GlBitfield = GlBitfield(0x00002000);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: GlBitfield = GlBitfield(0x00004000);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT_EXT: GlBitfield = GlBitfield(0x00004000);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_QUERY_BUFFER_BARRIER_BIT: GlBitfield = GlBitfield(0x00008000);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_ALL_BARRIER_BITS: GlBitfield = GlBitfield(0xFFFFFFFF);
/// 
/// * Group: [`MemoryBarrierMask`]
pub const GL_ALL_BARRIER_BITS_EXT: GlBitfield = GlBitfield(0xFFFFFFFF);
/// 
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_DEPTH_PASS_EVENT_BIT_AMD: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_DEPTH_FAIL_EVENT_BIT_AMD: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_STENCIL_FAIL_EVENT_BIT_AMD: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_DEPTH_BOUNDS_FAIL_EVENT_BIT_AMD: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_ALL_EVENT_BITS_AMD: GlBitfield = GlBitfield(0xFFFFFFFF);
/// 
/// * Group: [`SyncObjectMask`]
pub const GL_SYNC_FLUSH_COMMANDS_BIT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`SyncObjectMask`]
pub const GL_SYNC_FLUSH_COMMANDS_BIT_APPLE: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_VERTEX_SHADER_BIT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_VERTEX_SHADER_BIT_EXT: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_FRAGMENT_SHADER_BIT: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_FRAGMENT_SHADER_BIT_EXT: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_GEOMETRY_SHADER_BIT: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_GEOMETRY_SHADER_BIT_EXT: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_GEOMETRY_SHADER_BIT_OES: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_CONTROL_SHADER_BIT: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_CONTROL_SHADER_BIT_EXT: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_CONTROL_SHADER_BIT_OES: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_EVALUATION_SHADER_BIT: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_EVALUATION_SHADER_BIT_EXT: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_EVALUATION_SHADER_BIT_OES: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_COMPUTE_SHADER_BIT: GlBitfield = GlBitfield(0x00000020);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_MESH_SHADER_BIT_NV: GlBitfield = GlBitfield(0x00000040);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_TASK_SHADER_BIT_NV: GlBitfield = GlBitfield(0x00000080);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_ALL_SHADER_BITS: GlBitfield = GlBitfield(0xFFFFFFFF);
/// 
/// * Group: [`UseProgramStageMask`]
pub const GL_ALL_SHADER_BITS_EXT: GlBitfield = GlBitfield(0xFFFFFFFF);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_BASIC_BIT_KHR: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_VOTE_BIT_KHR: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_ARITHMETIC_BIT_KHR: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_BALLOT_BIT_KHR: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_SHUFFLE_BIT_KHR: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT_KHR: GlBitfield = GlBitfield(0x00000020);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_CLUSTERED_BIT_KHR: GlBitfield = GlBitfield(0x00000040);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_QUAD_BIT_KHR: GlBitfield = GlBitfield(0x00000080);
/// 
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_PARTITIONED_BIT_NV: GlBitfield = GlBitfield(0x00000100);
/// 
/// * Group: [`TextureStorageMaskAMD`]
pub const GL_TEXTURE_STORAGE_SPARSE_BIT_AMD: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`FragmentShaderDestMaskATI`]
pub const GL_RED_BIT_ATI: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`FragmentShaderDestMaskATI`]
pub const GL_GREEN_BIT_ATI: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`FragmentShaderDestMaskATI`]
pub const GL_BLUE_BIT_ATI: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_2X_BIT_ATI: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_4X_BIT_ATI: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_8X_BIT_ATI: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_HALF_BIT_ATI: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_QUARTER_BIT_ATI: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_EIGHTH_BIT_ATI: GlBitfield = GlBitfield(0x00000020);
/// 
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_SATURATE_BIT_ATI: GlBitfield = GlBitfield(0x00000040);
/// 
/// * Group: [`FragmentShaderColorModMaskATI`]
pub const GL_COMP_BIT_ATI: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`FragmentShaderColorModMaskATI`]
pub const GL_NEGATE_BIT_ATI: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`FragmentShaderColorModMaskATI`]
pub const GL_BIAS_BIT_ATI: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_OPERATIONS_BIT_MESA: GlBitfield = GlBitfield(0x0001);
/// 
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_PRIMITIVES_BIT_MESA: GlBitfield = GlBitfield(0x0002);
/// 
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_ARRAYS_BIT_MESA: GlBitfield = GlBitfield(0x0004);
/// 
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_TEXTURES_BIT_MESA: GlBitfield = GlBitfield(0x0008);
/// 
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_PIXELS_BIT_MESA: GlBitfield = GlBitfield(0x0010);
/// 
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_ERRORS_BIT_MESA: GlBitfield = GlBitfield(0x0020);
/// 
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_ALL_BITS_MESA: GlBitfield = GlBitfield(0xFFFF);
/// 
/// * Group: [`PathFontStyle`]
pub const GL_BOLD_BIT_NV: GlBitfield = GlBitfield(0x01);
/// 
/// * Group: [`PathFontStyle`]
pub const GL_ITALIC_BIT_NV: GlBitfield = GlBitfield(0x02);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_WIDTH_BIT_NV: GlBitfield = GlBitfield(0x01);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HEIGHT_BIT_NV: GlBitfield = GlBitfield(0x02);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HORIZONTAL_BEARING_X_BIT_NV: GlBitfield = GlBitfield(0x04);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HORIZONTAL_BEARING_Y_BIT_NV: GlBitfield = GlBitfield(0x08);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HORIZONTAL_BEARING_ADVANCE_BIT_NV: GlBitfield = GlBitfield(0x10);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_VERTICAL_BEARING_X_BIT_NV: GlBitfield = GlBitfield(0x20);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_VERTICAL_BEARING_Y_BIT_NV: GlBitfield = GlBitfield(0x40);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_VERTICAL_BEARING_ADVANCE_BIT_NV: GlBitfield = GlBitfield(0x80);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HAS_KERNING_BIT_NV: GlBitfield = GlBitfield(0x100);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_X_MIN_BOUNDS_BIT_NV: GlBitfield = GlBitfield(0x00010000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_Y_MIN_BOUNDS_BIT_NV: GlBitfield = GlBitfield(0x00020000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_X_MAX_BOUNDS_BIT_NV: GlBitfield = GlBitfield(0x00040000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_Y_MAX_BOUNDS_BIT_NV: GlBitfield = GlBitfield(0x00080000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_UNITS_PER_EM_BIT_NV: GlBitfield = GlBitfield(0x00100000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_ASCENDER_BIT_NV: GlBitfield = GlBitfield(0x00200000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_DESCENDER_BIT_NV: GlBitfield = GlBitfield(0x00400000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_HEIGHT_BIT_NV: GlBitfield = GlBitfield(0x00800000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_MAX_ADVANCE_WIDTH_BIT_NV: GlBitfield = GlBitfield(0x01000000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_MAX_ADVANCE_HEIGHT_BIT_NV: GlBitfield = GlBitfield(0x02000000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_UNDERLINE_POSITION_BIT_NV: GlBitfield = GlBitfield(0x04000000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_UNDERLINE_THICKNESS_BIT_NV: GlBitfield = GlBitfield(0x08000000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_HAS_KERNING_BIT_NV: GlBitfield = GlBitfield(0x10000000);
/// 
/// * Group: [`PathMetricMask`]
pub const GL_FONT_NUM_GLYPH_INDICES_BIT_NV: GlBitfield = GlBitfield(0x20000000);
/// 
/// * Group: [`PerformanceQueryCapsMaskINTEL`]
pub const GL_PERFQUERY_SINGLE_CONTEXT_INTEL: GlBitfield = GlBitfield(0x00000000);
/// 
/// * Group: [`PerformanceQueryCapsMaskINTEL`]
pub const GL_PERFQUERY_GLOBAL_CONTEXT_INTEL: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_VERTEX23_BIT_PGI: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_VERTEX4_BIT_PGI: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_COLOR3_BIT_PGI: GlBitfield = GlBitfield(0x00010000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_COLOR4_BIT_PGI: GlBitfield = GlBitfield(0x00020000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_EDGEFLAG_BIT_PGI: GlBitfield = GlBitfield(0x00040000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_INDEX_BIT_PGI: GlBitfield = GlBitfield(0x00080000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_AMBIENT_BIT_PGI: GlBitfield = GlBitfield(0x00100000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_AMBIENT_AND_DIFFUSE_BIT_PGI: GlBitfield = GlBitfield(0x00200000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_DIFFUSE_BIT_PGI: GlBitfield = GlBitfield(0x00400000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_EMISSION_BIT_PGI: GlBitfield = GlBitfield(0x00800000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_COLOR_INDEXES_BIT_PGI: GlBitfield = GlBitfield(0x01000000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_SHININESS_BIT_PGI: GlBitfield = GlBitfield(0x02000000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_SPECULAR_BIT_PGI: GlBitfield = GlBitfield(0x04000000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_NORMAL_BIT_PGI: GlBitfield = GlBitfield(0x08000000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_TEXCOORD1_BIT_PGI: GlBitfield = GlBitfield(0x10000000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_TEXCOORD2_BIT_PGI: GlBitfield = GlBitfield(0x20000000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_TEXCOORD3_BIT_PGI: GlBitfield = GlBitfield(0x40000000);
/// 
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_TEXCOORD4_BIT_PGI: GlBitfield = GlBitfield(0x80000000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT0_QCOM: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT1_QCOM: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT2_QCOM: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT3_QCOM: GlBitfield = GlBitfield(0x00000008);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT4_QCOM: GlBitfield = GlBitfield(0x00000010);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT5_QCOM: GlBitfield = GlBitfield(0x00000020);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT6_QCOM: GlBitfield = GlBitfield(0x00000040);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT7_QCOM: GlBitfield = GlBitfield(0x00000080);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT0_QCOM: GlBitfield = GlBitfield(0x00000100);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT1_QCOM: GlBitfield = GlBitfield(0x00000200);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT2_QCOM: GlBitfield = GlBitfield(0x00000400);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT3_QCOM: GlBitfield = GlBitfield(0x00000800);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT4_QCOM: GlBitfield = GlBitfield(0x00001000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT5_QCOM: GlBitfield = GlBitfield(0x00002000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT6_QCOM: GlBitfield = GlBitfield(0x00004000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT7_QCOM: GlBitfield = GlBitfield(0x00008000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT0_QCOM: GlBitfield = GlBitfield(0x00010000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT1_QCOM: GlBitfield = GlBitfield(0x00020000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT2_QCOM: GlBitfield = GlBitfield(0x00040000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT3_QCOM: GlBitfield = GlBitfield(0x00080000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT4_QCOM: GlBitfield = GlBitfield(0x00100000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT5_QCOM: GlBitfield = GlBitfield(0x00200000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT6_QCOM: GlBitfield = GlBitfield(0x00400000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT7_QCOM: GlBitfield = GlBitfield(0x00800000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT0_QCOM: GlBitfield = GlBitfield(0x01000000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT1_QCOM: GlBitfield = GlBitfield(0x02000000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT2_QCOM: GlBitfield = GlBitfield(0x04000000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT3_QCOM: GlBitfield = GlBitfield(0x08000000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT4_QCOM: GlBitfield = GlBitfield(0x10000000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT5_QCOM: GlBitfield = GlBitfield(0x20000000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT6_QCOM: GlBitfield = GlBitfield(0x40000000);
/// 
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT7_QCOM: GlBitfield = GlBitfield(0x80000000);
/// 
/// * Group: [`FoveationConfigBitQCOM`]
pub const GL_FOVEATION_ENABLE_BIT_QCOM: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`FoveationConfigBitQCOM`]
pub const GL_FOVEATION_SCALED_BIN_METHOD_BIT_QCOM: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`FoveationConfigBitQCOM`]
pub const GL_FOVEATION_SUBSAMPLED_LAYOUT_METHOD_BIT_QCOM: GlBitfield = GlBitfield(0x00000004);
/// 
/// * Group: [`FfdMaskSGIX`]
pub const GL_TEXTURE_DEFORMATION_BIT_SGIX: GlBitfield = GlBitfield(0x00000001);
/// 
/// * Group: [`FfdMaskSGIX`]
pub const GL_GEOMETRY_DEFORMATION_BIT_SGIX: GlBitfield = GlBitfield(0x00000002);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_TERMINATE_SEQUENCE_COMMAND_NV: GlEnum = GlEnum(0x0000);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_NOP_COMMAND_NV: GlEnum = GlEnum(0x0001);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ELEMENTS_COMMAND_NV: GlEnum = GlEnum(0x0002);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ARRAYS_COMMAND_NV: GlEnum = GlEnum(0x0003);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ELEMENTS_STRIP_COMMAND_NV: GlEnum = GlEnum(0x0004);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ARRAYS_STRIP_COMMAND_NV: GlEnum = GlEnum(0x0005);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ELEMENTS_INSTANCED_COMMAND_NV: GlEnum = GlEnum(0x0006);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ARRAYS_INSTANCED_COMMAND_NV: GlEnum = GlEnum(0x0007);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_ELEMENT_ADDRESS_COMMAND_NV: GlEnum = GlEnum(0x0008);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_ATTRIBUTE_ADDRESS_COMMAND_NV: GlEnum = GlEnum(0x0009);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_UNIFORM_ADDRESS_COMMAND_NV: GlEnum = GlEnum(0x000A);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_BLEND_COLOR_COMMAND_NV: GlEnum = GlEnum(0x000B);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_STENCIL_REF_COMMAND_NV: GlEnum = GlEnum(0x000C);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_LINE_WIDTH_COMMAND_NV: GlEnum = GlEnum(0x000D);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_POLYGON_OFFSET_COMMAND_NV: GlEnum = GlEnum(0x000E);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_ALPHA_REF_COMMAND_NV: GlEnum = GlEnum(0x000F);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_VIEWPORT_COMMAND_NV: GlEnum = GlEnum(0x0010);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_SCISSOR_COMMAND_NV: GlEnum = GlEnum(0x0011);
/// 
/// * Group: [`CommandOpcodesNV`]
pub const GL_FRONT_FACE_COMMAND_NV: GlEnum = GlEnum(0x0012);
/// 
/// * Group: [`MapTextureFormatINTEL`]
pub const GL_LAYOUT_DEFAULT_INTEL: GlEnum = GlEnum(0);
/// 
/// * Group: [`MapTextureFormatINTEL`]
pub const GL_LAYOUT_LINEAR_INTEL: GlEnum = GlEnum(1);
/// 
/// * Group: [`MapTextureFormatINTEL`]
pub const GL_LAYOUT_LINEAR_CPU_CACHED_INTEL: GlEnum = GlEnum(2);
/// 
/// * Group: [`PathCoordType`]
pub const GL_CLOSE_PATH_NV: GlEnum = GlEnum(0x00);
/// 
/// * Group: [`PathCoordType`]
pub const GL_MOVE_TO_NV: GlEnum = GlEnum(0x02);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_MOVE_TO_NV: GlEnum = GlEnum(0x03);
/// 
/// * Group: [`PathCoordType`]
pub const GL_LINE_TO_NV: GlEnum = GlEnum(0x04);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_LINE_TO_NV: GlEnum = GlEnum(0x05);
/// 
/// * Group: [`PathCoordType`]
pub const GL_HORIZONTAL_LINE_TO_NV: GlEnum = GlEnum(0x06);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_HORIZONTAL_LINE_TO_NV: GlEnum = GlEnum(0x07);
/// 
/// * Group: [`PathCoordType`]
pub const GL_VERTICAL_LINE_TO_NV: GlEnum = GlEnum(0x08);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_VERTICAL_LINE_TO_NV: GlEnum = GlEnum(0x09);
/// 
/// * Group: [`PathCoordType`]
pub const GL_QUADRATIC_CURVE_TO_NV: GlEnum = GlEnum(0x0A);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_QUADRATIC_CURVE_TO_NV: GlEnum = GlEnum(0x0B);
/// 
/// * Group: [`PathCoordType`]
pub const GL_CUBIC_CURVE_TO_NV: GlEnum = GlEnum(0x0C);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_CUBIC_CURVE_TO_NV: GlEnum = GlEnum(0x0D);
/// 
/// * Group: [`PathCoordType`]
pub const GL_SMOOTH_QUADRATIC_CURVE_TO_NV: GlEnum = GlEnum(0x0E);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_SMOOTH_QUADRATIC_CURVE_TO_NV: GlEnum = GlEnum(0x0F);
/// 
/// * Group: [`PathCoordType`]
pub const GL_SMOOTH_CUBIC_CURVE_TO_NV: GlEnum = GlEnum(0x10);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_SMOOTH_CUBIC_CURVE_TO_NV: GlEnum = GlEnum(0x11);
/// 
/// * Group: [`PathCoordType`]
pub const GL_SMALL_CCW_ARC_TO_NV: GlEnum = GlEnum(0x12);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_SMALL_CCW_ARC_TO_NV: GlEnum = GlEnum(0x13);
/// 
/// * Group: [`PathCoordType`]
pub const GL_SMALL_CW_ARC_TO_NV: GlEnum = GlEnum(0x14);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_SMALL_CW_ARC_TO_NV: GlEnum = GlEnum(0x15);
/// 
/// * Group: [`PathCoordType`]
pub const GL_LARGE_CCW_ARC_TO_NV: GlEnum = GlEnum(0x16);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_LARGE_CCW_ARC_TO_NV: GlEnum = GlEnum(0x17);
/// 
/// * Group: [`PathCoordType`]
pub const GL_LARGE_CW_ARC_TO_NV: GlEnum = GlEnum(0x18);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_LARGE_CW_ARC_TO_NV: GlEnum = GlEnum(0x19);
/// 
/// * Group: [`PathCoordType`]
pub const GL_CONIC_CURVE_TO_NV: GlEnum = GlEnum(0x1A);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_CONIC_CURVE_TO_NV: GlEnum = GlEnum(0x1B);
pub const GL_SHARED_EDGE_NV: GlEnum = GlEnum(0xC0);
/// 
/// * Group: [`PathCoordType`]
pub const GL_ROUNDED_RECT_NV: GlEnum = GlEnum(0xE8);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ROUNDED_RECT_NV: GlEnum = GlEnum(0xE9);
/// 
/// * Group: [`PathCoordType`]
pub const GL_ROUNDED_RECT2_NV: GlEnum = GlEnum(0xEA);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ROUNDED_RECT2_NV: GlEnum = GlEnum(0xEB);
/// 
/// * Group: [`PathCoordType`]
pub const GL_ROUNDED_RECT4_NV: GlEnum = GlEnum(0xEC);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ROUNDED_RECT4_NV: GlEnum = GlEnum(0xED);
/// 
/// * Group: [`PathCoordType`]
pub const GL_ROUNDED_RECT8_NV: GlEnum = GlEnum(0xEE);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ROUNDED_RECT8_NV: GlEnum = GlEnum(0xEF);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RESTART_PATH_NV: GlEnum = GlEnum(0xF0);
/// 
/// * Group: [`PathCoordType`]
pub const GL_DUP_FIRST_CUBIC_CURVE_TO_NV: GlEnum = GlEnum(0xF2);
/// 
/// * Group: [`PathCoordType`]
pub const GL_DUP_LAST_CUBIC_CURVE_TO_NV: GlEnum = GlEnum(0xF4);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RECT_NV: GlEnum = GlEnum(0xF6);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_RECT_NV: GlEnum = GlEnum(0xF7);
/// 
/// * Group: [`PathCoordType`]
pub const GL_CIRCULAR_CCW_ARC_TO_NV: GlEnum = GlEnum(0xF8);
/// 
/// * Group: [`PathCoordType`]
pub const GL_CIRCULAR_CW_ARC_TO_NV: GlEnum = GlEnum(0xFA);
/// 
/// * Group: [`PathCoordType`]
pub const GL_CIRCULAR_TANGENT_ARC_TO_NV: GlEnum = GlEnum(0xFC);
/// 
/// * Group: [`PathCoordType`]
pub const GL_ARC_TO_NV: GlEnum = GlEnum(0xFE);
/// 
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ARC_TO_NV: GlEnum = GlEnum(0xFF);
/// 
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_NEXT_BUFFER_NV: GlEnum = GlEnum(u32::MAX - 2);
/// 
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_SKIP_COMPONENTS4_NV: GlEnum = GlEnum(u32::MAX - 3);
/// 
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_SKIP_COMPONENTS3_NV: GlEnum = GlEnum(u32::MAX - 4);
/// 
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_SKIP_COMPONENTS2_NV: GlEnum = GlEnum(u32::MAX - 5);
/// 
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_SKIP_COMPONENTS1_NV: GlEnum = GlEnum(u32::MAX - 6);
/// 
/// * Group: [`TriangleListSUN`]
pub const GL_RESTART_SUN: GlEnum = GlEnum(0x0001);
/// 
/// * Group: [`TriangleListSUN`]
pub const GL_REPLACE_MIDDLE_SUN: GlEnum = GlEnum(0x0002);
/// 
/// * Group: [`TriangleListSUN`]
pub const GL_REPLACE_OLDEST_SUN: GlEnum = GlEnum(0x0003);
/// 
/// * Group: [`Boolean`], [`VertexShaderWriteMaskEXT`], [`ClampColorModeARB`]
pub const GL_FALSE: GlEnum = GlEnum(0);
/// 
/// * Group: [`GraphicsResetStatus`], [`ErrorCode`]
pub const GL_NO_ERROR: GlEnum = GlEnum(0);
/// 
/// * Group: [`TextureSwizzle`], [`StencilOp`], [`BlendingFactor`]
pub const GL_ZERO: GlEnum = GlEnum(0);
/// 
/// * Group: [`SyncBehaviorFlags`], [`TextureCompareMode`], [`PathColorFormat`],
///   [`CombinerBiasNV`], [`CombinerScaleNV`], [`DrawBufferMode`],
///   [`PixelTexGenMode`], [`ReadBufferMode`], [`ColorBuffer`], [`PathGenMode`],
///   [`PathTransformType`], [`PathFontStyle`]
pub const GL_NONE: GlEnum = GlEnum(0);
/// 
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_NONE_OES: GlEnum = GlEnum(0);
/// 
/// * Group: [`Boolean`], [`VertexShaderWriteMaskEXT`], [`ClampColorModeARB`]
pub const GL_TRUE: GlEnum = GlEnum(1);
/// 
/// * Group: [`TextureSwizzle`], [`BlendingFactor`]
pub const GL_ONE: GlEnum = GlEnum(1);
/// Tagged as uint
pub const GL_INVALID_INDEX: c_uint = 0xFFFFFFFF;
pub const GL_ALL_PIXELS_AMD: GlEnum = GlEnum(0xFFFFFFFF);
/// Tagged as uint64
pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
/// Tagged as uint64
pub const GL_TIMEOUT_IGNORED_APPLE: u64 = 0xFFFFFFFFFFFFFFFF;
/// Not an API enum. API definition macro for ES 1.0/1.1 headers
pub const GL_VERSION_ES_CL_1_0: GlEnum = GlEnum(1);
/// Not an API enum. API definition macro for ES 1.0/1.1 headers
pub const GL_VERSION_ES_CM_1_1: GlEnum = GlEnum(1);
/// Not an API enum. API definition macro for ES 1.0/1.1 headers
pub const GL_VERSION_ES_CL_1_1: GlEnum = GlEnum(1);
pub const GL_UUID_SIZE_EXT: GlEnum = GlEnum(16);
pub const GL_LUID_SIZE_EXT: GlEnum = GlEnum(8);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_POINTS: GlEnum = GlEnum(0x0000);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINES: GlEnum = GlEnum(0x0001);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINE_LOOP: GlEnum = GlEnum(0x0002);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINE_STRIP: GlEnum = GlEnum(0x0003);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLES: GlEnum = GlEnum(0x0004);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_STRIP: GlEnum = GlEnum(0x0005);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_FAN: GlEnum = GlEnum(0x0006);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_QUADS: GlEnum = GlEnum(0x0007);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_QUADS_EXT: GlEnum = GlEnum(0x0007);
pub const GL_QUADS_OES: GlEnum = GlEnum(0x0007);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_QUAD_STRIP: GlEnum = GlEnum(0x0008);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_POLYGON: GlEnum = GlEnum(0x0009);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINES_ADJACENCY: GlEnum = GlEnum(0x000A);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINES_ADJACENCY_ARB: GlEnum = GlEnum(0x000A);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINES_ADJACENCY_EXT: GlEnum = GlEnum(0x000A);
pub const GL_LINES_ADJACENCY_OES: GlEnum = GlEnum(0x000A);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINE_STRIP_ADJACENCY: GlEnum = GlEnum(0x000B);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINE_STRIP_ADJACENCY_ARB: GlEnum = GlEnum(0x000B);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_LINE_STRIP_ADJACENCY_EXT: GlEnum = GlEnum(0x000B);
pub const GL_LINE_STRIP_ADJACENCY_OES: GlEnum = GlEnum(0x000B);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLES_ADJACENCY: GlEnum = GlEnum(0x000C);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLES_ADJACENCY_ARB: GlEnum = GlEnum(0x000C);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLES_ADJACENCY_EXT: GlEnum = GlEnum(0x000C);
pub const GL_TRIANGLES_ADJACENCY_OES: GlEnum = GlEnum(0x000C);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_STRIP_ADJACENCY: GlEnum = GlEnum(0x000D);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_STRIP_ADJACENCY_ARB: GlEnum = GlEnum(0x000D);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_STRIP_ADJACENCY_EXT: GlEnum = GlEnum(0x000D);
pub const GL_TRIANGLE_STRIP_ADJACENCY_OES: GlEnum = GlEnum(0x000D);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_PATCHES: GlEnum = GlEnum(0x000E);
/// 
/// * Group: [`PrimitiveType`]
pub const GL_PATCHES_EXT: GlEnum = GlEnum(0x000E);
pub const GL_PATCHES_OES: GlEnum = GlEnum(0x000E);
/// 
/// * Group: [`AccumOp`]
pub const GL_ACCUM: GlEnum = GlEnum(0x0100);
/// 
/// * Group: [`AccumOp`]
pub const GL_LOAD: GlEnum = GlEnum(0x0101);
/// 
/// * Group: [`AccumOp`]
pub const GL_RETURN: GlEnum = GlEnum(0x0102);
/// 
/// * Group: [`AccumOp`]
pub const GL_MULT: GlEnum = GlEnum(0x0103);
/// 
/// * Group: [`TextureEnvMode`], [`AccumOp`], [`LightEnvModeSGIX`]
pub const GL_ADD: GlEnum = GlEnum(0x0104);
/// 
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_NEVER: GlEnum = GlEnum(0x0200);
/// 
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_LESS: GlEnum = GlEnum(0x0201);
/// 
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_EQUAL: GlEnum = GlEnum(0x0202);
/// 
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_LEQUAL: GlEnum = GlEnum(0x0203);
/// 
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_GREATER: GlEnum = GlEnum(0x0204);
/// 
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_NOTEQUAL: GlEnum = GlEnum(0x0205);
/// 
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_GEQUAL: GlEnum = GlEnum(0x0206);
/// 
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_ALWAYS: GlEnum = GlEnum(0x0207);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_SRC_COLOR: GlEnum = GlEnum(0x0300);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_SRC_COLOR: GlEnum = GlEnum(0x0301);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_SRC_ALPHA: GlEnum = GlEnum(0x0302);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_SRC_ALPHA: GlEnum = GlEnum(0x0303);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_DST_ALPHA: GlEnum = GlEnum(0x0304);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_DST_ALPHA: GlEnum = GlEnum(0x0305);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_DST_COLOR: GlEnum = GlEnum(0x0306);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_DST_COLOR: GlEnum = GlEnum(0x0307);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_SRC_ALPHA_SATURATE: GlEnum = GlEnum(0x0308);
pub const GL_SRC_ALPHA_SATURATE_EXT: GlEnum = GlEnum(0x0308);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_FRONT_LEFT: GlEnum = GlEnum(0x0400);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_FRONT_RIGHT: GlEnum = GlEnum(0x0401);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_BACK_LEFT: GlEnum = GlEnum(0x0402);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_BACK_RIGHT: GlEnum = GlEnum(0x0403);
/// 
/// * Group: [`ColorBuffer`], [`ColorMaterialFace`], [`CullFaceMode`],
///   [`DrawBufferMode`], [`ReadBufferMode`], [`StencilFaceDirection`],
///   [`MaterialFace`]
pub const GL_FRONT: GlEnum = GlEnum(0x0404);
/// 
/// * Group: [`ColorBuffer`], [`ColorMaterialFace`], [`CullFaceMode`],
///   [`DrawBufferMode`], [`ReadBufferMode`], [`StencilFaceDirection`],
///   [`MaterialFace`]
pub const GL_BACK: GlEnum = GlEnum(0x0405);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_LEFT: GlEnum = GlEnum(0x0406);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_RIGHT: GlEnum = GlEnum(0x0407);
/// 
/// * Group: [`ColorBuffer`], [`ColorMaterialFace`], [`CullFaceMode`],
///   [`DrawBufferMode`], [`StencilFaceDirection`], [`MaterialFace`]
pub const GL_FRONT_AND_BACK: GlEnum = GlEnum(0x0408);
/// 
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_AUX0: GlEnum = GlEnum(0x0409);
/// 
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_AUX1: GlEnum = GlEnum(0x040A);
/// 
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_AUX2: GlEnum = GlEnum(0x040B);
/// 
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_AUX3: GlEnum = GlEnum(0x040C);
/// 
/// * Group: [`ErrorCode`]
pub const GL_INVALID_ENUM: GlEnum = GlEnum(0x0500);
/// 
/// * Group: [`ErrorCode`]
pub const GL_INVALID_VALUE: GlEnum = GlEnum(0x0501);
/// 
/// * Group: [`ErrorCode`]
pub const GL_INVALID_OPERATION: GlEnum = GlEnum(0x0502);
/// 
/// * Group: [`ErrorCode`]
pub const GL_STACK_OVERFLOW: GlEnum = GlEnum(0x0503);
pub const GL_STACK_OVERFLOW_KHR: GlEnum = GlEnum(0x0503);
/// 
/// * Group: [`ErrorCode`]
pub const GL_STACK_UNDERFLOW: GlEnum = GlEnum(0x0504);
pub const GL_STACK_UNDERFLOW_KHR: GlEnum = GlEnum(0x0504);
/// 
/// * Group: [`ErrorCode`]
pub const GL_OUT_OF_MEMORY: GlEnum = GlEnum(0x0505);
/// 
/// * Group: [`ErrorCode`]
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GlEnum = GlEnum(0x0506);
/// 
/// * Group: [`ErrorCode`]
pub const GL_INVALID_FRAMEBUFFER_OPERATION_EXT: GlEnum = GlEnum(0x0506);
/// 
/// * Group: [`ErrorCode`]
pub const GL_INVALID_FRAMEBUFFER_OPERATION_OES: GlEnum = GlEnum(0x0506);
pub const GL_CONTEXT_LOST: GlEnum = GlEnum(0x0507);
pub const GL_CONTEXT_LOST_KHR: GlEnum = GlEnum(0x0507);
/// 
/// * Group: [`FeedbackType`]
pub const GL_2D: GlEnum = GlEnum(0x0600);
/// 
/// * Group: [`FeedbackType`]
pub const GL_3D: GlEnum = GlEnum(0x0601);
/// 
/// * Group: [`FeedbackType`]
pub const GL_3D_COLOR: GlEnum = GlEnum(0x0602);
/// 
/// * Group: [`FeedbackType`]
pub const GL_3D_COLOR_TEXTURE: GlEnum = GlEnum(0x0603);
/// 
/// * Group: [`FeedbackType`]
pub const GL_4D_COLOR_TEXTURE: GlEnum = GlEnum(0x0604);
/// 
/// * Group: [`FeedBackToken`]
pub const GL_PASS_THROUGH_TOKEN: GlEnum = GlEnum(0x0700);
/// 
/// * Group: [`FeedBackToken`]
pub const GL_POINT_TOKEN: GlEnum = GlEnum(0x0701);
/// 
/// * Group: [`FeedBackToken`]
pub const GL_LINE_TOKEN: GlEnum = GlEnum(0x0702);
/// 
/// * Group: [`FeedBackToken`]
pub const GL_POLYGON_TOKEN: GlEnum = GlEnum(0x0703);
/// 
/// * Group: [`FeedBackToken`]
pub const GL_BITMAP_TOKEN: GlEnum = GlEnum(0x0704);
/// 
/// * Group: [`FeedBackToken`]
pub const GL_DRAW_PIXEL_TOKEN: GlEnum = GlEnum(0x0705);
/// 
/// * Group: [`FeedBackToken`]
pub const GL_COPY_PIXEL_TOKEN: GlEnum = GlEnum(0x0706);
/// 
/// * Group: [`FeedBackToken`]
pub const GL_LINE_RESET_TOKEN: GlEnum = GlEnum(0x0707);
/// 
/// * Group: [`FogMode`]
pub const GL_EXP: GlEnum = GlEnum(0x0800);
/// 
/// * Group: [`FogMode`]
pub const GL_EXP2: GlEnum = GlEnum(0x0801);
/// 
/// * Group: [`FrontFaceDirection`]
pub const GL_CW: GlEnum = GlEnum(0x0900);
/// 
/// * Group: [`FrontFaceDirection`]
pub const GL_CCW: GlEnum = GlEnum(0x0901);
/// 
/// * Group: [`MapQuery`], [`GetMapQuery`]
pub const GL_COEFF: GlEnum = GlEnum(0x0A00);
/// 
/// * Group: [`MapQuery`], [`GetMapQuery`]
pub const GL_ORDER: GlEnum = GlEnum(0x0A01);
/// 
/// * Group: [`MapQuery`], [`GetMapQuery`]
pub const GL_DOMAIN: GlEnum = GlEnum(0x0A02);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_COLOR: GlEnum = GlEnum(0x0B00);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_INDEX: GlEnum = GlEnum(0x0B01);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_NORMAL: GlEnum = GlEnum(0x0B02);
/// 
/// * Group: [`GetPName`], [`VertexShaderTextureUnitParameter`]
pub const GL_CURRENT_TEXTURE_COORDS: GlEnum = GlEnum(0x0B03);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_COLOR: GlEnum = GlEnum(0x0B04);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_INDEX: GlEnum = GlEnum(0x0B05);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_TEXTURE_COORDS: GlEnum = GlEnum(0x0B06);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_POSITION: GlEnum = GlEnum(0x0B07);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_POSITION_VALID: GlEnum = GlEnum(0x0B08);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_DISTANCE: GlEnum = GlEnum(0x0B09);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POINT_SMOOTH: GlEnum = GlEnum(0x0B10);
/// 
/// * Group: [`GetPName`]
pub const GL_POINT_SIZE: GlEnum = GlEnum(0x0B11);
/// 
/// * Group: [`GetPName`]
pub const GL_POINT_SIZE_RANGE: GlEnum = GlEnum(0x0B12);
/// 
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_POINT_SIZE_RANGE`]
pub const GL_SMOOTH_POINT_SIZE_RANGE: GlEnum = GL_POINT_SIZE_RANGE;
/// 
/// * Group: [`GetPName`]
pub const GL_POINT_SIZE_GRANULARITY: GlEnum = GlEnum(0x0B13);
/// 
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_POINT_SIZE_GRANULARITY`]
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: GlEnum = GL_POINT_SIZE_GRANULARITY;
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_LINE_SMOOTH: GlEnum = GlEnum(0x0B20);
/// 
/// * Group: [`GetPName`]
pub const GL_LINE_WIDTH: GlEnum = GlEnum(0x0B21);
/// 
/// * Group: [`GetPName`]
pub const GL_LINE_WIDTH_RANGE: GlEnum = GlEnum(0x0B22);
/// 
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_LINE_WIDTH_RANGE`]
pub const GL_SMOOTH_LINE_WIDTH_RANGE: GlEnum = GL_LINE_WIDTH_RANGE;
/// 
/// * Group: [`GetPName`]
pub const GL_LINE_WIDTH_GRANULARITY: GlEnum = GlEnum(0x0B23);
/// 
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_LINE_WIDTH_GRANULARITY`]
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: GlEnum = GL_LINE_WIDTH_GRANULARITY;
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_LINE_STIPPLE: GlEnum = GlEnum(0x0B24);
/// 
/// * Group: [`GetPName`]
pub const GL_LINE_STIPPLE_PATTERN: GlEnum = GlEnum(0x0B25);
/// 
/// * Group: [`GetPName`]
pub const GL_LINE_STIPPLE_REPEAT: GlEnum = GlEnum(0x0B26);
/// 
/// * Group: [`GetPName`]
pub const GL_LIST_MODE: GlEnum = GlEnum(0x0B30);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_LIST_NESTING: GlEnum = GlEnum(0x0B31);
/// 
/// * Group: [`GetPName`]
pub const GL_LIST_BASE: GlEnum = GlEnum(0x0B32);
/// 
/// * Group: [`GetPName`]
pub const GL_LIST_INDEX: GlEnum = GlEnum(0x0B33);
/// 
/// * Group: [`GetPName`]
pub const GL_POLYGON_MODE: GlEnum = GlEnum(0x0B40);
pub const GL_POLYGON_MODE_NV: GlEnum = GlEnum(0x0B40);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_SMOOTH: GlEnum = GlEnum(0x0B41);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_STIPPLE: GlEnum = GlEnum(0x0B42);
/// 
/// * Group: [`GetPName`]
pub const GL_EDGE_FLAG: GlEnum = GlEnum(0x0B43);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_CULL_FACE: GlEnum = GlEnum(0x0B44);
/// 
/// * Group: [`GetPName`]
pub const GL_CULL_FACE_MODE: GlEnum = GlEnum(0x0B45);
/// 
/// * Group: [`GetPName`]
pub const GL_FRONT_FACE: GlEnum = GlEnum(0x0B46);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_LIGHTING: GlEnum = GlEnum(0x0B50);
/// 
/// * Group: [`LightModelParameter`], [`GetPName`]
pub const GL_LIGHT_MODEL_LOCAL_VIEWER: GlEnum = GlEnum(0x0B51);
/// 
/// * Group: [`LightModelParameter`], [`GetPName`]
pub const GL_LIGHT_MODEL_TWO_SIDE: GlEnum = GlEnum(0x0B52);
/// 
/// * Group: [`LightModelParameter`], [`GetPName`]
pub const GL_LIGHT_MODEL_AMBIENT: GlEnum = GlEnum(0x0B53);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADE_MODEL: GlEnum = GlEnum(0x0B54);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_MATERIAL_FACE: GlEnum = GlEnum(0x0B55);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_MATERIAL_PARAMETER: GlEnum = GlEnum(0x0B56);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_COLOR_MATERIAL: GlEnum = GlEnum(0x0B57);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FOG: GlEnum = GlEnum(0x0B60);
/// 
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_INDEX: GlEnum = GlEnum(0x0B61);
/// 
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_DENSITY: GlEnum = GlEnum(0x0B62);
/// 
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_START: GlEnum = GlEnum(0x0B63);
/// 
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_END: GlEnum = GlEnum(0x0B64);
/// 
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_MODE: GlEnum = GlEnum(0x0B65);
/// 
/// * Group: [`GetPName`], [`FogParameter`]
pub const GL_FOG_COLOR: GlEnum = GlEnum(0x0B66);
/// 
/// * Group: [`GetPName`]
pub const GL_DEPTH_RANGE: GlEnum = GlEnum(0x0B70);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_DEPTH_TEST: GlEnum = GlEnum(0x0B71);
/// 
/// * Group: [`GetPName`]
pub const GL_DEPTH_WRITEMASK: GlEnum = GlEnum(0x0B72);
/// 
/// * Group: [`GetPName`]
pub const GL_DEPTH_CLEAR_VALUE: GlEnum = GlEnum(0x0B73);
/// 
/// * Group: [`GetPName`]
pub const GL_DEPTH_FUNC: GlEnum = GlEnum(0x0B74);
/// 
/// * Group: [`GetPName`]
pub const GL_ACCUM_CLEAR_VALUE: GlEnum = GlEnum(0x0B80);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_STENCIL_TEST: GlEnum = GlEnum(0x0B90);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_CLEAR_VALUE: GlEnum = GlEnum(0x0B91);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_FUNC: GlEnum = GlEnum(0x0B92);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_VALUE_MASK: GlEnum = GlEnum(0x0B93);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_FAIL: GlEnum = GlEnum(0x0B94);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_PASS_DEPTH_FAIL: GlEnum = GlEnum(0x0B95);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_PASS_DEPTH_PASS: GlEnum = GlEnum(0x0B96);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_REF: GlEnum = GlEnum(0x0B97);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_WRITEMASK: GlEnum = GlEnum(0x0B98);
/// 
/// * Group: [`GetPName`]
pub const GL_MATRIX_MODE: GlEnum = GlEnum(0x0BA0);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_NORMALIZE: GlEnum = GlEnum(0x0BA1);
/// 
/// * Group: [`GetPName`]
pub const GL_VIEWPORT: GlEnum = GlEnum(0x0BA2);
/// 
/// * Group: [`GetPName`]
pub const GL_MODELVIEW_STACK_DEPTH: GlEnum = GlEnum(0x0BA3);
/// 
/// * Group: [`GetPName`]
pub const GL_MODELVIEW0_STACK_DEPTH_EXT: GlEnum = GlEnum(0x0BA3);
pub const GL_PATH_MODELVIEW_STACK_DEPTH_NV: GlEnum = GlEnum(0x0BA3);
/// 
/// * Group: [`GetPName`]
pub const GL_PROJECTION_STACK_DEPTH: GlEnum = GlEnum(0x0BA4);
pub const GL_PATH_PROJECTION_STACK_DEPTH_NV: GlEnum = GlEnum(0x0BA4);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_STACK_DEPTH: GlEnum = GlEnum(0x0BA5);
/// 
/// * Group: [`GetPName`]
pub const GL_MODELVIEW_MATRIX: GlEnum = GlEnum(0x0BA6);
/// 
/// * Group: [`GetPName`]
pub const GL_MODELVIEW0_MATRIX_EXT: GlEnum = GlEnum(0x0BA6);
pub const GL_PATH_MODELVIEW_MATRIX_NV: GlEnum = GlEnum(0x0BA6);
/// 
/// * Group: [`GetPName`]
pub const GL_PROJECTION_MATRIX: GlEnum = GlEnum(0x0BA7);
pub const GL_PATH_PROJECTION_MATRIX_NV: GlEnum = GlEnum(0x0BA7);
/// 
/// * Group: [`GetPName`], [`VertexShaderTextureUnitParameter`]
pub const GL_TEXTURE_MATRIX: GlEnum = GlEnum(0x0BA8);
/// 
/// * Group: [`GetPName`]
pub const GL_ATTRIB_STACK_DEPTH: GlEnum = GlEnum(0x0BB0);
/// 
/// * Group: [`GetPName`]
pub const GL_CLIENT_ATTRIB_STACK_DEPTH: GlEnum = GlEnum(0x0BB1);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ALPHA_TEST: GlEnum = GlEnum(0x0BC0);
/// 
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_QCOM: GlEnum = GlEnum(0x0BC0);
/// 
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_FUNC: GlEnum = GlEnum(0x0BC1);
/// 
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_FUNC_QCOM: GlEnum = GlEnum(0x0BC1);
/// 
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_REF: GlEnum = GlEnum(0x0BC2);
/// 
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_REF_QCOM: GlEnum = GlEnum(0x0BC2);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_DITHER: GlEnum = GlEnum(0x0BD0);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_DST: GlEnum = GlEnum(0x0BE0);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_SRC: GlEnum = GlEnum(0x0BE1);
/// 
/// * Group: [`TextureEnvMode`], [`EnableCap`], [`GetPName`]
pub const GL_BLEND: GlEnum = GlEnum(0x0BE2);
/// 
/// * Group: [`GetPName`]
pub const GL_LOGIC_OP_MODE: GlEnum = GlEnum(0x0BF0);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_INDEX_LOGIC_OP: GlEnum = GlEnum(0x0BF1);
/// 
/// * Group: [`GetPName`]
pub const GL_LOGIC_OP: GlEnum = GlEnum(0x0BF1);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_COLOR_LOGIC_OP: GlEnum = GlEnum(0x0BF2);
/// 
/// * Group: [`GetPName`]
pub const GL_AUX_BUFFERS: GlEnum = GlEnum(0x0C00);
/// 
/// * Group: [`GetPName`]
pub const GL_DRAW_BUFFER: GlEnum = GlEnum(0x0C01);
/// 
/// * Group: [`GetPName`]
pub const GL_DRAW_BUFFER_EXT: GlEnum = GlEnum(0x0C01);
/// 
/// * Group: [`GetPName`]
pub const GL_READ_BUFFER: GlEnum = GlEnum(0x0C02);
/// 
/// * Group: [`GetPName`]
pub const GL_READ_BUFFER_EXT: GlEnum = GlEnum(0x0C02);
/// 
/// * Group: [`GetPName`]
pub const GL_READ_BUFFER_NV: GlEnum = GlEnum(0x0C02);
/// 
/// * Group: [`GetPName`]
pub const GL_SCISSOR_BOX: GlEnum = GlEnum(0x0C10);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SCISSOR_TEST: GlEnum = GlEnum(0x0C11);
/// 
/// * Group: [`GetPName`]
pub const GL_INDEX_CLEAR_VALUE: GlEnum = GlEnum(0x0C20);
/// 
/// * Group: [`GetPName`]
pub const GL_INDEX_WRITEMASK: GlEnum = GlEnum(0x0C21);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_CLEAR_VALUE: GlEnum = GlEnum(0x0C22);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_WRITEMASK: GlEnum = GlEnum(0x0C23);
/// 
/// * Group: [`GetPName`]
pub const GL_INDEX_MODE: GlEnum = GlEnum(0x0C30);
/// 
/// * Group: [`GetPName`]
pub const GL_RGBA_MODE: GlEnum = GlEnum(0x0C31);
/// 
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_DOUBLEBUFFER: GlEnum = GlEnum(0x0C32);
/// 
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_STEREO: GlEnum = GlEnum(0x0C33);
/// 
/// * Group: [`GetPName`]
pub const GL_RENDER_MODE: GlEnum = GlEnum(0x0C40);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_PERSPECTIVE_CORRECTION_HINT: GlEnum = GlEnum(0x0C50);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_POINT_SMOOTH_HINT: GlEnum = GlEnum(0x0C51);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_LINE_SMOOTH_HINT: GlEnum = GlEnum(0x0C52);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_POLYGON_SMOOTH_HINT: GlEnum = GlEnum(0x0C53);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_FOG_HINT: GlEnum = GlEnum(0x0C54);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_GEN_S: GlEnum = GlEnum(0x0C60);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_GEN_T: GlEnum = GlEnum(0x0C61);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_GEN_R: GlEnum = GlEnum(0x0C62);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_GEN_Q: GlEnum = GlEnum(0x0C63);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_I: GlEnum = GlEnum(0x0C70);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_S_TO_S: GlEnum = GlEnum(0x0C71);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_R: GlEnum = GlEnum(0x0C72);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_G: GlEnum = GlEnum(0x0C73);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_B: GlEnum = GlEnum(0x0C74);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_A: GlEnum = GlEnum(0x0C75);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_R_TO_R: GlEnum = GlEnum(0x0C76);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_G_TO_G: GlEnum = GlEnum(0x0C77);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_B_TO_B: GlEnum = GlEnum(0x0C78);
/// 
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_A_TO_A: GlEnum = GlEnum(0x0C79);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_I_SIZE: GlEnum = GlEnum(0x0CB0);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_S_TO_S_SIZE: GlEnum = GlEnum(0x0CB1);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_R_SIZE: GlEnum = GlEnum(0x0CB2);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_G_SIZE: GlEnum = GlEnum(0x0CB3);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_B_SIZE: GlEnum = GlEnum(0x0CB4);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_A_SIZE: GlEnum = GlEnum(0x0CB5);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_R_TO_R_SIZE: GlEnum = GlEnum(0x0CB6);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_G_TO_G_SIZE: GlEnum = GlEnum(0x0CB7);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_B_TO_B_SIZE: GlEnum = GlEnum(0x0CB8);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_A_TO_A_SIZE: GlEnum = GlEnum(0x0CB9);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SWAP_BYTES: GlEnum = GlEnum(0x0CF0);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_LSB_FIRST: GlEnum = GlEnum(0x0CF1);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_ROW_LENGTH: GlEnum = GlEnum(0x0CF2);
/// 
/// * Group: [`PixelStoreParameter`]
pub const GL_UNPACK_ROW_LENGTH_EXT: GlEnum = GlEnum(0x0CF2);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_ROWS: GlEnum = GlEnum(0x0CF3);
/// 
/// * Group: [`PixelStoreParameter`]
pub const GL_UNPACK_SKIP_ROWS_EXT: GlEnum = GlEnum(0x0CF3);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_PIXELS: GlEnum = GlEnum(0x0CF4);
/// 
/// * Group: [`PixelStoreParameter`]
pub const GL_UNPACK_SKIP_PIXELS_EXT: GlEnum = GlEnum(0x0CF4);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_ALIGNMENT: GlEnum = GlEnum(0x0CF5);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SWAP_BYTES: GlEnum = GlEnum(0x0D00);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_LSB_FIRST: GlEnum = GlEnum(0x0D01);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_ROW_LENGTH: GlEnum = GlEnum(0x0D02);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_ROWS: GlEnum = GlEnum(0x0D03);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_PIXELS: GlEnum = GlEnum(0x0D04);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_ALIGNMENT: GlEnum = GlEnum(0x0D05);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_MAP_COLOR: GlEnum = GlEnum(0x0D10);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_MAP_STENCIL: GlEnum = GlEnum(0x0D11);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_INDEX_SHIFT: GlEnum = GlEnum(0x0D12);
/// 
/// * Group: [`PixelTransferParameter`], [`IndexMaterialParameterEXT`],
///   [`GetPName`]
pub const GL_INDEX_OFFSET: GlEnum = GlEnum(0x0D13);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_RED_SCALE: GlEnum = GlEnum(0x0D14);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_RED_BIAS: GlEnum = GlEnum(0x0D15);
/// 
/// * Group: [`GetPName`]
pub const GL_ZOOM_X: GlEnum = GlEnum(0x0D16);
/// 
/// * Group: [`GetPName`]
pub const GL_ZOOM_Y: GlEnum = GlEnum(0x0D17);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_GREEN_SCALE: GlEnum = GlEnum(0x0D18);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_GREEN_BIAS: GlEnum = GlEnum(0x0D19);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_BLUE_SCALE: GlEnum = GlEnum(0x0D1A);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_BLUE_BIAS: GlEnum = GlEnum(0x0D1B);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_ALPHA_SCALE: GlEnum = GlEnum(0x0D1C);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_ALPHA_BIAS: GlEnum = GlEnum(0x0D1D);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_DEPTH_SCALE: GlEnum = GlEnum(0x0D1E);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_DEPTH_BIAS: GlEnum = GlEnum(0x0D1F);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_EVAL_ORDER: GlEnum = GlEnum(0x0D30);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_LIGHTS: GlEnum = GlEnum(0x0D31);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_CLIP_PLANES: GlEnum = GlEnum(0x0D32);
pub const GL_MAX_CLIP_PLANES_IMG: GlEnum = GlEnum(0x0D32);
/// 
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_MAX_CLIP_PLANES`]
pub const GL_MAX_CLIP_DISTANCES: GlEnum = GL_MAX_CLIP_PLANES;
/// 
/// * Alias Of: [`GL_MAX_CLIP_PLANES`]
pub const GL_MAX_CLIP_DISTANCES_EXT: GlEnum = GL_MAX_CLIP_PLANES;
pub const GL_MAX_CLIP_DISTANCES_APPLE: GlEnum = GlEnum(0x0D32);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_SIZE: GlEnum = GlEnum(0x0D33);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_PIXEL_MAP_TABLE: GlEnum = GlEnum(0x0D34);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ATTRIB_STACK_DEPTH: GlEnum = GlEnum(0x0D35);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_MODELVIEW_STACK_DEPTH: GlEnum = GlEnum(0x0D36);
pub const GL_PATH_MAX_MODELVIEW_STACK_DEPTH_NV: GlEnum = GlEnum(0x0D36);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_NAME_STACK_DEPTH: GlEnum = GlEnum(0x0D37);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_PROJECTION_STACK_DEPTH: GlEnum = GlEnum(0x0D38);
pub const GL_PATH_MAX_PROJECTION_STACK_DEPTH_NV: GlEnum = GlEnum(0x0D38);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_STACK_DEPTH: GlEnum = GlEnum(0x0D39);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VIEWPORT_DIMS: GlEnum = GlEnum(0x0D3A);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: GlEnum = GlEnum(0x0D3B);
/// 
/// * Group: [`GetPName`]
pub const GL_SUBPIXEL_BITS: GlEnum = GlEnum(0x0D50);
/// 
/// * Group: [`GetPName`]
pub const GL_INDEX_BITS: GlEnum = GlEnum(0x0D51);
/// 
/// * Group: [`GetPName`]
pub const GL_RED_BITS: GlEnum = GlEnum(0x0D52);
/// 
/// * Group: [`GetPName`]
pub const GL_GREEN_BITS: GlEnum = GlEnum(0x0D53);
/// 
/// * Group: [`GetPName`]
pub const GL_BLUE_BITS: GlEnum = GlEnum(0x0D54);
/// 
/// * Group: [`GetPName`]
pub const GL_ALPHA_BITS: GlEnum = GlEnum(0x0D55);
/// 
/// * Group: [`GetPName`]
pub const GL_DEPTH_BITS: GlEnum = GlEnum(0x0D56);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_BITS: GlEnum = GlEnum(0x0D57);
/// 
/// * Group: [`GetPName`]
pub const GL_ACCUM_RED_BITS: GlEnum = GlEnum(0x0D58);
/// 
/// * Group: [`GetPName`]
pub const GL_ACCUM_GREEN_BITS: GlEnum = GlEnum(0x0D59);
/// 
/// * Group: [`GetPName`]
pub const GL_ACCUM_BLUE_BITS: GlEnum = GlEnum(0x0D5A);
/// 
/// * Group: [`GetPName`]
pub const GL_ACCUM_ALPHA_BITS: GlEnum = GlEnum(0x0D5B);
/// 
/// * Group: [`GetPName`]
pub const GL_NAME_STACK_DEPTH: GlEnum = GlEnum(0x0D70);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_AUTO_NORMAL: GlEnum = GlEnum(0x0D80);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_COLOR_4: GlEnum = GlEnum(0x0D90);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_INDEX: GlEnum = GlEnum(0x0D91);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_NORMAL: GlEnum = GlEnum(0x0D92);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_TEXTURE_COORD_1: GlEnum = GlEnum(0x0D93);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_TEXTURE_COORD_2: GlEnum = GlEnum(0x0D94);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_TEXTURE_COORD_3: GlEnum = GlEnum(0x0D95);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_TEXTURE_COORD_4: GlEnum = GlEnum(0x0D96);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_VERTEX_3: GlEnum = GlEnum(0x0D97);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_VERTEX_4: GlEnum = GlEnum(0x0D98);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_COLOR_4: GlEnum = GlEnum(0x0DB0);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_INDEX: GlEnum = GlEnum(0x0DB1);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_NORMAL: GlEnum = GlEnum(0x0DB2);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_TEXTURE_COORD_1: GlEnum = GlEnum(0x0DB3);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_TEXTURE_COORD_2: GlEnum = GlEnum(0x0DB4);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_TEXTURE_COORD_3: GlEnum = GlEnum(0x0DB5);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_TEXTURE_COORD_4: GlEnum = GlEnum(0x0DB6);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_VERTEX_3: GlEnum = GlEnum(0x0DB7);
/// 
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_VERTEX_4: GlEnum = GlEnum(0x0DB8);
/// 
/// * Group: [`GetPName`]
pub const GL_MAP1_GRID_DOMAIN: GlEnum = GlEnum(0x0DD0);
/// 
/// * Group: [`GetPName`]
pub const GL_MAP1_GRID_SEGMENTS: GlEnum = GlEnum(0x0DD1);
/// 
/// * Group: [`GetPName`]
pub const GL_MAP2_GRID_DOMAIN: GlEnum = GlEnum(0x0DD2);
/// 
/// * Group: [`GetPName`]
pub const GL_MAP2_GRID_SEGMENTS: GlEnum = GlEnum(0x0DD3);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`EnableCap`], [`GetPName`],
///   [`TextureTarget`]
pub const GL_TEXTURE_1D: GlEnum = GlEnum(0x0DE0);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`EnableCap`], [`GetPName`],
///   [`TextureTarget`]
pub const GL_TEXTURE_2D: GlEnum = GlEnum(0x0DE1);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_FEEDBACK_BUFFER_POINTER: GlEnum = GlEnum(0x0DF0);
/// 
/// * Group: [`GetPName`]
pub const GL_FEEDBACK_BUFFER_SIZE: GlEnum = GlEnum(0x0DF1);
/// 
/// * Group: [`GetPName`]
pub const GL_FEEDBACK_BUFFER_TYPE: GlEnum = GlEnum(0x0DF2);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_SELECTION_BUFFER_POINTER: GlEnum = GlEnum(0x0DF3);
/// 
/// * Group: [`GetPName`]
pub const GL_SELECTION_BUFFER_SIZE: GlEnum = GlEnum(0x0DF4);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_WIDTH: GlEnum = GlEnum(0x1000);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_HEIGHT: GlEnum = GlEnum(0x1001);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_INTERNAL_FORMAT: GlEnum = GlEnum(0x1003);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_COMPONENTS: GlEnum = GlEnum(0x1003);
/// 
/// * Group: [`SamplerParameterF`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_BORDER_COLOR: GlEnum = GlEnum(0x1004);
pub const GL_TEXTURE_BORDER_COLOR_EXT: GlEnum = GlEnum(0x1004);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_BORDER_COLOR_NV: GlEnum = GlEnum(0x1004);
pub const GL_TEXTURE_BORDER_COLOR_OES: GlEnum = GlEnum(0x1004);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_BORDER: GlEnum = GlEnum(0x1005);
pub const GL_TEXTURE_TARGET: GlEnum = GlEnum(0x1006);
/// 
/// * Group: [`DebugSeverity`], [`HintMode`], [`DebugSource`], [`DebugType`]
pub const GL_DONT_CARE: GlEnum = GlEnum(0x1100);
/// 
/// * Group: [`HintMode`]
pub const GL_FASTEST: GlEnum = GlEnum(0x1101);
/// 
/// * Group: [`HintMode`]
pub const GL_NICEST: GlEnum = GlEnum(0x1102);
/// 
/// * Group: [`MaterialParameter`], [`FragmentLightParameterSGIX`],
///   [`ColorMaterialParameter`]
pub const GL_AMBIENT: GlEnum = GlEnum(0x1200);
/// 
/// * Group: [`MaterialParameter`], [`FragmentLightParameterSGIX`],
///   [`ColorMaterialParameter`]
pub const GL_DIFFUSE: GlEnum = GlEnum(0x1201);
/// 
/// * Group: [`MaterialParameter`], [`FragmentLightParameterSGIX`],
///   [`ColorMaterialParameter`]
pub const GL_SPECULAR: GlEnum = GlEnum(0x1202);
/// 
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_POSITION: GlEnum = GlEnum(0x1203);
/// 
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_SPOT_DIRECTION: GlEnum = GlEnum(0x1204);
/// 
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_SPOT_EXPONENT: GlEnum = GlEnum(0x1205);
/// 
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_SPOT_CUTOFF: GlEnum = GlEnum(0x1206);
/// 
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_CONSTANT_ATTENUATION: GlEnum = GlEnum(0x1207);
/// 
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_LINEAR_ATTENUATION: GlEnum = GlEnum(0x1208);
/// 
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_QUADRATIC_ATTENUATION: GlEnum = GlEnum(0x1209);
/// 
/// * Group: [`ListMode`]
pub const GL_COMPILE: GlEnum = GlEnum(0x1300);
/// 
/// * Group: [`ListMode`]
pub const GL_COMPILE_AND_EXECUTE: GlEnum = GlEnum(0x1301);
/// 
/// * Group: [`VertexAttribIType`], [`WeightPointerTypeARB`],
///   [`TangentPointerTypeEXT`], [`BinormalPointerTypeEXT`],
///   [`ColorPointerType`], [`ListNameType`], [`NormalPointerType`],
///   [`PixelType`], [`VertexAttribType`], [`VertexAttribPointerType`]
pub const GL_BYTE: GlEnum = GlEnum(0x1400);
/// 
/// * Group: [`VertexAttribIType`], [`ScalarType`], [`ReplacementCodeTypeSUN`],
///   [`ElementPointerTypeATI`], [`MatrixIndexPointerTypeARB`],
///   [`WeightPointerTypeARB`], [`ColorPointerType`], [`DrawElementsType`],
///   [`ListNameType`], [`PixelType`], [`VertexAttribType`],
///   [`VertexAttribPointerType`]
pub const GL_UNSIGNED_BYTE: GlEnum = GlEnum(0x1401);
/// 
/// * Group: [`VertexAttribIType`], [`SecondaryColorPointerTypeIBM`],
///   [`WeightPointerTypeARB`], [`TangentPointerTypeEXT`],
///   [`BinormalPointerTypeEXT`], [`IndexPointerType`], [`ListNameType`],
///   [`NormalPointerType`], [`PixelType`], [`TexCoordPointerType`],
///   [`VertexPointerType`], [`VertexAttribType`], [`VertexAttribPointerType`]
pub const GL_SHORT: GlEnum = GlEnum(0x1402);
/// 
/// * Group: [`VertexAttribIType`], [`ScalarType`], [`ReplacementCodeTypeSUN`],
///   [`ElementPointerTypeATI`], [`MatrixIndexPointerTypeARB`],
///   [`WeightPointerTypeARB`], [`ColorPointerType`], [`DrawElementsType`],
///   [`ListNameType`], [`PixelFormat`], [`PixelType`], [`VertexAttribType`],
///   [`VertexAttribPointerType`]
pub const GL_UNSIGNED_SHORT: GlEnum = GlEnum(0x1403);
/// 
/// * Group: [`VertexAttribIType`], [`SecondaryColorPointerTypeIBM`],
///   [`WeightPointerTypeARB`], [`TangentPointerTypeEXT`],
///   [`BinormalPointerTypeEXT`], [`IndexPointerType`], [`ListNameType`],
///   [`NormalPointerType`], [`PixelType`], [`TexCoordPointerType`],
///   [`VertexPointerType`], [`VertexAttribType`], [`AttributeType`],
///   [`UniformType`], [`VertexAttribPointerType`], [`GlslTypeToken`]
pub const GL_INT: GlEnum = GlEnum(0x1404);
/// 
/// * Group: [`VertexAttribIType`], [`ScalarType`], [`ReplacementCodeTypeSUN`],
///   [`ElementPointerTypeATI`], [`MatrixIndexPointerTypeARB`],
///   [`WeightPointerTypeARB`], [`ColorPointerType`], [`DrawElementsType`],
///   [`ListNameType`], [`PixelFormat`], [`PixelType`], [`VertexAttribType`],
///   [`AttributeType`], [`UniformType`], [`VertexAttribPointerType`],
///   [`GlslTypeToken`]
pub const GL_UNSIGNED_INT: GlEnum = GlEnum(0x1405);
/// 
/// * Group: [`GlslTypeToken`], [`MapTypeNV`], [`SecondaryColorPointerTypeIBM`],
///   [`WeightPointerTypeARB`], [`VertexWeightPointerTypeEXT`],
///   [`TangentPointerTypeEXT`], [`BinormalPointerTypeEXT`],
///   [`FogCoordinatePointerType`], [`FogPointerTypeEXT`],
///   [`FogPointerTypeIBM`], [`IndexPointerType`], [`ListNameType`],
///   [`NormalPointerType`], [`PixelType`], [`TexCoordPointerType`],
///   [`VertexPointerType`], [`VertexAttribType`], [`AttributeType`],
///   [`UniformType`], [`VertexAttribPointerType`]
pub const GL_FLOAT: GlEnum = GlEnum(0x1406);
/// 
/// * Group: [`ListNameType`]
pub const GL_2_BYTES: GlEnum = GlEnum(0x1407);
pub const GL_2_BYTES_NV: GlEnum = GlEnum(0x1407);
/// 
/// * Group: [`ListNameType`]
pub const GL_3_BYTES: GlEnum = GlEnum(0x1408);
pub const GL_3_BYTES_NV: GlEnum = GlEnum(0x1408);
/// 
/// * Group: [`ListNameType`]
pub const GL_4_BYTES: GlEnum = GlEnum(0x1409);
pub const GL_4_BYTES_NV: GlEnum = GlEnum(0x1409);
/// 
/// * Group: [`VertexAttribLType`], [`MapTypeNV`],
///   [`SecondaryColorPointerTypeIBM`], [`WeightPointerTypeARB`],
///   [`TangentPointerTypeEXT`], [`BinormalPointerTypeEXT`],
///   [`FogCoordinatePointerType`], [`FogPointerTypeEXT`],
///   [`FogPointerTypeIBM`], [`IndexPointerType`], [`NormalPointerType`],
///   [`TexCoordPointerType`], [`VertexPointerType`], [`VertexAttribType`],
///   [`AttributeType`], [`UniformType`], [`VertexAttribPointerType`],
///   [`GlslTypeToken`]
pub const GL_DOUBLE: GlEnum = GlEnum(0x140A);
/// 
/// * Group: [`BinormalPointerTypeEXT`], [`TangentPointerTypeEXT`]
pub const GL_DOUBLE_EXT: GlEnum = GlEnum(0x140A);
/// 
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_HALF_FLOAT: GlEnum = GlEnum(0x140B);
pub const GL_HALF_FLOAT_ARB: GlEnum = GlEnum(0x140B);
pub const GL_HALF_FLOAT_NV: GlEnum = GlEnum(0x140B);
pub const GL_HALF_APPLE: GlEnum = GlEnum(0x140B);
/// 
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_FIXED: GlEnum = GlEnum(0x140C);
pub const GL_FIXED_OES: GlEnum = GlEnum(0x140C);
/// 
/// * Group: [`VertexAttribPointerType`], [`AttributeType`]
pub const GL_INT64_ARB: GlEnum = GlEnum(0x140E);
/// 
/// * Group: [`VertexAttribPointerType`], [`AttributeType`]
pub const GL_INT64_NV: GlEnum = GlEnum(0x140E);
/// 
/// * Group: [`VertexAttribPointerType`], [`AttributeType`]
pub const GL_UNSIGNED_INT64_ARB: GlEnum = GlEnum(0x140F);
/// 
/// * Group: [`VertexAttribPointerType`], [`AttributeType`]
pub const GL_UNSIGNED_INT64_NV: GlEnum = GlEnum(0x140F);
/// 
/// * Group: [`LogicOp`]
pub const GL_CLEAR: GlEnum = GlEnum(0x1500);
/// 
/// * Group: [`LogicOp`]
pub const GL_AND: GlEnum = GlEnum(0x1501);
/// 
/// * Group: [`LogicOp`]
pub const GL_AND_REVERSE: GlEnum = GlEnum(0x1502);
/// 
/// * Group: [`LogicOp`]
pub const GL_COPY: GlEnum = GlEnum(0x1503);
/// 
/// * Group: [`LogicOp`]
pub const GL_AND_INVERTED: GlEnum = GlEnum(0x1504);
/// 
/// * Group: [`LogicOp`]
pub const GL_NOOP: GlEnum = GlEnum(0x1505);
/// 
/// * Group: [`LogicOp`]
pub const GL_XOR: GlEnum = GlEnum(0x1506);
pub const GL_XOR_NV: GlEnum = GlEnum(0x1506);
/// 
/// * Group: [`LogicOp`]
pub const GL_OR: GlEnum = GlEnum(0x1507);
/// 
/// * Group: [`LogicOp`]
pub const GL_NOR: GlEnum = GlEnum(0x1508);
/// 
/// * Group: [`LogicOp`]
pub const GL_EQUIV: GlEnum = GlEnum(0x1509);
/// 
/// * Group: [`PathFillMode`], [`LogicOp`], [`StencilOp`]
pub const GL_INVERT: GlEnum = GlEnum(0x150A);
/// 
/// * Group: [`LogicOp`]
pub const GL_OR_REVERSE: GlEnum = GlEnum(0x150B);
/// 
/// * Group: [`LogicOp`]
pub const GL_COPY_INVERTED: GlEnum = GlEnum(0x150C);
/// 
/// * Group: [`LogicOp`]
pub const GL_OR_INVERTED: GlEnum = GlEnum(0x150D);
/// 
/// * Group: [`LogicOp`]
pub const GL_NAND: GlEnum = GlEnum(0x150E);
/// 
/// * Group: [`LogicOp`]
pub const GL_SET: GlEnum = GlEnum(0x150F);
/// 
/// * Group: [`MaterialParameter`], [`ColorMaterialParameter`]
pub const GL_EMISSION: GlEnum = GlEnum(0x1600);
/// 
/// * Group: [`MaterialParameter`]
pub const GL_SHININESS: GlEnum = GlEnum(0x1601);
/// 
/// * Group: [`MaterialParameter`], [`ColorMaterialParameter`]
pub const GL_AMBIENT_AND_DIFFUSE: GlEnum = GlEnum(0x1602);
/// 
/// * Group: [`MaterialParameter`]
pub const GL_COLOR_INDEXES: GlEnum = GlEnum(0x1603);
/// 
/// * Group: [`MatrixMode`]
pub const GL_MODELVIEW: GlEnum = GlEnum(0x1700);
pub const GL_MODELVIEW0_ARB: GlEnum = GlEnum(0x1700);
/// 
/// * Group: [`MatrixMode`]
pub const GL_MODELVIEW0_EXT: GlEnum = GlEnum(0x1700);
pub const GL_PATH_MODELVIEW_NV: GlEnum = GlEnum(0x1700);
/// 
/// * Group: [`MatrixMode`]
pub const GL_PROJECTION: GlEnum = GlEnum(0x1701);
pub const GL_PATH_PROJECTION_NV: GlEnum = GlEnum(0x1701);
/// 
/// * Group: [`ObjectIdentifier`], [`MatrixMode`]
pub const GL_TEXTURE: GlEnum = GlEnum(0x1702);
/// 
/// * Group: [`Buffer`], [`PixelCopyType`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR: GlEnum = GlEnum(0x1800);
/// 
/// * Group: [`PixelCopyType`]
pub const GL_COLOR_EXT: GlEnum = GlEnum(0x1800);
/// 
/// * Group: [`Buffer`], [`PixelCopyType`], [`InvalidateFramebufferAttachment`]
pub const GL_DEPTH: GlEnum = GlEnum(0x1801);
/// 
/// * Group: [`PixelCopyType`]
pub const GL_DEPTH_EXT: GlEnum = GlEnum(0x1801);
/// 
/// * Group: [`Buffer`], [`PixelCopyType`], [`InvalidateFramebufferAttachment`]
pub const GL_STENCIL: GlEnum = GlEnum(0x1802);
/// 
/// * Group: [`PixelCopyType`]
pub const GL_STENCIL_EXT: GlEnum = GlEnum(0x1802);
/// 
/// * Group: [`PixelFormat`]
pub const GL_COLOR_INDEX: GlEnum = GlEnum(0x1900);
/// 
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_STENCIL_INDEX: GlEnum = GlEnum(0x1901);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX_OES: GlEnum = GlEnum(0x1901);
/// 
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_DEPTH_COMPONENT: GlEnum = GlEnum(0x1902);
/// 
/// * Group: [`TextureSwizzle`], [`PixelFormat`], [`InternalFormat`]
pub const GL_RED: GlEnum = GlEnum(0x1903);
/// 
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_RED_EXT: GlEnum = GlEnum(0x1903);
pub const GL_RED_NV: GlEnum = GlEnum(0x1903);
/// 
/// * Group: [`TextureSwizzle`], [`PixelFormat`]
pub const GL_GREEN: GlEnum = GlEnum(0x1904);
pub const GL_GREEN_NV: GlEnum = GlEnum(0x1904);
/// 
/// * Group: [`TextureSwizzle`], [`CombinerComponentUsageNV`], [`PixelFormat`]
pub const GL_BLUE: GlEnum = GlEnum(0x1905);
pub const GL_BLUE_NV: GlEnum = GlEnum(0x1905);
/// 
/// * Group: [`TextureSwizzle`], [`CombinerPortionNV`], [`PathColorFormat`],
///   [`CombinerComponentUsageNV`], [`PixelFormat`]
pub const GL_ALPHA: GlEnum = GlEnum(0x1906);
/// 
/// * Group: [`PixelTexGenMode`], [`CombinerPortionNV`], [`PathColorFormat`],
///   [`CombinerComponentUsageNV`], [`PixelFormat`], [`InternalFormat`]
pub const GL_RGB: GlEnum = GlEnum(0x1907);
/// 
/// * Group: [`PixelTexGenMode`], [`PathColorFormat`], [`PixelFormat`],
///   [`InternalFormat`]
pub const GL_RGBA: GlEnum = GlEnum(0x1908);
/// 
/// * Group: [`PixelTexGenMode`], [`PathColorFormat`], [`PixelFormat`]
pub const GL_LUMINANCE: GlEnum = GlEnum(0x1909);
/// 
/// * Group: [`PixelTexGenMode`], [`PathColorFormat`], [`PixelFormat`]
pub const GL_LUMINANCE_ALPHA: GlEnum = GlEnum(0x190A);
/// 
/// * Group: [`PixelType`]
pub const GL_BITMAP: GlEnum = GlEnum(0x1A00);
/// 
/// * Group: [`PolygonMode`], [`MeshMode1`], [`MeshMode2`]
pub const GL_POINT: GlEnum = GlEnum(0x1B00);
pub const GL_POINT_NV: GlEnum = GlEnum(0x1B00);
/// 
/// * Group: [`PolygonMode`], [`MeshMode1`], [`MeshMode2`]
pub const GL_LINE: GlEnum = GlEnum(0x1B01);
pub const GL_LINE_NV: GlEnum = GlEnum(0x1B01);
/// 
/// * Group: [`PolygonMode`], [`MeshMode2`]
pub const GL_FILL: GlEnum = GlEnum(0x1B02);
/// 
/// * Group: [`EvalMapsModeNV`]
pub const GL_FILL_NV: GlEnum = GlEnum(0x1B02);
/// 
/// * Group: [`RenderingMode`]
pub const GL_RENDER: GlEnum = GlEnum(0x1C00);
/// 
/// * Group: [`RenderingMode`]
pub const GL_FEEDBACK: GlEnum = GlEnum(0x1C01);
/// 
/// * Group: [`RenderingMode`]
pub const GL_SELECT: GlEnum = GlEnum(0x1C02);
/// 
/// * Group: [`ShadingModel`]
pub const GL_FLAT: GlEnum = GlEnum(0x1D00);
/// 
/// * Group: [`ShadingModel`]
pub const GL_SMOOTH: GlEnum = GlEnum(0x1D01);
/// 
/// * Group: [`StencilOp`]
pub const GL_KEEP: GlEnum = GlEnum(0x1E00);
/// 
/// * Group: [`StencilOp`], [`LightEnvModeSGIX`]
pub const GL_REPLACE: GlEnum = GlEnum(0x1E01);
/// 
/// * Group: [`StencilOp`]
pub const GL_INCR: GlEnum = GlEnum(0x1E02);
/// 
/// * Group: [`StencilOp`]
pub const GL_DECR: GlEnum = GlEnum(0x1E03);
/// 
/// * Group: [`StringName`]
pub const GL_VENDOR: GlEnum = GlEnum(0x1F00);
/// 
/// * Group: [`StringName`]
pub const GL_RENDERER: GlEnum = GlEnum(0x1F01);
/// 
/// * Group: [`StringName`]
pub const GL_VERSION: GlEnum = GlEnum(0x1F02);
/// 
/// * Group: [`StringName`]
pub const GL_EXTENSIONS: GlEnum = GlEnum(0x1F03);
/// 
/// * Group: [`TextureCoordName`]
pub const GL_S: GlEnum = GlEnum(0x2000);
/// 
/// * Group: [`TextureCoordName`]
pub const GL_T: GlEnum = GlEnum(0x2001);
/// 
/// * Group: [`TextureCoordName`]
pub const GL_R: GlEnum = GlEnum(0x2002);
/// 
/// * Group: [`TextureCoordName`]
pub const GL_Q: GlEnum = GlEnum(0x2003);
/// 
/// * Group: [`TextureEnvMode`], [`LightEnvModeSGIX`]
pub const GL_MODULATE: GlEnum = GlEnum(0x2100);
/// 
/// * Group: [`TextureEnvMode`]
pub const GL_DECAL: GlEnum = GlEnum(0x2101);
/// 
/// * Group: [`TextureEnvParameter`]
pub const GL_TEXTURE_ENV_MODE: GlEnum = GlEnum(0x2200);
/// 
/// * Group: [`TextureEnvParameter`]
pub const GL_TEXTURE_ENV_COLOR: GlEnum = GlEnum(0x2201);
/// 
/// * Group: [`TextureEnvTarget`]
pub const GL_TEXTURE_ENV: GlEnum = GlEnum(0x2300);
/// 
/// * Group: [`PathGenMode`], [`TextureGenMode`]
pub const GL_EYE_LINEAR: GlEnum = GlEnum(0x2400);
pub const GL_EYE_LINEAR_NV: GlEnum = GlEnum(0x2400);
/// 
/// * Group: [`PathGenMode`], [`TextureGenMode`]
pub const GL_OBJECT_LINEAR: GlEnum = GlEnum(0x2401);
pub const GL_OBJECT_LINEAR_NV: GlEnum = GlEnum(0x2401);
/// 
/// * Group: [`TextureGenMode`]
pub const GL_SPHERE_MAP: GlEnum = GlEnum(0x2402);
/// 
/// * Group: [`TextureGenParameter`]
pub const GL_TEXTURE_GEN_MODE: GlEnum = GlEnum(0x2500);
pub const GL_TEXTURE_GEN_MODE_OES: GlEnum = GlEnum(0x2500);
/// 
/// * Group: [`TextureGenParameter`]
pub const GL_OBJECT_PLANE: GlEnum = GlEnum(0x2501);
/// 
/// * Group: [`TextureGenParameter`]
pub const GL_EYE_PLANE: GlEnum = GlEnum(0x2502);
/// 
/// * Group: [`BlitFramebufferFilter`], [`TextureMagFilter`],
///   [`TextureMinFilter`]
pub const GL_NEAREST: GlEnum = GlEnum(0x2600);
/// 
/// * Group: [`BlitFramebufferFilter`], [`FogMode`], [`TextureMagFilter`],
///   [`TextureMinFilter`]
pub const GL_LINEAR: GlEnum = GlEnum(0x2601);
/// 
/// * Group: [`TextureMinFilter`]
pub const GL_NEAREST_MIPMAP_NEAREST: GlEnum = GlEnum(0x2700);
/// 
/// * Group: [`TextureMinFilter`]
pub const GL_LINEAR_MIPMAP_NEAREST: GlEnum = GlEnum(0x2701);
/// 
/// * Group: [`TextureMinFilter`]
pub const GL_NEAREST_MIPMAP_LINEAR: GlEnum = GlEnum(0x2702);
/// 
/// * Group: [`TextureWrapMode`], [`TextureMinFilter`]
pub const GL_LINEAR_MIPMAP_LINEAR: GlEnum = GlEnum(0x2703);
/// 
/// * Group: [`SamplerParameterI`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_MAG_FILTER: GlEnum = GlEnum(0x2800);
/// 
/// * Group: [`SamplerParameterI`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_MIN_FILTER: GlEnum = GlEnum(0x2801);
/// 
/// * Group: [`SamplerParameterI`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_WRAP_S: GlEnum = GlEnum(0x2802);
/// 
/// * Group: [`SamplerParameterI`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_WRAP_T: GlEnum = GlEnum(0x2803);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP: GlEnum = GlEnum(0x2900);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_REPEAT: GlEnum = GlEnum(0x2901);
/// 
/// * Group: [`GetPName`]
pub const GL_POLYGON_OFFSET_UNITS: GlEnum = GlEnum(0x2A00);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_OFFSET_POINT: GlEnum = GlEnum(0x2A01);
pub const GL_POLYGON_OFFSET_POINT_NV: GlEnum = GlEnum(0x2A01);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_OFFSET_LINE: GlEnum = GlEnum(0x2A02);
pub const GL_POLYGON_OFFSET_LINE_NV: GlEnum = GlEnum(0x2A02);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R3_G3_B2: GlEnum = GlEnum(0x2A10);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_V2F: GlEnum = GlEnum(0x2A20);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_V3F: GlEnum = GlEnum(0x2A21);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_C4UB_V2F: GlEnum = GlEnum(0x2A22);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_C4UB_V3F: GlEnum = GlEnum(0x2A23);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_C3F_V3F: GlEnum = GlEnum(0x2A24);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_N3F_V3F: GlEnum = GlEnum(0x2A25);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_C4F_N3F_V3F: GlEnum = GlEnum(0x2A26);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_V3F: GlEnum = GlEnum(0x2A27);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T4F_V4F: GlEnum = GlEnum(0x2A28);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_C4UB_V3F: GlEnum = GlEnum(0x2A29);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_C3F_V3F: GlEnum = GlEnum(0x2A2A);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_N3F_V3F: GlEnum = GlEnum(0x2A2B);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_C4F_N3F_V3F: GlEnum = GlEnum(0x2A2C);
/// 
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T4F_C4F_N3F_V4F: GlEnum = GlEnum(0x2A2D);
/// 
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE0: GlEnum = GlEnum(0x3000);
pub const GL_CLIP_PLANE0_IMG: GlEnum = GlEnum(0x3000);
/// 
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE0`]
pub const GL_CLIP_DISTANCE0: GlEnum = GL_CLIP_PLANE0;
/// 
/// * Alias Of: [`GL_CLIP_PLANE0`]
pub const GL_CLIP_DISTANCE0_EXT: GlEnum = GL_CLIP_PLANE0;
pub const GL_CLIP_DISTANCE0_APPLE: GlEnum = GlEnum(0x3000);
/// 
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE1: GlEnum = GlEnum(0x3001);
pub const GL_CLIP_PLANE1_IMG: GlEnum = GlEnum(0x3001);
/// 
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE1`]
pub const GL_CLIP_DISTANCE1: GlEnum = GL_CLIP_PLANE1;
/// 
/// * Alias Of: [`GL_CLIP_PLANE1`]
pub const GL_CLIP_DISTANCE1_EXT: GlEnum = GL_CLIP_PLANE1;
pub const GL_CLIP_DISTANCE1_APPLE: GlEnum = GlEnum(0x3001);
/// 
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE2: GlEnum = GlEnum(0x3002);
pub const GL_CLIP_PLANE2_IMG: GlEnum = GlEnum(0x3002);
/// 
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE2`]
pub const GL_CLIP_DISTANCE2: GlEnum = GL_CLIP_PLANE2;
/// 
/// * Alias Of: [`GL_CLIP_PLANE2`]
pub const GL_CLIP_DISTANCE2_EXT: GlEnum = GL_CLIP_PLANE2;
pub const GL_CLIP_DISTANCE2_APPLE: GlEnum = GlEnum(0x3002);
/// 
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE3: GlEnum = GlEnum(0x3003);
pub const GL_CLIP_PLANE3_IMG: GlEnum = GlEnum(0x3003);
/// 
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE3`]
pub const GL_CLIP_DISTANCE3: GlEnum = GL_CLIP_PLANE3;
/// 
/// * Alias Of: [`GL_CLIP_PLANE3`]
pub const GL_CLIP_DISTANCE3_EXT: GlEnum = GL_CLIP_PLANE3;
pub const GL_CLIP_DISTANCE3_APPLE: GlEnum = GlEnum(0x3003);
/// 
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE4: GlEnum = GlEnum(0x3004);
pub const GL_CLIP_PLANE4_IMG: GlEnum = GlEnum(0x3004);
/// 
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE4`]
pub const GL_CLIP_DISTANCE4: GlEnum = GL_CLIP_PLANE4;
/// 
/// * Alias Of: [`GL_CLIP_PLANE4`]
pub const GL_CLIP_DISTANCE4_EXT: GlEnum = GL_CLIP_PLANE4;
pub const GL_CLIP_DISTANCE4_APPLE: GlEnum = GlEnum(0x3004);
/// 
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE5: GlEnum = GlEnum(0x3005);
pub const GL_CLIP_PLANE5_IMG: GlEnum = GlEnum(0x3005);
/// 
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE5`]
pub const GL_CLIP_DISTANCE5: GlEnum = GL_CLIP_PLANE5;
/// 
/// * Alias Of: [`GL_CLIP_PLANE5`]
pub const GL_CLIP_DISTANCE5_EXT: GlEnum = GL_CLIP_PLANE5;
pub const GL_CLIP_DISTANCE5_APPLE: GlEnum = GlEnum(0x3005);
/// 
/// * Group: [`EnableCap`], [`ClipPlaneName`]
pub const GL_CLIP_DISTANCE6: GlEnum = GlEnum(0x3006);
/// 
/// * Alias Of: [`GL_CLIP_DISTANCE6`]
pub const GL_CLIP_DISTANCE6_EXT: GlEnum = GL_CLIP_DISTANCE6;
pub const GL_CLIP_DISTANCE6_APPLE: GlEnum = GlEnum(0x3006);
/// 
/// * Group: [`EnableCap`], [`ClipPlaneName`]
pub const GL_CLIP_DISTANCE7: GlEnum = GlEnum(0x3007);
/// 
/// * Alias Of: [`GL_CLIP_DISTANCE7`]
pub const GL_CLIP_DISTANCE7_EXT: GlEnum = GL_CLIP_DISTANCE7;
pub const GL_CLIP_DISTANCE7_APPLE: GlEnum = GlEnum(0x3007);
/// 
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT0: GlEnum = GlEnum(0x4000);
/// 
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT1: GlEnum = GlEnum(0x4001);
/// 
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT2: GlEnum = GlEnum(0x4002);
/// 
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT3: GlEnum = GlEnum(0x4003);
/// 
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT4: GlEnum = GlEnum(0x4004);
/// 
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT5: GlEnum = GlEnum(0x4005);
/// 
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT6: GlEnum = GlEnum(0x4006);
/// 
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT7: GlEnum = GlEnum(0x4007);
/// 
/// * Group: [`PixelFormat`]
pub const GL_ABGR_EXT: GlEnum = GlEnum(0x8000);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_CONSTANT_COLOR: GlEnum = GlEnum(0x8001);
pub const GL_CONSTANT_COLOR_EXT: GlEnum = GlEnum(0x8001);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_CONSTANT_COLOR: GlEnum = GlEnum(0x8002);
pub const GL_ONE_MINUS_CONSTANT_COLOR_EXT: GlEnum = GlEnum(0x8002);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_CONSTANT_ALPHA: GlEnum = GlEnum(0x8003);
pub const GL_CONSTANT_ALPHA_EXT: GlEnum = GlEnum(0x8003);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GlEnum = GlEnum(0x8004);
pub const GL_ONE_MINUS_CONSTANT_ALPHA_EXT: GlEnum = GlEnum(0x8004);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_COLOR: GlEnum = GlEnum(0x8005);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_COLOR_EXT: GlEnum = GlEnum(0x8005);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_ADD: GlEnum = GlEnum(0x8006);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_ADD_EXT: GlEnum = GlEnum(0x8006);
pub const GL_FUNC_ADD_OES: GlEnum = GlEnum(0x8006);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_MIN: GlEnum = GlEnum(0x8007);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_MIN_EXT: GlEnum = GlEnum(0x8007);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_MAX: GlEnum = GlEnum(0x8008);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_MAX_EXT: GlEnum = GlEnum(0x8008);
pub const GL_BLEND_EQUATION: GlEnum = GlEnum(0x8009);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_EQUATION_EXT: GlEnum = GlEnum(0x8009);
pub const GL_BLEND_EQUATION_OES: GlEnum = GlEnum(0x8009);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_EQUATION_RGB: GlEnum = GlEnum(0x8009);
pub const GL_BLEND_EQUATION_RGB_EXT: GlEnum = GlEnum(0x8009);
pub const GL_BLEND_EQUATION_RGB_OES: GlEnum = GlEnum(0x8009);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_SUBTRACT: GlEnum = GlEnum(0x800A);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_SUBTRACT_EXT: GlEnum = GlEnum(0x800A);
pub const GL_FUNC_SUBTRACT_OES: GlEnum = GlEnum(0x800A);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_REVERSE_SUBTRACT: GlEnum = GlEnum(0x800B);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_REVERSE_SUBTRACT_EXT: GlEnum = GlEnum(0x800B);
pub const GL_FUNC_REVERSE_SUBTRACT_OES: GlEnum = GlEnum(0x800B);
/// 
/// * Group: [`PixelFormat`]
pub const GL_CMYK_EXT: GlEnum = GlEnum(0x800C);
/// 
/// * Group: [`PixelFormat`]
pub const GL_CMYKA_EXT: GlEnum = GlEnum(0x800D);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_PACK_CMYK_HINT_EXT: GlEnum = GlEnum(0x800E);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_UNPACK_CMYK_HINT_EXT: GlEnum = GlEnum(0x800F);
/// 
/// * Group: [`ConvolutionTarget`], [`ConvolutionTargetEXT`]
pub const GL_CONVOLUTION_1D: GlEnum = GlEnum(0x8010);
/// 
/// * Group: [`GetPName`], [`ConvolutionTargetEXT`], [`EnableCap`]
pub const GL_CONVOLUTION_1D_EXT: GlEnum = GlEnum(0x8010);
/// 
/// * Group: [`ConvolutionTarget`], [`ConvolutionTargetEXT`]
pub const GL_CONVOLUTION_2D: GlEnum = GlEnum(0x8011);
/// 
/// * Group: [`GetPName`], [`ConvolutionTargetEXT`], [`EnableCap`]
pub const GL_CONVOLUTION_2D_EXT: GlEnum = GlEnum(0x8011);
/// 
/// * Group: [`SeparableTarget`], [`SeparableTargetEXT`]
pub const GL_SEPARABLE_2D: GlEnum = GlEnum(0x8012);
/// 
/// * Group: [`SeparableTargetEXT`], [`EnableCap`], [`GetPName`]
pub const GL_SEPARABLE_2D_EXT: GlEnum = GlEnum(0x8012);
/// 
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_BORDER_MODE: GlEnum = GlEnum(0x8013);
/// 
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_BORDER_MODE_EXT: GlEnum = GlEnum(0x8013);
/// 
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_FILTER_SCALE: GlEnum = GlEnum(0x8014);
/// 
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_FILTER_SCALE_EXT: GlEnum = GlEnum(0x8014);
/// 
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_FILTER_BIAS: GlEnum = GlEnum(0x8015);
/// 
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_FILTER_BIAS_EXT: GlEnum = GlEnum(0x8015);
/// 
/// * Group: [`ConvolutionBorderModeEXT`]
pub const GL_REDUCE: GlEnum = GlEnum(0x8016);
/// 
/// * Group: [`ConvolutionBorderModeEXT`]
pub const GL_REDUCE_EXT: GlEnum = GlEnum(0x8016);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_FORMAT: GlEnum = GlEnum(0x8017);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_FORMAT_EXT: GlEnum = GlEnum(0x8017);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_WIDTH: GlEnum = GlEnum(0x8018);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_WIDTH_EXT: GlEnum = GlEnum(0x8018);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_HEIGHT: GlEnum = GlEnum(0x8019);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_HEIGHT_EXT: GlEnum = GlEnum(0x8019);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_MAX_CONVOLUTION_WIDTH: GlEnum = GlEnum(0x801A);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_MAX_CONVOLUTION_WIDTH_EXT: GlEnum = GlEnum(0x801A);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_MAX_CONVOLUTION_HEIGHT: GlEnum = GlEnum(0x801B);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_MAX_CONVOLUTION_HEIGHT_EXT: GlEnum = GlEnum(0x801B);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_RED_SCALE: GlEnum = GlEnum(0x801C);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_RED_SCALE_EXT: GlEnum = GlEnum(0x801C);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_GREEN_SCALE: GlEnum = GlEnum(0x801D);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_GREEN_SCALE_EXT: GlEnum = GlEnum(0x801D);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_BLUE_SCALE: GlEnum = GlEnum(0x801E);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_BLUE_SCALE_EXT: GlEnum = GlEnum(0x801E);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_ALPHA_SCALE: GlEnum = GlEnum(0x801F);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_ALPHA_SCALE_EXT: GlEnum = GlEnum(0x801F);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_RED_BIAS: GlEnum = GlEnum(0x8020);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_RED_BIAS_EXT: GlEnum = GlEnum(0x8020);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_GREEN_BIAS: GlEnum = GlEnum(0x8021);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_GREEN_BIAS_EXT: GlEnum = GlEnum(0x8021);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_BLUE_BIAS: GlEnum = GlEnum(0x8022);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_BLUE_BIAS_EXT: GlEnum = GlEnum(0x8022);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_ALPHA_BIAS: GlEnum = GlEnum(0x8023);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_ALPHA_BIAS_EXT: GlEnum = GlEnum(0x8023);
/// 
/// * Group: [`HistogramTarget`], [`HistogramTargetEXT`]
pub const GL_HISTOGRAM: GlEnum = GlEnum(0x8024);
/// 
/// * Group: [`HistogramTargetEXT`], [`EnableCap`], [`GetPName`]
pub const GL_HISTOGRAM_EXT: GlEnum = GlEnum(0x8024);
/// 
/// * Group: [`HistogramTarget`], [`HistogramTargetEXT`]
pub const GL_PROXY_HISTOGRAM: GlEnum = GlEnum(0x8025);
/// 
/// * Group: [`HistogramTargetEXT`]
pub const GL_PROXY_HISTOGRAM_EXT: GlEnum = GlEnum(0x8025);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_WIDTH: GlEnum = GlEnum(0x8026);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_WIDTH_EXT: GlEnum = GlEnum(0x8026);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_FORMAT: GlEnum = GlEnum(0x8027);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_FORMAT_EXT: GlEnum = GlEnum(0x8027);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_RED_SIZE: GlEnum = GlEnum(0x8028);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_RED_SIZE_EXT: GlEnum = GlEnum(0x8028);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_GREEN_SIZE: GlEnum = GlEnum(0x8029);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_GREEN_SIZE_EXT: GlEnum = GlEnum(0x8029);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_BLUE_SIZE: GlEnum = GlEnum(0x802A);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_BLUE_SIZE_EXT: GlEnum = GlEnum(0x802A);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_ALPHA_SIZE: GlEnum = GlEnum(0x802B);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_ALPHA_SIZE_EXT: GlEnum = GlEnum(0x802B);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_LUMINANCE_SIZE: GlEnum = GlEnum(0x802C);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_LUMINANCE_SIZE_EXT: GlEnum = GlEnum(0x802C);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_SINK: GlEnum = GlEnum(0x802D);
/// 
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_SINK_EXT: GlEnum = GlEnum(0x802D);
/// 
/// * Group: [`MinmaxTarget`], [`MinmaxTargetEXT`]
pub const GL_MINMAX: GlEnum = GlEnum(0x802E);
/// 
/// * Group: [`MinmaxTargetEXT`], [`EnableCap`], [`GetPName`]
pub const GL_MINMAX_EXT: GlEnum = GlEnum(0x802E);
/// 
/// * Group: [`GetMinmaxParameterPNameEXT`]
pub const GL_MINMAX_FORMAT: GlEnum = GlEnum(0x802F);
/// 
/// * Group: [`GetMinmaxParameterPNameEXT`]
pub const GL_MINMAX_FORMAT_EXT: GlEnum = GlEnum(0x802F);
/// 
/// * Group: [`GetMinmaxParameterPNameEXT`]
pub const GL_MINMAX_SINK: GlEnum = GlEnum(0x8030);
/// 
/// * Group: [`GetMinmaxParameterPNameEXT`]
pub const GL_MINMAX_SINK_EXT: GlEnum = GlEnum(0x8030);
/// 
/// * Group: [`ErrorCode`]
pub const GL_TABLE_TOO_LARGE_EXT: GlEnum = GlEnum(0x8031);
/// 
/// * Group: [`ErrorCode`]
pub const GL_TABLE_TOO_LARGE: GlEnum = GlEnum(0x8031);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_BYTE_3_3_2: GlEnum = GlEnum(0x8032);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_BYTE_3_3_2_EXT: GlEnum = GlEnum(0x8032);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_SHORT_4_4_4_4: GlEnum = GlEnum(0x8033);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_SHORT_4_4_4_4_EXT: GlEnum = GlEnum(0x8033);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_SHORT_5_5_5_1: GlEnum = GlEnum(0x8034);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_SHORT_5_5_5_1_EXT: GlEnum = GlEnum(0x8034);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_INT_8_8_8_8: GlEnum = GlEnum(0x8035);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_INT_8_8_8_8_EXT: GlEnum = GlEnum(0x8035);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_INT_10_10_10_2: GlEnum = GlEnum(0x8036);
/// 
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_INT_10_10_10_2_EXT: GlEnum = GlEnum(0x8036);
pub const GL_POLYGON_OFFSET_EXT: GlEnum = GlEnum(0x8037);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_OFFSET_FILL: GlEnum = GlEnum(0x8037);
/// 
/// * Group: [`GetPName`]
pub const GL_POLYGON_OFFSET_FACTOR: GlEnum = GlEnum(0x8038);
pub const GL_POLYGON_OFFSET_FACTOR_EXT: GlEnum = GlEnum(0x8038);
/// 
/// * Group: [`GetPName`]
pub const GL_POLYGON_OFFSET_BIAS_EXT: GlEnum = GlEnum(0x8039);
pub const GL_RESCALE_NORMAL: GlEnum = GlEnum(0x803A);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_RESCALE_NORMAL_EXT: GlEnum = GlEnum(0x803A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_ALPHA4: GlEnum = GlEnum(0x803B);
pub const GL_ALPHA4_EXT: GlEnum = GlEnum(0x803B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_ALPHA8: GlEnum = GlEnum(0x803C);
pub const GL_ALPHA8_EXT: GlEnum = GlEnum(0x803C);
pub const GL_ALPHA8_OES: GlEnum = GlEnum(0x803C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_ALPHA12: GlEnum = GlEnum(0x803D);
pub const GL_ALPHA12_EXT: GlEnum = GlEnum(0x803D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_ALPHA16: GlEnum = GlEnum(0x803E);
pub const GL_ALPHA16_EXT: GlEnum = GlEnum(0x803E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE4: GlEnum = GlEnum(0x803F);
pub const GL_LUMINANCE4_EXT: GlEnum = GlEnum(0x803F);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE8: GlEnum = GlEnum(0x8040);
pub const GL_LUMINANCE8_EXT: GlEnum = GlEnum(0x8040);
pub const GL_LUMINANCE8_OES: GlEnum = GlEnum(0x8040);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE12: GlEnum = GlEnum(0x8041);
pub const GL_LUMINANCE12_EXT: GlEnum = GlEnum(0x8041);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE16: GlEnum = GlEnum(0x8042);
pub const GL_LUMINANCE16_EXT: GlEnum = GlEnum(0x8042);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE4_ALPHA4: GlEnum = GlEnum(0x8043);
pub const GL_LUMINANCE4_ALPHA4_EXT: GlEnum = GlEnum(0x8043);
pub const GL_LUMINANCE4_ALPHA4_OES: GlEnum = GlEnum(0x8043);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE6_ALPHA2: GlEnum = GlEnum(0x8044);
pub const GL_LUMINANCE6_ALPHA2_EXT: GlEnum = GlEnum(0x8044);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE8_ALPHA8: GlEnum = GlEnum(0x8045);
pub const GL_LUMINANCE8_ALPHA8_EXT: GlEnum = GlEnum(0x8045);
pub const GL_LUMINANCE8_ALPHA8_OES: GlEnum = GlEnum(0x8045);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE12_ALPHA4: GlEnum = GlEnum(0x8046);
pub const GL_LUMINANCE12_ALPHA4_EXT: GlEnum = GlEnum(0x8046);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE12_ALPHA12: GlEnum = GlEnum(0x8047);
pub const GL_LUMINANCE12_ALPHA12_EXT: GlEnum = GlEnum(0x8047);
/// 
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE16_ALPHA16: GlEnum = GlEnum(0x8048);
pub const GL_LUMINANCE16_ALPHA16_EXT: GlEnum = GlEnum(0x8048);
/// 
/// * Group: [`InternalFormat`], [`PathColorFormat`]
pub const GL_INTENSITY: GlEnum = GlEnum(0x8049);
pub const GL_INTENSITY_EXT: GlEnum = GlEnum(0x8049);
/// 
/// * Group: [`InternalFormat`]
pub const GL_INTENSITY4: GlEnum = GlEnum(0x804A);
pub const GL_INTENSITY4_EXT: GlEnum = GlEnum(0x804A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_INTENSITY8: GlEnum = GlEnum(0x804B);
pub const GL_INTENSITY8_EXT: GlEnum = GlEnum(0x804B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_INTENSITY12: GlEnum = GlEnum(0x804C);
pub const GL_INTENSITY12_EXT: GlEnum = GlEnum(0x804C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_INTENSITY16: GlEnum = GlEnum(0x804D);
pub const GL_INTENSITY16_EXT: GlEnum = GlEnum(0x804D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB2_EXT: GlEnum = GlEnum(0x804E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB4: GlEnum = GlEnum(0x804F);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB4_EXT: GlEnum = GlEnum(0x804F);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB5: GlEnum = GlEnum(0x8050);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB5_EXT: GlEnum = GlEnum(0x8050);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB8: GlEnum = GlEnum(0x8051);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB8_EXT: GlEnum = GlEnum(0x8051);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB8_OES: GlEnum = GlEnum(0x8051);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB10: GlEnum = GlEnum(0x8052);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB10_EXT: GlEnum = GlEnum(0x8052);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB12: GlEnum = GlEnum(0x8053);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB12_EXT: GlEnum = GlEnum(0x8053);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16: GlEnum = GlEnum(0x8054);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16_EXT: GlEnum = GlEnum(0x8054);
pub const GL_RGBA2: GlEnum = GlEnum(0x8055);
pub const GL_RGBA2_EXT: GlEnum = GlEnum(0x8055);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA4: GlEnum = GlEnum(0x8056);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA4_EXT: GlEnum = GlEnum(0x8056);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA4_OES: GlEnum = GlEnum(0x8056);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB5_A1: GlEnum = GlEnum(0x8057);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB5_A1_EXT: GlEnum = GlEnum(0x8057);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB5_A1_OES: GlEnum = GlEnum(0x8057);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA8: GlEnum = GlEnum(0x8058);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA8_EXT: GlEnum = GlEnum(0x8058);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA8_OES: GlEnum = GlEnum(0x8058);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB10_A2: GlEnum = GlEnum(0x8059);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB10_A2_EXT: GlEnum = GlEnum(0x8059);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA12: GlEnum = GlEnum(0x805A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA12_EXT: GlEnum = GlEnum(0x805A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA16: GlEnum = GlEnum(0x805B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA16_EXT: GlEnum = GlEnum(0x805B);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_RED_SIZE: GlEnum = GlEnum(0x805C);
pub const GL_TEXTURE_RED_SIZE_EXT: GlEnum = GlEnum(0x805C);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_GREEN_SIZE: GlEnum = GlEnum(0x805D);
pub const GL_TEXTURE_GREEN_SIZE_EXT: GlEnum = GlEnum(0x805D);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_BLUE_SIZE: GlEnum = GlEnum(0x805E);
pub const GL_TEXTURE_BLUE_SIZE_EXT: GlEnum = GlEnum(0x805E);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_ALPHA_SIZE: GlEnum = GlEnum(0x805F);
pub const GL_TEXTURE_ALPHA_SIZE_EXT: GlEnum = GlEnum(0x805F);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LUMINANCE_SIZE: GlEnum = GlEnum(0x8060);
pub const GL_TEXTURE_LUMINANCE_SIZE_EXT: GlEnum = GlEnum(0x8060);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_INTENSITY_SIZE: GlEnum = GlEnum(0x8061);
pub const GL_TEXTURE_INTENSITY_SIZE_EXT: GlEnum = GlEnum(0x8061);
/// 
/// * Group: [`TextureEnvMode`]
pub const GL_REPLACE_EXT: GlEnum = GlEnum(0x8062);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_1D: GlEnum = GlEnum(0x8063);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_1D_EXT: GlEnum = GlEnum(0x8063);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D: GlEnum = GlEnum(0x8064);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_EXT: GlEnum = GlEnum(0x8064);
/// 
/// * Group: [`ErrorCode`]
pub const GL_TEXTURE_TOO_LARGE_EXT: GlEnum = GlEnum(0x8065);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_PRIORITY: GlEnum = GlEnum(0x8066);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_PRIORITY_EXT: GlEnum = GlEnum(0x8066);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_RESIDENT: GlEnum = GlEnum(0x8067);
pub const GL_TEXTURE_RESIDENT_EXT: GlEnum = GlEnum(0x8067);
pub const GL_TEXTURE_1D_BINDING_EXT: GlEnum = GlEnum(0x8068);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_1D: GlEnum = GlEnum(0x8068);
pub const GL_TEXTURE_2D_BINDING_EXT: GlEnum = GlEnum(0x8069);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_2D: GlEnum = GlEnum(0x8069);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_3D_BINDING_EXT: GlEnum = GlEnum(0x806A);
pub const GL_TEXTURE_3D_BINDING_OES: GlEnum = GlEnum(0x806A);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_3D: GlEnum = GlEnum(0x806A);
pub const GL_TEXTURE_BINDING_3D_OES: GlEnum = GlEnum(0x806A);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_IMAGES: GlEnum = GlEnum(0x806B);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_IMAGES_EXT: GlEnum = GlEnum(0x806B);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_IMAGE_HEIGHT: GlEnum = GlEnum(0x806C);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_IMAGE_HEIGHT_EXT: GlEnum = GlEnum(0x806C);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_IMAGES: GlEnum = GlEnum(0x806D);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_IMAGES_EXT: GlEnum = GlEnum(0x806D);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_IMAGE_HEIGHT: GlEnum = GlEnum(0x806E);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_IMAGE_HEIGHT_EXT: GlEnum = GlEnum(0x806E);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_3D: GlEnum = GlEnum(0x806F);
/// 
/// * Group: [`TextureTarget`], [`EnableCap`], [`GetPName`]
pub const GL_TEXTURE_3D_EXT: GlEnum = GlEnum(0x806F);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_3D_OES: GlEnum = GlEnum(0x806F);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_3D: GlEnum = GlEnum(0x8070);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_3D_EXT: GlEnum = GlEnum(0x8070);
pub const GL_TEXTURE_DEPTH: GlEnum = GlEnum(0x8071);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_DEPTH_EXT: GlEnum = GlEnum(0x8071);
/// 
/// * Group: [`SamplerParameterI`], [`TextureParameterName`]
pub const GL_TEXTURE_WRAP_R: GlEnum = GlEnum(0x8072);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_WRAP_R_EXT: GlEnum = GlEnum(0x8072);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_WRAP_R_OES: GlEnum = GlEnum(0x8072);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_3D_TEXTURE_SIZE: GlEnum = GlEnum(0x8073);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_3D_TEXTURE_SIZE_EXT: GlEnum = GlEnum(0x8073);
pub const GL_MAX_3D_TEXTURE_SIZE_OES: GlEnum = GlEnum(0x8073);
/// 
/// * Group: [`ObjectIdentifier`], [`EnableCap`], [`GetPName`]
pub const GL_VERTEX_ARRAY: GlEnum = GlEnum(0x8074);
pub const GL_VERTEX_ARRAY_EXT: GlEnum = GlEnum(0x8074);
pub const GL_VERTEX_ARRAY_KHR: GlEnum = GlEnum(0x8074);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_NORMAL_ARRAY: GlEnum = GlEnum(0x8075);
pub const GL_NORMAL_ARRAY_EXT: GlEnum = GlEnum(0x8075);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_COLOR_ARRAY: GlEnum = GlEnum(0x8076);
pub const GL_COLOR_ARRAY_EXT: GlEnum = GlEnum(0x8076);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_INDEX_ARRAY: GlEnum = GlEnum(0x8077);
pub const GL_INDEX_ARRAY_EXT: GlEnum = GlEnum(0x8077);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_COORD_ARRAY: GlEnum = GlEnum(0x8078);
pub const GL_TEXTURE_COORD_ARRAY_EXT: GlEnum = GlEnum(0x8078);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_EDGE_FLAG_ARRAY: GlEnum = GlEnum(0x8079);
pub const GL_EDGE_FLAG_ARRAY_EXT: GlEnum = GlEnum(0x8079);
/// 
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_SIZE: GlEnum = GlEnum(0x807A);
pub const GL_VERTEX_ARRAY_SIZE_EXT: GlEnum = GlEnum(0x807A);
/// 
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_TYPE: GlEnum = GlEnum(0x807B);
pub const GL_VERTEX_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x807B);
/// 
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_STRIDE: GlEnum = GlEnum(0x807C);
pub const GL_VERTEX_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x807C);
/// 
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_COUNT_EXT: GlEnum = GlEnum(0x807D);
/// 
/// * Group: [`GetPName`]
pub const GL_NORMAL_ARRAY_TYPE: GlEnum = GlEnum(0x807E);
pub const GL_NORMAL_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x807E);
/// 
/// * Group: [`GetPName`]
pub const GL_NORMAL_ARRAY_STRIDE: GlEnum = GlEnum(0x807F);
pub const GL_NORMAL_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x807F);
/// 
/// * Group: [`GetPName`]
pub const GL_NORMAL_ARRAY_COUNT_EXT: GlEnum = GlEnum(0x8080);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_ARRAY_SIZE: GlEnum = GlEnum(0x8081);
pub const GL_COLOR_ARRAY_SIZE_EXT: GlEnum = GlEnum(0x8081);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_ARRAY_TYPE: GlEnum = GlEnum(0x8082);
pub const GL_COLOR_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x8082);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_ARRAY_STRIDE: GlEnum = GlEnum(0x8083);
pub const GL_COLOR_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x8083);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_ARRAY_COUNT_EXT: GlEnum = GlEnum(0x8084);
/// 
/// * Group: [`GetPName`]
pub const GL_INDEX_ARRAY_TYPE: GlEnum = GlEnum(0x8085);
pub const GL_INDEX_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x8085);
/// 
/// * Group: [`GetPName`]
pub const GL_INDEX_ARRAY_STRIDE: GlEnum = GlEnum(0x8086);
pub const GL_INDEX_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x8086);
/// 
/// * Group: [`GetPName`]
pub const GL_INDEX_ARRAY_COUNT_EXT: GlEnum = GlEnum(0x8087);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_COORD_ARRAY_SIZE: GlEnum = GlEnum(0x8088);
pub const GL_TEXTURE_COORD_ARRAY_SIZE_EXT: GlEnum = GlEnum(0x8088);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_COORD_ARRAY_TYPE: GlEnum = GlEnum(0x8089);
pub const GL_TEXTURE_COORD_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x8089);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_COORD_ARRAY_STRIDE: GlEnum = GlEnum(0x808A);
pub const GL_TEXTURE_COORD_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x808A);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_COORD_ARRAY_COUNT_EXT: GlEnum = GlEnum(0x808B);
/// 
/// * Group: [`GetPName`]
pub const GL_EDGE_FLAG_ARRAY_STRIDE: GlEnum = GlEnum(0x808C);
pub const GL_EDGE_FLAG_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x808C);
/// 
/// * Group: [`GetPName`]
pub const GL_EDGE_FLAG_ARRAY_COUNT_EXT: GlEnum = GlEnum(0x808D);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_VERTEX_ARRAY_POINTER: GlEnum = GlEnum(0x808E);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_VERTEX_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x808E);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_NORMAL_ARRAY_POINTER: GlEnum = GlEnum(0x808F);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_NORMAL_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x808F);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_COLOR_ARRAY_POINTER: GlEnum = GlEnum(0x8090);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_COLOR_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x8090);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_INDEX_ARRAY_POINTER: GlEnum = GlEnum(0x8091);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_INDEX_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x8091);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_TEXTURE_COORD_ARRAY_POINTER: GlEnum = GlEnum(0x8092);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_TEXTURE_COORD_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x8092);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_EDGE_FLAG_ARRAY_POINTER: GlEnum = GlEnum(0x8093);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_EDGE_FLAG_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x8093);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_INTERLACE_SGIX: GlEnum = GlEnum(0x8094);
/// 
/// * Group: [`TextureTarget`]
pub const GL_DETAIL_TEXTURE_2D_SGIS: GlEnum = GlEnum(0x8095);
/// 
/// * Group: [`GetPName`]
pub const GL_DETAIL_TEXTURE_2D_BINDING_SGIS: GlEnum = GlEnum(0x8096);
/// 
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_DETAIL_SGIS: GlEnum = GlEnum(0x8097);
/// 
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_DETAIL_ALPHA_SGIS: GlEnum = GlEnum(0x8098);
/// 
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_DETAIL_COLOR_SGIS: GlEnum = GlEnum(0x8099);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_DETAIL_TEXTURE_LEVEL_SGIS: GlEnum = GlEnum(0x809A);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_DETAIL_TEXTURE_MODE_SGIS: GlEnum = GlEnum(0x809B);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_DETAIL_TEXTURE_FUNC_POINTS_SGIS: GlEnum = GlEnum(0x809C);
/// 
/// * Group: [`EnableCap`]
pub const GL_MULTISAMPLE: GlEnum = GlEnum(0x809D);
pub const GL_MULTISAMPLE_ARB: GlEnum = GlEnum(0x809D);
pub const GL_MULTISAMPLE_EXT: GlEnum = GlEnum(0x809D);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_MULTISAMPLE_SGIS: GlEnum = GlEnum(0x809D);
/// 
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GlEnum = GlEnum(0x809E);
pub const GL_SAMPLE_ALPHA_TO_COVERAGE_ARB: GlEnum = GlEnum(0x809E);
pub const GL_SAMPLE_ALPHA_TO_MASK_EXT: GlEnum = GlEnum(0x809E);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SAMPLE_ALPHA_TO_MASK_SGIS: GlEnum = GlEnum(0x809E);
/// 
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_ALPHA_TO_ONE: GlEnum = GlEnum(0x809F);
pub const GL_SAMPLE_ALPHA_TO_ONE_ARB: GlEnum = GlEnum(0x809F);
pub const GL_SAMPLE_ALPHA_TO_ONE_EXT: GlEnum = GlEnum(0x809F);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SAMPLE_ALPHA_TO_ONE_SGIS: GlEnum = GlEnum(0x809F);
/// 
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_COVERAGE: GlEnum = GlEnum(0x80A0);
pub const GL_SAMPLE_COVERAGE_ARB: GlEnum = GlEnum(0x80A0);
pub const GL_SAMPLE_MASK_EXT: GlEnum = GlEnum(0x80A0);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SAMPLE_MASK_SGIS: GlEnum = GlEnum(0x80A0);
/// 
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_1PASS_EXT: GlEnum = GlEnum(0x80A1);
/// 
/// * Group: [`SamplePatternSGIS`]
pub const GL_1PASS_SGIS: GlEnum = GlEnum(0x80A1);
/// 
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_2PASS_0_EXT: GlEnum = GlEnum(0x80A2);
/// 
/// * Group: [`SamplePatternSGIS`]
pub const GL_2PASS_0_SGIS: GlEnum = GlEnum(0x80A2);
/// 
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_2PASS_1_EXT: GlEnum = GlEnum(0x80A3);
/// 
/// * Group: [`SamplePatternSGIS`]
pub const GL_2PASS_1_SGIS: GlEnum = GlEnum(0x80A3);
/// 
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_4PASS_0_EXT: GlEnum = GlEnum(0x80A4);
/// 
/// * Group: [`SamplePatternSGIS`]
pub const GL_4PASS_0_SGIS: GlEnum = GlEnum(0x80A4);
/// 
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_4PASS_1_EXT: GlEnum = GlEnum(0x80A5);
/// 
/// * Group: [`SamplePatternSGIS`]
pub const GL_4PASS_1_SGIS: GlEnum = GlEnum(0x80A5);
/// 
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_4PASS_2_EXT: GlEnum = GlEnum(0x80A6);
/// 
/// * Group: [`SamplePatternSGIS`]
pub const GL_4PASS_2_SGIS: GlEnum = GlEnum(0x80A6);
/// 
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_4PASS_3_EXT: GlEnum = GlEnum(0x80A7);
/// 
/// * Group: [`SamplePatternSGIS`]
pub const GL_4PASS_3_SGIS: GlEnum = GlEnum(0x80A7);
/// 
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_SAMPLE_BUFFERS: GlEnum = GlEnum(0x80A8);
pub const GL_SAMPLE_BUFFERS_ARB: GlEnum = GlEnum(0x80A8);
pub const GL_SAMPLE_BUFFERS_EXT: GlEnum = GlEnum(0x80A8);
/// 
/// * Group: [`GetPName`]
pub const GL_SAMPLE_BUFFERS_SGIS: GlEnum = GlEnum(0x80A8);
/// 
/// * Group: [`GetFramebufferParameter`], [`GetPName`], [`InternalFormatPName`]
pub const GL_SAMPLES: GlEnum = GlEnum(0x80A9);
pub const GL_SAMPLES_ARB: GlEnum = GlEnum(0x80A9);
pub const GL_SAMPLES_EXT: GlEnum = GlEnum(0x80A9);
/// 
/// * Group: [`GetPName`]
pub const GL_SAMPLES_SGIS: GlEnum = GlEnum(0x80A9);
/// 
/// * Group: [`GetPName`]
pub const GL_SAMPLE_COVERAGE_VALUE: GlEnum = GlEnum(0x80AA);
pub const GL_SAMPLE_COVERAGE_VALUE_ARB: GlEnum = GlEnum(0x80AA);
pub const GL_SAMPLE_MASK_VALUE_EXT: GlEnum = GlEnum(0x80AA);
/// 
/// * Group: [`GetPName`]
pub const GL_SAMPLE_MASK_VALUE_SGIS: GlEnum = GlEnum(0x80AA);
/// 
/// * Group: [`GetPName`]
pub const GL_SAMPLE_COVERAGE_INVERT: GlEnum = GlEnum(0x80AB);
pub const GL_SAMPLE_COVERAGE_INVERT_ARB: GlEnum = GlEnum(0x80AB);
pub const GL_SAMPLE_MASK_INVERT_EXT: GlEnum = GlEnum(0x80AB);
/// 
/// * Group: [`GetPName`]
pub const GL_SAMPLE_MASK_INVERT_SGIS: GlEnum = GlEnum(0x80AB);
pub const GL_SAMPLE_PATTERN_EXT: GlEnum = GlEnum(0x80AC);
/// 
/// * Group: [`GetPName`]
pub const GL_SAMPLE_PATTERN_SGIS: GlEnum = GlEnum(0x80AC);
/// 
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_SHARPEN_SGIS: GlEnum = GlEnum(0x80AD);
/// 
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_SHARPEN_ALPHA_SGIS: GlEnum = GlEnum(0x80AE);
/// 
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_SHARPEN_COLOR_SGIS: GlEnum = GlEnum(0x80AF);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_SHARPEN_TEXTURE_FUNC_POINTS_SGIS: GlEnum = GlEnum(0x80B0);
pub const GL_COLOR_MATRIX: GlEnum = GlEnum(0x80B1);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_MATRIX_SGI: GlEnum = GlEnum(0x80B1);
pub const GL_COLOR_MATRIX_STACK_DEPTH: GlEnum = GlEnum(0x80B2);
/// 
/// * Group: [`GetPName`]
pub const GL_COLOR_MATRIX_STACK_DEPTH_SGI: GlEnum = GlEnum(0x80B2);
pub const GL_MAX_COLOR_MATRIX_STACK_DEPTH: GlEnum = GlEnum(0x80B3);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COLOR_MATRIX_STACK_DEPTH_SGI: GlEnum = GlEnum(0x80B3);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_RED_SCALE: GlEnum = GlEnum(0x80B4);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_RED_SCALE_SGI: GlEnum = GlEnum(0x80B4);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_GREEN_SCALE: GlEnum = GlEnum(0x80B5);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_GREEN_SCALE_SGI: GlEnum = GlEnum(0x80B5);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_BLUE_SCALE: GlEnum = GlEnum(0x80B6);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_BLUE_SCALE_SGI: GlEnum = GlEnum(0x80B6);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE: GlEnum = GlEnum(0x80B7);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE_SGI: GlEnum = GlEnum(0x80B7);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_RED_BIAS: GlEnum = GlEnum(0x80B8);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_RED_BIAS_SGI: GlEnum = GlEnum(0x80B8);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_GREEN_BIAS: GlEnum = GlEnum(0x80B9);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_GREEN_BIAS_SGI: GlEnum = GlEnum(0x80B9);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_BLUE_BIAS: GlEnum = GlEnum(0x80BA);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_BLUE_BIAS_SGI: GlEnum = GlEnum(0x80BA);
/// 
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS: GlEnum = GlEnum(0x80BB);
/// 
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS_SGI: GlEnum = GlEnum(0x80BB);
/// 
/// * Group: [`GetPName`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_TEXTURE_COLOR_TABLE_SGI: GlEnum = GlEnum(0x80BC);
/// 
/// * Group: [`ColorTableTargetSGI`]
pub const GL_PROXY_TEXTURE_COLOR_TABLE_SGI: GlEnum = GlEnum(0x80BD);
/// 
/// * Group: [`TextureEnvMode`]
pub const GL_TEXTURE_ENV_BIAS_SGIX: GlEnum = GlEnum(0x80BE);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_SHADOW_AMBIENT_SGIX: GlEnum = GlEnum(0x80BF);
pub const GL_TEXTURE_COMPARE_FAIL_VALUE_ARB: GlEnum = GlEnum(0x80BF);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_DST_RGB: GlEnum = GlEnum(0x80C8);
pub const GL_BLEND_DST_RGB_EXT: GlEnum = GlEnum(0x80C8);
pub const GL_BLEND_DST_RGB_OES: GlEnum = GlEnum(0x80C8);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_SRC_RGB: GlEnum = GlEnum(0x80C9);
pub const GL_BLEND_SRC_RGB_EXT: GlEnum = GlEnum(0x80C9);
pub const GL_BLEND_SRC_RGB_OES: GlEnum = GlEnum(0x80C9);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_DST_ALPHA: GlEnum = GlEnum(0x80CA);
pub const GL_BLEND_DST_ALPHA_EXT: GlEnum = GlEnum(0x80CA);
pub const GL_BLEND_DST_ALPHA_OES: GlEnum = GlEnum(0x80CA);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_SRC_ALPHA: GlEnum = GlEnum(0x80CB);
pub const GL_BLEND_SRC_ALPHA_EXT: GlEnum = GlEnum(0x80CB);
pub const GL_BLEND_SRC_ALPHA_OES: GlEnum = GlEnum(0x80CB);
pub const GL_422_EXT: GlEnum = GlEnum(0x80CC);
pub const GL_422_REV_EXT: GlEnum = GlEnum(0x80CD);
pub const GL_422_AVERAGE_EXT: GlEnum = GlEnum(0x80CE);
pub const GL_422_REV_AVERAGE_EXT: GlEnum = GlEnum(0x80CF);
/// 
/// * Group: [`ColorTableTarget`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_COLOR_TABLE: GlEnum = GlEnum(0x80D0);
/// 
/// * Group: [`GetPName`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_COLOR_TABLE_SGI: GlEnum = GlEnum(0x80D0);
/// 
/// * Group: [`ColorTableTarget`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_POST_CONVOLUTION_COLOR_TABLE: GlEnum = GlEnum(0x80D1);
/// 
/// * Group: [`GetPName`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_POST_CONVOLUTION_COLOR_TABLE_SGI: GlEnum = GlEnum(0x80D1);
/// 
/// * Group: [`ColorTableTarget`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_POST_COLOR_MATRIX_COLOR_TABLE: GlEnum = GlEnum(0x80D2);
/// 
/// * Group: [`GetPName`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_POST_COLOR_MATRIX_COLOR_TABLE_SGI: GlEnum = GlEnum(0x80D2);
/// 
/// * Group: [`ColorTableTargetSGI`], [`ColorTableTarget`]
pub const GL_PROXY_COLOR_TABLE: GlEnum = GlEnum(0x80D3);
/// 
/// * Group: [`ColorTableTargetSGI`]
pub const GL_PROXY_COLOR_TABLE_SGI: GlEnum = GlEnum(0x80D3);
/// 
/// * Group: [`ColorTableTargetSGI`], [`ColorTableTarget`]
pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE: GlEnum = GlEnum(0x80D4);
/// 
/// * Group: [`ColorTableTargetSGI`]
pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE_SGI: GlEnum = GlEnum(0x80D4);
/// 
/// * Group: [`ColorTableTargetSGI`], [`ColorTableTarget`]
pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE: GlEnum = GlEnum(0x80D5);
/// 
/// * Group: [`ColorTableTargetSGI`]
pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE_SGI: GlEnum = GlEnum(0x80D5);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_SCALE: GlEnum = GlEnum(0x80D6);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_SCALE_SGI: GlEnum = GlEnum(0x80D6);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_BIAS: GlEnum = GlEnum(0x80D7);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_BIAS_SGI: GlEnum = GlEnum(0x80D7);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_FORMAT: GlEnum = GlEnum(0x80D8);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_FORMAT_SGI: GlEnum = GlEnum(0x80D8);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_WIDTH: GlEnum = GlEnum(0x80D9);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_WIDTH_SGI: GlEnum = GlEnum(0x80D9);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_RED_SIZE: GlEnum = GlEnum(0x80DA);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_RED_SIZE_SGI: GlEnum = GlEnum(0x80DA);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_GREEN_SIZE: GlEnum = GlEnum(0x80DB);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_GREEN_SIZE_SGI: GlEnum = GlEnum(0x80DB);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_BLUE_SIZE: GlEnum = GlEnum(0x80DC);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_BLUE_SIZE_SGI: GlEnum = GlEnum(0x80DC);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_ALPHA_SIZE: GlEnum = GlEnum(0x80DD);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_ALPHA_SIZE_SGI: GlEnum = GlEnum(0x80DD);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_LUMINANCE_SIZE: GlEnum = GlEnum(0x80DE);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_LUMINANCE_SIZE_SGI: GlEnum = GlEnum(0x80DE);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_INTENSITY_SIZE: GlEnum = GlEnum(0x80DF);
/// 
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_INTENSITY_SIZE_SGI: GlEnum = GlEnum(0x80DF);
/// 
/// * Group: [`PixelFormat`]
pub const GL_BGR: GlEnum = GlEnum(0x80E0);
pub const GL_BGR_EXT: GlEnum = GlEnum(0x80E0);
/// 
/// * Group: [`PixelFormat`]
pub const GL_BGRA: GlEnum = GlEnum(0x80E1);
pub const GL_BGRA_EXT: GlEnum = GlEnum(0x80E1);
pub const GL_BGRA_IMG: GlEnum = GlEnum(0x80E1);
pub const GL_COLOR_INDEX1_EXT: GlEnum = GlEnum(0x80E2);
pub const GL_COLOR_INDEX2_EXT: GlEnum = GlEnum(0x80E3);
pub const GL_COLOR_INDEX4_EXT: GlEnum = GlEnum(0x80E4);
pub const GL_COLOR_INDEX8_EXT: GlEnum = GlEnum(0x80E5);
pub const GL_COLOR_INDEX12_EXT: GlEnum = GlEnum(0x80E6);
pub const GL_COLOR_INDEX16_EXT: GlEnum = GlEnum(0x80E7);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ELEMENTS_VERTICES: GlEnum = GlEnum(0x80E8);
pub const GL_MAX_ELEMENTS_VERTICES_EXT: GlEnum = GlEnum(0x80E8);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ELEMENTS_INDICES: GlEnum = GlEnum(0x80E9);
pub const GL_MAX_ELEMENTS_INDICES_EXT: GlEnum = GlEnum(0x80E9);
pub const GL_PHONG_WIN: GlEnum = GlEnum(0x80EA);
/// 
/// * Group: [`HintTarget`]
pub const GL_PHONG_HINT_WIN: GlEnum = GlEnum(0x80EB);
pub const GL_FOG_SPECULAR_TEXTURE_WIN: GlEnum = GlEnum(0x80EC);
pub const GL_TEXTURE_INDEX_SIZE_EXT: GlEnum = GlEnum(0x80ED);
/// 
/// * Group: [`BufferTargetARB`]
pub const GL_PARAMETER_BUFFER: GlEnum = GlEnum(0x80EE);
/// 
/// * Alias Of: [`GL_PARAMETER_BUFFER`]
pub const GL_PARAMETER_BUFFER_ARB: GlEnum = GL_PARAMETER_BUFFER;
pub const GL_PARAMETER_BUFFER_BINDING: GlEnum = GlEnum(0x80EF);
/// 
/// * Alias Of: [`GL_PARAMETER_BUFFER_BINDING`]
pub const GL_PARAMETER_BUFFER_BINDING_ARB: GlEnum = GL_PARAMETER_BUFFER_BINDING;
/// 
/// * Group: [`HintTarget`]
pub const GL_CLIP_VOLUME_CLIPPING_HINT_EXT: GlEnum = GlEnum(0x80F0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_ALPHA4_SGIS: GlEnum = GlEnum(0x8110);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_ALPHA8_SGIS: GlEnum = GlEnum(0x8111);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_ALPHA12_SGIS: GlEnum = GlEnum(0x8112);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_ALPHA16_SGIS: GlEnum = GlEnum(0x8113);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE4_SGIS: GlEnum = GlEnum(0x8114);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE8_SGIS: GlEnum = GlEnum(0x8115);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE12_SGIS: GlEnum = GlEnum(0x8116);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE16_SGIS: GlEnum = GlEnum(0x8117);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_INTENSITY4_SGIS: GlEnum = GlEnum(0x8118);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_INTENSITY8_SGIS: GlEnum = GlEnum(0x8119);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_INTENSITY12_SGIS: GlEnum = GlEnum(0x811A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_INTENSITY16_SGIS: GlEnum = GlEnum(0x811B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE_ALPHA4_SGIS: GlEnum = GlEnum(0x811C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE_ALPHA8_SGIS: GlEnum = GlEnum(0x811D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_QUAD_ALPHA4_SGIS: GlEnum = GlEnum(0x811E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_QUAD_ALPHA8_SGIS: GlEnum = GlEnum(0x811F);
/// 
/// * Group: [`InternalFormat`]
pub const GL_QUAD_LUMINANCE4_SGIS: GlEnum = GlEnum(0x8120);
/// 
/// * Group: [`InternalFormat`]
pub const GL_QUAD_LUMINANCE8_SGIS: GlEnum = GlEnum(0x8121);
/// 
/// * Group: [`InternalFormat`]
pub const GL_QUAD_INTENSITY4_SGIS: GlEnum = GlEnum(0x8122);
/// 
/// * Group: [`InternalFormat`]
pub const GL_QUAD_INTENSITY8_SGIS: GlEnum = GlEnum(0x8123);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_DUAL_TEXTURE_SELECT_SGIS: GlEnum = GlEnum(0x8124);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_QUAD_TEXTURE_SELECT_SGIS: GlEnum = GlEnum(0x8125);
/// 
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_SIZE_MIN: GlEnum = GlEnum(0x8126);
/// 
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_SIZE_MIN_ARB: GlEnum = GlEnum(0x8126);
/// 
/// * Group: [`PointParameterNameSGIS`], [`PointParameterNameARB`]
pub const GL_POINT_SIZE_MIN_EXT: GlEnum = GlEnum(0x8126);
/// 
/// * Group: [`PointParameterNameSGIS`], [`GetPName`]
pub const GL_POINT_SIZE_MIN_SGIS: GlEnum = GlEnum(0x8126);
/// 
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_SIZE_MAX: GlEnum = GlEnum(0x8127);
/// 
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_SIZE_MAX_ARB: GlEnum = GlEnum(0x8127);
/// 
/// * Group: [`PointParameterNameSGIS`], [`PointParameterNameARB`]
pub const GL_POINT_SIZE_MAX_EXT: GlEnum = GlEnum(0x8127);
/// 
/// * Group: [`PointParameterNameSGIS`], [`GetPName`]
pub const GL_POINT_SIZE_MAX_SGIS: GlEnum = GlEnum(0x8127);
/// 
/// * Group: [`PointParameterNameSGIS`], [`PointParameterNameARB`], [`GetPName`]
pub const GL_POINT_FADE_THRESHOLD_SIZE: GlEnum = GlEnum(0x8128);
/// 
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_FADE_THRESHOLD_SIZE_ARB: GlEnum = GlEnum(0x8128);
/// 
/// * Group: [`PointParameterNameSGIS`], [`PointParameterNameARB`]
pub const GL_POINT_FADE_THRESHOLD_SIZE_EXT: GlEnum = GlEnum(0x8128);
/// 
/// * Group: [`PointParameterNameSGIS`], [`GetPName`]
pub const GL_POINT_FADE_THRESHOLD_SIZE_SGIS: GlEnum = GlEnum(0x8128);
/// 
/// * Group: [`PointParameterNameSGIS`]
pub const GL_DISTANCE_ATTENUATION_EXT: GlEnum = GlEnum(0x8129);
/// 
/// * Group: [`PointParameterNameSGIS`], [`GetPName`]
pub const GL_DISTANCE_ATTENUATION_SGIS: GlEnum = GlEnum(0x8129);
/// 
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_DISTANCE_ATTENUATION: GlEnum = GlEnum(0x8129);
/// 
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_DISTANCE_ATTENUATION_ARB: GlEnum = GlEnum(0x8129);
/// 
/// * Group: [`FogMode`]
pub const GL_FOG_FUNC_SGIS: GlEnum = GlEnum(0x812A);
/// 
/// * Group: [`GetPName`]
pub const GL_FOG_FUNC_POINTS_SGIS: GlEnum = GlEnum(0x812B);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FOG_FUNC_POINTS_SGIS: GlEnum = GlEnum(0x812C);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_BORDER: GlEnum = GlEnum(0x812D);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_BORDER_ARB: GlEnum = GlEnum(0x812D);
pub const GL_CLAMP_TO_BORDER_EXT: GlEnum = GlEnum(0x812D);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_BORDER_NV: GlEnum = GlEnum(0x812D);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_BORDER_SGIS: GlEnum = GlEnum(0x812D);
pub const GL_CLAMP_TO_BORDER_OES: GlEnum = GlEnum(0x812D);
/// 
/// * Group: [`HintTarget`]
pub const GL_TEXTURE_MULTI_BUFFER_HINT_SGIX: GlEnum = GlEnum(0x812E);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_EDGE: GlEnum = GlEnum(0x812F);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_EDGE_SGIS: GlEnum = GlEnum(0x812F);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_VOLUMES_SGIS: GlEnum = GlEnum(0x8130);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_IMAGE_DEPTH_SGIS: GlEnum = GlEnum(0x8131);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_VOLUMES_SGIS: GlEnum = GlEnum(0x8132);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_IMAGE_DEPTH_SGIS: GlEnum = GlEnum(0x8133);
/// 
/// * Group: [`TextureTarget`], [`EnableCap`], [`GetPName`]
pub const GL_TEXTURE_4D_SGIS: GlEnum = GlEnum(0x8134);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_4D_SGIS: GlEnum = GlEnum(0x8135);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_4DSIZE_SGIS: GlEnum = GlEnum(0x8136);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_WRAP_Q_SGIS: GlEnum = GlEnum(0x8137);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_4D_TEXTURE_SIZE_SGIS: GlEnum = GlEnum(0x8138);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_PIXEL_TEX_GEN_SGIX: GlEnum = GlEnum(0x8139);
/// 
/// * Group: [`SamplerParameterF`], [`TextureParameterName`]
pub const GL_TEXTURE_MIN_LOD: GlEnum = GlEnum(0x813A);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MIN_LOD_SGIS: GlEnum = GlEnum(0x813A);
/// 
/// * Group: [`SamplerParameterF`], [`TextureParameterName`]
pub const GL_TEXTURE_MAX_LOD: GlEnum = GlEnum(0x813B);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_LOD_SGIS: GlEnum = GlEnum(0x813B);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_BASE_LEVEL: GlEnum = GlEnum(0x813C);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_BASE_LEVEL_SGIS: GlEnum = GlEnum(0x813C);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_MAX_LEVEL: GlEnum = GlEnum(0x813D);
pub const GL_TEXTURE_MAX_LEVEL_APPLE: GlEnum = GlEnum(0x813D);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_LEVEL_SGIS: GlEnum = GlEnum(0x813D);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_TILE_BEST_ALIGNMENT_SGIX: GlEnum = GlEnum(0x813E);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_TILE_CACHE_INCREMENT_SGIX: GlEnum = GlEnum(0x813F);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_WIDTH_SGIX: GlEnum = GlEnum(0x8140);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_HEIGHT_SGIX: GlEnum = GlEnum(0x8141);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_GRID_WIDTH_SGIX: GlEnum = GlEnum(0x8142);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_GRID_HEIGHT_SGIX: GlEnum = GlEnum(0x8143);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_GRID_DEPTH_SGIX: GlEnum = GlEnum(0x8144);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_CACHE_SIZE_SGIX: GlEnum = GlEnum(0x8145);
/// 
/// * Group: [`TextureMinFilter`], [`TextureFilterSGIS`],
///   [`TextureFilterFuncSGIS`], [`TextureMagFilter`]
pub const GL_FILTER4_SGIS: GlEnum = GlEnum(0x8146);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_FILTER4_SIZE_SGIS: GlEnum = GlEnum(0x8147);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SPRITE_SGIX: GlEnum = GlEnum(0x8148);
/// 
/// * Group: [`GetPName`], [`SpriteParameterNameSGIX`]
pub const GL_SPRITE_MODE_SGIX: GlEnum = GlEnum(0x8149);
/// 
/// * Group: [`GetPName`]
pub const GL_SPRITE_AXIS_SGIX: GlEnum = GlEnum(0x814A);
/// 
/// * Group: [`GetPName`]
pub const GL_SPRITE_TRANSLATION_SGIX: GlEnum = GlEnum(0x814B);
pub const GL_SPRITE_AXIAL_SGIX: GlEnum = GlEnum(0x814C);
pub const GL_SPRITE_OBJECT_ALIGNED_SGIX: GlEnum = GlEnum(0x814D);
pub const GL_SPRITE_EYE_ALIGNED_SGIX: GlEnum = GlEnum(0x814E);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_4D_BINDING_SGIS: GlEnum = GlEnum(0x814F);
pub const GL_IGNORE_BORDER_HP: GlEnum = GlEnum(0x8150);
pub const GL_CONSTANT_BORDER: GlEnum = GlEnum(0x8151);
pub const GL_CONSTANT_BORDER_HP: GlEnum = GlEnum(0x8151);
pub const GL_REPLICATE_BORDER: GlEnum = GlEnum(0x8153);
pub const GL_REPLICATE_BORDER_HP: GlEnum = GlEnum(0x8153);
/// 
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_BORDER_COLOR: GlEnum = GlEnum(0x8154);
pub const GL_CONVOLUTION_BORDER_COLOR_HP: GlEnum = GlEnum(0x8154);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_SCALE_X_HP: GlEnum = GlEnum(0x8155);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_SCALE_Y_HP: GlEnum = GlEnum(0x8156);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_TRANSLATE_X_HP: GlEnum = GlEnum(0x8157);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_TRANSLATE_Y_HP: GlEnum = GlEnum(0x8158);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_ROTATE_ANGLE_HP: GlEnum = GlEnum(0x8159);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_ROTATE_ORIGIN_X_HP: GlEnum = GlEnum(0x815A);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_ROTATE_ORIGIN_Y_HP: GlEnum = GlEnum(0x815B);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_MAG_FILTER_HP: GlEnum = GlEnum(0x815C);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_MIN_FILTER_HP: GlEnum = GlEnum(0x815D);
/// 
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_CUBIC_WEIGHT_HP: GlEnum = GlEnum(0x815E);
pub const GL_CUBIC_HP: GlEnum = GlEnum(0x815F);
pub const GL_AVERAGE_HP: GlEnum = GlEnum(0x8160);
/// 
/// * Group: [`ImageTransformTargetHP`]
pub const GL_IMAGE_TRANSFORM_2D_HP: GlEnum = GlEnum(0x8161);
pub const GL_POST_IMAGE_TRANSFORM_COLOR_TABLE_HP: GlEnum = GlEnum(0x8162);
pub const GL_PROXY_POST_IMAGE_TRANSFORM_COLOR_TABLE_HP: GlEnum = GlEnum(0x8163);
pub const GL_OCCLUSION_TEST_HP: GlEnum = GlEnum(0x8165);
pub const GL_OCCLUSION_TEST_RESULT_HP: GlEnum = GlEnum(0x8166);
pub const GL_TEXTURE_LIGHTING_MODE_HP: GlEnum = GlEnum(0x8167);
pub const GL_TEXTURE_POST_SPECULAR_HP: GlEnum = GlEnum(0x8168);
pub const GL_TEXTURE_PRE_SPECULAR_HP: GlEnum = GlEnum(0x8169);
/// 
/// * Group: [`TextureMinFilter`]
pub const GL_LINEAR_CLIPMAP_LINEAR_SGIX: GlEnum = GlEnum(0x8170);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_CENTER_SGIX: GlEnum = GlEnum(0x8171);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_FRAME_SGIX: GlEnum = GlEnum(0x8172);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_OFFSET_SGIX: GlEnum = GlEnum(0x8173);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_VIRTUAL_DEPTH_SGIX: GlEnum = GlEnum(0x8174);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_LOD_OFFSET_SGIX: GlEnum = GlEnum(0x8175);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_DEPTH_SGIX: GlEnum = GlEnum(0x8176);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_CLIPMAP_DEPTH_SGIX: GlEnum = GlEnum(0x8177);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_CLIPMAP_VIRTUAL_DEPTH_SGIX: GlEnum = GlEnum(0x8178);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_POST_TEXTURE_FILTER_BIAS_SGIX: GlEnum = GlEnum(0x8179);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_POST_TEXTURE_FILTER_SCALE_SGIX: GlEnum = GlEnum(0x817A);
/// 
/// * Group: [`GetPName`]
pub const GL_POST_TEXTURE_FILTER_BIAS_RANGE_SGIX: GlEnum = GlEnum(0x817B);
/// 
/// * Group: [`GetPName`]
pub const GL_POST_TEXTURE_FILTER_SCALE_RANGE_SGIX: GlEnum = GlEnum(0x817C);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_REFERENCE_PLANE_SGIX: GlEnum = GlEnum(0x817D);
/// 
/// * Group: [`GetPName`]
pub const GL_REFERENCE_PLANE_EQUATION_SGIX: GlEnum = GlEnum(0x817E);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_IR_INSTRUMENT1_SGIX: GlEnum = GlEnum(0x817F);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_INSTRUMENT_BUFFER_POINTER_SGIX: GlEnum = GlEnum(0x8180);
/// 
/// * Group: [`GetPName`]
pub const GL_INSTRUMENT_MEASUREMENTS_SGIX: GlEnum = GlEnum(0x8181);
/// 
/// * Group: [`ListParameterName`]
pub const GL_LIST_PRIORITY_SGIX: GlEnum = GlEnum(0x8182);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_CALLIGRAPHIC_FRAGMENT_SGIX: GlEnum = GlEnum(0x8183);
/// 
/// * Group: [`TextureMinFilter`], [`PixelTexGenModeSGIX`], [`TextureMagFilter`]
pub const GL_PIXEL_TEX_GEN_Q_CEILING_SGIX: GlEnum = GlEnum(0x8184);
/// 
/// * Group: [`TextureMinFilter`], [`PixelTexGenModeSGIX`], [`TextureMagFilter`]
pub const GL_PIXEL_TEX_GEN_Q_ROUND_SGIX: GlEnum = GlEnum(0x8185);
/// 
/// * Group: [`TextureMinFilter`], [`PixelTexGenModeSGIX`], [`TextureMagFilter`]
pub const GL_PIXEL_TEX_GEN_Q_FLOOR_SGIX: GlEnum = GlEnum(0x8186);
/// 
/// * Group: [`PixelTexGenMode`]
pub const GL_PIXEL_TEX_GEN_ALPHA_REPLACE_SGIX: GlEnum = GlEnum(0x8187);
/// 
/// * Group: [`PixelTexGenMode`]
pub const GL_PIXEL_TEX_GEN_ALPHA_NO_REPLACE_SGIX: GlEnum = GlEnum(0x8188);
/// 
/// * Group: [`PixelTexGenMode`], [`PixelTexGenModeSGIX`]
pub const GL_PIXEL_TEX_GEN_ALPHA_LS_SGIX: GlEnum = GlEnum(0x8189);
/// 
/// * Group: [`PixelTexGenMode`], [`PixelTexGenModeSGIX`]
pub const GL_PIXEL_TEX_GEN_ALPHA_MS_SGIX: GlEnum = GlEnum(0x818A);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FRAMEZOOM_SGIX: GlEnum = GlEnum(0x818B);
/// 
/// * Group: [`GetPName`]
pub const GL_FRAMEZOOM_FACTOR_SGIX: GlEnum = GlEnum(0x818C);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEZOOM_FACTOR_SGIX: GlEnum = GlEnum(0x818D);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LOD_BIAS_S_SGIX: GlEnum = GlEnum(0x818E);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LOD_BIAS_T_SGIX: GlEnum = GlEnum(0x818F);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LOD_BIAS_R_SGIX: GlEnum = GlEnum(0x8190);
/// 
/// * Group: [`InternalFormatPName`], [`TextureParameterName`]
pub const GL_GENERATE_MIPMAP: GlEnum = GlEnum(0x8191);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_GENERATE_MIPMAP_SGIS: GlEnum = GlEnum(0x8191);
/// 
/// * Group: [`HintTarget`]
pub const GL_GENERATE_MIPMAP_HINT: GlEnum = GlEnum(0x8192);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_GENERATE_MIPMAP_HINT_SGIS: GlEnum = GlEnum(0x8192);
/// 
/// * Group: [`MapTarget`], [`FfdTargetSGIX`]
pub const GL_GEOMETRY_DEFORMATION_SGIX: GlEnum = GlEnum(0x8194);
/// 
/// * Group: [`MapTarget`], [`FfdTargetSGIX`]
pub const GL_TEXTURE_DEFORMATION_SGIX: GlEnum = GlEnum(0x8195);
/// 
/// * Group: [`GetPName`]
pub const GL_DEFORMATIONS_MASK_SGIX: GlEnum = GlEnum(0x8196);
pub const GL_MAX_DEFORMATION_ORDER_SGIX: GlEnum = GlEnum(0x8197);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FOG_OFFSET_SGIX: GlEnum = GlEnum(0x8198);
/// 
/// * Group: [`GetPName`], [`FogParameter`]
pub const GL_FOG_OFFSET_VALUE_SGIX: GlEnum = GlEnum(0x8199);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_COMPARE_SGIX: GlEnum = GlEnum(0x819A);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_COMPARE_OPERATOR_SGIX: GlEnum = GlEnum(0x819B);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LEQUAL_R_SGIX: GlEnum = GlEnum(0x819C);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_GEQUAL_R_SGIX: GlEnum = GlEnum(0x819D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT16: GlEnum = GlEnum(0x81A5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT16_ARB: GlEnum = GlEnum(0x81A5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT16_OES: GlEnum = GlEnum(0x81A5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT16_SGIX: GlEnum = GlEnum(0x81A5);
pub const GL_DEPTH_COMPONENT24: GlEnum = GlEnum(0x81A6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT24_ARB: GlEnum = GlEnum(0x81A6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT24_OES: GlEnum = GlEnum(0x81A6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT24_SGIX: GlEnum = GlEnum(0x81A6);
pub const GL_DEPTH_COMPONENT32: GlEnum = GlEnum(0x81A7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32_ARB: GlEnum = GlEnum(0x81A7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32_OES: GlEnum = GlEnum(0x81A7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32_SGIX: GlEnum = GlEnum(0x81A7);
pub const GL_ARRAY_ELEMENT_LOCK_FIRST_EXT: GlEnum = GlEnum(0x81A8);
pub const GL_ARRAY_ELEMENT_LOCK_COUNT_EXT: GlEnum = GlEnum(0x81A9);
pub const GL_CULL_VERTEX_EXT: GlEnum = GlEnum(0x81AA);
/// 
/// * Group: [`CullParameterEXT`]
pub const GL_CULL_VERTEX_EYE_POSITION_EXT: GlEnum = GlEnum(0x81AB);
/// 
/// * Group: [`CullParameterEXT`]
pub const GL_CULL_VERTEX_OBJECT_POSITION_EXT: GlEnum = GlEnum(0x81AC);
pub const GL_IUI_V2F_EXT: GlEnum = GlEnum(0x81AD);
pub const GL_IUI_V3F_EXT: GlEnum = GlEnum(0x81AE);
pub const GL_IUI_N3F_V2F_EXT: GlEnum = GlEnum(0x81AF);
pub const GL_IUI_N3F_V3F_EXT: GlEnum = GlEnum(0x81B0);
pub const GL_T2F_IUI_V2F_EXT: GlEnum = GlEnum(0x81B1);
pub const GL_T2F_IUI_V3F_EXT: GlEnum = GlEnum(0x81B2);
pub const GL_T2F_IUI_N3F_V2F_EXT: GlEnum = GlEnum(0x81B3);
pub const GL_T2F_IUI_N3F_V3F_EXT: GlEnum = GlEnum(0x81B4);
pub const GL_INDEX_TEST_EXT: GlEnum = GlEnum(0x81B5);
pub const GL_INDEX_TEST_FUNC_EXT: GlEnum = GlEnum(0x81B6);
pub const GL_INDEX_TEST_REF_EXT: GlEnum = GlEnum(0x81B7);
pub const GL_INDEX_MATERIAL_EXT: GlEnum = GlEnum(0x81B8);
pub const GL_INDEX_MATERIAL_PARAMETER_EXT: GlEnum = GlEnum(0x81B9);
pub const GL_INDEX_MATERIAL_FACE_EXT: GlEnum = GlEnum(0x81BA);
/// 
/// * Group: [`PixelFormat`]
pub const GL_YCRCB_422_SGIX: GlEnum = GlEnum(0x81BB);
/// 
/// * Group: [`PixelFormat`]
pub const GL_YCRCB_444_SGIX: GlEnum = GlEnum(0x81BC);
pub const GL_WRAP_BORDER_SUN: GlEnum = GlEnum(0x81D4);
pub const GL_UNPACK_CONSTANT_DATA_SUNX: GlEnum = GlEnum(0x81D5);
pub const GL_TEXTURE_CONSTANT_DATA_SUNX: GlEnum = GlEnum(0x81D6);
pub const GL_TRIANGLE_LIST_SUN: GlEnum = GlEnum(0x81D7);
pub const GL_REPLACEMENT_CODE_SUN: GlEnum = GlEnum(0x81D8);
pub const GL_GLOBAL_ALPHA_SUN: GlEnum = GlEnum(0x81D9);
pub const GL_GLOBAL_ALPHA_FACTOR_SUN: GlEnum = GlEnum(0x81DA);
pub const GL_TEXTURE_COLOR_WRITEMASK_SGIS: GlEnum = GlEnum(0x81EF);
/// 
/// * Group: [`TextureGenMode`]
pub const GL_EYE_DISTANCE_TO_POINT_SGIS: GlEnum = GlEnum(0x81F0);
/// 
/// * Group: [`TextureGenMode`]
pub const GL_OBJECT_DISTANCE_TO_POINT_SGIS: GlEnum = GlEnum(0x81F1);
/// 
/// * Group: [`TextureGenMode`]
pub const GL_EYE_DISTANCE_TO_LINE_SGIS: GlEnum = GlEnum(0x81F2);
/// 
/// * Group: [`TextureGenMode`]
pub const GL_OBJECT_DISTANCE_TO_LINE_SGIS: GlEnum = GlEnum(0x81F3);
/// 
/// * Group: [`TextureGenParameter`]
pub const GL_EYE_POINT_SGIS: GlEnum = GlEnum(0x81F4);
/// 
/// * Group: [`TextureGenParameter`]
pub const GL_OBJECT_POINT_SGIS: GlEnum = GlEnum(0x81F5);
/// 
/// * Group: [`TextureGenParameter`]
pub const GL_EYE_LINE_SGIS: GlEnum = GlEnum(0x81F6);
/// 
/// * Group: [`TextureGenParameter`]
pub const GL_OBJECT_LINE_SGIS: GlEnum = GlEnum(0x81F7);
/// 
/// * Group: [`LightModelParameter`], [`GetPName`]
pub const GL_LIGHT_MODEL_COLOR_CONTROL: GlEnum = GlEnum(0x81F8);
/// 
/// * Group: [`LightModelParameter`]
pub const GL_LIGHT_MODEL_COLOR_CONTROL_EXT: GlEnum = GlEnum(0x81F8);
/// 
/// * Group: [`LightModelColorControl`]
pub const GL_SINGLE_COLOR: GlEnum = GlEnum(0x81F9);
/// 
/// * Group: [`LightModelColorControl`]
pub const GL_SINGLE_COLOR_EXT: GlEnum = GlEnum(0x81F9);
/// 
/// * Group: [`LightModelColorControl`]
pub const GL_SEPARATE_SPECULAR_COLOR: GlEnum = GlEnum(0x81FA);
/// 
/// * Group: [`LightModelColorControl`]
pub const GL_SEPARATE_SPECULAR_COLOR_EXT: GlEnum = GlEnum(0x81FA);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SHARED_TEXTURE_PALETTE_EXT: GlEnum = GlEnum(0x81FB);
/// 
/// * Group: [`ProgramTarget`]
pub const GL_TEXT_FRAGMENT_SHADER_ATI: GlEnum = GlEnum(0x8200);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GlEnum = GlEnum(0x8210);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: GlEnum = GlEnum(0x8210);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GlEnum = GlEnum(0x8211);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: GlEnum = GlEnum(0x8211);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GlEnum = GlEnum(0x8212);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GlEnum = GlEnum(0x8213);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GlEnum = GlEnum(0x8214);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GlEnum = GlEnum(0x8215);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GlEnum = GlEnum(0x8216);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GlEnum = GlEnum(0x8217);
pub const GL_FRAMEBUFFER_DEFAULT: GlEnum = GlEnum(0x8218);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_UNDEFINED: GlEnum = GlEnum(0x8219);
pub const GL_FRAMEBUFFER_UNDEFINED_OES: GlEnum = GlEnum(0x8219);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_DEPTH_STENCIL_ATTACHMENT: GlEnum = GlEnum(0x821A);
/// 
/// * Group: [`GetPName`]
pub const GL_MAJOR_VERSION: GlEnum = GlEnum(0x821B);
/// 
/// * Group: [`GetPName`]
pub const GL_MINOR_VERSION: GlEnum = GlEnum(0x821C);
/// 
/// * Group: [`GetPName`]
pub const GL_NUM_EXTENSIONS: GlEnum = GlEnum(0x821D);
/// 
/// * Group: [`GetPName`]
pub const GL_CONTEXT_FLAGS: GlEnum = GlEnum(0x821E);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_IMMUTABLE_STORAGE: GlEnum = GlEnum(0x821F);
pub const GL_BUFFER_IMMUTABLE_STORAGE_EXT: GlEnum = GlEnum(0x821F);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_STORAGE_FLAGS: GlEnum = GlEnum(0x8220);
pub const GL_BUFFER_STORAGE_FLAGS_EXT: GlEnum = GlEnum(0x8220);
pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GlEnum = GlEnum(0x8221);
pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED_OES: GlEnum = GlEnum(0x8221);
pub const GL_INDEX: GlEnum = GlEnum(0x8222);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RED: GlEnum = GlEnum(0x8225);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RG: GlEnum = GlEnum(0x8226);
/// 
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_RG: GlEnum = GlEnum(0x8227);
pub const GL_RG_EXT: GlEnum = GlEnum(0x8227);
/// 
/// * Group: [`PixelFormat`]
pub const GL_RG_INTEGER: GlEnum = GlEnum(0x8228);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R8: GlEnum = GlEnum(0x8229);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R8_EXT: GlEnum = GlEnum(0x8229);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R16: GlEnum = GlEnum(0x822A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R16_EXT: GlEnum = GlEnum(0x822A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG8: GlEnum = GlEnum(0x822B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG8_EXT: GlEnum = GlEnum(0x822B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG16: GlEnum = GlEnum(0x822C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG16_EXT: GlEnum = GlEnum(0x822C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R16F: GlEnum = GlEnum(0x822D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R16F_EXT: GlEnum = GlEnum(0x822D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R32F: GlEnum = GlEnum(0x822E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R32F_EXT: GlEnum = GlEnum(0x822E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG16F: GlEnum = GlEnum(0x822F);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG16F_EXT: GlEnum = GlEnum(0x822F);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG32F: GlEnum = GlEnum(0x8230);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG32F_EXT: GlEnum = GlEnum(0x8230);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R8I: GlEnum = GlEnum(0x8231);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R8UI: GlEnum = GlEnum(0x8232);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R16I: GlEnum = GlEnum(0x8233);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R16UI: GlEnum = GlEnum(0x8234);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R32I: GlEnum = GlEnum(0x8235);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R32UI: GlEnum = GlEnum(0x8236);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG8I: GlEnum = GlEnum(0x8237);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG8UI: GlEnum = GlEnum(0x8238);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG16I: GlEnum = GlEnum(0x8239);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG16UI: GlEnum = GlEnum(0x823A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG32I: GlEnum = GlEnum(0x823B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG32UI: GlEnum = GlEnum(0x823C);
pub const GL_SYNC_CL_EVENT_ARB: GlEnum = GlEnum(0x8240);
pub const GL_SYNC_CL_EVENT_COMPLETE_ARB: GlEnum = GlEnum(0x8241);
/// 
/// * Group: [`EnableCap`]
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GlEnum = GlEnum(0x8242);
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB: GlEnum = GlEnum(0x8242);
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_KHR: GlEnum = GlEnum(0x8242);
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GlEnum = GlEnum(0x8243);
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB: GlEnum = GlEnum(0x8243);
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR: GlEnum = GlEnum(0x8243);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_DEBUG_CALLBACK_FUNCTION: GlEnum = GlEnum(0x8244);
pub const GL_DEBUG_CALLBACK_FUNCTION_ARB: GlEnum = GlEnum(0x8244);
pub const GL_DEBUG_CALLBACK_FUNCTION_KHR: GlEnum = GlEnum(0x8244);
/// 
/// * Group: [`GetPointervPName`]
pub const GL_DEBUG_CALLBACK_USER_PARAM: GlEnum = GlEnum(0x8245);
pub const GL_DEBUG_CALLBACK_USER_PARAM_ARB: GlEnum = GlEnum(0x8245);
pub const GL_DEBUG_CALLBACK_USER_PARAM_KHR: GlEnum = GlEnum(0x8245);
/// 
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_API: GlEnum = GlEnum(0x8246);
pub const GL_DEBUG_SOURCE_API_ARB: GlEnum = GlEnum(0x8246);
pub const GL_DEBUG_SOURCE_API_KHR: GlEnum = GlEnum(0x8246);
/// 
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GlEnum = GlEnum(0x8247);
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB: GlEnum = GlEnum(0x8247);
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR: GlEnum = GlEnum(0x8247);
/// 
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GlEnum = GlEnum(0x8248);
pub const GL_DEBUG_SOURCE_SHADER_COMPILER_ARB: GlEnum = GlEnum(0x8248);
pub const GL_DEBUG_SOURCE_SHADER_COMPILER_KHR: GlEnum = GlEnum(0x8248);
/// 
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GlEnum = GlEnum(0x8249);
pub const GL_DEBUG_SOURCE_THIRD_PARTY_ARB: GlEnum = GlEnum(0x8249);
pub const GL_DEBUG_SOURCE_THIRD_PARTY_KHR: GlEnum = GlEnum(0x8249);
/// 
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_APPLICATION: GlEnum = GlEnum(0x824A);
pub const GL_DEBUG_SOURCE_APPLICATION_ARB: GlEnum = GlEnum(0x824A);
pub const GL_DEBUG_SOURCE_APPLICATION_KHR: GlEnum = GlEnum(0x824A);
/// 
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_OTHER: GlEnum = GlEnum(0x824B);
pub const GL_DEBUG_SOURCE_OTHER_ARB: GlEnum = GlEnum(0x824B);
pub const GL_DEBUG_SOURCE_OTHER_KHR: GlEnum = GlEnum(0x824B);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_ERROR: GlEnum = GlEnum(0x824C);
pub const GL_DEBUG_TYPE_ERROR_ARB: GlEnum = GlEnum(0x824C);
pub const GL_DEBUG_TYPE_ERROR_KHR: GlEnum = GlEnum(0x824C);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GlEnum = GlEnum(0x824D);
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB: GlEnum = GlEnum(0x824D);
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR: GlEnum = GlEnum(0x824D);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GlEnum = GlEnum(0x824E);
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB: GlEnum = GlEnum(0x824E);
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR: GlEnum = GlEnum(0x824E);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_PORTABILITY: GlEnum = GlEnum(0x824F);
pub const GL_DEBUG_TYPE_PORTABILITY_ARB: GlEnum = GlEnum(0x824F);
pub const GL_DEBUG_TYPE_PORTABILITY_KHR: GlEnum = GlEnum(0x824F);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_PERFORMANCE: GlEnum = GlEnum(0x8250);
pub const GL_DEBUG_TYPE_PERFORMANCE_ARB: GlEnum = GlEnum(0x8250);
pub const GL_DEBUG_TYPE_PERFORMANCE_KHR: GlEnum = GlEnum(0x8250);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_OTHER: GlEnum = GlEnum(0x8251);
pub const GL_DEBUG_TYPE_OTHER_ARB: GlEnum = GlEnum(0x8251);
pub const GL_DEBUG_TYPE_OTHER_KHR: GlEnum = GlEnum(0x8251);
pub const GL_LOSE_CONTEXT_ON_RESET: GlEnum = GlEnum(0x8252);
pub const GL_LOSE_CONTEXT_ON_RESET_ARB: GlEnum = GlEnum(0x8252);
pub const GL_LOSE_CONTEXT_ON_RESET_EXT: GlEnum = GlEnum(0x8252);
pub const GL_LOSE_CONTEXT_ON_RESET_KHR: GlEnum = GlEnum(0x8252);
/// 
/// * Group: [`GraphicsResetStatus`]
pub const GL_GUILTY_CONTEXT_RESET: GlEnum = GlEnum(0x8253);
pub const GL_GUILTY_CONTEXT_RESET_ARB: GlEnum = GlEnum(0x8253);
pub const GL_GUILTY_CONTEXT_RESET_EXT: GlEnum = GlEnum(0x8253);
pub const GL_GUILTY_CONTEXT_RESET_KHR: GlEnum = GlEnum(0x8253);
/// 
/// * Group: [`GraphicsResetStatus`]
pub const GL_INNOCENT_CONTEXT_RESET: GlEnum = GlEnum(0x8254);
pub const GL_INNOCENT_CONTEXT_RESET_ARB: GlEnum = GlEnum(0x8254);
pub const GL_INNOCENT_CONTEXT_RESET_EXT: GlEnum = GlEnum(0x8254);
pub const GL_INNOCENT_CONTEXT_RESET_KHR: GlEnum = GlEnum(0x8254);
/// 
/// * Group: [`GraphicsResetStatus`]
pub const GL_UNKNOWN_CONTEXT_RESET: GlEnum = GlEnum(0x8255);
pub const GL_UNKNOWN_CONTEXT_RESET_ARB: GlEnum = GlEnum(0x8255);
pub const GL_UNKNOWN_CONTEXT_RESET_EXT: GlEnum = GlEnum(0x8255);
pub const GL_UNKNOWN_CONTEXT_RESET_KHR: GlEnum = GlEnum(0x8255);
pub const GL_RESET_NOTIFICATION_STRATEGY: GlEnum = GlEnum(0x8256);
pub const GL_RESET_NOTIFICATION_STRATEGY_ARB: GlEnum = GlEnum(0x8256);
pub const GL_RESET_NOTIFICATION_STRATEGY_EXT: GlEnum = GlEnum(0x8256);
pub const GL_RESET_NOTIFICATION_STRATEGY_KHR: GlEnum = GlEnum(0x8256);
/// 
/// * Group: [`ProgramParameterPName`], [`HintTarget`]
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GlEnum = GlEnum(0x8257);
/// 
/// * Group: [`ProgramParameterPName`]
pub const GL_PROGRAM_SEPARABLE: GlEnum = GlEnum(0x8258);
pub const GL_PROGRAM_SEPARABLE_EXT: GlEnum = GlEnum(0x8258);
/// 
/// * Group: [`PipelineParameterName`]
pub const GL_ACTIVE_PROGRAM: GlEnum = GlEnum(0x8259);
/// For the OpenGL ES version of `EXT_separate_shader_objects`.
#[doc(alias = "GL_ACTIVE_PROGRAM_EXT")]
pub const GL_ACTIVE_PROGRAM_EXT_ES: GlEnum = GlEnum(0x8259);
/// 
/// * Group: [`GetPName`]
pub const GL_PROGRAM_PIPELINE_BINDING: GlEnum = GlEnum(0x825A);
pub const GL_PROGRAM_PIPELINE_BINDING_EXT: GlEnum = GlEnum(0x825A);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VIEWPORTS: GlEnum = GlEnum(0x825B);
pub const GL_MAX_VIEWPORTS_NV: GlEnum = GlEnum(0x825B);
pub const GL_MAX_VIEWPORTS_OES: GlEnum = GlEnum(0x825B);
/// 
/// * Group: [`GetPName`]
pub const GL_VIEWPORT_SUBPIXEL_BITS: GlEnum = GlEnum(0x825C);
pub const GL_VIEWPORT_SUBPIXEL_BITS_EXT: GlEnum = GlEnum(0x825C);
pub const GL_VIEWPORT_SUBPIXEL_BITS_NV: GlEnum = GlEnum(0x825C);
pub const GL_VIEWPORT_SUBPIXEL_BITS_OES: GlEnum = GlEnum(0x825C);
/// 
/// * Group: [`GetPName`]
pub const GL_VIEWPORT_BOUNDS_RANGE: GlEnum = GlEnum(0x825D);
pub const GL_VIEWPORT_BOUNDS_RANGE_EXT: GlEnum = GlEnum(0x825D);
pub const GL_VIEWPORT_BOUNDS_RANGE_NV: GlEnum = GlEnum(0x825D);
pub const GL_VIEWPORT_BOUNDS_RANGE_OES: GlEnum = GlEnum(0x825D);
/// 
/// * Group: [`GetPName`]
pub const GL_LAYER_PROVOKING_VERTEX: GlEnum = GlEnum(0x825E);
pub const GL_LAYER_PROVOKING_VERTEX_EXT: GlEnum = GlEnum(0x825E);
pub const GL_LAYER_PROVOKING_VERTEX_OES: GlEnum = GlEnum(0x825E);
/// 
/// * Group: [`GetPName`]
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: GlEnum = GlEnum(0x825F);
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX_EXT: GlEnum = GlEnum(0x825F);
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX_NV: GlEnum = GlEnum(0x825F);
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX_OES: GlEnum = GlEnum(0x825F);
pub const GL_UNDEFINED_VERTEX: GlEnum = GlEnum(0x8260);
pub const GL_UNDEFINED_VERTEX_EXT: GlEnum = GlEnum(0x8260);
pub const GL_UNDEFINED_VERTEX_OES: GlEnum = GlEnum(0x8260);
pub const GL_NO_RESET_NOTIFICATION: GlEnum = GlEnum(0x8261);
pub const GL_NO_RESET_NOTIFICATION_ARB: GlEnum = GlEnum(0x8261);
pub const GL_NO_RESET_NOTIFICATION_EXT: GlEnum = GlEnum(0x8261);
pub const GL_NO_RESET_NOTIFICATION_KHR: GlEnum = GlEnum(0x8261);
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GlEnum = GlEnum(0x8262);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8263);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GlEnum = GlEnum(0x8264);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GlEnum = GlEnum(0x8265);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8266);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_COMPUTE_WORK_GROUP_SIZE: GlEnum = GlEnum(0x8267);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_MARKER: GlEnum = GlEnum(0x8268);
pub const GL_DEBUG_TYPE_MARKER_KHR: GlEnum = GlEnum(0x8268);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_PUSH_GROUP: GlEnum = GlEnum(0x8269);
pub const GL_DEBUG_TYPE_PUSH_GROUP_KHR: GlEnum = GlEnum(0x8269);
/// 
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_POP_GROUP: GlEnum = GlEnum(0x826A);
pub const GL_DEBUG_TYPE_POP_GROUP_KHR: GlEnum = GlEnum(0x826A);
/// 
/// * Group: [`DebugSeverity`]
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GlEnum = GlEnum(0x826B);
pub const GL_DEBUG_SEVERITY_NOTIFICATION_KHR: GlEnum = GlEnum(0x826B);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GlEnum = GlEnum(0x826C);
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH_KHR: GlEnum = GlEnum(0x826C);
/// 
/// * Group: [`GetPName`]
pub const GL_DEBUG_GROUP_STACK_DEPTH: GlEnum = GlEnum(0x826D);
pub const GL_DEBUG_GROUP_STACK_DEPTH_KHR: GlEnum = GlEnum(0x826D);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_UNIFORM_LOCATIONS: GlEnum = GlEnum(0x826E);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_SUPPORTED: GlEnum = GlEnum(0x826F);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_PREFERRED: GlEnum = GlEnum(0x8270);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_RED_SIZE: GlEnum = GlEnum(0x8271);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_GREEN_SIZE: GlEnum = GlEnum(0x8272);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_BLUE_SIZE: GlEnum = GlEnum(0x8273);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_ALPHA_SIZE: GlEnum = GlEnum(0x8274);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_DEPTH_SIZE: GlEnum = GlEnum(0x8275);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_STENCIL_SIZE: GlEnum = GlEnum(0x8276);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_SHARED_SIZE: GlEnum = GlEnum(0x8277);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_RED_TYPE: GlEnum = GlEnum(0x8278);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_GREEN_TYPE: GlEnum = GlEnum(0x8279);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_BLUE_TYPE: GlEnum = GlEnum(0x827A);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_ALPHA_TYPE: GlEnum = GlEnum(0x827B);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_DEPTH_TYPE: GlEnum = GlEnum(0x827C);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_STENCIL_TYPE: GlEnum = GlEnum(0x827D);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_MAX_WIDTH: GlEnum = GlEnum(0x827E);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_MAX_HEIGHT: GlEnum = GlEnum(0x827F);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_MAX_DEPTH: GlEnum = GlEnum(0x8280);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_MAX_LAYERS: GlEnum = GlEnum(0x8281);
pub const GL_MAX_COMBINED_DIMENSIONS: GlEnum = GlEnum(0x8282);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_COLOR_COMPONENTS: GlEnum = GlEnum(0x8283);
pub const GL_DEPTH_COMPONENTS: GlEnum = GlEnum(0x8284);
pub const GL_STENCIL_COMPONENTS: GlEnum = GlEnum(0x8285);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_COLOR_RENDERABLE: GlEnum = GlEnum(0x8286);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_DEPTH_RENDERABLE: GlEnum = GlEnum(0x8287);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_STENCIL_RENDERABLE: GlEnum = GlEnum(0x8288);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_FRAMEBUFFER_RENDERABLE: GlEnum = GlEnum(0x8289);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: GlEnum = GlEnum(0x828A);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_FRAMEBUFFER_BLEND: GlEnum = GlEnum(0x828B);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_READ_PIXELS: GlEnum = GlEnum(0x828C);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_READ_PIXELS_FORMAT: GlEnum = GlEnum(0x828D);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_READ_PIXELS_TYPE: GlEnum = GlEnum(0x828E);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_IMAGE_FORMAT: GlEnum = GlEnum(0x828F);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_IMAGE_TYPE: GlEnum = GlEnum(0x8290);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_GET_TEXTURE_IMAGE_FORMAT: GlEnum = GlEnum(0x8291);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_GET_TEXTURE_IMAGE_TYPE: GlEnum = GlEnum(0x8292);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_MIPMAP: GlEnum = GlEnum(0x8293);
pub const GL_MANUAL_GENERATE_MIPMAP: GlEnum = GlEnum(0x8294);
/// Should be deprecated
/// * Group: [`InternalFormatPName`]
pub const GL_AUTO_GENERATE_MIPMAP: GlEnum = GlEnum(0x8295);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_COLOR_ENCODING: GlEnum = GlEnum(0x8296);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SRGB_READ: GlEnum = GlEnum(0x8297);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SRGB_WRITE: GlEnum = GlEnum(0x8298);
pub const GL_SRGB_DECODE_ARB: GlEnum = GlEnum(0x8299);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_FILTER: GlEnum = GlEnum(0x829A);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_VERTEX_TEXTURE: GlEnum = GlEnum(0x829B);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TESS_CONTROL_TEXTURE: GlEnum = GlEnum(0x829C);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TESS_EVALUATION_TEXTURE: GlEnum = GlEnum(0x829D);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_GEOMETRY_TEXTURE: GlEnum = GlEnum(0x829E);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_FRAGMENT_TEXTURE: GlEnum = GlEnum(0x829F);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_COMPUTE_TEXTURE: GlEnum = GlEnum(0x82A0);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_SHADOW: GlEnum = GlEnum(0x82A1);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_GATHER: GlEnum = GlEnum(0x82A2);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_GATHER_SHADOW: GlEnum = GlEnum(0x82A3);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SHADER_IMAGE_LOAD: GlEnum = GlEnum(0x82A4);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SHADER_IMAGE_STORE: GlEnum = GlEnum(0x82A5);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SHADER_IMAGE_ATOMIC: GlEnum = GlEnum(0x82A6);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_TEXEL_SIZE: GlEnum = GlEnum(0x82A7);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_COMPATIBILITY_CLASS: GlEnum = GlEnum(0x82A8);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_PIXEL_FORMAT: GlEnum = GlEnum(0x82A9);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_PIXEL_TYPE: GlEnum = GlEnum(0x82AA);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GlEnum = GlEnum(0x82AC);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GlEnum = GlEnum(0x82AD);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GlEnum = GlEnum(0x82AE);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GlEnum = GlEnum(0x82AF);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: GlEnum = GlEnum(0x82B1);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: GlEnum = GlEnum(0x82B2);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: GlEnum = GlEnum(0x82B3);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_CLEAR_BUFFER: GlEnum = GlEnum(0x82B4);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_VIEW: GlEnum = GlEnum(0x82B5);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_VIEW_COMPATIBILITY_CLASS: GlEnum = GlEnum(0x82B6);
pub const GL_FULL_SUPPORT: GlEnum = GlEnum(0x82B7);
pub const GL_CAVEAT_SUPPORT: GlEnum = GlEnum(0x82B8);
pub const GL_IMAGE_CLASS_4_X_32: GlEnum = GlEnum(0x82B9);
pub const GL_IMAGE_CLASS_2_X_32: GlEnum = GlEnum(0x82BA);
pub const GL_IMAGE_CLASS_1_X_32: GlEnum = GlEnum(0x82BB);
pub const GL_IMAGE_CLASS_4_X_16: GlEnum = GlEnum(0x82BC);
pub const GL_IMAGE_CLASS_2_X_16: GlEnum = GlEnum(0x82BD);
pub const GL_IMAGE_CLASS_1_X_16: GlEnum = GlEnum(0x82BE);
pub const GL_IMAGE_CLASS_4_X_8: GlEnum = GlEnum(0x82BF);
pub const GL_IMAGE_CLASS_2_X_8: GlEnum = GlEnum(0x82C0);
pub const GL_IMAGE_CLASS_1_X_8: GlEnum = GlEnum(0x82C1);
pub const GL_IMAGE_CLASS_11_11_10: GlEnum = GlEnum(0x82C2);
pub const GL_IMAGE_CLASS_10_10_10_2: GlEnum = GlEnum(0x82C3);
pub const GL_VIEW_CLASS_128_BITS: GlEnum = GlEnum(0x82C4);
pub const GL_VIEW_CLASS_96_BITS: GlEnum = GlEnum(0x82C5);
pub const GL_VIEW_CLASS_64_BITS: GlEnum = GlEnum(0x82C6);
pub const GL_VIEW_CLASS_48_BITS: GlEnum = GlEnum(0x82C7);
pub const GL_VIEW_CLASS_32_BITS: GlEnum = GlEnum(0x82C8);
pub const GL_VIEW_CLASS_24_BITS: GlEnum = GlEnum(0x82C9);
pub const GL_VIEW_CLASS_16_BITS: GlEnum = GlEnum(0x82CA);
pub const GL_VIEW_CLASS_8_BITS: GlEnum = GlEnum(0x82CB);
pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: GlEnum = GlEnum(0x82CC);
pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: GlEnum = GlEnum(0x82CD);
pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: GlEnum = GlEnum(0x82CE);
pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: GlEnum = GlEnum(0x82CF);
pub const GL_VIEW_CLASS_RGTC1_RED: GlEnum = GlEnum(0x82D0);
pub const GL_VIEW_CLASS_RGTC2_RG: GlEnum = GlEnum(0x82D1);
pub const GL_VIEW_CLASS_BPTC_UNORM: GlEnum = GlEnum(0x82D2);
pub const GL_VIEW_CLASS_BPTC_FLOAT: GlEnum = GlEnum(0x82D3);
/// 
/// * Group: [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_BINDING: GlEnum = GlEnum(0x82D4);
/// 
/// * Group: [`VertexArrayPName`], [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GlEnum = GlEnum(0x82D5);
/// 
/// * Group: [`GetPName`]
pub const GL_VERTEX_BINDING_DIVISOR: GlEnum = GlEnum(0x82D6);
/// 
/// * Group: [`GetPName`]
pub const GL_VERTEX_BINDING_OFFSET: GlEnum = GlEnum(0x82D7);
/// 
/// * Group: [`GetPName`]
pub const GL_VERTEX_BINDING_STRIDE: GlEnum = GlEnum(0x82D8);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GlEnum = GlEnum(0x82D9);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GlEnum = GlEnum(0x82DA);
pub const GL_TEXTURE_VIEW_MIN_LEVEL: GlEnum = GlEnum(0x82DB);
pub const GL_TEXTURE_VIEW_MIN_LEVEL_EXT: GlEnum = GlEnum(0x82DB);
pub const GL_TEXTURE_VIEW_MIN_LEVEL_OES: GlEnum = GlEnum(0x82DB);
pub const GL_TEXTURE_VIEW_NUM_LEVELS: GlEnum = GlEnum(0x82DC);
pub const GL_TEXTURE_VIEW_NUM_LEVELS_EXT: GlEnum = GlEnum(0x82DC);
pub const GL_TEXTURE_VIEW_NUM_LEVELS_OES: GlEnum = GlEnum(0x82DC);
pub const GL_TEXTURE_VIEW_MIN_LAYER: GlEnum = GlEnum(0x82DD);
pub const GL_TEXTURE_VIEW_MIN_LAYER_EXT: GlEnum = GlEnum(0x82DD);
pub const GL_TEXTURE_VIEW_MIN_LAYER_OES: GlEnum = GlEnum(0x82DD);
pub const GL_TEXTURE_VIEW_NUM_LAYERS: GlEnum = GlEnum(0x82DE);
pub const GL_TEXTURE_VIEW_NUM_LAYERS_EXT: GlEnum = GlEnum(0x82DE);
pub const GL_TEXTURE_VIEW_NUM_LAYERS_OES: GlEnum = GlEnum(0x82DE);
pub const GL_TEXTURE_IMMUTABLE_LEVELS: GlEnum = GlEnum(0x82DF);
/// 
/// * Group: [`ObjectIdentifier`]
pub const GL_BUFFER: GlEnum = GlEnum(0x82E0);
pub const GL_BUFFER_KHR: GlEnum = GlEnum(0x82E0);
/// 
/// * Group: [`ObjectIdentifier`]
pub const GL_SHADER: GlEnum = GlEnum(0x82E1);
pub const GL_SHADER_KHR: GlEnum = GlEnum(0x82E1);
/// 
/// * Group: [`ObjectIdentifier`]
pub const GL_PROGRAM: GlEnum = GlEnum(0x82E2);
pub const GL_PROGRAM_KHR: GlEnum = GlEnum(0x82E2);
/// 
/// * Group: [`ObjectIdentifier`]
pub const GL_QUERY: GlEnum = GlEnum(0x82E3);
pub const GL_QUERY_KHR: GlEnum = GlEnum(0x82E3);
/// 
/// * Group: [`ObjectIdentifier`]
pub const GL_PROGRAM_PIPELINE: GlEnum = GlEnum(0x82E4);
pub const GL_PROGRAM_PIPELINE_KHR: GlEnum = GlEnum(0x82E4);
pub const GL_MAX_VERTEX_ATTRIB_STRIDE: GlEnum = GlEnum(0x82E5);
/// 
/// * Group: [`ObjectIdentifier`]
pub const GL_SAMPLER: GlEnum = GlEnum(0x82E6);
pub const GL_SAMPLER_KHR: GlEnum = GlEnum(0x82E6);
pub const GL_DISPLAY_LIST: GlEnum = GlEnum(0x82E7);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_LABEL_LENGTH: GlEnum = GlEnum(0x82E8);
pub const GL_MAX_LABEL_LENGTH_KHR: GlEnum = GlEnum(0x82E8);
pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: GlEnum = GlEnum(0x82E9);
/// 
/// * Group: [`QueryObjectParameterName`]
pub const GL_QUERY_TARGET: GlEnum = GlEnum(0x82EA);
/// 
/// * Group: [`QueryTarget`]
pub const GL_TRANSFORM_FEEDBACK_OVERFLOW: GlEnum = GlEnum(0x82EC);
/// 
/// * Alias Of: [`GL_TRANSFORM_FEEDBACK_OVERFLOW`]
pub const GL_TRANSFORM_FEEDBACK_OVERFLOW_ARB: GlEnum = GL_TRANSFORM_FEEDBACK_OVERFLOW;
pub const GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW: GlEnum = GlEnum(0x82ED);
/// 
/// * Alias Of: [`GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW`]
pub const GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW_ARB: GlEnum = GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW;
/// 
/// * Group: [`QueryTarget`]
pub const GL_VERTICES_SUBMITTED: GlEnum = GlEnum(0x82EE);
/// 
/// * Alias Of: [`GL_VERTICES_SUBMITTED`]
pub const GL_VERTICES_SUBMITTED_ARB: GlEnum = GL_VERTICES_SUBMITTED;
/// 
/// * Group: [`QueryTarget`]
pub const GL_PRIMITIVES_SUBMITTED: GlEnum = GlEnum(0x82EF);
/// 
/// * Alias Of: [`GL_PRIMITIVES_SUBMITTED`]
pub const GL_PRIMITIVES_SUBMITTED_ARB: GlEnum = GL_PRIMITIVES_SUBMITTED;
/// 
/// * Group: [`QueryTarget`]
pub const GL_VERTEX_SHADER_INVOCATIONS: GlEnum = GlEnum(0x82F0);
/// 
/// * Alias Of: [`GL_VERTEX_SHADER_INVOCATIONS`]
pub const GL_VERTEX_SHADER_INVOCATIONS_ARB: GlEnum = GL_VERTEX_SHADER_INVOCATIONS;
pub const GL_TESS_CONTROL_SHADER_PATCHES: GlEnum = GlEnum(0x82F1);
/// 
/// * Alias Of: [`GL_TESS_CONTROL_SHADER_PATCHES`]
pub const GL_TESS_CONTROL_SHADER_PATCHES_ARB: GlEnum = GL_TESS_CONTROL_SHADER_PATCHES;
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS: GlEnum = GlEnum(0x82F2);
/// 
/// * Alias Of: [`GL_TESS_EVALUATION_SHADER_INVOCATIONS`]
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS_ARB: GlEnum = GL_TESS_EVALUATION_SHADER_INVOCATIONS;
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED: GlEnum = GlEnum(0x82F3);
/// 
/// * Alias Of: [`GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED`]
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED_ARB: GlEnum = GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED;
pub const GL_FRAGMENT_SHADER_INVOCATIONS: GlEnum = GlEnum(0x82F4);
/// 
/// * Alias Of: [`GL_FRAGMENT_SHADER_INVOCATIONS`]
pub const GL_FRAGMENT_SHADER_INVOCATIONS_ARB: GlEnum = GL_FRAGMENT_SHADER_INVOCATIONS;
pub const GL_COMPUTE_SHADER_INVOCATIONS: GlEnum = GlEnum(0x82F5);
/// 
/// * Alias Of: [`GL_COMPUTE_SHADER_INVOCATIONS`]
pub const GL_COMPUTE_SHADER_INVOCATIONS_ARB: GlEnum = GL_COMPUTE_SHADER_INVOCATIONS;
pub const GL_CLIPPING_INPUT_PRIMITIVES: GlEnum = GlEnum(0x82F6);
/// 
/// * Alias Of: [`GL_CLIPPING_INPUT_PRIMITIVES`]
pub const GL_CLIPPING_INPUT_PRIMITIVES_ARB: GlEnum = GL_CLIPPING_INPUT_PRIMITIVES;
pub const GL_CLIPPING_OUTPUT_PRIMITIVES: GlEnum = GlEnum(0x82F7);
/// 
/// * Alias Of: [`GL_CLIPPING_OUTPUT_PRIMITIVES`]
pub const GL_CLIPPING_OUTPUT_PRIMITIVES_ARB: GlEnum = GL_CLIPPING_OUTPUT_PRIMITIVES;
pub const GL_SPARSE_BUFFER_PAGE_SIZE_ARB: GlEnum = GlEnum(0x82F8);
pub const GL_MAX_CULL_DISTANCES: GlEnum = GlEnum(0x82F9);
/// 
/// * Alias Of: [`GL_MAX_CULL_DISTANCES`]
pub const GL_MAX_CULL_DISTANCES_EXT: GlEnum = GL_MAX_CULL_DISTANCES;
pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GlEnum = GlEnum(0x82FA);
/// 
/// * Alias Of: [`GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES`]
pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES_EXT: GlEnum = GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES;
pub const GL_CONTEXT_RELEASE_BEHAVIOR: GlEnum = GlEnum(0x82FB);
pub const GL_CONTEXT_RELEASE_BEHAVIOR_KHR: GlEnum = GlEnum(0x82FB);
pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: GlEnum = GlEnum(0x82FC);
pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_KHR: GlEnum = GlEnum(0x82FC);
/// Reserved for future
pub const GL_ROBUST_GPU_TIMEOUT_MS_KHR: GlEnum = GlEnum(0x82FD);
pub const GL_DEPTH_PASS_INSTRUMENT_SGIX: GlEnum = GlEnum(0x8310);
pub const GL_DEPTH_PASS_INSTRUMENT_COUNTERS_SGIX: GlEnum = GlEnum(0x8311);
pub const GL_DEPTH_PASS_INSTRUMENT_MAX_SGIX: GlEnum = GlEnum(0x8312);
pub const GL_FRAGMENTS_INSTRUMENT_SGIX: GlEnum = GlEnum(0x8313);
pub const GL_FRAGMENTS_INSTRUMENT_COUNTERS_SGIX: GlEnum = GlEnum(0x8314);
pub const GL_FRAGMENTS_INSTRUMENT_MAX_SGIX: GlEnum = GlEnum(0x8315);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_CONVOLUTION_HINT_SGIX: GlEnum = GlEnum(0x8316);
pub const GL_YCRCB_SGIX: GlEnum = GlEnum(0x8318);
pub const GL_YCRCBA_SGIX: GlEnum = GlEnum(0x8319);
pub const GL_UNPACK_COMPRESSED_SIZE_SGIX: GlEnum = GlEnum(0x831A);
pub const GL_PACK_MAX_COMPRESSED_SIZE_SGIX: GlEnum = GlEnum(0x831B);
pub const GL_PACK_COMPRESSED_SIZE_SGIX: GlEnum = GlEnum(0x831C);
pub const GL_SLIM8U_SGIX: GlEnum = GlEnum(0x831D);
pub const GL_SLIM10U_SGIX: GlEnum = GlEnum(0x831E);
pub const GL_SLIM12S_SGIX: GlEnum = GlEnum(0x831F);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_ALPHA_MIN_SGIX: GlEnum = GlEnum(0x8320);
/// 
/// * Group: [`BlendEquationModeEXT`]
pub const GL_ALPHA_MAX_SGIX: GlEnum = GlEnum(0x8321);
/// 
/// * Group: [`HintTarget`]
pub const GL_SCALEBIAS_HINT_SGIX: GlEnum = GlEnum(0x8322);
/// 
/// * Group: [`GetPName`]
pub const GL_ASYNC_MARKER_SGIX: GlEnum = GlEnum(0x8329);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_TEX_GEN_MODE_SGIX: GlEnum = GlEnum(0x832B);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ASYNC_HISTOGRAM_SGIX: GlEnum = GlEnum(0x832C);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ASYNC_HISTOGRAM_SGIX: GlEnum = GlEnum(0x832D);
/// 
/// * Group: [`PixelTransformTargetEXT`]
pub const GL_PIXEL_TRANSFORM_2D_EXT: GlEnum = GlEnum(0x8330);
/// 
/// * Group: [`PixelTransformPNameEXT`]
pub const GL_PIXEL_MAG_FILTER_EXT: GlEnum = GlEnum(0x8331);
/// 
/// * Group: [`PixelTransformPNameEXT`]
pub const GL_PIXEL_MIN_FILTER_EXT: GlEnum = GlEnum(0x8332);
/// 
/// * Group: [`PixelTransformPNameEXT`]
pub const GL_PIXEL_CUBIC_WEIGHT_EXT: GlEnum = GlEnum(0x8333);
pub const GL_CUBIC_EXT: GlEnum = GlEnum(0x8334);
pub const GL_AVERAGE_EXT: GlEnum = GlEnum(0x8335);
pub const GL_PIXEL_TRANSFORM_2D_STACK_DEPTH_EXT: GlEnum = GlEnum(0x8336);
pub const GL_MAX_PIXEL_TRANSFORM_2D_STACK_DEPTH_EXT: GlEnum = GlEnum(0x8337);
pub const GL_PIXEL_TRANSFORM_2D_MATRIX_EXT: GlEnum = GlEnum(0x8338);
/// 
/// * Group: [`LightTextureModeEXT`]
pub const GL_FRAGMENT_MATERIAL_EXT: GlEnum = GlEnum(0x8349);
/// 
/// * Group: [`LightTextureModeEXT`]
pub const GL_FRAGMENT_NORMAL_EXT: GlEnum = GlEnum(0x834A);
/// 
/// * Group: [`LightTextureModeEXT`]
pub const GL_FRAGMENT_COLOR_EXT: GlEnum = GlEnum(0x834C);
/// 
/// * Group: [`LightTexturePNameEXT`]
pub const GL_ATTENUATION_EXT: GlEnum = GlEnum(0x834D);
/// 
/// * Group: [`LightTexturePNameEXT`]
pub const GL_SHADOW_ATTENUATION_EXT: GlEnum = GlEnum(0x834E);
pub const GL_TEXTURE_APPLICATION_MODE_EXT: GlEnum = GlEnum(0x834F);
pub const GL_TEXTURE_LIGHT_EXT: GlEnum = GlEnum(0x8350);
pub const GL_TEXTURE_MATERIAL_FACE_EXT: GlEnum = GlEnum(0x8351);
pub const GL_TEXTURE_MATERIAL_PARAMETER_EXT: GlEnum = GlEnum(0x8352);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_PIXEL_TEXTURE_SGIS: GlEnum = GlEnum(0x8353);
/// 
/// * Group: [`PixelTexGenParameterNameSGIS`]
pub const GL_PIXEL_FRAGMENT_RGB_SOURCE_SGIS: GlEnum = GlEnum(0x8354);
/// 
/// * Group: [`PixelTexGenParameterNameSGIS`]
pub const GL_PIXEL_FRAGMENT_ALPHA_SOURCE_SGIS: GlEnum = GlEnum(0x8355);
pub const GL_PIXEL_GROUP_COLOR_SGIS: GlEnum = GlEnum(0x8356);
/// 
/// * Group: [`HintTarget`]
pub const GL_LINE_QUALITY_HINT_SGIX: GlEnum = GlEnum(0x835B);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ASYNC_TEX_IMAGE_SGIX: GlEnum = GlEnum(0x835C);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ASYNC_DRAW_PIXELS_SGIX: GlEnum = GlEnum(0x835D);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ASYNC_READ_PIXELS_SGIX: GlEnum = GlEnum(0x835E);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ASYNC_TEX_IMAGE_SGIX: GlEnum = GlEnum(0x835F);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ASYNC_DRAW_PIXELS_SGIX: GlEnum = GlEnum(0x8360);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ASYNC_READ_PIXELS_SGIX: GlEnum = GlEnum(0x8361);
pub const GL_UNSIGNED_BYTE_2_3_3_REV: GlEnum = GlEnum(0x8362);
pub const GL_UNSIGNED_BYTE_2_3_3_REV_EXT: GlEnum = GlEnum(0x8362);
pub const GL_UNSIGNED_SHORT_5_6_5: GlEnum = GlEnum(0x8363);
pub const GL_UNSIGNED_SHORT_5_6_5_EXT: GlEnum = GlEnum(0x8363);
pub const GL_UNSIGNED_SHORT_5_6_5_REV: GlEnum = GlEnum(0x8364);
pub const GL_UNSIGNED_SHORT_5_6_5_REV_EXT: GlEnum = GlEnum(0x8364);
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: GlEnum = GlEnum(0x8365);
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV_EXT: GlEnum = GlEnum(0x8365);
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV_IMG: GlEnum = GlEnum(0x8365);
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: GlEnum = GlEnum(0x8366);
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV_EXT: GlEnum = GlEnum(0x8366);
pub const GL_UNSIGNED_INT_8_8_8_8_REV: GlEnum = GlEnum(0x8367);
pub const GL_UNSIGNED_INT_8_8_8_8_REV_EXT: GlEnum = GlEnum(0x8367);
/// 
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_UNSIGNED_INT_2_10_10_10_REV: GlEnum = GlEnum(0x8368);
pub const GL_UNSIGNED_INT_2_10_10_10_REV_EXT: GlEnum = GlEnum(0x8368);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_CLAMP_S_SGIX: GlEnum = GlEnum(0x8369);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_CLAMP_T_SGIX: GlEnum = GlEnum(0x836A);
/// 
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_CLAMP_R_SGIX: GlEnum = GlEnum(0x836B);
/// 
/// * Group: [`TextureWrapMode`]
pub const GL_MIRRORED_REPEAT: GlEnum = GlEnum(0x8370);
pub const GL_MIRRORED_REPEAT_ARB: GlEnum = GlEnum(0x8370);
pub const GL_MIRRORED_REPEAT_IBM: GlEnum = GlEnum(0x8370);
pub const GL_MIRRORED_REPEAT_OES: GlEnum = GlEnum(0x8370);
pub const GL_RGB_S3TC: GlEnum = GlEnum(0x83A0);
pub const GL_RGB4_S3TC: GlEnum = GlEnum(0x83A1);
pub const GL_RGBA_S3TC: GlEnum = GlEnum(0x83A2);
pub const GL_RGBA4_S3TC: GlEnum = GlEnum(0x83A3);
pub const GL_RGBA_DXT5_S3TC: GlEnum = GlEnum(0x83A4);
pub const GL_RGBA4_DXT5_S3TC: GlEnum = GlEnum(0x83A5);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_VERTEX_PRECLIP_SGIX: GlEnum = GlEnum(0x83EE);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_VERTEX_PRECLIP_HINT_SGIX: GlEnum = GlEnum(0x83EF);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: GlEnum = GlEnum(0x83F0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: GlEnum = GlEnum(0x83F1);
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_ANGLE: GlEnum = GlEnum(0x83F2);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: GlEnum = GlEnum(0x83F2);
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_ANGLE: GlEnum = GlEnum(0x83F3);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: GlEnum = GlEnum(0x83F3);
pub const GL_PARALLEL_ARRAYS_INTEL: GlEnum = GlEnum(0x83F4);
pub const GL_VERTEX_ARRAY_PARALLEL_POINTERS_INTEL: GlEnum = GlEnum(0x83F5);
pub const GL_NORMAL_ARRAY_PARALLEL_POINTERS_INTEL: GlEnum = GlEnum(0x83F6);
pub const GL_COLOR_ARRAY_PARALLEL_POINTERS_INTEL: GlEnum = GlEnum(0x83F7);
pub const GL_TEXTURE_COORD_ARRAY_PARALLEL_POINTERS_INTEL: GlEnum = GlEnum(0x83F8);
pub const GL_PERFQUERY_DONOT_FLUSH_INTEL: GlEnum = GlEnum(0x83F9);
pub const GL_PERFQUERY_FLUSH_INTEL: GlEnum = GlEnum(0x83FA);
pub const GL_PERFQUERY_WAIT_INTEL: GlEnum = GlEnum(0x83FB);
pub const GL_BLACKHOLE_RENDER_INTEL: GlEnum = GlEnum(0x83FC);
pub const GL_CONSERVATIVE_RASTERIZATION_INTEL: GlEnum = GlEnum(0x83FE);
pub const GL_TEXTURE_MEMORY_LAYOUT_INTEL: GlEnum = GlEnum(0x83FF);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHTING_SGIX: GlEnum = GlEnum(0x8400);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FRAGMENT_COLOR_MATERIAL_SGIX: GlEnum = GlEnum(0x8401);
/// 
/// * Group: [`GetPName`]
pub const GL_FRAGMENT_COLOR_MATERIAL_FACE_SGIX: GlEnum = GlEnum(0x8402);
/// 
/// * Group: [`GetPName`]
pub const GL_FRAGMENT_COLOR_MATERIAL_PARAMETER_SGIX: GlEnum = GlEnum(0x8403);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_LIGHTS_SGIX: GlEnum = GlEnum(0x8404);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ACTIVE_LIGHTS_SGIX: GlEnum = GlEnum(0x8405);
pub const GL_CURRENT_RASTER_NORMAL_SGIX: GlEnum = GlEnum(0x8406);
/// 
/// * Group: [`LightEnvParameterSGIX`], [`GetPName`]
pub const GL_LIGHT_ENV_MODE_SGIX: GlEnum = GlEnum(0x8407);
/// 
/// * Group: [`GetPName`], [`FragmentLightModelParameterSGIX`]
pub const GL_FRAGMENT_LIGHT_MODEL_LOCAL_VIEWER_SGIX: GlEnum = GlEnum(0x8408);
/// 
/// * Group: [`GetPName`], [`FragmentLightModelParameterSGIX`]
pub const GL_FRAGMENT_LIGHT_MODEL_TWO_SIDE_SGIX: GlEnum = GlEnum(0x8409);
/// 
/// * Group: [`GetPName`], [`FragmentLightModelParameterSGIX`]
pub const GL_FRAGMENT_LIGHT_MODEL_AMBIENT_SGIX: GlEnum = GlEnum(0x840A);
/// 
/// * Group: [`GetPName`], [`FragmentLightModelParameterSGIX`]
pub const GL_FRAGMENT_LIGHT_MODEL_NORMAL_INTERPOLATION_SGIX: GlEnum = GlEnum(0x840B);
/// 
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`],
///   [`GetPName`]
pub const GL_FRAGMENT_LIGHT0_SGIX: GlEnum = GlEnum(0x840C);
/// 
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT1_SGIX: GlEnum = GlEnum(0x840D);
/// 
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT2_SGIX: GlEnum = GlEnum(0x840E);
/// 
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT3_SGIX: GlEnum = GlEnum(0x840F);
/// 
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT4_SGIX: GlEnum = GlEnum(0x8410);
/// 
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT5_SGIX: GlEnum = GlEnum(0x8411);
/// 
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT6_SGIX: GlEnum = GlEnum(0x8412);
/// 
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT7_SGIX: GlEnum = GlEnum(0x8413);
/// Formerly 0x842C in SGI specfile
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_RESAMPLE_SGIX: GlEnum = GlEnum(0x842E);
/// Formerly 0x842D in SGI specfile
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_RESAMPLE_SGIX: GlEnum = GlEnum(0x842F);
/// Formerly 0x8430 in SGI specfile
/// * Group: [`PixelStoreResampleMode`]
pub const GL_RESAMPLE_DECIMATE_SGIX: GlEnum = GlEnum(0x8430);
/// Formerly 0x842E in SGI specfile
/// * Group: [`PixelStoreResampleMode`]
pub const GL_RESAMPLE_REPLICATE_SGIX: GlEnum = GlEnum(0x8433);
/// Formerly 0x842F in SGI specfile
/// * Group: [`PixelStoreResampleMode`]
pub const GL_RESAMPLE_ZERO_FILL_SGIX: GlEnum = GlEnum(0x8434);
pub const GL_TANGENT_ARRAY_EXT: GlEnum = GlEnum(0x8439);
pub const GL_BINORMAL_ARRAY_EXT: GlEnum = GlEnum(0x843A);
pub const GL_CURRENT_TANGENT_EXT: GlEnum = GlEnum(0x843B);
pub const GL_CURRENT_BINORMAL_EXT: GlEnum = GlEnum(0x843C);
pub const GL_TANGENT_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x843E);
pub const GL_TANGENT_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x843F);
pub const GL_BINORMAL_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x8440);
pub const GL_BINORMAL_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x8441);
pub const GL_TANGENT_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x8442);
pub const GL_BINORMAL_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x8443);
pub const GL_MAP1_TANGENT_EXT: GlEnum = GlEnum(0x8444);
pub const GL_MAP2_TANGENT_EXT: GlEnum = GlEnum(0x8445);
pub const GL_MAP1_BINORMAL_EXT: GlEnum = GlEnum(0x8446);
pub const GL_MAP2_BINORMAL_EXT: GlEnum = GlEnum(0x8447);
/// 
/// * Group: [`TextureMinFilter`]
pub const GL_NEAREST_CLIPMAP_NEAREST_SGIX: GlEnum = GlEnum(0x844D);
/// 
/// * Group: [`TextureMinFilter`]
pub const GL_NEAREST_CLIPMAP_LINEAR_SGIX: GlEnum = GlEnum(0x844E);
/// 
/// * Group: [`TextureMinFilter`]
pub const GL_LINEAR_CLIPMAP_NEAREST_SGIX: GlEnum = GlEnum(0x844F);
pub const GL_FOG_COORDINATE_SOURCE: GlEnum = GlEnum(0x8450);
pub const GL_FOG_COORDINATE_SOURCE_EXT: GlEnum = GlEnum(0x8450);
/// 
/// * Group: [`FogPName`]
/// * Alias Of: [`GL_FOG_COORDINATE_SOURCE`]
pub const GL_FOG_COORD_SRC: GlEnum = GL_FOG_COORDINATE_SOURCE;
pub const GL_FOG_COORDINATE: GlEnum = GlEnum(0x8451);
/// 
/// * Alias Of: [`GL_FOG_COORDINATE`]
pub const GL_FOG_COORD: GlEnum = GL_FOG_COORDINATE;
pub const GL_FOG_COORDINATE_EXT: GlEnum = GlEnum(0x8451);
pub const GL_FRAGMENT_DEPTH: GlEnum = GlEnum(0x8452);
/// 
/// * Group: [`LightTextureModeEXT`]
pub const GL_FRAGMENT_DEPTH_EXT: GlEnum = GlEnum(0x8452);
pub const GL_CURRENT_FOG_COORDINATE: GlEnum = GlEnum(0x8453);
/// 
/// * Alias Of: [`GL_CURRENT_FOG_COORDINATE`]
pub const GL_CURRENT_FOG_COORD: GlEnum = GL_CURRENT_FOG_COORDINATE;
pub const GL_CURRENT_FOG_COORDINATE_EXT: GlEnum = GlEnum(0x8453);
pub const GL_FOG_COORDINATE_ARRAY_TYPE: GlEnum = GlEnum(0x8454);
pub const GL_FOG_COORDINATE_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x8454);
/// 
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY_TYPE`]
pub const GL_FOG_COORD_ARRAY_TYPE: GlEnum = GL_FOG_COORDINATE_ARRAY_TYPE;
pub const GL_FOG_COORDINATE_ARRAY_STRIDE: GlEnum = GlEnum(0x8455);
pub const GL_FOG_COORDINATE_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x8455);
/// 
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY_STRIDE`]
pub const GL_FOG_COORD_ARRAY_STRIDE: GlEnum = GL_FOG_COORDINATE_ARRAY_STRIDE;
pub const GL_FOG_COORDINATE_ARRAY_POINTER: GlEnum = GlEnum(0x8456);
pub const GL_FOG_COORDINATE_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x8456);
/// 
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY_POINTER`]
pub const GL_FOG_COORD_ARRAY_POINTER: GlEnum = GL_FOG_COORDINATE_ARRAY_POINTER;
pub const GL_FOG_COORDINATE_ARRAY: GlEnum = GlEnum(0x8457);
pub const GL_FOG_COORDINATE_ARRAY_EXT: GlEnum = GlEnum(0x8457);
/// 
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY`]
pub const GL_FOG_COORD_ARRAY: GlEnum = GL_FOG_COORDINATE_ARRAY;
pub const GL_COLOR_SUM: GlEnum = GlEnum(0x8458);
pub const GL_COLOR_SUM_ARB: GlEnum = GlEnum(0x8458);
pub const GL_COLOR_SUM_EXT: GlEnum = GlEnum(0x8458);
pub const GL_CURRENT_SECONDARY_COLOR: GlEnum = GlEnum(0x8459);
pub const GL_CURRENT_SECONDARY_COLOR_EXT: GlEnum = GlEnum(0x8459);
pub const GL_SECONDARY_COLOR_ARRAY_SIZE: GlEnum = GlEnum(0x845A);
pub const GL_SECONDARY_COLOR_ARRAY_SIZE_EXT: GlEnum = GlEnum(0x845A);
pub const GL_SECONDARY_COLOR_ARRAY_TYPE: GlEnum = GlEnum(0x845B);
pub const GL_SECONDARY_COLOR_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x845B);
pub const GL_SECONDARY_COLOR_ARRAY_STRIDE: GlEnum = GlEnum(0x845C);
pub const GL_SECONDARY_COLOR_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x845C);
pub const GL_SECONDARY_COLOR_ARRAY_POINTER: GlEnum = GlEnum(0x845D);
pub const GL_SECONDARY_COLOR_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x845D);
pub const GL_SECONDARY_COLOR_ARRAY: GlEnum = GlEnum(0x845E);
pub const GL_SECONDARY_COLOR_ARRAY_EXT: GlEnum = GlEnum(0x845E);
pub const GL_CURRENT_RASTER_SECONDARY_COLOR: GlEnum = GlEnum(0x845F);
/// 
/// * Group: [`GetPName`]
pub const GL_ALIASED_POINT_SIZE_RANGE: GlEnum = GlEnum(0x846D);
/// 
/// * Group: [`GetPName`]
pub const GL_ALIASED_LINE_WIDTH_RANGE: GlEnum = GlEnum(0x846E);
pub const GL_SCREEN_COORDINATES_REND: GlEnum = GlEnum(0x8490);
pub const GL_INVERTED_SCREEN_W_REND: GlEnum = GlEnum(0x8491);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE0: GlEnum = GlEnum(0x84C0);
/// 
/// * Group: [`CombinerRegisterNV`]
pub const GL_TEXTURE0_ARB: GlEnum = GlEnum(0x84C0);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE1: GlEnum = GlEnum(0x84C1);
/// 
/// * Group: [`CombinerRegisterNV`]
pub const GL_TEXTURE1_ARB: GlEnum = GlEnum(0x84C1);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE2: GlEnum = GlEnum(0x84C2);
pub const GL_TEXTURE2_ARB: GlEnum = GlEnum(0x84C2);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE3: GlEnum = GlEnum(0x84C3);
pub const GL_TEXTURE3_ARB: GlEnum = GlEnum(0x84C3);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE4: GlEnum = GlEnum(0x84C4);
pub const GL_TEXTURE4_ARB: GlEnum = GlEnum(0x84C4);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE5: GlEnum = GlEnum(0x84C5);
pub const GL_TEXTURE5_ARB: GlEnum = GlEnum(0x84C5);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE6: GlEnum = GlEnum(0x84C6);
pub const GL_TEXTURE6_ARB: GlEnum = GlEnum(0x84C6);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE7: GlEnum = GlEnum(0x84C7);
pub const GL_TEXTURE7_ARB: GlEnum = GlEnum(0x84C7);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE8: GlEnum = GlEnum(0x84C8);
pub const GL_TEXTURE8_ARB: GlEnum = GlEnum(0x84C8);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE9: GlEnum = GlEnum(0x84C9);
pub const GL_TEXTURE9_ARB: GlEnum = GlEnum(0x84C9);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE10: GlEnum = GlEnum(0x84CA);
pub const GL_TEXTURE10_ARB: GlEnum = GlEnum(0x84CA);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE11: GlEnum = GlEnum(0x84CB);
pub const GL_TEXTURE11_ARB: GlEnum = GlEnum(0x84CB);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE12: GlEnum = GlEnum(0x84CC);
pub const GL_TEXTURE12_ARB: GlEnum = GlEnum(0x84CC);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE13: GlEnum = GlEnum(0x84CD);
pub const GL_TEXTURE13_ARB: GlEnum = GlEnum(0x84CD);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE14: GlEnum = GlEnum(0x84CE);
pub const GL_TEXTURE14_ARB: GlEnum = GlEnum(0x84CE);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE15: GlEnum = GlEnum(0x84CF);
pub const GL_TEXTURE15_ARB: GlEnum = GlEnum(0x84CF);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE16: GlEnum = GlEnum(0x84D0);
pub const GL_TEXTURE16_ARB: GlEnum = GlEnum(0x84D0);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE17: GlEnum = GlEnum(0x84D1);
pub const GL_TEXTURE17_ARB: GlEnum = GlEnum(0x84D1);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE18: GlEnum = GlEnum(0x84D2);
pub const GL_TEXTURE18_ARB: GlEnum = GlEnum(0x84D2);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE19: GlEnum = GlEnum(0x84D3);
pub const GL_TEXTURE19_ARB: GlEnum = GlEnum(0x84D3);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE20: GlEnum = GlEnum(0x84D4);
pub const GL_TEXTURE20_ARB: GlEnum = GlEnum(0x84D4);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE21: GlEnum = GlEnum(0x84D5);
pub const GL_TEXTURE21_ARB: GlEnum = GlEnum(0x84D5);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE22: GlEnum = GlEnum(0x84D6);
pub const GL_TEXTURE22_ARB: GlEnum = GlEnum(0x84D6);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE23: GlEnum = GlEnum(0x84D7);
pub const GL_TEXTURE23_ARB: GlEnum = GlEnum(0x84D7);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE24: GlEnum = GlEnum(0x84D8);
pub const GL_TEXTURE24_ARB: GlEnum = GlEnum(0x84D8);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE25: GlEnum = GlEnum(0x84D9);
pub const GL_TEXTURE25_ARB: GlEnum = GlEnum(0x84D9);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE26: GlEnum = GlEnum(0x84DA);
pub const GL_TEXTURE26_ARB: GlEnum = GlEnum(0x84DA);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE27: GlEnum = GlEnum(0x84DB);
pub const GL_TEXTURE27_ARB: GlEnum = GlEnum(0x84DB);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE28: GlEnum = GlEnum(0x84DC);
pub const GL_TEXTURE28_ARB: GlEnum = GlEnum(0x84DC);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE29: GlEnum = GlEnum(0x84DD);
pub const GL_TEXTURE29_ARB: GlEnum = GlEnum(0x84DD);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE30: GlEnum = GlEnum(0x84DE);
pub const GL_TEXTURE30_ARB: GlEnum = GlEnum(0x84DE);
/// 
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE31: GlEnum = GlEnum(0x84DF);
pub const GL_TEXTURE31_ARB: GlEnum = GlEnum(0x84DF);
/// 
/// * Group: [`GetPName`]
pub const GL_ACTIVE_TEXTURE: GlEnum = GlEnum(0x84E0);
pub const GL_ACTIVE_TEXTURE_ARB: GlEnum = GlEnum(0x84E0);
pub const GL_CLIENT_ACTIVE_TEXTURE: GlEnum = GlEnum(0x84E1);
pub const GL_CLIENT_ACTIVE_TEXTURE_ARB: GlEnum = GlEnum(0x84E1);
pub const GL_MAX_TEXTURE_UNITS: GlEnum = GlEnum(0x84E2);
pub const GL_MAX_TEXTURE_UNITS_ARB: GlEnum = GlEnum(0x84E2);
pub const GL_TRANSPOSE_MODELVIEW_MATRIX: GlEnum = GlEnum(0x84E3);
pub const GL_TRANSPOSE_MODELVIEW_MATRIX_ARB: GlEnum = GlEnum(0x84E3);
pub const GL_PATH_TRANSPOSE_MODELVIEW_MATRIX_NV: GlEnum = GlEnum(0x84E3);
pub const GL_TRANSPOSE_PROJECTION_MATRIX: GlEnum = GlEnum(0x84E4);
pub const GL_TRANSPOSE_PROJECTION_MATRIX_ARB: GlEnum = GlEnum(0x84E4);
pub const GL_PATH_TRANSPOSE_PROJECTION_MATRIX_NV: GlEnum = GlEnum(0x84E4);
pub const GL_TRANSPOSE_TEXTURE_MATRIX: GlEnum = GlEnum(0x84E5);
pub const GL_TRANSPOSE_TEXTURE_MATRIX_ARB: GlEnum = GlEnum(0x84E5);
pub const GL_TRANSPOSE_COLOR_MATRIX: GlEnum = GlEnum(0x84E6);
pub const GL_TRANSPOSE_COLOR_MATRIX_ARB: GlEnum = GlEnum(0x84E6);
pub const GL_SUBTRACT: GlEnum = GlEnum(0x84E7);
pub const GL_SUBTRACT_ARB: GlEnum = GlEnum(0x84E7);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_RENDERBUFFER_SIZE: GlEnum = GlEnum(0x84E8);
pub const GL_MAX_RENDERBUFFER_SIZE_EXT: GlEnum = GlEnum(0x84E8);
pub const GL_MAX_RENDERBUFFER_SIZE_OES: GlEnum = GlEnum(0x84E8);
pub const GL_COMPRESSED_ALPHA: GlEnum = GlEnum(0x84E9);
pub const GL_COMPRESSED_ALPHA_ARB: GlEnum = GlEnum(0x84E9);
pub const GL_COMPRESSED_LUMINANCE: GlEnum = GlEnum(0x84EA);
pub const GL_COMPRESSED_LUMINANCE_ARB: GlEnum = GlEnum(0x84EA);
pub const GL_COMPRESSED_LUMINANCE_ALPHA: GlEnum = GlEnum(0x84EB);
pub const GL_COMPRESSED_LUMINANCE_ALPHA_ARB: GlEnum = GlEnum(0x84EB);
pub const GL_COMPRESSED_INTENSITY: GlEnum = GlEnum(0x84EC);
pub const GL_COMPRESSED_INTENSITY_ARB: GlEnum = GlEnum(0x84EC);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB: GlEnum = GlEnum(0x84ED);
pub const GL_COMPRESSED_RGB_ARB: GlEnum = GlEnum(0x84ED);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA: GlEnum = GlEnum(0x84EE);
pub const GL_COMPRESSED_RGBA_ARB: GlEnum = GlEnum(0x84EE);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_TEXTURE_COMPRESSION_HINT: GlEnum = GlEnum(0x84EF);
/// 
/// * Group: [`HintTarget`]
pub const GL_TEXTURE_COMPRESSION_HINT_ARB: GlEnum = GlEnum(0x84EF);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GlEnum = GlEnum(0x84F0);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GlEnum = GlEnum(0x84F1);
/// 
/// * Group: [`FenceConditionNV`]
pub const GL_ALL_COMPLETED_NV: GlEnum = GlEnum(0x84F2);
/// 
/// * Group: [`FenceParameterNameNV`]
pub const GL_FENCE_STATUS_NV: GlEnum = GlEnum(0x84F3);
/// 
/// * Group: [`FenceParameterNameNV`]
pub const GL_FENCE_CONDITION_NV: GlEnum = GlEnum(0x84F4);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_RECTANGLE: GlEnum = GlEnum(0x84F5);
pub const GL_TEXTURE_RECTANGLE_ARB: GlEnum = GlEnum(0x84F5);
pub const GL_TEXTURE_RECTANGLE_NV: GlEnum = GlEnum(0x84F5);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_RECTANGLE: GlEnum = GlEnum(0x84F6);
pub const GL_TEXTURE_BINDING_RECTANGLE_ARB: GlEnum = GlEnum(0x84F6);
pub const GL_TEXTURE_BINDING_RECTANGLE_NV: GlEnum = GlEnum(0x84F6);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_RECTANGLE: GlEnum = GlEnum(0x84F7);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_RECTANGLE_ARB: GlEnum = GlEnum(0x84F7);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_RECTANGLE_NV: GlEnum = GlEnum(0x84F7);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: GlEnum = GlEnum(0x84F8);
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE_ARB: GlEnum = GlEnum(0x84F8);
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE_NV: GlEnum = GlEnum(0x84F8);
/// 
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_DEPTH_STENCIL: GlEnum = GlEnum(0x84F9);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_STENCIL_EXT: GlEnum = GlEnum(0x84F9);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_STENCIL_NV: GlEnum = GlEnum(0x84F9);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_STENCIL_OES: GlEnum = GlEnum(0x84F9);
pub const GL_UNSIGNED_INT_24_8: GlEnum = GlEnum(0x84FA);
pub const GL_UNSIGNED_INT_24_8_EXT: GlEnum = GlEnum(0x84FA);
pub const GL_UNSIGNED_INT_24_8_NV: GlEnum = GlEnum(0x84FA);
pub const GL_UNSIGNED_INT_24_8_OES: GlEnum = GlEnum(0x84FA);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_LOD_BIAS: GlEnum = GlEnum(0x84FD);
pub const GL_MAX_TEXTURE_LOD_BIAS_EXT: GlEnum = GlEnum(0x84FD);
/// 
/// * Group: [`SamplerParameterF`]
pub const GL_TEXTURE_MAX_ANISOTROPY: GlEnum = GlEnum(0x84FE);
/// 
/// * Alias Of: [`GL_TEXTURE_MAX_ANISOTROPY`]
pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: GlEnum = GL_TEXTURE_MAX_ANISOTROPY;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: GlEnum = GlEnum(0x84FF);
/// 
/// * Alias Of: [`GL_MAX_TEXTURE_MAX_ANISOTROPY`]
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: GlEnum = GL_MAX_TEXTURE_MAX_ANISOTROPY;
pub const GL_TEXTURE_FILTER_CONTROL: GlEnum = GlEnum(0x8500);
pub const GL_TEXTURE_FILTER_CONTROL_EXT: GlEnum = GlEnum(0x8500);
/// 
/// * Group: [`TextureParameterName`], [`SamplerParameterF`]
pub const GL_TEXTURE_LOD_BIAS: GlEnum = GlEnum(0x8501);
pub const GL_TEXTURE_LOD_BIAS_EXT: GlEnum = GlEnum(0x8501);
pub const GL_MODELVIEW1_STACK_DEPTH_EXT: GlEnum = GlEnum(0x8502);
pub const GL_COMBINE4_NV: GlEnum = GlEnum(0x8503);
pub const GL_MAX_SHININESS_NV: GlEnum = GlEnum(0x8504);
pub const GL_MAX_SPOT_EXPONENT_NV: GlEnum = GlEnum(0x8505);
pub const GL_MODELVIEW1_MATRIX_EXT: GlEnum = GlEnum(0x8506);
/// 
/// * Group: [`StencilOp`]
pub const GL_INCR_WRAP: GlEnum = GlEnum(0x8507);
pub const GL_INCR_WRAP_EXT: GlEnum = GlEnum(0x8507);
pub const GL_INCR_WRAP_OES: GlEnum = GlEnum(0x8507);
/// 
/// * Group: [`StencilOp`]
pub const GL_DECR_WRAP: GlEnum = GlEnum(0x8508);
pub const GL_DECR_WRAP_EXT: GlEnum = GlEnum(0x8508);
pub const GL_DECR_WRAP_OES: GlEnum = GlEnum(0x8508);
pub const GL_VERTEX_WEIGHTING_EXT: GlEnum = GlEnum(0x8509);
pub const GL_MODELVIEW1_ARB: GlEnum = GlEnum(0x850A);
pub const GL_MODELVIEW1_EXT: GlEnum = GlEnum(0x850A);
pub const GL_CURRENT_VERTEX_WEIGHT_EXT: GlEnum = GlEnum(0x850B);
pub const GL_VERTEX_WEIGHT_ARRAY_EXT: GlEnum = GlEnum(0x850C);
pub const GL_VERTEX_WEIGHT_ARRAY_SIZE_EXT: GlEnum = GlEnum(0x850D);
pub const GL_VERTEX_WEIGHT_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x850E);
pub const GL_VERTEX_WEIGHT_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x850F);
pub const GL_VERTEX_WEIGHT_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x8510);
pub const GL_NORMAL_MAP: GlEnum = GlEnum(0x8511);
pub const GL_NORMAL_MAP_ARB: GlEnum = GlEnum(0x8511);
pub const GL_NORMAL_MAP_EXT: GlEnum = GlEnum(0x8511);
pub const GL_NORMAL_MAP_NV: GlEnum = GlEnum(0x8511);
pub const GL_NORMAL_MAP_OES: GlEnum = GlEnum(0x8511);
pub const GL_REFLECTION_MAP: GlEnum = GlEnum(0x8512);
pub const GL_REFLECTION_MAP_ARB: GlEnum = GlEnum(0x8512);
pub const GL_REFLECTION_MAP_EXT: GlEnum = GlEnum(0x8512);
pub const GL_REFLECTION_MAP_NV: GlEnum = GlEnum(0x8512);
pub const GL_REFLECTION_MAP_OES: GlEnum = GlEnum(0x8512);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP: GlEnum = GlEnum(0x8513);
pub const GL_TEXTURE_CUBE_MAP_ARB: GlEnum = GlEnum(0x8513);
pub const GL_TEXTURE_CUBE_MAP_EXT: GlEnum = GlEnum(0x8513);
pub const GL_TEXTURE_CUBE_MAP_OES: GlEnum = GlEnum(0x8513);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_CUBE_MAP: GlEnum = GlEnum(0x8514);
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARB: GlEnum = GlEnum(0x8514);
pub const GL_TEXTURE_BINDING_CUBE_MAP_EXT: GlEnum = GlEnum(0x8514);
pub const GL_TEXTURE_BINDING_CUBE_MAP_OES: GlEnum = GlEnum(0x8514);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GlEnum = GlEnum(0x8515);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_ARB: GlEnum = GlEnum(0x8515);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_EXT: GlEnum = GlEnum(0x8515);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_OES: GlEnum = GlEnum(0x8515);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GlEnum = GlEnum(0x8516);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_ARB: GlEnum = GlEnum(0x8516);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_EXT: GlEnum = GlEnum(0x8516);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_OES: GlEnum = GlEnum(0x8516);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GlEnum = GlEnum(0x8517);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_ARB: GlEnum = GlEnum(0x8517);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_EXT: GlEnum = GlEnum(0x8517);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_OES: GlEnum = GlEnum(0x8517);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GlEnum = GlEnum(0x8518);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_ARB: GlEnum = GlEnum(0x8518);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_EXT: GlEnum = GlEnum(0x8518);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_OES: GlEnum = GlEnum(0x8518);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GlEnum = GlEnum(0x8519);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_ARB: GlEnum = GlEnum(0x8519);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_EXT: GlEnum = GlEnum(0x8519);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_OES: GlEnum = GlEnum(0x8519);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GlEnum = GlEnum(0x851A);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_ARB: GlEnum = GlEnum(0x851A);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_EXT: GlEnum = GlEnum(0x851A);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_OES: GlEnum = GlEnum(0x851A);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP: GlEnum = GlEnum(0x851B);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARB: GlEnum = GlEnum(0x851B);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP_EXT: GlEnum = GlEnum(0x851B);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GlEnum = GlEnum(0x851C);
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_ARB: GlEnum = GlEnum(0x851C);
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_EXT: GlEnum = GlEnum(0x851C);
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_OES: GlEnum = GlEnum(0x851C);
pub const GL_VERTEX_ARRAY_RANGE_APPLE: GlEnum = GlEnum(0x851D);
pub const GL_VERTEX_ARRAY_RANGE_NV: GlEnum = GlEnum(0x851D);
pub const GL_VERTEX_ARRAY_RANGE_LENGTH_APPLE: GlEnum = GlEnum(0x851E);
pub const GL_VERTEX_ARRAY_RANGE_LENGTH_NV: GlEnum = GlEnum(0x851E);
pub const GL_VERTEX_ARRAY_RANGE_VALID_NV: GlEnum = GlEnum(0x851F);
/// 
/// * Group: [`HintTarget`]
pub const GL_VERTEX_ARRAY_STORAGE_HINT_APPLE: GlEnum = GlEnum(0x851F);
pub const GL_MAX_VERTEX_ARRAY_RANGE_ELEMENT_NV: GlEnum = GlEnum(0x8520);
pub const GL_VERTEX_ARRAY_RANGE_POINTER_APPLE: GlEnum = GlEnum(0x8521);
pub const GL_VERTEX_ARRAY_RANGE_POINTER_NV: GlEnum = GlEnum(0x8521);
pub const GL_REGISTER_COMBINERS_NV: GlEnum = GlEnum(0x8522);
/// 
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_A_NV: GlEnum = GlEnum(0x8523);
/// 
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_B_NV: GlEnum = GlEnum(0x8524);
/// 
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_C_NV: GlEnum = GlEnum(0x8525);
/// 
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_D_NV: GlEnum = GlEnum(0x8526);
/// 
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_E_NV: GlEnum = GlEnum(0x8527);
/// 
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_F_NV: GlEnum = GlEnum(0x8528);
/// 
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_G_NV: GlEnum = GlEnum(0x8529);
pub const GL_CONSTANT_COLOR0_NV: GlEnum = GlEnum(0x852A);
pub const GL_CONSTANT_COLOR1_NV: GlEnum = GlEnum(0x852B);
/// 
/// * Group: [`PathColor`], [`CombinerRegisterNV`]
pub const GL_PRIMARY_COLOR_NV: GlEnum = GlEnum(0x852C);
/// 
/// * Group: [`PathColor`], [`CombinerRegisterNV`]
pub const GL_SECONDARY_COLOR_NV: GlEnum = GlEnum(0x852D);
/// 
/// * Group: [`CombinerRegisterNV`]
pub const GL_SPARE0_NV: GlEnum = GlEnum(0x852E);
/// 
/// * Group: [`CombinerRegisterNV`]
pub const GL_SPARE1_NV: GlEnum = GlEnum(0x852F);
/// 
/// * Group: [`CombinerRegisterNV`]
pub const GL_DISCARD_NV: GlEnum = GlEnum(0x8530);
pub const GL_E_TIMES_F_NV: GlEnum = GlEnum(0x8531);
pub const GL_SPARE0_PLUS_SECONDARY_COLOR_NV: GlEnum = GlEnum(0x8532);
pub const GL_VERTEX_ARRAY_RANGE_WITHOUT_FLUSH_NV: GlEnum = GlEnum(0x8533);
/// 
/// * Group: [`HintTarget`]
pub const GL_MULTISAMPLE_FILTER_HINT_NV: GlEnum = GlEnum(0x8534);
pub const GL_PER_STAGE_CONSTANTS_NV: GlEnum = GlEnum(0x8535);
/// 
/// * Group: [`CombinerMappingNV`]
pub const GL_UNSIGNED_IDENTITY_NV: GlEnum = GlEnum(0x8536);
/// 
/// * Group: [`CombinerMappingNV`]
pub const GL_UNSIGNED_INVERT_NV: GlEnum = GlEnum(0x8537);
/// 
/// * Group: [`CombinerMappingNV`]
pub const GL_EXPAND_NORMAL_NV: GlEnum = GlEnum(0x8538);
/// 
/// * Group: [`CombinerMappingNV`]
pub const GL_EXPAND_NEGATE_NV: GlEnum = GlEnum(0x8539);
/// 
/// * Group: [`CombinerMappingNV`]
pub const GL_HALF_BIAS_NORMAL_NV: GlEnum = GlEnum(0x853A);
/// 
/// * Group: [`CombinerMappingNV`]
pub const GL_HALF_BIAS_NEGATE_NV: GlEnum = GlEnum(0x853B);
/// 
/// * Group: [`CombinerMappingNV`]
pub const GL_SIGNED_IDENTITY_NV: GlEnum = GlEnum(0x853C);
/// 
/// * Group: [`CombinerMappingNV`]
pub const GL_SIGNED_NEGATE_NV: GlEnum = GlEnum(0x853D);
/// 
/// * Group: [`CombinerScaleNV`]
pub const GL_SCALE_BY_TWO_NV: GlEnum = GlEnum(0x853E);
/// 
/// * Group: [`CombinerScaleNV`]
pub const GL_SCALE_BY_FOUR_NV: GlEnum = GlEnum(0x853F);
/// 
/// * Group: [`CombinerScaleNV`]
pub const GL_SCALE_BY_ONE_HALF_NV: GlEnum = GlEnum(0x8540);
/// 
/// * Group: [`CombinerBiasNV`]
pub const GL_BIAS_BY_NEGATIVE_ONE_HALF_NV: GlEnum = GlEnum(0x8541);
/// 
/// * Group: [`CombinerParameterNV`]
pub const GL_COMBINER_INPUT_NV: GlEnum = GlEnum(0x8542);
/// 
/// * Group: [`CombinerParameterNV`]
pub const GL_COMBINER_MAPPING_NV: GlEnum = GlEnum(0x8543);
/// 
/// * Group: [`CombinerParameterNV`]
pub const GL_COMBINER_COMPONENT_USAGE_NV: GlEnum = GlEnum(0x8544);
pub const GL_COMBINER_AB_DOT_PRODUCT_NV: GlEnum = GlEnum(0x8545);
pub const GL_COMBINER_CD_DOT_PRODUCT_NV: GlEnum = GlEnum(0x8546);
pub const GL_COMBINER_MUX_SUM_NV: GlEnum = GlEnum(0x8547);
pub const GL_COMBINER_SCALE_NV: GlEnum = GlEnum(0x8548);
pub const GL_COMBINER_BIAS_NV: GlEnum = GlEnum(0x8549);
pub const GL_COMBINER_AB_OUTPUT_NV: GlEnum = GlEnum(0x854A);
pub const GL_COMBINER_CD_OUTPUT_NV: GlEnum = GlEnum(0x854B);
pub const GL_COMBINER_SUM_OUTPUT_NV: GlEnum = GlEnum(0x854C);
pub const GL_MAX_GENERAL_COMBINERS_NV: GlEnum = GlEnum(0x854D);
pub const GL_NUM_GENERAL_COMBINERS_NV: GlEnum = GlEnum(0x854E);
pub const GL_COLOR_SUM_CLAMP_NV: GlEnum = GlEnum(0x854F);
/// 
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER0_NV: GlEnum = GlEnum(0x8550);
/// 
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER1_NV: GlEnum = GlEnum(0x8551);
/// 
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER2_NV: GlEnum = GlEnum(0x8552);
/// 
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER3_NV: GlEnum = GlEnum(0x8553);
/// 
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER4_NV: GlEnum = GlEnum(0x8554);
/// 
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER5_NV: GlEnum = GlEnum(0x8555);
/// 
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER6_NV: GlEnum = GlEnum(0x8556);
/// 
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER7_NV: GlEnum = GlEnum(0x8557);
pub const GL_PRIMITIVE_RESTART_NV: GlEnum = GlEnum(0x8558);
pub const GL_PRIMITIVE_RESTART_INDEX_NV: GlEnum = GlEnum(0x8559);
pub const GL_FOG_DISTANCE_MODE_NV: GlEnum = GlEnum(0x855A);
pub const GL_EYE_RADIAL_NV: GlEnum = GlEnum(0x855B);
pub const GL_EYE_PLANE_ABSOLUTE_NV: GlEnum = GlEnum(0x855C);
pub const GL_EMBOSS_LIGHT_NV: GlEnum = GlEnum(0x855D);
pub const GL_EMBOSS_CONSTANT_NV: GlEnum = GlEnum(0x855E);
pub const GL_EMBOSS_MAP_NV: GlEnum = GlEnum(0x855F);
pub const GL_RED_MIN_CLAMP_INGR: GlEnum = GlEnum(0x8560);
pub const GL_GREEN_MIN_CLAMP_INGR: GlEnum = GlEnum(0x8561);
pub const GL_BLUE_MIN_CLAMP_INGR: GlEnum = GlEnum(0x8562);
pub const GL_ALPHA_MIN_CLAMP_INGR: GlEnum = GlEnum(0x8563);
pub const GL_RED_MAX_CLAMP_INGR: GlEnum = GlEnum(0x8564);
pub const GL_GREEN_MAX_CLAMP_INGR: GlEnum = GlEnum(0x8565);
pub const GL_BLUE_MAX_CLAMP_INGR: GlEnum = GlEnum(0x8566);
pub const GL_ALPHA_MAX_CLAMP_INGR: GlEnum = GlEnum(0x8567);
pub const GL_INTERLACE_READ_INGR: GlEnum = GlEnum(0x8568);
pub const GL_COMBINE: GlEnum = GlEnum(0x8570);
pub const GL_COMBINE_ARB: GlEnum = GlEnum(0x8570);
pub const GL_COMBINE_EXT: GlEnum = GlEnum(0x8570);
pub const GL_COMBINE_RGB: GlEnum = GlEnum(0x8571);
pub const GL_COMBINE_RGB_ARB: GlEnum = GlEnum(0x8571);
pub const GL_COMBINE_RGB_EXT: GlEnum = GlEnum(0x8571);
pub const GL_COMBINE_ALPHA: GlEnum = GlEnum(0x8572);
pub const GL_COMBINE_ALPHA_ARB: GlEnum = GlEnum(0x8572);
pub const GL_COMBINE_ALPHA_EXT: GlEnum = GlEnum(0x8572);
pub const GL_RGB_SCALE: GlEnum = GlEnum(0x8573);
pub const GL_RGB_SCALE_ARB: GlEnum = GlEnum(0x8573);
pub const GL_RGB_SCALE_EXT: GlEnum = GlEnum(0x8573);
pub const GL_ADD_SIGNED: GlEnum = GlEnum(0x8574);
pub const GL_ADD_SIGNED_ARB: GlEnum = GlEnum(0x8574);
pub const GL_ADD_SIGNED_EXT: GlEnum = GlEnum(0x8574);
pub const GL_INTERPOLATE: GlEnum = GlEnum(0x8575);
pub const GL_INTERPOLATE_ARB: GlEnum = GlEnum(0x8575);
pub const GL_INTERPOLATE_EXT: GlEnum = GlEnum(0x8575);
/// 
/// * Group: [`PathGenMode`]
pub const GL_CONSTANT: GlEnum = GlEnum(0x8576);
pub const GL_CONSTANT_ARB: GlEnum = GlEnum(0x8576);
pub const GL_CONSTANT_EXT: GlEnum = GlEnum(0x8576);
pub const GL_CONSTANT_NV: GlEnum = GlEnum(0x8576);
/// 
/// * Group: [`PathColor`]
pub const GL_PRIMARY_COLOR: GlEnum = GlEnum(0x8577);
pub const GL_PRIMARY_COLOR_ARB: GlEnum = GlEnum(0x8577);
pub const GL_PRIMARY_COLOR_EXT: GlEnum = GlEnum(0x8577);
pub const GL_PREVIOUS: GlEnum = GlEnum(0x8578);
pub const GL_PREVIOUS_ARB: GlEnum = GlEnum(0x8578);
pub const GL_PREVIOUS_EXT: GlEnum = GlEnum(0x8578);
pub const GL_SOURCE0_RGB: GlEnum = GlEnum(0x8580);
pub const GL_SOURCE0_RGB_ARB: GlEnum = GlEnum(0x8580);
pub const GL_SOURCE0_RGB_EXT: GlEnum = GlEnum(0x8580);
/// 
/// * Alias Of: [`GL_SOURCE0_RGB`]
pub const GL_SRC0_RGB: GlEnum = GL_SOURCE0_RGB;
pub const GL_SOURCE1_RGB: GlEnum = GlEnum(0x8581);
pub const GL_SOURCE1_RGB_ARB: GlEnum = GlEnum(0x8581);
pub const GL_SOURCE1_RGB_EXT: GlEnum = GlEnum(0x8581);
/// 
/// * Alias Of: [`GL_SOURCE1_RGB`]
pub const GL_SRC1_RGB: GlEnum = GL_SOURCE1_RGB;
pub const GL_SOURCE2_RGB: GlEnum = GlEnum(0x8582);
pub const GL_SOURCE2_RGB_ARB: GlEnum = GlEnum(0x8582);
pub const GL_SOURCE2_RGB_EXT: GlEnum = GlEnum(0x8582);
/// 
/// * Alias Of: [`GL_SOURCE2_RGB`]
pub const GL_SRC2_RGB: GlEnum = GL_SOURCE2_RGB;
pub const GL_SOURCE3_RGB_NV: GlEnum = GlEnum(0x8583);
pub const GL_SOURCE0_ALPHA: GlEnum = GlEnum(0x8588);
pub const GL_SOURCE0_ALPHA_ARB: GlEnum = GlEnum(0x8588);
pub const GL_SOURCE0_ALPHA_EXT: GlEnum = GlEnum(0x8588);
/// 
/// * Alias Of: [`GL_SOURCE0_ALPHA`]
pub const GL_SRC0_ALPHA: GlEnum = GL_SOURCE0_ALPHA;
pub const GL_SOURCE1_ALPHA: GlEnum = GlEnum(0x8589);
pub const GL_SOURCE1_ALPHA_ARB: GlEnum = GlEnum(0x8589);
pub const GL_SOURCE1_ALPHA_EXT: GlEnum = GlEnum(0x8589);
/// 
/// * Group: [`BlendingFactor`]
/// * Alias Of: [`GL_SOURCE1_ALPHA`]
pub const GL_SRC1_ALPHA: GlEnum = GL_SOURCE1_ALPHA;
pub const GL_SRC1_ALPHA_EXT: GlEnum = GlEnum(0x8589);
pub const GL_SOURCE2_ALPHA: GlEnum = GlEnum(0x858A);
pub const GL_SOURCE2_ALPHA_ARB: GlEnum = GlEnum(0x858A);
pub const GL_SOURCE2_ALPHA_EXT: GlEnum = GlEnum(0x858A);
/// 
/// * Alias Of: [`GL_SOURCE2_ALPHA`]
pub const GL_SRC2_ALPHA: GlEnum = GL_SOURCE2_ALPHA;
pub const GL_SOURCE3_ALPHA_NV: GlEnum = GlEnum(0x858B);
pub const GL_OPERAND0_RGB: GlEnum = GlEnum(0x8590);
pub const GL_OPERAND0_RGB_ARB: GlEnum = GlEnum(0x8590);
pub const GL_OPERAND0_RGB_EXT: GlEnum = GlEnum(0x8590);
pub const GL_OPERAND1_RGB: GlEnum = GlEnum(0x8591);
pub const GL_OPERAND1_RGB_ARB: GlEnum = GlEnum(0x8591);
pub const GL_OPERAND1_RGB_EXT: GlEnum = GlEnum(0x8591);
pub const GL_OPERAND2_RGB: GlEnum = GlEnum(0x8592);
pub const GL_OPERAND2_RGB_ARB: GlEnum = GlEnum(0x8592);
pub const GL_OPERAND2_RGB_EXT: GlEnum = GlEnum(0x8592);
pub const GL_OPERAND3_RGB_NV: GlEnum = GlEnum(0x8593);
pub const GL_OPERAND0_ALPHA: GlEnum = GlEnum(0x8598);
pub const GL_OPERAND0_ALPHA_ARB: GlEnum = GlEnum(0x8598);
pub const GL_OPERAND0_ALPHA_EXT: GlEnum = GlEnum(0x8598);
pub const GL_OPERAND1_ALPHA: GlEnum = GlEnum(0x8599);
pub const GL_OPERAND1_ALPHA_ARB: GlEnum = GlEnum(0x8599);
pub const GL_OPERAND1_ALPHA_EXT: GlEnum = GlEnum(0x8599);
pub const GL_OPERAND2_ALPHA: GlEnum = GlEnum(0x859A);
pub const GL_OPERAND2_ALPHA_ARB: GlEnum = GlEnum(0x859A);
pub const GL_OPERAND2_ALPHA_EXT: GlEnum = GlEnum(0x859A);
pub const GL_OPERAND3_ALPHA_NV: GlEnum = GlEnum(0x859B);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SUBSAMPLE_RATE_SGIX: GlEnum = GlEnum(0x85A0);
/// 
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SUBSAMPLE_RATE_SGIX: GlEnum = GlEnum(0x85A1);
/// 
/// * Group: [`PixelStoreSubsampleRate`]
pub const GL_PIXEL_SUBSAMPLE_4444_SGIX: GlEnum = GlEnum(0x85A2);
/// 
/// * Group: [`PixelStoreSubsampleRate`]
pub const GL_PIXEL_SUBSAMPLE_2424_SGIX: GlEnum = GlEnum(0x85A3);
/// 
/// * Group: [`PixelStoreSubsampleRate`]
pub const GL_PIXEL_SUBSAMPLE_4242_SGIX: GlEnum = GlEnum(0x85A4);
/// 
/// * Group: [`TextureNormalModeEXT`]
pub const GL_PERTURB_EXT: GlEnum = GlEnum(0x85AE);
pub const GL_TEXTURE_NORMAL_EXT: GlEnum = GlEnum(0x85AF);
pub const GL_LIGHT_MODEL_SPECULAR_VECTOR_APPLE: GlEnum = GlEnum(0x85B0);
/// 
/// * Group: [`HintTarget`]
pub const GL_TRANSFORM_HINT_APPLE: GlEnum = GlEnum(0x85B1);
pub const GL_UNPACK_CLIENT_STORAGE_APPLE: GlEnum = GlEnum(0x85B2);
pub const GL_BUFFER_OBJECT_APPLE: GlEnum = GlEnum(0x85B3);
/// 
/// * Group: [`VertexArrayPNameAPPLE`]
pub const GL_STORAGE_CLIENT_APPLE: GlEnum = GlEnum(0x85B4);
/// 
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_BINDING: GlEnum = GlEnum(0x85B5);
pub const GL_VERTEX_ARRAY_BINDING_APPLE: GlEnum = GlEnum(0x85B5);
pub const GL_VERTEX_ARRAY_BINDING_OES: GlEnum = GlEnum(0x85B5);
pub const GL_TEXTURE_RANGE_LENGTH_APPLE: GlEnum = GlEnum(0x85B7);
pub const GL_TEXTURE_RANGE_POINTER_APPLE: GlEnum = GlEnum(0x85B8);
pub const GL_YCBCR_422_APPLE: GlEnum = GlEnum(0x85B9);
pub const GL_UNSIGNED_SHORT_8_8_APPLE: GlEnum = GlEnum(0x85BA);
pub const GL_UNSIGNED_SHORT_8_8_MESA: GlEnum = GlEnum(0x85BA);
pub const GL_UNSIGNED_SHORT_8_8_REV_APPLE: GlEnum = GlEnum(0x85BB);
pub const GL_UNSIGNED_SHORT_8_8_REV_MESA: GlEnum = GlEnum(0x85BB);
/// 
/// * Group: [`HintTarget`]
pub const GL_TEXTURE_STORAGE_HINT_APPLE: GlEnum = GlEnum(0x85BC);
pub const GL_STORAGE_PRIVATE_APPLE: GlEnum = GlEnum(0x85BD);
/// 
/// * Group: [`VertexArrayPNameAPPLE`]
pub const GL_STORAGE_CACHED_APPLE: GlEnum = GlEnum(0x85BE);
/// 
/// * Group: [`VertexArrayPNameAPPLE`]
pub const GL_STORAGE_SHARED_APPLE: GlEnum = GlEnum(0x85BF);
pub const GL_REPLACEMENT_CODE_ARRAY_SUN: GlEnum = GlEnum(0x85C0);
pub const GL_REPLACEMENT_CODE_ARRAY_TYPE_SUN: GlEnum = GlEnum(0x85C1);
pub const GL_REPLACEMENT_CODE_ARRAY_STRIDE_SUN: GlEnum = GlEnum(0x85C2);
pub const GL_REPLACEMENT_CODE_ARRAY_POINTER_SUN: GlEnum = GlEnum(0x85C3);
pub const GL_R1UI_V3F_SUN: GlEnum = GlEnum(0x85C4);
pub const GL_R1UI_C4UB_V3F_SUN: GlEnum = GlEnum(0x85C5);
pub const GL_R1UI_C3F_V3F_SUN: GlEnum = GlEnum(0x85C6);
pub const GL_R1UI_N3F_V3F_SUN: GlEnum = GlEnum(0x85C7);
pub const GL_R1UI_C4F_N3F_V3F_SUN: GlEnum = GlEnum(0x85C8);
pub const GL_R1UI_T2F_V3F_SUN: GlEnum = GlEnum(0x85C9);
pub const GL_R1UI_T2F_N3F_V3F_SUN: GlEnum = GlEnum(0x85CA);
pub const GL_R1UI_T2F_C4F_N3F_V3F_SUN: GlEnum = GlEnum(0x85CB);
pub const GL_SLICE_ACCUM_SUN: GlEnum = GlEnum(0x85CC);
pub const GL_QUAD_MESH_SUN: GlEnum = GlEnum(0x8614);
pub const GL_TRIANGLE_MESH_SUN: GlEnum = GlEnum(0x8615);
/// 
/// * Group: [`ProgramTarget`]
pub const GL_VERTEX_PROGRAM_ARB: GlEnum = GlEnum(0x8620);
pub const GL_VERTEX_PROGRAM_NV: GlEnum = GlEnum(0x8620);
pub const GL_VERTEX_STATE_PROGRAM_NV: GlEnum = GlEnum(0x8621);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GlEnum = GlEnum(0x8622);
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED_ARB: GlEnum = GlEnum(0x8622);
pub const GL_ATTRIB_ARRAY_SIZE_NV: GlEnum = GlEnum(0x8623);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GlEnum = GlEnum(0x8623);
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE_ARB: GlEnum = GlEnum(0x8623);
pub const GL_ATTRIB_ARRAY_STRIDE_NV: GlEnum = GlEnum(0x8624);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GlEnum = GlEnum(0x8624);
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE_ARB: GlEnum = GlEnum(0x8624);
pub const GL_ATTRIB_ARRAY_TYPE_NV: GlEnum = GlEnum(0x8625);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GlEnum = GlEnum(0x8625);
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE_ARB: GlEnum = GlEnum(0x8625);
pub const GL_CURRENT_ATTRIB_NV: GlEnum = GlEnum(0x8626);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`]
pub const GL_CURRENT_VERTEX_ATTRIB: GlEnum = GlEnum(0x8626);
pub const GL_CURRENT_VERTEX_ATTRIB_ARB: GlEnum = GlEnum(0x8626);
pub const GL_PROGRAM_LENGTH_ARB: GlEnum = GlEnum(0x8627);
pub const GL_PROGRAM_LENGTH_NV: GlEnum = GlEnum(0x8627);
/// 
/// * Group: [`ProgramStringProperty`]
pub const GL_PROGRAM_STRING_ARB: GlEnum = GlEnum(0x8628);
pub const GL_PROGRAM_STRING_NV: GlEnum = GlEnum(0x8628);
pub const GL_MODELVIEW_PROJECTION_NV: GlEnum = GlEnum(0x8629);
pub const GL_IDENTITY_NV: GlEnum = GlEnum(0x862A);
pub const GL_INVERSE_NV: GlEnum = GlEnum(0x862B);
pub const GL_TRANSPOSE_NV: GlEnum = GlEnum(0x862C);
pub const GL_INVERSE_TRANSPOSE_NV: GlEnum = GlEnum(0x862D);
pub const GL_MAX_PROGRAM_MATRIX_STACK_DEPTH_ARB: GlEnum = GlEnum(0x862E);
pub const GL_MAX_TRACK_MATRIX_STACK_DEPTH_NV: GlEnum = GlEnum(0x862E);
pub const GL_MAX_PROGRAM_MATRICES_ARB: GlEnum = GlEnum(0x862F);
pub const GL_MAX_TRACK_MATRICES_NV: GlEnum = GlEnum(0x862F);
pub const GL_MATRIX0_NV: GlEnum = GlEnum(0x8630);
pub const GL_MATRIX1_NV: GlEnum = GlEnum(0x8631);
pub const GL_MATRIX2_NV: GlEnum = GlEnum(0x8632);
pub const GL_MATRIX3_NV: GlEnum = GlEnum(0x8633);
pub const GL_MATRIX4_NV: GlEnum = GlEnum(0x8634);
pub const GL_MATRIX5_NV: GlEnum = GlEnum(0x8635);
pub const GL_MATRIX6_NV: GlEnum = GlEnum(0x8636);
pub const GL_MATRIX7_NV: GlEnum = GlEnum(0x8637);
pub const GL_CURRENT_MATRIX_STACK_DEPTH_ARB: GlEnum = GlEnum(0x8640);
pub const GL_CURRENT_MATRIX_STACK_DEPTH_NV: GlEnum = GlEnum(0x8640);
pub const GL_CURRENT_MATRIX_ARB: GlEnum = GlEnum(0x8641);
pub const GL_CURRENT_MATRIX_NV: GlEnum = GlEnum(0x8641);
pub const GL_VERTEX_PROGRAM_POINT_SIZE: GlEnum = GlEnum(0x8642);
pub const GL_VERTEX_PROGRAM_POINT_SIZE_ARB: GlEnum = GlEnum(0x8642);
pub const GL_VERTEX_PROGRAM_POINT_SIZE_NV: GlEnum = GlEnum(0x8642);
/// 
/// * Group: [`GetPName`], [`EnableCap`]
/// * Alias Of: [`GL_VERTEX_PROGRAM_POINT_SIZE`]
pub const GL_PROGRAM_POINT_SIZE: GlEnum = GL_VERTEX_PROGRAM_POINT_SIZE;
pub const GL_PROGRAM_POINT_SIZE_ARB: GlEnum = GlEnum(0x8642);
pub const GL_PROGRAM_POINT_SIZE_EXT: GlEnum = GlEnum(0x8642);
pub const GL_VERTEX_PROGRAM_TWO_SIDE: GlEnum = GlEnum(0x8643);
pub const GL_VERTEX_PROGRAM_TWO_SIDE_ARB: GlEnum = GlEnum(0x8643);
pub const GL_VERTEX_PROGRAM_TWO_SIDE_NV: GlEnum = GlEnum(0x8643);
/// 
/// * Group: [`VertexAttribEnumNV`]
pub const GL_PROGRAM_PARAMETER_NV: GlEnum = GlEnum(0x8644);
pub const GL_ATTRIB_ARRAY_POINTER_NV: GlEnum = GlEnum(0x8645);
/// 
/// * Group: [`VertexAttribPointerPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GlEnum = GlEnum(0x8645);
/// 
/// * Group: [`VertexAttribPointerPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER_ARB: GlEnum = GlEnum(0x8645);
pub const GL_PROGRAM_TARGET_NV: GlEnum = GlEnum(0x8646);
pub const GL_PROGRAM_RESIDENT_NV: GlEnum = GlEnum(0x8647);
pub const GL_TRACK_MATRIX_NV: GlEnum = GlEnum(0x8648);
pub const GL_TRACK_MATRIX_TRANSFORM_NV: GlEnum = GlEnum(0x8649);
pub const GL_VERTEX_PROGRAM_BINDING_NV: GlEnum = GlEnum(0x864A);
pub const GL_PROGRAM_ERROR_POSITION_ARB: GlEnum = GlEnum(0x864B);
pub const GL_PROGRAM_ERROR_POSITION_NV: GlEnum = GlEnum(0x864B);
pub const GL_OFFSET_TEXTURE_RECTANGLE_NV: GlEnum = GlEnum(0x864C);
pub const GL_OFFSET_TEXTURE_RECTANGLE_SCALE_NV: GlEnum = GlEnum(0x864D);
pub const GL_DOT_PRODUCT_TEXTURE_RECTANGLE_NV: GlEnum = GlEnum(0x864E);
/// 
/// * Group: [`EnableCap`]
pub const GL_DEPTH_CLAMP: GlEnum = GlEnum(0x864F);
pub const GL_DEPTH_CLAMP_NV: GlEnum = GlEnum(0x864F);
pub const GL_DEPTH_CLAMP_EXT: GlEnum = GlEnum(0x864F);
pub const GL_VERTEX_ATTRIB_ARRAY0_NV: GlEnum = GlEnum(0x8650);
pub const GL_VERTEX_ATTRIB_ARRAY1_NV: GlEnum = GlEnum(0x8651);
pub const GL_VERTEX_ATTRIB_ARRAY2_NV: GlEnum = GlEnum(0x8652);
pub const GL_VERTEX_ATTRIB_ARRAY3_NV: GlEnum = GlEnum(0x8653);
pub const GL_VERTEX_ATTRIB_ARRAY4_NV: GlEnum = GlEnum(0x8654);
pub const GL_VERTEX_ATTRIB_ARRAY5_NV: GlEnum = GlEnum(0x8655);
pub const GL_VERTEX_ATTRIB_ARRAY6_NV: GlEnum = GlEnum(0x8656);
pub const GL_VERTEX_ATTRIB_ARRAY7_NV: GlEnum = GlEnum(0x8657);
pub const GL_VERTEX_ATTRIB_ARRAY8_NV: GlEnum = GlEnum(0x8658);
pub const GL_VERTEX_ATTRIB_ARRAY9_NV: GlEnum = GlEnum(0x8659);
pub const GL_VERTEX_ATTRIB_ARRAY10_NV: GlEnum = GlEnum(0x865A);
pub const GL_VERTEX_ATTRIB_ARRAY11_NV: GlEnum = GlEnum(0x865B);
pub const GL_VERTEX_ATTRIB_ARRAY12_NV: GlEnum = GlEnum(0x865C);
pub const GL_VERTEX_ATTRIB_ARRAY13_NV: GlEnum = GlEnum(0x865D);
pub const GL_VERTEX_ATTRIB_ARRAY14_NV: GlEnum = GlEnum(0x865E);
pub const GL_VERTEX_ATTRIB_ARRAY15_NV: GlEnum = GlEnum(0x865F);
pub const GL_MAP1_VERTEX_ATTRIB0_4_NV: GlEnum = GlEnum(0x8660);
pub const GL_MAP1_VERTEX_ATTRIB1_4_NV: GlEnum = GlEnum(0x8661);
pub const GL_MAP1_VERTEX_ATTRIB2_4_NV: GlEnum = GlEnum(0x8662);
pub const GL_MAP1_VERTEX_ATTRIB3_4_NV: GlEnum = GlEnum(0x8663);
pub const GL_MAP1_VERTEX_ATTRIB4_4_NV: GlEnum = GlEnum(0x8664);
pub const GL_MAP1_VERTEX_ATTRIB5_4_NV: GlEnum = GlEnum(0x8665);
pub const GL_MAP1_VERTEX_ATTRIB6_4_NV: GlEnum = GlEnum(0x8666);
pub const GL_MAP1_VERTEX_ATTRIB7_4_NV: GlEnum = GlEnum(0x8667);
pub const GL_MAP1_VERTEX_ATTRIB8_4_NV: GlEnum = GlEnum(0x8668);
pub const GL_MAP1_VERTEX_ATTRIB9_4_NV: GlEnum = GlEnum(0x8669);
pub const GL_MAP1_VERTEX_ATTRIB10_4_NV: GlEnum = GlEnum(0x866A);
pub const GL_MAP1_VERTEX_ATTRIB11_4_NV: GlEnum = GlEnum(0x866B);
pub const GL_MAP1_VERTEX_ATTRIB12_4_NV: GlEnum = GlEnum(0x866C);
pub const GL_MAP1_VERTEX_ATTRIB13_4_NV: GlEnum = GlEnum(0x866D);
pub const GL_MAP1_VERTEX_ATTRIB14_4_NV: GlEnum = GlEnum(0x866E);
pub const GL_MAP1_VERTEX_ATTRIB15_4_NV: GlEnum = GlEnum(0x866F);
pub const GL_MAP2_VERTEX_ATTRIB0_4_NV: GlEnum = GlEnum(0x8670);
pub const GL_MAP2_VERTEX_ATTRIB1_4_NV: GlEnum = GlEnum(0x8671);
pub const GL_MAP2_VERTEX_ATTRIB2_4_NV: GlEnum = GlEnum(0x8672);
pub const GL_MAP2_VERTEX_ATTRIB3_4_NV: GlEnum = GlEnum(0x8673);
pub const GL_MAP2_VERTEX_ATTRIB4_4_NV: GlEnum = GlEnum(0x8674);
pub const GL_MAP2_VERTEX_ATTRIB5_4_NV: GlEnum = GlEnum(0x8675);
pub const GL_MAP2_VERTEX_ATTRIB6_4_NV: GlEnum = GlEnum(0x8676);
pub const GL_MAP2_VERTEX_ATTRIB7_4_NV: GlEnum = GlEnum(0x8677);
/// NOT an alias. Accidental reuse of GL_MAP2_VERTEX_ATTRIB7_4_NV
pub const GL_PROGRAM_BINDING_ARB: GlEnum = GlEnum(0x8677);
pub const GL_MAP2_VERTEX_ATTRIB8_4_NV: GlEnum = GlEnum(0x8678);
pub const GL_MAP2_VERTEX_ATTRIB9_4_NV: GlEnum = GlEnum(0x8679);
pub const GL_MAP2_VERTEX_ATTRIB10_4_NV: GlEnum = GlEnum(0x867A);
pub const GL_MAP2_VERTEX_ATTRIB11_4_NV: GlEnum = GlEnum(0x867B);
pub const GL_MAP2_VERTEX_ATTRIB12_4_NV: GlEnum = GlEnum(0x867C);
pub const GL_MAP2_VERTEX_ATTRIB13_4_NV: GlEnum = GlEnum(0x867D);
pub const GL_MAP2_VERTEX_ATTRIB14_4_NV: GlEnum = GlEnum(0x867E);
pub const GL_MAP2_VERTEX_ATTRIB15_4_NV: GlEnum = GlEnum(0x867F);
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: GlEnum = GlEnum(0x86A0);
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE_ARB: GlEnum = GlEnum(0x86A0);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_COMPRESSED: GlEnum = GlEnum(0x86A1);
pub const GL_TEXTURE_COMPRESSED_ARB: GlEnum = GlEnum(0x86A1);
/// 
/// * Group: [`GetPName`]
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GlEnum = GlEnum(0x86A2);
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS_ARB: GlEnum = GlEnum(0x86A2);
/// 
/// * Group: [`GetPName`]
pub const GL_COMPRESSED_TEXTURE_FORMATS: GlEnum = GlEnum(0x86A3);
pub const GL_COMPRESSED_TEXTURE_FORMATS_ARB: GlEnum = GlEnum(0x86A3);
pub const GL_MAX_VERTEX_UNITS_ARB: GlEnum = GlEnum(0x86A4);
pub const GL_MAX_VERTEX_UNITS_OES: GlEnum = GlEnum(0x86A4);
pub const GL_ACTIVE_VERTEX_UNITS_ARB: GlEnum = GlEnum(0x86A5);
pub const GL_WEIGHT_SUM_UNITY_ARB: GlEnum = GlEnum(0x86A6);
pub const GL_VERTEX_BLEND_ARB: GlEnum = GlEnum(0x86A7);
pub const GL_CURRENT_WEIGHT_ARB: GlEnum = GlEnum(0x86A8);
pub const GL_WEIGHT_ARRAY_TYPE_ARB: GlEnum = GlEnum(0x86A9);
pub const GL_WEIGHT_ARRAY_TYPE_OES: GlEnum = GlEnum(0x86A9);
pub const GL_WEIGHT_ARRAY_STRIDE_ARB: GlEnum = GlEnum(0x86AA);
pub const GL_WEIGHT_ARRAY_STRIDE_OES: GlEnum = GlEnum(0x86AA);
pub const GL_WEIGHT_ARRAY_SIZE_ARB: GlEnum = GlEnum(0x86AB);
pub const GL_WEIGHT_ARRAY_SIZE_OES: GlEnum = GlEnum(0x86AB);
pub const GL_WEIGHT_ARRAY_POINTER_ARB: GlEnum = GlEnum(0x86AC);
pub const GL_WEIGHT_ARRAY_POINTER_OES: GlEnum = GlEnum(0x86AC);
pub const GL_WEIGHT_ARRAY_ARB: GlEnum = GlEnum(0x86AD);
pub const GL_WEIGHT_ARRAY_OES: GlEnum = GlEnum(0x86AD);
pub const GL_DOT3_RGB: GlEnum = GlEnum(0x86AE);
pub const GL_DOT3_RGB_ARB: GlEnum = GlEnum(0x86AE);
pub const GL_DOT3_RGBA: GlEnum = GlEnum(0x86AF);
pub const GL_DOT3_RGBA_ARB: GlEnum = GlEnum(0x86AF);
pub const GL_DOT3_RGBA_IMG: GlEnum = GlEnum(0x86AF);
pub const GL_COMPRESSED_RGB_FXT1_3DFX: GlEnum = GlEnum(0x86B0);
pub const GL_COMPRESSED_RGBA_FXT1_3DFX: GlEnum = GlEnum(0x86B1);
pub const GL_MULTISAMPLE_3DFX: GlEnum = GlEnum(0x86B2);
pub const GL_SAMPLE_BUFFERS_3DFX: GlEnum = GlEnum(0x86B3);
pub const GL_SAMPLES_3DFX: GlEnum = GlEnum(0x86B4);
/// 
/// * Group: [`EvalTargetNV`]
pub const GL_EVAL_2D_NV: GlEnum = GlEnum(0x86C0);
/// 
/// * Group: [`EvalTargetNV`]
pub const GL_EVAL_TRIANGULAR_2D_NV: GlEnum = GlEnum(0x86C1);
/// 
/// * Group: [`MapParameterNV`]
pub const GL_MAP_TESSELLATION_NV: GlEnum = GlEnum(0x86C2);
/// 
/// * Group: [`MapAttribParameterNV`]
pub const GL_MAP_ATTRIB_U_ORDER_NV: GlEnum = GlEnum(0x86C3);
/// 
/// * Group: [`MapAttribParameterNV`]
pub const GL_MAP_ATTRIB_V_ORDER_NV: GlEnum = GlEnum(0x86C4);
pub const GL_EVAL_FRACTIONAL_TESSELLATION_NV: GlEnum = GlEnum(0x86C5);
pub const GL_EVAL_VERTEX_ATTRIB0_NV: GlEnum = GlEnum(0x86C6);
pub const GL_EVAL_VERTEX_ATTRIB1_NV: GlEnum = GlEnum(0x86C7);
pub const GL_EVAL_VERTEX_ATTRIB2_NV: GlEnum = GlEnum(0x86C8);
pub const GL_EVAL_VERTEX_ATTRIB3_NV: GlEnum = GlEnum(0x86C9);
pub const GL_EVAL_VERTEX_ATTRIB4_NV: GlEnum = GlEnum(0x86CA);
pub const GL_EVAL_VERTEX_ATTRIB5_NV: GlEnum = GlEnum(0x86CB);
pub const GL_EVAL_VERTEX_ATTRIB6_NV: GlEnum = GlEnum(0x86CC);
pub const GL_EVAL_VERTEX_ATTRIB7_NV: GlEnum = GlEnum(0x86CD);
pub const GL_EVAL_VERTEX_ATTRIB8_NV: GlEnum = GlEnum(0x86CE);
pub const GL_EVAL_VERTEX_ATTRIB9_NV: GlEnum = GlEnum(0x86CF);
pub const GL_EVAL_VERTEX_ATTRIB10_NV: GlEnum = GlEnum(0x86D0);
pub const GL_EVAL_VERTEX_ATTRIB11_NV: GlEnum = GlEnum(0x86D1);
pub const GL_EVAL_VERTEX_ATTRIB12_NV: GlEnum = GlEnum(0x86D2);
pub const GL_EVAL_VERTEX_ATTRIB13_NV: GlEnum = GlEnum(0x86D3);
pub const GL_EVAL_VERTEX_ATTRIB14_NV: GlEnum = GlEnum(0x86D4);
pub const GL_EVAL_VERTEX_ATTRIB15_NV: GlEnum = GlEnum(0x86D5);
pub const GL_MAX_MAP_TESSELLATION_NV: GlEnum = GlEnum(0x86D6);
pub const GL_MAX_RATIONAL_EVAL_ORDER_NV: GlEnum = GlEnum(0x86D7);
pub const GL_MAX_PROGRAM_PATCH_ATTRIBS_NV: GlEnum = GlEnum(0x86D8);
pub const GL_RGBA_UNSIGNED_DOT_PRODUCT_MAPPING_NV: GlEnum = GlEnum(0x86D9);
pub const GL_UNSIGNED_INT_S8_S8_8_8_NV: GlEnum = GlEnum(0x86DA);
pub const GL_UNSIGNED_INT_8_8_S8_S8_REV_NV: GlEnum = GlEnum(0x86DB);
pub const GL_DSDT_MAG_INTENSITY_NV: GlEnum = GlEnum(0x86DC);
pub const GL_SHADER_CONSISTENT_NV: GlEnum = GlEnum(0x86DD);
pub const GL_TEXTURE_SHADER_NV: GlEnum = GlEnum(0x86DE);
pub const GL_SHADER_OPERATION_NV: GlEnum = GlEnum(0x86DF);
pub const GL_CULL_MODES_NV: GlEnum = GlEnum(0x86E0);
pub const GL_OFFSET_TEXTURE_MATRIX_NV: GlEnum = GlEnum(0x86E1);
/// 
/// * Alias Of: [`GL_OFFSET_TEXTURE_MATRIX_NV`]
pub const GL_OFFSET_TEXTURE_2D_MATRIX_NV: GlEnum = GL_OFFSET_TEXTURE_MATRIX_NV;
pub const GL_OFFSET_TEXTURE_SCALE_NV: GlEnum = GlEnum(0x86E2);
/// 
/// * Alias Of: [`GL_OFFSET_TEXTURE_SCALE_NV`]
pub const GL_OFFSET_TEXTURE_2D_SCALE_NV: GlEnum = GL_OFFSET_TEXTURE_SCALE_NV;
pub const GL_OFFSET_TEXTURE_BIAS_NV: GlEnum = GlEnum(0x86E3);
/// 
/// * Alias Of: [`GL_OFFSET_TEXTURE_BIAS_NV`]
pub const GL_OFFSET_TEXTURE_2D_BIAS_NV: GlEnum = GL_OFFSET_TEXTURE_BIAS_NV;
pub const GL_PREVIOUS_TEXTURE_INPUT_NV: GlEnum = GlEnum(0x86E4);
pub const GL_CONST_EYE_NV: GlEnum = GlEnum(0x86E5);
pub const GL_PASS_THROUGH_NV: GlEnum = GlEnum(0x86E6);
pub const GL_CULL_FRAGMENT_NV: GlEnum = GlEnum(0x86E7);
pub const GL_OFFSET_TEXTURE_2D_NV: GlEnum = GlEnum(0x86E8);
pub const GL_DEPENDENT_AR_TEXTURE_2D_NV: GlEnum = GlEnum(0x86E9);
pub const GL_DEPENDENT_GB_TEXTURE_2D_NV: GlEnum = GlEnum(0x86EA);
pub const GL_SURFACE_STATE_NV: GlEnum = GlEnum(0x86EB);
pub const GL_DOT_PRODUCT_NV: GlEnum = GlEnum(0x86EC);
pub const GL_DOT_PRODUCT_DEPTH_REPLACE_NV: GlEnum = GlEnum(0x86ED);
pub const GL_DOT_PRODUCT_TEXTURE_2D_NV: GlEnum = GlEnum(0x86EE);
pub const GL_DOT_PRODUCT_TEXTURE_3D_NV: GlEnum = GlEnum(0x86EF);
pub const GL_DOT_PRODUCT_TEXTURE_CUBE_MAP_NV: GlEnum = GlEnum(0x86F0);
pub const GL_DOT_PRODUCT_DIFFUSE_CUBE_MAP_NV: GlEnum = GlEnum(0x86F1);
pub const GL_DOT_PRODUCT_REFLECT_CUBE_MAP_NV: GlEnum = GlEnum(0x86F2);
pub const GL_DOT_PRODUCT_CONST_EYE_REFLECT_CUBE_MAP_NV: GlEnum = GlEnum(0x86F3);
pub const GL_HILO_NV: GlEnum = GlEnum(0x86F4);
pub const GL_DSDT_NV: GlEnum = GlEnum(0x86F5);
pub const GL_DSDT_MAG_NV: GlEnum = GlEnum(0x86F6);
pub const GL_DSDT_MAG_VIB_NV: GlEnum = GlEnum(0x86F7);
pub const GL_HILO16_NV: GlEnum = GlEnum(0x86F8);
pub const GL_SIGNED_HILO_NV: GlEnum = GlEnum(0x86F9);
pub const GL_SIGNED_HILO16_NV: GlEnum = GlEnum(0x86FA);
pub const GL_SIGNED_RGBA_NV: GlEnum = GlEnum(0x86FB);
pub const GL_SIGNED_RGBA8_NV: GlEnum = GlEnum(0x86FC);
pub const GL_SURFACE_REGISTERED_NV: GlEnum = GlEnum(0x86FD);
pub const GL_SIGNED_RGB_NV: GlEnum = GlEnum(0x86FE);
pub const GL_SIGNED_RGB8_NV: GlEnum = GlEnum(0x86FF);
pub const GL_SURFACE_MAPPED_NV: GlEnum = GlEnum(0x8700);
pub const GL_SIGNED_LUMINANCE_NV: GlEnum = GlEnum(0x8701);
pub const GL_SIGNED_LUMINANCE8_NV: GlEnum = GlEnum(0x8702);
pub const GL_SIGNED_LUMINANCE_ALPHA_NV: GlEnum = GlEnum(0x8703);
pub const GL_SIGNED_LUMINANCE8_ALPHA8_NV: GlEnum = GlEnum(0x8704);
pub const GL_SIGNED_ALPHA_NV: GlEnum = GlEnum(0x8705);
pub const GL_SIGNED_ALPHA8_NV: GlEnum = GlEnum(0x8706);
pub const GL_SIGNED_INTENSITY_NV: GlEnum = GlEnum(0x8707);
pub const GL_SIGNED_INTENSITY8_NV: GlEnum = GlEnum(0x8708);
pub const GL_DSDT8_NV: GlEnum = GlEnum(0x8709);
pub const GL_DSDT8_MAG8_NV: GlEnum = GlEnum(0x870A);
pub const GL_DSDT8_MAG8_INTENSITY8_NV: GlEnum = GlEnum(0x870B);
pub const GL_SIGNED_RGB_UNSIGNED_ALPHA_NV: GlEnum = GlEnum(0x870C);
pub const GL_SIGNED_RGB8_UNSIGNED_ALPHA8_NV: GlEnum = GlEnum(0x870D);
pub const GL_HI_SCALE_NV: GlEnum = GlEnum(0x870E);
pub const GL_LO_SCALE_NV: GlEnum = GlEnum(0x870F);
pub const GL_DS_SCALE_NV: GlEnum = GlEnum(0x8710);
pub const GL_DT_SCALE_NV: GlEnum = GlEnum(0x8711);
pub const GL_MAGNITUDE_SCALE_NV: GlEnum = GlEnum(0x8712);
pub const GL_VIBRANCE_SCALE_NV: GlEnum = GlEnum(0x8713);
pub const GL_HI_BIAS_NV: GlEnum = GlEnum(0x8714);
pub const GL_LO_BIAS_NV: GlEnum = GlEnum(0x8715);
pub const GL_DS_BIAS_NV: GlEnum = GlEnum(0x8716);
pub const GL_DT_BIAS_NV: GlEnum = GlEnum(0x8717);
pub const GL_MAGNITUDE_BIAS_NV: GlEnum = GlEnum(0x8718);
pub const GL_VIBRANCE_BIAS_NV: GlEnum = GlEnum(0x8719);
pub const GL_TEXTURE_BORDER_VALUES_NV: GlEnum = GlEnum(0x871A);
pub const GL_TEXTURE_HI_SIZE_NV: GlEnum = GlEnum(0x871B);
pub const GL_TEXTURE_LO_SIZE_NV: GlEnum = GlEnum(0x871C);
pub const GL_TEXTURE_DS_SIZE_NV: GlEnum = GlEnum(0x871D);
pub const GL_TEXTURE_DT_SIZE_NV: GlEnum = GlEnum(0x871E);
pub const GL_TEXTURE_MAG_SIZE_NV: GlEnum = GlEnum(0x871F);
pub const GL_MODELVIEW2_ARB: GlEnum = GlEnum(0x8722);
pub const GL_MODELVIEW3_ARB: GlEnum = GlEnum(0x8723);
pub const GL_MODELVIEW4_ARB: GlEnum = GlEnum(0x8724);
pub const GL_MODELVIEW5_ARB: GlEnum = GlEnum(0x8725);
pub const GL_MODELVIEW6_ARB: GlEnum = GlEnum(0x8726);
pub const GL_MODELVIEW7_ARB: GlEnum = GlEnum(0x8727);
pub const GL_MODELVIEW8_ARB: GlEnum = GlEnum(0x8728);
pub const GL_MODELVIEW9_ARB: GlEnum = GlEnum(0x8729);
pub const GL_MODELVIEW10_ARB: GlEnum = GlEnum(0x872A);
pub const GL_MODELVIEW11_ARB: GlEnum = GlEnum(0x872B);
pub const GL_MODELVIEW12_ARB: GlEnum = GlEnum(0x872C);
pub const GL_MODELVIEW13_ARB: GlEnum = GlEnum(0x872D);
pub const GL_MODELVIEW14_ARB: GlEnum = GlEnum(0x872E);
pub const GL_MODELVIEW15_ARB: GlEnum = GlEnum(0x872F);
pub const GL_MODELVIEW16_ARB: GlEnum = GlEnum(0x8730);
pub const GL_MODELVIEW17_ARB: GlEnum = GlEnum(0x8731);
pub const GL_MODELVIEW18_ARB: GlEnum = GlEnum(0x8732);
pub const GL_MODELVIEW19_ARB: GlEnum = GlEnum(0x8733);
pub const GL_MODELVIEW20_ARB: GlEnum = GlEnum(0x8734);
pub const GL_MODELVIEW21_ARB: GlEnum = GlEnum(0x8735);
pub const GL_MODELVIEW22_ARB: GlEnum = GlEnum(0x8736);
pub const GL_MODELVIEW23_ARB: GlEnum = GlEnum(0x8737);
pub const GL_MODELVIEW24_ARB: GlEnum = GlEnum(0x8738);
pub const GL_MODELVIEW25_ARB: GlEnum = GlEnum(0x8739);
pub const GL_MODELVIEW26_ARB: GlEnum = GlEnum(0x873A);
pub const GL_MODELVIEW27_ARB: GlEnum = GlEnum(0x873B);
pub const GL_MODELVIEW28_ARB: GlEnum = GlEnum(0x873C);
pub const GL_MODELVIEW29_ARB: GlEnum = GlEnum(0x873D);
pub const GL_MODELVIEW30_ARB: GlEnum = GlEnum(0x873E);
pub const GL_MODELVIEW31_ARB: GlEnum = GlEnum(0x873F);
pub const GL_DOT3_RGB_EXT: GlEnum = GlEnum(0x8740);
/// NOT an alias. Accidental reuse of GL_DOT3_RGB_EXT
pub const GL_Z400_BINARY_AMD: GlEnum = GlEnum(0x8740);
pub const GL_DOT3_RGBA_EXT: GlEnum = GlEnum(0x8741);
/// NOT an alias. Accidental reuse of GL_DOT3_RGBA_EXT
pub const GL_PROGRAM_BINARY_LENGTH_OES: GlEnum = GlEnum(0x8741);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_PROGRAM_BINARY_LENGTH: GlEnum = GlEnum(0x8741);
pub const GL_MIRROR_CLAMP_ATI: GlEnum = GlEnum(0x8742);
pub const GL_MIRROR_CLAMP_EXT: GlEnum = GlEnum(0x8742);
pub const GL_MIRROR_CLAMP_TO_EDGE: GlEnum = GlEnum(0x8743);
pub const GL_MIRROR_CLAMP_TO_EDGE_ATI: GlEnum = GlEnum(0x8743);
pub const GL_MIRROR_CLAMP_TO_EDGE_EXT: GlEnum = GlEnum(0x8743);
pub const GL_MODULATE_ADD_ATI: GlEnum = GlEnum(0x8744);
pub const GL_MODULATE_SIGNED_ADD_ATI: GlEnum = GlEnum(0x8745);
pub const GL_MODULATE_SUBTRACT_ATI: GlEnum = GlEnum(0x8746);
pub const GL_SET_AMD: GlEnum = GlEnum(0x874A);
pub const GL_REPLACE_VALUE_AMD: GlEnum = GlEnum(0x874B);
pub const GL_STENCIL_OP_VALUE_AMD: GlEnum = GlEnum(0x874C);
pub const GL_STENCIL_BACK_OP_VALUE_AMD: GlEnum = GlEnum(0x874D);
/// 
/// * Group: [`VertexArrayPName`], [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_LONG: GlEnum = GlEnum(0x874E);
pub const GL_OCCLUSION_QUERY_EVENT_MASK_AMD: GlEnum = GlEnum(0x874F);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_STENCIL_MESA: GlEnum = GlEnum(0x8750);
pub const GL_UNSIGNED_INT_24_8_MESA: GlEnum = GlEnum(0x8751);
pub const GL_UNSIGNED_INT_8_24_REV_MESA: GlEnum = GlEnum(0x8752);
pub const GL_UNSIGNED_SHORT_15_1_MESA: GlEnum = GlEnum(0x8753);
pub const GL_UNSIGNED_SHORT_1_15_REV_MESA: GlEnum = GlEnum(0x8754);
pub const GL_TRACE_MASK_MESA: GlEnum = GlEnum(0x8755);
pub const GL_TRACE_NAME_MESA: GlEnum = GlEnum(0x8756);
pub const GL_YCBCR_MESA: GlEnum = GlEnum(0x8757);
pub const GL_PACK_INVERT_MESA: GlEnum = GlEnum(0x8758);
/// NOT an alias. Accidental reuse of GL_TEXTURE_1D_STACK_MESAX
pub const GL_DEBUG_OBJECT_MESA: GlEnum = GlEnum(0x8759);
pub const GL_TEXTURE_1D_STACK_MESAX: GlEnum = GlEnum(0x8759);
/// NOT an alias. Accidental reuse of GL_TEXTURE_2D_STACK_MESAX
pub const GL_DEBUG_PRINT_MESA: GlEnum = GlEnum(0x875A);
pub const GL_TEXTURE_2D_STACK_MESAX: GlEnum = GlEnum(0x875A);
/// NOT an alias. Accidental reuse of GL_PROXY_TEXTURE_1D_STACK_MESAX
pub const GL_DEBUG_ASSERT_MESA: GlEnum = GlEnum(0x875B);
pub const GL_PROXY_TEXTURE_1D_STACK_MESAX: GlEnum = GlEnum(0x875B);
pub const GL_PROXY_TEXTURE_2D_STACK_MESAX: GlEnum = GlEnum(0x875C);
pub const GL_TEXTURE_1D_STACK_BINDING_MESAX: GlEnum = GlEnum(0x875D);
pub const GL_TEXTURE_2D_STACK_BINDING_MESAX: GlEnum = GlEnum(0x875E);
pub const GL_PROGRAM_BINARY_FORMAT_MESA: GlEnum = GlEnum(0x875F);
/// 
/// * Group: [`ArrayObjectUsageATI`]
pub const GL_STATIC_ATI: GlEnum = GlEnum(0x8760);
/// 
/// * Group: [`ArrayObjectUsageATI`]
pub const GL_DYNAMIC_ATI: GlEnum = GlEnum(0x8761);
/// 
/// * Group: [`PreserveModeATI`]
pub const GL_PRESERVE_ATI: GlEnum = GlEnum(0x8762);
/// 
/// * Group: [`PreserveModeATI`]
pub const GL_DISCARD_ATI: GlEnum = GlEnum(0x8763);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_SIZE: GlEnum = GlEnum(0x8764);
/// 
/// * Group: [`BufferPNameARB`]
pub const GL_BUFFER_SIZE_ARB: GlEnum = GlEnum(0x8764);
/// 
/// * Group: [`ArrayObjectPNameATI`]
pub const GL_OBJECT_BUFFER_SIZE_ATI: GlEnum = GlEnum(0x8764);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_USAGE: GlEnum = GlEnum(0x8765);
/// 
/// * Group: [`BufferPNameARB`]
pub const GL_BUFFER_USAGE_ARB: GlEnum = GlEnum(0x8765);
/// 
/// * Group: [`ArrayObjectPNameATI`]
pub const GL_OBJECT_BUFFER_USAGE_ATI: GlEnum = GlEnum(0x8765);
pub const GL_ARRAY_OBJECT_BUFFER_ATI: GlEnum = GlEnum(0x8766);
pub const GL_ARRAY_OBJECT_OFFSET_ATI: GlEnum = GlEnum(0x8767);
pub const GL_ELEMENT_ARRAY_ATI: GlEnum = GlEnum(0x8768);
pub const GL_ELEMENT_ARRAY_TYPE_ATI: GlEnum = GlEnum(0x8769);
pub const GL_ELEMENT_ARRAY_POINTER_ATI: GlEnum = GlEnum(0x876A);
pub const GL_MAX_VERTEX_STREAMS_ATI: GlEnum = GlEnum(0x876B);
/// 
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM0_ATI: GlEnum = GlEnum(0x876C);
/// 
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM1_ATI: GlEnum = GlEnum(0x876D);
/// 
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM2_ATI: GlEnum = GlEnum(0x876E);
/// 
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM3_ATI: GlEnum = GlEnum(0x876F);
/// 
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM4_ATI: GlEnum = GlEnum(0x8770);
/// 
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM5_ATI: GlEnum = GlEnum(0x8771);
/// 
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM6_ATI: GlEnum = GlEnum(0x8772);
/// 
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM7_ATI: GlEnum = GlEnum(0x8773);
pub const GL_VERTEX_SOURCE_ATI: GlEnum = GlEnum(0x8774);
/// 
/// * Group: [`GetTexBumpParameterATI`], [`TexBumpParameterATI`]
pub const GL_BUMP_ROT_MATRIX_ATI: GlEnum = GlEnum(0x8775);
/// 
/// * Group: [`GetTexBumpParameterATI`]
pub const GL_BUMP_ROT_MATRIX_SIZE_ATI: GlEnum = GlEnum(0x8776);
/// 
/// * Group: [`GetTexBumpParameterATI`]
pub const GL_BUMP_NUM_TEX_UNITS_ATI: GlEnum = GlEnum(0x8777);
/// 
/// * Group: [`GetTexBumpParameterATI`]
pub const GL_BUMP_TEX_UNITS_ATI: GlEnum = GlEnum(0x8778);
pub const GL_DUDV_ATI: GlEnum = GlEnum(0x8779);
pub const GL_DU8DV8_ATI: GlEnum = GlEnum(0x877A);
pub const GL_BUMP_ENVMAP_ATI: GlEnum = GlEnum(0x877B);
pub const GL_BUMP_TARGET_ATI: GlEnum = GlEnum(0x877C);
pub const GL_VERTEX_SHADER_EXT: GlEnum = GlEnum(0x8780);
pub const GL_VERTEX_SHADER_BINDING_EXT: GlEnum = GlEnum(0x8781);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_INDEX_EXT: GlEnum = GlEnum(0x8782);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_NEGATE_EXT: GlEnum = GlEnum(0x8783);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_DOT3_EXT: GlEnum = GlEnum(0x8784);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_DOT4_EXT: GlEnum = GlEnum(0x8785);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MUL_EXT: GlEnum = GlEnum(0x8786);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_ADD_EXT: GlEnum = GlEnum(0x8787);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MADD_EXT: GlEnum = GlEnum(0x8788);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_FRAC_EXT: GlEnum = GlEnum(0x8789);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MAX_EXT: GlEnum = GlEnum(0x878A);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MIN_EXT: GlEnum = GlEnum(0x878B);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_SET_GE_EXT: GlEnum = GlEnum(0x878C);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_SET_LT_EXT: GlEnum = GlEnum(0x878D);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_CLAMP_EXT: GlEnum = GlEnum(0x878E);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_FLOOR_EXT: GlEnum = GlEnum(0x878F);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_ROUND_EXT: GlEnum = GlEnum(0x8790);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_EXP_BASE_2_EXT: GlEnum = GlEnum(0x8791);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_LOG_BASE_2_EXT: GlEnum = GlEnum(0x8792);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_POWER_EXT: GlEnum = GlEnum(0x8793);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_RECIP_EXT: GlEnum = GlEnum(0x8794);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_RECIP_SQRT_EXT: GlEnum = GlEnum(0x8795);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_SUB_EXT: GlEnum = GlEnum(0x8796);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_CROSS_PRODUCT_EXT: GlEnum = GlEnum(0x8797);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MULTIPLY_MATRIX_EXT: GlEnum = GlEnum(0x8798);
/// 
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MOV_EXT: GlEnum = GlEnum(0x8799);
pub const GL_OUTPUT_VERTEX_EXT: GlEnum = GlEnum(0x879A);
pub const GL_OUTPUT_COLOR0_EXT: GlEnum = GlEnum(0x879B);
pub const GL_OUTPUT_COLOR1_EXT: GlEnum = GlEnum(0x879C);
pub const GL_OUTPUT_TEXTURE_COORD0_EXT: GlEnum = GlEnum(0x879D);
pub const GL_OUTPUT_TEXTURE_COORD1_EXT: GlEnum = GlEnum(0x879E);
pub const GL_OUTPUT_TEXTURE_COORD2_EXT: GlEnum = GlEnum(0x879F);
pub const GL_OUTPUT_TEXTURE_COORD3_EXT: GlEnum = GlEnum(0x87A0);
pub const GL_OUTPUT_TEXTURE_COORD4_EXT: GlEnum = GlEnum(0x87A1);
pub const GL_OUTPUT_TEXTURE_COORD5_EXT: GlEnum = GlEnum(0x87A2);
pub const GL_OUTPUT_TEXTURE_COORD6_EXT: GlEnum = GlEnum(0x87A3);
pub const GL_OUTPUT_TEXTURE_COORD7_EXT: GlEnum = GlEnum(0x87A4);
pub const GL_OUTPUT_TEXTURE_COORD8_EXT: GlEnum = GlEnum(0x87A5);
pub const GL_OUTPUT_TEXTURE_COORD9_EXT: GlEnum = GlEnum(0x87A6);
pub const GL_OUTPUT_TEXTURE_COORD10_EXT: GlEnum = GlEnum(0x87A7);
pub const GL_OUTPUT_TEXTURE_COORD11_EXT: GlEnum = GlEnum(0x87A8);
pub const GL_OUTPUT_TEXTURE_COORD12_EXT: GlEnum = GlEnum(0x87A9);
pub const GL_OUTPUT_TEXTURE_COORD13_EXT: GlEnum = GlEnum(0x87AA);
pub const GL_OUTPUT_TEXTURE_COORD14_EXT: GlEnum = GlEnum(0x87AB);
pub const GL_OUTPUT_TEXTURE_COORD15_EXT: GlEnum = GlEnum(0x87AC);
pub const GL_OUTPUT_TEXTURE_COORD16_EXT: GlEnum = GlEnum(0x87AD);
pub const GL_OUTPUT_TEXTURE_COORD17_EXT: GlEnum = GlEnum(0x87AE);
pub const GL_OUTPUT_TEXTURE_COORD18_EXT: GlEnum = GlEnum(0x87AF);
pub const GL_OUTPUT_TEXTURE_COORD19_EXT: GlEnum = GlEnum(0x87B0);
pub const GL_OUTPUT_TEXTURE_COORD20_EXT: GlEnum = GlEnum(0x87B1);
pub const GL_OUTPUT_TEXTURE_COORD21_EXT: GlEnum = GlEnum(0x87B2);
pub const GL_OUTPUT_TEXTURE_COORD22_EXT: GlEnum = GlEnum(0x87B3);
pub const GL_OUTPUT_TEXTURE_COORD23_EXT: GlEnum = GlEnum(0x87B4);
pub const GL_OUTPUT_TEXTURE_COORD24_EXT: GlEnum = GlEnum(0x87B5);
pub const GL_OUTPUT_TEXTURE_COORD25_EXT: GlEnum = GlEnum(0x87B6);
pub const GL_OUTPUT_TEXTURE_COORD26_EXT: GlEnum = GlEnum(0x87B7);
pub const GL_OUTPUT_TEXTURE_COORD27_EXT: GlEnum = GlEnum(0x87B8);
pub const GL_OUTPUT_TEXTURE_COORD28_EXT: GlEnum = GlEnum(0x87B9);
pub const GL_OUTPUT_TEXTURE_COORD29_EXT: GlEnum = GlEnum(0x87BA);
pub const GL_OUTPUT_TEXTURE_COORD30_EXT: GlEnum = GlEnum(0x87BB);
pub const GL_OUTPUT_TEXTURE_COORD31_EXT: GlEnum = GlEnum(0x87BC);
pub const GL_OUTPUT_FOG_EXT: GlEnum = GlEnum(0x87BD);
/// 
/// * Group: [`DataTypeEXT`]
pub const GL_SCALAR_EXT: GlEnum = GlEnum(0x87BE);
/// 
/// * Group: [`DataTypeEXT`]
pub const GL_VECTOR_EXT: GlEnum = GlEnum(0x87BF);
/// 
/// * Group: [`DataTypeEXT`]
pub const GL_MATRIX_EXT: GlEnum = GlEnum(0x87C0);
/// 
/// * Group: [`VertexShaderStorageTypeEXT`]
pub const GL_VARIANT_EXT: GlEnum = GlEnum(0x87C1);
/// 
/// * Group: [`VertexShaderStorageTypeEXT`]
pub const GL_INVARIANT_EXT: GlEnum = GlEnum(0x87C2);
/// 
/// * Group: [`VertexShaderStorageTypeEXT`]
pub const GL_LOCAL_CONSTANT_EXT: GlEnum = GlEnum(0x87C3);
/// 
/// * Group: [`VertexShaderStorageTypeEXT`]
pub const GL_LOCAL_EXT: GlEnum = GlEnum(0x87C4);
pub const GL_MAX_VERTEX_SHADER_INSTRUCTIONS_EXT: GlEnum = GlEnum(0x87C5);
pub const GL_MAX_VERTEX_SHADER_VARIANTS_EXT: GlEnum = GlEnum(0x87C6);
pub const GL_MAX_VERTEX_SHADER_INVARIANTS_EXT: GlEnum = GlEnum(0x87C7);
pub const GL_MAX_VERTEX_SHADER_LOCAL_CONSTANTS_EXT: GlEnum = GlEnum(0x87C8);
pub const GL_MAX_VERTEX_SHADER_LOCALS_EXT: GlEnum = GlEnum(0x87C9);
pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_INSTRUCTIONS_EXT: GlEnum = GlEnum(0x87CA);
pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_VARIANTS_EXT: GlEnum = GlEnum(0x87CB);
pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_LOCAL_CONSTANTS_EXT: GlEnum = GlEnum(0x87CC);
pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_INVARIANTS_EXT: GlEnum = GlEnum(0x87CD);
pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_LOCALS_EXT: GlEnum = GlEnum(0x87CE);
pub const GL_VERTEX_SHADER_INSTRUCTIONS_EXT: GlEnum = GlEnum(0x87CF);
pub const GL_VERTEX_SHADER_VARIANTS_EXT: GlEnum = GlEnum(0x87D0);
pub const GL_VERTEX_SHADER_INVARIANTS_EXT: GlEnum = GlEnum(0x87D1);
pub const GL_VERTEX_SHADER_LOCAL_CONSTANTS_EXT: GlEnum = GlEnum(0x87D2);
pub const GL_VERTEX_SHADER_LOCALS_EXT: GlEnum = GlEnum(0x87D3);
pub const GL_VERTEX_SHADER_OPTIMIZED_EXT: GlEnum = GlEnum(0x87D4);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_X_EXT: GlEnum = GlEnum(0x87D5);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_Y_EXT: GlEnum = GlEnum(0x87D6);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_Z_EXT: GlEnum = GlEnum(0x87D7);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_W_EXT: GlEnum = GlEnum(0x87D8);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_X_EXT: GlEnum = GlEnum(0x87D9);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_Y_EXT: GlEnum = GlEnum(0x87DA);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_Z_EXT: GlEnum = GlEnum(0x87DB);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_W_EXT: GlEnum = GlEnum(0x87DC);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_ZERO_EXT: GlEnum = GlEnum(0x87DD);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_ONE_EXT: GlEnum = GlEnum(0x87DE);
/// 
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_ONE_EXT: GlEnum = GlEnum(0x87DF);
/// 
/// * Group: [`ParameterRangeEXT`]
pub const GL_NORMALIZED_RANGE_EXT: GlEnum = GlEnum(0x87E0);
/// 
/// * Group: [`ParameterRangeEXT`]
pub const GL_FULL_RANGE_EXT: GlEnum = GlEnum(0x87E1);
/// 
/// * Group: [`VertexShaderParameterEXT`]
pub const GL_CURRENT_VERTEX_EXT: GlEnum = GlEnum(0x87E2);
/// 
/// * Group: [`VertexShaderParameterEXT`]
pub const GL_MVP_MATRIX_EXT: GlEnum = GlEnum(0x87E3);
/// 
/// * Group: [`GetVariantValueEXT`]
pub const GL_VARIANT_VALUE_EXT: GlEnum = GlEnum(0x87E4);
/// 
/// * Group: [`GetVariantValueEXT`]
pub const GL_VARIANT_DATATYPE_EXT: GlEnum = GlEnum(0x87E5);
/// 
/// * Group: [`GetVariantValueEXT`]
pub const GL_VARIANT_ARRAY_STRIDE_EXT: GlEnum = GlEnum(0x87E6);
/// 
/// * Group: [`GetVariantValueEXT`]
pub const GL_VARIANT_ARRAY_TYPE_EXT: GlEnum = GlEnum(0x87E7);
/// 
/// * Group: [`VariantCapEXT`]
pub const GL_VARIANT_ARRAY_EXT: GlEnum = GlEnum(0x87E8);
pub const GL_VARIANT_ARRAY_POINTER_EXT: GlEnum = GlEnum(0x87E9);
pub const GL_INVARIANT_VALUE_EXT: GlEnum = GlEnum(0x87EA);
pub const GL_INVARIANT_DATATYPE_EXT: GlEnum = GlEnum(0x87EB);
pub const GL_LOCAL_CONSTANT_VALUE_EXT: GlEnum = GlEnum(0x87EC);
pub const GL_LOCAL_CONSTANT_DATATYPE_EXT: GlEnum = GlEnum(0x87ED);
pub const GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD: GlEnum = GlEnum(0x87EE);
pub const GL_PN_TRIANGLES_ATI: GlEnum = GlEnum(0x87F0);
pub const GL_MAX_PN_TRIANGLES_TESSELATION_LEVEL_ATI: GlEnum = GlEnum(0x87F1);
/// 
/// * Group: [`PNTrianglesPNameATI`]
pub const GL_PN_TRIANGLES_POINT_MODE_ATI: GlEnum = GlEnum(0x87F2);
/// 
/// * Group: [`PNTrianglesPNameATI`]
pub const GL_PN_TRIANGLES_NORMAL_MODE_ATI: GlEnum = GlEnum(0x87F3);
/// 
/// * Group: [`PNTrianglesPNameATI`]
pub const GL_PN_TRIANGLES_TESSELATION_LEVEL_ATI: GlEnum = GlEnum(0x87F4);
pub const GL_PN_TRIANGLES_POINT_MODE_LINEAR_ATI: GlEnum = GlEnum(0x87F5);
pub const GL_PN_TRIANGLES_POINT_MODE_CUBIC_ATI: GlEnum = GlEnum(0x87F6);
pub const GL_PN_TRIANGLES_NORMAL_MODE_LINEAR_ATI: GlEnum = GlEnum(0x87F7);
pub const GL_PN_TRIANGLES_NORMAL_MODE_QUADRATIC_ATI: GlEnum = GlEnum(0x87F8);
pub const GL_3DC_X_AMD: GlEnum = GlEnum(0x87F9);
pub const GL_3DC_XY_AMD: GlEnum = GlEnum(0x87FA);
pub const GL_VBO_FREE_MEMORY_ATI: GlEnum = GlEnum(0x87FB);
pub const GL_TEXTURE_FREE_MEMORY_ATI: GlEnum = GlEnum(0x87FC);
pub const GL_RENDERBUFFER_FREE_MEMORY_ATI: GlEnum = GlEnum(0x87FD);
/// 
/// * Group: [`GetPName`]
pub const GL_NUM_PROGRAM_BINARY_FORMATS: GlEnum = GlEnum(0x87FE);
pub const GL_NUM_PROGRAM_BINARY_FORMATS_OES: GlEnum = GlEnum(0x87FE);
/// 
/// * Group: [`GetPName`]
pub const GL_PROGRAM_BINARY_FORMATS: GlEnum = GlEnum(0x87FF);
pub const GL_PROGRAM_BINARY_FORMATS_OES: GlEnum = GlEnum(0x87FF);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_FUNC: GlEnum = GlEnum(0x8800);
pub const GL_STENCIL_BACK_FUNC_ATI: GlEnum = GlEnum(0x8800);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_FAIL: GlEnum = GlEnum(0x8801);
pub const GL_STENCIL_BACK_FAIL_ATI: GlEnum = GlEnum(0x8801);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GlEnum = GlEnum(0x8802);
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL_ATI: GlEnum = GlEnum(0x8802);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GlEnum = GlEnum(0x8803);
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS_ATI: GlEnum = GlEnum(0x8803);
/// 
/// * Group: [`ProgramTarget`]
pub const GL_FRAGMENT_PROGRAM_ARB: GlEnum = GlEnum(0x8804);
pub const GL_PROGRAM_ALU_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x8805);
pub const GL_PROGRAM_TEX_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x8806);
pub const GL_PROGRAM_TEX_INDIRECTIONS_ARB: GlEnum = GlEnum(0x8807);
pub const GL_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x8808);
pub const GL_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x8809);
pub const GL_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB: GlEnum = GlEnum(0x880A);
pub const GL_MAX_PROGRAM_ALU_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x880B);
pub const GL_MAX_PROGRAM_TEX_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x880C);
pub const GL_MAX_PROGRAM_TEX_INDIRECTIONS_ARB: GlEnum = GlEnum(0x880D);
pub const GL_MAX_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x880E);
pub const GL_MAX_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x880F);
pub const GL_MAX_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB: GlEnum = GlEnum(0x8810);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA32F: GlEnum = GlEnum(0x8814);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA32F_ARB: GlEnum = GlEnum(0x8814);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA32F_EXT: GlEnum = GlEnum(0x8814);
pub const GL_RGBA_FLOAT32_APPLE: GlEnum = GlEnum(0x8814);
pub const GL_RGBA_FLOAT32_ATI: GlEnum = GlEnum(0x8814);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB32F: GlEnum = GlEnum(0x8815);
pub const GL_RGB32F_ARB: GlEnum = GlEnum(0x8815);
pub const GL_RGB32F_EXT: GlEnum = GlEnum(0x8815);
pub const GL_RGB_FLOAT32_APPLE: GlEnum = GlEnum(0x8815);
pub const GL_RGB_FLOAT32_ATI: GlEnum = GlEnum(0x8815);
pub const GL_ALPHA32F_ARB: GlEnum = GlEnum(0x8816);
pub const GL_ALPHA32F_EXT: GlEnum = GlEnum(0x8816);
pub const GL_ALPHA_FLOAT32_APPLE: GlEnum = GlEnum(0x8816);
pub const GL_ALPHA_FLOAT32_ATI: GlEnum = GlEnum(0x8816);
pub const GL_INTENSITY32F_ARB: GlEnum = GlEnum(0x8817);
pub const GL_INTENSITY_FLOAT32_APPLE: GlEnum = GlEnum(0x8817);
pub const GL_INTENSITY_FLOAT32_ATI: GlEnum = GlEnum(0x8817);
pub const GL_LUMINANCE32F_ARB: GlEnum = GlEnum(0x8818);
pub const GL_LUMINANCE32F_EXT: GlEnum = GlEnum(0x8818);
pub const GL_LUMINANCE_FLOAT32_APPLE: GlEnum = GlEnum(0x8818);
pub const GL_LUMINANCE_FLOAT32_ATI: GlEnum = GlEnum(0x8818);
pub const GL_LUMINANCE_ALPHA32F_ARB: GlEnum = GlEnum(0x8819);
pub const GL_LUMINANCE_ALPHA32F_EXT: GlEnum = GlEnum(0x8819);
pub const GL_LUMINANCE_ALPHA_FLOAT32_APPLE: GlEnum = GlEnum(0x8819);
pub const GL_LUMINANCE_ALPHA_FLOAT32_ATI: GlEnum = GlEnum(0x8819);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA16F: GlEnum = GlEnum(0x881A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA16F_ARB: GlEnum = GlEnum(0x881A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA16F_EXT: GlEnum = GlEnum(0x881A);
pub const GL_RGBA_FLOAT16_APPLE: GlEnum = GlEnum(0x881A);
pub const GL_RGBA_FLOAT16_ATI: GlEnum = GlEnum(0x881A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16F: GlEnum = GlEnum(0x881B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16F_ARB: GlEnum = GlEnum(0x881B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16F_EXT: GlEnum = GlEnum(0x881B);
pub const GL_RGB_FLOAT16_APPLE: GlEnum = GlEnum(0x881B);
pub const GL_RGB_FLOAT16_ATI: GlEnum = GlEnum(0x881B);
pub const GL_ALPHA16F_ARB: GlEnum = GlEnum(0x881C);
pub const GL_ALPHA16F_EXT: GlEnum = GlEnum(0x881C);
pub const GL_ALPHA_FLOAT16_APPLE: GlEnum = GlEnum(0x881C);
pub const GL_ALPHA_FLOAT16_ATI: GlEnum = GlEnum(0x881C);
pub const GL_INTENSITY16F_ARB: GlEnum = GlEnum(0x881D);
pub const GL_INTENSITY_FLOAT16_APPLE: GlEnum = GlEnum(0x881D);
pub const GL_INTENSITY_FLOAT16_ATI: GlEnum = GlEnum(0x881D);
pub const GL_LUMINANCE16F_ARB: GlEnum = GlEnum(0x881E);
pub const GL_LUMINANCE16F_EXT: GlEnum = GlEnum(0x881E);
pub const GL_LUMINANCE_FLOAT16_APPLE: GlEnum = GlEnum(0x881E);
pub const GL_LUMINANCE_FLOAT16_ATI: GlEnum = GlEnum(0x881E);
pub const GL_LUMINANCE_ALPHA16F_ARB: GlEnum = GlEnum(0x881F);
pub const GL_LUMINANCE_ALPHA16F_EXT: GlEnum = GlEnum(0x881F);
pub const GL_LUMINANCE_ALPHA_FLOAT16_APPLE: GlEnum = GlEnum(0x881F);
pub const GL_LUMINANCE_ALPHA_FLOAT16_ATI: GlEnum = GlEnum(0x881F);
pub const GL_RGBA_FLOAT_MODE_ARB: GlEnum = GlEnum(0x8820);
pub const GL_RGBA_FLOAT_MODE_ATI: GlEnum = GlEnum(0x8820);
pub const GL_WRITEONLY_RENDERING_QCOM: GlEnum = GlEnum(0x8823);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_DRAW_BUFFERS: GlEnum = GlEnum(0x8824);
pub const GL_MAX_DRAW_BUFFERS_ARB: GlEnum = GlEnum(0x8824);
pub const GL_MAX_DRAW_BUFFERS_ATI: GlEnum = GlEnum(0x8824);
pub const GL_MAX_DRAW_BUFFERS_EXT: GlEnum = GlEnum(0x8824);
pub const GL_MAX_DRAW_BUFFERS_NV: GlEnum = GlEnum(0x8824);
pub const GL_DRAW_BUFFER0: GlEnum = GlEnum(0x8825);
pub const GL_DRAW_BUFFER0_ARB: GlEnum = GlEnum(0x8825);
pub const GL_DRAW_BUFFER0_ATI: GlEnum = GlEnum(0x8825);
pub const GL_DRAW_BUFFER0_EXT: GlEnum = GlEnum(0x8825);
pub const GL_DRAW_BUFFER0_NV: GlEnum = GlEnum(0x8825);
pub const GL_DRAW_BUFFER1: GlEnum = GlEnum(0x8826);
pub const GL_DRAW_BUFFER1_ARB: GlEnum = GlEnum(0x8826);
pub const GL_DRAW_BUFFER1_ATI: GlEnum = GlEnum(0x8826);
pub const GL_DRAW_BUFFER1_EXT: GlEnum = GlEnum(0x8826);
pub const GL_DRAW_BUFFER1_NV: GlEnum = GlEnum(0x8826);
pub const GL_DRAW_BUFFER2: GlEnum = GlEnum(0x8827);
pub const GL_DRAW_BUFFER2_ARB: GlEnum = GlEnum(0x8827);
pub const GL_DRAW_BUFFER2_ATI: GlEnum = GlEnum(0x8827);
pub const GL_DRAW_BUFFER2_EXT: GlEnum = GlEnum(0x8827);
pub const GL_DRAW_BUFFER2_NV: GlEnum = GlEnum(0x8827);
pub const GL_DRAW_BUFFER3: GlEnum = GlEnum(0x8828);
pub const GL_DRAW_BUFFER3_ARB: GlEnum = GlEnum(0x8828);
pub const GL_DRAW_BUFFER3_ATI: GlEnum = GlEnum(0x8828);
pub const GL_DRAW_BUFFER3_EXT: GlEnum = GlEnum(0x8828);
pub const GL_DRAW_BUFFER3_NV: GlEnum = GlEnum(0x8828);
pub const GL_DRAW_BUFFER4: GlEnum = GlEnum(0x8829);
pub const GL_DRAW_BUFFER4_ARB: GlEnum = GlEnum(0x8829);
pub const GL_DRAW_BUFFER4_ATI: GlEnum = GlEnum(0x8829);
pub const GL_DRAW_BUFFER4_EXT: GlEnum = GlEnum(0x8829);
pub const GL_DRAW_BUFFER4_NV: GlEnum = GlEnum(0x8829);
pub const GL_DRAW_BUFFER5: GlEnum = GlEnum(0x882A);
pub const GL_DRAW_BUFFER5_ARB: GlEnum = GlEnum(0x882A);
pub const GL_DRAW_BUFFER5_ATI: GlEnum = GlEnum(0x882A);
pub const GL_DRAW_BUFFER5_EXT: GlEnum = GlEnum(0x882A);
pub const GL_DRAW_BUFFER5_NV: GlEnum = GlEnum(0x882A);
pub const GL_DRAW_BUFFER6: GlEnum = GlEnum(0x882B);
pub const GL_DRAW_BUFFER6_ARB: GlEnum = GlEnum(0x882B);
pub const GL_DRAW_BUFFER6_ATI: GlEnum = GlEnum(0x882B);
pub const GL_DRAW_BUFFER6_EXT: GlEnum = GlEnum(0x882B);
pub const GL_DRAW_BUFFER6_NV: GlEnum = GlEnum(0x882B);
pub const GL_DRAW_BUFFER7: GlEnum = GlEnum(0x882C);
pub const GL_DRAW_BUFFER7_ARB: GlEnum = GlEnum(0x882C);
pub const GL_DRAW_BUFFER7_ATI: GlEnum = GlEnum(0x882C);
pub const GL_DRAW_BUFFER7_EXT: GlEnum = GlEnum(0x882C);
pub const GL_DRAW_BUFFER7_NV: GlEnum = GlEnum(0x882C);
pub const GL_DRAW_BUFFER8: GlEnum = GlEnum(0x882D);
pub const GL_DRAW_BUFFER8_ARB: GlEnum = GlEnum(0x882D);
pub const GL_DRAW_BUFFER8_ATI: GlEnum = GlEnum(0x882D);
pub const GL_DRAW_BUFFER8_EXT: GlEnum = GlEnum(0x882D);
pub const GL_DRAW_BUFFER8_NV: GlEnum = GlEnum(0x882D);
pub const GL_DRAW_BUFFER9: GlEnum = GlEnum(0x882E);
pub const GL_DRAW_BUFFER9_ARB: GlEnum = GlEnum(0x882E);
pub const GL_DRAW_BUFFER9_ATI: GlEnum = GlEnum(0x882E);
pub const GL_DRAW_BUFFER9_EXT: GlEnum = GlEnum(0x882E);
pub const GL_DRAW_BUFFER9_NV: GlEnum = GlEnum(0x882E);
pub const GL_DRAW_BUFFER10: GlEnum = GlEnum(0x882F);
pub const GL_DRAW_BUFFER10_ARB: GlEnum = GlEnum(0x882F);
pub const GL_DRAW_BUFFER10_ATI: GlEnum = GlEnum(0x882F);
pub const GL_DRAW_BUFFER10_EXT: GlEnum = GlEnum(0x882F);
pub const GL_DRAW_BUFFER10_NV: GlEnum = GlEnum(0x882F);
pub const GL_DRAW_BUFFER11: GlEnum = GlEnum(0x8830);
pub const GL_DRAW_BUFFER11_ARB: GlEnum = GlEnum(0x8830);
pub const GL_DRAW_BUFFER11_ATI: GlEnum = GlEnum(0x8830);
pub const GL_DRAW_BUFFER11_EXT: GlEnum = GlEnum(0x8830);
pub const GL_DRAW_BUFFER11_NV: GlEnum = GlEnum(0x8830);
pub const GL_DRAW_BUFFER12: GlEnum = GlEnum(0x8831);
pub const GL_DRAW_BUFFER12_ARB: GlEnum = GlEnum(0x8831);
pub const GL_DRAW_BUFFER12_ATI: GlEnum = GlEnum(0x8831);
pub const GL_DRAW_BUFFER12_EXT: GlEnum = GlEnum(0x8831);
pub const GL_DRAW_BUFFER12_NV: GlEnum = GlEnum(0x8831);
pub const GL_DRAW_BUFFER13: GlEnum = GlEnum(0x8832);
pub const GL_DRAW_BUFFER13_ARB: GlEnum = GlEnum(0x8832);
pub const GL_DRAW_BUFFER13_ATI: GlEnum = GlEnum(0x8832);
pub const GL_DRAW_BUFFER13_EXT: GlEnum = GlEnum(0x8832);
pub const GL_DRAW_BUFFER13_NV: GlEnum = GlEnum(0x8832);
pub const GL_DRAW_BUFFER14: GlEnum = GlEnum(0x8833);
pub const GL_DRAW_BUFFER14_ARB: GlEnum = GlEnum(0x8833);
pub const GL_DRAW_BUFFER14_ATI: GlEnum = GlEnum(0x8833);
pub const GL_DRAW_BUFFER14_EXT: GlEnum = GlEnum(0x8833);
pub const GL_DRAW_BUFFER14_NV: GlEnum = GlEnum(0x8833);
pub const GL_DRAW_BUFFER15: GlEnum = GlEnum(0x8834);
pub const GL_DRAW_BUFFER15_ARB: GlEnum = GlEnum(0x8834);
pub const GL_DRAW_BUFFER15_ATI: GlEnum = GlEnum(0x8834);
pub const GL_DRAW_BUFFER15_EXT: GlEnum = GlEnum(0x8834);
pub const GL_DRAW_BUFFER15_NV: GlEnum = GlEnum(0x8834);
pub const GL_COLOR_CLEAR_UNCLAMPED_VALUE_ATI: GlEnum = GlEnum(0x8835);
/// Defined by Mesa but not ATI
pub const GL_COMPRESSED_LUMINANCE_ALPHA_3DC_ATI: GlEnum = GlEnum(0x8837);
/// 
/// * Group: [`GetPName`]
pub const GL_BLEND_EQUATION_ALPHA: GlEnum = GlEnum(0x883D);
pub const GL_BLEND_EQUATION_ALPHA_EXT: GlEnum = GlEnum(0x883D);
pub const GL_BLEND_EQUATION_ALPHA_OES: GlEnum = GlEnum(0x883D);
pub const GL_SUBSAMPLE_DISTANCE_AMD: GlEnum = GlEnum(0x883F);
pub const GL_MATRIX_PALETTE_ARB: GlEnum = GlEnum(0x8840);
pub const GL_MATRIX_PALETTE_OES: GlEnum = GlEnum(0x8840);
pub const GL_MAX_MATRIX_PALETTE_STACK_DEPTH_ARB: GlEnum = GlEnum(0x8841);
pub const GL_MAX_PALETTE_MATRICES_ARB: GlEnum = GlEnum(0x8842);
pub const GL_MAX_PALETTE_MATRICES_OES: GlEnum = GlEnum(0x8842);
pub const GL_CURRENT_PALETTE_MATRIX_ARB: GlEnum = GlEnum(0x8843);
pub const GL_CURRENT_PALETTE_MATRIX_OES: GlEnum = GlEnum(0x8843);
pub const GL_MATRIX_INDEX_ARRAY_ARB: GlEnum = GlEnum(0x8844);
pub const GL_MATRIX_INDEX_ARRAY_OES: GlEnum = GlEnum(0x8844);
pub const GL_CURRENT_MATRIX_INDEX_ARB: GlEnum = GlEnum(0x8845);
pub const GL_MATRIX_INDEX_ARRAY_SIZE_ARB: GlEnum = GlEnum(0x8846);
pub const GL_MATRIX_INDEX_ARRAY_SIZE_OES: GlEnum = GlEnum(0x8846);
pub const GL_MATRIX_INDEX_ARRAY_TYPE_ARB: GlEnum = GlEnum(0x8847);
pub const GL_MATRIX_INDEX_ARRAY_TYPE_OES: GlEnum = GlEnum(0x8847);
pub const GL_MATRIX_INDEX_ARRAY_STRIDE_ARB: GlEnum = GlEnum(0x8848);
pub const GL_MATRIX_INDEX_ARRAY_STRIDE_OES: GlEnum = GlEnum(0x8848);
pub const GL_MATRIX_INDEX_ARRAY_POINTER_ARB: GlEnum = GlEnum(0x8849);
pub const GL_MATRIX_INDEX_ARRAY_POINTER_OES: GlEnum = GlEnum(0x8849);
pub const GL_TEXTURE_DEPTH_SIZE: GlEnum = GlEnum(0x884A);
pub const GL_TEXTURE_DEPTH_SIZE_ARB: GlEnum = GlEnum(0x884A);
pub const GL_DEPTH_TEXTURE_MODE: GlEnum = GlEnum(0x884B);
pub const GL_DEPTH_TEXTURE_MODE_ARB: GlEnum = GlEnum(0x884B);
/// 
/// * Group: [`SamplerParameterI`], [`TextureParameterName`]
pub const GL_TEXTURE_COMPARE_MODE: GlEnum = GlEnum(0x884C);
pub const GL_TEXTURE_COMPARE_MODE_ARB: GlEnum = GlEnum(0x884C);
pub const GL_TEXTURE_COMPARE_MODE_EXT: GlEnum = GlEnum(0x884C);
/// 
/// * Group: [`SamplerParameterI`], [`TextureParameterName`]
pub const GL_TEXTURE_COMPARE_FUNC: GlEnum = GlEnum(0x884D);
pub const GL_TEXTURE_COMPARE_FUNC_ARB: GlEnum = GlEnum(0x884D);
pub const GL_TEXTURE_COMPARE_FUNC_EXT: GlEnum = GlEnum(0x884D);
/// 
/// * Group: [`TextureCompareMode`]
pub const GL_COMPARE_R_TO_TEXTURE: GlEnum = GlEnum(0x884E);
pub const GL_COMPARE_R_TO_TEXTURE_ARB: GlEnum = GlEnum(0x884E);
pub const GL_COMPARE_REF_DEPTH_TO_TEXTURE_EXT: GlEnum = GlEnum(0x884E);
/// 
/// * Group: [`TextureCompareMode`]
/// * Alias Of: [`GL_COMPARE_R_TO_TEXTURE`]
pub const GL_COMPARE_REF_TO_TEXTURE: GlEnum = GL_COMPARE_R_TO_TEXTURE;
pub const GL_COMPARE_REF_TO_TEXTURE_EXT: GlEnum = GlEnum(0x884E);
/// 
/// * Group: [`EnableCap`]
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GlEnum = GlEnum(0x884F);
pub const GL_OFFSET_PROJECTIVE_TEXTURE_2D_NV: GlEnum = GlEnum(0x8850);
pub const GL_OFFSET_PROJECTIVE_TEXTURE_2D_SCALE_NV: GlEnum = GlEnum(0x8851);
pub const GL_OFFSET_PROJECTIVE_TEXTURE_RECTANGLE_NV: GlEnum = GlEnum(0x8852);
pub const GL_OFFSET_PROJECTIVE_TEXTURE_RECTANGLE_SCALE_NV: GlEnum = GlEnum(0x8853);
pub const GL_OFFSET_HILO_TEXTURE_2D_NV: GlEnum = GlEnum(0x8854);
pub const GL_OFFSET_HILO_TEXTURE_RECTANGLE_NV: GlEnum = GlEnum(0x8855);
pub const GL_OFFSET_HILO_PROJECTIVE_TEXTURE_2D_NV: GlEnum = GlEnum(0x8856);
pub const GL_OFFSET_HILO_PROJECTIVE_TEXTURE_RECTANGLE_NV: GlEnum = GlEnum(0x8857);
pub const GL_DEPENDENT_HILO_TEXTURE_2D_NV: GlEnum = GlEnum(0x8858);
pub const GL_DEPENDENT_RGB_TEXTURE_3D_NV: GlEnum = GlEnum(0x8859);
pub const GL_DEPENDENT_RGB_TEXTURE_CUBE_MAP_NV: GlEnum = GlEnum(0x885A);
pub const GL_DOT_PRODUCT_PASS_THROUGH_NV: GlEnum = GlEnum(0x885B);
pub const GL_DOT_PRODUCT_TEXTURE_1D_NV: GlEnum = GlEnum(0x885C);
pub const GL_DOT_PRODUCT_AFFINE_DEPTH_REPLACE_NV: GlEnum = GlEnum(0x885D);
pub const GL_HILO8_NV: GlEnum = GlEnum(0x885E);
pub const GL_SIGNED_HILO8_NV: GlEnum = GlEnum(0x885F);
pub const GL_FORCE_BLUE_TO_ONE_NV: GlEnum = GlEnum(0x8860);
pub const GL_POINT_SPRITE: GlEnum = GlEnum(0x8861);
pub const GL_POINT_SPRITE_ARB: GlEnum = GlEnum(0x8861);
pub const GL_POINT_SPRITE_NV: GlEnum = GlEnum(0x8861);
pub const GL_POINT_SPRITE_OES: GlEnum = GlEnum(0x8861);
pub const GL_COORD_REPLACE: GlEnum = GlEnum(0x8862);
pub const GL_COORD_REPLACE_ARB: GlEnum = GlEnum(0x8862);
pub const GL_COORD_REPLACE_NV: GlEnum = GlEnum(0x8862);
pub const GL_COORD_REPLACE_OES: GlEnum = GlEnum(0x8862);
pub const GL_POINT_SPRITE_R_MODE_NV: GlEnum = GlEnum(0x8863);
pub const GL_PIXEL_COUNTER_BITS_NV: GlEnum = GlEnum(0x8864);
/// 
/// * Group: [`QueryParameterName`]
pub const GL_QUERY_COUNTER_BITS: GlEnum = GlEnum(0x8864);
pub const GL_QUERY_COUNTER_BITS_ARB: GlEnum = GlEnum(0x8864);
pub const GL_QUERY_COUNTER_BITS_EXT: GlEnum = GlEnum(0x8864);
pub const GL_CURRENT_OCCLUSION_QUERY_ID_NV: GlEnum = GlEnum(0x8865);
/// 
/// * Group: [`QueryParameterName`]
pub const GL_CURRENT_QUERY: GlEnum = GlEnum(0x8865);
pub const GL_CURRENT_QUERY_ARB: GlEnum = GlEnum(0x8865);
pub const GL_CURRENT_QUERY_EXT: GlEnum = GlEnum(0x8865);
/// 
/// * Group: [`OcclusionQueryParameterNameNV`]
pub const GL_PIXEL_COUNT_NV: GlEnum = GlEnum(0x8866);
/// 
/// * Group: [`QueryObjectParameterName`]
pub const GL_QUERY_RESULT: GlEnum = GlEnum(0x8866);
pub const GL_QUERY_RESULT_ARB: GlEnum = GlEnum(0x8866);
pub const GL_QUERY_RESULT_EXT: GlEnum = GlEnum(0x8866);
/// 
/// * Group: [`OcclusionQueryParameterNameNV`]
pub const GL_PIXEL_COUNT_AVAILABLE_NV: GlEnum = GlEnum(0x8867);
/// 
/// * Group: [`QueryObjectParameterName`]
pub const GL_QUERY_RESULT_AVAILABLE: GlEnum = GlEnum(0x8867);
pub const GL_QUERY_RESULT_AVAILABLE_ARB: GlEnum = GlEnum(0x8867);
pub const GL_QUERY_RESULT_AVAILABLE_EXT: GlEnum = GlEnum(0x8867);
pub const GL_MAX_FRAGMENT_PROGRAM_LOCAL_PARAMETERS_NV: GlEnum = GlEnum(0x8868);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_ATTRIBS: GlEnum = GlEnum(0x8869);
pub const GL_MAX_VERTEX_ATTRIBS_ARB: GlEnum = GlEnum(0x8869);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GlEnum = GlEnum(0x886A);
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED_ARB: GlEnum = GlEnum(0x886A);
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GlEnum = GlEnum(0x886C);
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS_EXT: GlEnum = GlEnum(0x886C);
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS_OES: GlEnum = GlEnum(0x886C);
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GlEnum = GlEnum(0x886D);
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS_EXT: GlEnum = GlEnum(0x886D);
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS_OES: GlEnum = GlEnum(0x886D);
pub const GL_DEPTH_STENCIL_TO_RGBA_NV: GlEnum = GlEnum(0x886E);
pub const GL_DEPTH_STENCIL_TO_BGRA_NV: GlEnum = GlEnum(0x886F);
pub const GL_FRAGMENT_PROGRAM_NV: GlEnum = GlEnum(0x8870);
pub const GL_MAX_TEXTURE_COORDS: GlEnum = GlEnum(0x8871);
pub const GL_MAX_TEXTURE_COORDS_ARB: GlEnum = GlEnum(0x8871);
pub const GL_MAX_TEXTURE_COORDS_NV: GlEnum = GlEnum(0x8871);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_IMAGE_UNITS: GlEnum = GlEnum(0x8872);
pub const GL_MAX_TEXTURE_IMAGE_UNITS_ARB: GlEnum = GlEnum(0x8872);
pub const GL_MAX_TEXTURE_IMAGE_UNITS_NV: GlEnum = GlEnum(0x8872);
pub const GL_FRAGMENT_PROGRAM_BINDING_NV: GlEnum = GlEnum(0x8873);
pub const GL_PROGRAM_ERROR_STRING_ARB: GlEnum = GlEnum(0x8874);
pub const GL_PROGRAM_ERROR_STRING_NV: GlEnum = GlEnum(0x8874);
/// 
/// * Group: [`ProgramFormat`]
pub const GL_PROGRAM_FORMAT_ASCII_ARB: GlEnum = GlEnum(0x8875);
pub const GL_PROGRAM_FORMAT_ARB: GlEnum = GlEnum(0x8876);
/// 
/// * Group: [`PixelDataRangeTargetNV`]
pub const GL_WRITE_PIXEL_DATA_RANGE_NV: GlEnum = GlEnum(0x8878);
/// 
/// * Group: [`PixelDataRangeTargetNV`]
pub const GL_READ_PIXEL_DATA_RANGE_NV: GlEnum = GlEnum(0x8879);
pub const GL_WRITE_PIXEL_DATA_RANGE_LENGTH_NV: GlEnum = GlEnum(0x887A);
pub const GL_READ_PIXEL_DATA_RANGE_LENGTH_NV: GlEnum = GlEnum(0x887B);
pub const GL_WRITE_PIXEL_DATA_RANGE_POINTER_NV: GlEnum = GlEnum(0x887C);
pub const GL_READ_PIXEL_DATA_RANGE_POINTER_NV: GlEnum = GlEnum(0x887D);
pub const GL_GEOMETRY_SHADER_INVOCATIONS: GlEnum = GlEnum(0x887F);
pub const GL_GEOMETRY_SHADER_INVOCATIONS_EXT: GlEnum = GlEnum(0x887F);
pub const GL_GEOMETRY_SHADER_INVOCATIONS_OES: GlEnum = GlEnum(0x887F);
pub const GL_FLOAT_R_NV: GlEnum = GlEnum(0x8880);
pub const GL_FLOAT_RG_NV: GlEnum = GlEnum(0x8881);
pub const GL_FLOAT_RGB_NV: GlEnum = GlEnum(0x8882);
pub const GL_FLOAT_RGBA_NV: GlEnum = GlEnum(0x8883);
pub const GL_FLOAT_R16_NV: GlEnum = GlEnum(0x8884);
pub const GL_FLOAT_R32_NV: GlEnum = GlEnum(0x8885);
pub const GL_FLOAT_RG16_NV: GlEnum = GlEnum(0x8886);
pub const GL_FLOAT_RG32_NV: GlEnum = GlEnum(0x8887);
pub const GL_FLOAT_RGB16_NV: GlEnum = GlEnum(0x8888);
pub const GL_FLOAT_RGB32_NV: GlEnum = GlEnum(0x8889);
pub const GL_FLOAT_RGBA16_NV: GlEnum = GlEnum(0x888A);
pub const GL_FLOAT_RGBA32_NV: GlEnum = GlEnum(0x888B);
pub const GL_TEXTURE_FLOAT_COMPONENTS_NV: GlEnum = GlEnum(0x888C);
pub const GL_FLOAT_CLEAR_COLOR_VALUE_NV: GlEnum = GlEnum(0x888D);
pub const GL_FLOAT_RGBA_MODE_NV: GlEnum = GlEnum(0x888E);
pub const GL_TEXTURE_UNSIGNED_REMAP_MODE_NV: GlEnum = GlEnum(0x888F);
pub const GL_DEPTH_BOUNDS_TEST_EXT: GlEnum = GlEnum(0x8890);
pub const GL_DEPTH_BOUNDS_EXT: GlEnum = GlEnum(0x8891);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_ARRAY_BUFFER: GlEnum = GlEnum(0x8892);
pub const GL_ARRAY_BUFFER_ARB: GlEnum = GlEnum(0x8892);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_ELEMENT_ARRAY_BUFFER: GlEnum = GlEnum(0x8893);
pub const GL_ELEMENT_ARRAY_BUFFER_ARB: GlEnum = GlEnum(0x8893);
/// 
/// * Group: [`GetPName`]
pub const GL_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x8894);
pub const GL_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x8894);
/// 
/// * Group: [`GetPName`]
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x8895);
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x8895);
pub const GL_VERTEX_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x8896);
pub const GL_VERTEX_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x8896);
pub const GL_NORMAL_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x8897);
pub const GL_NORMAL_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x8897);
pub const GL_COLOR_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x8898);
pub const GL_COLOR_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x8898);
pub const GL_INDEX_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x8899);
pub const GL_INDEX_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x8899);
pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x889A);
pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x889A);
pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x889B);
pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x889B);
pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x889C);
pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x889C);
pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x889D);
pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x889D);
/// 
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING`]
pub const GL_FOG_COORD_ARRAY_BUFFER_BINDING: GlEnum = GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING;
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x889E);
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x889E);
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING_OES: GlEnum = GlEnum(0x889E);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GlEnum = GlEnum(0x889F);
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x889F);
pub const GL_PROGRAM_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x88A0);
pub const GL_MAX_PROGRAM_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x88A1);
pub const GL_PROGRAM_NATIVE_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x88A2);
pub const GL_MAX_PROGRAM_NATIVE_INSTRUCTIONS_ARB: GlEnum = GlEnum(0x88A3);
pub const GL_PROGRAM_TEMPORARIES_ARB: GlEnum = GlEnum(0x88A4);
pub const GL_MAX_PROGRAM_TEMPORARIES_ARB: GlEnum = GlEnum(0x88A5);
pub const GL_PROGRAM_NATIVE_TEMPORARIES_ARB: GlEnum = GlEnum(0x88A6);
pub const GL_MAX_PROGRAM_NATIVE_TEMPORARIES_ARB: GlEnum = GlEnum(0x88A7);
pub const GL_PROGRAM_PARAMETERS_ARB: GlEnum = GlEnum(0x88A8);
pub const GL_MAX_PROGRAM_PARAMETERS_ARB: GlEnum = GlEnum(0x88A9);
pub const GL_PROGRAM_NATIVE_PARAMETERS_ARB: GlEnum = GlEnum(0x88AA);
pub const GL_MAX_PROGRAM_NATIVE_PARAMETERS_ARB: GlEnum = GlEnum(0x88AB);
pub const GL_PROGRAM_ATTRIBS_ARB: GlEnum = GlEnum(0x88AC);
pub const GL_MAX_PROGRAM_ATTRIBS_ARB: GlEnum = GlEnum(0x88AD);
pub const GL_PROGRAM_NATIVE_ATTRIBS_ARB: GlEnum = GlEnum(0x88AE);
pub const GL_MAX_PROGRAM_NATIVE_ATTRIBS_ARB: GlEnum = GlEnum(0x88AF);
pub const GL_PROGRAM_ADDRESS_REGISTERS_ARB: GlEnum = GlEnum(0x88B0);
pub const GL_MAX_PROGRAM_ADDRESS_REGISTERS_ARB: GlEnum = GlEnum(0x88B1);
pub const GL_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: GlEnum = GlEnum(0x88B2);
pub const GL_MAX_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: GlEnum = GlEnum(0x88B3);
pub const GL_MAX_PROGRAM_LOCAL_PARAMETERS_ARB: GlEnum = GlEnum(0x88B4);
pub const GL_MAX_PROGRAM_ENV_PARAMETERS_ARB: GlEnum = GlEnum(0x88B5);
pub const GL_PROGRAM_UNDER_NATIVE_LIMITS_ARB: GlEnum = GlEnum(0x88B6);
pub const GL_TRANSPOSE_CURRENT_MATRIX_ARB: GlEnum = GlEnum(0x88B7);
/// 
/// * Group: [`BufferAccessARB`]
pub const GL_READ_ONLY: GlEnum = GlEnum(0x88B8);
pub const GL_READ_ONLY_ARB: GlEnum = GlEnum(0x88B8);
/// 
/// * Group: [`BufferAccessARB`]
pub const GL_WRITE_ONLY: GlEnum = GlEnum(0x88B9);
pub const GL_WRITE_ONLY_ARB: GlEnum = GlEnum(0x88B9);
pub const GL_WRITE_ONLY_OES: GlEnum = GlEnum(0x88B9);
/// 
/// * Group: [`BufferAccessARB`]
pub const GL_READ_WRITE: GlEnum = GlEnum(0x88BA);
pub const GL_READ_WRITE_ARB: GlEnum = GlEnum(0x88BA);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_ACCESS: GlEnum = GlEnum(0x88BB);
/// 
/// * Group: [`BufferPNameARB`]
pub const GL_BUFFER_ACCESS_ARB: GlEnum = GlEnum(0x88BB);
pub const GL_BUFFER_ACCESS_OES: GlEnum = GlEnum(0x88BB);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_MAPPED: GlEnum = GlEnum(0x88BC);
/// 
/// * Group: [`BufferPNameARB`]
pub const GL_BUFFER_MAPPED_ARB: GlEnum = GlEnum(0x88BC);
pub const GL_BUFFER_MAPPED_OES: GlEnum = GlEnum(0x88BC);
/// 
/// * Group: [`BufferPointerNameARB`]
pub const GL_BUFFER_MAP_POINTER: GlEnum = GlEnum(0x88BD);
/// 
/// * Group: [`BufferPointerNameARB`]
pub const GL_BUFFER_MAP_POINTER_ARB: GlEnum = GlEnum(0x88BD);
pub const GL_BUFFER_MAP_POINTER_OES: GlEnum = GlEnum(0x88BD);
pub const GL_WRITE_DISCARD_NV: GlEnum = GlEnum(0x88BE);
/// 
/// * Group: [`QueryTarget`]
pub const GL_TIME_ELAPSED: GlEnum = GlEnum(0x88BF);
pub const GL_TIME_ELAPSED_EXT: GlEnum = GlEnum(0x88BF);
pub const GL_MATRIX0_ARB: GlEnum = GlEnum(0x88C0);
pub const GL_MATRIX1_ARB: GlEnum = GlEnum(0x88C1);
pub const GL_MATRIX2_ARB: GlEnum = GlEnum(0x88C2);
pub const GL_MATRIX3_ARB: GlEnum = GlEnum(0x88C3);
pub const GL_MATRIX4_ARB: GlEnum = GlEnum(0x88C4);
pub const GL_MATRIX5_ARB: GlEnum = GlEnum(0x88C5);
pub const GL_MATRIX6_ARB: GlEnum = GlEnum(0x88C6);
pub const GL_MATRIX7_ARB: GlEnum = GlEnum(0x88C7);
pub const GL_MATRIX8_ARB: GlEnum = GlEnum(0x88C8);
pub const GL_MATRIX9_ARB: GlEnum = GlEnum(0x88C9);
pub const GL_MATRIX10_ARB: GlEnum = GlEnum(0x88CA);
pub const GL_MATRIX11_ARB: GlEnum = GlEnum(0x88CB);
pub const GL_MATRIX12_ARB: GlEnum = GlEnum(0x88CC);
pub const GL_MATRIX13_ARB: GlEnum = GlEnum(0x88CD);
pub const GL_MATRIX14_ARB: GlEnum = GlEnum(0x88CE);
pub const GL_MATRIX15_ARB: GlEnum = GlEnum(0x88CF);
pub const GL_MATRIX16_ARB: GlEnum = GlEnum(0x88D0);
pub const GL_MATRIX17_ARB: GlEnum = GlEnum(0x88D1);
pub const GL_MATRIX18_ARB: GlEnum = GlEnum(0x88D2);
pub const GL_MATRIX19_ARB: GlEnum = GlEnum(0x88D3);
pub const GL_MATRIX20_ARB: GlEnum = GlEnum(0x88D4);
pub const GL_MATRIX21_ARB: GlEnum = GlEnum(0x88D5);
pub const GL_MATRIX22_ARB: GlEnum = GlEnum(0x88D6);
pub const GL_MATRIX23_ARB: GlEnum = GlEnum(0x88D7);
pub const GL_MATRIX24_ARB: GlEnum = GlEnum(0x88D8);
pub const GL_MATRIX25_ARB: GlEnum = GlEnum(0x88D9);
pub const GL_MATRIX26_ARB: GlEnum = GlEnum(0x88DA);
pub const GL_MATRIX27_ARB: GlEnum = GlEnum(0x88DB);
pub const GL_MATRIX28_ARB: GlEnum = GlEnum(0x88DC);
pub const GL_MATRIX29_ARB: GlEnum = GlEnum(0x88DD);
pub const GL_MATRIX30_ARB: GlEnum = GlEnum(0x88DE);
pub const GL_MATRIX31_ARB: GlEnum = GlEnum(0x88DF);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STREAM_DRAW: GlEnum = GlEnum(0x88E0);
pub const GL_STREAM_DRAW_ARB: GlEnum = GlEnum(0x88E0);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STREAM_READ: GlEnum = GlEnum(0x88E1);
pub const GL_STREAM_READ_ARB: GlEnum = GlEnum(0x88E1);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STREAM_COPY: GlEnum = GlEnum(0x88E2);
pub const GL_STREAM_COPY_ARB: GlEnum = GlEnum(0x88E2);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STATIC_DRAW: GlEnum = GlEnum(0x88E4);
pub const GL_STATIC_DRAW_ARB: GlEnum = GlEnum(0x88E4);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STATIC_READ: GlEnum = GlEnum(0x88E5);
pub const GL_STATIC_READ_ARB: GlEnum = GlEnum(0x88E5);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STATIC_COPY: GlEnum = GlEnum(0x88E6);
pub const GL_STATIC_COPY_ARB: GlEnum = GlEnum(0x88E6);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_DYNAMIC_DRAW: GlEnum = GlEnum(0x88E8);
pub const GL_DYNAMIC_DRAW_ARB: GlEnum = GlEnum(0x88E8);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_DYNAMIC_READ: GlEnum = GlEnum(0x88E9);
pub const GL_DYNAMIC_READ_ARB: GlEnum = GlEnum(0x88E9);
/// 
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_DYNAMIC_COPY: GlEnum = GlEnum(0x88EA);
pub const GL_DYNAMIC_COPY_ARB: GlEnum = GlEnum(0x88EA);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_PIXEL_PACK_BUFFER: GlEnum = GlEnum(0x88EB);
pub const GL_PIXEL_PACK_BUFFER_ARB: GlEnum = GlEnum(0x88EB);
pub const GL_PIXEL_PACK_BUFFER_EXT: GlEnum = GlEnum(0x88EB);
pub const GL_PIXEL_PACK_BUFFER_NV: GlEnum = GlEnum(0x88EB);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_PIXEL_UNPACK_BUFFER: GlEnum = GlEnum(0x88EC);
pub const GL_PIXEL_UNPACK_BUFFER_ARB: GlEnum = GlEnum(0x88EC);
pub const GL_PIXEL_UNPACK_BUFFER_EXT: GlEnum = GlEnum(0x88EC);
pub const GL_PIXEL_UNPACK_BUFFER_NV: GlEnum = GlEnum(0x88EC);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_PACK_BUFFER_BINDING: GlEnum = GlEnum(0x88ED);
pub const GL_PIXEL_PACK_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x88ED);
pub const GL_PIXEL_PACK_BUFFER_BINDING_EXT: GlEnum = GlEnum(0x88ED);
pub const GL_PIXEL_PACK_BUFFER_BINDING_NV: GlEnum = GlEnum(0x88ED);
pub const GL_ETC1_SRGB8_NV: GlEnum = GlEnum(0x88EE);
/// 
/// * Group: [`GetPName`]
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GlEnum = GlEnum(0x88EF);
pub const GL_PIXEL_UNPACK_BUFFER_BINDING_ARB: GlEnum = GlEnum(0x88EF);
pub const GL_PIXEL_UNPACK_BUFFER_BINDING_EXT: GlEnum = GlEnum(0x88EF);
pub const GL_PIXEL_UNPACK_BUFFER_BINDING_NV: GlEnum = GlEnum(0x88EF);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH24_STENCIL8: GlEnum = GlEnum(0x88F0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH24_STENCIL8_EXT: GlEnum = GlEnum(0x88F0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH24_STENCIL8_OES: GlEnum = GlEnum(0x88F0);
pub const GL_TEXTURE_STENCIL_SIZE: GlEnum = GlEnum(0x88F1);
pub const GL_TEXTURE_STENCIL_SIZE_EXT: GlEnum = GlEnum(0x88F1);
pub const GL_STENCIL_TAG_BITS_EXT: GlEnum = GlEnum(0x88F2);
pub const GL_STENCIL_CLEAR_TAG_VALUE_EXT: GlEnum = GlEnum(0x88F3);
pub const GL_MAX_PROGRAM_EXEC_INSTRUCTIONS_NV: GlEnum = GlEnum(0x88F4);
pub const GL_MAX_PROGRAM_CALL_DEPTH_NV: GlEnum = GlEnum(0x88F5);
pub const GL_MAX_PROGRAM_IF_DEPTH_NV: GlEnum = GlEnum(0x88F6);
pub const GL_MAX_PROGRAM_LOOP_DEPTH_NV: GlEnum = GlEnum(0x88F7);
pub const GL_MAX_PROGRAM_LOOP_COUNT_NV: GlEnum = GlEnum(0x88F8);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_SRC1_COLOR: GlEnum = GlEnum(0x88F9);
pub const GL_SRC1_COLOR_EXT: GlEnum = GlEnum(0x88F9);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_SRC1_COLOR: GlEnum = GlEnum(0x88FA);
pub const GL_ONE_MINUS_SRC1_COLOR_EXT: GlEnum = GlEnum(0x88FA);
/// 
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_SRC1_ALPHA: GlEnum = GlEnum(0x88FB);
pub const GL_ONE_MINUS_SRC1_ALPHA_EXT: GlEnum = GlEnum(0x88FB);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: GlEnum = GlEnum(0x88FC);
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS_EXT: GlEnum = GlEnum(0x88FC);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GlEnum = GlEnum(0x88FD);
/// 
/// * Group: [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER_EXT: GlEnum = GlEnum(0x88FD);
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER_NV: GlEnum = GlEnum(0x88FD);
/// 
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GlEnum = GlEnum(0x88FE);
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: GlEnum = GlEnum(0x88FE);
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_ARB: GlEnum = GlEnum(0x88FE);
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_EXT: GlEnum = GlEnum(0x88FE);
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_NV: GlEnum = GlEnum(0x88FE);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GlEnum = GlEnum(0x88FF);
pub const GL_MAX_ARRAY_TEXTURE_LAYERS_EXT: GlEnum = GlEnum(0x88FF);
/// 
/// * Group: [`GetPName`]
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GlEnum = GlEnum(0x8904);
pub const GL_MIN_PROGRAM_TEXEL_OFFSET_EXT: GlEnum = GlEnum(0x8904);
pub const GL_MIN_PROGRAM_TEXEL_OFFSET_NV: GlEnum = GlEnum(0x8904);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GlEnum = GlEnum(0x8905);
pub const GL_MAX_PROGRAM_TEXEL_OFFSET_EXT: GlEnum = GlEnum(0x8905);
pub const GL_MAX_PROGRAM_TEXEL_OFFSET_NV: GlEnum = GlEnum(0x8905);
pub const GL_PROGRAM_ATTRIB_COMPONENTS_NV: GlEnum = GlEnum(0x8906);
pub const GL_PROGRAM_RESULT_COMPONENTS_NV: GlEnum = GlEnum(0x8907);
pub const GL_MAX_PROGRAM_ATTRIB_COMPONENTS_NV: GlEnum = GlEnum(0x8908);
pub const GL_MAX_PROGRAM_RESULT_COMPONENTS_NV: GlEnum = GlEnum(0x8909);
pub const GL_STENCIL_TEST_TWO_SIDE_EXT: GlEnum = GlEnum(0x8910);
pub const GL_ACTIVE_STENCIL_FACE_EXT: GlEnum = GlEnum(0x8911);
pub const GL_MIRROR_CLAMP_TO_BORDER_EXT: GlEnum = GlEnum(0x8912);
/// 
/// * Group: [`QueryTarget`]
pub const GL_SAMPLES_PASSED: GlEnum = GlEnum(0x8914);
pub const GL_SAMPLES_PASSED_ARB: GlEnum = GlEnum(0x8914);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_GEOMETRY_VERTICES_OUT: GlEnum = GlEnum(0x8916);
pub const GL_GEOMETRY_LINKED_VERTICES_OUT_EXT: GlEnum = GlEnum(0x8916);
pub const GL_GEOMETRY_LINKED_VERTICES_OUT_OES: GlEnum = GlEnum(0x8916);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_GEOMETRY_INPUT_TYPE: GlEnum = GlEnum(0x8917);
pub const GL_GEOMETRY_LINKED_INPUT_TYPE_EXT: GlEnum = GlEnum(0x8917);
pub const GL_GEOMETRY_LINKED_INPUT_TYPE_OES: GlEnum = GlEnum(0x8917);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_GEOMETRY_OUTPUT_TYPE: GlEnum = GlEnum(0x8918);
pub const GL_GEOMETRY_LINKED_OUTPUT_TYPE_EXT: GlEnum = GlEnum(0x8918);
pub const GL_GEOMETRY_LINKED_OUTPUT_TYPE_OES: GlEnum = GlEnum(0x8918);
/// 
/// * Group: [`GetPName`]
pub const GL_SAMPLER_BINDING: GlEnum = GlEnum(0x8919);
pub const GL_CLAMP_VERTEX_COLOR: GlEnum = GlEnum(0x891A);
/// 
/// * Group: [`ClampColorTargetARB`]
pub const GL_CLAMP_VERTEX_COLOR_ARB: GlEnum = GlEnum(0x891A);
pub const GL_CLAMP_FRAGMENT_COLOR: GlEnum = GlEnum(0x891B);
/// 
/// * Group: [`ClampColorTargetARB`]
pub const GL_CLAMP_FRAGMENT_COLOR_ARB: GlEnum = GlEnum(0x891B);
/// 
/// * Group: [`ClampColorTargetARB`]
pub const GL_CLAMP_READ_COLOR: GlEnum = GlEnum(0x891C);
/// 
/// * Group: [`ClampColorTargetARB`]
pub const GL_CLAMP_READ_COLOR_ARB: GlEnum = GlEnum(0x891C);
/// 
/// * Group: [`ClampColorModeARB`]
pub const GL_FIXED_ONLY: GlEnum = GlEnum(0x891D);
/// 
/// * Group: [`ClampColorModeARB`]
pub const GL_FIXED_ONLY_ARB: GlEnum = GlEnum(0x891D);
/// 
/// * Group: [`ProgramTarget`]
pub const GL_TESS_CONTROL_PROGRAM_NV: GlEnum = GlEnum(0x891E);
/// 
/// * Group: [`ProgramTarget`]
pub const GL_TESS_EVALUATION_PROGRAM_NV: GlEnum = GlEnum(0x891F);
pub const GL_FRAGMENT_SHADER_ATI: GlEnum = GlEnum(0x8920);
pub const GL_REG_0_ATI: GlEnum = GlEnum(0x8921);
pub const GL_REG_1_ATI: GlEnum = GlEnum(0x8922);
pub const GL_REG_2_ATI: GlEnum = GlEnum(0x8923);
pub const GL_REG_3_ATI: GlEnum = GlEnum(0x8924);
pub const GL_REG_4_ATI: GlEnum = GlEnum(0x8925);
pub const GL_REG_5_ATI: GlEnum = GlEnum(0x8926);
pub const GL_REG_6_ATI: GlEnum = GlEnum(0x8927);
pub const GL_REG_7_ATI: GlEnum = GlEnum(0x8928);
pub const GL_REG_8_ATI: GlEnum = GlEnum(0x8929);
pub const GL_REG_9_ATI: GlEnum = GlEnum(0x892A);
pub const GL_REG_10_ATI: GlEnum = GlEnum(0x892B);
pub const GL_REG_11_ATI: GlEnum = GlEnum(0x892C);
pub const GL_REG_12_ATI: GlEnum = GlEnum(0x892D);
pub const GL_REG_13_ATI: GlEnum = GlEnum(0x892E);
pub const GL_REG_14_ATI: GlEnum = GlEnum(0x892F);
pub const GL_REG_15_ATI: GlEnum = GlEnum(0x8930);
pub const GL_REG_16_ATI: GlEnum = GlEnum(0x8931);
pub const GL_REG_17_ATI: GlEnum = GlEnum(0x8932);
pub const GL_REG_18_ATI: GlEnum = GlEnum(0x8933);
pub const GL_REG_19_ATI: GlEnum = GlEnum(0x8934);
pub const GL_REG_20_ATI: GlEnum = GlEnum(0x8935);
pub const GL_REG_21_ATI: GlEnum = GlEnum(0x8936);
pub const GL_REG_22_ATI: GlEnum = GlEnum(0x8937);
pub const GL_REG_23_ATI: GlEnum = GlEnum(0x8938);
pub const GL_REG_24_ATI: GlEnum = GlEnum(0x8939);
pub const GL_REG_25_ATI: GlEnum = GlEnum(0x893A);
pub const GL_REG_26_ATI: GlEnum = GlEnum(0x893B);
pub const GL_REG_27_ATI: GlEnum = GlEnum(0x893C);
pub const GL_REG_28_ATI: GlEnum = GlEnum(0x893D);
pub const GL_REG_29_ATI: GlEnum = GlEnum(0x893E);
pub const GL_REG_30_ATI: GlEnum = GlEnum(0x893F);
pub const GL_REG_31_ATI: GlEnum = GlEnum(0x8940);
pub const GL_CON_0_ATI: GlEnum = GlEnum(0x8941);
pub const GL_CON_1_ATI: GlEnum = GlEnum(0x8942);
pub const GL_CON_2_ATI: GlEnum = GlEnum(0x8943);
pub const GL_CON_3_ATI: GlEnum = GlEnum(0x8944);
pub const GL_CON_4_ATI: GlEnum = GlEnum(0x8945);
pub const GL_CON_5_ATI: GlEnum = GlEnum(0x8946);
pub const GL_CON_6_ATI: GlEnum = GlEnum(0x8947);
pub const GL_CON_7_ATI: GlEnum = GlEnum(0x8948);
pub const GL_CON_8_ATI: GlEnum = GlEnum(0x8949);
pub const GL_CON_9_ATI: GlEnum = GlEnum(0x894A);
pub const GL_CON_10_ATI: GlEnum = GlEnum(0x894B);
pub const GL_CON_11_ATI: GlEnum = GlEnum(0x894C);
pub const GL_CON_12_ATI: GlEnum = GlEnum(0x894D);
pub const GL_CON_13_ATI: GlEnum = GlEnum(0x894E);
pub const GL_CON_14_ATI: GlEnum = GlEnum(0x894F);
pub const GL_CON_15_ATI: GlEnum = GlEnum(0x8950);
pub const GL_CON_16_ATI: GlEnum = GlEnum(0x8951);
pub const GL_CON_17_ATI: GlEnum = GlEnum(0x8952);
pub const GL_CON_18_ATI: GlEnum = GlEnum(0x8953);
pub const GL_CON_19_ATI: GlEnum = GlEnum(0x8954);
pub const GL_CON_20_ATI: GlEnum = GlEnum(0x8955);
pub const GL_CON_21_ATI: GlEnum = GlEnum(0x8956);
pub const GL_CON_22_ATI: GlEnum = GlEnum(0x8957);
pub const GL_CON_23_ATI: GlEnum = GlEnum(0x8958);
pub const GL_CON_24_ATI: GlEnum = GlEnum(0x8959);
pub const GL_CON_25_ATI: GlEnum = GlEnum(0x895A);
pub const GL_CON_26_ATI: GlEnum = GlEnum(0x895B);
pub const GL_CON_27_ATI: GlEnum = GlEnum(0x895C);
pub const GL_CON_28_ATI: GlEnum = GlEnum(0x895D);
pub const GL_CON_29_ATI: GlEnum = GlEnum(0x895E);
pub const GL_CON_30_ATI: GlEnum = GlEnum(0x895F);
pub const GL_CON_31_ATI: GlEnum = GlEnum(0x8960);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_MOV_ATI: GlEnum = GlEnum(0x8961);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_ADD_ATI: GlEnum = GlEnum(0x8963);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_MUL_ATI: GlEnum = GlEnum(0x8964);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_SUB_ATI: GlEnum = GlEnum(0x8965);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_DOT3_ATI: GlEnum = GlEnum(0x8966);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_DOT4_ATI: GlEnum = GlEnum(0x8967);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_MAD_ATI: GlEnum = GlEnum(0x8968);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_LERP_ATI: GlEnum = GlEnum(0x8969);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_CND_ATI: GlEnum = GlEnum(0x896A);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_CND0_ATI: GlEnum = GlEnum(0x896B);
/// 
/// * Group: [`FragmentOpATI`]
pub const GL_DOT2_ADD_ATI: GlEnum = GlEnum(0x896C);
pub const GL_SECONDARY_INTERPOLATOR_ATI: GlEnum = GlEnum(0x896D);
pub const GL_NUM_FRAGMENT_REGISTERS_ATI: GlEnum = GlEnum(0x896E);
pub const GL_NUM_FRAGMENT_CONSTANTS_ATI: GlEnum = GlEnum(0x896F);
pub const GL_NUM_PASSES_ATI: GlEnum = GlEnum(0x8970);
pub const GL_NUM_INSTRUCTIONS_PER_PASS_ATI: GlEnum = GlEnum(0x8971);
pub const GL_NUM_INSTRUCTIONS_TOTAL_ATI: GlEnum = GlEnum(0x8972);
pub const GL_NUM_INPUT_INTERPOLATOR_COMPONENTS_ATI: GlEnum = GlEnum(0x8973);
pub const GL_NUM_LOOPBACK_COMPONENTS_ATI: GlEnum = GlEnum(0x8974);
pub const GL_COLOR_ALPHA_PAIRING_ATI: GlEnum = GlEnum(0x8975);
/// 
/// * Group: [`SwizzleOpATI`]
pub const GL_SWIZZLE_STR_ATI: GlEnum = GlEnum(0x8976);
/// 
/// * Group: [`SwizzleOpATI`]
pub const GL_SWIZZLE_STQ_ATI: GlEnum = GlEnum(0x8977);
/// 
/// * Group: [`SwizzleOpATI`]
pub const GL_SWIZZLE_STR_DR_ATI: GlEnum = GlEnum(0x8978);
/// 
/// * Group: [`SwizzleOpATI`]
pub const GL_SWIZZLE_STQ_DQ_ATI: GlEnum = GlEnum(0x8979);
pub const GL_SWIZZLE_STRQ_ATI: GlEnum = GlEnum(0x897A);
pub const GL_SWIZZLE_STRQ_DQ_ATI: GlEnum = GlEnum(0x897B);
pub const GL_INTERLACE_OML: GlEnum = GlEnum(0x8980);
pub const GL_INTERLACE_READ_OML: GlEnum = GlEnum(0x8981);
pub const GL_FORMAT_SUBSAMPLE_24_24_OML: GlEnum = GlEnum(0x8982);
pub const GL_FORMAT_SUBSAMPLE_244_244_OML: GlEnum = GlEnum(0x8983);
/// 
/// * Group: [`PixelStoreParameter`]
pub const GL_PACK_RESAMPLE_OML: GlEnum = GlEnum(0x8984);
/// 
/// * Group: [`PixelStoreParameter`]
pub const GL_UNPACK_RESAMPLE_OML: GlEnum = GlEnum(0x8985);
pub const GL_RESAMPLE_REPLICATE_OML: GlEnum = GlEnum(0x8986);
pub const GL_RESAMPLE_ZERO_FILL_OML: GlEnum = GlEnum(0x8987);
pub const GL_RESAMPLE_AVERAGE_OML: GlEnum = GlEnum(0x8988);
pub const GL_RESAMPLE_DECIMATE_OML: GlEnum = GlEnum(0x8989);
pub const GL_POINT_SIZE_ARRAY_TYPE_OES: GlEnum = GlEnum(0x898A);
pub const GL_POINT_SIZE_ARRAY_STRIDE_OES: GlEnum = GlEnum(0x898B);
pub const GL_POINT_SIZE_ARRAY_POINTER_OES: GlEnum = GlEnum(0x898C);
pub const GL_MODELVIEW_MATRIX_FLOAT_AS_INT_BITS_OES: GlEnum = GlEnum(0x898D);
pub const GL_PROJECTION_MATRIX_FLOAT_AS_INT_BITS_OES: GlEnum = GlEnum(0x898E);
pub const GL_TEXTURE_MATRIX_FLOAT_AS_INT_BITS_OES: GlEnum = GlEnum(0x898F);
pub const GL_VERTEX_ATTRIB_MAP1_APPLE: GlEnum = GlEnum(0x8A00);
pub const GL_VERTEX_ATTRIB_MAP2_APPLE: GlEnum = GlEnum(0x8A01);
pub const GL_VERTEX_ATTRIB_MAP1_SIZE_APPLE: GlEnum = GlEnum(0x8A02);
pub const GL_VERTEX_ATTRIB_MAP1_COEFF_APPLE: GlEnum = GlEnum(0x8A03);
pub const GL_VERTEX_ATTRIB_MAP1_ORDER_APPLE: GlEnum = GlEnum(0x8A04);
pub const GL_VERTEX_ATTRIB_MAP1_DOMAIN_APPLE: GlEnum = GlEnum(0x8A05);
pub const GL_VERTEX_ATTRIB_MAP2_SIZE_APPLE: GlEnum = GlEnum(0x8A06);
pub const GL_VERTEX_ATTRIB_MAP2_COEFF_APPLE: GlEnum = GlEnum(0x8A07);
pub const GL_VERTEX_ATTRIB_MAP2_ORDER_APPLE: GlEnum = GlEnum(0x8A08);
pub const GL_VERTEX_ATTRIB_MAP2_DOMAIN_APPLE: GlEnum = GlEnum(0x8A09);
/// 
/// * Group: [`ObjectTypeAPPLE`]
pub const GL_DRAW_PIXELS_APPLE: GlEnum = GlEnum(0x8A0A);
/// 
/// * Group: [`ObjectTypeAPPLE`]
pub const GL_FENCE_APPLE: GlEnum = GlEnum(0x8A0B);
pub const GL_ELEMENT_ARRAY_APPLE: GlEnum = GlEnum(0x8A0C);
pub const GL_ELEMENT_ARRAY_TYPE_APPLE: GlEnum = GlEnum(0x8A0D);
pub const GL_ELEMENT_ARRAY_POINTER_APPLE: GlEnum = GlEnum(0x8A0E);
pub const GL_COLOR_FLOAT_APPLE: GlEnum = GlEnum(0x8A0F);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_UNIFORM_BUFFER: GlEnum = GlEnum(0x8A11);
pub const GL_BUFFER_SERIALIZED_MODIFY_APPLE: GlEnum = GlEnum(0x8A12);
pub const GL_BUFFER_FLUSHING_UNMAP_APPLE: GlEnum = GlEnum(0x8A13);
pub const GL_AUX_DEPTH_STENCIL_APPLE: GlEnum = GlEnum(0x8A14);
pub const GL_PACK_ROW_BYTES_APPLE: GlEnum = GlEnum(0x8A15);
pub const GL_UNPACK_ROW_BYTES_APPLE: GlEnum = GlEnum(0x8A16);
pub const GL_RELEASED_APPLE: GlEnum = GlEnum(0x8A19);
pub const GL_VOLATILE_APPLE: GlEnum = GlEnum(0x8A1A);
pub const GL_RETAINED_APPLE: GlEnum = GlEnum(0x8A1B);
pub const GL_UNDEFINED_APPLE: GlEnum = GlEnum(0x8A1C);
pub const GL_PURGEABLE_APPLE: GlEnum = GlEnum(0x8A1D);
pub const GL_RGB_422_APPLE: GlEnum = GlEnum(0x8A1F);
/// 
/// * Group: [`GetPName`]
pub const GL_UNIFORM_BUFFER_BINDING: GlEnum = GlEnum(0x8A28);
/// 
/// * Group: [`GetPName`]
pub const GL_UNIFORM_BUFFER_START: GlEnum = GlEnum(0x8A29);
/// 
/// * Group: [`GetPName`]
pub const GL_UNIFORM_BUFFER_SIZE: GlEnum = GlEnum(0x8A2A);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GlEnum = GlEnum(0x8A2B);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GlEnum = GlEnum(0x8A2C);
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS_EXT: GlEnum = GlEnum(0x8A2C);
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS_OES: GlEnum = GlEnum(0x8A2C);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GlEnum = GlEnum(0x8A2D);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GlEnum = GlEnum(0x8A2E);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GlEnum = GlEnum(0x8A2F);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_UNIFORM_BLOCK_SIZE: GlEnum = GlEnum(0x8A30);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8A31);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8A32);
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS_EXT: GlEnum = GlEnum(0x8A32);
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS_OES: GlEnum = GlEnum(0x8A32);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8A33);
/// 
/// * Group: [`GetPName`]
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GlEnum = GlEnum(0x8A34);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GlEnum = GlEnum(0x8A35);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_UNIFORM_BLOCKS: GlEnum = GlEnum(0x8A36);
/// 
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_TYPE: GlEnum = GlEnum(0x8A37);
/// 
/// * Group: [`SubroutineParameterName`], [`UniformPName`]
pub const GL_UNIFORM_SIZE: GlEnum = GlEnum(0x8A38);
/// 
/// * Group: [`SubroutineParameterName`], [`UniformPName`]
pub const GL_UNIFORM_NAME_LENGTH: GlEnum = GlEnum(0x8A39);
/// 
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_BLOCK_INDEX: GlEnum = GlEnum(0x8A3A);
/// 
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_OFFSET: GlEnum = GlEnum(0x8A3B);
/// 
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_ARRAY_STRIDE: GlEnum = GlEnum(0x8A3C);
/// 
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_MATRIX_STRIDE: GlEnum = GlEnum(0x8A3D);
/// 
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_IS_ROW_MAJOR: GlEnum = GlEnum(0x8A3E);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_BINDING: GlEnum = GlEnum(0x8A3F);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_DATA_SIZE: GlEnum = GlEnum(0x8A40);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GlEnum = GlEnum(0x8A41);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GlEnum = GlEnum(0x8A42);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GlEnum = GlEnum(0x8A43);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GlEnum = GlEnum(0x8A44);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GlEnum = GlEnum(0x8A45);
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GlEnum = GlEnum(0x8A46);
pub const GL_TEXTURE_SRGB_DECODE_EXT: GlEnum = GlEnum(0x8A48);
pub const GL_DECODE_EXT: GlEnum = GlEnum(0x8A49);
pub const GL_SKIP_DECODE_EXT: GlEnum = GlEnum(0x8A4A);
pub const GL_PROGRAM_PIPELINE_OBJECT_EXT: GlEnum = GlEnum(0x8A4F);
pub const GL_RGB_RAW_422_APPLE: GlEnum = GlEnum(0x8A51);
pub const GL_FRAGMENT_SHADER_DISCARDS_SAMPLES_EXT: GlEnum = GlEnum(0x8A52);
pub const GL_SYNC_OBJECT_APPLE: GlEnum = GlEnum(0x8A53);
pub const GL_COMPRESSED_SRGB_PVRTC_2BPPV1_EXT: GlEnum = GlEnum(0x8A54);
pub const GL_COMPRESSED_SRGB_PVRTC_4BPPV1_EXT: GlEnum = GlEnum(0x8A55);
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV1_EXT: GlEnum = GlEnum(0x8A56);
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV1_EXT: GlEnum = GlEnum(0x8A57);
/// 
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_FRAGMENT_SHADER: GlEnum = GlEnum(0x8B30);
/// 
/// * Group: [`ShaderType`]
pub const GL_FRAGMENT_SHADER_ARB: GlEnum = GlEnum(0x8B30);
/// 
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_VERTEX_SHADER: GlEnum = GlEnum(0x8B31);
/// 
/// * Group: [`ShaderType`]
pub const GL_VERTEX_SHADER_ARB: GlEnum = GlEnum(0x8B31);
pub const GL_PROGRAM_OBJECT_ARB: GlEnum = GlEnum(0x8B40);
pub const GL_PROGRAM_OBJECT_EXT: GlEnum = GlEnum(0x8B40);
pub const GL_SHADER_OBJECT_ARB: GlEnum = GlEnum(0x8B48);
pub const GL_SHADER_OBJECT_EXT: GlEnum = GlEnum(0x8B48);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8B49);
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS_ARB: GlEnum = GlEnum(0x8B49);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8B4A);
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS_ARB: GlEnum = GlEnum(0x8B4A);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VARYING_FLOATS: GlEnum = GlEnum(0x8B4B);
/// 
/// * Group: [`GetPName`]
/// * Alias Of: [`MAX_VARYING_FLOATS`]
pub const GL_MAX_VARYING_COMPONENTS: GlEnum = GL_MAX_VARYING_FLOATS;
pub const GL_MAX_VARYING_COMPONENTS_EXT: GlEnum = GlEnum(0x8B4B);
pub const GL_MAX_VARYING_FLOATS_ARB: GlEnum = GlEnum(0x8B4B);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GlEnum = GlEnum(0x8B4C);
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS_ARB: GlEnum = GlEnum(0x8B4C);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GlEnum = GlEnum(0x8B4D);
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS_ARB: GlEnum = GlEnum(0x8B4D);
pub const GL_OBJECT_TYPE_ARB: GlEnum = GlEnum(0x8B4E);
/// 
/// * Group: [`ShaderParameterName`]
pub const GL_SHADER_TYPE: GlEnum = GlEnum(0x8B4F);
pub const GL_OBJECT_SUBTYPE_ARB: GlEnum = GlEnum(0x8B4F);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_VEC2: GlEnum = GlEnum(0x8B50);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_VEC2_ARB: GlEnum = GlEnum(0x8B50);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_VEC3: GlEnum = GlEnum(0x8B51);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_VEC3_ARB: GlEnum = GlEnum(0x8B51);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_VEC4: GlEnum = GlEnum(0x8B52);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_VEC4_ARB: GlEnum = GlEnum(0x8B52);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_VEC2: GlEnum = GlEnum(0x8B53);
/// 
/// * Group: [`AttributeType`]
pub const GL_INT_VEC2_ARB: GlEnum = GlEnum(0x8B53);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_VEC3: GlEnum = GlEnum(0x8B54);
/// 
/// * Group: [`AttributeType`]
pub const GL_INT_VEC3_ARB: GlEnum = GlEnum(0x8B54);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_VEC4: GlEnum = GlEnum(0x8B55);
/// 
/// * Group: [`AttributeType`]
pub const GL_INT_VEC4_ARB: GlEnum = GlEnum(0x8B55);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_BOOL: GlEnum = GlEnum(0x8B56);
/// 
/// * Group: [`AttributeType`]
pub const GL_BOOL_ARB: GlEnum = GlEnum(0x8B56);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_BOOL_VEC2: GlEnum = GlEnum(0x8B57);
/// 
/// * Group: [`AttributeType`]
pub const GL_BOOL_VEC2_ARB: GlEnum = GlEnum(0x8B57);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_BOOL_VEC3: GlEnum = GlEnum(0x8B58);
/// 
/// * Group: [`AttributeType`]
pub const GL_BOOL_VEC3_ARB: GlEnum = GlEnum(0x8B58);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_BOOL_VEC4: GlEnum = GlEnum(0x8B59);
/// 
/// * Group: [`AttributeType`]
pub const GL_BOOL_VEC4_ARB: GlEnum = GlEnum(0x8B59);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT2: GlEnum = GlEnum(0x8B5A);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT2_ARB: GlEnum = GlEnum(0x8B5A);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT3: GlEnum = GlEnum(0x8B5B);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT3_ARB: GlEnum = GlEnum(0x8B5B);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT4: GlEnum = GlEnum(0x8B5C);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT4_ARB: GlEnum = GlEnum(0x8B5C);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_1D: GlEnum = GlEnum(0x8B5D);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_1D_ARB: GlEnum = GlEnum(0x8B5D);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D: GlEnum = GlEnum(0x8B5E);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_ARB: GlEnum = GlEnum(0x8B5E);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_3D: GlEnum = GlEnum(0x8B5F);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_3D_ARB: GlEnum = GlEnum(0x8B5F);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_3D_OES: GlEnum = GlEnum(0x8B5F);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_CUBE: GlEnum = GlEnum(0x8B60);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_CUBE_ARB: GlEnum = GlEnum(0x8B60);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_1D_SHADOW: GlEnum = GlEnum(0x8B61);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_1D_SHADOW_ARB: GlEnum = GlEnum(0x8B61);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_SHADOW: GlEnum = GlEnum(0x8B62);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_SHADOW_ARB: GlEnum = GlEnum(0x8B62);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_SHADOW_EXT: GlEnum = GlEnum(0x8B62);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_RECT: GlEnum = GlEnum(0x8B63);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_RECT_ARB: GlEnum = GlEnum(0x8B63);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_RECT_SHADOW: GlEnum = GlEnum(0x8B64);
/// 
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_RECT_SHADOW_ARB: GlEnum = GlEnum(0x8B64);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT2x3: GlEnum = GlEnum(0x8B65);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT2x3_NV: GlEnum = GlEnum(0x8B65);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT2x4: GlEnum = GlEnum(0x8B66);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT2x4_NV: GlEnum = GlEnum(0x8B66);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT3x2: GlEnum = GlEnum(0x8B67);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT3x2_NV: GlEnum = GlEnum(0x8B67);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT3x4: GlEnum = GlEnum(0x8B68);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT3x4_NV: GlEnum = GlEnum(0x8B68);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT4x2: GlEnum = GlEnum(0x8B69);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT4x2_NV: GlEnum = GlEnum(0x8B69);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT4x3: GlEnum = GlEnum(0x8B6A);
/// 
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT4x3_NV: GlEnum = GlEnum(0x8B6A);
/// 
/// * Group: [`ProgramPropertyARB`], [`ShaderParameterName`]
pub const GL_DELETE_STATUS: GlEnum = GlEnum(0x8B80);
pub const GL_OBJECT_DELETE_STATUS_ARB: GlEnum = GlEnum(0x8B80);
/// 
/// * Group: [`ShaderParameterName`]
pub const GL_COMPILE_STATUS: GlEnum = GlEnum(0x8B81);
pub const GL_OBJECT_COMPILE_STATUS_ARB: GlEnum = GlEnum(0x8B81);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_LINK_STATUS: GlEnum = GlEnum(0x8B82);
pub const GL_OBJECT_LINK_STATUS_ARB: GlEnum = GlEnum(0x8B82);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_VALIDATE_STATUS: GlEnum = GlEnum(0x8B83);
pub const GL_OBJECT_VALIDATE_STATUS_ARB: GlEnum = GlEnum(0x8B83);
/// 
/// * Group: [`ProgramPropertyARB`], [`ShaderParameterName`],
///   [`PipelineParameterName`]
pub const GL_INFO_LOG_LENGTH: GlEnum = GlEnum(0x8B84);
pub const GL_OBJECT_INFO_LOG_LENGTH_ARB: GlEnum = GlEnum(0x8B84);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_ATTACHED_SHADERS: GlEnum = GlEnum(0x8B85);
pub const GL_OBJECT_ATTACHED_OBJECTS_ARB: GlEnum = GlEnum(0x8B85);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_UNIFORMS: GlEnum = GlEnum(0x8B86);
pub const GL_OBJECT_ACTIVE_UNIFORMS_ARB: GlEnum = GlEnum(0x8B86);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GlEnum = GlEnum(0x8B87);
pub const GL_OBJECT_ACTIVE_UNIFORM_MAX_LENGTH_ARB: GlEnum = GlEnum(0x8B87);
/// 
/// * Group: [`ShaderParameterName`]
pub const GL_SHADER_SOURCE_LENGTH: GlEnum = GlEnum(0x8B88);
pub const GL_OBJECT_SHADER_SOURCE_LENGTH_ARB: GlEnum = GlEnum(0x8B88);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_ATTRIBUTES: GlEnum = GlEnum(0x8B89);
pub const GL_OBJECT_ACTIVE_ATTRIBUTES_ARB: GlEnum = GlEnum(0x8B89);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GlEnum = GlEnum(0x8B8A);
pub const GL_OBJECT_ACTIVE_ATTRIBUTE_MAX_LENGTH_ARB: GlEnum = GlEnum(0x8B8A);
/// 
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GlEnum = GlEnum(0x8B8B);
/// 
/// * Group: [`HintTarget`]
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_ARB: GlEnum = GlEnum(0x8B8B);
/// 
/// * Group: [`HintTarget`]
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES: GlEnum = GlEnum(0x8B8B);
/// 
/// * Group: [`StringName`]
pub const GL_SHADING_LANGUAGE_VERSION: GlEnum = GlEnum(0x8B8C);
pub const GL_SHADING_LANGUAGE_VERSION_ARB: GlEnum = GlEnum(0x8B8C);
/// 
/// * Group: [`GetPName`]
pub const GL_CURRENT_PROGRAM: GlEnum = GlEnum(0x8B8D);
/// For the OpenGL version of `EXT_separate_shader_objects`.
#[doc(alias = "GL_ACTIVE_PROGRAM_EXT")]
pub const GL_ACTIVE_PROGRAM_EXT_GL: GlEnum = GlEnum(0x8259);
pub const GL_PALETTE4_RGB8_OES: GlEnum = GlEnum(0x8B90);
pub const GL_PALETTE4_RGBA8_OES: GlEnum = GlEnum(0x8B91);
pub const GL_PALETTE4_R5_G6_B5_OES: GlEnum = GlEnum(0x8B92);
pub const GL_PALETTE4_RGBA4_OES: GlEnum = GlEnum(0x8B93);
pub const GL_PALETTE4_RGB5_A1_OES: GlEnum = GlEnum(0x8B94);
pub const GL_PALETTE8_RGB8_OES: GlEnum = GlEnum(0x8B95);
pub const GL_PALETTE8_RGBA8_OES: GlEnum = GlEnum(0x8B96);
pub const GL_PALETTE8_R5_G6_B5_OES: GlEnum = GlEnum(0x8B97);
pub const GL_PALETTE8_RGBA4_OES: GlEnum = GlEnum(0x8B98);
pub const GL_PALETTE8_RGB5_A1_OES: GlEnum = GlEnum(0x8B99);
/// 
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GlEnum = GlEnum(0x8B9A);
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE_OES: GlEnum = GlEnum(0x8B9A);
/// 
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GlEnum = GlEnum(0x8B9B);
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT_OES: GlEnum = GlEnum(0x8B9B);
pub const GL_POINT_SIZE_ARRAY_OES: GlEnum = GlEnum(0x8B9C);
pub const GL_TEXTURE_CROP_RECT_OES: GlEnum = GlEnum(0x8B9D);
pub const GL_MATRIX_INDEX_ARRAY_BUFFER_BINDING_OES: GlEnum = GlEnum(0x8B9E);
pub const GL_POINT_SIZE_ARRAY_BUFFER_BINDING_OES: GlEnum = GlEnum(0x8B9F);
pub const GL_FRAGMENT_PROGRAM_POSITION_MESA: GlEnum = GlEnum(0x8BB0);
pub const GL_FRAGMENT_PROGRAM_CALLBACK_MESA: GlEnum = GlEnum(0x8BB1);
pub const GL_FRAGMENT_PROGRAM_CALLBACK_FUNC_MESA: GlEnum = GlEnum(0x8BB2);
pub const GL_FRAGMENT_PROGRAM_CALLBACK_DATA_MESA: GlEnum = GlEnum(0x8BB3);
pub const GL_VERTEX_PROGRAM_POSITION_MESA: GlEnum = GlEnum(0x8BB4);
pub const GL_VERTEX_PROGRAM_CALLBACK_MESA: GlEnum = GlEnum(0x8BB5);
pub const GL_VERTEX_PROGRAM_CALLBACK_FUNC_MESA: GlEnum = GlEnum(0x8BB6);
pub const GL_VERTEX_PROGRAM_CALLBACK_DATA_MESA: GlEnum = GlEnum(0x8BB7);
pub const GL_TILE_RASTER_ORDER_FIXED_MESA: GlEnum = GlEnum(0x8BB8);
pub const GL_TILE_RASTER_ORDER_INCREASING_X_MESA: GlEnum = GlEnum(0x8BB9);
pub const GL_TILE_RASTER_ORDER_INCREASING_Y_MESA: GlEnum = GlEnum(0x8BBA);
pub const GL_FRAMEBUFFER_FLIP_Y_MESA: GlEnum = GlEnum(0x8BBB);
pub const GL_FRAMEBUFFER_FLIP_X_MESA: GlEnum = GlEnum(0x8BBC);
pub const GL_FRAMEBUFFER_SWAP_XY_MESA: GlEnum = GlEnum(0x8BBD);
pub const GL_COUNTER_TYPE_AMD: GlEnum = GlEnum(0x8BC0);
pub const GL_COUNTER_RANGE_AMD: GlEnum = GlEnum(0x8BC1);
pub const GL_UNSIGNED_INT64_AMD: GlEnum = GlEnum(0x8BC2);
pub const GL_PERCENTAGE_AMD: GlEnum = GlEnum(0x8BC3);
pub const GL_PERFMON_RESULT_AVAILABLE_AMD: GlEnum = GlEnum(0x8BC4);
pub const GL_PERFMON_RESULT_SIZE_AMD: GlEnum = GlEnum(0x8BC5);
pub const GL_PERFMON_RESULT_AMD: GlEnum = GlEnum(0x8BC6);
pub const GL_TEXTURE_WIDTH_QCOM: GlEnum = GlEnum(0x8BD2);
pub const GL_TEXTURE_HEIGHT_QCOM: GlEnum = GlEnum(0x8BD3);
pub const GL_TEXTURE_DEPTH_QCOM: GlEnum = GlEnum(0x8BD4);
pub const GL_TEXTURE_INTERNAL_FORMAT_QCOM: GlEnum = GlEnum(0x8BD5);
pub const GL_TEXTURE_FORMAT_QCOM: GlEnum = GlEnum(0x8BD6);
pub const GL_TEXTURE_TYPE_QCOM: GlEnum = GlEnum(0x8BD7);
pub const GL_TEXTURE_IMAGE_VALID_QCOM: GlEnum = GlEnum(0x8BD8);
pub const GL_TEXTURE_NUM_LEVELS_QCOM: GlEnum = GlEnum(0x8BD9);
pub const GL_TEXTURE_TARGET_QCOM: GlEnum = GlEnum(0x8BDA);
pub const GL_TEXTURE_OBJECT_VALID_QCOM: GlEnum = GlEnum(0x8BDB);
pub const GL_STATE_RESTORE: GlEnum = GlEnum(0x8BDC);
pub const GL_SAMPLER_EXTERNAL_2D_Y2Y_EXT: GlEnum = GlEnum(0x8BE7);
pub const GL_TEXTURE_PROTECTED_EXT: GlEnum = GlEnum(0x8BFA);
pub const GL_TEXTURE_FOVEATED_FEATURE_BITS_QCOM: GlEnum = GlEnum(0x8BFB);
pub const GL_TEXTURE_FOVEATED_MIN_PIXEL_DENSITY_QCOM: GlEnum = GlEnum(0x8BFC);
pub const GL_TEXTURE_FOVEATED_FEATURE_QUERY_QCOM: GlEnum = GlEnum(0x8BFD);
pub const GL_TEXTURE_FOVEATED_NUM_FOCAL_POINTS_QUERY_QCOM: GlEnum = GlEnum(0x8BFE);
pub const GL_FRAMEBUFFER_INCOMPLETE_FOVEATION_QCOM: GlEnum = GlEnum(0x8BFF);
pub const GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG: GlEnum = GlEnum(0x8C00);
pub const GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG: GlEnum = GlEnum(0x8C01);
pub const GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: GlEnum = GlEnum(0x8C02);
pub const GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: GlEnum = GlEnum(0x8C03);
pub const GL_MODULATE_COLOR_IMG: GlEnum = GlEnum(0x8C04);
pub const GL_RECIP_ADD_SIGNED_ALPHA_IMG: GlEnum = GlEnum(0x8C05);
pub const GL_TEXTURE_ALPHA_MODULATE_IMG: GlEnum = GlEnum(0x8C06);
pub const GL_FACTOR_ALPHA_MODULATE_IMG: GlEnum = GlEnum(0x8C07);
pub const GL_FRAGMENT_ALPHA_MODULATE_IMG: GlEnum = GlEnum(0x8C08);
pub const GL_ADD_BLEND_IMG: GlEnum = GlEnum(0x8C09);
/// 
/// * Group: [`ShaderBinaryFormat`]
pub const GL_SGX_BINARY_IMG: GlEnum = GlEnum(0x8C0A);
pub const GL_TEXTURE_RED_TYPE: GlEnum = GlEnum(0x8C10);
pub const GL_TEXTURE_RED_TYPE_ARB: GlEnum = GlEnum(0x8C10);
pub const GL_TEXTURE_GREEN_TYPE: GlEnum = GlEnum(0x8C11);
pub const GL_TEXTURE_GREEN_TYPE_ARB: GlEnum = GlEnum(0x8C11);
pub const GL_TEXTURE_BLUE_TYPE: GlEnum = GlEnum(0x8C12);
pub const GL_TEXTURE_BLUE_TYPE_ARB: GlEnum = GlEnum(0x8C12);
pub const GL_TEXTURE_ALPHA_TYPE: GlEnum = GlEnum(0x8C13);
pub const GL_TEXTURE_ALPHA_TYPE_ARB: GlEnum = GlEnum(0x8C13);
pub const GL_TEXTURE_LUMINANCE_TYPE: GlEnum = GlEnum(0x8C14);
pub const GL_TEXTURE_LUMINANCE_TYPE_ARB: GlEnum = GlEnum(0x8C14);
pub const GL_TEXTURE_INTENSITY_TYPE: GlEnum = GlEnum(0x8C15);
pub const GL_TEXTURE_INTENSITY_TYPE_ARB: GlEnum = GlEnum(0x8C15);
pub const GL_TEXTURE_DEPTH_TYPE: GlEnum = GlEnum(0x8C16);
pub const GL_TEXTURE_DEPTH_TYPE_ARB: GlEnum = GlEnum(0x8C16);
pub const GL_UNSIGNED_NORMALIZED: GlEnum = GlEnum(0x8C17);
pub const GL_UNSIGNED_NORMALIZED_ARB: GlEnum = GlEnum(0x8C17);
pub const GL_UNSIGNED_NORMALIZED_EXT: GlEnum = GlEnum(0x8C17);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_1D_ARRAY: GlEnum = GlEnum(0x8C18);
pub const GL_TEXTURE_1D_ARRAY_EXT: GlEnum = GlEnum(0x8C18);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_1D_ARRAY: GlEnum = GlEnum(0x8C19);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_1D_ARRAY_EXT: GlEnum = GlEnum(0x8C19);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_2D_ARRAY: GlEnum = GlEnum(0x8C1A);
pub const GL_TEXTURE_2D_ARRAY_EXT: GlEnum = GlEnum(0x8C1A);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_ARRAY: GlEnum = GlEnum(0x8C1B);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_ARRAY_EXT: GlEnum = GlEnum(0x8C1B);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_1D_ARRAY: GlEnum = GlEnum(0x8C1C);
pub const GL_TEXTURE_BINDING_1D_ARRAY_EXT: GlEnum = GlEnum(0x8C1C);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_2D_ARRAY: GlEnum = GlEnum(0x8C1D);
pub const GL_TEXTURE_BINDING_2D_ARRAY_EXT: GlEnum = GlEnum(0x8C1D);
/// 
/// * Group: [`ProgramTarget`]
pub const GL_GEOMETRY_PROGRAM_NV: GlEnum = GlEnum(0x8C26);
pub const GL_MAX_PROGRAM_OUTPUT_VERTICES_NV: GlEnum = GlEnum(0x8C27);
pub const GL_MAX_PROGRAM_TOTAL_OUTPUT_COMPONENTS_NV: GlEnum = GlEnum(0x8C28);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GlEnum = GlEnum(0x8C29);
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_ARB: GlEnum = GlEnum(0x8C29);
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_EXT: GlEnum = GlEnum(0x8C29);
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_OES: GlEnum = GlEnum(0x8C29);
/// 
/// * Group: [`TextureTarget`], [`CopyBufferSubDataTarget`],
///   [`BufferTargetARB`], [`BufferStorageTarget`]
pub const GL_TEXTURE_BUFFER: GlEnum = GlEnum(0x8C2A);
pub const GL_TEXTURE_BUFFER_ARB: GlEnum = GlEnum(0x8C2A);
pub const GL_TEXTURE_BUFFER_EXT: GlEnum = GlEnum(0x8C2A);
pub const GL_TEXTURE_BUFFER_OES: GlEnum = GlEnum(0x8C2A);
/// Equivalent to GL_TEXTURE_BUFFER_ARB query, but named more consistently
pub const GL_TEXTURE_BUFFER_BINDING: GlEnum = GlEnum(0x8C2A);
pub const GL_TEXTURE_BUFFER_BINDING_EXT: GlEnum = GlEnum(0x8C2A);
pub const GL_TEXTURE_BUFFER_BINDING_OES: GlEnum = GlEnum(0x8C2A);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_BUFFER_SIZE: GlEnum = GlEnum(0x8C2B);
pub const GL_MAX_TEXTURE_BUFFER_SIZE_ARB: GlEnum = GlEnum(0x8C2B);
pub const GL_MAX_TEXTURE_BUFFER_SIZE_EXT: GlEnum = GlEnum(0x8C2B);
pub const GL_MAX_TEXTURE_BUFFER_SIZE_OES: GlEnum = GlEnum(0x8C2B);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_BUFFER: GlEnum = GlEnum(0x8C2C);
pub const GL_TEXTURE_BINDING_BUFFER_ARB: GlEnum = GlEnum(0x8C2C);
pub const GL_TEXTURE_BINDING_BUFFER_EXT: GlEnum = GlEnum(0x8C2C);
pub const GL_TEXTURE_BINDING_BUFFER_OES: GlEnum = GlEnum(0x8C2C);
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GlEnum = GlEnum(0x8C2D);
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING_ARB: GlEnum = GlEnum(0x8C2D);
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING_EXT: GlEnum = GlEnum(0x8C2D);
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING_OES: GlEnum = GlEnum(0x8C2D);
pub const GL_TEXTURE_BUFFER_FORMAT_ARB: GlEnum = GlEnum(0x8C2E);
pub const GL_TEXTURE_BUFFER_FORMAT_EXT: GlEnum = GlEnum(0x8C2E);
/// 
/// * Group: [`QueryTarget`]
pub const GL_ANY_SAMPLES_PASSED: GlEnum = GlEnum(0x8C2F);
pub const GL_ANY_SAMPLES_PASSED_EXT: GlEnum = GlEnum(0x8C2F);
/// 
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_SHADING: GlEnum = GlEnum(0x8C36);
pub const GL_SAMPLE_SHADING_ARB: GlEnum = GlEnum(0x8C36);
pub const GL_SAMPLE_SHADING_OES: GlEnum = GlEnum(0x8C36);
pub const GL_MIN_SAMPLE_SHADING_VALUE: GlEnum = GlEnum(0x8C37);
pub const GL_MIN_SAMPLE_SHADING_VALUE_ARB: GlEnum = GlEnum(0x8C37);
pub const GL_MIN_SAMPLE_SHADING_VALUE_OES: GlEnum = GlEnum(0x8C37);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R11F_G11F_B10F: GlEnum = GlEnum(0x8C3A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R11F_G11F_B10F_APPLE: GlEnum = GlEnum(0x8C3A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R11F_G11F_B10F_EXT: GlEnum = GlEnum(0x8C3A);
/// 
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GlEnum = GlEnum(0x8C3B);
pub const GL_UNSIGNED_INT_10F_11F_11F_REV_APPLE: GlEnum = GlEnum(0x8C3B);
pub const GL_UNSIGNED_INT_10F_11F_11F_REV_EXT: GlEnum = GlEnum(0x8C3B);
pub const GL_RGBA_SIGNED_COMPONENTS_EXT: GlEnum = GlEnum(0x8C3C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB9_E5: GlEnum = GlEnum(0x8C3D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB9_E5_APPLE: GlEnum = GlEnum(0x8C3D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB9_E5_EXT: GlEnum = GlEnum(0x8C3D);
pub const GL_UNSIGNED_INT_5_9_9_9_REV: GlEnum = GlEnum(0x8C3E);
pub const GL_UNSIGNED_INT_5_9_9_9_REV_APPLE: GlEnum = GlEnum(0x8C3E);
pub const GL_UNSIGNED_INT_5_9_9_9_REV_EXT: GlEnum = GlEnum(0x8C3E);
pub const GL_TEXTURE_SHARED_SIZE: GlEnum = GlEnum(0x8C3F);
pub const GL_TEXTURE_SHARED_SIZE_EXT: GlEnum = GlEnum(0x8C3F);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB: GlEnum = GlEnum(0x8C40);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB_EXT: GlEnum = GlEnum(0x8C40);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB8: GlEnum = GlEnum(0x8C41);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB8_EXT: GlEnum = GlEnum(0x8C41);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB8_NV: GlEnum = GlEnum(0x8C41);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB_ALPHA: GlEnum = GlEnum(0x8C42);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB_ALPHA_EXT: GlEnum = GlEnum(0x8C42);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB8_ALPHA8: GlEnum = GlEnum(0x8C43);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRGB8_ALPHA8_EXT: GlEnum = GlEnum(0x8C43);
pub const GL_SLUMINANCE_ALPHA: GlEnum = GlEnum(0x8C44);
pub const GL_SLUMINANCE_ALPHA_EXT: GlEnum = GlEnum(0x8C44);
pub const GL_SLUMINANCE_ALPHA_NV: GlEnum = GlEnum(0x8C44);
pub const GL_SLUMINANCE8_ALPHA8: GlEnum = GlEnum(0x8C45);
pub const GL_SLUMINANCE8_ALPHA8_EXT: GlEnum = GlEnum(0x8C45);
pub const GL_SLUMINANCE8_ALPHA8_NV: GlEnum = GlEnum(0x8C45);
pub const GL_SLUMINANCE: GlEnum = GlEnum(0x8C46);
pub const GL_SLUMINANCE_EXT: GlEnum = GlEnum(0x8C46);
pub const GL_SLUMINANCE_NV: GlEnum = GlEnum(0x8C46);
pub const GL_SLUMINANCE8: GlEnum = GlEnum(0x8C47);
pub const GL_SLUMINANCE8_EXT: GlEnum = GlEnum(0x8C47);
pub const GL_SLUMINANCE8_NV: GlEnum = GlEnum(0x8C47);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB: GlEnum = GlEnum(0x8C48);
pub const GL_COMPRESSED_SRGB_EXT: GlEnum = GlEnum(0x8C48);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA: GlEnum = GlEnum(0x8C49);
pub const GL_COMPRESSED_SRGB_ALPHA_EXT: GlEnum = GlEnum(0x8C49);
pub const GL_COMPRESSED_SLUMINANCE: GlEnum = GlEnum(0x8C4A);
pub const GL_COMPRESSED_SLUMINANCE_EXT: GlEnum = GlEnum(0x8C4A);
pub const GL_COMPRESSED_SLUMINANCE_ALPHA: GlEnum = GlEnum(0x8C4B);
pub const GL_COMPRESSED_SLUMINANCE_ALPHA_EXT: GlEnum = GlEnum(0x8C4B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: GlEnum = GlEnum(0x8C4C);
pub const GL_COMPRESSED_SRGB_S3TC_DXT1_NV: GlEnum = GlEnum(0x8C4C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: GlEnum = GlEnum(0x8C4D);
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_NV: GlEnum = GlEnum(0x8C4D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: GlEnum = GlEnum(0x8C4E);
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_NV: GlEnum = GlEnum(0x8C4E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: GlEnum = GlEnum(0x8C4F);
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_NV: GlEnum = GlEnum(0x8C4F);
pub const GL_COMPRESSED_LUMINANCE_LATC1_EXT: GlEnum = GlEnum(0x8C70);
pub const GL_COMPRESSED_SIGNED_LUMINANCE_LATC1_EXT: GlEnum = GlEnum(0x8C71);
pub const GL_COMPRESSED_LUMINANCE_ALPHA_LATC2_EXT: GlEnum = GlEnum(0x8C72);
pub const GL_COMPRESSED_SIGNED_LUMINANCE_ALPHA_LATC2_EXT: GlEnum = GlEnum(0x8C73);
pub const GL_TESS_CONTROL_PROGRAM_PARAMETER_BUFFER_NV: GlEnum = GlEnum(0x8C74);
pub const GL_TESS_EVALUATION_PROGRAM_PARAMETER_BUFFER_NV: GlEnum = GlEnum(0x8C75);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GlEnum = GlEnum(0x8C76);
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH_EXT: GlEnum = GlEnum(0x8C76);
pub const GL_BACK_PRIMARY_COLOR_NV: GlEnum = GlEnum(0x8C77);
pub const GL_BACK_SECONDARY_COLOR_NV: GlEnum = GlEnum(0x8C78);
pub const GL_TEXTURE_COORD_NV: GlEnum = GlEnum(0x8C79);
pub const GL_CLIP_DISTANCE_NV: GlEnum = GlEnum(0x8C7A);
pub const GL_VERTEX_ID_NV: GlEnum = GlEnum(0x8C7B);
pub const GL_PRIMITIVE_ID_NV: GlEnum = GlEnum(0x8C7C);
pub const GL_GENERIC_ATTRIB_NV: GlEnum = GlEnum(0x8C7D);
pub const GL_TRANSFORM_FEEDBACK_ATTRIBS_NV: GlEnum = GlEnum(0x8C7E);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GlEnum = GlEnum(0x8C7F);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE_EXT: GlEnum = GlEnum(0x8C7F);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE_NV: GlEnum = GlEnum(0x8C7F);
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GlEnum = GlEnum(0x8C80);
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS_EXT: GlEnum = GlEnum(0x8C80);
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS_NV: GlEnum = GlEnum(0x8C80);
pub const GL_ACTIVE_VARYINGS_NV: GlEnum = GlEnum(0x8C81);
pub const GL_ACTIVE_VARYING_MAX_LENGTH_NV: GlEnum = GlEnum(0x8C82);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GlEnum = GlEnum(0x8C83);
pub const GL_TRANSFORM_FEEDBACK_VARYINGS_EXT: GlEnum = GlEnum(0x8C83);
pub const GL_TRANSFORM_FEEDBACK_VARYINGS_NV: GlEnum = GlEnum(0x8C83);
/// 
/// * Group: [`TransformFeedbackPName`], [`GetPName`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GlEnum = GlEnum(0x8C84);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START_EXT: GlEnum = GlEnum(0x8C84);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START_NV: GlEnum = GlEnum(0x8C84);
/// 
/// * Group: [`TransformFeedbackPName`], [`GetPName`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GlEnum = GlEnum(0x8C85);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE_EXT: GlEnum = GlEnum(0x8C85);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE_NV: GlEnum = GlEnum(0x8C85);
pub const GL_TRANSFORM_FEEDBACK_RECORD_NV: GlEnum = GlEnum(0x8C86);
/// 
/// * Group: [`QueryTarget`]
pub const GL_PRIMITIVES_GENERATED: GlEnum = GlEnum(0x8C87);
pub const GL_PRIMITIVES_GENERATED_EXT: GlEnum = GlEnum(0x8C87);
pub const GL_PRIMITIVES_GENERATED_NV: GlEnum = GlEnum(0x8C87);
pub const GL_PRIMITIVES_GENERATED_OES: GlEnum = GlEnum(0x8C87);
/// 
/// * Group: [`QueryTarget`]
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GlEnum = GlEnum(0x8C88);
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN_EXT: GlEnum = GlEnum(0x8C88);
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN_NV: GlEnum = GlEnum(0x8C88);
/// 
/// * Group: [`EnableCap`]
pub const GL_RASTERIZER_DISCARD: GlEnum = GlEnum(0x8C89);
pub const GL_RASTERIZER_DISCARD_EXT: GlEnum = GlEnum(0x8C89);
pub const GL_RASTERIZER_DISCARD_NV: GlEnum = GlEnum(0x8C89);
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GlEnum = GlEnum(0x8C8A);
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS_EXT: GlEnum = GlEnum(0x8C8A);
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS_NV: GlEnum = GlEnum(0x8C8A);
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GlEnum = GlEnum(0x8C8B);
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS_EXT: GlEnum = GlEnum(0x8C8B);
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS_NV: GlEnum = GlEnum(0x8C8B);
/// 
/// * Group: [`TransformFeedbackBufferMode`]
pub const GL_INTERLEAVED_ATTRIBS: GlEnum = GlEnum(0x8C8C);
pub const GL_INTERLEAVED_ATTRIBS_EXT: GlEnum = GlEnum(0x8C8C);
pub const GL_INTERLEAVED_ATTRIBS_NV: GlEnum = GlEnum(0x8C8C);
/// 
/// * Group: [`TransformFeedbackBufferMode`]
pub const GL_SEPARATE_ATTRIBS: GlEnum = GlEnum(0x8C8D);
pub const GL_SEPARATE_ATTRIBS_EXT: GlEnum = GlEnum(0x8C8D);
pub const GL_SEPARATE_ATTRIBS_NV: GlEnum = GlEnum(0x8C8D);
/// 
/// * Group: [`ProgramInterface`], [`BufferTargetARB`], [`BufferStorageTarget`],
///   [`CopyBufferSubDataTarget`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER: GlEnum = GlEnum(0x8C8E);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_EXT: GlEnum = GlEnum(0x8C8E);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_NV: GlEnum = GlEnum(0x8C8E);
/// 
/// * Group: [`TransformFeedbackPName`], [`GetPName`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GlEnum = GlEnum(0x8C8F);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING_EXT: GlEnum = GlEnum(0x8C8F);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING_NV: GlEnum = GlEnum(0x8C8F);
/// 
/// * Group: [`GetPName`]
pub const GL_MOTION_ESTIMATION_SEARCH_BLOCK_X_QCOM: GlEnum = GlEnum(0x8C90);
/// 
/// * Group: [`GetPName`]
pub const GL_MOTION_ESTIMATION_SEARCH_BLOCK_Y_QCOM: GlEnum = GlEnum(0x8C91);
pub const GL_ATC_RGB_AMD: GlEnum = GlEnum(0x8C92);
pub const GL_ATC_RGBA_EXPLICIT_ALPHA_AMD: GlEnum = GlEnum(0x8C93);
pub const GL_POINT_SPRITE_COORD_ORIGIN: GlEnum = GlEnum(0x8CA0);
/// 
/// * Group: [`ClipControlOrigin`]
pub const GL_LOWER_LEFT: GlEnum = GlEnum(0x8CA1);
/// 
/// * Alias Of: [`GL_LOWER_LEFT`]
pub const GL_LOWER_LEFT_EXT: GlEnum = GL_LOWER_LEFT;
/// 
/// * Group: [`ClipControlOrigin`]
pub const GL_UPPER_LEFT: GlEnum = GlEnum(0x8CA2);
/// 
/// * Alias Of: [`GL_UPPER_LEFT`]
pub const GL_UPPER_LEFT_EXT: GlEnum = GL_UPPER_LEFT;
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_REF: GlEnum = GlEnum(0x8CA3);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_VALUE_MASK: GlEnum = GlEnum(0x8CA4);
/// 
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_WRITEMASK: GlEnum = GlEnum(0x8CA5);
/// 
/// * Group: [`GetPName`]
pub const GL_DRAW_FRAMEBUFFER_BINDING: GlEnum = GlEnum(0x8CA6);
pub const GL_DRAW_FRAMEBUFFER_BINDING_ANGLE: GlEnum = GlEnum(0x8CA6);
pub const GL_DRAW_FRAMEBUFFER_BINDING_APPLE: GlEnum = GlEnum(0x8CA6);
pub const GL_DRAW_FRAMEBUFFER_BINDING_EXT: GlEnum = GlEnum(0x8CA6);
pub const GL_DRAW_FRAMEBUFFER_BINDING_NV: GlEnum = GlEnum(0x8CA6);
pub const GL_FRAMEBUFFER_BINDING: GlEnum = GlEnum(0x8CA6);
pub const GL_FRAMEBUFFER_BINDING_ANGLE: GlEnum = GlEnum(0x8CA6);
pub const GL_FRAMEBUFFER_BINDING_EXT: GlEnum = GlEnum(0x8CA6);
pub const GL_FRAMEBUFFER_BINDING_OES: GlEnum = GlEnum(0x8CA6);
/// 
/// * Group: [`GetPName`]
pub const GL_RENDERBUFFER_BINDING: GlEnum = GlEnum(0x8CA7);
pub const GL_RENDERBUFFER_BINDING_ANGLE: GlEnum = GlEnum(0x8CA7);
pub const GL_RENDERBUFFER_BINDING_EXT: GlEnum = GlEnum(0x8CA7);
pub const GL_RENDERBUFFER_BINDING_OES: GlEnum = GlEnum(0x8CA7);
/// 
/// * Group: [`CheckFramebufferStatusTarget`], [`FramebufferTarget`]
pub const GL_READ_FRAMEBUFFER: GlEnum = GlEnum(0x8CA8);
pub const GL_READ_FRAMEBUFFER_ANGLE: GlEnum = GlEnum(0x8CA8);
pub const GL_READ_FRAMEBUFFER_APPLE: GlEnum = GlEnum(0x8CA8);
pub const GL_READ_FRAMEBUFFER_EXT: GlEnum = GlEnum(0x8CA8);
pub const GL_READ_FRAMEBUFFER_NV: GlEnum = GlEnum(0x8CA8);
/// 
/// * Group: [`CheckFramebufferStatusTarget`], [`FramebufferTarget`]
pub const GL_DRAW_FRAMEBUFFER: GlEnum = GlEnum(0x8CA9);
pub const GL_DRAW_FRAMEBUFFER_ANGLE: GlEnum = GlEnum(0x8CA9);
pub const GL_DRAW_FRAMEBUFFER_APPLE: GlEnum = GlEnum(0x8CA9);
pub const GL_DRAW_FRAMEBUFFER_EXT: GlEnum = GlEnum(0x8CA9);
pub const GL_DRAW_FRAMEBUFFER_NV: GlEnum = GlEnum(0x8CA9);
/// 
/// * Group: [`GetPName`]
pub const GL_READ_FRAMEBUFFER_BINDING: GlEnum = GlEnum(0x8CAA);
pub const GL_READ_FRAMEBUFFER_BINDING_ANGLE: GlEnum = GlEnum(0x8CAA);
pub const GL_READ_FRAMEBUFFER_BINDING_APPLE: GlEnum = GlEnum(0x8CAA);
pub const GL_READ_FRAMEBUFFER_BINDING_EXT: GlEnum = GlEnum(0x8CAA);
pub const GL_READ_FRAMEBUFFER_BINDING_NV: GlEnum = GlEnum(0x8CAA);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_COVERAGE_SAMPLES_NV: GlEnum = GlEnum(0x8CAB);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES: GlEnum = GlEnum(0x8CAB);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_ANGLE: GlEnum = GlEnum(0x8CAB);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_APPLE: GlEnum = GlEnum(0x8CAB);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_EXT: GlEnum = GlEnum(0x8CAB);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_NV: GlEnum = GlEnum(0x8CAB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32F: GlEnum = GlEnum(0x8CAC);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH32F_STENCIL8: GlEnum = GlEnum(0x8CAD);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GlEnum = GlEnum(0x8CD0);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_EXT: GlEnum = GlEnum(0x8CD0);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_OES: GlEnum = GlEnum(0x8CD0);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GlEnum = GlEnum(0x8CD1);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_EXT: GlEnum = GlEnum(0x8CD1);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_OES: GlEnum = GlEnum(0x8CD1);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GlEnum = GlEnum(0x8CD2);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_EXT: GlEnum = GlEnum(0x8CD2);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_OES: GlEnum = GlEnum(0x8CD2);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GlEnum = GlEnum(0x8CD3);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_EXT: GlEnum = GlEnum(0x8CD3);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_OES: GlEnum = GlEnum(0x8CD3);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_EXT: GlEnum = GlEnum(0x8CD4);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_OES: GlEnum = GlEnum(0x8CD4);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GlEnum = GlEnum(0x8CD4);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER_EXT: GlEnum = GlEnum(0x8CD4);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_COMPLETE: GlEnum = GlEnum(0x8CD5);
pub const GL_FRAMEBUFFER_COMPLETE_EXT: GlEnum = GlEnum(0x8CD5);
pub const GL_FRAMEBUFFER_COMPLETE_OES: GlEnum = GlEnum(0x8CD5);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GlEnum = GlEnum(0x8CD6);
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT_EXT: GlEnum = GlEnum(0x8CD6);
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT_OES: GlEnum = GlEnum(0x8CD6);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GlEnum = GlEnum(0x8CD7);
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_EXT: GlEnum = GlEnum(0x8CD7);
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_OES: GlEnum = GlEnum(0x8CD7);
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GlEnum = GlEnum(0x8CD9);
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS_EXT: GlEnum = GlEnum(0x8CD9);
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS_OES: GlEnum = GlEnum(0x8CD9);
pub const GL_FRAMEBUFFER_INCOMPLETE_FORMATS_EXT: GlEnum = GlEnum(0x8CDA);
pub const GL_FRAMEBUFFER_INCOMPLETE_FORMATS_OES: GlEnum = GlEnum(0x8CDA);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GlEnum = GlEnum(0x8CDB);
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_EXT: GlEnum = GlEnum(0x8CDB);
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_OES: GlEnum = GlEnum(0x8CDB);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GlEnum = GlEnum(0x8CDC);
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER_EXT: GlEnum = GlEnum(0x8CDC);
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER_OES: GlEnum = GlEnum(0x8CDC);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_UNSUPPORTED: GlEnum = GlEnum(0x8CDD);
pub const GL_FRAMEBUFFER_UNSUPPORTED_EXT: GlEnum = GlEnum(0x8CDD);
pub const GL_FRAMEBUFFER_UNSUPPORTED_OES: GlEnum = GlEnum(0x8CDD);
pub const GL_MAX_COLOR_ATTACHMENTS: GlEnum = GlEnum(0x8CDF);
pub const GL_MAX_COLOR_ATTACHMENTS_EXT: GlEnum = GlEnum(0x8CDF);
pub const GL_MAX_COLOR_ATTACHMENTS_NV: GlEnum = GlEnum(0x8CDF);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT0: GlEnum = GlEnum(0x8CE0);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT0_EXT: GlEnum = GlEnum(0x8CE0);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT0_NV: GlEnum = GlEnum(0x8CE0);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT0_OES: GlEnum = GlEnum(0x8CE0);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT1: GlEnum = GlEnum(0x8CE1);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT1_EXT: GlEnum = GlEnum(0x8CE1);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT1_NV: GlEnum = GlEnum(0x8CE1);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT2: GlEnum = GlEnum(0x8CE2);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT2_EXT: GlEnum = GlEnum(0x8CE2);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT2_NV: GlEnum = GlEnum(0x8CE2);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT3: GlEnum = GlEnum(0x8CE3);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT3_EXT: GlEnum = GlEnum(0x8CE3);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT3_NV: GlEnum = GlEnum(0x8CE3);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT4: GlEnum = GlEnum(0x8CE4);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT4_EXT: GlEnum = GlEnum(0x8CE4);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT4_NV: GlEnum = GlEnum(0x8CE4);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT5: GlEnum = GlEnum(0x8CE5);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT5_EXT: GlEnum = GlEnum(0x8CE5);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT5_NV: GlEnum = GlEnum(0x8CE5);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT6: GlEnum = GlEnum(0x8CE6);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT6_EXT: GlEnum = GlEnum(0x8CE6);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT6_NV: GlEnum = GlEnum(0x8CE6);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT7: GlEnum = GlEnum(0x8CE7);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT7_EXT: GlEnum = GlEnum(0x8CE7);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT7_NV: GlEnum = GlEnum(0x8CE7);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT8: GlEnum = GlEnum(0x8CE8);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT8_EXT: GlEnum = GlEnum(0x8CE8);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT8_NV: GlEnum = GlEnum(0x8CE8);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT9: GlEnum = GlEnum(0x8CE9);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT9_EXT: GlEnum = GlEnum(0x8CE9);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT9_NV: GlEnum = GlEnum(0x8CE9);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT10: GlEnum = GlEnum(0x8CEA);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT10_EXT: GlEnum = GlEnum(0x8CEA);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT10_NV: GlEnum = GlEnum(0x8CEA);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT11: GlEnum = GlEnum(0x8CEB);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT11_EXT: GlEnum = GlEnum(0x8CEB);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT11_NV: GlEnum = GlEnum(0x8CEB);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT12: GlEnum = GlEnum(0x8CEC);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT12_EXT: GlEnum = GlEnum(0x8CEC);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT12_NV: GlEnum = GlEnum(0x8CEC);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT13: GlEnum = GlEnum(0x8CED);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT13_EXT: GlEnum = GlEnum(0x8CED);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT13_NV: GlEnum = GlEnum(0x8CED);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT14: GlEnum = GlEnum(0x8CEE);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT14_EXT: GlEnum = GlEnum(0x8CEE);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT14_NV: GlEnum = GlEnum(0x8CEE);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT15: GlEnum = GlEnum(0x8CEF);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT15_EXT: GlEnum = GlEnum(0x8CEF);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT15_NV: GlEnum = GlEnum(0x8CEF);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT16: GlEnum = GlEnum(0x8CF0);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT17: GlEnum = GlEnum(0x8CF1);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT18: GlEnum = GlEnum(0x8CF2);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT19: GlEnum = GlEnum(0x8CF3);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT20: GlEnum = GlEnum(0x8CF4);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT21: GlEnum = GlEnum(0x8CF5);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT22: GlEnum = GlEnum(0x8CF6);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT23: GlEnum = GlEnum(0x8CF7);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT24: GlEnum = GlEnum(0x8CF8);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT25: GlEnum = GlEnum(0x8CF9);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT26: GlEnum = GlEnum(0x8CFA);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT27: GlEnum = GlEnum(0x8CFB);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT28: GlEnum = GlEnum(0x8CFC);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT29: GlEnum = GlEnum(0x8CFD);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT30: GlEnum = GlEnum(0x8CFE);
/// 
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT31: GlEnum = GlEnum(0x8CFF);
/// 
/// * Group: [`InvalidateFramebufferAttachment`], [`FramebufferAttachment`]
pub const GL_DEPTH_ATTACHMENT: GlEnum = GlEnum(0x8D00);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_DEPTH_ATTACHMENT_EXT: GlEnum = GlEnum(0x8D00);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_DEPTH_ATTACHMENT_OES: GlEnum = GlEnum(0x8D00);
/// 
/// * Group: [`FramebufferAttachment`]
pub const GL_STENCIL_ATTACHMENT: GlEnum = GlEnum(0x8D20);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_STENCIL_ATTACHMENT_EXT: GlEnum = GlEnum(0x8D20);
/// 
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_STENCIL_ATTACHMENT_OES: GlEnum = GlEnum(0x8D20);
/// 
/// * Group: [`ObjectIdentifier`], [`FramebufferTarget`],
///   [`CheckFramebufferStatusTarget`]
pub const GL_FRAMEBUFFER: GlEnum = GlEnum(0x8D40);
pub const GL_FRAMEBUFFER_EXT: GlEnum = GlEnum(0x8D40);
/// 
/// * Group: [`FramebufferTarget`]
pub const GL_FRAMEBUFFER_OES: GlEnum = GlEnum(0x8D40);
/// 
/// * Group: [`ObjectIdentifier`], [`RenderbufferTarget`],
///   [`CopyImageSubDataTarget`]
pub const GL_RENDERBUFFER: GlEnum = GlEnum(0x8D41);
pub const GL_RENDERBUFFER_EXT: GlEnum = GlEnum(0x8D41);
/// 
/// * Group: [`RenderbufferTarget`]
pub const GL_RENDERBUFFER_OES: GlEnum = GlEnum(0x8D41);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_WIDTH: GlEnum = GlEnum(0x8D42);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_WIDTH_EXT: GlEnum = GlEnum(0x8D42);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_WIDTH_OES: GlEnum = GlEnum(0x8D42);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_HEIGHT: GlEnum = GlEnum(0x8D43);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_HEIGHT_EXT: GlEnum = GlEnum(0x8D43);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_HEIGHT_OES: GlEnum = GlEnum(0x8D43);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GlEnum = GlEnum(0x8D44);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_INTERNAL_FORMAT_EXT: GlEnum = GlEnum(0x8D44);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_INTERNAL_FORMAT_OES: GlEnum = GlEnum(0x8D44);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX1: GlEnum = GlEnum(0x8D46);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX1_EXT: GlEnum = GlEnum(0x8D46);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX1_OES: GlEnum = GlEnum(0x8D46);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX4: GlEnum = GlEnum(0x8D47);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX4_EXT: GlEnum = GlEnum(0x8D47);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX4_OES: GlEnum = GlEnum(0x8D47);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX8: GlEnum = GlEnum(0x8D48);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX8_EXT: GlEnum = GlEnum(0x8D48);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX8_OES: GlEnum = GlEnum(0x8D48);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX16: GlEnum = GlEnum(0x8D49);
/// 
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX16_EXT: GlEnum = GlEnum(0x8D49);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_RED_SIZE: GlEnum = GlEnum(0x8D50);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_RED_SIZE_EXT: GlEnum = GlEnum(0x8D50);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_RED_SIZE_OES: GlEnum = GlEnum(0x8D50);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_GREEN_SIZE: GlEnum = GlEnum(0x8D51);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_GREEN_SIZE_EXT: GlEnum = GlEnum(0x8D51);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_GREEN_SIZE_OES: GlEnum = GlEnum(0x8D51);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_BLUE_SIZE: GlEnum = GlEnum(0x8D52);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_BLUE_SIZE_EXT: GlEnum = GlEnum(0x8D52);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_BLUE_SIZE_OES: GlEnum = GlEnum(0x8D52);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_ALPHA_SIZE: GlEnum = GlEnum(0x8D53);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_ALPHA_SIZE_EXT: GlEnum = GlEnum(0x8D53);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_ALPHA_SIZE_OES: GlEnum = GlEnum(0x8D53);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_DEPTH_SIZE: GlEnum = GlEnum(0x8D54);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_DEPTH_SIZE_EXT: GlEnum = GlEnum(0x8D54);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_DEPTH_SIZE_OES: GlEnum = GlEnum(0x8D54);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_STENCIL_SIZE: GlEnum = GlEnum(0x8D55);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_STENCIL_SIZE_EXT: GlEnum = GlEnum(0x8D55);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_STENCIL_SIZE_OES: GlEnum = GlEnum(0x8D55);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GlEnum = GlEnum(0x8D56);
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_ANGLE: GlEnum = GlEnum(0x8D56);
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_APPLE: GlEnum = GlEnum(0x8D56);
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_EXT: GlEnum = GlEnum(0x8D56);
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_NV: GlEnum = GlEnum(0x8D56);
pub const GL_MAX_SAMPLES: GlEnum = GlEnum(0x8D57);
pub const GL_MAX_SAMPLES_ANGLE: GlEnum = GlEnum(0x8D57);
pub const GL_MAX_SAMPLES_APPLE: GlEnum = GlEnum(0x8D57);
pub const GL_MAX_SAMPLES_EXT: GlEnum = GlEnum(0x8D57);
pub const GL_MAX_SAMPLES_NV: GlEnum = GlEnum(0x8D57);
pub const GL_TEXTURE_GEN_STR_OES: GlEnum = GlEnum(0x8D60);
pub const GL_HALF_FLOAT_OES: GlEnum = GlEnum(0x8D61);
pub const GL_RGB565_OES: GlEnum = GlEnum(0x8D62);
pub const GL_RGB565: GlEnum = GlEnum(0x8D62);
pub const GL_ETC1_RGB8_OES: GlEnum = GlEnum(0x8D64);
pub const GL_TEXTURE_EXTERNAL_OES: GlEnum = GlEnum(0x8D65);
pub const GL_SAMPLER_EXTERNAL_OES: GlEnum = GlEnum(0x8D66);
pub const GL_TEXTURE_BINDING_EXTERNAL_OES: GlEnum = GlEnum(0x8D67);
pub const GL_REQUIRED_TEXTURE_IMAGE_UNITS_OES: GlEnum = GlEnum(0x8D68);
/// 
/// * Group: [`EnableCap`]
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GlEnum = GlEnum(0x8D69);
/// 
/// * Group: [`QueryTarget`]
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GlEnum = GlEnum(0x8D6A);
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE_EXT: GlEnum = GlEnum(0x8D6A);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_ELEMENT_INDEX: GlEnum = GlEnum(0x8D6B);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SAMPLES_EXT: GlEnum = GlEnum(0x8D6C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA32UI: GlEnum = GlEnum(0x8D70);
pub const GL_RGBA32UI_EXT: GlEnum = GlEnum(0x8D70);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB32UI: GlEnum = GlEnum(0x8D71);
pub const GL_RGB32UI_EXT: GlEnum = GlEnum(0x8D71);
pub const GL_ALPHA32UI_EXT: GlEnum = GlEnum(0x8D72);
pub const GL_INTENSITY32UI_EXT: GlEnum = GlEnum(0x8D73);
pub const GL_LUMINANCE32UI_EXT: GlEnum = GlEnum(0x8D74);
pub const GL_LUMINANCE_ALPHA32UI_EXT: GlEnum = GlEnum(0x8D75);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA16UI: GlEnum = GlEnum(0x8D76);
pub const GL_RGBA16UI_EXT: GlEnum = GlEnum(0x8D76);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16UI: GlEnum = GlEnum(0x8D77);
pub const GL_RGB16UI_EXT: GlEnum = GlEnum(0x8D77);
pub const GL_ALPHA16UI_EXT: GlEnum = GlEnum(0x8D78);
pub const GL_INTENSITY16UI_EXT: GlEnum = GlEnum(0x8D79);
pub const GL_LUMINANCE16UI_EXT: GlEnum = GlEnum(0x8D7A);
pub const GL_LUMINANCE_ALPHA16UI_EXT: GlEnum = GlEnum(0x8D7B);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA8UI: GlEnum = GlEnum(0x8D7C);
pub const GL_RGBA8UI_EXT: GlEnum = GlEnum(0x8D7C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB8UI: GlEnum = GlEnum(0x8D7D);
pub const GL_RGB8UI_EXT: GlEnum = GlEnum(0x8D7D);
pub const GL_ALPHA8UI_EXT: GlEnum = GlEnum(0x8D7E);
pub const GL_INTENSITY8UI_EXT: GlEnum = GlEnum(0x8D7F);
pub const GL_LUMINANCE8UI_EXT: GlEnum = GlEnum(0x8D80);
pub const GL_LUMINANCE_ALPHA8UI_EXT: GlEnum = GlEnum(0x8D81);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA32I: GlEnum = GlEnum(0x8D82);
pub const GL_RGBA32I_EXT: GlEnum = GlEnum(0x8D82);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB32I: GlEnum = GlEnum(0x8D83);
pub const GL_RGB32I_EXT: GlEnum = GlEnum(0x8D83);
pub const GL_ALPHA32I_EXT: GlEnum = GlEnum(0x8D84);
pub const GL_INTENSITY32I_EXT: GlEnum = GlEnum(0x8D85);
pub const GL_LUMINANCE32I_EXT: GlEnum = GlEnum(0x8D86);
pub const GL_LUMINANCE_ALPHA32I_EXT: GlEnum = GlEnum(0x8D87);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA16I: GlEnum = GlEnum(0x8D88);
pub const GL_RGBA16I_EXT: GlEnum = GlEnum(0x8D88);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16I: GlEnum = GlEnum(0x8D89);
pub const GL_RGB16I_EXT: GlEnum = GlEnum(0x8D89);
pub const GL_ALPHA16I_EXT: GlEnum = GlEnum(0x8D8A);
pub const GL_INTENSITY16I_EXT: GlEnum = GlEnum(0x8D8B);
pub const GL_LUMINANCE16I_EXT: GlEnum = GlEnum(0x8D8C);
pub const GL_LUMINANCE_ALPHA16I_EXT: GlEnum = GlEnum(0x8D8D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA8I: GlEnum = GlEnum(0x8D8E);
pub const GL_RGBA8I_EXT: GlEnum = GlEnum(0x8D8E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB8I: GlEnum = GlEnum(0x8D8F);
pub const GL_RGB8I_EXT: GlEnum = GlEnum(0x8D8F);
pub const GL_ALPHA8I_EXT: GlEnum = GlEnum(0x8D90);
pub const GL_INTENSITY8I_EXT: GlEnum = GlEnum(0x8D91);
pub const GL_LUMINANCE8I_EXT: GlEnum = GlEnum(0x8D92);
pub const GL_LUMINANCE_ALPHA8I_EXT: GlEnum = GlEnum(0x8D93);
/// 
/// * Group: [`PixelFormat`]
pub const GL_RED_INTEGER: GlEnum = GlEnum(0x8D94);
pub const GL_RED_INTEGER_EXT: GlEnum = GlEnum(0x8D94);
/// 
/// * Group: [`PixelFormat`]
pub const GL_GREEN_INTEGER: GlEnum = GlEnum(0x8D95);
pub const GL_GREEN_INTEGER_EXT: GlEnum = GlEnum(0x8D95);
/// 
/// * Group: [`PixelFormat`]
pub const GL_BLUE_INTEGER: GlEnum = GlEnum(0x8D96);
pub const GL_BLUE_INTEGER_EXT: GlEnum = GlEnum(0x8D96);
pub const GL_ALPHA_INTEGER: GlEnum = GlEnum(0x8D97);
pub const GL_ALPHA_INTEGER_EXT: GlEnum = GlEnum(0x8D97);
/// 
/// * Group: [`PixelFormat`]
pub const GL_RGB_INTEGER: GlEnum = GlEnum(0x8D98);
pub const GL_RGB_INTEGER_EXT: GlEnum = GlEnum(0x8D98);
/// 
/// * Group: [`PixelFormat`]
pub const GL_RGBA_INTEGER: GlEnum = GlEnum(0x8D99);
pub const GL_RGBA_INTEGER_EXT: GlEnum = GlEnum(0x8D99);
/// 
/// * Group: [`PixelFormat`]
pub const GL_BGR_INTEGER: GlEnum = GlEnum(0x8D9A);
pub const GL_BGR_INTEGER_EXT: GlEnum = GlEnum(0x8D9A);
/// 
/// * Group: [`PixelFormat`]
pub const GL_BGRA_INTEGER: GlEnum = GlEnum(0x8D9B);
pub const GL_BGRA_INTEGER_EXT: GlEnum = GlEnum(0x8D9B);
pub const GL_LUMINANCE_INTEGER_EXT: GlEnum = GlEnum(0x8D9C);
pub const GL_LUMINANCE_ALPHA_INTEGER_EXT: GlEnum = GlEnum(0x8D9D);
pub const GL_RGBA_INTEGER_MODE_EXT: GlEnum = GlEnum(0x8D9E);
/// 
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_INT_2_10_10_10_REV: GlEnum = GlEnum(0x8D9F);
pub const GL_MAX_PROGRAM_PARAMETER_BUFFER_BINDINGS_NV: GlEnum = GlEnum(0x8DA0);
pub const GL_MAX_PROGRAM_PARAMETER_BUFFER_SIZE_NV: GlEnum = GlEnum(0x8DA1);
pub const GL_VERTEX_PROGRAM_PARAMETER_BUFFER_NV: GlEnum = GlEnum(0x8DA2);
pub const GL_GEOMETRY_PROGRAM_PARAMETER_BUFFER_NV: GlEnum = GlEnum(0x8DA3);
pub const GL_FRAGMENT_PROGRAM_PARAMETER_BUFFER_NV: GlEnum = GlEnum(0x8DA4);
pub const GL_MAX_PROGRAM_GENERIC_ATTRIBS_NV: GlEnum = GlEnum(0x8DA5);
pub const GL_MAX_PROGRAM_GENERIC_RESULTS_NV: GlEnum = GlEnum(0x8DA6);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GlEnum = GlEnum(0x8DA7);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_ARB: GlEnum = GlEnum(0x8DA7);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_EXT: GlEnum = GlEnum(0x8DA7);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_OES: GlEnum = GlEnum(0x8DA7);
/// 
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GlEnum = GlEnum(0x8DA8);
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_ARB: GlEnum = GlEnum(0x8DA8);
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_EXT: GlEnum = GlEnum(0x8DA8);
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_OES: GlEnum = GlEnum(0x8DA8);
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_ARB: GlEnum = GlEnum(0x8DA9);
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_EXT: GlEnum = GlEnum(0x8DA9);
pub const GL_LAYER_NV: GlEnum = GlEnum(0x8DAA);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32F_NV: GlEnum = GlEnum(0x8DAB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_DEPTH32F_STENCIL8_NV: GlEnum = GlEnum(0x8DAC);
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GlEnum = GlEnum(0x8DAD);
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV_NV: GlEnum = GlEnum(0x8DAD);
pub const GL_SHADER_INCLUDE_ARB: GlEnum = GlEnum(0x8DAE);
pub const GL_DEPTH_BUFFER_FLOAT_MODE_NV: GlEnum = GlEnum(0x8DAF);
/// 
/// * Group: [`EnableCap`]
pub const GL_FRAMEBUFFER_SRGB: GlEnum = GlEnum(0x8DB9);
pub const GL_FRAMEBUFFER_SRGB_EXT: GlEnum = GlEnum(0x8DB9);
pub const GL_FRAMEBUFFER_SRGB_CAPABLE_EXT: GlEnum = GlEnum(0x8DBA);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RED_RGTC1: GlEnum = GlEnum(0x8DBB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RED_RGTC1_EXT: GlEnum = GlEnum(0x8DBB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: GlEnum = GlEnum(0x8DBC);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_RED_RGTC1_EXT: GlEnum = GlEnum(0x8DBC);
pub const GL_COMPRESSED_RED_GREEN_RGTC2_EXT: GlEnum = GlEnum(0x8DBD);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RG_RGTC2: GlEnum = GlEnum(0x8DBD);
pub const GL_COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT: GlEnum = GlEnum(0x8DBE);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: GlEnum = GlEnum(0x8DBE);
/// 
/// * Group: [`GlslTypeToken`], [`UniformType`]
pub const GL_SAMPLER_1D_ARRAY: GlEnum = GlEnum(0x8DC0);
pub const GL_SAMPLER_1D_ARRAY_EXT: GlEnum = GlEnum(0x8DC0);
/// 
/// * Group: [`GlslTypeToken`], [`UniformType`]
pub const GL_SAMPLER_2D_ARRAY: GlEnum = GlEnum(0x8DC1);
pub const GL_SAMPLER_2D_ARRAY_EXT: GlEnum = GlEnum(0x8DC1);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_BUFFER: GlEnum = GlEnum(0x8DC2);
pub const GL_SAMPLER_BUFFER_EXT: GlEnum = GlEnum(0x8DC2);
pub const GL_SAMPLER_BUFFER_OES: GlEnum = GlEnum(0x8DC2);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_1D_ARRAY_SHADOW: GlEnum = GlEnum(0x8DC3);
pub const GL_SAMPLER_1D_ARRAY_SHADOW_EXT: GlEnum = GlEnum(0x8DC3);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_ARRAY_SHADOW: GlEnum = GlEnum(0x8DC4);
pub const GL_SAMPLER_2D_ARRAY_SHADOW_EXT: GlEnum = GlEnum(0x8DC4);
pub const GL_SAMPLER_2D_ARRAY_SHADOW_NV: GlEnum = GlEnum(0x8DC4);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_CUBE_SHADOW: GlEnum = GlEnum(0x8DC5);
pub const GL_SAMPLER_CUBE_SHADOW_EXT: GlEnum = GlEnum(0x8DC5);
pub const GL_SAMPLER_CUBE_SHADOW_NV: GlEnum = GlEnum(0x8DC5);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_VEC2: GlEnum = GlEnum(0x8DC6);
pub const GL_UNSIGNED_INT_VEC2_EXT: GlEnum = GlEnum(0x8DC6);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_VEC3: GlEnum = GlEnum(0x8DC7);
pub const GL_UNSIGNED_INT_VEC3_EXT: GlEnum = GlEnum(0x8DC7);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_VEC4: GlEnum = GlEnum(0x8DC8);
pub const GL_UNSIGNED_INT_VEC4_EXT: GlEnum = GlEnum(0x8DC8);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_1D: GlEnum = GlEnum(0x8DC9);
pub const GL_INT_SAMPLER_1D_EXT: GlEnum = GlEnum(0x8DC9);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D: GlEnum = GlEnum(0x8DCA);
pub const GL_INT_SAMPLER_2D_EXT: GlEnum = GlEnum(0x8DCA);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_3D: GlEnum = GlEnum(0x8DCB);
pub const GL_INT_SAMPLER_3D_EXT: GlEnum = GlEnum(0x8DCB);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_CUBE: GlEnum = GlEnum(0x8DCC);
pub const GL_INT_SAMPLER_CUBE_EXT: GlEnum = GlEnum(0x8DCC);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D_RECT: GlEnum = GlEnum(0x8DCD);
pub const GL_INT_SAMPLER_2D_RECT_EXT: GlEnum = GlEnum(0x8DCD);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_1D_ARRAY: GlEnum = GlEnum(0x8DCE);
pub const GL_INT_SAMPLER_1D_ARRAY_EXT: GlEnum = GlEnum(0x8DCE);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D_ARRAY: GlEnum = GlEnum(0x8DCF);
pub const GL_INT_SAMPLER_2D_ARRAY_EXT: GlEnum = GlEnum(0x8DCF);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_BUFFER: GlEnum = GlEnum(0x8DD0);
pub const GL_INT_SAMPLER_BUFFER_EXT: GlEnum = GlEnum(0x8DD0);
pub const GL_INT_SAMPLER_BUFFER_OES: GlEnum = GlEnum(0x8DD0);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_1D: GlEnum = GlEnum(0x8DD1);
pub const GL_UNSIGNED_INT_SAMPLER_1D_EXT: GlEnum = GlEnum(0x8DD1);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D: GlEnum = GlEnum(0x8DD2);
pub const GL_UNSIGNED_INT_SAMPLER_2D_EXT: GlEnum = GlEnum(0x8DD2);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_3D: GlEnum = GlEnum(0x8DD3);
pub const GL_UNSIGNED_INT_SAMPLER_3D_EXT: GlEnum = GlEnum(0x8DD3);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GlEnum = GlEnum(0x8DD4);
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_EXT: GlEnum = GlEnum(0x8DD4);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: GlEnum = GlEnum(0x8DD5);
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT_EXT: GlEnum = GlEnum(0x8DD5);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: GlEnum = GlEnum(0x8DD6);
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY_EXT: GlEnum = GlEnum(0x8DD6);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GlEnum = GlEnum(0x8DD7);
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY_EXT: GlEnum = GlEnum(0x8DD7);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GlEnum = GlEnum(0x8DD8);
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER_EXT: GlEnum = GlEnum(0x8DD8);
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER_OES: GlEnum = GlEnum(0x8DD8);
/// 
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_GEOMETRY_SHADER: GlEnum = GlEnum(0x8DD9);
pub const GL_GEOMETRY_SHADER_ARB: GlEnum = GlEnum(0x8DD9);
pub const GL_GEOMETRY_SHADER_EXT: GlEnum = GlEnum(0x8DD9);
pub const GL_GEOMETRY_SHADER_OES: GlEnum = GlEnum(0x8DD9);
pub const GL_GEOMETRY_VERTICES_OUT_ARB: GlEnum = GlEnum(0x8DDA);
pub const GL_GEOMETRY_VERTICES_OUT_EXT: GlEnum = GlEnum(0x8DDA);
pub const GL_GEOMETRY_INPUT_TYPE_ARB: GlEnum = GlEnum(0x8DDB);
pub const GL_GEOMETRY_INPUT_TYPE_EXT: GlEnum = GlEnum(0x8DDB);
pub const GL_GEOMETRY_OUTPUT_TYPE_ARB: GlEnum = GlEnum(0x8DDC);
pub const GL_GEOMETRY_OUTPUT_TYPE_EXT: GlEnum = GlEnum(0x8DDC);
pub const GL_MAX_GEOMETRY_VARYING_COMPONENTS_ARB: GlEnum = GlEnum(0x8DDD);
pub const GL_MAX_GEOMETRY_VARYING_COMPONENTS_EXT: GlEnum = GlEnum(0x8DDD);
pub const GL_MAX_VERTEX_VARYING_COMPONENTS_ARB: GlEnum = GlEnum(0x8DDE);
pub const GL_MAX_VERTEX_VARYING_COMPONENTS_EXT: GlEnum = GlEnum(0x8DDE);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8DDF);
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_ARB: GlEnum = GlEnum(0x8DDF);
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_EXT: GlEnum = GlEnum(0x8DDF);
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_OES: GlEnum = GlEnum(0x8DDF);
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GlEnum = GlEnum(0x8DE0);
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_ARB: GlEnum = GlEnum(0x8DE0);
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_EXT: GlEnum = GlEnum(0x8DE0);
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_OES: GlEnum = GlEnum(0x8DE0);
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GlEnum = GlEnum(0x8DE1);
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_ARB: GlEnum = GlEnum(0x8DE1);
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_EXT: GlEnum = GlEnum(0x8DE1);
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_OES: GlEnum = GlEnum(0x8DE1);
pub const GL_MAX_VERTEX_BINDABLE_UNIFORMS_EXT: GlEnum = GlEnum(0x8DE2);
pub const GL_MAX_FRAGMENT_BINDABLE_UNIFORMS_EXT: GlEnum = GlEnum(0x8DE3);
pub const GL_MAX_GEOMETRY_BINDABLE_UNIFORMS_EXT: GlEnum = GlEnum(0x8DE4);
/// 
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINES: GlEnum = GlEnum(0x8DE5);
/// 
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: GlEnum = GlEnum(0x8DE6);
pub const GL_MAX_SUBROUTINES: GlEnum = GlEnum(0x8DE7);
pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS: GlEnum = GlEnum(0x8DE8);
pub const GL_NAMED_STRING_LENGTH_ARB: GlEnum = GlEnum(0x8DE9);
pub const GL_NAMED_STRING_TYPE_ARB: GlEnum = GlEnum(0x8DEA);
pub const GL_MAX_BINDABLE_UNIFORM_SIZE_EXT: GlEnum = GlEnum(0x8DED);
pub const GL_UNIFORM_BUFFER_EXT: GlEnum = GlEnum(0x8DEE);
pub const GL_UNIFORM_BUFFER_BINDING_EXT: GlEnum = GlEnum(0x8DEF);
/// 
/// * Group: [`PrecisionType`]
pub const GL_LOW_FLOAT: GlEnum = GlEnum(0x8DF0);
/// 
/// * Group: [`PrecisionType`]
pub const GL_MEDIUM_FLOAT: GlEnum = GlEnum(0x8DF1);
/// 
/// * Group: [`PrecisionType`]
pub const GL_HIGH_FLOAT: GlEnum = GlEnum(0x8DF2);
/// 
/// * Group: [`PrecisionType`]
pub const GL_LOW_INT: GlEnum = GlEnum(0x8DF3);
/// 
/// * Group: [`PrecisionType`]
pub const GL_MEDIUM_INT: GlEnum = GlEnum(0x8DF4);
/// 
/// * Group: [`PrecisionType`]
pub const GL_HIGH_INT: GlEnum = GlEnum(0x8DF5);
pub const GL_UNSIGNED_INT_10_10_10_2_OES: GlEnum = GlEnum(0x8DF6);
pub const GL_INT_10_10_10_2_OES: GlEnum = GlEnum(0x8DF7);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADER_BINARY_FORMATS: GlEnum = GlEnum(0x8DF8);
/// 
/// * Group: [`GetPName`]
pub const GL_NUM_SHADER_BINARY_FORMATS: GlEnum = GlEnum(0x8DF9);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADER_COMPILER: GlEnum = GlEnum(0x8DFA);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GlEnum = GlEnum(0x8DFB);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VARYING_VECTORS: GlEnum = GlEnum(0x8DFC);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GlEnum = GlEnum(0x8DFD);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_COLOR_SAMPLES_NV: GlEnum = GlEnum(0x8E10);
pub const GL_MAX_MULTISAMPLE_COVERAGE_MODES_NV: GlEnum = GlEnum(0x8E11);
pub const GL_MULTISAMPLE_COVERAGE_MODES_NV: GlEnum = GlEnum(0x8E12);
/// 
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_WAIT: GlEnum = GlEnum(0x8E13);
pub const GL_QUERY_WAIT_NV: GlEnum = GlEnum(0x8E13);
/// 
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_NO_WAIT: GlEnum = GlEnum(0x8E14);
pub const GL_QUERY_NO_WAIT_NV: GlEnum = GlEnum(0x8E14);
/// 
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_BY_REGION_WAIT: GlEnum = GlEnum(0x8E15);
pub const GL_QUERY_BY_REGION_WAIT_NV: GlEnum = GlEnum(0x8E15);
/// 
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_BY_REGION_NO_WAIT: GlEnum = GlEnum(0x8E16);
pub const GL_QUERY_BY_REGION_NO_WAIT_NV: GlEnum = GlEnum(0x8E16);
/// 
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_WAIT_INVERTED: GlEnum = GlEnum(0x8E17);
/// 
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_NO_WAIT_INVERTED: GlEnum = GlEnum(0x8E18);
/// 
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_BY_REGION_WAIT_INVERTED: GlEnum = GlEnum(0x8E19);
/// 
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_BY_REGION_NO_WAIT_INVERTED: GlEnum = GlEnum(0x8E1A);
pub const GL_POLYGON_OFFSET_CLAMP: GlEnum = GlEnum(0x8E1B);
/// 
/// * Alias Of: [`GL_POLYGON_OFFSET_CLAMP`]
pub const GL_POLYGON_OFFSET_CLAMP_EXT: GlEnum = GL_POLYGON_OFFSET_CLAMP;
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8E1E);
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS_EXT: GlEnum = GlEnum(0x8E1E);
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS_OES: GlEnum = GlEnum(0x8E1E);
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8E1F);
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS_EXT: GlEnum = GlEnum(0x8E1F);
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS_OES: GlEnum = GlEnum(0x8E1F);
pub const GL_COLOR_SAMPLES_NV: GlEnum = GlEnum(0x8E20);
/// 
/// * Group: [`ObjectIdentifier`], [`BindTransformFeedbackTarget`]
pub const GL_TRANSFORM_FEEDBACK: GlEnum = GlEnum(0x8E22);
pub const GL_TRANSFORM_FEEDBACK_NV: GlEnum = GlEnum(0x8E22);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: GlEnum = GlEnum(0x8E23);
/// 
/// * Group: [`TransformFeedbackPName`]
/// * Alias Of: [`GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED`]
pub const GL_TRANSFORM_FEEDBACK_PAUSED: GlEnum = GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED_NV: GlEnum = GlEnum(0x8E23);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GlEnum = GlEnum(0x8E24);
/// 
/// * Group: [`TransformFeedbackPName`]
/// * Alias Of: [`GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE`]
pub const GL_TRANSFORM_FEEDBACK_ACTIVE: GlEnum = GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE_NV: GlEnum = GlEnum(0x8E24);
pub const GL_TRANSFORM_FEEDBACK_BINDING: GlEnum = GlEnum(0x8E25);
pub const GL_TRANSFORM_FEEDBACK_BINDING_NV: GlEnum = GlEnum(0x8E25);
pub const GL_FRAME_NV: GlEnum = GlEnum(0x8E26);
pub const GL_FIELDS_NV: GlEnum = GlEnum(0x8E27);
pub const GL_CURRENT_TIME_NV: GlEnum = GlEnum(0x8E28);
/// 
/// * Group: [`QueryCounterTarget`], [`GetPName`]
pub const GL_TIMESTAMP: GlEnum = GlEnum(0x8E28);
pub const GL_TIMESTAMP_EXT: GlEnum = GlEnum(0x8E28);
pub const GL_NUM_FILL_STREAMS_NV: GlEnum = GlEnum(0x8E29);
pub const GL_PRESENT_TIME_NV: GlEnum = GlEnum(0x8E2A);
pub const GL_PRESENT_DURATION_NV: GlEnum = GlEnum(0x8E2B);
pub const GL_DEPTH_COMPONENT16_NONLINEAR_NV: GlEnum = GlEnum(0x8E2C);
pub const GL_PROGRAM_MATRIX_EXT: GlEnum = GlEnum(0x8E2D);
pub const GL_TRANSPOSE_PROGRAM_MATRIX_EXT: GlEnum = GlEnum(0x8E2E);
pub const GL_PROGRAM_MATRIX_STACK_DEPTH_EXT: GlEnum = GlEnum(0x8E2F);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_R: GlEnum = GlEnum(0x8E42);
pub const GL_TEXTURE_SWIZZLE_R_EXT: GlEnum = GlEnum(0x8E42);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_G: GlEnum = GlEnum(0x8E43);
pub const GL_TEXTURE_SWIZZLE_G_EXT: GlEnum = GlEnum(0x8E43);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_B: GlEnum = GlEnum(0x8E44);
pub const GL_TEXTURE_SWIZZLE_B_EXT: GlEnum = GlEnum(0x8E44);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_A: GlEnum = GlEnum(0x8E45);
pub const GL_TEXTURE_SWIZZLE_A_EXT: GlEnum = GlEnum(0x8E45);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_RGBA: GlEnum = GlEnum(0x8E46);
pub const GL_TEXTURE_SWIZZLE_RGBA_EXT: GlEnum = GlEnum(0x8E46);
/// 
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GlEnum = GlEnum(0x8E47);
/// 
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: GlEnum = GlEnum(0x8E48);
/// 
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GlEnum = GlEnum(0x8E49);
/// 
/// * Group: [`ProgramResourceProperty`], [`SubroutineParameterName`]
pub const GL_NUM_COMPATIBLE_SUBROUTINES: GlEnum = GlEnum(0x8E4A);
/// 
/// * Group: [`ProgramResourceProperty`], [`SubroutineParameterName`]
pub const GL_COMPATIBLE_SUBROUTINES: GlEnum = GlEnum(0x8E4B);
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GlEnum = GlEnum(0x8E4C);
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION_EXT: GlEnum = GlEnum(0x8E4C);
/// 
/// * Group: [`VertexProvokingMode`]
pub const GL_FIRST_VERTEX_CONVENTION: GlEnum = GlEnum(0x8E4D);
pub const GL_FIRST_VERTEX_CONVENTION_EXT: GlEnum = GlEnum(0x8E4D);
pub const GL_FIRST_VERTEX_CONVENTION_OES: GlEnum = GlEnum(0x8E4D);
/// 
/// * Group: [`VertexProvokingMode`]
pub const GL_LAST_VERTEX_CONVENTION: GlEnum = GlEnum(0x8E4E);
pub const GL_LAST_VERTEX_CONVENTION_EXT: GlEnum = GlEnum(0x8E4E);
pub const GL_LAST_VERTEX_CONVENTION_OES: GlEnum = GlEnum(0x8E4E);
/// 
/// * Group: [`GetPName`]
pub const GL_PROVOKING_VERTEX: GlEnum = GlEnum(0x8E4F);
pub const GL_PROVOKING_VERTEX_EXT: GlEnum = GlEnum(0x8E4F);
/// 
/// * Group: [`GetMultisamplePNameNV`]
pub const GL_SAMPLE_POSITION: GlEnum = GlEnum(0x8E50);
pub const GL_SAMPLE_POSITION_NV: GlEnum = GlEnum(0x8E50);
/// 
/// * Group: [`GetMultisamplePNameNV`]
/// * Alias Of: [`GL_SAMPLE_POSITION`]
pub const GL_SAMPLE_LOCATION_ARB: GlEnum = GL_SAMPLE_POSITION;
/// 
/// * Alias Of: [`GL_SAMPLE_POSITION_NV`]
pub const GL_SAMPLE_LOCATION_NV: GlEnum = GL_SAMPLE_POSITION_NV;
/// 
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_MASK: GlEnum = GlEnum(0x8E51);
pub const GL_SAMPLE_MASK_NV: GlEnum = GlEnum(0x8E51);
pub const GL_SAMPLE_MASK_VALUE: GlEnum = GlEnum(0x8E52);
pub const GL_SAMPLE_MASK_VALUE_NV: GlEnum = GlEnum(0x8E52);
pub const GL_TEXTURE_BINDING_RENDERBUFFER_NV: GlEnum = GlEnum(0x8E53);
pub const GL_TEXTURE_RENDERBUFFER_DATA_STORE_BINDING_NV: GlEnum = GlEnum(0x8E54);
pub const GL_TEXTURE_RENDERBUFFER_NV: GlEnum = GlEnum(0x8E55);
pub const GL_SAMPLER_RENDERBUFFER_NV: GlEnum = GlEnum(0x8E56);
pub const GL_INT_SAMPLER_RENDERBUFFER_NV: GlEnum = GlEnum(0x8E57);
pub const GL_UNSIGNED_INT_SAMPLER_RENDERBUFFER_NV: GlEnum = GlEnum(0x8E58);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_SAMPLE_MASK_WORDS: GlEnum = GlEnum(0x8E59);
pub const GL_MAX_SAMPLE_MASK_WORDS_NV: GlEnum = GlEnum(0x8E59);
pub const GL_MAX_GEOMETRY_PROGRAM_INVOCATIONS_NV: GlEnum = GlEnum(0x8E5A);
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: GlEnum = GlEnum(0x8E5A);
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS_EXT: GlEnum = GlEnum(0x8E5A);
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS_OES: GlEnum = GlEnum(0x8E5A);
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: GlEnum = GlEnum(0x8E5B);
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET_OES: GlEnum = GlEnum(0x8E5B);
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET_NV: GlEnum = GlEnum(0x8E5B);
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: GlEnum = GlEnum(0x8E5C);
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET_OES: GlEnum = GlEnum(0x8E5C);
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET_NV: GlEnum = GlEnum(0x8E5C);
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: GlEnum = GlEnum(0x8E5D);
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS_OES: GlEnum = GlEnum(0x8E5D);
pub const GL_FRAGMENT_PROGRAM_INTERPOLATION_OFFSET_BITS_NV: GlEnum = GlEnum(0x8E5D);
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GlEnum = GlEnum(0x8E5E);
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET_ARB: GlEnum = GlEnum(0x8E5E);
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET_NV: GlEnum = GlEnum(0x8E5E);
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GlEnum = GlEnum(0x8E5F);
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET_ARB: GlEnum = GlEnum(0x8E5F);
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET_NV: GlEnum = GlEnum(0x8E5F);
pub const GL_MAX_MESH_UNIFORM_BLOCKS_NV: GlEnum = GlEnum(0x8E60);
pub const GL_MAX_MESH_TEXTURE_IMAGE_UNITS_NV: GlEnum = GlEnum(0x8E61);
pub const GL_MAX_MESH_IMAGE_UNIFORMS_NV: GlEnum = GlEnum(0x8E62);
pub const GL_MAX_MESH_UNIFORM_COMPONENTS_NV: GlEnum = GlEnum(0x8E63);
pub const GL_MAX_MESH_ATOMIC_COUNTER_BUFFERS_NV: GlEnum = GlEnum(0x8E64);
pub const GL_MAX_MESH_ATOMIC_COUNTERS_NV: GlEnum = GlEnum(0x8E65);
pub const GL_MAX_MESH_SHADER_STORAGE_BLOCKS_NV: GlEnum = GlEnum(0x8E66);
pub const GL_MAX_COMBINED_MESH_UNIFORM_COMPONENTS_NV: GlEnum = GlEnum(0x8E67);
pub const GL_MAX_TASK_UNIFORM_BLOCKS_NV: GlEnum = GlEnum(0x8E68);
pub const GL_MAX_TASK_TEXTURE_IMAGE_UNITS_NV: GlEnum = GlEnum(0x8E69);
pub const GL_MAX_TASK_IMAGE_UNIFORMS_NV: GlEnum = GlEnum(0x8E6A);
pub const GL_MAX_TASK_UNIFORM_COMPONENTS_NV: GlEnum = GlEnum(0x8E6B);
pub const GL_MAX_TASK_ATOMIC_COUNTER_BUFFERS_NV: GlEnum = GlEnum(0x8E6C);
pub const GL_MAX_TASK_ATOMIC_COUNTERS_NV: GlEnum = GlEnum(0x8E6D);
pub const GL_MAX_TASK_SHADER_STORAGE_BLOCKS_NV: GlEnum = GlEnum(0x8E6E);
pub const GL_MAX_COMBINED_TASK_UNIFORM_COMPONENTS_NV: GlEnum = GlEnum(0x8E6F);
pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS: GlEnum = GlEnum(0x8E70);
pub const GL_MAX_VERTEX_STREAMS: GlEnum = GlEnum(0x8E71);
/// 
/// * Group: [`PatchParameterName`]
pub const GL_PATCH_VERTICES: GlEnum = GlEnum(0x8E72);
pub const GL_PATCH_VERTICES_EXT: GlEnum = GlEnum(0x8E72);
pub const GL_PATCH_VERTICES_OES: GlEnum = GlEnum(0x8E72);
/// 
/// * Group: [`PatchParameterName`]
pub const GL_PATCH_DEFAULT_INNER_LEVEL: GlEnum = GlEnum(0x8E73);
pub const GL_PATCH_DEFAULT_INNER_LEVEL_EXT: GlEnum = GlEnum(0x8E73);
/// 
/// * Group: [`PatchParameterName`]
pub const GL_PATCH_DEFAULT_OUTER_LEVEL: GlEnum = GlEnum(0x8E74);
pub const GL_PATCH_DEFAULT_OUTER_LEVEL_EXT: GlEnum = GlEnum(0x8E74);
pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GlEnum = GlEnum(0x8E75);
pub const GL_TESS_CONTROL_OUTPUT_VERTICES_EXT: GlEnum = GlEnum(0x8E75);
pub const GL_TESS_CONTROL_OUTPUT_VERTICES_OES: GlEnum = GlEnum(0x8E75);
pub const GL_TESS_GEN_MODE: GlEnum = GlEnum(0x8E76);
pub const GL_TESS_GEN_MODE_EXT: GlEnum = GlEnum(0x8E76);
pub const GL_TESS_GEN_MODE_OES: GlEnum = GlEnum(0x8E76);
pub const GL_TESS_GEN_SPACING: GlEnum = GlEnum(0x8E77);
pub const GL_TESS_GEN_SPACING_EXT: GlEnum = GlEnum(0x8E77);
pub const GL_TESS_GEN_SPACING_OES: GlEnum = GlEnum(0x8E77);
pub const GL_TESS_GEN_VERTEX_ORDER: GlEnum = GlEnum(0x8E78);
pub const GL_TESS_GEN_VERTEX_ORDER_EXT: GlEnum = GlEnum(0x8E78);
pub const GL_TESS_GEN_VERTEX_ORDER_OES: GlEnum = GlEnum(0x8E78);
pub const GL_TESS_GEN_POINT_MODE: GlEnum = GlEnum(0x8E79);
pub const GL_TESS_GEN_POINT_MODE_EXT: GlEnum = GlEnum(0x8E79);
pub const GL_TESS_GEN_POINT_MODE_OES: GlEnum = GlEnum(0x8E79);
pub const GL_ISOLINES: GlEnum = GlEnum(0x8E7A);
pub const GL_ISOLINES_EXT: GlEnum = GlEnum(0x8E7A);
pub const GL_ISOLINES_OES: GlEnum = GlEnum(0x8E7A);
pub const GL_FRACTIONAL_ODD: GlEnum = GlEnum(0x8E7B);
pub const GL_FRACTIONAL_ODD_EXT: GlEnum = GlEnum(0x8E7B);
pub const GL_FRACTIONAL_ODD_OES: GlEnum = GlEnum(0x8E7B);
pub const GL_FRACTIONAL_EVEN: GlEnum = GlEnum(0x8E7C);
pub const GL_FRACTIONAL_EVEN_EXT: GlEnum = GlEnum(0x8E7C);
pub const GL_FRACTIONAL_EVEN_OES: GlEnum = GlEnum(0x8E7C);
pub const GL_MAX_PATCH_VERTICES: GlEnum = GlEnum(0x8E7D);
pub const GL_MAX_PATCH_VERTICES_EXT: GlEnum = GlEnum(0x8E7D);
pub const GL_MAX_PATCH_VERTICES_OES: GlEnum = GlEnum(0x8E7D);
pub const GL_MAX_TESS_GEN_LEVEL: GlEnum = GlEnum(0x8E7E);
pub const GL_MAX_TESS_GEN_LEVEL_EXT: GlEnum = GlEnum(0x8E7E);
pub const GL_MAX_TESS_GEN_LEVEL_OES: GlEnum = GlEnum(0x8E7E);
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8E7F);
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS_EXT: GlEnum = GlEnum(0x8E7F);
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS_OES: GlEnum = GlEnum(0x8E7F);
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GlEnum = GlEnum(0x8E80);
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS_EXT: GlEnum = GlEnum(0x8E80);
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS_OES: GlEnum = GlEnum(0x8E80);
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GlEnum = GlEnum(0x8E81);
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS_EXT: GlEnum = GlEnum(0x8E81);
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS_OES: GlEnum = GlEnum(0x8E81);
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GlEnum = GlEnum(0x8E82);
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS_EXT: GlEnum = GlEnum(0x8E82);
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS_OES: GlEnum = GlEnum(0x8E82);
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GlEnum = GlEnum(0x8E83);
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS_EXT: GlEnum = GlEnum(0x8E83);
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS_OES: GlEnum = GlEnum(0x8E83);
pub const GL_MAX_TESS_PATCH_COMPONENTS: GlEnum = GlEnum(0x8E84);
pub const GL_MAX_TESS_PATCH_COMPONENTS_EXT: GlEnum = GlEnum(0x8E84);
pub const GL_MAX_TESS_PATCH_COMPONENTS_OES: GlEnum = GlEnum(0x8E84);
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GlEnum = GlEnum(0x8E85);
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS_EXT: GlEnum = GlEnum(0x8E85);
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS_OES: GlEnum = GlEnum(0x8E85);
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GlEnum = GlEnum(0x8E86);
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS_EXT: GlEnum = GlEnum(0x8E86);
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS_OES: GlEnum = GlEnum(0x8E86);
/// 
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_TESS_EVALUATION_SHADER: GlEnum = GlEnum(0x8E87);
pub const GL_TESS_EVALUATION_SHADER_EXT: GlEnum = GlEnum(0x8E87);
pub const GL_TESS_EVALUATION_SHADER_OES: GlEnum = GlEnum(0x8E87);
/// 
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_TESS_CONTROL_SHADER: GlEnum = GlEnum(0x8E88);
pub const GL_TESS_CONTROL_SHADER_EXT: GlEnum = GlEnum(0x8E88);
pub const GL_TESS_CONTROL_SHADER_OES: GlEnum = GlEnum(0x8E88);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GlEnum = GlEnum(0x8E89);
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS_EXT: GlEnum = GlEnum(0x8E89);
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS_OES: GlEnum = GlEnum(0x8E89);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GlEnum = GlEnum(0x8E8A);
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS_EXT: GlEnum = GlEnum(0x8E8A);
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS_OES: GlEnum = GlEnum(0x8E8A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_BPTC_UNORM: GlEnum = GlEnum(0x8E8C);
pub const GL_COMPRESSED_RGBA_BPTC_UNORM_ARB: GlEnum = GlEnum(0x8E8C);
pub const GL_COMPRESSED_RGBA_BPTC_UNORM_EXT: GlEnum = GlEnum(0x8E8C);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GlEnum = GlEnum(0x8E8D);
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB: GlEnum = GlEnum(0x8E8D);
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT: GlEnum = GlEnum(0x8E8D);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GlEnum = GlEnum(0x8E8E);
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB: GlEnum = GlEnum(0x8E8E);
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT: GlEnum = GlEnum(0x8E8E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GlEnum = GlEnum(0x8E8F);
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB: GlEnum = GlEnum(0x8E8F);
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT: GlEnum = GlEnum(0x8E8F);
pub const GL_COVERAGE_COMPONENT_NV: GlEnum = GlEnum(0x8ED0);
pub const GL_COVERAGE_COMPONENT4_NV: GlEnum = GlEnum(0x8ED1);
pub const GL_COVERAGE_ATTACHMENT_NV: GlEnum = GlEnum(0x8ED2);
pub const GL_COVERAGE_BUFFERS_NV: GlEnum = GlEnum(0x8ED3);
pub const GL_COVERAGE_SAMPLES_NV: GlEnum = GlEnum(0x8ED4);
pub const GL_COVERAGE_ALL_FRAGMENTS_NV: GlEnum = GlEnum(0x8ED5);
pub const GL_COVERAGE_EDGE_FRAGMENTS_NV: GlEnum = GlEnum(0x8ED6);
pub const GL_COVERAGE_AUTOMATIC_NV: GlEnum = GlEnum(0x8ED7);
pub const GL_INCLUSIVE_EXT: GlEnum = GlEnum(0x8F10);
pub const GL_EXCLUSIVE_EXT: GlEnum = GlEnum(0x8F11);
pub const GL_WINDOW_RECTANGLE_EXT: GlEnum = GlEnum(0x8F12);
pub const GL_WINDOW_RECTANGLE_MODE_EXT: GlEnum = GlEnum(0x8F13);
pub const GL_MAX_WINDOW_RECTANGLES_EXT: GlEnum = GlEnum(0x8F14);
pub const GL_NUM_WINDOW_RECTANGLES_EXT: GlEnum = GlEnum(0x8F15);
pub const GL_BUFFER_GPU_ADDRESS_NV: GlEnum = GlEnum(0x8F1D);
pub const GL_VERTEX_ATTRIB_ARRAY_UNIFIED_NV: GlEnum = GlEnum(0x8F1E);
pub const GL_ELEMENT_ARRAY_UNIFIED_NV: GlEnum = GlEnum(0x8F1F);
pub const GL_VERTEX_ATTRIB_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F20);
pub const GL_VERTEX_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F21);
pub const GL_NORMAL_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F22);
pub const GL_COLOR_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F23);
pub const GL_INDEX_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F24);
pub const GL_TEXTURE_COORD_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F25);
pub const GL_EDGE_FLAG_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F26);
pub const GL_SECONDARY_COLOR_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F27);
pub const GL_FOG_COORD_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F28);
pub const GL_ELEMENT_ARRAY_ADDRESS_NV: GlEnum = GlEnum(0x8F29);
pub const GL_VERTEX_ATTRIB_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F2A);
pub const GL_VERTEX_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F2B);
pub const GL_NORMAL_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F2C);
pub const GL_COLOR_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F2D);
pub const GL_INDEX_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F2E);
pub const GL_TEXTURE_COORD_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F2F);
pub const GL_EDGE_FLAG_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F30);
pub const GL_SECONDARY_COLOR_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F31);
pub const GL_FOG_COORD_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F32);
pub const GL_ELEMENT_ARRAY_LENGTH_NV: GlEnum = GlEnum(0x8F33);
pub const GL_GPU_ADDRESS_NV: GlEnum = GlEnum(0x8F34);
pub const GL_MAX_SHADER_BUFFER_ADDRESS_NV: GlEnum = GlEnum(0x8F35);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_COPY_READ_BUFFER: GlEnum = GlEnum(0x8F36);
pub const GL_COPY_READ_BUFFER_NV: GlEnum = GlEnum(0x8F36);
/// 
/// * Alias Of: [`GL_COPY_READ_BUFFER`]
pub const GL_COPY_READ_BUFFER_BINDING: GlEnum = GL_COPY_READ_BUFFER;
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_COPY_WRITE_BUFFER: GlEnum = GlEnum(0x8F37);
pub const GL_COPY_WRITE_BUFFER_NV: GlEnum = GlEnum(0x8F37);
/// 
/// * Alias Of: [`GL_COPY_WRITE_BUFFER`]
pub const GL_COPY_WRITE_BUFFER_BINDING: GlEnum = GL_COPY_WRITE_BUFFER;
pub const GL_MAX_IMAGE_UNITS: GlEnum = GlEnum(0x8F38);
pub const GL_MAX_IMAGE_UNITS_EXT: GlEnum = GlEnum(0x8F38);
pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GlEnum = GlEnum(0x8F39);
pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS_EXT: GlEnum = GlEnum(0x8F39);
/// 
/// * Alias Of: [`GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS`]
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GlEnum = GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS;
pub const GL_IMAGE_BINDING_NAME: GlEnum = GlEnum(0x8F3A);
pub const GL_IMAGE_BINDING_NAME_EXT: GlEnum = GlEnum(0x8F3A);
pub const GL_IMAGE_BINDING_LEVEL: GlEnum = GlEnum(0x8F3B);
pub const GL_IMAGE_BINDING_LEVEL_EXT: GlEnum = GlEnum(0x8F3B);
pub const GL_IMAGE_BINDING_LAYERED: GlEnum = GlEnum(0x8F3C);
pub const GL_IMAGE_BINDING_LAYERED_EXT: GlEnum = GlEnum(0x8F3C);
pub const GL_IMAGE_BINDING_LAYER: GlEnum = GlEnum(0x8F3D);
pub const GL_IMAGE_BINDING_LAYER_EXT: GlEnum = GlEnum(0x8F3D);
pub const GL_IMAGE_BINDING_ACCESS: GlEnum = GlEnum(0x8F3E);
pub const GL_IMAGE_BINDING_ACCESS_EXT: GlEnum = GlEnum(0x8F3E);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_DRAW_INDIRECT_BUFFER: GlEnum = GlEnum(0x8F3F);
pub const GL_DRAW_INDIRECT_UNIFIED_NV: GlEnum = GlEnum(0x8F40);
pub const GL_DRAW_INDIRECT_ADDRESS_NV: GlEnum = GlEnum(0x8F41);
pub const GL_DRAW_INDIRECT_LENGTH_NV: GlEnum = GlEnum(0x8F42);
pub const GL_DRAW_INDIRECT_BUFFER_BINDING: GlEnum = GlEnum(0x8F43);
pub const GL_MAX_PROGRAM_SUBROUTINE_PARAMETERS_NV: GlEnum = GlEnum(0x8F44);
pub const GL_MAX_PROGRAM_SUBROUTINE_NUM_NV: GlEnum = GlEnum(0x8F45);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_MAT2: GlEnum = GlEnum(0x8F46);
pub const GL_DOUBLE_MAT2_EXT: GlEnum = GlEnum(0x8F46);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_MAT3: GlEnum = GlEnum(0x8F47);
pub const GL_DOUBLE_MAT3_EXT: GlEnum = GlEnum(0x8F47);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_MAT4: GlEnum = GlEnum(0x8F48);
pub const GL_DOUBLE_MAT4_EXT: GlEnum = GlEnum(0x8F48);
/// 
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT2x3: GlEnum = GlEnum(0x8F49);
pub const GL_DOUBLE_MAT2x3_EXT: GlEnum = GlEnum(0x8F49);
/// 
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT2x4: GlEnum = GlEnum(0x8F4A);
pub const GL_DOUBLE_MAT2x4_EXT: GlEnum = GlEnum(0x8F4A);
/// 
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT3x2: GlEnum = GlEnum(0x8F4B);
pub const GL_DOUBLE_MAT3x2_EXT: GlEnum = GlEnum(0x8F4B);
/// 
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT3x4: GlEnum = GlEnum(0x8F4C);
pub const GL_DOUBLE_MAT3x4_EXT: GlEnum = GlEnum(0x8F4C);
/// 
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT4x2: GlEnum = GlEnum(0x8F4D);
pub const GL_DOUBLE_MAT4x2_EXT: GlEnum = GlEnum(0x8F4D);
/// 
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT4x3: GlEnum = GlEnum(0x8F4E);
pub const GL_DOUBLE_MAT4x3_EXT: GlEnum = GlEnum(0x8F4E);
pub const GL_VERTEX_BINDING_BUFFER: GlEnum = GlEnum(0x8F4F);
/// 
/// * Group: [`ShaderBinaryFormat`]
pub const GL_MALI_SHADER_BINARY_ARM: GlEnum = GlEnum(0x8F60);
pub const GL_MALI_PROGRAM_BINARY_ARM: GlEnum = GlEnum(0x8F61);
pub const GL_MAX_SHADER_PIXEL_LOCAL_STORAGE_FAST_SIZE_EXT: GlEnum = GlEnum(0x8F63);
pub const GL_SHADER_PIXEL_LOCAL_STORAGE_EXT: GlEnum = GlEnum(0x8F64);
pub const GL_FETCH_PER_SAMPLE_ARM: GlEnum = GlEnum(0x8F65);
pub const GL_FRAGMENT_SHADER_FRAMEBUFFER_FETCH_MRT_ARM: GlEnum = GlEnum(0x8F66);
pub const GL_MAX_SHADER_PIXEL_LOCAL_STORAGE_SIZE_EXT: GlEnum = GlEnum(0x8F67);
pub const GL_TEXTURE_ASTC_DECODE_PRECISION_EXT: GlEnum = GlEnum(0x8F69);
/// 
/// * Group: [`SamplerParameterF`], [`SamplerParameterI`],
///   [`GetTextureParameter`], [`TextureParameterName`]
pub const GL_TEXTURE_UNNORMALIZED_COORDINATES_ARM: GlEnum = GlEnum(0x8F6A);
pub const GL_RED_SNORM: GlEnum = GlEnum(0x8F90);
pub const GL_RG_SNORM: GlEnum = GlEnum(0x8F91);
pub const GL_RGB_SNORM: GlEnum = GlEnum(0x8F92);
pub const GL_RGBA_SNORM: GlEnum = GlEnum(0x8F93);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R8_SNORM: GlEnum = GlEnum(0x8F94);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG8_SNORM: GlEnum = GlEnum(0x8F95);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB8_SNORM: GlEnum = GlEnum(0x8F96);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGBA8_SNORM: GlEnum = GlEnum(0x8F97);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R16_SNORM: GlEnum = GlEnum(0x8F98);
/// 
/// * Group: [`InternalFormat`]
pub const GL_R16_SNORM_EXT: GlEnum = GlEnum(0x8F98);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG16_SNORM: GlEnum = GlEnum(0x8F99);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RG16_SNORM_EXT: GlEnum = GlEnum(0x8F99);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16_SNORM: GlEnum = GlEnum(0x8F9A);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB16_SNORM_EXT: GlEnum = GlEnum(0x8F9A);
pub const GL_RGBA16_SNORM: GlEnum = GlEnum(0x8F9B);
pub const GL_RGBA16_SNORM_EXT: GlEnum = GlEnum(0x8F9B);
pub const GL_SIGNED_NORMALIZED: GlEnum = GlEnum(0x8F9C);
/// 
/// * Group: [`EnableCap`]
pub const GL_PRIMITIVE_RESTART: GlEnum = GlEnum(0x8F9D);
/// 
/// * Group: [`GetPName`]
pub const GL_PRIMITIVE_RESTART_INDEX: GlEnum = GlEnum(0x8F9E);
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_COMPONENTS_ARB: GlEnum = GlEnum(0x8F9F);
pub const GL_PERFMON_GLOBAL_MODE_QCOM: GlEnum = GlEnum(0x8FA0);
pub const GL_MAX_SHADER_SUBSAMPLED_IMAGE_UNITS_QCOM: GlEnum = GlEnum(0x8FA1);
/// 
/// * Group: [`HintTarget`]
pub const GL_BINNING_CONTROL_HINT_QCOM: GlEnum = GlEnum(0x8FB0);
pub const GL_CPU_OPTIMIZED_QCOM: GlEnum = GlEnum(0x8FB1);
pub const GL_GPU_OPTIMIZED_QCOM: GlEnum = GlEnum(0x8FB2);
pub const GL_RENDER_DIRECT_TO_FRAMEBUFFER_QCOM: GlEnum = GlEnum(0x8FB3);
pub const GL_GPU_DISJOINT_EXT: GlEnum = GlEnum(0x8FBB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SR8_EXT: GlEnum = GlEnum(0x8FBD);
/// 
/// * Group: [`InternalFormat`]
pub const GL_SRG8_EXT: GlEnum = GlEnum(0x8FBE);
pub const GL_TEXTURE_FORMAT_SRGB_OVERRIDE_EXT: GlEnum = GlEnum(0x8FBF);
/// 
/// * Group: [`ShaderBinaryFormat`]
pub const GL_SHADER_BINARY_VIV: GlEnum = GlEnum(0x8FC4);
pub const GL_INT8_NV: GlEnum = GlEnum(0x8FE0);
pub const GL_INT8_VEC2_NV: GlEnum = GlEnum(0x8FE1);
pub const GL_INT8_VEC3_NV: GlEnum = GlEnum(0x8FE2);
pub const GL_INT8_VEC4_NV: GlEnum = GlEnum(0x8FE3);
pub const GL_INT16_NV: GlEnum = GlEnum(0x8FE4);
pub const GL_INT16_VEC2_NV: GlEnum = GlEnum(0x8FE5);
pub const GL_INT16_VEC3_NV: GlEnum = GlEnum(0x8FE6);
pub const GL_INT16_VEC4_NV: GlEnum = GlEnum(0x8FE7);
/// 
/// * Group: [`AttributeType`]
pub const GL_INT64_VEC2_ARB: GlEnum = GlEnum(0x8FE9);
pub const GL_INT64_VEC2_NV: GlEnum = GlEnum(0x8FE9);
/// 
/// * Group: [`AttributeType`]
pub const GL_INT64_VEC3_ARB: GlEnum = GlEnum(0x8FEA);
pub const GL_INT64_VEC3_NV: GlEnum = GlEnum(0x8FEA);
/// 
/// * Group: [`AttributeType`]
pub const GL_INT64_VEC4_ARB: GlEnum = GlEnum(0x8FEB);
pub const GL_INT64_VEC4_NV: GlEnum = GlEnum(0x8FEB);
pub const GL_UNSIGNED_INT8_NV: GlEnum = GlEnum(0x8FEC);
pub const GL_UNSIGNED_INT8_VEC2_NV: GlEnum = GlEnum(0x8FED);
pub const GL_UNSIGNED_INT8_VEC3_NV: GlEnum = GlEnum(0x8FEE);
pub const GL_UNSIGNED_INT8_VEC4_NV: GlEnum = GlEnum(0x8FEF);
pub const GL_UNSIGNED_INT16_NV: GlEnum = GlEnum(0x8FF0);
pub const GL_UNSIGNED_INT16_VEC2_NV: GlEnum = GlEnum(0x8FF1);
pub const GL_UNSIGNED_INT16_VEC3_NV: GlEnum = GlEnum(0x8FF2);
pub const GL_UNSIGNED_INT16_VEC4_NV: GlEnum = GlEnum(0x8FF3);
/// 
/// * Group: [`AttributeType`]
pub const GL_UNSIGNED_INT64_VEC2_ARB: GlEnum = GlEnum(0x8FF5);
pub const GL_UNSIGNED_INT64_VEC2_NV: GlEnum = GlEnum(0x8FF5);
/// 
/// * Group: [`AttributeType`]
pub const GL_UNSIGNED_INT64_VEC3_ARB: GlEnum = GlEnum(0x8FF6);
pub const GL_UNSIGNED_INT64_VEC3_NV: GlEnum = GlEnum(0x8FF6);
/// 
/// * Group: [`AttributeType`]
pub const GL_UNSIGNED_INT64_VEC4_ARB: GlEnum = GlEnum(0x8FF7);
pub const GL_UNSIGNED_INT64_VEC4_NV: GlEnum = GlEnum(0x8FF7);
pub const GL_FLOAT16_NV: GlEnum = GlEnum(0x8FF8);
pub const GL_FLOAT16_VEC2_NV: GlEnum = GlEnum(0x8FF9);
pub const GL_FLOAT16_VEC3_NV: GlEnum = GlEnum(0x8FFA);
pub const GL_FLOAT16_VEC4_NV: GlEnum = GlEnum(0x8FFB);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_VEC2: GlEnum = GlEnum(0x8FFC);
pub const GL_DOUBLE_VEC2_EXT: GlEnum = GlEnum(0x8FFC);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_VEC3: GlEnum = GlEnum(0x8FFD);
pub const GL_DOUBLE_VEC3_EXT: GlEnum = GlEnum(0x8FFD);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_VEC4: GlEnum = GlEnum(0x8FFE);
pub const GL_DOUBLE_VEC4_EXT: GlEnum = GlEnum(0x8FFE);
pub const GL_SAMPLER_BUFFER_AMD: GlEnum = GlEnum(0x9001);
pub const GL_INT_SAMPLER_BUFFER_AMD: GlEnum = GlEnum(0x9002);
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER_AMD: GlEnum = GlEnum(0x9003);
pub const GL_TESSELLATION_MODE_AMD: GlEnum = GlEnum(0x9004);
pub const GL_TESSELLATION_FACTOR_AMD: GlEnum = GlEnum(0x9005);
pub const GL_DISCRETE_AMD: GlEnum = GlEnum(0x9006);
pub const GL_CONTINUOUS_AMD: GlEnum = GlEnum(0x9007);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x9009);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_ARRAY_ARB: GlEnum = GlEnum(0x9009);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_ARRAY_EXT: GlEnum = GlEnum(0x9009);
/// 
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_ARRAY_OES: GlEnum = GlEnum(0x9009);
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x900A);
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_ARB: GlEnum = GlEnum(0x900A);
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_EXT: GlEnum = GlEnum(0x900A);
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_OES: GlEnum = GlEnum(0x900A);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x900B);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY_ARB: GlEnum = GlEnum(0x900B);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x900C);
pub const GL_SAMPLER_CUBE_MAP_ARRAY_ARB: GlEnum = GlEnum(0x900C);
pub const GL_SAMPLER_CUBE_MAP_ARRAY_EXT: GlEnum = GlEnum(0x900C);
pub const GL_SAMPLER_CUBE_MAP_ARRAY_OES: GlEnum = GlEnum(0x900C);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GlEnum = GlEnum(0x900D);
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_ARB: GlEnum = GlEnum(0x900D);
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_EXT: GlEnum = GlEnum(0x900D);
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_OES: GlEnum = GlEnum(0x900D);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x900E);
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_ARB: GlEnum = GlEnum(0x900E);
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_EXT: GlEnum = GlEnum(0x900E);
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_OES: GlEnum = GlEnum(0x900E);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x900F);
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_ARB: GlEnum = GlEnum(0x900F);
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_EXT: GlEnum = GlEnum(0x900F);
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_OES: GlEnum = GlEnum(0x900F);
pub const GL_ALPHA_SNORM: GlEnum = GlEnum(0x9010);
pub const GL_LUMINANCE_SNORM: GlEnum = GlEnum(0x9011);
pub const GL_LUMINANCE_ALPHA_SNORM: GlEnum = GlEnum(0x9012);
pub const GL_INTENSITY_SNORM: GlEnum = GlEnum(0x9013);
pub const GL_ALPHA8_SNORM: GlEnum = GlEnum(0x9014);
pub const GL_LUMINANCE8_SNORM: GlEnum = GlEnum(0x9015);
pub const GL_LUMINANCE8_ALPHA8_SNORM: GlEnum = GlEnum(0x9016);
pub const GL_INTENSITY8_SNORM: GlEnum = GlEnum(0x9017);
pub const GL_ALPHA16_SNORM: GlEnum = GlEnum(0x9018);
pub const GL_LUMINANCE16_SNORM: GlEnum = GlEnum(0x9019);
pub const GL_LUMINANCE16_ALPHA16_SNORM: GlEnum = GlEnum(0x901A);
pub const GL_INTENSITY16_SNORM: GlEnum = GlEnum(0x901B);
pub const GL_FACTOR_MIN_AMD: GlEnum = GlEnum(0x901C);
pub const GL_FACTOR_MAX_AMD: GlEnum = GlEnum(0x901D);
pub const GL_DEPTH_CLAMP_NEAR_AMD: GlEnum = GlEnum(0x901E);
pub const GL_DEPTH_CLAMP_FAR_AMD: GlEnum = GlEnum(0x901F);
pub const GL_VIDEO_BUFFER_NV: GlEnum = GlEnum(0x9020);
pub const GL_VIDEO_BUFFER_BINDING_NV: GlEnum = GlEnum(0x9021);
pub const GL_FIELD_UPPER_NV: GlEnum = GlEnum(0x9022);
pub const GL_FIELD_LOWER_NV: GlEnum = GlEnum(0x9023);
pub const GL_NUM_VIDEO_CAPTURE_STREAMS_NV: GlEnum = GlEnum(0x9024);
pub const GL_NEXT_VIDEO_CAPTURE_BUFFER_STATUS_NV: GlEnum = GlEnum(0x9025);
pub const GL_VIDEO_CAPTURE_TO_422_SUPPORTED_NV: GlEnum = GlEnum(0x9026);
pub const GL_LAST_VIDEO_CAPTURE_STATUS_NV: GlEnum = GlEnum(0x9027);
pub const GL_VIDEO_BUFFER_PITCH_NV: GlEnum = GlEnum(0x9028);
pub const GL_VIDEO_COLOR_CONVERSION_MATRIX_NV: GlEnum = GlEnum(0x9029);
pub const GL_VIDEO_COLOR_CONVERSION_MAX_NV: GlEnum = GlEnum(0x902A);
pub const GL_VIDEO_COLOR_CONVERSION_MIN_NV: GlEnum = GlEnum(0x902B);
pub const GL_VIDEO_COLOR_CONVERSION_OFFSET_NV: GlEnum = GlEnum(0x902C);
pub const GL_VIDEO_BUFFER_INTERNAL_FORMAT_NV: GlEnum = GlEnum(0x902D);
pub const GL_PARTIAL_SUCCESS_NV: GlEnum = GlEnum(0x902E);
pub const GL_SUCCESS_NV: GlEnum = GlEnum(0x902F);
pub const GL_FAILURE_NV: GlEnum = GlEnum(0x9030);
pub const GL_YCBYCR8_422_NV: GlEnum = GlEnum(0x9031);
pub const GL_YCBAYCR8A_4224_NV: GlEnum = GlEnum(0x9032);
pub const GL_Z6Y10Z6CB10Z6Y10Z6CR10_422_NV: GlEnum = GlEnum(0x9033);
pub const GL_Z6Y10Z6CB10Z6A10Z6Y10Z6CR10Z6A10_4224_NV: GlEnum = GlEnum(0x9034);
pub const GL_Z4Y12Z4CB12Z4Y12Z4CR12_422_NV: GlEnum = GlEnum(0x9035);
pub const GL_Z4Y12Z4CB12Z4A12Z4Y12Z4CR12Z4A12_4224_NV: GlEnum = GlEnum(0x9036);
pub const GL_Z4Y12Z4CB12Z4CR12_444_NV: GlEnum = GlEnum(0x9037);
pub const GL_VIDEO_CAPTURE_FRAME_WIDTH_NV: GlEnum = GlEnum(0x9038);
pub const GL_VIDEO_CAPTURE_FRAME_HEIGHT_NV: GlEnum = GlEnum(0x9039);
pub const GL_VIDEO_CAPTURE_FIELD_UPPER_HEIGHT_NV: GlEnum = GlEnum(0x903A);
pub const GL_VIDEO_CAPTURE_FIELD_LOWER_HEIGHT_NV: GlEnum = GlEnum(0x903B);
pub const GL_VIDEO_CAPTURE_SURFACE_ORIGIN_NV: GlEnum = GlEnum(0x903C);
pub const GL_TEXTURE_COVERAGE_SAMPLES_NV: GlEnum = GlEnum(0x9045);
pub const GL_TEXTURE_COLOR_SAMPLES_NV: GlEnum = GlEnum(0x9046);
pub const GL_GPU_MEMORY_INFO_DEDICATED_VIDMEM_NVX: GlEnum = GlEnum(0x9047);
pub const GL_GPU_MEMORY_INFO_TOTAL_AVAILABLE_MEMORY_NVX: GlEnum = GlEnum(0x9048);
pub const GL_GPU_MEMORY_INFO_CURRENT_AVAILABLE_VIDMEM_NVX: GlEnum = GlEnum(0x9049);
pub const GL_GPU_MEMORY_INFO_EVICTION_COUNT_NVX: GlEnum = GlEnum(0x904A);
pub const GL_GPU_MEMORY_INFO_EVICTED_MEMORY_NVX: GlEnum = GlEnum(0x904B);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_1D: GlEnum = GlEnum(0x904C);
pub const GL_IMAGE_1D_EXT: GlEnum = GlEnum(0x904C);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D: GlEnum = GlEnum(0x904D);
pub const GL_IMAGE_2D_EXT: GlEnum = GlEnum(0x904D);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_3D: GlEnum = GlEnum(0x904E);
pub const GL_IMAGE_3D_EXT: GlEnum = GlEnum(0x904E);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D_RECT: GlEnum = GlEnum(0x904F);
pub const GL_IMAGE_2D_RECT_EXT: GlEnum = GlEnum(0x904F);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_CUBE: GlEnum = GlEnum(0x9050);
pub const GL_IMAGE_CUBE_EXT: GlEnum = GlEnum(0x9050);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_BUFFER: GlEnum = GlEnum(0x9051);
pub const GL_IMAGE_BUFFER_EXT: GlEnum = GlEnum(0x9051);
pub const GL_IMAGE_BUFFER_OES: GlEnum = GlEnum(0x9051);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_1D_ARRAY: GlEnum = GlEnum(0x9052);
pub const GL_IMAGE_1D_ARRAY_EXT: GlEnum = GlEnum(0x9052);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D_ARRAY: GlEnum = GlEnum(0x9053);
pub const GL_IMAGE_2D_ARRAY_EXT: GlEnum = GlEnum(0x9053);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x9054);
pub const GL_IMAGE_CUBE_MAP_ARRAY_EXT: GlEnum = GlEnum(0x9054);
pub const GL_IMAGE_CUBE_MAP_ARRAY_OES: GlEnum = GlEnum(0x9054);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D_MULTISAMPLE: GlEnum = GlEnum(0x9055);
pub const GL_IMAGE_2D_MULTISAMPLE_EXT: GlEnum = GlEnum(0x9055);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x9056);
pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: GlEnum = GlEnum(0x9056);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_1D: GlEnum = GlEnum(0x9057);
pub const GL_INT_IMAGE_1D_EXT: GlEnum = GlEnum(0x9057);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D: GlEnum = GlEnum(0x9058);
pub const GL_INT_IMAGE_2D_EXT: GlEnum = GlEnum(0x9058);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_3D: GlEnum = GlEnum(0x9059);
pub const GL_INT_IMAGE_3D_EXT: GlEnum = GlEnum(0x9059);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D_RECT: GlEnum = GlEnum(0x905A);
pub const GL_INT_IMAGE_2D_RECT_EXT: GlEnum = GlEnum(0x905A);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_CUBE: GlEnum = GlEnum(0x905B);
pub const GL_INT_IMAGE_CUBE_EXT: GlEnum = GlEnum(0x905B);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_BUFFER: GlEnum = GlEnum(0x905C);
pub const GL_INT_IMAGE_BUFFER_EXT: GlEnum = GlEnum(0x905C);
pub const GL_INT_IMAGE_BUFFER_OES: GlEnum = GlEnum(0x905C);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_1D_ARRAY: GlEnum = GlEnum(0x905D);
pub const GL_INT_IMAGE_1D_ARRAY_EXT: GlEnum = GlEnum(0x905D);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D_ARRAY: GlEnum = GlEnum(0x905E);
pub const GL_INT_IMAGE_2D_ARRAY_EXT: GlEnum = GlEnum(0x905E);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x905F);
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY_EXT: GlEnum = GlEnum(0x905F);
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY_OES: GlEnum = GlEnum(0x905F);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D_MULTISAMPLE: GlEnum = GlEnum(0x9060);
pub const GL_INT_IMAGE_2D_MULTISAMPLE_EXT: GlEnum = GlEnum(0x9060);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x9061);
pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: GlEnum = GlEnum(0x9061);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_1D: GlEnum = GlEnum(0x9062);
pub const GL_UNSIGNED_INT_IMAGE_1D_EXT: GlEnum = GlEnum(0x9062);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D: GlEnum = GlEnum(0x9063);
pub const GL_UNSIGNED_INT_IMAGE_2D_EXT: GlEnum = GlEnum(0x9063);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_3D: GlEnum = GlEnum(0x9064);
pub const GL_UNSIGNED_INT_IMAGE_3D_EXT: GlEnum = GlEnum(0x9064);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: GlEnum = GlEnum(0x9065);
pub const GL_UNSIGNED_INT_IMAGE_2D_RECT_EXT: GlEnum = GlEnum(0x9065);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_CUBE: GlEnum = GlEnum(0x9066);
pub const GL_UNSIGNED_INT_IMAGE_CUBE_EXT: GlEnum = GlEnum(0x9066);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_BUFFER: GlEnum = GlEnum(0x9067);
pub const GL_UNSIGNED_INT_IMAGE_BUFFER_EXT: GlEnum = GlEnum(0x9067);
pub const GL_UNSIGNED_INT_IMAGE_BUFFER_OES: GlEnum = GlEnum(0x9067);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: GlEnum = GlEnum(0x9068);
pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY_EXT: GlEnum = GlEnum(0x9068);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GlEnum = GlEnum(0x9069);
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY_EXT: GlEnum = GlEnum(0x9069);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GlEnum = GlEnum(0x906A);
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_EXT: GlEnum = GlEnum(0x906A);
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_OES: GlEnum = GlEnum(0x906A);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GlEnum = GlEnum(0x906B);
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_EXT: GlEnum = GlEnum(0x906B);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x906C);
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: GlEnum = GlEnum(0x906C);
pub const GL_MAX_IMAGE_SAMPLES: GlEnum = GlEnum(0x906D);
pub const GL_MAX_IMAGE_SAMPLES_EXT: GlEnum = GlEnum(0x906D);
pub const GL_IMAGE_BINDING_FORMAT: GlEnum = GlEnum(0x906E);
pub const GL_IMAGE_BINDING_FORMAT_EXT: GlEnum = GlEnum(0x906E);
/// 
/// * Group: [`InternalFormat`]
pub const GL_RGB10_A2UI: GlEnum = GlEnum(0x906F);
/// 
/// * Group: [`PathStringFormat`]
pub const GL_PATH_FORMAT_SVG_NV: GlEnum = GlEnum(0x9070);
/// 
/// * Group: [`PathStringFormat`]
pub const GL_PATH_FORMAT_PS_NV: GlEnum = GlEnum(0x9071);
/// 
/// * Group: [`PathFontTarget`]
pub const GL_STANDARD_FONT_NAME_NV: GlEnum = GlEnum(0x9072);
/// 
/// * Group: [`PathFontTarget`]
pub const GL_SYSTEM_FONT_NAME_NV: GlEnum = GlEnum(0x9073);
/// 
/// * Group: [`PathFontTarget`]
pub const GL_FILE_NAME_NV: GlEnum = GlEnum(0x9074);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_STROKE_WIDTH_NV: GlEnum = GlEnum(0x9075);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_END_CAPS_NV: GlEnum = GlEnum(0x9076);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_INITIAL_END_CAP_NV: GlEnum = GlEnum(0x9077);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_TERMINAL_END_CAP_NV: GlEnum = GlEnum(0x9078);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_JOIN_STYLE_NV: GlEnum = GlEnum(0x9079);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_MITER_LIMIT_NV: GlEnum = GlEnum(0x907A);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_DASH_CAPS_NV: GlEnum = GlEnum(0x907B);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_INITIAL_DASH_CAP_NV: GlEnum = GlEnum(0x907C);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_TERMINAL_DASH_CAP_NV: GlEnum = GlEnum(0x907D);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_DASH_OFFSET_NV: GlEnum = GlEnum(0x907E);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_CLIENT_LENGTH_NV: GlEnum = GlEnum(0x907F);
/// 
/// * Group: [`PathParameter`], [`PathFillMode`]
pub const GL_PATH_FILL_MODE_NV: GlEnum = GlEnum(0x9080);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_FILL_MASK_NV: GlEnum = GlEnum(0x9081);
/// 
/// * Group: [`PathCoverMode`], [`PathParameter`]
pub const GL_PATH_FILL_COVER_MODE_NV: GlEnum = GlEnum(0x9082);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_STROKE_COVER_MODE_NV: GlEnum = GlEnum(0x9083);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_STROKE_MASK_NV: GlEnum = GlEnum(0x9084);
/// 
/// * Group: [`PathFillMode`]
pub const GL_COUNT_UP_NV: GlEnum = GlEnum(0x9088);
/// 
/// * Group: [`PathFillMode`]
pub const GL_COUNT_DOWN_NV: GlEnum = GlEnum(0x9089);
/// 
/// * Group: [`PathGenMode`], [`PathParameter`]
pub const GL_PATH_OBJECT_BOUNDING_BOX_NV: GlEnum = GlEnum(0x908A);
/// 
/// * Group: [`PathCoverMode`]
pub const GL_CONVEX_HULL_NV: GlEnum = GlEnum(0x908B);
/// 
/// * Group: [`PathCoverMode`]
pub const GL_BOUNDING_BOX_NV: GlEnum = GlEnum(0x908D);
/// 
/// * Group: [`PathTransformType`]
pub const GL_TRANSLATE_X_NV: GlEnum = GlEnum(0x908E);
/// 
/// * Group: [`PathTransformType`]
pub const GL_TRANSLATE_Y_NV: GlEnum = GlEnum(0x908F);
/// 
/// * Group: [`PathTransformType`]
pub const GL_TRANSLATE_2D_NV: GlEnum = GlEnum(0x9090);
/// 
/// * Group: [`PathTransformType`]
pub const GL_TRANSLATE_3D_NV: GlEnum = GlEnum(0x9091);
/// 
/// * Group: [`PathTransformType`]
pub const GL_AFFINE_2D_NV: GlEnum = GlEnum(0x9092);
/// 
/// * Group: [`PathTransformType`]
pub const GL_AFFINE_3D_NV: GlEnum = GlEnum(0x9094);
/// 
/// * Group: [`PathTransformType`]
pub const GL_TRANSPOSE_AFFINE_2D_NV: GlEnum = GlEnum(0x9096);
/// 
/// * Group: [`PathTransformType`]
pub const GL_TRANSPOSE_AFFINE_3D_NV: GlEnum = GlEnum(0x9098);
/// 
/// * Group: [`PathElementType`]
pub const GL_UTF8_NV: GlEnum = GlEnum(0x909A);
/// 
/// * Group: [`PathElementType`]
pub const GL_UTF16_NV: GlEnum = GlEnum(0x909B);
/// 
/// * Group: [`PathCoverMode`]
pub const GL_BOUNDING_BOX_OF_BOUNDING_BOXES_NV: GlEnum = GlEnum(0x909C);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_COMMAND_COUNT_NV: GlEnum = GlEnum(0x909D);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_COORD_COUNT_NV: GlEnum = GlEnum(0x909E);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_DASH_ARRAY_COUNT_NV: GlEnum = GlEnum(0x909F);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_COMPUTED_LENGTH_NV: GlEnum = GlEnum(0x90A0);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_FILL_BOUNDING_BOX_NV: GlEnum = GlEnum(0x90A1);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_STROKE_BOUNDING_BOX_NV: GlEnum = GlEnum(0x90A2);
pub const GL_SQUARE_NV: GlEnum = GlEnum(0x90A3);
pub const GL_ROUND_NV: GlEnum = GlEnum(0x90A4);
pub const GL_TRIANGULAR_NV: GlEnum = GlEnum(0x90A5);
pub const GL_BEVEL_NV: GlEnum = GlEnum(0x90A6);
pub const GL_MITER_REVERT_NV: GlEnum = GlEnum(0x90A7);
pub const GL_MITER_TRUNCATE_NV: GlEnum = GlEnum(0x90A8);
/// 
/// * Group: [`PathHandleMissingGlyphs`]
pub const GL_SKIP_MISSING_GLYPH_NV: GlEnum = GlEnum(0x90A9);
/// 
/// * Group: [`PathHandleMissingGlyphs`]
pub const GL_USE_MISSING_GLYPH_NV: GlEnum = GlEnum(0x90AA);
pub const GL_PATH_ERROR_POSITION_NV: GlEnum = GlEnum(0x90AB);
pub const GL_PATH_FOG_GEN_MODE_NV: GlEnum = GlEnum(0x90AC);
/// 
/// * Group: [`PathListMode`]
pub const GL_ACCUM_ADJACENT_PAIRS_NV: GlEnum = GlEnum(0x90AD);
/// 
/// * Group: [`PathListMode`]
pub const GL_ADJACENT_PAIRS_NV: GlEnum = GlEnum(0x90AE);
/// 
/// * Group: [`PathListMode`]
pub const GL_FIRST_TO_REST_NV: GlEnum = GlEnum(0x90AF);
pub const GL_PATH_GEN_MODE_NV: GlEnum = GlEnum(0x90B0);
pub const GL_PATH_GEN_COEFF_NV: GlEnum = GlEnum(0x90B1);
pub const GL_PATH_GEN_COLOR_FORMAT_NV: GlEnum = GlEnum(0x90B2);
pub const GL_PATH_GEN_COMPONENTS_NV: GlEnum = GlEnum(0x90B3);
/// 
/// * Group: [`PathParameter`]
pub const GL_PATH_DASH_OFFSET_RESET_NV: GlEnum = GlEnum(0x90B4);
pub const GL_MOVE_TO_RESETS_NV: GlEnum = GlEnum(0x90B5);
pub const GL_MOVE_TO_CONTINUES_NV: GlEnum = GlEnum(0x90B6);
pub const GL_PATH_STENCIL_FUNC_NV: GlEnum = GlEnum(0x90B7);
pub const GL_PATH_STENCIL_REF_NV: GlEnum = GlEnum(0x90B8);
pub const GL_PATH_STENCIL_VALUE_MASK_NV: GlEnum = GlEnum(0x90B9);
pub const GL_SCALED_RESOLVE_FASTEST_EXT: GlEnum = GlEnum(0x90BA);
pub const GL_SCALED_RESOLVE_NICEST_EXT: GlEnum = GlEnum(0x90BB);
/// 
/// * Group: [`GetPName`]
pub const GL_MIN_MAP_BUFFER_ALIGNMENT: GlEnum = GlEnum(0x90BC);
pub const GL_PATH_STENCIL_DEPTH_OFFSET_FACTOR_NV: GlEnum = GlEnum(0x90BD);
pub const GL_PATH_STENCIL_DEPTH_OFFSET_UNITS_NV: GlEnum = GlEnum(0x90BE);
pub const GL_PATH_COVER_DEPTH_FUNC_NV: GlEnum = GlEnum(0x90BF);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GlEnum = GlEnum(0x90C7);
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GlEnum = GlEnum(0x90C8);
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GlEnum = GlEnum(0x90C9);
pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: GlEnum = GlEnum(0x90CA);
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GlEnum = GlEnum(0x90CB);
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS_EXT: GlEnum = GlEnum(0x90CB);
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS_OES: GlEnum = GlEnum(0x90CB);
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GlEnum = GlEnum(0x90CC);
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS_EXT: GlEnum = GlEnum(0x90CC);
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS_OES: GlEnum = GlEnum(0x90CC);
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GlEnum = GlEnum(0x90CD);
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS_EXT: GlEnum = GlEnum(0x90CD);
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS_OES: GlEnum = GlEnum(0x90CD);
pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GlEnum = GlEnum(0x90CE);
pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: GlEnum = GlEnum(0x90CF);
pub const GL_MAX_DEEP_3D_TEXTURE_WIDTH_HEIGHT_NV: GlEnum = GlEnum(0x90D0);
pub const GL_MAX_DEEP_3D_TEXTURE_DEPTH_NV: GlEnum = GlEnum(0x90D1);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_SHADER_STORAGE_BUFFER: GlEnum = GlEnum(0x90D2);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADER_STORAGE_BUFFER_BINDING: GlEnum = GlEnum(0x90D3);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADER_STORAGE_BUFFER_START: GlEnum = GlEnum(0x90D4);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADER_STORAGE_BUFFER_SIZE: GlEnum = GlEnum(0x90D5);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GlEnum = GlEnum(0x90D6);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GlEnum = GlEnum(0x90D7);
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS_EXT: GlEnum = GlEnum(0x90D7);
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS_OES: GlEnum = GlEnum(0x90D7);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GlEnum = GlEnum(0x90D8);
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS_EXT: GlEnum = GlEnum(0x90D8);
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS_OES: GlEnum = GlEnum(0x90D8);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GlEnum = GlEnum(0x90D9);
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS_EXT: GlEnum = GlEnum(0x90D9);
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS_OES: GlEnum = GlEnum(0x90D9);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GlEnum = GlEnum(0x90DA);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GlEnum = GlEnum(0x90DB);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GlEnum = GlEnum(0x90DC);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GlEnum = GlEnum(0x90DD);
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GlEnum = GlEnum(0x90DE);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GlEnum = GlEnum(0x90DF);
pub const GL_SYNC_X11_FENCE_EXT: GlEnum = GlEnum(0x90E1);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_DEPTH_STENCIL_TEXTURE_MODE: GlEnum = GlEnum(0x90EA);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GlEnum = GlEnum(0x90EB);
/// 
/// * Alias Of: [`GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS`]
pub const GL_MAX_COMPUTE_FIXED_GROUP_INVOCATIONS_ARB: GlEnum = GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS;
/// 
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GlEnum = GlEnum(0x90EC);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GlEnum = GlEnum(0x90ED);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_DISPATCH_INDIRECT_BUFFER: GlEnum = GlEnum(0x90EE);
/// 
/// * Group: [`GetPName`]
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GlEnum = GlEnum(0x90EF);
pub const GL_COLOR_ATTACHMENT_EXT: GlEnum = GlEnum(0x90F0);
pub const GL_MULTIVIEW_EXT: GlEnum = GlEnum(0x90F1);
pub const GL_MAX_MULTIVIEW_BUFFERS_EXT: GlEnum = GlEnum(0x90F2);
pub const GL_CONTEXT_ROBUST_ACCESS: GlEnum = GlEnum(0x90F3);
pub const GL_CONTEXT_ROBUST_ACCESS_EXT: GlEnum = GlEnum(0x90F3);
pub const GL_CONTEXT_ROBUST_ACCESS_KHR: GlEnum = GlEnum(0x90F3);
/// 
/// * Group: [`ProgramTarget`]
pub const GL_COMPUTE_PROGRAM_NV: GlEnum = GlEnum(0x90FB);
pub const GL_COMPUTE_PROGRAM_PARAMETER_BUFFER_NV: GlEnum = GlEnum(0x90FC);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_2D_MULTISAMPLE: GlEnum = GlEnum(0x9100);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: GlEnum = GlEnum(0x9101);
/// 
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x9102);
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY_OES: GlEnum = GlEnum(0x9102);
/// 
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x9103);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GlEnum = GlEnum(0x9104);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x9105);
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY_OES: GlEnum = GlEnum(0x9105);
pub const GL_TEXTURE_SAMPLES: GlEnum = GlEnum(0x9106);
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GlEnum = GlEnum(0x9107);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_MULTISAMPLE: GlEnum = GlEnum(0x9108);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GlEnum = GlEnum(0x9109);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GlEnum = GlEnum(0x910A);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x910B);
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GlEnum = GlEnum(0x910B);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x910C);
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GlEnum = GlEnum(0x910C);
/// 
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GlEnum = GlEnum(0x910D);
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GlEnum = GlEnum(0x910D);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GlEnum = GlEnum(0x910E);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GlEnum = GlEnum(0x910F);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_INTEGER_SAMPLES: GlEnum = GlEnum(0x9110);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_SERVER_WAIT_TIMEOUT: GlEnum = GlEnum(0x9111);
pub const GL_MAX_SERVER_WAIT_TIMEOUT_APPLE: GlEnum = GlEnum(0x9111);
/// 
/// * Group: [`SyncParameterName`]
pub const GL_OBJECT_TYPE: GlEnum = GlEnum(0x9112);
pub const GL_OBJECT_TYPE_APPLE: GlEnum = GlEnum(0x9112);
/// 
/// * Group: [`SyncParameterName`]
pub const GL_SYNC_CONDITION: GlEnum = GlEnum(0x9113);
pub const GL_SYNC_CONDITION_APPLE: GlEnum = GlEnum(0x9113);
/// 
/// * Group: [`SyncParameterName`]
pub const GL_SYNC_STATUS: GlEnum = GlEnum(0x9114);
pub const GL_SYNC_STATUS_APPLE: GlEnum = GlEnum(0x9114);
/// 
/// * Group: [`SyncParameterName`]
pub const GL_SYNC_FLAGS: GlEnum = GlEnum(0x9115);
pub const GL_SYNC_FLAGS_APPLE: GlEnum = GlEnum(0x9115);
pub const GL_SYNC_FENCE: GlEnum = GlEnum(0x9116);
pub const GL_SYNC_FENCE_APPLE: GlEnum = GlEnum(0x9116);
/// 
/// * Group: [`SyncCondition`]
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GlEnum = GlEnum(0x9117);
pub const GL_SYNC_GPU_COMMANDS_COMPLETE_APPLE: GlEnum = GlEnum(0x9117);
pub const GL_UNSIGNALED: GlEnum = GlEnum(0x9118);
pub const GL_UNSIGNALED_APPLE: GlEnum = GlEnum(0x9118);
pub const GL_SIGNALED: GlEnum = GlEnum(0x9119);
pub const GL_SIGNALED_APPLE: GlEnum = GlEnum(0x9119);
/// 
/// * Group: [`SyncStatus`]
pub const GL_ALREADY_SIGNALED: GlEnum = GlEnum(0x911A);
pub const GL_ALREADY_SIGNALED_APPLE: GlEnum = GlEnum(0x911A);
/// 
/// * Group: [`SyncStatus`]
pub const GL_TIMEOUT_EXPIRED: GlEnum = GlEnum(0x911B);
pub const GL_TIMEOUT_EXPIRED_APPLE: GlEnum = GlEnum(0x911B);
/// 
/// * Group: [`SyncStatus`]
pub const GL_CONDITION_SATISFIED: GlEnum = GlEnum(0x911C);
pub const GL_CONDITION_SATISFIED_APPLE: GlEnum = GlEnum(0x911C);
/// 
/// * Group: [`SyncStatus`]
pub const GL_WAIT_FAILED: GlEnum = GlEnum(0x911D);
pub const GL_WAIT_FAILED_APPLE: GlEnum = GlEnum(0x911D);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_ACCESS_FLAGS: GlEnum = GlEnum(0x911F);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_MAP_LENGTH: GlEnum = GlEnum(0x9120);
/// 
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_MAP_OFFSET: GlEnum = GlEnum(0x9121);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GlEnum = GlEnum(0x9122);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GlEnum = GlEnum(0x9123);
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS_EXT: GlEnum = GlEnum(0x9123);
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS_OES: GlEnum = GlEnum(0x9123);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GlEnum = GlEnum(0x9124);
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS_EXT: GlEnum = GlEnum(0x9124);
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS_OES: GlEnum = GlEnum(0x9124);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GlEnum = GlEnum(0x9125);
/// 
/// * Group: [`GetPName`]
pub const GL_CONTEXT_PROFILE_MASK: GlEnum = GlEnum(0x9126);
pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH: GlEnum = GlEnum(0x9127);
pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT: GlEnum = GlEnum(0x9128);
pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH: GlEnum = GlEnum(0x9129);
pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE: GlEnum = GlEnum(0x912A);
pub const GL_PACK_COMPRESSED_BLOCK_WIDTH: GlEnum = GlEnum(0x912B);
pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT: GlEnum = GlEnum(0x912C);
pub const GL_PACK_COMPRESSED_BLOCK_DEPTH: GlEnum = GlEnum(0x912D);
pub const GL_PACK_COMPRESSED_BLOCK_SIZE: GlEnum = GlEnum(0x912E);
pub const GL_TEXTURE_IMMUTABLE_FORMAT: GlEnum = GlEnum(0x912F);
pub const GL_TEXTURE_IMMUTABLE_FORMAT_EXT: GlEnum = GlEnum(0x912F);
pub const GL_SGX_PROGRAM_BINARY_IMG: GlEnum = GlEnum(0x9130);
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_IMG: GlEnum = GlEnum(0x9133);
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_IMG: GlEnum = GlEnum(0x9134);
pub const GL_MAX_SAMPLES_IMG: GlEnum = GlEnum(0x9135);
pub const GL_TEXTURE_SAMPLES_IMG: GlEnum = GlEnum(0x9136);
pub const GL_COMPRESSED_RGBA_PVRTC_2BPPV2_IMG: GlEnum = GlEnum(0x9137);
pub const GL_COMPRESSED_RGBA_PVRTC_4BPPV2_IMG: GlEnum = GlEnum(0x9138);
pub const GL_CUBIC_IMG: GlEnum = GlEnum(0x9139);
pub const GL_CUBIC_MIPMAP_NEAREST_IMG: GlEnum = GlEnum(0x913A);
pub const GL_CUBIC_MIPMAP_LINEAR_IMG: GlEnum = GlEnum(0x913B);
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_AND_DOWNSAMPLE_IMG: GlEnum = GlEnum(0x913C);
pub const GL_NUM_DOWNSAMPLE_SCALES_IMG: GlEnum = GlEnum(0x913D);
pub const GL_DOWNSAMPLE_SCALES_IMG: GlEnum = GlEnum(0x913E);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SCALE_IMG: GlEnum = GlEnum(0x913F);
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GlEnum = GlEnum(0x9143);
pub const GL_MAX_DEBUG_MESSAGE_LENGTH_AMD: GlEnum = GlEnum(0x9143);
pub const GL_MAX_DEBUG_MESSAGE_LENGTH_ARB: GlEnum = GlEnum(0x9143);
pub const GL_MAX_DEBUG_MESSAGE_LENGTH_KHR: GlEnum = GlEnum(0x9143);
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GlEnum = GlEnum(0x9144);
pub const GL_MAX_DEBUG_LOGGED_MESSAGES_AMD: GlEnum = GlEnum(0x9144);
pub const GL_MAX_DEBUG_LOGGED_MESSAGES_ARB: GlEnum = GlEnum(0x9144);
pub const GL_MAX_DEBUG_LOGGED_MESSAGES_KHR: GlEnum = GlEnum(0x9144);
pub const GL_DEBUG_LOGGED_MESSAGES: GlEnum = GlEnum(0x9145);
pub const GL_DEBUG_LOGGED_MESSAGES_AMD: GlEnum = GlEnum(0x9145);
pub const GL_DEBUG_LOGGED_MESSAGES_ARB: GlEnum = GlEnum(0x9145);
pub const GL_DEBUG_LOGGED_MESSAGES_KHR: GlEnum = GlEnum(0x9145);
/// 
/// * Group: [`DebugSeverity`]
pub const GL_DEBUG_SEVERITY_HIGH: GlEnum = GlEnum(0x9146);
pub const GL_DEBUG_SEVERITY_HIGH_AMD: GlEnum = GlEnum(0x9146);
pub const GL_DEBUG_SEVERITY_HIGH_ARB: GlEnum = GlEnum(0x9146);
pub const GL_DEBUG_SEVERITY_HIGH_KHR: GlEnum = GlEnum(0x9146);
/// 
/// * Group: [`DebugSeverity`]
pub const GL_DEBUG_SEVERITY_MEDIUM: GlEnum = GlEnum(0x9147);
pub const GL_DEBUG_SEVERITY_MEDIUM_AMD: GlEnum = GlEnum(0x9147);
pub const GL_DEBUG_SEVERITY_MEDIUM_ARB: GlEnum = GlEnum(0x9147);
pub const GL_DEBUG_SEVERITY_MEDIUM_KHR: GlEnum = GlEnum(0x9147);
/// 
/// * Group: [`DebugSeverity`]
pub const GL_DEBUG_SEVERITY_LOW: GlEnum = GlEnum(0x9148);
pub const GL_DEBUG_SEVERITY_LOW_AMD: GlEnum = GlEnum(0x9148);
pub const GL_DEBUG_SEVERITY_LOW_ARB: GlEnum = GlEnum(0x9148);
pub const GL_DEBUG_SEVERITY_LOW_KHR: GlEnum = GlEnum(0x9148);
pub const GL_DEBUG_CATEGORY_API_ERROR_AMD: GlEnum = GlEnum(0x9149);
pub const GL_DEBUG_CATEGORY_WINDOW_SYSTEM_AMD: GlEnum = GlEnum(0x914A);
pub const GL_DEBUG_CATEGORY_DEPRECATION_AMD: GlEnum = GlEnum(0x914B);
pub const GL_DEBUG_CATEGORY_UNDEFINED_BEHAVIOR_AMD: GlEnum = GlEnum(0x914C);
pub const GL_DEBUG_CATEGORY_PERFORMANCE_AMD: GlEnum = GlEnum(0x914D);
pub const GL_DEBUG_CATEGORY_SHADER_COMPILER_AMD: GlEnum = GlEnum(0x914E);
pub const GL_DEBUG_CATEGORY_APPLICATION_AMD: GlEnum = GlEnum(0x914F);
pub const GL_DEBUG_CATEGORY_OTHER_AMD: GlEnum = GlEnum(0x9150);
pub const GL_BUFFER_OBJECT_EXT: GlEnum = GlEnum(0x9151);
pub const GL_DATA_BUFFER_AMD: GlEnum = GlEnum(0x9151);
pub const GL_PERFORMANCE_MONITOR_AMD: GlEnum = GlEnum(0x9152);
pub const GL_QUERY_OBJECT_AMD: GlEnum = GlEnum(0x9153);
pub const GL_QUERY_OBJECT_EXT: GlEnum = GlEnum(0x9153);
pub const GL_VERTEX_ARRAY_OBJECT_AMD: GlEnum = GlEnum(0x9154);
pub const GL_VERTEX_ARRAY_OBJECT_EXT: GlEnum = GlEnum(0x9154);
pub const GL_SAMPLER_OBJECT_AMD: GlEnum = GlEnum(0x9155);
pub const GL_EXTERNAL_VIRTUAL_MEMORY_BUFFER_AMD: GlEnum = GlEnum(0x9160);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_QUERY_BUFFER: GlEnum = GlEnum(0x9192);
pub const GL_QUERY_BUFFER_AMD: GlEnum = GlEnum(0x9192);
pub const GL_QUERY_BUFFER_BINDING: GlEnum = GlEnum(0x9193);
pub const GL_QUERY_BUFFER_BINDING_AMD: GlEnum = GlEnum(0x9193);
/// 
/// * Group: [`QueryObjectParameterName`]
pub const GL_QUERY_RESULT_NO_WAIT: GlEnum = GlEnum(0x9194);
pub const GL_QUERY_RESULT_NO_WAIT_AMD: GlEnum = GlEnum(0x9194);
pub const GL_VIRTUAL_PAGE_SIZE_X_ARB: GlEnum = GlEnum(0x9195);
pub const GL_VIRTUAL_PAGE_SIZE_X_EXT: GlEnum = GlEnum(0x9195);
pub const GL_VIRTUAL_PAGE_SIZE_X_AMD: GlEnum = GlEnum(0x9195);
pub const GL_VIRTUAL_PAGE_SIZE_Y_ARB: GlEnum = GlEnum(0x9196);
pub const GL_VIRTUAL_PAGE_SIZE_Y_EXT: GlEnum = GlEnum(0x9196);
pub const GL_VIRTUAL_PAGE_SIZE_Y_AMD: GlEnum = GlEnum(0x9196);
pub const GL_VIRTUAL_PAGE_SIZE_Z_ARB: GlEnum = GlEnum(0x9197);
pub const GL_VIRTUAL_PAGE_SIZE_Z_EXT: GlEnum = GlEnum(0x9197);
pub const GL_VIRTUAL_PAGE_SIZE_Z_AMD: GlEnum = GlEnum(0x9197);
pub const GL_MAX_SPARSE_TEXTURE_SIZE_ARB: GlEnum = GlEnum(0x9198);
pub const GL_MAX_SPARSE_TEXTURE_SIZE_EXT: GlEnum = GlEnum(0x9198);
pub const GL_MAX_SPARSE_TEXTURE_SIZE_AMD: GlEnum = GlEnum(0x9198);
pub const GL_MAX_SPARSE_3D_TEXTURE_SIZE_ARB: GlEnum = GlEnum(0x9199);
pub const GL_MAX_SPARSE_3D_TEXTURE_SIZE_EXT: GlEnum = GlEnum(0x9199);
pub const GL_MAX_SPARSE_3D_TEXTURE_SIZE_AMD: GlEnum = GlEnum(0x9199);
pub const GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS: GlEnum = GlEnum(0x919A);
pub const GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS_ARB: GlEnum = GlEnum(0x919A);
pub const GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS_EXT: GlEnum = GlEnum(0x919A);
pub const GL_MIN_SPARSE_LEVEL_AMD: GlEnum = GlEnum(0x919B);
pub const GL_MIN_LOD_WARNING_AMD: GlEnum = GlEnum(0x919C);
pub const GL_TEXTURE_BUFFER_OFFSET: GlEnum = GlEnum(0x919D);
pub const GL_TEXTURE_BUFFER_OFFSET_EXT: GlEnum = GlEnum(0x919D);
pub const GL_TEXTURE_BUFFER_OFFSET_OES: GlEnum = GlEnum(0x919D);
pub const GL_TEXTURE_BUFFER_SIZE: GlEnum = GlEnum(0x919E);
pub const GL_TEXTURE_BUFFER_SIZE_EXT: GlEnum = GlEnum(0x919E);
pub const GL_TEXTURE_BUFFER_SIZE_OES: GlEnum = GlEnum(0x919E);
/// 
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GlEnum = GlEnum(0x919F);
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT_EXT: GlEnum = GlEnum(0x919F);
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT_OES: GlEnum = GlEnum(0x919F);
pub const GL_STREAM_RASTERIZATION_AMD: GlEnum = GlEnum(0x91A0);
pub const GL_VERTEX_ELEMENT_SWIZZLE_AMD: GlEnum = GlEnum(0x91A4);
pub const GL_VERTEX_ID_SWIZZLE_AMD: GlEnum = GlEnum(0x91A5);
pub const GL_TEXTURE_SPARSE_ARB: GlEnum = GlEnum(0x91A6);
pub const GL_TEXTURE_SPARSE_EXT: GlEnum = GlEnum(0x91A6);
pub const GL_VIRTUAL_PAGE_SIZE_INDEX_ARB: GlEnum = GlEnum(0x91A7);
pub const GL_VIRTUAL_PAGE_SIZE_INDEX_EXT: GlEnum = GlEnum(0x91A7);
pub const GL_NUM_VIRTUAL_PAGE_SIZES_ARB: GlEnum = GlEnum(0x91A8);
pub const GL_NUM_VIRTUAL_PAGE_SIZES_EXT: GlEnum = GlEnum(0x91A8);
pub const GL_SPARSE_TEXTURE_FULL_ARRAY_CUBE_MIPMAPS_ARB: GlEnum = GlEnum(0x91A9);
pub const GL_SPARSE_TEXTURE_FULL_ARRAY_CUBE_MIPMAPS_EXT: GlEnum = GlEnum(0x91A9);
pub const GL_NUM_SPARSE_LEVELS_ARB: GlEnum = GlEnum(0x91AA);
pub const GL_NUM_SPARSE_LEVELS_EXT: GlEnum = GlEnum(0x91AA);
pub const GL_PIXELS_PER_SAMPLE_PATTERN_X_AMD: GlEnum = GlEnum(0x91AE);
pub const GL_PIXELS_PER_SAMPLE_PATTERN_Y_AMD: GlEnum = GlEnum(0x91AF);
pub const GL_MAX_SHADER_COMPILER_THREADS_KHR: GlEnum = GlEnum(0x91B0);
/// 
/// * Alias Of: [`GL_MAX_SHADER_COMPILER_THREADS_KHR`]
pub const GL_MAX_SHADER_COMPILER_THREADS_ARB: GlEnum = GL_MAX_SHADER_COMPILER_THREADS_KHR;
pub const GL_COMPLETION_STATUS_KHR: GlEnum = GlEnum(0x91B1);
/// 
/// * Alias Of: [`GL_COMPLETION_STATUS_KHR`]
pub const GL_COMPLETION_STATUS_ARB: GlEnum = GL_COMPLETION_STATUS_KHR;
/// 
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_STORAGE_SAMPLES_AMD: GlEnum = GlEnum(0x91B2);
pub const GL_MAX_COLOR_FRAMEBUFFER_SAMPLES_AMD: GlEnum = GlEnum(0x91B3);
pub const GL_MAX_COLOR_FRAMEBUFFER_STORAGE_SAMPLES_AMD: GlEnum = GlEnum(0x91B4);
pub const GL_MAX_DEPTH_STENCIL_FRAMEBUFFER_SAMPLES_AMD: GlEnum = GlEnum(0x91B5);
pub const GL_NUM_SUPPORTED_MULTISAMPLE_MODES_AMD: GlEnum = GlEnum(0x91B6);
pub const GL_SUPPORTED_MULTISAMPLE_MODES_AMD: GlEnum = GlEnum(0x91B7);
/// 
/// * Group: [`ShaderType`]
pub const GL_COMPUTE_SHADER: GlEnum = GlEnum(0x91B9);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GlEnum = GlEnum(0x91BB);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GlEnum = GlEnum(0x91BC);
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GlEnum = GlEnum(0x91BD);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GlEnum = GlEnum(0x91BE);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GlEnum = GlEnum(0x91BF);
/// 
/// * Alias Of: [`GL_MAX_COMPUTE_WORK_GROUP_SIZE`]
pub const GL_MAX_COMPUTE_FIXED_GROUP_SIZE_ARB: GlEnum = GL_MAX_COMPUTE_WORK_GROUP_SIZE;
pub const GL_FLOAT16_MAT2_AMD: GlEnum = GlEnum(0x91C5);
pub const GL_FLOAT16_MAT3_AMD: GlEnum = GlEnum(0x91C6);
pub const GL_FLOAT16_MAT4_AMD: GlEnum = GlEnum(0x91C7);
pub const GL_FLOAT16_MAT2x3_AMD: GlEnum = GlEnum(0x91C8);
pub const GL_FLOAT16_MAT2x4_AMD: GlEnum = GlEnum(0x91C9);
pub const GL_FLOAT16_MAT3x2_AMD: GlEnum = GlEnum(0x91CA);
pub const GL_FLOAT16_MAT3x4_AMD: GlEnum = GlEnum(0x91CB);
pub const GL_FLOAT16_MAT4x2_AMD: GlEnum = GlEnum(0x91CC);
pub const GL_FLOAT16_MAT4x3_AMD: GlEnum = GlEnum(0x91CD);
pub const GL_UNPACK_FLIP_Y_WEBGL: GlEnum = GlEnum(0x9240);
pub const GL_UNPACK_PREMULTIPLY_ALPHA_WEBGL: GlEnum = GlEnum(0x9241);
pub const GL_CONTEXT_LOST_WEBGL: GlEnum = GlEnum(0x9242);
pub const GL_UNPACK_COLORSPACE_CONVERSION_WEBGL: GlEnum = GlEnum(0x9243);
pub const GL_BROWSER_DEFAULT_WEBGL: GlEnum = GlEnum(0x9244);
/// 
/// * Group: [`ShaderBinaryFormat`]
pub const GL_SHADER_BINARY_DMP: GlEnum = GlEnum(0x9250);
pub const GL_SMAPHS30_PROGRAM_BINARY_DMP: GlEnum = GlEnum(0x9251);
pub const GL_SMAPHS_PROGRAM_BINARY_DMP: GlEnum = GlEnum(0x9252);
pub const GL_DMP_PROGRAM_BINARY_DMP: GlEnum = GlEnum(0x9253);
/// 
/// * Group: [`ShaderBinaryFormat`]
pub const GL_GCCSO_SHADER_BINARY_FJ: GlEnum = GlEnum(0x9260);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_R11_EAC: GlEnum = GlEnum(0x9270);
pub const GL_COMPRESSED_R11_EAC_OES: GlEnum = GlEnum(0x9270);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_R11_EAC: GlEnum = GlEnum(0x9271);
pub const GL_COMPRESSED_SIGNED_R11_EAC_OES: GlEnum = GlEnum(0x9271);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RG11_EAC: GlEnum = GlEnum(0x9272);
pub const GL_COMPRESSED_RG11_EAC_OES: GlEnum = GlEnum(0x9272);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_RG11_EAC: GlEnum = GlEnum(0x9273);
pub const GL_COMPRESSED_SIGNED_RG11_EAC_OES: GlEnum = GlEnum(0x9273);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB8_ETC2: GlEnum = GlEnum(0x9274);
pub const GL_COMPRESSED_RGB8_ETC2_OES: GlEnum = GlEnum(0x9274);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ETC2: GlEnum = GlEnum(0x9275);
pub const GL_COMPRESSED_SRGB8_ETC2_OES: GlEnum = GlEnum(0x9275);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GlEnum = GlEnum(0x9276);
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2_OES: GlEnum = GlEnum(0x9276);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GlEnum = GlEnum(0x9277);
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2_OES: GlEnum = GlEnum(0x9277);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GlEnum = GlEnum(0x9278);
pub const GL_COMPRESSED_RGBA8_ETC2_EAC_OES: GlEnum = GlEnum(0x9278);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GlEnum = GlEnum(0x9279);
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC_OES: GlEnum = GlEnum(0x9279);
pub const GL_BLEND_PREMULTIPLIED_SRC_NV: GlEnum = GlEnum(0x9280);
pub const GL_BLEND_OVERLAP_NV: GlEnum = GlEnum(0x9281);
pub const GL_UNCORRELATED_NV: GlEnum = GlEnum(0x9282);
pub const GL_DISJOINT_NV: GlEnum = GlEnum(0x9283);
pub const GL_CONJOINT_NV: GlEnum = GlEnum(0x9284);
pub const GL_BLEND_ADVANCED_COHERENT_KHR: GlEnum = GlEnum(0x9285);
pub const GL_BLEND_ADVANCED_COHERENT_NV: GlEnum = GlEnum(0x9285);
pub const GL_SRC_NV: GlEnum = GlEnum(0x9286);
pub const GL_DST_NV: GlEnum = GlEnum(0x9287);
pub const GL_SRC_OVER_NV: GlEnum = GlEnum(0x9288);
pub const GL_DST_OVER_NV: GlEnum = GlEnum(0x9289);
pub const GL_SRC_IN_NV: GlEnum = GlEnum(0x928A);
pub const GL_DST_IN_NV: GlEnum = GlEnum(0x928B);
pub const GL_SRC_OUT_NV: GlEnum = GlEnum(0x928C);
pub const GL_DST_OUT_NV: GlEnum = GlEnum(0x928D);
pub const GL_SRC_ATOP_NV: GlEnum = GlEnum(0x928E);
pub const GL_DST_ATOP_NV: GlEnum = GlEnum(0x928F);
pub const GL_PLUS_NV: GlEnum = GlEnum(0x9291);
pub const GL_PLUS_DARKER_NV: GlEnum = GlEnum(0x9292);
pub const GL_MULTIPLY: GlEnum = GlEnum(0x9294);
pub const GL_MULTIPLY_KHR: GlEnum = GlEnum(0x9294);
pub const GL_MULTIPLY_NV: GlEnum = GlEnum(0x9294);
pub const GL_SCREEN: GlEnum = GlEnum(0x9295);
pub const GL_SCREEN_KHR: GlEnum = GlEnum(0x9295);
pub const GL_SCREEN_NV: GlEnum = GlEnum(0x9295);
pub const GL_OVERLAY: GlEnum = GlEnum(0x9296);
pub const GL_OVERLAY_KHR: GlEnum = GlEnum(0x9296);
pub const GL_OVERLAY_NV: GlEnum = GlEnum(0x9296);
pub const GL_DARKEN: GlEnum = GlEnum(0x9297);
pub const GL_DARKEN_KHR: GlEnum = GlEnum(0x9297);
pub const GL_DARKEN_NV: GlEnum = GlEnum(0x9297);
pub const GL_LIGHTEN: GlEnum = GlEnum(0x9298);
pub const GL_LIGHTEN_KHR: GlEnum = GlEnum(0x9298);
pub const GL_LIGHTEN_NV: GlEnum = GlEnum(0x9298);
pub const GL_COLORDODGE: GlEnum = GlEnum(0x9299);
pub const GL_COLORDODGE_KHR: GlEnum = GlEnum(0x9299);
pub const GL_COLORDODGE_NV: GlEnum = GlEnum(0x9299);
pub const GL_COLORBURN: GlEnum = GlEnum(0x929A);
pub const GL_COLORBURN_KHR: GlEnum = GlEnum(0x929A);
pub const GL_COLORBURN_NV: GlEnum = GlEnum(0x929A);
pub const GL_HARDLIGHT: GlEnum = GlEnum(0x929B);
pub const GL_HARDLIGHT_KHR: GlEnum = GlEnum(0x929B);
pub const GL_HARDLIGHT_NV: GlEnum = GlEnum(0x929B);
pub const GL_SOFTLIGHT: GlEnum = GlEnum(0x929C);
pub const GL_SOFTLIGHT_KHR: GlEnum = GlEnum(0x929C);
pub const GL_SOFTLIGHT_NV: GlEnum = GlEnum(0x929C);
pub const GL_DIFFERENCE: GlEnum = GlEnum(0x929E);
pub const GL_DIFFERENCE_KHR: GlEnum = GlEnum(0x929E);
pub const GL_DIFFERENCE_NV: GlEnum = GlEnum(0x929E);
pub const GL_MINUS_NV: GlEnum = GlEnum(0x929F);
pub const GL_EXCLUSION: GlEnum = GlEnum(0x92A0);
pub const GL_EXCLUSION_KHR: GlEnum = GlEnum(0x92A0);
pub const GL_EXCLUSION_NV: GlEnum = GlEnum(0x92A0);
pub const GL_CONTRAST_NV: GlEnum = GlEnum(0x92A1);
pub const GL_INVERT_RGB_NV: GlEnum = GlEnum(0x92A3);
pub const GL_LINEARDODGE_NV: GlEnum = GlEnum(0x92A4);
pub const GL_LINEARBURN_NV: GlEnum = GlEnum(0x92A5);
pub const GL_VIVIDLIGHT_NV: GlEnum = GlEnum(0x92A6);
pub const GL_LINEARLIGHT_NV: GlEnum = GlEnum(0x92A7);
pub const GL_PINLIGHT_NV: GlEnum = GlEnum(0x92A8);
pub const GL_HARDMIX_NV: GlEnum = GlEnum(0x92A9);
pub const GL_HSL_HUE: GlEnum = GlEnum(0x92AD);
pub const GL_HSL_HUE_KHR: GlEnum = GlEnum(0x92AD);
pub const GL_HSL_HUE_NV: GlEnum = GlEnum(0x92AD);
pub const GL_HSL_SATURATION: GlEnum = GlEnum(0x92AE);
pub const GL_HSL_SATURATION_KHR: GlEnum = GlEnum(0x92AE);
pub const GL_HSL_SATURATION_NV: GlEnum = GlEnum(0x92AE);
pub const GL_HSL_COLOR: GlEnum = GlEnum(0x92AF);
pub const GL_HSL_COLOR_KHR: GlEnum = GlEnum(0x92AF);
pub const GL_HSL_COLOR_NV: GlEnum = GlEnum(0x92AF);
pub const GL_HSL_LUMINOSITY: GlEnum = GlEnum(0x92B0);
pub const GL_HSL_LUMINOSITY_KHR: GlEnum = GlEnum(0x92B0);
pub const GL_HSL_LUMINOSITY_NV: GlEnum = GlEnum(0x92B0);
pub const GL_PLUS_CLAMPED_NV: GlEnum = GlEnum(0x92B1);
pub const GL_PLUS_CLAMPED_ALPHA_NV: GlEnum = GlEnum(0x92B2);
pub const GL_MINUS_CLAMPED_NV: GlEnum = GlEnum(0x92B3);
pub const GL_INVERT_OVG_NV: GlEnum = GlEnum(0x92B4);
pub const GL_MAX_LGPU_GPUS_NVX: GlEnum = GlEnum(0x92BA);
pub const GL_MULTICAST_GPUS_NV: GlEnum = GlEnum(0x92BA);
pub const GL_PURGED_CONTEXT_RESET_NV: GlEnum = GlEnum(0x92BB);
pub const GL_PRIMITIVE_BOUNDING_BOX_ARB: GlEnum = GlEnum(0x92BE);
pub const GL_PRIMITIVE_BOUNDING_BOX: GlEnum = GlEnum(0x92BE);
pub const GL_PRIMITIVE_BOUNDING_BOX_EXT: GlEnum = GlEnum(0x92BE);
pub const GL_PRIMITIVE_BOUNDING_BOX_OES: GlEnum = GlEnum(0x92BE);
pub const GL_ALPHA_TO_COVERAGE_DITHER_MODE_NV: GlEnum = GlEnum(0x92BF);
/// 
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_ATOMIC_COUNTER_BUFFER: GlEnum = GlEnum(0x92C0);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: GlEnum = GlEnum(0x92C1);
pub const GL_ATOMIC_COUNTER_BUFFER_START: GlEnum = GlEnum(0x92C2);
pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: GlEnum = GlEnum(0x92C3);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: GlEnum = GlEnum(0x92C4);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GlEnum = GlEnum(0x92C5);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GlEnum = GlEnum(0x92C6);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GlEnum = GlEnum(0x92C7);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GlEnum = GlEnum(0x92C8);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GlEnum = GlEnum(0x92C9);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GlEnum = GlEnum(0x92CA);
/// 
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GlEnum = GlEnum(0x92CB);
pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GlEnum = GlEnum(0x92CC);
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GlEnum = GlEnum(0x92CD);
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS_EXT: GlEnum = GlEnum(0x92CD);
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS_OES: GlEnum = GlEnum(0x92CD);
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GlEnum = GlEnum(0x92CE);
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS_EXT: GlEnum = GlEnum(0x92CE);
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS_OES: GlEnum = GlEnum(0x92CE);
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GlEnum = GlEnum(0x92CF);
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS_EXT: GlEnum = GlEnum(0x92CF);
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS_OES: GlEnum = GlEnum(0x92CF);
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GlEnum = GlEnum(0x92D0);
pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GlEnum = GlEnum(0x92D1);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: GlEnum = GlEnum(0x92D2);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GlEnum = GlEnum(0x92D3);
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS_EXT: GlEnum = GlEnum(0x92D3);
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS_OES: GlEnum = GlEnum(0x92D3);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GlEnum = GlEnum(0x92D4);
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS_EXT: GlEnum = GlEnum(0x92D4);
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS_OES: GlEnum = GlEnum(0x92D4);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GlEnum = GlEnum(0x92D5);
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS_EXT: GlEnum = GlEnum(0x92D5);
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS_OES: GlEnum = GlEnum(0x92D5);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GlEnum = GlEnum(0x92D6);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: GlEnum = GlEnum(0x92D7);
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GlEnum = GlEnum(0x92D8);
/// 
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GlEnum = GlEnum(0x92D9);
/// 
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GlEnum = GlEnum(0x92DA);
/// 
/// * Group: [`GlslTypeToken`]
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: GlEnum = GlEnum(0x92DB);
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GlEnum = GlEnum(0x92DC);
pub const GL_FRAGMENT_COVERAGE_TO_COLOR_NV: GlEnum = GlEnum(0x92DD);
pub const GL_FRAGMENT_COVERAGE_COLOR_NV: GlEnum = GlEnum(0x92DE);
pub const GL_MESH_OUTPUT_PER_VERTEX_GRANULARITY_NV: GlEnum = GlEnum(0x92DF);
/// 
/// * Group: [`EnableCap`]
pub const GL_DEBUG_OUTPUT: GlEnum = GlEnum(0x92E0);
pub const GL_DEBUG_OUTPUT_KHR: GlEnum = GlEnum(0x92E0);
/// 
/// * Group: [`ProgramResourceProperty`], [`ProgramInterface`]
pub const GL_UNIFORM: GlEnum = GlEnum(0x92E1);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_UNIFORM_BLOCK: GlEnum = GlEnum(0x92E2);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_PROGRAM_INPUT: GlEnum = GlEnum(0x92E3);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_PROGRAM_OUTPUT: GlEnum = GlEnum(0x92E4);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_BUFFER_VARIABLE: GlEnum = GlEnum(0x92E5);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_SHADER_STORAGE_BLOCK: GlEnum = GlEnum(0x92E6);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_IS_PER_PATCH: GlEnum = GlEnum(0x92E7);
pub const GL_IS_PER_PATCH_EXT: GlEnum = GlEnum(0x92E7);
pub const GL_IS_PER_PATCH_OES: GlEnum = GlEnum(0x92E7);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_VERTEX_SUBROUTINE: GlEnum = GlEnum(0x92E8);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_TESS_CONTROL_SUBROUTINE: GlEnum = GlEnum(0x92E9);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_TESS_EVALUATION_SUBROUTINE: GlEnum = GlEnum(0x92EA);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_GEOMETRY_SUBROUTINE: GlEnum = GlEnum(0x92EB);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_FRAGMENT_SUBROUTINE: GlEnum = GlEnum(0x92EC);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_COMPUTE_SUBROUTINE: GlEnum = GlEnum(0x92ED);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_VERTEX_SUBROUTINE_UNIFORM: GlEnum = GlEnum(0x92EE);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: GlEnum = GlEnum(0x92EF);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: GlEnum = GlEnum(0x92F0);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: GlEnum = GlEnum(0x92F1);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: GlEnum = GlEnum(0x92F2);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_COMPUTE_SUBROUTINE_UNIFORM: GlEnum = GlEnum(0x92F3);
/// 
/// * Group: [`ProgramInterface`]
pub const GL_TRANSFORM_FEEDBACK_VARYING: GlEnum = GlEnum(0x92F4);
/// 
/// * Group: [`ProgramInterfacePName`]
pub const GL_ACTIVE_RESOURCES: GlEnum = GlEnum(0x92F5);
/// 
/// * Group: [`ProgramInterfacePName`]
pub const GL_MAX_NAME_LENGTH: GlEnum = GlEnum(0x92F6);
/// 
/// * Group: [`ProgramInterfacePName`]
pub const GL_MAX_NUM_ACTIVE_VARIABLES: GlEnum = GlEnum(0x92F7);
/// 
/// * Group: [`ProgramInterfacePName`]
pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: GlEnum = GlEnum(0x92F8);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_NAME_LENGTH: GlEnum = GlEnum(0x92F9);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_TYPE: GlEnum = GlEnum(0x92FA);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_ARRAY_SIZE: GlEnum = GlEnum(0x92FB);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_OFFSET: GlEnum = GlEnum(0x92FC);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_BLOCK_INDEX: GlEnum = GlEnum(0x92FD);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_ARRAY_STRIDE: GlEnum = GlEnum(0x92FE);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_MATRIX_STRIDE: GlEnum = GlEnum(0x92FF);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_IS_ROW_MAJOR: GlEnum = GlEnum(0x9300);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GlEnum = GlEnum(0x9301);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_BUFFER_BINDING: GlEnum = GlEnum(0x9302);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_BUFFER_DATA_SIZE: GlEnum = GlEnum(0x9303);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_NUM_ACTIVE_VARIABLES: GlEnum = GlEnum(0x9304);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_ACTIVE_VARIABLES: GlEnum = GlEnum(0x9305);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_VERTEX_SHADER: GlEnum = GlEnum(0x9306);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GlEnum = GlEnum(0x9307);
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER_EXT: GlEnum = GlEnum(0x9307);
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER_OES: GlEnum = GlEnum(0x9307);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GlEnum = GlEnum(0x9308);
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER_EXT: GlEnum = GlEnum(0x9308);
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER_OES: GlEnum = GlEnum(0x9308);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GlEnum = GlEnum(0x9309);
pub const GL_REFERENCED_BY_GEOMETRY_SHADER_EXT: GlEnum = GlEnum(0x9309);
pub const GL_REFERENCED_BY_GEOMETRY_SHADER_OES: GlEnum = GlEnum(0x9309);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GlEnum = GlEnum(0x930A);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_COMPUTE_SHADER: GlEnum = GlEnum(0x930B);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_TOP_LEVEL_ARRAY_SIZE: GlEnum = GlEnum(0x930C);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_TOP_LEVEL_ARRAY_STRIDE: GlEnum = GlEnum(0x930D);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_LOCATION: GlEnum = GlEnum(0x930E);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_LOCATION_INDEX: GlEnum = GlEnum(0x930F);
pub const GL_LOCATION_INDEX_EXT: GlEnum = GlEnum(0x930F);
/// 
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GlEnum = GlEnum(0x9310);
/// 
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GlEnum = GlEnum(0x9311);
/// 
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GlEnum = GlEnum(0x9312);
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS_EXT: GlEnum = GlEnum(0x9312);
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS_OES: GlEnum = GlEnum(0x9312);
/// 
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GlEnum = GlEnum(0x9313);
/// 
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GlEnum = GlEnum(0x9314);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEBUFFER_WIDTH: GlEnum = GlEnum(0x9315);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEBUFFER_HEIGHT: GlEnum = GlEnum(0x9316);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEBUFFER_LAYERS: GlEnum = GlEnum(0x9317);
pub const GL_MAX_FRAMEBUFFER_LAYERS_EXT: GlEnum = GlEnum(0x9317);
pub const GL_MAX_FRAMEBUFFER_LAYERS_OES: GlEnum = GlEnum(0x9317);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEBUFFER_SAMPLES: GlEnum = GlEnum(0x9318);
pub const GL_RASTER_MULTISAMPLE_EXT: GlEnum = GlEnum(0x9327);
pub const GL_RASTER_SAMPLES_EXT: GlEnum = GlEnum(0x9328);
pub const GL_MAX_RASTER_SAMPLES_EXT: GlEnum = GlEnum(0x9329);
pub const GL_RASTER_FIXED_SAMPLE_LOCATIONS_EXT: GlEnum = GlEnum(0x932A);
pub const GL_MULTISAMPLE_RASTERIZATION_ALLOWED_EXT: GlEnum = GlEnum(0x932B);
pub const GL_EFFECTIVE_RASTER_SAMPLES_EXT: GlEnum = GlEnum(0x932C);
pub const GL_DEPTH_SAMPLES_NV: GlEnum = GlEnum(0x932D);
pub const GL_STENCIL_SAMPLES_NV: GlEnum = GlEnum(0x932E);
pub const GL_MIXED_DEPTH_SAMPLES_SUPPORTED_NV: GlEnum = GlEnum(0x932F);
pub const GL_MIXED_STENCIL_SAMPLES_SUPPORTED_NV: GlEnum = GlEnum(0x9330);
pub const GL_COVERAGE_MODULATION_TABLE_NV: GlEnum = GlEnum(0x9331);
pub const GL_COVERAGE_MODULATION_NV: GlEnum = GlEnum(0x9332);
pub const GL_COVERAGE_MODULATION_TABLE_SIZE_NV: GlEnum = GlEnum(0x9333);
pub const GL_WARP_SIZE_NV: GlEnum = GlEnum(0x9339);
pub const GL_WARPS_PER_SM_NV: GlEnum = GlEnum(0x933A);
pub const GL_SM_COUNT_NV: GlEnum = GlEnum(0x933B);
pub const GL_FILL_RECTANGLE_NV: GlEnum = GlEnum(0x933C);
pub const GL_SAMPLE_LOCATION_SUBPIXEL_BITS_ARB: GlEnum = GlEnum(0x933D);
pub const GL_SAMPLE_LOCATION_SUBPIXEL_BITS_NV: GlEnum = GlEnum(0x933D);
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_WIDTH_ARB: GlEnum = GlEnum(0x933E);
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_WIDTH_NV: GlEnum = GlEnum(0x933E);
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_ARB: GlEnum = GlEnum(0x933F);
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_NV: GlEnum = GlEnum(0x933F);
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_ARB: GlEnum = GlEnum(0x9340);
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_NV: GlEnum = GlEnum(0x9340);
/// 
/// * Group: [`GetMultisamplePNameNV`]
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_ARB: GlEnum = GlEnum(0x9341);
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_NV: GlEnum = GlEnum(0x9341);
pub const GL_FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_ARB: GlEnum = GlEnum(0x9342);
pub const GL_FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_NV: GlEnum = GlEnum(0x9342);
pub const GL_FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_ARB: GlEnum = GlEnum(0x9343);
pub const GL_FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_NV: GlEnum = GlEnum(0x9343);
pub const GL_MAX_COMPUTE_VARIABLE_GROUP_INVOCATIONS_ARB: GlEnum = GlEnum(0x9344);
pub const GL_MAX_COMPUTE_VARIABLE_GROUP_SIZE_ARB: GlEnum = GlEnum(0x9345);
pub const GL_CONSERVATIVE_RASTERIZATION_NV: GlEnum = GlEnum(0x9346);
pub const GL_SUBPIXEL_PRECISION_BIAS_X_BITS_NV: GlEnum = GlEnum(0x9347);
pub const GL_SUBPIXEL_PRECISION_BIAS_Y_BITS_NV: GlEnum = GlEnum(0x9348);
pub const GL_MAX_SUBPIXEL_PRECISION_BIAS_BITS_NV: GlEnum = GlEnum(0x9349);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_LOCATION_COMPONENT: GlEnum = GlEnum(0x934A);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: GlEnum = GlEnum(0x934B);
/// 
/// * Group: [`ProgramResourceProperty`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: GlEnum = GlEnum(0x934C);
pub const GL_ALPHA_TO_COVERAGE_DITHER_DEFAULT_NV: GlEnum = GlEnum(0x934D);
pub const GL_ALPHA_TO_COVERAGE_DITHER_ENABLE_NV: GlEnum = GlEnum(0x934E);
pub const GL_ALPHA_TO_COVERAGE_DITHER_DISABLE_NV: GlEnum = GlEnum(0x934F);
pub const GL_VIEWPORT_SWIZZLE_POSITIVE_X_NV: GlEnum = GlEnum(0x9350);
pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_X_NV: GlEnum = GlEnum(0x9351);
pub const GL_VIEWPORT_SWIZZLE_POSITIVE_Y_NV: GlEnum = GlEnum(0x9352);
pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_Y_NV: GlEnum = GlEnum(0x9353);
pub const GL_VIEWPORT_SWIZZLE_POSITIVE_Z_NV: GlEnum = GlEnum(0x9354);
pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_Z_NV: GlEnum = GlEnum(0x9355);
pub const GL_VIEWPORT_SWIZZLE_POSITIVE_W_NV: GlEnum = GlEnum(0x9356);
pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_W_NV: GlEnum = GlEnum(0x9357);
pub const GL_VIEWPORT_SWIZZLE_X_NV: GlEnum = GlEnum(0x9358);
pub const GL_VIEWPORT_SWIZZLE_Y_NV: GlEnum = GlEnum(0x9359);
pub const GL_VIEWPORT_SWIZZLE_Z_NV: GlEnum = GlEnum(0x935A);
pub const GL_VIEWPORT_SWIZZLE_W_NV: GlEnum = GlEnum(0x935B);
pub const GL_CLIP_ORIGIN: GlEnum = GlEnum(0x935C);
/// 
/// * Alias Of: [`GL_CLIP_ORIGIN`]
pub const GL_CLIP_ORIGIN_EXT: GlEnum = GL_CLIP_ORIGIN;
pub const GL_CLIP_DEPTH_MODE: GlEnum = GlEnum(0x935D);
/// 
/// * Alias Of: [`GL_CLIP_DEPTH_MODE`]
pub const GL_CLIP_DEPTH_MODE_EXT: GlEnum = GL_CLIP_DEPTH_MODE;
/// 
/// * Group: [`ClipControlDepth`]
pub const GL_NEGATIVE_ONE_TO_ONE: GlEnum = GlEnum(0x935E);
/// 
/// * Alias Of: [`GL_NEGATIVE_ONE_TO_ONE`]
pub const GL_NEGATIVE_ONE_TO_ONE_EXT: GlEnum = GL_NEGATIVE_ONE_TO_ONE;
/// 
/// * Group: [`ClipControlDepth`]
pub const GL_ZERO_TO_ONE: GlEnum = GlEnum(0x935F);
/// 
/// * Alias Of: [`GL_ZERO_TO_ONE`]
pub const GL_ZERO_TO_ONE_EXT: GlEnum = GL_ZERO_TO_ONE;
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_CLEAR_TEXTURE: GlEnum = GlEnum(0x9365);
pub const GL_TEXTURE_REDUCTION_MODE_ARB: GlEnum = GlEnum(0x9366);
/// 
/// * Alias Of: [`GL_TEXTURE_REDUCTION_MODE_ARB`]
pub const GL_TEXTURE_REDUCTION_MODE_EXT: GlEnum = GL_TEXTURE_REDUCTION_MODE_ARB;
pub const GL_WEIGHTED_AVERAGE_ARB: GlEnum = GlEnum(0x9367);
/// 
/// * Alias Of: [`GL_WEIGHTED_AVERAGE_ARB`]
pub const GL_WEIGHTED_AVERAGE_EXT: GlEnum = GL_WEIGHTED_AVERAGE_ARB;
pub const GL_FONT_GLYPHS_AVAILABLE_NV: GlEnum = GlEnum(0x9368);
pub const GL_FONT_TARGET_UNAVAILABLE_NV: GlEnum = GlEnum(0x9369);
pub const GL_FONT_UNAVAILABLE_NV: GlEnum = GlEnum(0x936A);
pub const GL_FONT_UNINTELLIGIBLE_NV: GlEnum = GlEnum(0x936B);
pub const GL_STANDARD_FONT_FORMAT_NV: GlEnum = GlEnum(0x936C);
pub const GL_FRAGMENT_INPUT_NV: GlEnum = GlEnum(0x936D);
pub const GL_UNIFORM_BUFFER_UNIFIED_NV: GlEnum = GlEnum(0x936E);
pub const GL_UNIFORM_BUFFER_ADDRESS_NV: GlEnum = GlEnum(0x936F);
pub const GL_UNIFORM_BUFFER_LENGTH_NV: GlEnum = GlEnum(0x9370);
pub const GL_MULTISAMPLES_NV: GlEnum = GlEnum(0x9371);
pub const GL_SUPERSAMPLE_SCALE_X_NV: GlEnum = GlEnum(0x9372);
pub const GL_SUPERSAMPLE_SCALE_Y_NV: GlEnum = GlEnum(0x9373);
pub const GL_CONFORMANT_NV: GlEnum = GlEnum(0x9374);
pub const GL_CONSERVATIVE_RASTER_DILATE_NV: GlEnum = GlEnum(0x9379);
pub const GL_CONSERVATIVE_RASTER_DILATE_RANGE_NV: GlEnum = GlEnum(0x937A);
pub const GL_CONSERVATIVE_RASTER_DILATE_GRANULARITY_NV: GlEnum = GlEnum(0x937B);
pub const GL_VIEWPORT_POSITION_W_SCALE_NV: GlEnum = GlEnum(0x937C);
pub const GL_VIEWPORT_POSITION_W_SCALE_X_COEFF_NV: GlEnum = GlEnum(0x937D);
pub const GL_VIEWPORT_POSITION_W_SCALE_Y_COEFF_NV: GlEnum = GlEnum(0x937E);
pub const GL_REPRESENTATIVE_FRAGMENT_TEST_NV: GlEnum = GlEnum(0x937F);
/// 
/// * Group: [`InternalFormatPName`]
pub const GL_NUM_SAMPLE_COUNTS: GlEnum = GlEnum(0x9380);
pub const GL_MULTISAMPLE_LINE_WIDTH_RANGE_ARB: GlEnum = GlEnum(0x9381);
pub const GL_MULTISAMPLE_LINE_WIDTH_RANGE: GlEnum = GlEnum(0x9381);
pub const GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY_ARB: GlEnum = GlEnum(0x9382);
pub const GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY: GlEnum = GlEnum(0x9382);
pub const GL_VIEW_CLASS_EAC_R11: GlEnum = GlEnum(0x9383);
pub const GL_VIEW_CLASS_EAC_RG11: GlEnum = GlEnum(0x9384);
pub const GL_VIEW_CLASS_ETC2_RGB: GlEnum = GlEnum(0x9385);
pub const GL_VIEW_CLASS_ETC2_RGBA: GlEnum = GlEnum(0x9386);
pub const GL_VIEW_CLASS_ETC2_EAC_RGBA: GlEnum = GlEnum(0x9387);
pub const GL_VIEW_CLASS_ASTC_4x4_RGBA: GlEnum = GlEnum(0x9388);
pub const GL_VIEW_CLASS_ASTC_5x4_RGBA: GlEnum = GlEnum(0x9389);
pub const GL_VIEW_CLASS_ASTC_5x5_RGBA: GlEnum = GlEnum(0x938A);
pub const GL_VIEW_CLASS_ASTC_6x5_RGBA: GlEnum = GlEnum(0x938B);
pub const GL_VIEW_CLASS_ASTC_6x6_RGBA: GlEnum = GlEnum(0x938C);
pub const GL_VIEW_CLASS_ASTC_8x5_RGBA: GlEnum = GlEnum(0x938D);
pub const GL_VIEW_CLASS_ASTC_8x6_RGBA: GlEnum = GlEnum(0x938E);
pub const GL_VIEW_CLASS_ASTC_8x8_RGBA: GlEnum = GlEnum(0x938F);
pub const GL_VIEW_CLASS_ASTC_10x5_RGBA: GlEnum = GlEnum(0x9390);
pub const GL_VIEW_CLASS_ASTC_10x6_RGBA: GlEnum = GlEnum(0x9391);
pub const GL_VIEW_CLASS_ASTC_10x8_RGBA: GlEnum = GlEnum(0x9392);
pub const GL_VIEW_CLASS_ASTC_10x10_RGBA: GlEnum = GlEnum(0x9393);
pub const GL_VIEW_CLASS_ASTC_12x10_RGBA: GlEnum = GlEnum(0x9394);
pub const GL_VIEW_CLASS_ASTC_12x12_RGBA: GlEnum = GlEnum(0x9395);
pub const GL_TRANSLATED_SHADER_SOURCE_LENGTH_ANGLE: GlEnum = GlEnum(0x93A0);
pub const GL_BGRA8_EXT: GlEnum = GlEnum(0x93A1);
pub const GL_TEXTURE_USAGE_ANGLE: GlEnum = GlEnum(0x93A2);
pub const GL_FRAMEBUFFER_ATTACHMENT_ANGLE: GlEnum = GlEnum(0x93A3);
pub const GL_PACK_REVERSE_ROW_ORDER_ANGLE: GlEnum = GlEnum(0x93A4);
pub const GL_PROGRAM_BINARY_ANGLE: GlEnum = GlEnum(0x93A6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x4: GlEnum = GlEnum(0x93B0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR: GlEnum = GlEnum(0x93B0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x4: GlEnum = GlEnum(0x93B1);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR: GlEnum = GlEnum(0x93B1);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x5: GlEnum = GlEnum(0x93B2);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR: GlEnum = GlEnum(0x93B2);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x5: GlEnum = GlEnum(0x93B3);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR: GlEnum = GlEnum(0x93B3);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x6: GlEnum = GlEnum(0x93B4);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR: GlEnum = GlEnum(0x93B4);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x5: GlEnum = GlEnum(0x93B5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR: GlEnum = GlEnum(0x93B5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x6: GlEnum = GlEnum(0x93B6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR: GlEnum = GlEnum(0x93B6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x8: GlEnum = GlEnum(0x93B7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR: GlEnum = GlEnum(0x93B7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x5: GlEnum = GlEnum(0x93B8);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR: GlEnum = GlEnum(0x93B8);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x6: GlEnum = GlEnum(0x93B9);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR: GlEnum = GlEnum(0x93B9);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x8: GlEnum = GlEnum(0x93BA);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR: GlEnum = GlEnum(0x93BA);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x10: GlEnum = GlEnum(0x93BB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR: GlEnum = GlEnum(0x93BB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_12x10: GlEnum = GlEnum(0x93BC);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR: GlEnum = GlEnum(0x93BC);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_12x12: GlEnum = GlEnum(0x93BD);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR: GlEnum = GlEnum(0x93BD);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_3x3x3_OES: GlEnum = GlEnum(0x93C0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x3x3_OES: GlEnum = GlEnum(0x93C1);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x4x3_OES: GlEnum = GlEnum(0x93C2);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x4x4_OES: GlEnum = GlEnum(0x93C3);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x4x4_OES: GlEnum = GlEnum(0x93C4);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x5x4_OES: GlEnum = GlEnum(0x93C5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x5x5_OES: GlEnum = GlEnum(0x93C6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x5x5_OES: GlEnum = GlEnum(0x93C7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x6x5_OES: GlEnum = GlEnum(0x93C8);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x6x6_OES: GlEnum = GlEnum(0x93C9);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: GlEnum = GlEnum(0x93D0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: GlEnum = GlEnum(0x93D0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: GlEnum = GlEnum(0x93D1);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: GlEnum = GlEnum(0x93D1);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: GlEnum = GlEnum(0x93D2);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: GlEnum = GlEnum(0x93D2);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: GlEnum = GlEnum(0x93D3);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: GlEnum = GlEnum(0x93D3);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: GlEnum = GlEnum(0x93D4);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: GlEnum = GlEnum(0x93D4);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: GlEnum = GlEnum(0x93D5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: GlEnum = GlEnum(0x93D5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: GlEnum = GlEnum(0x93D6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: GlEnum = GlEnum(0x93D6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: GlEnum = GlEnum(0x93D7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: GlEnum = GlEnum(0x93D7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: GlEnum = GlEnum(0x93D8);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: GlEnum = GlEnum(0x93D8);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: GlEnum = GlEnum(0x93D9);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: GlEnum = GlEnum(0x93D9);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: GlEnum = GlEnum(0x93DA);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: GlEnum = GlEnum(0x93DA);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: GlEnum = GlEnum(0x93DB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: GlEnum = GlEnum(0x93DB);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: GlEnum = GlEnum(0x93DC);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: GlEnum = GlEnum(0x93DC);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: GlEnum = GlEnum(0x93DD);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: GlEnum = GlEnum(0x93DD);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_3x3x3_OES: GlEnum = GlEnum(0x93E0);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x3x3_OES: GlEnum = GlEnum(0x93E1);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x3_OES: GlEnum = GlEnum(0x93E2);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x4_OES: GlEnum = GlEnum(0x93E3);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4x4_OES: GlEnum = GlEnum(0x93E4);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x4_OES: GlEnum = GlEnum(0x93E5);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x5_OES: GlEnum = GlEnum(0x93E6);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5x5_OES: GlEnum = GlEnum(0x93E7);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x5_OES: GlEnum = GlEnum(0x93E8);
/// 
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x6_OES: GlEnum = GlEnum(0x93E9);
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV2_IMG: GlEnum = GlEnum(0x93F0);
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV2_IMG: GlEnum = GlEnum(0x93F1);
pub const GL_PERFQUERY_COUNTER_EVENT_INTEL: GlEnum = GlEnum(0x94F0);
pub const GL_PERFQUERY_COUNTER_DURATION_NORM_INTEL: GlEnum = GlEnum(0x94F1);
pub const GL_PERFQUERY_COUNTER_DURATION_RAW_INTEL: GlEnum = GlEnum(0x94F2);
pub const GL_PERFQUERY_COUNTER_THROUGHPUT_INTEL: GlEnum = GlEnum(0x94F3);
pub const GL_PERFQUERY_COUNTER_RAW_INTEL: GlEnum = GlEnum(0x94F4);
pub const GL_PERFQUERY_COUNTER_TIMESTAMP_INTEL: GlEnum = GlEnum(0x94F5);
pub const GL_PERFQUERY_COUNTER_DATA_UINT32_INTEL: GlEnum = GlEnum(0x94F8);
pub const GL_PERFQUERY_COUNTER_DATA_UINT64_INTEL: GlEnum = GlEnum(0x94F9);
pub const GL_PERFQUERY_COUNTER_DATA_FLOAT_INTEL: GlEnum = GlEnum(0x94FA);
pub const GL_PERFQUERY_COUNTER_DATA_DOUBLE_INTEL: GlEnum = GlEnum(0x94FB);
pub const GL_PERFQUERY_COUNTER_DATA_BOOL32_INTEL: GlEnum = GlEnum(0x94FC);
pub const GL_PERFQUERY_QUERY_NAME_LENGTH_MAX_INTEL: GlEnum = GlEnum(0x94FD);
pub const GL_PERFQUERY_COUNTER_NAME_LENGTH_MAX_INTEL: GlEnum = GlEnum(0x94FE);
pub const GL_PERFQUERY_COUNTER_DESC_LENGTH_MAX_INTEL: GlEnum = GlEnum(0x94FF);
pub const GL_PERFQUERY_GPA_EXTENDED_COUNTERS_INTEL: GlEnum = GlEnum(0x9500);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_EXT: GlEnum = GlEnum(0x9530);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_EXT: GlEnum = GlEnum(0x9531);
pub const GL_SUBGROUP_SIZE_KHR: GlEnum = GlEnum(0x9532);
pub const GL_SUBGROUP_SUPPORTED_STAGES_KHR: GlEnum = GlEnum(0x9533);
pub const GL_SUBGROUP_SUPPORTED_FEATURES_KHR: GlEnum = GlEnum(0x9534);
pub const GL_SUBGROUP_QUAD_ALL_STAGES_KHR: GlEnum = GlEnum(0x9535);
pub const GL_MAX_MESH_TOTAL_MEMORY_SIZE_NV: GlEnum = GlEnum(0x9536);
pub const GL_MAX_TASK_TOTAL_MEMORY_SIZE_NV: GlEnum = GlEnum(0x9537);
pub const GL_MAX_MESH_OUTPUT_VERTICES_NV: GlEnum = GlEnum(0x9538);
pub const GL_MAX_MESH_OUTPUT_PRIMITIVES_NV: GlEnum = GlEnum(0x9539);
pub const GL_MAX_TASK_OUTPUT_COUNT_NV: GlEnum = GlEnum(0x953A);
pub const GL_MAX_MESH_WORK_GROUP_SIZE_NV: GlEnum = GlEnum(0x953B);
pub const GL_MAX_TASK_WORK_GROUP_SIZE_NV: GlEnum = GlEnum(0x953C);
pub const GL_MAX_DRAW_MESH_TASKS_COUNT_NV: GlEnum = GlEnum(0x953D);
pub const GL_MESH_WORK_GROUP_SIZE_NV: GlEnum = GlEnum(0x953E);
pub const GL_TASK_WORK_GROUP_SIZE_NV: GlEnum = GlEnum(0x953F);
pub const GL_QUERY_RESOURCE_TYPE_VIDMEM_ALLOC_NV: GlEnum = GlEnum(0x9540);
pub const GL_QUERY_RESOURCE_MEMTYPE_VIDMEM_NV: GlEnum = GlEnum(0x9542);
pub const GL_MESH_OUTPUT_PER_PRIMITIVE_GRANULARITY_NV: GlEnum = GlEnum(0x9543);
pub const GL_QUERY_RESOURCE_SYS_RESERVED_NV: GlEnum = GlEnum(0x9544);
pub const GL_QUERY_RESOURCE_TEXTURE_NV: GlEnum = GlEnum(0x9545);
pub const GL_QUERY_RESOURCE_RENDERBUFFER_NV: GlEnum = GlEnum(0x9546);
pub const GL_QUERY_RESOURCE_BUFFEROBJECT_NV: GlEnum = GlEnum(0x9547);
pub const GL_PER_GPU_STORAGE_NV: GlEnum = GlEnum(0x9548);
pub const GL_MULTICAST_PROGRAMMABLE_SAMPLE_LOCATION_NV: GlEnum = GlEnum(0x9549);
pub const GL_UPLOAD_GPU_MASK_NVX: GlEnum = GlEnum(0x954A);
pub const GL_CONSERVATIVE_RASTER_MODE_NV: GlEnum = GlEnum(0x954D);
pub const GL_CONSERVATIVE_RASTER_MODE_POST_SNAP_NV: GlEnum = GlEnum(0x954E);
pub const GL_CONSERVATIVE_RASTER_MODE_PRE_SNAP_TRIANGLES_NV: GlEnum = GlEnum(0x954F);
pub const GL_CONSERVATIVE_RASTER_MODE_PRE_SNAP_NV: GlEnum = GlEnum(0x9550);
/// 
/// * Group: [`ShaderBinaryFormat`]
pub const GL_SHADER_BINARY_FORMAT_SPIR_V: GlEnum = GlEnum(0x9551);
/// 
/// * Alias Of: [`GL_SHADER_BINARY_FORMAT_SPIR_V`]
pub const GL_SHADER_BINARY_FORMAT_SPIR_V_ARB: GlEnum = GL_SHADER_BINARY_FORMAT_SPIR_V;
pub const GL_SPIR_V_BINARY: GlEnum = GlEnum(0x9552);
/// 
/// * Alias Of: [`GL_SPIR_V_BINARY`]
pub const GL_SPIR_V_BINARY_ARB: GlEnum = GL_SPIR_V_BINARY;
pub const GL_SPIR_V_EXTENSIONS: GlEnum = GlEnum(0x9553);
pub const GL_NUM_SPIR_V_EXTENSIONS: GlEnum = GlEnum(0x9554);
pub const GL_SCISSOR_TEST_EXCLUSIVE_NV: GlEnum = GlEnum(0x9555);
pub const GL_SCISSOR_BOX_EXCLUSIVE_NV: GlEnum = GlEnum(0x9556);
pub const GL_MAX_MESH_VIEWS_NV: GlEnum = GlEnum(0x9557);
pub const GL_RENDER_GPU_MASK_NV: GlEnum = GlEnum(0x9558);
pub const GL_MESH_SHADER_NV: GlEnum = GlEnum(0x9559);
pub const GL_TASK_SHADER_NV: GlEnum = GlEnum(0x955A);
pub const GL_SHADING_RATE_IMAGE_BINDING_NV: GlEnum = GlEnum(0x955B);
pub const GL_SHADING_RATE_IMAGE_TEXEL_WIDTH_NV: GlEnum = GlEnum(0x955C);
pub const GL_SHADING_RATE_IMAGE_TEXEL_HEIGHT_NV: GlEnum = GlEnum(0x955D);
pub const GL_SHADING_RATE_IMAGE_PALETTE_SIZE_NV: GlEnum = GlEnum(0x955E);
pub const GL_MAX_COARSE_FRAGMENT_SAMPLES_NV: GlEnum = GlEnum(0x955F);
pub const GL_SHADING_RATE_IMAGE_NV: GlEnum = GlEnum(0x9563);
pub const GL_SHADING_RATE_NO_INVOCATIONS_NV: GlEnum = GlEnum(0x9564);
pub const GL_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: GlEnum = GlEnum(0x9565);
pub const GL_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: GlEnum = GlEnum(0x9566);
pub const GL_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: GlEnum = GlEnum(0x9567);
pub const GL_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: GlEnum = GlEnum(0x9568);
pub const GL_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: GlEnum = GlEnum(0x9569);
pub const GL_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: GlEnum = GlEnum(0x956A);
pub const GL_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: GlEnum = GlEnum(0x956B);
pub const GL_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: GlEnum = GlEnum(0x956C);
pub const GL_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: GlEnum = GlEnum(0x956D);
pub const GL_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: GlEnum = GlEnum(0x956E);
pub const GL_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: GlEnum = GlEnum(0x956F);
pub const GL_MESH_VERTICES_OUT_NV: GlEnum = GlEnum(0x9579);
pub const GL_MESH_PRIMITIVES_OUT_NV: GlEnum = GlEnum(0x957A);
pub const GL_MESH_OUTPUT_TYPE_NV: GlEnum = GlEnum(0x957B);
pub const GL_MESH_SUBROUTINE_NV: GlEnum = GlEnum(0x957C);
pub const GL_TASK_SUBROUTINE_NV: GlEnum = GlEnum(0x957D);
pub const GL_MESH_SUBROUTINE_UNIFORM_NV: GlEnum = GlEnum(0x957E);
pub const GL_TASK_SUBROUTINE_UNIFORM_NV: GlEnum = GlEnum(0x957F);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_TILING_EXT: GlEnum = GlEnum(0x9580);
/// 
/// * Group: [`MemoryObjectParameterName`]
pub const GL_DEDICATED_MEMORY_OBJECT_EXT: GlEnum = GlEnum(0x9581);
pub const GL_NUM_TILING_TYPES_EXT: GlEnum = GlEnum(0x9582);
pub const GL_TILING_TYPES_EXT: GlEnum = GlEnum(0x9583);
pub const GL_OPTIMAL_TILING_EXT: GlEnum = GlEnum(0x9584);
pub const GL_LINEAR_TILING_EXT: GlEnum = GlEnum(0x9585);
/// 
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_OPAQUE_FD_EXT: GlEnum = GlEnum(0x9586);
/// 
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_OPAQUE_WIN32_EXT: GlEnum = GlEnum(0x9587);
/// 
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_OPAQUE_WIN32_KMT_EXT: GlEnum = GlEnum(0x9588);
/// 
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D12_TILEPOOL_EXT: GlEnum = GlEnum(0x9589);
/// 
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D12_RESOURCE_EXT: GlEnum = GlEnum(0x958A);
/// 
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D11_IMAGE_EXT: GlEnum = GlEnum(0x958B);
/// 
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D11_IMAGE_KMT_EXT: GlEnum = GlEnum(0x958C);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_GENERAL_EXT: GlEnum = GlEnum(0x958D);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_COLOR_ATTACHMENT_EXT: GlEnum = GlEnum(0x958E);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_DEPTH_STENCIL_ATTACHMENT_EXT: GlEnum = GlEnum(0x958F);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_DEPTH_STENCIL_READ_ONLY_EXT: GlEnum = GlEnum(0x9590);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_SHADER_READ_ONLY_EXT: GlEnum = GlEnum(0x9591);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_TRANSFER_SRC_EXT: GlEnum = GlEnum(0x9592);
/// 
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_TRANSFER_DST_EXT: GlEnum = GlEnum(0x9593);
/// 
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D12_FENCE_EXT: GlEnum = GlEnum(0x9594);
/// 
/// * Group: [`SemaphoreParameterName`]
pub const GL_D3D12_FENCE_VALUE_EXT: GlEnum = GlEnum(0x9595);
/// 
/// * Group: [`SemaphoreParameterName`]
pub const GL_TIMELINE_SEMAPHORE_VALUE_NV: GlEnum = GlEnum(0x9595);
/// 
/// * Group: [`GetPName`]
pub const GL_NUM_DEVICE_UUIDS_EXT: GlEnum = GlEnum(0x9596);
/// 
/// * Group: [`GetPName`]
pub const GL_DEVICE_UUID_EXT: GlEnum = GlEnum(0x9597);
/// 
/// * Group: [`GetPName`]
pub const GL_DRIVER_UUID_EXT: GlEnum = GlEnum(0x9598);
/// 
/// * Group: [`GetPName`]
pub const GL_DEVICE_LUID_EXT: GlEnum = GlEnum(0x9599);
/// 
/// * Group: [`GetPName`]
pub const GL_DEVICE_NODE_MASK_EXT: GlEnum = GlEnum(0x959A);
/// 
/// * Group: [`MemoryObjectParameterName`]
pub const GL_PROTECTED_MEMORY_OBJECT_EXT: GlEnum = GlEnum(0x959B);
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_MESH_SHADER_NV: GlEnum = GlEnum(0x959C);
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TASK_SHADER_NV: GlEnum = GlEnum(0x959D);
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_MESH_SHADER_NV: GlEnum = GlEnum(0x959E);
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TASK_SHADER_NV: GlEnum = GlEnum(0x959F);
pub const GL_REFERENCED_BY_MESH_SHADER_NV: GlEnum = GlEnum(0x95A0);
pub const GL_REFERENCED_BY_TASK_SHADER_NV: GlEnum = GlEnum(0x95A1);
pub const GL_MAX_MESH_WORK_GROUP_INVOCATIONS_NV: GlEnum = GlEnum(0x95A2);
pub const GL_MAX_TASK_WORK_GROUP_INVOCATIONS_NV: GlEnum = GlEnum(0x95A3);
pub const GL_ATTACHED_MEMORY_OBJECT_NV: GlEnum = GlEnum(0x95A4);
pub const GL_ATTACHED_MEMORY_OFFSET_NV: GlEnum = GlEnum(0x95A5);
pub const GL_MEMORY_ATTACHABLE_ALIGNMENT_NV: GlEnum = GlEnum(0x95A6);
pub const GL_MEMORY_ATTACHABLE_SIZE_NV: GlEnum = GlEnum(0x95A7);
pub const GL_MEMORY_ATTACHABLE_NV: GlEnum = GlEnum(0x95A8);
pub const GL_DETACHED_MEMORY_INCARNATION_NV: GlEnum = GlEnum(0x95A9);
pub const GL_DETACHED_TEXTURES_NV: GlEnum = GlEnum(0x95AA);
pub const GL_DETACHED_BUFFERS_NV: GlEnum = GlEnum(0x95AB);
pub const GL_MAX_DETACHED_TEXTURES_NV: GlEnum = GlEnum(0x95AC);
pub const GL_MAX_DETACHED_BUFFERS_NV: GlEnum = GlEnum(0x95AD);
pub const GL_SHADING_RATE_SAMPLE_ORDER_DEFAULT_NV: GlEnum = GlEnum(0x95AE);
pub const GL_SHADING_RATE_SAMPLE_ORDER_PIXEL_MAJOR_NV: GlEnum = GlEnum(0x95AF);
pub const GL_SHADING_RATE_SAMPLE_ORDER_SAMPLE_MAJOR_NV: GlEnum = GlEnum(0x95B0);
/// 
/// * Group: [`EnableCap`], [`GetPName`]
pub const GL_SHADING_RATE_IMAGE_PER_PRIMITIVE_NV: GlEnum = GlEnum(0x95B1);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADING_RATE_IMAGE_PALETTE_COUNT_NV: GlEnum = GlEnum(0x95B2);
/// 
/// * Group: [`SemaphoreParameterName`]
pub const GL_SEMAPHORE_TYPE_NV: GlEnum = GlEnum(0x95B3);
/// 
/// * Group: [`SemaphoreParameterName`]
pub const GL_SEMAPHORE_TYPE_BINARY_NV: GlEnum = GlEnum(0x95B4);
/// 
/// * Group: [`SemaphoreParameterName`]
pub const GL_SEMAPHORE_TYPE_TIMELINE_NV: GlEnum = GlEnum(0x95B5);
/// 
/// * Group: [`GetPName`]
pub const GL_MAX_TIMELINE_SEMAPHORE_VALUE_DIFFERENCE_NV: GlEnum = GlEnum(0x95B6);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR: GlEnum = GlEnum(0x9630);
pub const GL_MAX_VIEWS_OVR: GlEnum = GlEnum(0x9631);
/// 
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR: GlEnum = GlEnum(0x9632);
pub const GL_FRAMEBUFFER_INCOMPLETE_VIEW_TARGETS_OVR: GlEnum = GlEnum(0x9633);
pub const GL_GS_SHADER_BINARY_MTK: GlEnum = GlEnum(0x9640);
pub const GL_GS_PROGRAM_BINARY_MTK: GlEnum = GlEnum(0x9641);
pub const GL_MAX_SHADER_COMBINED_LOCAL_STORAGE_FAST_SIZE_EXT: GlEnum = GlEnum(0x9650);
pub const GL_MAX_SHADER_COMBINED_LOCAL_STORAGE_SIZE_EXT: GlEnum = GlEnum(0x9651);
pub const GL_FRAMEBUFFER_INCOMPLETE_INSUFFICIENT_SHADER_COMBINED_LOCAL_STORAGE_EXT: GlEnum = GlEnum(0x9652);
/// 
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_FOVEATED_CUTOFF_DENSITY_QCOM: GlEnum = GlEnum(0x96A0);
/// 
/// * Group: [`FramebufferFetchNoncoherent`]
pub const GL_FRAMEBUFFER_FETCH_NONCOHERENT_QCOM: GlEnum = GlEnum(0x96A2);
pub const GL_VALIDATE_SHADER_BINARY_QCOM: GlEnum = GlEnum(0x96A3);
/// 
/// * Group: [`GetPName`]
pub const GL_SHADING_RATE_QCOM: GlEnum = GlEnum(0x96A4);
/// 
/// * Group: [`EnableCap`]
pub const GL_SHADING_RATE_PRESERVE_ASPECT_RATIO_QCOM: GlEnum = GlEnum(0x96A5);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_1X1_PIXELS_QCOM: GlEnum = GlEnum(0x96A6);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_1X2_PIXELS_QCOM: GlEnum = GlEnum(0x96A7);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_2X1_PIXELS_QCOM: GlEnum = GlEnum(0x96A8);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_2X2_PIXELS_QCOM: GlEnum = GlEnum(0x96A9);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_1X4_PIXELS_QCOM: GlEnum = GlEnum(0x96AA);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_4X1_PIXELS_QCOM: GlEnum = GlEnum(0x96AB);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_4X2_PIXELS_QCOM: GlEnum = GlEnum(0x96AC);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_2X4_PIXELS_QCOM: GlEnum = GlEnum(0x96AD);
/// 
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_4X4_PIXELS_QCOM: GlEnum = GlEnum(0x96AE);
pub const GL_RASTER_POSITION_UNCLIPPED_IBM: GlEnum = GlEnum(0x19262);
pub const GL_CULL_VERTEX_IBM: GlEnum = GlEnum(103050);
pub const GL_ALL_STATIC_DATA_IBM: GlEnum = GlEnum(103060);
pub const GL_STATIC_VERTEX_ARRAY_IBM: GlEnum = GlEnum(103061);
pub const GL_VERTEX_ARRAY_LIST_IBM: GlEnum = GlEnum(103070);
pub const GL_NORMAL_ARRAY_LIST_IBM: GlEnum = GlEnum(103071);
pub const GL_COLOR_ARRAY_LIST_IBM: GlEnum = GlEnum(103072);
pub const GL_INDEX_ARRAY_LIST_IBM: GlEnum = GlEnum(103073);
pub const GL_TEXTURE_COORD_ARRAY_LIST_IBM: GlEnum = GlEnum(103074);
pub const GL_EDGE_FLAG_ARRAY_LIST_IBM: GlEnum = GlEnum(103075);
pub const GL_FOG_COORDINATE_ARRAY_LIST_IBM: GlEnum = GlEnum(103076);
pub const GL_SECONDARY_COLOR_ARRAY_LIST_IBM: GlEnum = GlEnum(103077);
pub const GL_VERTEX_ARRAY_LIST_STRIDE_IBM: GlEnum = GlEnum(103080);
pub const GL_NORMAL_ARRAY_LIST_STRIDE_IBM: GlEnum = GlEnum(103081);
pub const GL_COLOR_ARRAY_LIST_STRIDE_IBM: GlEnum = GlEnum(103082);
pub const GL_INDEX_ARRAY_LIST_STRIDE_IBM: GlEnum = GlEnum(103083);
pub const GL_TEXTURE_COORD_ARRAY_LIST_STRIDE_IBM: GlEnum = GlEnum(103084);
pub const GL_EDGE_FLAG_ARRAY_LIST_STRIDE_IBM: GlEnum = GlEnum(103085);
pub const GL_FOG_COORDINATE_ARRAY_LIST_STRIDE_IBM: GlEnum = GlEnum(103086);
pub const GL_SECONDARY_COLOR_ARRAY_LIST_STRIDE_IBM: GlEnum = GlEnum(103087);
/// 
/// * Group: [`HintTarget`]
pub const GL_PREFER_DOUBLEBUFFER_HINT_PGI: GlEnum = GlEnum(0x1A1F8);
/// 
/// * Group: [`HintTarget`]
pub const GL_CONSERVE_MEMORY_HINT_PGI: GlEnum = GlEnum(0x1A1FD);
/// 
/// * Group: [`HintTarget`]
pub const GL_RECLAIM_MEMORY_HINT_PGI: GlEnum = GlEnum(0x1A1FE);
pub const GL_NATIVE_GRAPHICS_HANDLE_PGI: GlEnum = GlEnum(0x1A202);
/// 
/// * Group: [`HintTarget`]
pub const GL_NATIVE_GRAPHICS_BEGIN_HINT_PGI: GlEnum = GlEnum(0x1A203);
/// 
/// * Group: [`HintTarget`]
pub const GL_NATIVE_GRAPHICS_END_HINT_PGI: GlEnum = GlEnum(0x1A204);
/// 
/// * Group: [`HintTarget`]
pub const GL_ALWAYS_FAST_HINT_PGI: GlEnum = GlEnum(0x1A20C);
/// 
/// * Group: [`HintTarget`]
pub const GL_ALWAYS_SOFT_HINT_PGI: GlEnum = GlEnum(0x1A20D);
/// 
/// * Group: [`HintTarget`]
pub const GL_ALLOW_DRAW_OBJ_HINT_PGI: GlEnum = GlEnum(0x1A20E);
/// 
/// * Group: [`HintTarget`]
pub const GL_ALLOW_DRAW_WIN_HINT_PGI: GlEnum = GlEnum(0x1A20F);
/// 
/// * Group: [`HintTarget`]
pub const GL_ALLOW_DRAW_FRG_HINT_PGI: GlEnum = GlEnum(0x1A210);
/// 
/// * Group: [`HintTarget`]
pub const GL_ALLOW_DRAW_MEM_HINT_PGI: GlEnum = GlEnum(0x1A211);
/// 
/// * Group: [`HintTarget`]
pub const GL_STRICT_DEPTHFUNC_HINT_PGI: GlEnum = GlEnum(0x1A216);
/// 
/// * Group: [`HintTarget`]
pub const GL_STRICT_LIGHTING_HINT_PGI: GlEnum = GlEnum(0x1A217);
/// 
/// * Group: [`HintTarget`]
pub const GL_STRICT_SCISSOR_HINT_PGI: GlEnum = GlEnum(0x1A218);
/// 
/// * Group: [`HintTarget`]
pub const GL_FULL_STIPPLE_HINT_PGI: GlEnum = GlEnum(0x1A219);
/// 
/// * Group: [`HintTarget`]
pub const GL_CLIP_NEAR_HINT_PGI: GlEnum = GlEnum(0x1A220);
/// 
/// * Group: [`HintTarget`]
pub const GL_CLIP_FAR_HINT_PGI: GlEnum = GlEnum(0x1A221);
/// 
/// * Group: [`HintTarget`]
pub const GL_WIDE_LINE_HINT_PGI: GlEnum = GlEnum(0x1A222);
/// 
/// * Group: [`HintTarget`]
pub const GL_BACK_NORMALS_HINT_PGI: GlEnum = GlEnum(0x1A223);
/// 
/// * Group: [`HintTarget`], [`HintTargetPGI`]
pub const GL_VERTEX_DATA_HINT_PGI: GlEnum = GlEnum(0x1A22A);
/// 
/// * Group: [`HintTarget`], [`HintTargetPGI`]
pub const GL_VERTEX_CONSISTENT_HINT_PGI: GlEnum = GlEnum(0x1A22B);
/// 
/// * Group: [`HintTarget`], [`HintTargetPGI`]
pub const GL_MATERIAL_SIDE_HINT_PGI: GlEnum = GlEnum(0x1A22C);
/// 
/// * Group: [`HintTarget`], [`HintTargetPGI`]
pub const GL_MAX_VERTEX_HINT_PGI: GlEnum = GlEnum(0x1A22D);
