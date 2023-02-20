use super::type_alias::*;

#[allow(non_camel_case_types)]
pub(crate) type glActiveShaderProgram_t =
  unsafe extern "system" fn(pipeline: GLuint, program: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glActiveTexture_t = unsafe extern "system" fn(texture: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glAttachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBeginConditionalRender_t = unsafe extern "system" fn(id: GLuint, mode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBeginQuery_t = unsafe extern "system" fn(target: GLenum, id: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBeginQueryIndexed_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, id: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBeginTransformFeedback_t = unsafe extern "system" fn(primitiveMode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBindAttribLocation_t =
  unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glBindBuffer_t = unsafe extern "system" fn(target: GLenum, buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindBufferBase_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindBufferRange_t = unsafe extern "system" fn(
  target: GLenum,
  index: GLuint,
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub(crate) type glBindBuffersBase_t =
  unsafe extern "system" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindBuffersRange_t = unsafe extern "system" fn(
  target: GLenum,
  first: GLuint,
  count: GLsizei,
  buffers: *const GLuint,
  offsets: *const GLintptr,
  sizes: *const GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub(crate) type glBindFragDataLocation_t =
  unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glBindFragDataLocationIndexed_t = unsafe extern "system" fn(
  program: GLuint,
  colorNumber: GLuint,
  index: GLuint,
  name: *const GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glBindFramebuffer_t =
  unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindImageTexture_t = unsafe extern "system" fn(
  unit: GLuint,
  texture: GLuint,
  level: GLint,
  layered: GLboolean,
  layer: GLint,
  access: GLenum,
  format: GLenum,
);
#[allow(non_camel_case_types)]
pub(crate) type glBindImageTextures_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindRenderbuffer_t =
  unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindSampler_t = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindSamplers_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindTexture_t = unsafe extern "system" fn(target: GLenum, texture: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindTextureUnit_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindTextures_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindTransformFeedback_t = unsafe extern "system" fn(target: GLenum, id: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindVertexArray_t = unsafe extern "system" fn(array: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glBindVertexBuffer_t = unsafe extern "system" fn(
  bindingindex: GLuint,
  buffer: GLuint,
  offset: GLintptr,
  stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glBindVertexBuffers_t = unsafe extern "system" fn(
  first: GLuint,
  count: GLsizei,
  buffers: *const GLuint,
  offsets: *const GLintptr,
  strides: *const GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glBlendColor_t =
  unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glBlendEquation_t = unsafe extern "system" fn(mode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBlendEquationSeparate_t =
  unsafe extern "system" fn(modeRGB: GLenum, modeAlpha: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBlendEquationSeparatei_t =
  unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBlendEquationi_t = unsafe extern "system" fn(buf: GLuint, mode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBlendFunc_t = unsafe extern "system" fn(sfactor: GLenum, dfactor: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBlendFuncSeparate_t = unsafe extern "system" fn(
  sfactorRGB: GLenum,
  dfactorRGB: GLenum,
  sfactorAlpha: GLenum,
  dfactorAlpha: GLenum,
);
#[allow(non_camel_case_types)]
pub(crate) type glBlendFuncSeparatei_t = unsafe extern "system" fn(
  buf: GLuint,
  srcRGB: GLenum,
  dstRGB: GLenum,
  srcAlpha: GLenum,
  dstAlpha: GLenum,
);
#[allow(non_camel_case_types)]
pub(crate) type glBlendFunci_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBlitFramebuffer_t = unsafe extern "system" fn(
  srcX0: GLint,
  srcY0: GLint,
  srcX1: GLint,
  srcY1: GLint,
  dstX0: GLint,
  dstY0: GLint,
  dstX1: GLint,
  dstY1: GLint,
  mask: GLbitfield,
  filter: GLenum,
);
#[allow(non_camel_case_types)]
pub(crate) type glBlitNamedFramebuffer_t = unsafe extern "system" fn(
  readFramebuffer: GLuint,
  drawFramebuffer: GLuint,
  srcX0: GLint,
  srcY0: GLint,
  srcX1: GLint,
  srcY1: GLint,
  dstX0: GLint,
  dstY0: GLint,
  dstX1: GLint,
  dstY1: GLint,
  mask: GLbitfield,
  filter: GLenum,
);
#[allow(non_camel_case_types)]
pub(crate) type glBufferData_t =
  unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glBufferStorage_t = unsafe extern "system" fn(
  target: GLenum,
  size: GLsizeiptr,
  data: *const c_void,
  flags: GLbitfield,
);
#[allow(non_camel_case_types)]
pub(crate) type glBufferSubData_t = unsafe extern "system" fn(
  target: GLenum,
  offset: GLintptr,
  size: GLsizeiptr,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCheckFramebufferStatus_t = unsafe extern "system" fn(target: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glCheckNamedFramebufferStatus_t =
  unsafe extern "system" fn(framebuffer: GLuint, target: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glClampColor_t = unsafe extern "system" fn(target: GLenum, clamp: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glClear_t = unsafe extern "system" fn(mask: GLbitfield);
#[allow(non_camel_case_types)]
pub(crate) type glClearBufferData_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearBufferSubData_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  offset: GLintptr,
  size: GLsizeiptr,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearBufferfi_t =
  unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glClearBufferfv_t =
  unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glClearBufferiv_t =
  unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glClearBufferuiv_t =
  unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glClearColor_t =
  unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glClearDepth_t = unsafe extern "system" fn(depth: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glClearDepthf_t = unsafe extern "system" fn(d: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glClearNamedBufferData_t = unsafe extern "system" fn(
  buffer: GLuint,
  internalformat: GLenum,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearNamedBufferSubData_t = unsafe extern "system" fn(
  buffer: GLuint,
  internalformat: GLenum,
  offset: GLintptr,
  size: GLsizeiptr,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearNamedFramebufferfi_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  buffer: GLenum,
  drawbuffer: GLint,
  depth: GLfloat,
  stencil: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearNamedFramebufferfv_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  buffer: GLenum,
  drawbuffer: GLint,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearNamedFramebufferiv_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  buffer: GLenum,
  drawbuffer: GLint,
  value: *const GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearNamedFramebufferuiv_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  buffer: GLenum,
  drawbuffer: GLint,
  value: *const GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearStencil_t = unsafe extern "system" fn(s: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glClearTexImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glClearTexSubImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glClientWaitSync_t =
  unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
#[allow(non_camel_case_types)]
pub(crate) type glClipControl_t = unsafe extern "system" fn(origin: GLenum, depth: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glColorMask_t =
  unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
#[allow(non_camel_case_types)]
pub(crate) type glColorMaski_t =
  unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
#[allow(non_camel_case_types)]
pub(crate) type glColorP3ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glColorP3uiv_t = unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glColorP4ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glColorP4uiv_t = unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCompileShader_t = unsafe extern "system" fn(shader: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTexImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  width: GLsizei,
  border: GLint,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTexImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  border: GLint,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTexImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  border: GLint,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTexSubImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  width: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTexSubImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTexSubImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTextureSubImage1D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  width: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTextureSubImage2D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCompressedTextureSubImage3D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyBufferSubData_t = unsafe extern "system" fn(
  readTarget: GLenum,
  writeTarget: GLenum,
  readOffset: GLintptr,
  writeOffset: GLintptr,
  size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyImageSubData_t = unsafe extern "system" fn(
  srcName: GLuint,
  srcTarget: GLenum,
  srcLevel: GLint,
  srcX: GLint,
  srcY: GLint,
  srcZ: GLint,
  dstName: GLuint,
  dstTarget: GLenum,
  dstLevel: GLint,
  dstX: GLint,
  dstY: GLint,
  dstZ: GLint,
  srcWidth: GLsizei,
  srcHeight: GLsizei,
  srcDepth: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyNamedBufferSubData_t = unsafe extern "system" fn(
  readBuffer: GLuint,
  writeBuffer: GLuint,
  readOffset: GLintptr,
  writeOffset: GLintptr,
  size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyTexImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  x: GLint,
  y: GLint,
  width: GLsizei,
  border: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyTexImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
  border: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyTexSubImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyTexSubImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyTexSubImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyTextureSubImage1D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyTextureSubImage2D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glCopyTextureSubImage3D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glCreateBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCreateFramebuffers_t =
  unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCreateProgram_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glCreateProgramPipelines_t =
  unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCreateQueries_t =
  unsafe extern "system" fn(target: GLenum, n: GLsizei, ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCreateRenderbuffers_t =
  unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCreateSamplers_t = unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCreateShader_t = unsafe extern "system" fn(ty: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glCreateShaderProgramv_t =
  unsafe extern "system" fn(ty: GLenum, count: GLsizei, strings: *const *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glCreateTextures_t =
  unsafe extern "system" fn(target: GLenum, n: GLsizei, textures: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCreateTransformFeedbacks_t =
  unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCreateVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glCullFace_t = unsafe extern "system" fn(mode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glDebugMessageCallback_t =
  unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const c_void);
#[allow(non_camel_case_types)]
pub(crate) type glDebugMessageControl_t = unsafe extern "system" fn(
  source: GLenum,
  ty: GLenum,
  severity: GLenum,
  count: GLsizei,
  ids: *const GLuint,
  enabled: GLboolean,
);
#[allow(non_camel_case_types)]
pub(crate) type glDebugMessageInsert_t = unsafe extern "system" fn(
  source: GLenum,
  ty: GLenum,
  id: GLuint,
  severity: GLenum,
  length: GLsizei,
  buf: *const GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteFramebuffers_t =
  unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteProgram_t = unsafe extern "system" fn(program: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteProgramPipelines_t =
  unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteRenderbuffers_t =
  unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteSamplers_t =
  unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteShader_t = unsafe extern "system" fn(shader: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteSync_t = unsafe extern "system" fn(sync: GLsync);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteTransformFeedbacks_t =
  unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDeleteVertexArrays_t =
  unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDepthFunc_t = unsafe extern "system" fn(func: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glDepthMask_t = unsafe extern "system" fn(flag: GLboolean);
#[allow(non_camel_case_types)]
pub(crate) type glDepthRange_t = unsafe extern "system" fn(n: GLdouble, f: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glDepthRangeArrayv_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glDepthRangeIndexed_t =
  unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glDepthRangef_t = unsafe extern "system" fn(n: GLfloat, f: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glDetachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDisable_t = unsafe extern "system" fn(cap: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glDisableVertexArrayAttrib_t =
  unsafe extern "system" fn(vaobj: GLuint, index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDisableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDisablei_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDispatchCompute_t =
  unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDispatchComputeIndirect_t = unsafe extern "system" fn(indirect: GLintptr);
#[allow(non_camel_case_types)]
pub(crate) type glDrawArrays_t =
  unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);
#[allow(non_camel_case_types)]
pub(crate) type glDrawArraysIndirect_t =
  unsafe extern "system" fn(mode: GLenum, indirect: *const c_void);
#[allow(non_camel_case_types)]
pub(crate) type glDrawArraysInstanced_t =
  unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
#[allow(non_camel_case_types)]
pub(crate) type glDrawArraysInstancedBaseInstance_t = unsafe extern "system" fn(
  mode: GLenum,
  first: GLint,
  count: GLsizei,
  instancecount: GLsizei,
  baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glDrawBuffer_t = unsafe extern "system" fn(buf: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glDrawBuffers_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glDrawElements_t =
  unsafe extern "system" fn(mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void);
#[allow(non_camel_case_types)]
pub(crate) type glDrawElementsBaseVertex_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glDrawElementsIndirect_t =
  unsafe extern "system" fn(mode: GLenum, ty: GLenum, indirect: *const c_void);
#[allow(non_camel_case_types)]
pub(crate) type glDrawElementsInstanced_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  instancecount: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glDrawElementsInstancedBaseInstance_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  instancecount: GLsizei,
  baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glDrawElementsInstancedBaseVertex_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  instancecount: GLsizei,
  basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glDrawElementsInstancedBaseVertexBaseInstance_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  instancecount: GLsizei,
  basevertex: GLint,
  baseinstance: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glDrawRangeElements_t = unsafe extern "system" fn(
  mode: GLenum,
  start: GLuint,
  end: GLuint,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glDrawRangeElementsBaseVertex_t = unsafe extern "system" fn(
  mode: GLenum,
  start: GLuint,
  end: GLuint,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  basevertex: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glDrawTransformFeedback_t = unsafe extern "system" fn(mode: GLenum, id: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDrawTransformFeedbackInstanced_t =
  unsafe extern "system" fn(mode: GLenum, id: GLuint, instancecount: GLsizei);
#[allow(non_camel_case_types)]
pub(crate) type glDrawTransformFeedbackStream_t =
  unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glDrawTransformFeedbackStreamInstanced_t =
  unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei);
#[allow(non_camel_case_types)]
pub(crate) type glEnable_t = unsafe extern "system" fn(cap: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glEnableVertexArrayAttrib_t =
  unsafe extern "system" fn(vaobj: GLuint, index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glEnableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glEnablei_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glEndConditionalRender_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glEndQuery_t = unsafe extern "system" fn(target: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glEndQueryIndexed_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glEndTransformFeedback_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glFenceSync_t = unsafe extern "system" fn(condition: GLenum, flags: GLbitfield);
#[allow(non_camel_case_types)]
pub(crate) type glFinish_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glFlush_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glFlushMappedBufferRange_t =
  unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr);
#[allow(non_camel_case_types)]
pub(crate) type glFlushMappedNamedBufferRange_t =
  unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
#[allow(non_camel_case_types)]
pub(crate) type glFramebufferParameteri_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glFramebufferRenderbuffer_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  renderbuffertarget: GLenum,
  renderbuffer: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glFramebufferTexture_t =
  unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glFramebufferTexture1D_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  textarget: GLenum,
  texture: GLuint,
  level: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glFramebufferTexture2D_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  textarget: GLenum,
  texture: GLuint,
  level: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glFramebufferTexture3D_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  textarget: GLenum,
  texture: GLuint,
  level: GLint,
  zoffset: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glFramebufferTextureLayer_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  texture: GLuint,
  level: GLint,
  layer: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glFrontFace_t = unsafe extern "system" fn(mode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glGenBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenFramebuffers_t =
  unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenProgramPipelines_t =
  unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenRenderbuffers_t =
  unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGenerateMipmap_t = unsafe extern "system" fn(target: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glGenerateTextureMipmap_t = unsafe extern "system" fn(texture: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveAtomicCounterBufferiv_t = unsafe extern "system" fn(
  program: GLuint,
  bufferIndex: GLuint,
  pname: GLenum,
  params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveAttrib_t = unsafe extern "system" fn(
  program: GLuint,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  size: *mut GLint,
  ty: *mut GLenum,
  name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveSubroutineName_t = unsafe extern "system" fn(
  program: GLuint,
  shadertype: GLenum,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveSubroutineUniformName_t = unsafe extern "system" fn(
  program: GLuint,
  shadertype: GLenum,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveSubroutineUniformiv_t = unsafe extern "system" fn(
  program: GLuint,
  shadertype: GLenum,
  index: GLuint,
  pname: GLenum,
  values: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveUniform_t = unsafe extern "system" fn(
  program: GLuint,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  size: *mut GLint,
  ty: *mut GLenum,
  name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveUniformBlockName_t = unsafe extern "system" fn(
  program: GLuint,
  uniformBlockIndex: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  uniformBlockName: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveUniformBlockiv_t = unsafe extern "system" fn(
  program: GLuint,
  uniformBlockIndex: GLuint,
  pname: GLenum,
  params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveUniformName_t = unsafe extern "system" fn(
  program: GLuint,
  uniformIndex: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  uniformName: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetActiveUniformsiv_t = unsafe extern "system" fn(
  program: GLuint,
  uniformCount: GLsizei,
  uniformIndices: *const GLuint,
  pname: GLenum,
  params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetAttachedShaders_t = unsafe extern "system" fn(
  program: GLuint,
  maxCount: GLsizei,
  count: *mut GLsizei,
  shaders: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetAttribLocation_t =
  unsafe extern "system" fn(program: GLuint, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetBooleani_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub(crate) type glGetBooleanv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub(crate) type glGetBufferParameteri64v_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint64);
#[allow(non_camel_case_types)]
pub(crate) type glGetBufferParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetBufferPointerv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetBufferSubData_t =
  unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetCompressedTexImage_t =
  unsafe extern "system" fn(target: GLenum, level: GLint, img: *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetCompressedTextureImage_t =
  unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetCompressedTextureSubImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  bufSize: GLsizei,
  pixels: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetDebugMessageLog_t = unsafe extern "system" fn(
  count: GLuint,
  bufSize: GLsizei,
  sources: *mut GLenum,
  types: *mut GLenum,
  ids: *mut GLuint,
  severities: *mut GLenum,
  lengths: *mut GLsizei,
  messageLog: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetDoublei_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glGetDoublev_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glGetError_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glGetFloati_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetFloatv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetFragDataIndex_t =
  unsafe extern "system" fn(program: GLuint, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetFragDataLocation_t =
  unsafe extern "system" fn(program: GLuint, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetFramebufferAttachmentParameteriv_t =
  unsafe extern "system" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetFramebufferParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetGraphicsResetStatus_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glGetInteger64i_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint64);
#[allow(non_camel_case_types)]
pub(crate) type glGetInteger64v_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint64);
#[allow(non_camel_case_types)]
pub(crate) type glGetIntegeri_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetIntegerv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetInternalformati64v_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  pname: GLenum,
  count: GLsizei,
  params: *mut GLint64,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetInternalformativ_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  pname: GLenum,
  count: GLsizei,
  params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetMultisamplefv_t =
  unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetNamedBufferParameteri64v_t =
  unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint64);
#[allow(non_camel_case_types)]
pub(crate) type glGetNamedBufferParameteriv_t =
  unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetNamedBufferPointerv_t =
  unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetNamedBufferSubData_t =
  unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetNamedFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  attachment: GLenum,
  pname: GLenum,
  params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetNamedFramebufferParameteriv_t =
  unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetNamedRenderbufferParameteriv_t =
  unsafe extern "system" fn(renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetObjectLabel_t = unsafe extern "system" fn(
  identifier: GLenum,
  name: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  label: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetObjectPtrLabel_t = unsafe extern "system" fn(
  ptr: *const c_void,
  bufSize: GLsizei,
  length: *mut GLsizei,
  label: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetPointerv_t =
  unsafe extern "system" fn(pname: GLenum, params: *mut *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramBinary_t = unsafe extern "system" fn(
  program: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  binaryFormat: *mut GLenum,
  binary: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramInfoLog_t = unsafe extern "system" fn(
  program: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  infoLog: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramInterfaceiv_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  pname: GLenum,
  params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramPipelineInfoLog_t = unsafe extern "system" fn(
  pipeline: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  infoLog: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramPipelineiv_t =
  unsafe extern "system" fn(pipeline: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramResourceIndex_t =
  unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramResourceLocation_t =
  unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramResourceLocationIndex_t =
  unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramResourceName_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramResourceiv_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  index: GLuint,
  propCount: GLsizei,
  props: *const GLenum,
  count: GLsizei,
  length: *mut GLsizei,
  params: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramStageiv_t =
  unsafe extern "system" fn(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetProgramiv_t =
  unsafe extern "system" fn(program: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryBufferObjecti64v_t =
  unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryBufferObjectiv_t =
  unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryBufferObjectui64v_t =
  unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryBufferObjectuiv_t =
  unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryIndexediv_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryObjecti64v_t =
  unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint64);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryObjectiv_t =
  unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryObjectui64v_t =
  unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint64);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryObjectuiv_t =
  unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetQueryiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetRenderbufferParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetSamplerParameterIiv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetSamplerParameterIuiv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetSamplerParameterfv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetSamplerParameteriv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetShaderInfoLog_t = unsafe extern "system" fn(
  shader: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  infoLog: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetShaderPrecisionFormat_t = unsafe extern "system" fn(
  shadertype: GLenum,
  precisiontype: GLenum,
  range: *mut GLint,
  precision: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetShaderSource_t = unsafe extern "system" fn(
  shader: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  source: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetShaderiv_t =
  unsafe extern "system" fn(shader: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetString_t = unsafe extern "system" fn(name: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glGetStringi_t = unsafe extern "system" fn(name: GLenum, index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetSubroutineIndex_t =
  unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetSubroutineUniformLocation_t =
  unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetSynciv_t = unsafe extern "system" fn(
  sync: GLsync,
  pname: GLenum,
  count: GLsizei,
  length: *mut GLsizei,
  values: *mut GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetTexImage_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  format: GLenum,
  ty: GLenum,
  pixels: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetTexLevelParameterfv_t =
  unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetTexLevelParameteriv_t =
  unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTexParameterIiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTexParameterIuiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTexParameterfv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetTexParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTextureImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  pixels: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetTextureLevelParameterfv_t =
  unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetTextureLevelParameteriv_t =
  unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTextureParameterIiv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTextureParameterIuiv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTextureParameterfv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetTextureParameteriv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTextureSubImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  pixels: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetTransformFeedbackVarying_t = unsafe extern "system" fn(
  program: GLuint,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  size: *mut GLsizei,
  ty: *mut GLenum,
  name: *mut GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetTransformFeedbacki64_v_t =
  unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64);
#[allow(non_camel_case_types)]
pub(crate) type glGetTransformFeedbacki_v_t =
  unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetTransformFeedbackiv_t =
  unsafe extern "system" fn(xfb: GLuint, pname: GLenum, param: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetUniformBlockIndex_t =
  unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetUniformIndices_t = unsafe extern "system" fn(
  program: GLuint,
  uniformCount: GLsizei,
  uniformNames: *const *const GLchar,
  uniformIndices: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetUniformLocation_t =
  unsafe extern "system" fn(program: GLuint, name: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glGetUniformSubroutineuiv_t =
  unsafe extern "system" fn(shadertype: GLenum, location: GLint, params: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetUniformdv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glGetUniformfv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetUniformiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetUniformuiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexArrayIndexed64iv_t =
  unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexArrayIndexediv_t =
  unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexArrayiv_t =
  unsafe extern "system" fn(vaobj: GLuint, pname: GLenum, param: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexAttribIiv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexAttribIuiv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexAttribLdv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexAttribPointerv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexAttribdv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexAttribfv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetVertexAttribiv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetnColorTable_t = unsafe extern "system" fn(
  target: GLenum,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  table: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetnCompressedTexImage_t =
  unsafe extern "system" fn(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void);
#[allow(non_camel_case_types)]
pub(crate) type glGetnConvolutionFilter_t = unsafe extern "system" fn(
  target: GLenum,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  image: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetnHistogram_t = unsafe extern "system" fn(
  target: GLenum,
  reset: GLboolean,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  values: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetnMapdv_t =
  unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glGetnMapfv_t =
  unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetnMapiv_t =
  unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetnMinmax_t = unsafe extern "system" fn(
  target: GLenum,
  reset: GLboolean,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  values: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetnPixelMapfv_t =
  unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glGetnPixelMapuiv_t =
  unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glGetnPixelMapusv_t =
  unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLushort);
#[allow(non_camel_case_types)]
pub(crate) type glGetnPolygonStipple_t =
  unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);
#[allow(non_camel_case_types)]
pub(crate) type glGetnSeparableFilter_t = unsafe extern "system" fn(
  target: GLenum,
  format: GLenum,
  ty: GLenum,
  rowBufSize: GLsizei,
  row: *mut c_void,
  columnBufSize: GLsizei,
  column: *mut c_void,
  span: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetnTexImage_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  pixels: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetnUniformdv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  bufSize: GLsizei,
  params: *mut GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetnUniformfv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  bufSize: GLsizei,
  params: *mut GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glGetnUniformiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);
#[allow(non_camel_case_types)]
pub(crate) type glGetnUniformuiv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  bufSize: GLsizei,
  params: *mut GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glHint_t = unsafe extern "system" fn(target: GLenum, mode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glInvalidateBufferData_t = unsafe extern "system" fn(buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glInvalidateBufferSubData_t =
  unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
#[allow(non_camel_case_types)]
pub(crate) type glInvalidateFramebuffer_t =
  unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glInvalidateNamedFramebufferData_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  numAttachments: GLsizei,
  attachments: *const GLenum,
);
#[allow(non_camel_case_types)]
pub(crate) type glInvalidateNamedFramebufferSubData_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  numAttachments: GLsizei,
  attachments: *const GLenum,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glInvalidateSubFramebuffer_t = unsafe extern "system" fn(
  target: GLenum,
  numAttachments: GLsizei,
  attachments: *const GLenum,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glInvalidateTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glInvalidateTexSubImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glIsBuffer_t = unsafe extern "system" fn(buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsEnabled_t = unsafe extern "system" fn(cap: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glIsEnabledi_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsFramebuffer_t = unsafe extern "system" fn(framebuffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsProgram_t = unsafe extern "system" fn(program: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsQuery_t = unsafe extern "system" fn(id: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsRenderbuffer_t = unsafe extern "system" fn(renderbuffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsSampler_t = unsafe extern "system" fn(sampler: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsShader_t = unsafe extern "system" fn(shader: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsSync_t = unsafe extern "system" fn(sync: GLsync);
#[allow(non_camel_case_types)]
pub(crate) type glIsTexture_t = unsafe extern "system" fn(texture: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsTransformFeedback_t = unsafe extern "system" fn(id: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glIsVertexArray_t = unsafe extern "system" fn(array: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glLineWidth_t = unsafe extern "system" fn(width: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glLinkProgram_t = unsafe extern "system" fn(program: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glLogicOp_t = unsafe extern "system" fn(opcode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glMapBuffer_t = unsafe extern "system" fn(target: GLenum, access: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glMapBufferRange_t = unsafe extern "system" fn(
  target: GLenum,
  offset: GLintptr,
  length: GLsizeiptr,
  access: GLbitfield,
);
#[allow(non_camel_case_types)]
pub(crate) type glMapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint, access: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glMapNamedBufferRange_t = unsafe extern "system" fn(
  buffer: GLuint,
  offset: GLintptr,
  length: GLsizeiptr,
  access: GLbitfield,
);
#[allow(non_camel_case_types)]
pub(crate) type glMemoryBarrier_t = unsafe extern "system" fn(barriers: GLbitfield);
#[allow(non_camel_case_types)]
pub(crate) type glMemoryBarrierByRegion_t = unsafe extern "system" fn(barriers: GLbitfield);
#[allow(non_camel_case_types)]
pub(crate) type glMinSampleShading_t = unsafe extern "system" fn(value: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glMultiDrawArrays_t = unsafe extern "system" fn(
  mode: GLenum,
  first: *const GLint,
  count: *const GLsizei,
  drawcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glMultiDrawArraysIndirect_t = unsafe extern "system" fn(
  mode: GLenum,
  indirect: *const c_void,
  drawcount: GLsizei,
  stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glMultiDrawArraysIndirectCount_t = unsafe extern "system" fn(
  mode: GLenum,
  indirect: *const c_void,
  drawcount: GLintptr,
  maxdrawcount: GLsizei,
  stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glMultiDrawElements_t = unsafe extern "system" fn(
  mode: GLenum,
  count: *const GLsizei,
  ty: GLenum,
  indices: *const *const c_void,
  drawcount: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glMultiDrawElementsBaseVertex_t = unsafe extern "system" fn(
  mode: GLenum,
  count: *const GLsizei,
  ty: GLenum,
  indices: *const *const c_void,
  drawcount: GLsizei,
  basevertex: *const GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glMultiDrawElementsIndirect_t = unsafe extern "system" fn(
  mode: GLenum,
  ty: GLenum,
  indirect: *const c_void,
  drawcount: GLsizei,
  stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glMultiDrawElementsIndirectCount_t = unsafe extern "system" fn(
  mode: GLenum,
  ty: GLenum,
  indirect: *const c_void,
  drawcount: GLintptr,
  maxdrawcount: GLsizei,
  stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glMultiTexCoordP1ui_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glMultiTexCoordP1uiv_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glMultiTexCoordP2ui_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glMultiTexCoordP2uiv_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glMultiTexCoordP3ui_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glMultiTexCoordP3uiv_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glMultiTexCoordP4ui_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glMultiTexCoordP4uiv_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glNamedBufferData_t =
  unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glNamedBufferStorage_t = unsafe extern "system" fn(
  buffer: GLuint,
  size: GLsizeiptr,
  data: *const c_void,
  flags: GLbitfield,
);
#[allow(non_camel_case_types)]
pub(crate) type glNamedBufferSubData_t = unsafe extern "system" fn(
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
  data: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glNamedFramebufferDrawBuffer_t =
  unsafe extern "system" fn(framebuffer: GLuint, buf: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glNamedFramebufferDrawBuffers_t =
  unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glNamedFramebufferParameteri_t =
  unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glNamedFramebufferReadBuffer_t =
  unsafe extern "system" fn(framebuffer: GLuint, src: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glNamedFramebufferRenderbuffer_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  attachment: GLenum,
  renderbuffertarget: GLenum,
  renderbuffer: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glNamedFramebufferTexture_t =
  unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glNamedFramebufferTextureLayer_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  attachment: GLenum,
  texture: GLuint,
  level: GLint,
  layer: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glNamedRenderbufferStorage_t = unsafe extern "system" fn(
  renderbuffer: GLuint,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glNamedRenderbufferStorageMultisample_t = unsafe extern "system" fn(
  renderbuffer: GLuint,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glNormalP3ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glNormalP3uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glObjectLabel_t = unsafe extern "system" fn(
  identifier: GLenum,
  name: GLuint,
  length: GLsizei,
  label: *const GLchar,
);
#[allow(non_camel_case_types)]
pub(crate) type glObjectPtrLabel_t =
  unsafe extern "system" fn(ptr: *const c_void, length: GLsizei, label: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glPatchParameterfv_t =
  unsafe extern "system" fn(pname: GLenum, values: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glPatchParameteri_t = unsafe extern "system" fn(pname: GLenum, value: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glPauseTransformFeedback_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glPixelStoref_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glPixelStorei_t = unsafe extern "system" fn(pname: GLenum, param: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glPointParameterf_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glPointParameterfv_t =
  unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glPointParameteri_t = unsafe extern "system" fn(pname: GLenum, param: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glPointParameteriv_t =
  unsafe extern "system" fn(pname: GLenum, params: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glPointSize_t = unsafe extern "system" fn(size: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glPolygonMode_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glPolygonOffset_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glPolygonOffsetClamp_t =
  unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glPopDebugGroup_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glPrimitiveRestartIndex_t = unsafe extern "system" fn(index: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramBinary_t = unsafe extern "system" fn(
  program: GLuint,
  binaryFormat: GLenum,
  binary: *const c_void,
  length: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramParameteri_t =
  unsafe extern "system" fn(program: GLuint, pname: GLenum, value: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform1d_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform1dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform1f_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform1fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform1i_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform1iv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform1ui_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform1uiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform2d_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform2dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform2f_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform2fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform2i_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform2iv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform2ui_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform2uiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform3d_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLdouble,
  v1: GLdouble,
  v2: GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform3dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform3f_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLfloat,
  v1: GLfloat,
  v2: GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform3fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform3i_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform3iv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform3ui_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform3uiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform4d_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLdouble,
  v1: GLdouble,
  v2: GLdouble,
  v3: GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform4dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform4f_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLfloat,
  v1: GLfloat,
  v2: GLfloat,
  v3: GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform4fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform4i_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLint,
  v1: GLint,
  v2: GLint,
  v3: GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform4iv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform4ui_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLuint,
  v1: GLuint,
  v2: GLuint,
  v3: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniform4uiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix2dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix2fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix2x3dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix2x3fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix2x4dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix2x4fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix3dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix3fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix3x2dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix3x2fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix3x4dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix3x4fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix4dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix4fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix4x2dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix4x2fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix4x3dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glProgramUniformMatrix4x3fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glProvokingVertex_t = unsafe extern "system" fn(mode: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glPushDebugGroup_t =
  unsafe extern "system" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
#[allow(non_camel_case_types)]
pub(crate) type glQueryCounter_t = unsafe extern "system" fn(id: GLuint, target: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glReadBuffer_t = unsafe extern "system" fn(src: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glReadPixels_t = unsafe extern "system" fn(
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glReadnPixels_t = unsafe extern "system" fn(
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  data: *mut c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glReleaseShaderCompiler_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glRenderbufferStorage_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glRenderbufferStorageMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glResumeTransformFeedback_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glSampleCoverage_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);
#[allow(non_camel_case_types)]
pub(crate) type glSampleMaski_t = unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield);
#[allow(non_camel_case_types)]
pub(crate) type glSamplerParameterIiv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glSamplerParameterIuiv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glSamplerParameterf_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glSamplerParameterfv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glSamplerParameteri_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glSamplerParameteriv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glScissor_t =
  unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
#[allow(non_camel_case_types)]
pub(crate) type glScissorArrayv_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glScissorIndexed_t = unsafe extern "system" fn(
  index: GLuint,
  left: GLint,
  bottom: GLint,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glScissorIndexedv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glSecondaryColorP3ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glSecondaryColorP3uiv_t =
  unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glShaderBinary_t = unsafe extern "system" fn(
  count: GLsizei,
  shaders: *const GLuint,
  binaryFormat: GLenum,
  binary: *const c_void,
  length: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glShaderSource_t = unsafe extern "system" fn(
  shader: GLuint,
  count: GLsizei,
  string: *const *const GLchar,
  length: *const GLint,
);
#[allow(non_camel_case_types)]
pub(crate) type glShaderStorageBlockBinding_t = unsafe extern "system" fn(
  program: GLuint,
  storageBlockIndex: GLuint,
  storageBlockBinding: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glSpecializeShader_t = unsafe extern "system" fn(
  shader: GLuint,
  pEntryPoint: *const GLchar,
  numSpecializationConstants: GLuint,
  pConstantIndex: *const GLuint,
  pConstantValue: *const GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glStencilFunc_t =
  unsafe extern "system" fn(func: GLenum, reference: GLint, mask: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glStencilFuncSeparate_t =
  unsafe extern "system" fn(face: GLenum, func: GLenum, reference: GLint, mask: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glStencilMask_t = unsafe extern "system" fn(mask: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glStencilMaskSeparate_t = unsafe extern "system" fn(face: GLenum, mask: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glStencilOp_t =
  unsafe extern "system" fn(fail: GLenum, zfail: GLenum, zpass: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glStencilOpSeparate_t =
  unsafe extern "system" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glTexBuffer_t =
  unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexBufferRange_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexCoordP1ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexCoordP1uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexCoordP2ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexCoordP2uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexCoordP3ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexCoordP3uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexCoordP4ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexCoordP4uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLint,
  width: GLsizei,
  border: GLint,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLint,
  width: GLsizei,
  height: GLsizei,
  border: GLint,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexImage2DMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  border: GLint,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexImage3DMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexParameterIiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glTexParameterIuiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTexParameterf_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glTexParameterfv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glTexParameteri_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glTexParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glTexStorage1D_t = unsafe extern "system" fn(
  target: GLenum,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexStorage2D_t = unsafe extern "system" fn(
  target: GLenum,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexStorage2DMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexStorage3D_t = unsafe extern "system" fn(
  target: GLenum,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexStorage3DMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexSubImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  width: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexSubImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTexSubImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureBarrier_t = unsafe extern "system" fn();
#[allow(non_camel_case_types)]
pub(crate) type glTextureBuffer_t =
  unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTextureBufferRange_t = unsafe extern "system" fn(
  texture: GLuint,
  internalformat: GLenum,
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureParameterIiv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glTextureParameterIuiv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTextureParameterf_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glTextureParameterfv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glTextureParameteri_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glTextureParameteriv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glTextureStorage1D_t = unsafe extern "system" fn(
  texture: GLuint,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureStorage2D_t = unsafe extern "system" fn(
  texture: GLuint,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureStorage2DMultisample_t = unsafe extern "system" fn(
  texture: GLuint,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureStorage3D_t = unsafe extern "system" fn(
  texture: GLuint,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureStorage3DMultisample_t = unsafe extern "system" fn(
  texture: GLuint,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  fixedsamplelocations: GLboolean,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureSubImage1D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  width: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureSubImage2D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureSubImage3D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glTextureView_t = unsafe extern "system" fn(
  texture: GLuint,
  target: GLenum,
  origtexture: GLuint,
  internalformat: GLenum,
  minlevel: GLuint,
  numlevels: GLuint,
  minlayer: GLuint,
  numlayers: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glTransformFeedbackBufferBase_t =
  unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glTransformFeedbackBufferRange_t = unsafe extern "system" fn(
  xfb: GLuint,
  index: GLuint,
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
);
#[allow(non_camel_case_types)]
pub(crate) type glTransformFeedbackVaryings_t = unsafe extern "system" fn(
  program: GLuint,
  count: GLsizei,
  varyings: *const *const GLchar,
  bufferMode: GLenum,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniform1d_t = unsafe extern "system" fn(location: GLint, x: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glUniform1dv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glUniform1f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glUniform1fv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glUniform1i_t = unsafe extern "system" fn(location: GLint, v0: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform1iv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform1ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform1uiv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform2d_t =
  unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glUniform2dv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glUniform2f_t =
  unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glUniform2fv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glUniform2i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform2iv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform2ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform2uiv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform3d_t =
  unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glUniform3dv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glUniform3f_t =
  unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glUniform3fv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glUniform3i_t =
  unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform3iv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform3ui_t =
  unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform3uiv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform4d_t =
  unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glUniform4dv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glUniform4f_t =
  unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glUniform4fv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glUniform4i_t =
  unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform4iv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform4ui_t =
  unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUniform4uiv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUniformBlockBinding_t = unsafe extern "system" fn(
  program: GLuint,
  uniformBlockIndex: GLuint,
  uniformBlockBinding: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix2dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix2fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix2x3dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix2x3fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix2x4dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix2x4fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix3dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix3fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix3x2dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix3x2fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix3x4dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix3x4fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix4dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix4fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix4x2dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix4x2fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix4x3dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformMatrix4x3fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
#[allow(non_camel_case_types)]
pub(crate) type glUniformSubroutinesuiv_t =
  unsafe extern "system" fn(shadertype: GLenum, count: GLsizei, indices: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUnmapBuffer_t = unsafe extern "system" fn(target: GLenum);
#[allow(non_camel_case_types)]
pub(crate) type glUnmapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUseProgram_t = unsafe extern "system" fn(program: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glUseProgramStages_t =
  unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glValidateProgram_t = unsafe extern "system" fn(program: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glValidateProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexArrayAttribBinding_t =
  unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexArrayAttribFormat_t = unsafe extern "system" fn(
  vaobj: GLuint,
  attribindex: GLuint,
  size: GLint,
  ty: GLenum,
  normalized: GLboolean,
  relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexArrayAttribIFormat_t = unsafe extern "system" fn(
  vaobj: GLuint,
  attribindex: GLuint,
  size: GLint,
  ty: GLenum,
  relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexArrayAttribLFormat_t = unsafe extern "system" fn(
  vaobj: GLuint,
  attribindex: GLuint,
  size: GLint,
  ty: GLenum,
  relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexArrayBindingDivisor_t =
  unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexArrayElementBuffer_t =
  unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexArrayVertexBuffer_t = unsafe extern "system" fn(
  vaobj: GLuint,
  bindingindex: GLuint,
  buffer: GLuint,
  offset: GLintptr,
  stride: GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexArrayVertexBuffers_t = unsafe extern "system" fn(
  vaobj: GLuint,
  first: GLuint,
  count: GLsizei,
  buffers: *const GLuint,
  offsets: *const GLintptr,
  strides: *const GLsizei,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib1f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib1fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib1s_t = unsafe extern "system" fn(index: GLuint, x: GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib1sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib2d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib2dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib2f_t =
  unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib2fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib2s_t =
  unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib2sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib3d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib3dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib3f_t =
  unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib3fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib3s_t =
  unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib3sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4Nbv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4Niv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4Nsv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4Nub_t =
  unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4Nubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4Nuiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4Nusv_t =
  unsafe extern "system" fn(index: GLuint, v: *const GLushort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4bv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4f_t =
  unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4s_t =
  unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttrib4usv_t = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribBinding_t =
  unsafe extern "system" fn(attribindex: GLuint, bindingindex: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribDivisor_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribFormat_t = unsafe extern "system" fn(
  attribindex: GLuint,
  size: GLint,
  ty: GLenum,
  normalized: GLboolean,
  relativeoffset: GLuint,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI1i_t = unsafe extern "system" fn(index: GLuint, x: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI1iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI1ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI1uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI2i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI2iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI2ui_t =
  unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI2uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI3i_t =
  unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI3iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI3ui_t =
  unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI3uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI4bv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI4i_t =
  unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI4iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI4sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI4ui_t =
  unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribI4usv_t =
  unsafe extern "system" fn(index: GLuint, v: *const GLushort);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribIFormat_t =
  unsafe extern "system" fn(attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribIPointer_t = unsafe extern "system" fn(
  index: GLuint,
  size: GLint,
  ty: GLenum,
  stride: GLsizei,
  pointer: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribL1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribL1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribL2d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribL2dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribL3d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribL3dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribL4d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribL4dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribLFormat_t =
  unsafe extern "system" fn(attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribLPointer_t = unsafe extern "system" fn(
  index: GLuint,
  size: GLint,
  ty: GLenum,
  stride: GLsizei,
  pointer: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribP1ui_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribP1uiv_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribP2ui_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribP2uiv_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribP3ui_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribP3uiv_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribP4ui_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribP4uiv_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexAttribPointer_t = unsafe extern "system" fn(
  index: GLuint,
  size: GLint,
  ty: GLenum,
  normalized: GLboolean,
  stride: GLsizei,
  pointer: *const c_void,
);
#[allow(non_camel_case_types)]
pub(crate) type glVertexBindingDivisor_t =
  unsafe extern "system" fn(bindingindex: GLuint, divisor: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexP2ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexP2uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexP3ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexP3uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexP4ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glVertexP4uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
#[allow(non_camel_case_types)]
pub(crate) type glViewport_t =
  unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
#[allow(non_camel_case_types)]
pub(crate) type glViewportArrayv_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glViewportIndexedf_t =
  unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glViewportIndexedfv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
#[allow(non_camel_case_types)]
pub(crate) type glWaitSync_t =
  unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
