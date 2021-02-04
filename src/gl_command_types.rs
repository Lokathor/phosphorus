use super::*;

/// [glAccum](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAccum.xhtml)
/// * `op` group: AccumOp
/// * `value` group: CoordF
pub type glAccum_t = unsafe extern "system" fn(op: AccumOp, value: GLfloat);

/// [glAccumxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAccumxOES.xhtml)
pub type glAccumxOES_t = unsafe extern "system" fn(op: GLenum, value: GLfixed);

/// [glAcquireKeyedMutexWin32EXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAcquireKeyedMutexWin32EXT.xhtml)
pub type glAcquireKeyedMutexWin32EXT_t = unsafe extern "system" fn(memory: GLuint, key: GLuint64, timeout: GLuint) -> GLboolean;

/// [glActiveProgramEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveProgramEXT.xhtml)
/// * `program` class: program
pub type glActiveProgramEXT_t = unsafe extern "system" fn(program: GLuint);

/// [glActiveShaderProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveShaderProgram.xhtml)
/// * `pipeline` class: program pipeline
/// * `program` class: program
pub type glActiveShaderProgram_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);

/// [glActiveShaderProgramEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveShaderProgramEXT.xhtml)
/// * `pipeline` class: program pipeline
/// * `program` class: program
pub type glActiveShaderProgramEXT_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);

/// [glActiveStencilFaceEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveStencilFaceEXT.xhtml)
/// * `face` group: StencilFaceDirection
pub type glActiveStencilFaceEXT_t = unsafe extern "system" fn(face: StencilFaceDirection);

/// [glActiveTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveTexture.xhtml)
/// * `texture` group: TextureUnit
pub type glActiveTexture_t = unsafe extern "system" fn(texture: TextureUnit);

/// [glActiveTextureARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveTextureARB.xhtml)
/// * `texture` group: TextureUnit
pub type glActiveTextureARB_t = unsafe extern "system" fn(texture: TextureUnit);

/// [glActiveVaryingNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveVaryingNV.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glActiveVaryingNV_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar);

/// [glAlphaFragmentOp1ATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAlphaFragmentOp1ATI.xhtml)
/// * `op` group: FragmentOpATI
pub type glAlphaFragmentOp1ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint);

/// [glAlphaFragmentOp2ATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAlphaFragmentOp2ATI.xhtml)
/// * `op` group: FragmentOpATI
pub type glAlphaFragmentOp2ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint);

/// [glAlphaFragmentOp3ATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAlphaFragmentOp3ATI.xhtml)
/// * `op` group: FragmentOpATI
pub type glAlphaFragmentOp3ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint, arg3: GLuint, arg3Rep: GLuint, arg3Mod: GLuint);

/// [glAlphaFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAlphaFunc.xhtml)
/// * `func` group: AlphaFunction
pub type glAlphaFunc_t = unsafe extern "system" fn(func: AlphaFunction, ref_: GLfloat);

/// [glAlphaFuncQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAlphaFuncQCOM.xhtml)
pub type glAlphaFuncQCOM_t = unsafe extern "system" fn(func: GLenum, ref_: GLclampf);

/// [glAlphaFuncx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAlphaFuncx.xhtml)
/// * `func` group: AlphaFunction
pub type glAlphaFuncx_t = unsafe extern "system" fn(func: AlphaFunction, ref_: GLfixed);

/// [glAlphaFuncxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAlphaFuncxOES.xhtml)
/// * `func` group: AlphaFunction
/// * `ref` group: ClampedFixed
pub type glAlphaFuncxOES_t = unsafe extern "system" fn(func: AlphaFunction, ref_: GLfixed);

/// [glAlphaToCoverageDitherControlNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAlphaToCoverageDitherControlNV.xhtml)
pub type glAlphaToCoverageDitherControlNV_t = unsafe extern "system" fn(mode: GLenum);

/// [glApplyFramebufferAttachmentCMAAINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glApplyFramebufferAttachmentCMAAINTEL.xhtml)
pub type glApplyFramebufferAttachmentCMAAINTEL_t = unsafe extern "system" fn();

/// [glApplyTextureEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glApplyTextureEXT.xhtml)
/// * `mode` group: LightTextureModeEXT
pub type glApplyTextureEXT_t = unsafe extern "system" fn(mode: LightTextureModeEXT);

/// [glAreProgramsResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAreProgramsResidentNV.xhtml)
/// * `programs` class: program
/// * `programs` len: n
/// * `residences` group: Boolean
/// * `residences` len: n
pub type glAreProgramsResidentNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint, residences: *mut GLboolean) -> GLboolean;

/// [glAreTexturesResident](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAreTexturesResident.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
/// * `residences` group: Boolean
/// * `residences` len: n
pub type glAreTexturesResident_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean;

/// [glAreTexturesResidentEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAreTexturesResidentEXT.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
/// * `residences` group: Boolean
/// * `residences` len: n
pub type glAreTexturesResidentEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean;

/// [glArrayElement](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glArrayElement.xhtml)
pub type glArrayElement_t = unsafe extern "system" fn(i: GLint);

/// [glArrayElementEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glArrayElementEXT.xhtml)
pub type glArrayElementEXT_t = unsafe extern "system" fn(i: GLint);

/// [glArrayObjectATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glArrayObjectATI.xhtml)
/// * `array` group: EnableCap
/// * `type` group: ScalarType
/// * `buffer` class: buffer
pub type glArrayObjectATI_t = unsafe extern "system" fn(array: EnableCap, size: GLint, type_: ScalarType, stride: GLsizei, buffer: GLuint, offset: GLuint);

/// [glAsyncCopyBufferSubDataNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAsyncCopyBufferSubDataNVX.xhtml)
/// * `waitSemaphoreArray` len: waitSemaphoreCount
/// * `fenceValueArray` len: waitSemaphoreCount
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
/// * `signalSemaphoreArray` len: signalSemaphoreCount
/// * `signalValueArray` len: signalSemaphoreCount
pub type glAsyncCopyBufferSubDataNVX_t = unsafe extern "system" fn(waitSemaphoreCount: GLsizei, waitSemaphoreArray: *const GLuint, fenceValueArray: *const GLuint64, readGpu: GLuint, writeGpuMask: GLbitfield, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr, signalSemaphoreCount: GLsizei, signalSemaphoreArray: *const GLuint, signalValueArray: *const GLuint64) -> GLuint;

/// [glAsyncCopyImageSubDataNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAsyncCopyImageSubDataNVX.xhtml)
/// * `waitSemaphoreArray` len: waitSemaphoreCount
/// * `waitValueArray` len: waitSemaphoreCount
/// * `signalSemaphoreArray` len: signalSemaphoreCount
/// * `signalValueArray` len: signalSemaphoreCount
pub type glAsyncCopyImageSubDataNVX_t = unsafe extern "system" fn(waitSemaphoreCount: GLsizei, waitSemaphoreArray: *const GLuint, waitValueArray: *const GLuint64, srcGpu: GLuint, dstGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei, signalSemaphoreCount: GLsizei, signalSemaphoreArray: *const GLuint, signalValueArray: *const GLuint64) -> GLuint;

/// [glAsyncMarkerSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAsyncMarkerSGIX.xhtml)
pub type glAsyncMarkerSGIX_t = unsafe extern "system" fn(marker: GLuint);

/// [glAttachObjectARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachObjectARB.xhtml)
/// * `containerObj` group: handleARB
/// * `obj` group: handleARB
pub type glAttachObjectARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, obj: GLhandleARB);

/// [glAttachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml)
/// * `program` class: program
/// * `shader` class: shader
pub type glAttachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);

/// [glBegin](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBegin.xhtml)
/// * `mode` group: PrimitiveType
pub type glBegin_t = unsafe extern "system" fn(mode: PrimitiveType);

/// [glBeginConditionalRender](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginConditionalRender.xhtml)
/// * `mode` group: ConditionalRenderMode
pub type glBeginConditionalRender_t = unsafe extern "system" fn(id: GLuint, mode: ConditionalRenderMode);

/// [glBeginConditionalRenderNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginConditionalRenderNV.xhtml)
/// * `mode` group: ConditionalRenderMode
pub type glBeginConditionalRenderNV_t = unsafe extern "system" fn(id: GLuint, mode: ConditionalRenderMode);

/// [glBeginConditionalRenderNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginConditionalRenderNVX.xhtml)
pub type glBeginConditionalRenderNVX_t = unsafe extern "system" fn(id: GLuint);

/// [glBeginFragmentShaderATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginFragmentShaderATI.xhtml)
pub type glBeginFragmentShaderATI_t = unsafe extern "system" fn();

/// [glBeginOcclusionQueryNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginOcclusionQueryNV.xhtml)
pub type glBeginOcclusionQueryNV_t = unsafe extern "system" fn(id: GLuint);

/// [glBeginPerfMonitorAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginPerfMonitorAMD.xhtml)
pub type glBeginPerfMonitorAMD_t = unsafe extern "system" fn(monitor: GLuint);

/// [glBeginPerfQueryINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginPerfQueryINTEL.xhtml)
pub type glBeginPerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

/// [glBeginQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginQuery.xhtml)
/// * `target` group: QueryTarget
/// * `id` class: query
pub type glBeginQuery_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint);

/// [glBeginQueryARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginQueryARB.xhtml)
/// * `target` group: QueryTarget
/// * `id` class: query
pub type glBeginQueryARB_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint);

/// [glBeginQueryEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginQueryEXT.xhtml)
/// * `target` group: QueryTarget
/// * `id` class: query
pub type glBeginQueryEXT_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint);

/// [glBeginQueryIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginQueryIndexed.xhtml)
/// * `target` group: QueryTarget
/// * `id` class: query
pub type glBeginQueryIndexed_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint, id: GLuint);

/// [glBeginTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginTransformFeedback.xhtml)
/// * `primitiveMode` group: PrimitiveType
pub type glBeginTransformFeedback_t = unsafe extern "system" fn(primitiveMode: PrimitiveType);

/// [glBeginTransformFeedbackEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginTransformFeedbackEXT.xhtml)
/// * `primitiveMode` group: PrimitiveType
pub type glBeginTransformFeedbackEXT_t = unsafe extern "system" fn(primitiveMode: PrimitiveType);

/// [glBeginTransformFeedbackNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginTransformFeedbackNV.xhtml)
/// * `primitiveMode` group: PrimitiveType
pub type glBeginTransformFeedbackNV_t = unsafe extern "system" fn(primitiveMode: PrimitiveType);

/// [glBeginVertexShaderEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginVertexShaderEXT.xhtml)
pub type glBeginVertexShaderEXT_t = unsafe extern "system" fn();

/// [glBeginVideoCaptureNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginVideoCaptureNV.xhtml)
pub type glBeginVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint);

/// [glBindAttribLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindAttribLocation.xhtml)
/// * `program` class: program
pub type glBindAttribLocation_t = unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);

/// [glBindAttribLocationARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindAttribLocationARB.xhtml)
/// * `programObj` group: handleARB
pub type glBindAttribLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, name: *const GLcharARB);

/// [glBindBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBuffer_t = unsafe extern "system" fn(target: BufferTargetARB, buffer: GLuint);

/// [glBindBufferARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferARB.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBufferARB_t = unsafe extern "system" fn(target: BufferTargetARB, buffer: GLuint);

/// [glBindBufferBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferBase.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBufferBase_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint);

/// [glBindBufferBaseEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferBaseEXT.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBufferBaseEXT_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint);

/// [glBindBufferBaseNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferBaseNV.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBufferBaseNV_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint);

/// [glBindBufferOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferOffsetEXT.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
pub type glBindBufferOffsetEXT_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr);

/// [glBindBufferOffsetNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferOffsetNV.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
pub type glBindBufferOffsetNV_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr);

/// [glBindBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferRange.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glBindBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glBindBufferRangeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferRangeEXT.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glBindBufferRangeEXT_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glBindBufferRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferRangeNV.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glBindBufferRangeNV_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glBindBuffersBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffersBase.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffers` class: buffer
/// * `buffers` len: count
pub type glBindBuffersBase_t = unsafe extern "system" fn(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint);

/// [glBindBuffersRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffersRange.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffers` class: buffer
/// * `buffers` len: count
/// * `offsets` len: count
/// * `sizes` len: count
pub type glBindBuffersRange_t = unsafe extern "system" fn(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);

/// [glBindFragDataLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocation.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glBindFragDataLocation_t = unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);

/// [glBindFragDataLocationEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocationEXT.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glBindFragDataLocationEXT_t = unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);

/// [glBindFragDataLocationIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocationIndexed.xhtml)
/// * `program` class: program
pub type glBindFragDataLocationIndexed_t = unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);

/// [glBindFragDataLocationIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocationIndexedEXT.xhtml)
/// * `program` class: program
pub type glBindFragDataLocationIndexedEXT_t = unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);

/// [glBindFragmentShaderATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragmentShaderATI.xhtml)
pub type glBindFragmentShaderATI_t = unsafe extern "system" fn(id: GLuint);

/// [glBindFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFramebuffer.xhtml)
/// * `target` group: FramebufferTarget
/// * `framebuffer` class: framebuffer
pub type glBindFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, framebuffer: GLuint);

/// [glBindFramebufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFramebufferEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `framebuffer` class: framebuffer
pub type glBindFramebufferEXT_t = unsafe extern "system" fn(target: FramebufferTarget, framebuffer: GLuint);

/// [glBindFramebufferOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFramebufferOES.xhtml)
/// * `target` group: FramebufferTarget
/// * `framebuffer` class: framebuffer
pub type glBindFramebufferOES_t = unsafe extern "system" fn(target: FramebufferTarget, framebuffer: GLuint);

/// [glBindImageTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindImageTexture.xhtml)
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `access` group: BufferAccessARB
/// * `format` group: InternalFormat
pub type glBindImageTexture_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: BufferAccessARB, format: InternalFormat);

/// [glBindImageTextureEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindImageTextureEXT.xhtml)
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `access` group: BufferAccessARB
pub type glBindImageTextureEXT_t = unsafe extern "system" fn(index: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: BufferAccessARB, format: GLint);

/// [glBindImageTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindImageTextures.xhtml)
/// * `textures` class: texture
/// * `textures` len: count
pub type glBindImageTextures_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);

/// [glBindLightParameterEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindLightParameterEXT.xhtml)
/// * `light` group: LightName
/// * `value` group: LightParameter
pub type glBindLightParameterEXT_t = unsafe extern "system" fn(light: LightName, value: LightParameter) -> GLuint;

/// [glBindMaterialParameterEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindMaterialParameterEXT.xhtml)
/// * `face` group: MaterialFace
/// * `value` group: MaterialParameter
pub type glBindMaterialParameterEXT_t = unsafe extern "system" fn(face: MaterialFace, value: MaterialParameter) -> GLuint;

/// [glBindMultiTextureEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindMultiTextureEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glBindMultiTextureEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, texture: GLuint);

/// [glBindParameterEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindParameterEXT.xhtml)
/// * `value` group: VertexShaderParameterEXT
pub type glBindParameterEXT_t = unsafe extern "system" fn(value: VertexShaderParameterEXT) -> GLuint;

/// [glBindProgramARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindProgramARB.xhtml)
/// * `target` group: ProgramTarget
/// * `program` class: program
pub type glBindProgramARB_t = unsafe extern "system" fn(target: ProgramTarget, program: GLuint);

/// [glBindProgramNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindProgramNV.xhtml)
/// * `target` group: VertexAttribEnumNV
/// * `id` class: program
pub type glBindProgramNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, id: GLuint);

/// [glBindProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindProgramPipeline.xhtml)
/// * `pipeline` class: program pipeline
pub type glBindProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);

/// [glBindProgramPipelineEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindProgramPipelineEXT.xhtml)
/// * `pipeline` class: program pipeline
pub type glBindProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint);

/// [glBindRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindRenderbuffer.xhtml)
/// * `target` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glBindRenderbuffer_t = unsafe extern "system" fn(target: RenderbufferTarget, renderbuffer: GLuint);

/// [glBindRenderbufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindRenderbufferEXT.xhtml)
/// * `target` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glBindRenderbufferEXT_t = unsafe extern "system" fn(target: RenderbufferTarget, renderbuffer: GLuint);

/// [glBindRenderbufferOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindRenderbufferOES.xhtml)
/// * `target` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glBindRenderbufferOES_t = unsafe extern "system" fn(target: RenderbufferTarget, renderbuffer: GLuint);

/// [glBindSampler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindSampler.xhtml)
/// * `sampler` class: sampler
pub type glBindSampler_t = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);

/// [glBindSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindSamplers.xhtml)
/// * `samplers` class: sampler
/// * `samplers` len: count
pub type glBindSamplers_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint);

/// [glBindShadingRateImageNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindShadingRateImageNV.xhtml)
/// * `texture` class: texture
pub type glBindShadingRateImageNV_t = unsafe extern "system" fn(texture: GLuint);

/// [glBindTexGenParameterEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTexGenParameterEXT.xhtml)
/// * `unit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `value` group: TextureGenParameter
pub type glBindTexGenParameterEXT_t = unsafe extern "system" fn(unit: TextureUnit, coord: TextureCoordName, value: TextureGenParameter) -> GLuint;

/// [glBindTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTexture.xhtml)
/// * `target` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glBindTexture_t = unsafe extern "system" fn(target: TextureTarget, texture: GLuint);

/// [glBindTextureEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTextureEXT.xhtml)
/// * `target` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glBindTextureEXT_t = unsafe extern "system" fn(target: TextureTarget, texture: GLuint);

/// [glBindTextureUnit](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTextureUnit.xhtml)
/// * `texture` class: texture
pub type glBindTextureUnit_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint);

/// [glBindTextureUnitParameterEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTextureUnitParameterEXT.xhtml)
/// * `unit` group: TextureUnit
/// * `value` group: VertexShaderTextureUnitParameter
pub type glBindTextureUnitParameterEXT_t = unsafe extern "system" fn(unit: TextureUnit, value: VertexShaderTextureUnitParameter) -> GLuint;

/// [glBindTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTextures.xhtml)
/// * `textures` class: texture
/// * `textures` len: count
pub type glBindTextures_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);

/// [glBindTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTransformFeedback.xhtml)
/// * `target` group: BindTransformFeedbackTarget
/// * `id` class: transform feedback
pub type glBindTransformFeedback_t = unsafe extern "system" fn(target: BindTransformFeedbackTarget, id: GLuint);

/// [glBindTransformFeedbackNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTransformFeedbackNV.xhtml)
/// * `target` group: BufferTargetARB
/// * `id` class: transform feedback
pub type glBindTransformFeedbackNV_t = unsafe extern "system" fn(target: BufferTargetARB, id: GLuint);

/// [glBindVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml)
/// * `array` class: vertex array
pub type glBindVertexArray_t = unsafe extern "system" fn(array: GLuint);

/// [glBindVertexArrayAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArrayAPPLE.xhtml)
/// * `array` class: vertex array
pub type glBindVertexArrayAPPLE_t = unsafe extern "system" fn(array: GLuint);

/// [glBindVertexArrayOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArrayOES.xhtml)
/// * `array` class: vertex array
pub type glBindVertexArrayOES_t = unsafe extern "system" fn(array: GLuint);

/// [glBindVertexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffer.xhtml)
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
pub type glBindVertexBuffer_t = unsafe extern "system" fn(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

/// [glBindVertexBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffers.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: count
/// * `offsets` len: count
/// * `strides` len: count
pub type glBindVertexBuffers_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);

/// [glBindVertexShaderEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexShaderEXT.xhtml)
pub type glBindVertexShaderEXT_t = unsafe extern "system" fn(id: GLuint);

/// [glBindVideoCaptureStreamBufferNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVideoCaptureStreamBufferNV.xhtml)
/// * `offset` group: BufferOffsetARB
pub type glBindVideoCaptureStreamBufferNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, frame_region: GLenum, offset: GLintptrARB);

/// [glBindVideoCaptureStreamTextureNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVideoCaptureStreamTextureNV.xhtml)
/// * `texture` class: texture
pub type glBindVideoCaptureStreamTextureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, frame_region: GLenum, target: GLenum, texture: GLuint);

/// [glBinormal3bEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3bEXT.xhtml)
pub type glBinormal3bEXT_t = unsafe extern "system" fn(bx: GLbyte, by: GLbyte, bz: GLbyte);

/// [glBinormal3bvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3bvEXT.xhtml)
pub type glBinormal3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// [glBinormal3dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3dEXT.xhtml)
/// * `bx` group: CoordD
/// * `by` group: CoordD
/// * `bz` group: CoordD
pub type glBinormal3dEXT_t = unsafe extern "system" fn(bx: GLdouble, by: GLdouble, bz: GLdouble);

/// [glBinormal3dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3dvEXT.xhtml)
/// * `v` group: CoordD
pub type glBinormal3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glBinormal3fEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3fEXT.xhtml)
/// * `bx` group: CoordF
/// * `by` group: CoordF
/// * `bz` group: CoordF
pub type glBinormal3fEXT_t = unsafe extern "system" fn(bx: GLfloat, by: GLfloat, bz: GLfloat);

/// [glBinormal3fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3fvEXT.xhtml)
/// * `v` group: CoordF
pub type glBinormal3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glBinormal3iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3iEXT.xhtml)
pub type glBinormal3iEXT_t = unsafe extern "system" fn(bx: GLint, by: GLint, bz: GLint);

/// [glBinormal3ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3ivEXT.xhtml)
pub type glBinormal3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glBinormal3sEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3sEXT.xhtml)
pub type glBinormal3sEXT_t = unsafe extern "system" fn(bx: GLshort, by: GLshort, bz: GLshort);

/// [glBinormal3svEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormal3svEXT.xhtml)
pub type glBinormal3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glBinormalPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBinormalPointerEXT.xhtml)
/// * `type` group: BinormalPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glBinormalPointerEXT_t = unsafe extern "system" fn(type_: BinormalPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// [glBitmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBitmap.xhtml)
/// * `xorig` group: CoordF
/// * `yorig` group: CoordF
/// * `xmove` group: CoordF
/// * `ymove` group: CoordF
/// * `bitmap` len: COMPSIZE(width,height)
pub type glBitmap_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *const GLubyte);

/// [glBitmapxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBitmapxOES.xhtml)
/// * `bitmap` len: COMPSIZE(width,height)
pub type glBitmapxOES_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, xorig: GLfixed, yorig: GLfixed, xmove: GLfixed, ymove: GLfixed, bitmap: *const GLubyte);

/// [glBlendBarrier](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendBarrier.xhtml)
pub type glBlendBarrier_t = unsafe extern "system" fn();

/// [glBlendBarrierKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendBarrierKHR.xhtml)
pub type glBlendBarrierKHR_t = unsafe extern "system" fn();

/// [glBlendBarrierNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendBarrierNV.xhtml)
pub type glBlendBarrierNV_t = unsafe extern "system" fn();

/// [glBlendColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendColor.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
pub type glBlendColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// [glBlendColorEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendColorEXT.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
pub type glBlendColorEXT_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// [glBlendColorxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendColorxOES.xhtml)
/// * `red` group: ClampedFixed
/// * `green` group: ClampedFixed
/// * `blue` group: ClampedFixed
/// * `alpha` group: ClampedFixed
pub type glBlendColorxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// [glBlendEquation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquation.xhtml)
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquation_t = unsafe extern "system" fn(mode: BlendEquationModeEXT);

/// [glBlendEquationEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationEXT.xhtml)
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationEXT_t = unsafe extern "system" fn(mode: BlendEquationModeEXT);

/// [glBlendEquationIndexedAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationIndexedAMD.xhtml)
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// [glBlendEquationOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationOES.xhtml)
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationOES_t = unsafe extern "system" fn(mode: BlendEquationModeEXT);

/// [glBlendEquationSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparate.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparate_t = unsafe extern "system" fn(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// [glBlendEquationSeparateEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparateEXT.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateEXT_t = unsafe extern "system" fn(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// [glBlendEquationSeparateIndexedAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparateIndexedAMD.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// [glBlendEquationSeparateOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparateOES.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateOES_t = unsafe extern "system" fn(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// [glBlendEquationSeparatei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparatei.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparatei_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// [glBlendEquationSeparateiARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparateiARB.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateiARB_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// [glBlendEquationSeparateiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparateiEXT.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateiEXT_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// [glBlendEquationSeparateiOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparateiOES.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateiOES_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// [glBlendEquationi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationi.xhtml)
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationi_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// [glBlendEquationiARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationiARB.xhtml)
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationiARB_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// [glBlendEquationiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationiEXT.xhtml)
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationiEXT_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// [glBlendEquationiOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationiOES.xhtml)
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationiOES_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// [glBlendFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunc.xhtml)
/// * `sfactor` group: BlendingFactor
/// * `dfactor` group: BlendingFactor
pub type glBlendFunc_t = unsafe extern "system" fn(sfactor: BlendingFactor, dfactor: BlendingFactor);

/// [glBlendFuncIndexedAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncIndexedAMD.xhtml)
pub type glBlendFuncIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);

/// [glBlendFuncSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparate.xhtml)
/// * `sfactorRGB` group: BlendingFactor
/// * `dfactorRGB` group: BlendingFactor
/// * `sfactorAlpha` group: BlendingFactor
/// * `dfactorAlpha` group: BlendingFactor
pub type glBlendFuncSeparate_t = unsafe extern "system" fn(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);

/// [glBlendFuncSeparateEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparateEXT.xhtml)
/// * `sfactorRGB` group: BlendingFactor
/// * `dfactorRGB` group: BlendingFactor
/// * `sfactorAlpha` group: BlendingFactor
/// * `dfactorAlpha` group: BlendingFactor
pub type glBlendFuncSeparateEXT_t = unsafe extern "system" fn(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);

/// [glBlendFuncSeparateINGR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparateINGR.xhtml)
/// * `sfactorRGB` group: BlendingFactor
/// * `dfactorRGB` group: BlendingFactor
/// * `sfactorAlpha` group: BlendingFactor
/// * `dfactorAlpha` group: BlendingFactor
pub type glBlendFuncSeparateINGR_t = unsafe extern "system" fn(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);

/// [glBlendFuncSeparateIndexedAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparateIndexedAMD.xhtml)
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// [glBlendFuncSeparateOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparateOES.xhtml)
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateOES_t = unsafe extern "system" fn(srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// [glBlendFuncSeparatei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparatei.xhtml)
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparatei_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// [glBlendFuncSeparateiARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparateiARB.xhtml)
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateiARB_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// [glBlendFuncSeparateiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparateiEXT.xhtml)
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateiEXT_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// [glBlendFuncSeparateiOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparateiOES.xhtml)
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateiOES_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// [glBlendFunci](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunci.xhtml)
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
pub type glBlendFunci_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

/// [glBlendFunciARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunciARB.xhtml)
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
pub type glBlendFunciARB_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

/// [glBlendFunciEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunciEXT.xhtml)
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
pub type glBlendFunciEXT_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

/// [glBlendFunciOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunciOES.xhtml)
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
pub type glBlendFunciOES_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

/// [glBlendParameteriNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendParameteriNV.xhtml)
pub type glBlendParameteriNV_t = unsafe extern "system" fn(pname: GLenum, value: GLint);

/// [glBlitFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitFramebuffer.xhtml)
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitFramebuffer_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// [glBlitFramebufferANGLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitFramebufferANGLE.xhtml)
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitFramebufferANGLE_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// [glBlitFramebufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitFramebufferEXT.xhtml)
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitFramebufferEXT_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// [glBlitFramebufferNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitFramebufferNV.xhtml)
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitFramebufferNV_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// [glBlitNamedFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitNamedFramebuffer.xhtml)
/// * `readFramebuffer` class: framebuffer
/// * `drawFramebuffer` class: framebuffer
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitNamedFramebuffer_t = unsafe extern "system" fn(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// [glBufferAddressRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferAddressRangeNV.xhtml)
/// * `length` group: BufferSize
pub type glBufferAddressRangeNV_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, address: GLuint64EXT, length: GLsizeiptr);

/// [glBufferAttachMemoryNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferAttachMemoryNV.xhtml)
/// * `target` group: BufferTargetARB
pub type glBufferAttachMemoryNV_t = unsafe extern "system" fn(target: BufferTargetARB, memory: GLuint, offset: GLuint64);

/// [glBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml)
/// * `target` group: BufferTargetARB
/// * `size` group: BufferSize
/// * `data` len: size
/// * `usage` group: BufferUsageARB
pub type glBufferData_t = unsafe extern "system" fn(target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB);

/// [glBufferDataARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferDataARB.xhtml)
/// * `target` group: BufferTargetARB
/// * `size` group: BufferSizeARB
/// * `data` len: size
/// * `usage` group: BufferUsageARB
pub type glBufferDataARB_t = unsafe extern "system" fn(target: BufferTargetARB, size: GLsizeiptrARB, data: *const void, usage: BufferUsageARB);

/// [glBufferPageCommitmentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferPageCommitmentARB.xhtml)
/// * `commit` group: Boolean
pub type glBufferPageCommitmentARB_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

/// [glBufferPageCommitmentMemNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferPageCommitmentMemNV.xhtml)
/// * `target` group: BufferStorageTarget
/// * `commit` group: Boolean
pub type glBufferPageCommitmentMemNV_t = unsafe extern "system" fn(target: BufferStorageTarget, offset: GLintptr, size: GLsizeiptr, memory: GLuint, memOffset: GLuint64, commit: GLboolean);

/// [glBufferParameteriAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferParameteriAPPLE.xhtml)
pub type glBufferParameteriAPPLE_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

/// [glBufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferStorage.xhtml)
/// * `target` group: BufferStorageTarget
/// * `data` len: size
/// * `flags` group: BufferStorageMask
pub type glBufferStorage_t = unsafe extern "system" fn(target: BufferStorageTarget, size: GLsizeiptr, data: *const void, flags: GLbitfield);

/// [glBufferStorageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferStorageEXT.xhtml)
/// * `target` group: BufferStorageTarget
/// * `data` len: size
/// * `flags` group: BufferStorageMask
pub type glBufferStorageEXT_t = unsafe extern "system" fn(target: BufferStorageTarget, size: GLsizeiptr, data: *const void, flags: GLbitfield);

/// [glBufferStorageExternalEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferStorageExternalEXT.xhtml)
/// * `flags` group: BufferStorageMask
pub type glBufferStorageExternalEXT_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, clientBuffer: GLeglClientBufferEXT, flags: GLbitfield);

/// [glBufferStorageMemEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferStorageMemEXT.xhtml)
/// * `target` group: BufferTargetARB
/// * `size` group: BufferSize
pub type glBufferStorageMemEXT_t = unsafe extern "system" fn(target: BufferTargetARB, size: GLsizeiptr, memory: GLuint, offset: GLuint64);

/// [glBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferSubData.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `data` len: size
pub type glBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// [glBufferSubDataARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferSubDataARB.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffsetARB
/// * `size` group: BufferSizeARB
/// * `data` len: size
pub type glBufferSubDataARB_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptrARB, size: GLsizeiptrARB, data: *const void);

/// [glCallCommandListNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCallCommandListNV.xhtml)
pub type glCallCommandListNV_t = unsafe extern "system" fn(list: GLuint);

/// [glCallList](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCallList.xhtml)
/// * `list` group: List
pub type glCallList_t = unsafe extern "system" fn(list: GLuint);

/// [glCallLists](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCallLists.xhtml)
/// * `type` group: ListNameType
/// * `lists` len: COMPSIZE(n,type)
pub type glCallLists_t = unsafe extern "system" fn(n: GLsizei, type_: ListNameType, lists: *const void);

/// [glCheckFramebufferStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckFramebufferStatus.xhtml)
/// * `target` group: FramebufferTarget
pub type glCheckFramebufferStatus_t = unsafe extern "system" fn(target: FramebufferTarget) -> FramebufferStatus;

/// [glCheckFramebufferStatusEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckFramebufferStatusEXT.xhtml)
/// * `target` group: FramebufferTarget
pub type glCheckFramebufferStatusEXT_t = unsafe extern "system" fn(target: FramebufferTarget) -> FramebufferStatus;

/// [glCheckFramebufferStatusOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckFramebufferStatusOES.xhtml)
/// * `target` group: FramebufferTarget
pub type glCheckFramebufferStatusOES_t = unsafe extern "system" fn(target: FramebufferTarget) -> FramebufferStatus;

/// [glCheckNamedFramebufferStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckNamedFramebufferStatus.xhtml)
/// * `framebuffer` class: framebuffer
/// * `target` group: FramebufferTarget
pub type glCheckNamedFramebufferStatus_t = unsafe extern "system" fn(framebuffer: GLuint, target: FramebufferTarget) -> FramebufferStatus;

/// [glCheckNamedFramebufferStatusEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckNamedFramebufferStatusEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `target` group: FramebufferTarget
pub type glCheckNamedFramebufferStatusEXT_t = unsafe extern "system" fn(framebuffer: GLuint, target: FramebufferTarget) -> FramebufferStatus;

/// [glClampColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClampColor.xhtml)
/// * `target` group: ClampColorTargetARB
/// * `clamp` group: ClampColorModeARB
pub type glClampColor_t = unsafe extern "system" fn(target: ClampColorTargetARB, clamp: ClampColorModeARB);

/// [glClampColorARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClampColorARB.xhtml)
/// * `target` group: ClampColorTargetARB
/// * `clamp` group: ClampColorModeARB
pub type glClampColorARB_t = unsafe extern "system" fn(target: ClampColorTargetARB, clamp: ClampColorModeARB);

/// [glClear](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClear.xhtml)
/// * `mask` group: ClearBufferMask
pub type glClear_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glClearAccum](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearAccum.xhtml)
pub type glClearAccum_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// [glClearAccumxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearAccumxOES.xhtml)
/// * `red` group: ClampedFixed
/// * `green` group: ClampedFixed
/// * `blue` group: ClampedFixed
/// * `alpha` group: ClampedFixed
pub type glClearAccumxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// [glClearBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferData.xhtml)
/// * `target` group: BufferStorageTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearBufferData_t = unsafe extern "system" fn(target: BufferStorageTarget, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferSubData.xhtml)
/// * `target` group: BufferTargetARB
/// * `internalformat` group: InternalFormat
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearBufferfi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferfi.xhtml)
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
pub type glClearBufferfi_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

/// [glClearBufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferfv.xhtml)
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
pub type glClearBufferfv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);

/// [glClearBufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferiv.xhtml)
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
pub type glClearBufferiv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLint);

/// [glClearBufferuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferuiv.xhtml)
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
pub type glClearBufferuiv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLuint);

/// [glClearColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
pub type glClearColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// [glClearColorIiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColorIiEXT.xhtml)
pub type glClearColorIiEXT_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint, alpha: GLint);

/// [glClearColorIuiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColorIuiEXT.xhtml)
pub type glClearColorIuiEXT_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);

/// [glClearColorx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColorx.xhtml)
pub type glClearColorx_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// [glClearColorxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColorxOES.xhtml)
/// * `red` group: ClampedFixed
/// * `green` group: ClampedFixed
/// * `blue` group: ClampedFixed
/// * `alpha` group: ClampedFixed
pub type glClearColorxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// [glClearDepth](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepth.xhtml)
pub type glClearDepth_t = unsafe extern "system" fn(depth: GLdouble);

/// [glClearDepthdNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepthdNV.xhtml)
pub type glClearDepthdNV_t = unsafe extern "system" fn(depth: GLdouble);

/// [glClearDepthf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepthf.xhtml)
pub type glClearDepthf_t = unsafe extern "system" fn(d: GLfloat);

/// [glClearDepthfOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepthfOES.xhtml)
/// * `depth` group: ClampedFloat32
pub type glClearDepthfOES_t = unsafe extern "system" fn(depth: GLclampf);

/// [glClearDepthx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepthx.xhtml)
pub type glClearDepthx_t = unsafe extern "system" fn(depth: GLfixed);

/// [glClearDepthxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepthxOES.xhtml)
/// * `depth` group: ClampedFixed
pub type glClearDepthxOES_t = unsafe extern "system" fn(depth: GLfixed);

/// [glClearIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearIndex.xhtml)
/// * `c` group: MaskedColorIndexValueF
pub type glClearIndex_t = unsafe extern "system" fn(c: GLfloat);

/// [glClearNamedBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedBufferData.xhtml)
/// * `buffer` class: buffer
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glClearNamedBufferData_t = unsafe extern "system" fn(buffer: GLuint, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearNamedBufferDataEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedBufferDataEXT.xhtml)
/// * `buffer` class: buffer
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearNamedBufferDataEXT_t = unsafe extern "system" fn(buffer: GLuint, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedBufferSubData.xhtml)
/// * `buffer` class: buffer
/// * `internalformat` group: InternalFormat
/// * `size` group: BufferSize
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glClearNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearNamedBufferSubDataEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedBufferSubDataEXT.xhtml)
/// * `buffer` class: buffer
/// * `offset` group: BufferSize
/// * `size` group: BufferSize
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, offset: GLsizeiptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearNamedFramebufferfi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferfi.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
pub type glClearNamedFramebufferfi_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

/// [glClearNamedFramebufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferfv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
pub type glClearNamedFramebufferfv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);

/// [glClearNamedFramebufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferiv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
pub type glClearNamedFramebufferiv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLint);

/// [glClearNamedFramebufferuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferuiv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
pub type glClearNamedFramebufferuiv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLuint);

/// [glClearPixelLocalStorageuiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearPixelLocalStorageuiEXT.xhtml)
/// * `values` len: n
pub type glClearPixelLocalStorageuiEXT_t = unsafe extern "system" fn(offset: GLsizei, n: GLsizei, values: *const GLuint);

/// [glClearStencil](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearStencil.xhtml)
/// * `s` group: StencilValue
pub type glClearStencil_t = unsafe extern "system" fn(s: GLint);

/// [glClearTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearTexImage.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearTexImageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearTexImageEXT.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearTexImageEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearTexSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearTexSubImage.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearTexSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClearTexSubImageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearTexSubImageEXT.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearTexSubImageEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

/// [glClientActiveTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientActiveTexture.xhtml)
/// * `texture` group: TextureUnit
pub type glClientActiveTexture_t = unsafe extern "system" fn(texture: TextureUnit);

/// [glClientActiveTextureARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientActiveTextureARB.xhtml)
/// * `texture` group: TextureUnit
pub type glClientActiveTextureARB_t = unsafe extern "system" fn(texture: TextureUnit);

/// [glClientActiveVertexStreamATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientActiveVertexStreamATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glClientActiveVertexStreamATI_t = unsafe extern "system" fn(stream: VertexStreamATI);

/// [glClientAttribDefaultEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientAttribDefaultEXT.xhtml)
/// * `mask` group: ClientAttribMask
pub type glClientAttribDefaultEXT_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glClientWaitSemaphoreui64NVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientWaitSemaphoreui64NVX.xhtml)
/// * `semaphoreArray` len: fenceObjectCount
/// * `fenceValueArray` len: fenceObjectCount
pub type glClientWaitSemaphoreui64NVX_t = unsafe extern "system" fn(fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

/// [glClientWaitSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientWaitSync.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncObjectMask
pub type glClientWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus;

/// [glClientWaitSyncAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientWaitSyncAPPLE.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncObjectMask
pub type glClientWaitSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus;

/// [glClipControl](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipControl.xhtml)
/// * `origin` group: ClipControlOrigin
/// * `depth` group: ClipControlDepth
pub type glClipControl_t = unsafe extern "system" fn(origin: ClipControlOrigin, depth: ClipControlDepth);

/// [glClipControlEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipControlEXT.xhtml)
pub type glClipControlEXT_t = unsafe extern "system" fn(origin: GLenum, depth: GLenum);

/// [glClipPlane](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipPlane.xhtml)
/// * `plane` group: ClipPlaneName
pub type glClipPlane_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *const [GLdouble; 4]);

/// [glClipPlanef](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipPlanef.xhtml)
/// * `p` group: ClipPlaneName
pub type glClipPlanef_t = unsafe extern "system" fn(p: ClipPlaneName, eqn: *const [GLfloat; 4]);

/// [glClipPlanefIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipPlanefIMG.xhtml)
/// * `p` group: ClipPlaneName
pub type glClipPlanefIMG_t = unsafe extern "system" fn(p: ClipPlaneName, eqn: *const [GLfloat; 4]);

/// [glClipPlanefOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipPlanefOES.xhtml)
/// * `plane` group: ClipPlaneName
pub type glClipPlanefOES_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *const [GLfloat; 4]);

/// [glClipPlanex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipPlanex.xhtml)
/// * `plane` group: ClipPlaneName
pub type glClipPlanex_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *const [GLfixed; 4]);

/// [glClipPlanexIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipPlanexIMG.xhtml)
/// * `p` group: ClipPlaneName
pub type glClipPlanexIMG_t = unsafe extern "system" fn(p: ClipPlaneName, eqn: *const [GLfixed; 4]);

/// [glClipPlanexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipPlanexOES.xhtml)
/// * `plane` group: ClipPlaneName
pub type glClipPlanexOES_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *const [GLfixed; 4]);

/// [glColor3b](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3b.xhtml)
/// * `red` group: ColorB
/// * `green` group: ColorB
/// * `blue` group: ColorB
pub type glColor3b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

/// [glColor3bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3bv.xhtml)
/// * `v` group: ColorB
pub type glColor3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// [glColor3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3d.xhtml)
/// * `red` group: ColorD
/// * `green` group: ColorD
/// * `blue` group: ColorD
pub type glColor3d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

/// [glColor3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3dv.xhtml)
/// * `v` group: ColorD
pub type glColor3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glColor3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3f.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
pub type glColor3f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

/// [glColor3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3fVertex3fSUN.xhtml)
pub type glColor3fVertex3fSUN_t = unsafe extern "system" fn(r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glColor3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3fVertex3fvSUN.xhtml)
pub type glColor3fVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glColor3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3fv.xhtml)
/// * `v` group: ColorF
pub type glColor3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glColor3hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3hNV.xhtml)
/// * `red` group: Half16NV
/// * `green` group: Half16NV
/// * `blue` group: Half16NV
pub type glColor3hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV);

/// [glColor3hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3hvNV.xhtml)
/// * `v` group: Half16NV
pub type glColor3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// [glColor3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3i.xhtml)
/// * `red` group: ColorI
/// * `green` group: ColorI
/// * `blue` group: ColorI
pub type glColor3i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

/// [glColor3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3iv.xhtml)
/// * `v` group: ColorI
pub type glColor3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glColor3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3s.xhtml)
/// * `red` group: ColorS
/// * `green` group: ColorS
/// * `blue` group: ColorS
pub type glColor3s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

/// [glColor3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3sv.xhtml)
/// * `v` group: ColorS
pub type glColor3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glColor3ub](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3ub.xhtml)
/// * `red` group: ColorUB
/// * `green` group: ColorUB
/// * `blue` group: ColorUB
pub type glColor3ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

/// [glColor3ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3ubv.xhtml)
/// * `v` group: ColorUB
pub type glColor3ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

/// [glColor3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3ui.xhtml)
/// * `red` group: ColorUI
/// * `green` group: ColorUI
/// * `blue` group: ColorUI
pub type glColor3ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

/// [glColor3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3uiv.xhtml)
/// * `v` group: ColorUI
pub type glColor3uiv_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

/// [glColor3us](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3us.xhtml)
/// * `red` group: ColorUS
/// * `green` group: ColorUS
/// * `blue` group: ColorUS
pub type glColor3us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

/// [glColor3usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3usv.xhtml)
/// * `v` group: ColorUS
pub type glColor3usv_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

/// [glColor3xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3xOES.xhtml)
pub type glColor3xOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed);

/// [glColor3xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor3xvOES.xhtml)
pub type glColor3xvOES_t = unsafe extern "system" fn(components: *const [GLfixed; 3]);

/// [glColor4b](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4b.xhtml)
/// * `red` group: ColorB
/// * `green` group: ColorB
/// * `blue` group: ColorB
/// * `alpha` group: ColorB
pub type glColor4b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte);

/// [glColor4bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4bv.xhtml)
/// * `v` group: ColorB
pub type glColor4bv_t = unsafe extern "system" fn(v: *const [GLbyte; 4]);

/// [glColor4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4d.xhtml)
/// * `red` group: ColorD
/// * `green` group: ColorD
/// * `blue` group: ColorD
/// * `alpha` group: ColorD
pub type glColor4d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble);

/// [glColor4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4dv.xhtml)
/// * `v` group: ColorD
pub type glColor4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// [glColor4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4f.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
pub type glColor4f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// [glColor4fNormal3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4fNormal3fVertex3fSUN.xhtml)
pub type glColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glColor4fNormal3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4fNormal3fVertex3fvSUN.xhtml)
pub type glColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glColor4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4fv.xhtml)
/// * `v` group: ColorF
pub type glColor4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// [glColor4hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4hNV.xhtml)
/// * `red` group: Half16NV
/// * `green` group: Half16NV
/// * `blue` group: Half16NV
/// * `alpha` group: Half16NV
pub type glColor4hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV, alpha: GLhalfNV);

/// [glColor4hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4hvNV.xhtml)
/// * `v` group: Half16NV
pub type glColor4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

/// [glColor4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4i.xhtml)
/// * `red` group: ColorI
/// * `green` group: ColorI
/// * `blue` group: ColorI
/// * `alpha` group: ColorI
pub type glColor4i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint, alpha: GLint);

/// [glColor4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4iv.xhtml)
/// * `v` group: ColorI
pub type glColor4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// [glColor4s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4s.xhtml)
/// * `red` group: ColorS
/// * `green` group: ColorS
/// * `blue` group: ColorS
/// * `alpha` group: ColorS
pub type glColor4s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort);

/// [glColor4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4sv.xhtml)
/// * `v` group: ColorS
pub type glColor4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// [glColor4ub](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4ub.xhtml)
/// * `red` group: ColorUB
/// * `green` group: ColorUB
/// * `blue` group: ColorUB
/// * `alpha` group: ColorUB
pub type glColor4ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte);

/// [glColor4ubVertex2fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4ubVertex2fSUN.xhtml)
pub type glColor4ubVertex2fSUN_t = unsafe extern "system" fn(r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat);

/// [glColor4ubVertex2fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4ubVertex2fvSUN.xhtml)
pub type glColor4ubVertex2fvSUN_t = unsafe extern "system" fn(c: *const [GLubyte; 4], v: *const [GLfloat; 2]);

/// [glColor4ubVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4ubVertex3fSUN.xhtml)
pub type glColor4ubVertex3fSUN_t = unsafe extern "system" fn(r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glColor4ubVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4ubVertex3fvSUN.xhtml)
pub type glColor4ubVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

/// [glColor4ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4ubv.xhtml)
/// * `v` group: ColorUB
pub type glColor4ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 4]);

/// [glColor4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4ui.xhtml)
/// * `red` group: ColorUI
/// * `green` group: ColorUI
/// * `blue` group: ColorUI
/// * `alpha` group: ColorUI
pub type glColor4ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);

/// [glColor4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4uiv.xhtml)
/// * `v` group: ColorUI
pub type glColor4uiv_t = unsafe extern "system" fn(v: *const [GLuint; 4]);

/// [glColor4us](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4us.xhtml)
/// * `red` group: ColorUS
/// * `green` group: ColorUS
/// * `blue` group: ColorUS
/// * `alpha` group: ColorUS
pub type glColor4us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort);

/// [glColor4usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4usv.xhtml)
/// * `v` group: ColorUS
pub type glColor4usv_t = unsafe extern "system" fn(v: *const [GLushort; 4]);

/// [glColor4x](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4x.xhtml)
pub type glColor4x_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// [glColor4xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4xOES.xhtml)
pub type glColor4xOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// [glColor4xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColor4xvOES.xhtml)
pub type glColor4xvOES_t = unsafe extern "system" fn(components: *const [GLfixed; 4]);

/// [glColorFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorFormatNV.xhtml)
pub type glColorFormatNV_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei);

/// [glColorFragmentOp1ATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorFragmentOp1ATI.xhtml)
/// * `op` group: FragmentOpATI
pub type glColorFragmentOp1ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint);

/// [glColorFragmentOp2ATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorFragmentOp2ATI.xhtml)
/// * `op` group: FragmentOpATI
pub type glColorFragmentOp2ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint);

/// [glColorFragmentOp3ATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorFragmentOp3ATI.xhtml)
/// * `op` group: FragmentOpATI
pub type glColorFragmentOp3ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint, arg3: GLuint, arg3Rep: GLuint, arg3Mod: GLuint);

/// [glColorMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMask.xhtml)
/// * `red` group: Boolean
/// * `green` group: Boolean
/// * `blue` group: Boolean
/// * `alpha` group: Boolean
pub type glColorMask_t = unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

/// [glColorMaskIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMaskIndexedEXT.xhtml)
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
pub type glColorMaskIndexedEXT_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

/// [glColorMaski](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMaski.xhtml)
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
pub type glColorMaski_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

/// [glColorMaskiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMaskiEXT.xhtml)
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
pub type glColorMaskiEXT_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

/// [glColorMaskiOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMaskiOES.xhtml)
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
pub type glColorMaskiOES_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

/// [glColorMaterial](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMaterial.xhtml)
/// * `face` group: MaterialFace
/// * `mode` group: ColorMaterialParameter
pub type glColorMaterial_t = unsafe extern "system" fn(face: MaterialFace, mode: ColorMaterialParameter);

/// [glColorP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP3ui.xhtml)
/// * `type` group: ColorPointerType
pub type glColorP3ui_t = unsafe extern "system" fn(type_: ColorPointerType, color: GLuint);

/// [glColorP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP3uiv.xhtml)
/// * `type` group: ColorPointerType
pub type glColorP3uiv_t = unsafe extern "system" fn(type_: ColorPointerType, color: *const GLuint);

/// [glColorP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP4ui.xhtml)
/// * `type` group: ColorPointerType
pub type glColorP4ui_t = unsafe extern "system" fn(type_: ColorPointerType, color: GLuint);

/// [glColorP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP4uiv.xhtml)
/// * `type` group: ColorPointerType
pub type glColorP4uiv_t = unsafe extern "system" fn(type_: ColorPointerType, color: *const GLuint);

/// [glColorPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorPointer.xhtml)
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glColorPointer_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei, pointer: *const void);

/// [glColorPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorPointerEXT.xhtml)
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride,count)
pub type glColorPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// [glColorPointerListIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorPointerListIBM.xhtml)
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glColorPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// [glColorPointervINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorPointervINTEL.xhtml)
/// * `type` group: VertexPointerType
pub type glColorPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, pointer: *const [*mut void; 4]);

/// [glColorSubTable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorSubTable.xhtml)
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type,count)
pub type glColorSubTable_t = unsafe extern "system" fn(target: ColorTableTarget, start: GLsizei, count: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

/// [glColorSubTableEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorSubTableEXT.xhtml)
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type,count)
pub type glColorSubTableEXT_t = unsafe extern "system" fn(target: ColorTableTarget, start: GLsizei, count: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

/// [glColorTable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorTable.xhtml)
/// * `target` group: ColorTableTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(format,type,width)
pub type glColorTable_t = unsafe extern "system" fn(target: ColorTableTarget, internalformat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, table: *const void);

/// [glColorTableEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorTableEXT.xhtml)
/// * `target` group: ColorTableTarget
/// * `internalFormat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(format,type,width)
pub type glColorTableEXT_t = unsafe extern "system" fn(target: ColorTableTarget, internalFormat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, table: *const void);

/// [glColorTableParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorTableParameterfv.xhtml)
/// * `target` group: ColorTableTarget
/// * `pname` group: ColorTableParameterPNameSGI
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glColorTableParameterfv_t = unsafe extern "system" fn(target: ColorTableTarget, pname: ColorTableParameterPNameSGI, params: *const GLfloat);

/// [glColorTableParameterfvSGI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorTableParameterfvSGI.xhtml)
/// * `target` group: ColorTableTargetSGI
/// * `pname` group: ColorTableParameterPNameSGI
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glColorTableParameterfvSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, pname: ColorTableParameterPNameSGI, params: *const GLfloat);

/// [glColorTableParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorTableParameteriv.xhtml)
/// * `target` group: ColorTableTarget
/// * `pname` group: ColorTableParameterPNameSGI
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glColorTableParameteriv_t = unsafe extern "system" fn(target: ColorTableTarget, pname: ColorTableParameterPNameSGI, params: *const GLint);

/// [glColorTableParameterivSGI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorTableParameterivSGI.xhtml)
/// * `target` group: ColorTableTargetSGI
/// * `pname` group: ColorTableParameterPNameSGI
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glColorTableParameterivSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, pname: ColorTableParameterPNameSGI, params: *const GLint);

/// [glColorTableSGI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorTableSGI.xhtml)
/// * `target` group: ColorTableTargetSGI
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(format,type,width)
pub type glColorTableSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, internalformat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, table: *const void);

/// [glCombinerInputNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCombinerInputNV.xhtml)
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `variable` group: CombinerVariableNV
/// * `input` group: CombinerRegisterNV
/// * `mapping` group: CombinerMappingNV
/// * `componentUsage` group: CombinerComponentUsageNV
pub type glCombinerInputNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, variable: CombinerVariableNV, input: CombinerRegisterNV, mapping: CombinerMappingNV, componentUsage: CombinerComponentUsageNV);

/// [glCombinerOutputNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCombinerOutputNV.xhtml)
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `abOutput` group: CombinerRegisterNV
/// * `cdOutput` group: CombinerRegisterNV
/// * `sumOutput` group: CombinerRegisterNV
/// * `scale` group: CombinerScaleNV
/// * `bias` group: CombinerBiasNV
/// * `abDotProduct` group: Boolean
/// * `cdDotProduct` group: Boolean
/// * `muxSum` group: Boolean
pub type glCombinerOutputNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, abOutput: CombinerRegisterNV, cdOutput: CombinerRegisterNV, sumOutput: CombinerRegisterNV, scale: CombinerScaleNV, bias: CombinerBiasNV, abDotProduct: GLboolean, cdDotProduct: GLboolean, muxSum: GLboolean);

/// [glCombinerParameterfNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCombinerParameterfNV.xhtml)
/// * `pname` group: CombinerParameterNV
pub type glCombinerParameterfNV_t = unsafe extern "system" fn(pname: CombinerParameterNV, param: GLfloat);

/// [glCombinerParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCombinerParameterfvNV.xhtml)
/// * `pname` group: CombinerParameterNV
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glCombinerParameterfvNV_t = unsafe extern "system" fn(pname: CombinerParameterNV, params: *const GLfloat);

/// [glCombinerParameteriNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCombinerParameteriNV.xhtml)
/// * `pname` group: CombinerParameterNV
pub type glCombinerParameteriNV_t = unsafe extern "system" fn(pname: CombinerParameterNV, param: GLint);

/// [glCombinerParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCombinerParameterivNV.xhtml)
/// * `pname` group: CombinerParameterNV
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glCombinerParameterivNV_t = unsafe extern "system" fn(pname: CombinerParameterNV, params: *const GLint);

/// [glCombinerStageParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCombinerStageParameterfvNV.xhtml)
/// * `stage` group: CombinerStageNV
/// * `pname` group: CombinerParameterNV
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glCombinerStageParameterfvNV_t = unsafe extern "system" fn(stage: CombinerStageNV, pname: CombinerParameterNV, params: *const GLfloat);

/// [glCommandListSegmentsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCommandListSegmentsNV.xhtml)
pub type glCommandListSegmentsNV_t = unsafe extern "system" fn(list: GLuint, segments: GLuint);

/// [glCompileCommandListNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileCommandListNV.xhtml)
pub type glCompileCommandListNV_t = unsafe extern "system" fn(list: GLuint);

/// [glCompileShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml)
/// * `shader` class: shader
pub type glCompileShader_t = unsafe extern "system" fn(shader: GLuint);

/// [glCompileShaderARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShaderARB.xhtml)
/// * `shaderObj` group: handleARB
pub type glCompileShaderARB_t = unsafe extern "system" fn(shaderObj: GLhandleARB);

/// [glCompileShaderIncludeARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShaderIncludeARB.xhtml)
/// * `shader` class: shader
/// * `path` len: count
/// * `length` len: count
pub type glCompileShaderIncludeARB_t = unsafe extern "system" fn(shader: GLuint, count: GLsizei, path: *const *const GLchar, length: *const GLint);

/// [glCompressedMultiTexImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedMultiTexImage1DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// [glCompressedMultiTexImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedMultiTexImage2DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// [glCompressedMultiTexImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedMultiTexImage3DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedMultiTexImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// [glCompressedMultiTexSubImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedMultiTexSubImage1DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// [glCompressedMultiTexSubImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedMultiTexSubImage2DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// [glCompressedMultiTexSubImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedMultiTexSubImage3DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedMultiTexSubImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// [glCompressedTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// [glCompressedTexImage1DARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage1DARB.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage1DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// [glCompressedTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// [glCompressedTexImage2DARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage2DARB.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage2DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// [glCompressedTexImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// [glCompressedTexImage3DARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage3DARB.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage3DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// [glCompressedTexImage3DOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage3DOES.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `data` len: imageSize
pub type glCompressedTexImage3DOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// [glCompressedTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTexSubImage1DARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage1DARB.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage1DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTexSubImage2DARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage2DARB.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage2DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTexSubImage3DARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage3DARB.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage3DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTexSubImage3DOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage3DOES.xhtml)
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `data` len: imageSize
pub type glCompressedTexSubImage3DOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTextureImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureImage1DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedTextureImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// [glCompressedTextureImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureImage2DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedTextureImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// [glCompressedTextureImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureImage3DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedTextureImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// [glCompressedTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage1D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
pub type glCompressedTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTextureSubImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage1DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// [glCompressedTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage2D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
pub type glCompressedTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTextureSubImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage2DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedTextureSubImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// [glCompressedTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage3D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
pub type glCompressedTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// [glCompressedTextureSubImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage3DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedTextureSubImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// [glConservativeRasterParameterfNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConservativeRasterParameterfNV.xhtml)
pub type glConservativeRasterParameterfNV_t = unsafe extern "system" fn(pname: GLenum, value: GLfloat);

/// [glConservativeRasterParameteriNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConservativeRasterParameteriNV.xhtml)
pub type glConservativeRasterParameteriNV_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

/// [glConvolutionFilter1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionFilter1D.xhtml)
/// * `target` group: ConvolutionTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(format,type,width)
pub type glConvolutionFilter1D_t = unsafe extern "system" fn(target: ConvolutionTarget, internalformat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, image: *const void);

/// [glConvolutionFilter1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionFilter1DEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(format,type,width)
pub type glConvolutionFilter1DEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, internalformat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, image: *const void);

/// [glConvolutionFilter2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionFilter2D.xhtml)
/// * `target` group: ConvolutionTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(format,type,width,height)
pub type glConvolutionFilter2D_t = unsafe extern "system" fn(target: ConvolutionTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, image: *const void);

/// [glConvolutionFilter2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionFilter2DEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(format,type,width,height)
pub type glConvolutionFilter2DEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, internalformat: InternalFormat, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, image: *const void);

/// [glConvolutionParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameterf.xhtml)
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedFloat32
pub type glConvolutionParameterf_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: GLfloat);

/// [glConvolutionParameterfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameterfEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedFloat32
pub type glConvolutionParameterfEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: GLfloat);

/// [glConvolutionParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameterfv.xhtml)
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameterfv_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: *const GLfloat);

/// [glConvolutionParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameterfvEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameterfvEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *const GLfloat);

/// [glConvolutionParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameteri.xhtml)
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedInt32
pub type glConvolutionParameteri_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: GLint);

/// [glConvolutionParameteriEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameteriEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedInt32
pub type glConvolutionParameteriEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: GLint);

/// [glConvolutionParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameteriv.xhtml)
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameteriv_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: *const GLint);

/// [glConvolutionParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameterivEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameterivEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *const GLint);

/// [glConvolutionParameterxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameterxOES.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
pub type glConvolutionParameterxOES_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, param: GLfixed);

/// [glConvolutionParameterxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glConvolutionParameterxvOES.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameterxvOES_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *const GLfixed);

/// [glCopyBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyBufferSubData.xhtml)
/// * `readTarget` group: CopyBufferSubDataTarget
/// * `writeTarget` group: CopyBufferSubDataTarget
/// * `readOffset` group: BufferOffset
/// * `writeOffset` group: BufferOffset
/// * `size` group: BufferSize
pub type glCopyBufferSubData_t = unsafe extern "system" fn(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// [glCopyBufferSubDataNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyBufferSubDataNV.xhtml)
/// * `readTarget` group: CopyBufferSubDataTarget
/// * `writeTarget` group: CopyBufferSubDataTarget
/// * `readOffset` group: BufferOffset
/// * `writeOffset` group: BufferOffset
/// * `size` group: BufferSize
pub type glCopyBufferSubDataNV_t = unsafe extern "system" fn(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// [glCopyColorSubTable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyColorSubTable.xhtml)
/// * `target` group: ColorTableTarget
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyColorSubTable_t = unsafe extern "system" fn(target: ColorTableTarget, start: GLsizei, x: GLint, y: GLint, width: GLsizei);

/// [glCopyColorSubTableEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyColorSubTableEXT.xhtml)
/// * `target` group: ColorTableTarget
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyColorSubTableEXT_t = unsafe extern "system" fn(target: ColorTableTarget, start: GLsizei, x: GLint, y: GLint, width: GLsizei);

/// [glCopyColorTable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyColorTable.xhtml)
/// * `target` group: ColorTableTarget
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyColorTable_t = unsafe extern "system" fn(target: ColorTableTarget, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei);

/// [glCopyColorTableSGI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyColorTableSGI.xhtml)
/// * `target` group: ColorTableTargetSGI
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyColorTableSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei);

/// [glCopyConvolutionFilter1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyConvolutionFilter1D.xhtml)
/// * `target` group: ConvolutionTarget
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyConvolutionFilter1D_t = unsafe extern "system" fn(target: ConvolutionTarget, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei);

/// [glCopyConvolutionFilter1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyConvolutionFilter1DEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyConvolutionFilter1DEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei);

/// [glCopyConvolutionFilter2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyConvolutionFilter2D.xhtml)
/// * `target` group: ConvolutionTarget
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyConvolutionFilter2D_t = unsafe extern "system" fn(target: ConvolutionTarget, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyConvolutionFilter2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyConvolutionFilter2DEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyConvolutionFilter2DEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyImageSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyImageSubData.xhtml)
/// * `srcTarget` group: CopyImageSubDataTarget
/// * `dstTarget` group: CopyImageSubDataTarget
pub type glCopyImageSubData_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyImageSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyImageSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

/// [glCopyImageSubDataEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyImageSubDataEXT.xhtml)
/// * `srcTarget` group: CopyBufferSubDataTarget
/// * `dstTarget` group: CopyBufferSubDataTarget
pub type glCopyImageSubDataEXT_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyBufferSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyBufferSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

/// [glCopyImageSubDataNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyImageSubDataNV.xhtml)
/// * `srcTarget` group: CopyBufferSubDataTarget
/// * `dstTarget` group: CopyBufferSubDataTarget
pub type glCopyImageSubDataNV_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyBufferSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyBufferSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

/// [glCopyImageSubDataOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyImageSubDataOES.xhtml)
/// * `srcTarget` group: CopyBufferSubDataTarget
/// * `dstTarget` group: CopyBufferSubDataTarget
pub type glCopyImageSubDataOES_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyBufferSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyBufferSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

/// [glCopyMultiTexImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyMultiTexImage1DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

/// [glCopyMultiTexImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyMultiTexImage2DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

/// [glCopyMultiTexSubImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyMultiTexSubImage1DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// [glCopyMultiTexSubImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyMultiTexSubImage2DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyMultiTexSubImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyMultiTexSubImage3DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyMultiTexSubImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyNamedBufferSubData.xhtml)
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
/// * `size` group: BufferSize
pub type glCopyNamedBufferSubData_t = unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// [glCopyPathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyPathNV.xhtml)
/// * `resultPath` group: Path
/// * `srcPath` group: Path
pub type glCopyPathNV_t = unsafe extern "system" fn(resultPath: GLuint, srcPath: GLuint);

/// [glCopyPixels](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyPixels.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `type` group: PixelCopyType
pub type glCopyPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_: PixelCopyType);

/// [glCopyTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

/// [glCopyTexImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexImage1DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTexImage1DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

/// [glCopyTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

/// [glCopyTexImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexImage2DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTexImage2DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

/// [glCopyTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// [glCopyTexSubImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage1DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage1DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// [glCopyTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyTexSubImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage2DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage2DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyTexSubImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage3DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage3DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyTexSubImage3DOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage3DOES.xhtml)
pub type glCopyTexSubImage3DOES_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyTextureImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureImage1DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTextureImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

/// [glCopyTextureImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureImage2DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTextureImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

/// [glCopyTextureLevelsAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureLevelsAPPLE.xhtml)
pub type glCopyTextureLevelsAPPLE_t = unsafe extern "system" fn(destinationTexture: GLuint, sourceTexture: GLuint, sourceBaseLevel: GLint, sourceLevelCount: GLsizei);

/// [glCopyTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage1D.xhtml)
/// * `texture` class: texture
pub type glCopyTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// [glCopyTextureSubImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage1DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// [glCopyTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage2D.xhtml)
/// * `texture` class: texture
pub type glCopyTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyTextureSubImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage2DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTextureSubImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage3D.xhtml)
/// * `texture` class: texture
pub type glCopyTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCopyTextureSubImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage3DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTextureSubImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glCoverFillPathInstancedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCoverFillPathInstancedNV.xhtml)
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `coverMode` group: PathCoverMode
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(numPaths,transformType)
pub type glCoverFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, coverMode: PathCoverMode, transformType: PathTransformType, transformValues: *const GLfloat);

/// [glCoverFillPathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCoverFillPathNV.xhtml)
/// * `path` group: Path
/// * `coverMode` group: PathCoverMode
pub type glCoverFillPathNV_t = unsafe extern "system" fn(path: GLuint, coverMode: PathCoverMode);

/// [glCoverStrokePathInstancedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCoverStrokePathInstancedNV.xhtml)
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `coverMode` group: PathCoverMode
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(numPaths,transformType)
pub type glCoverStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, coverMode: PathCoverMode, transformType: PathTransformType, transformValues: *const GLfloat);

/// [glCoverStrokePathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCoverStrokePathNV.xhtml)
/// * `path` group: Path
/// * `coverMode` group: PathCoverMode
pub type glCoverStrokePathNV_t = unsafe extern "system" fn(path: GLuint, coverMode: PathCoverMode);

/// [glCoverageMaskNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCoverageMaskNV.xhtml)
/// * `mask` group: Boolean
pub type glCoverageMaskNV_t = unsafe extern "system" fn(mask: GLboolean);

/// [glCoverageModulationNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCoverageModulationNV.xhtml)
pub type glCoverageModulationNV_t = unsafe extern "system" fn(components: GLenum);

/// [glCoverageModulationTableNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCoverageModulationTableNV.xhtml)
/// * `v` len: n
pub type glCoverageModulationTableNV_t = unsafe extern "system" fn(n: GLsizei, v: *const GLfloat);

/// [glCoverageOperationNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCoverageOperationNV.xhtml)
pub type glCoverageOperationNV_t = unsafe extern "system" fn(operation: GLenum);

/// [glCreateBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateBuffers.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glCreateBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

/// [glCreateCommandListsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateCommandListsNV.xhtml)
/// * `lists` len: n
pub type glCreateCommandListsNV_t = unsafe extern "system" fn(n: GLsizei, lists: *mut GLuint);

/// [glCreateFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateFramebuffers.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glCreateFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

/// [glCreateMemoryObjectsEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateMemoryObjectsEXT.xhtml)
pub type glCreateMemoryObjectsEXT_t = unsafe extern "system" fn(n: GLsizei, memoryObjects: *mut GLuint);

/// [glCreatePerfQueryINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreatePerfQueryINTEL.xhtml)
pub type glCreatePerfQueryINTEL_t = unsafe extern "system" fn(queryId: GLuint, queryHandle: *mut GLuint);

/// [glCreateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml)
pub type glCreateProgram_t = unsafe extern "system" fn() -> GLuint;

/// [glCreateProgramObjectARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgramObjectARB.xhtml)
pub type glCreateProgramObjectARB_t = unsafe extern "system" fn() -> GLhandleARB;

/// [glCreateProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgramPipelines.xhtml)
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glCreateProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

/// [glCreateProgressFenceNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgressFenceNVX.xhtml)
pub type glCreateProgressFenceNVX_t = unsafe extern "system" fn() -> GLuint;

/// [glCreateQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateQueries.xhtml)
/// * `target` group: QueryTarget
/// * `ids` class: query
/// * `ids` len: n
pub type glCreateQueries_t = unsafe extern "system" fn(target: QueryTarget, n: GLsizei, ids: *mut GLuint);

/// [glCreateRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateRenderbuffers.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glCreateRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

/// [glCreateSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateSamplers.xhtml)
/// * `samplers` class: sampler
/// * `samplers` len: n
pub type glCreateSamplers_t = unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint);

/// [glCreateSemaphoresNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateSemaphoresNV.xhtml)
/// * `semaphores` len: n
pub type glCreateSemaphoresNV_t = unsafe extern "system" fn(n: GLsizei, semaphores: *mut GLuint);

/// [glCreateShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml)
/// * `type` group: ShaderType
pub type glCreateShader_t = unsafe extern "system" fn(type_: ShaderType) -> GLuint;

/// [glCreateShaderObjectARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShaderObjectARB.xhtml)
/// * `shaderType` group: ShaderType
pub type glCreateShaderObjectARB_t = unsafe extern "system" fn(shaderType: ShaderType) -> GLhandleARB;

/// [glCreateShaderProgramEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShaderProgramEXT.xhtml)
/// * `type` group: ShaderType
pub type glCreateShaderProgramEXT_t = unsafe extern "system" fn(type_: ShaderType, string: *const GLchar) -> GLuint;

/// [glCreateShaderProgramv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShaderProgramv.xhtml)
/// * `type` group: ShaderType
/// * `strings` len: count
pub type glCreateShaderProgramv_t = unsafe extern "system" fn(type_: ShaderType, count: GLsizei, strings: *const *const GLchar) -> GLuint;

/// [glCreateShaderProgramvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShaderProgramvEXT.xhtml)
/// * `type` group: ShaderType
/// * `strings` len: count
pub type glCreateShaderProgramvEXT_t = unsafe extern "system" fn(type_: ShaderType, count: GLsizei, strings: *const *mut GLchar) -> GLuint;

/// [glCreateStatesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateStatesNV.xhtml)
/// * `states` len: n
pub type glCreateStatesNV_t = unsafe extern "system" fn(n: GLsizei, states: *mut GLuint);

/// [glCreateSyncFromCLeventARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateSyncFromCLeventARB.xhtml)
/// * `context` group: cl_context
/// * `event` group: cl_event
pub type glCreateSyncFromCLeventARB_t = unsafe extern "system" fn(context: *mut _cl_context, event: *mut _cl_event, flags: GLbitfield) -> GLsync;

/// [glCreateTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateTextures.xhtml)
/// * `target` group: TextureTarget
/// * `textures` class: texture
/// * `textures` len: n
pub type glCreateTextures_t = unsafe extern "system" fn(target: TextureTarget, n: GLsizei, textures: *mut GLuint);

/// [glCreateTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateTransformFeedbacks.xhtml)
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glCreateTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// [glCreateVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateVertexArrays.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glCreateVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

/// [glCullFace](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCullFace.xhtml)
/// * `mode` group: CullFaceMode
pub type glCullFace_t = unsafe extern "system" fn(mode: CullFaceMode);

/// [glCullParameterdvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCullParameterdvEXT.xhtml)
/// * `pname` group: CullParameterEXT
pub type glCullParameterdvEXT_t = unsafe extern "system" fn(pname: CullParameterEXT, params: *mut [GLdouble; 4]);

/// [glCullParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCullParameterfvEXT.xhtml)
/// * `pname` group: CullParameterEXT
pub type glCullParameterfvEXT_t = unsafe extern "system" fn(pname: CullParameterEXT, params: *mut [GLfloat; 4]);

/// [glCurrentPaletteMatrixARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCurrentPaletteMatrixARB.xhtml)
pub type glCurrentPaletteMatrixARB_t = unsafe extern "system" fn(index: GLint);

/// [glCurrentPaletteMatrixOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCurrentPaletteMatrixOES.xhtml)
pub type glCurrentPaletteMatrixOES_t = unsafe extern "system" fn(matrixpaletteindex: GLuint);

/// [glDebugMessageCallback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallback.xhtml)
pub type glDebugMessageCallback_t = unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const void);

/// [glDebugMessageCallbackAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallbackAMD.xhtml)
pub type glDebugMessageCallbackAMD_t = unsafe extern "system" fn(callback: GLDEBUGPROCAMD, userParam: *mut void);

/// [glDebugMessageCallbackARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallbackARB.xhtml)
/// * `userParam` len: COMPSIZE(callback)
pub type glDebugMessageCallbackARB_t = unsafe extern "system" fn(callback: GLDEBUGPROCARB, userParam: *const void);

/// [glDebugMessageCallbackKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallbackKHR.xhtml)
pub type glDebugMessageCallbackKHR_t = unsafe extern "system" fn(callback: GLDEBUGPROCKHR, userParam: *const void);

/// [glDebugMessageControl](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControl.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
pub type glDebugMessageControl_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

/// [glDebugMessageControlARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControlARB.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
pub type glDebugMessageControlARB_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

/// [glDebugMessageControlKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControlKHR.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `enabled` group: Boolean
pub type glDebugMessageControlKHR_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

/// [glDebugMessageEnableAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageEnableAMD.xhtml)
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
pub type glDebugMessageEnableAMD_t = unsafe extern "system" fn(category: GLenum, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

/// [glDebugMessageInsert](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsert.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `buf` len: COMPSIZE(buf,length)
pub type glDebugMessageInsert_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);

/// [glDebugMessageInsertAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsertAMD.xhtml)
/// * `severity` group: DebugSeverity
/// * `buf` len: length
pub type glDebugMessageInsertAMD_t = unsafe extern "system" fn(category: GLenum, severity: DebugSeverity, id: GLuint, length: GLsizei, buf: *const GLchar);

/// [glDebugMessageInsertARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsertARB.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `buf` len: length
pub type glDebugMessageInsertARB_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);

/// [glDebugMessageInsertKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsertKHR.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
pub type glDebugMessageInsertKHR_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);

/// [glDeformSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeformSGIX.xhtml)
/// * `mask` group: FfdMaskSGIX
pub type glDeformSGIX_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glDeformationMap3dSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeformationMap3dSGIX.xhtml)
/// * `target` group: FfdTargetSGIX
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `uorder` group: CheckedInt32
/// * `v1` group: CoordD
/// * `v2` group: CoordD
/// * `vorder` group: CheckedInt32
/// * `w1` group: CoordD
/// * `w2` group: CoordD
/// * `worder` group: CheckedInt32
/// * `points` group: CoordD
/// * `points` len:
///   COMPSIZE(target,ustride,uorder,vstride,vorder,wstride,worder)
pub type glDeformationMap3dSGIX_t = unsafe extern "system" fn(target: FfdTargetSGIX, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, w1: GLdouble, w2: GLdouble, wstride: GLint, worder: GLint, points: *const GLdouble);

/// [glDeformationMap3fSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeformationMap3fSGIX.xhtml)
/// * `target` group: FfdTargetSGIX
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `uorder` group: CheckedInt32
/// * `v1` group: CoordF
/// * `v2` group: CoordF
/// * `vorder` group: CheckedInt32
/// * `w1` group: CoordF
/// * `w2` group: CoordF
/// * `worder` group: CheckedInt32
/// * `points` group: CoordF
/// * `points` len:
///   COMPSIZE(target,ustride,uorder,vstride,vorder,wstride,worder)
pub type glDeformationMap3fSGIX_t = unsafe extern "system" fn(target: FfdTargetSGIX, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, w1: GLfloat, w2: GLfloat, wstride: GLint, worder: GLint, points: *const GLfloat);

/// [glDeleteAsyncMarkersSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteAsyncMarkersSGIX.xhtml)
pub type glDeleteAsyncMarkersSGIX_t = unsafe extern "system" fn(marker: GLuint, range: GLsizei);

/// [glDeleteBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteBuffers.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glDeleteBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);

/// [glDeleteBuffersARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteBuffersARB.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glDeleteBuffersARB_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);

/// [glDeleteCommandListsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteCommandListsNV.xhtml)
/// * `lists` len: n
pub type glDeleteCommandListsNV_t = unsafe extern "system" fn(n: GLsizei, lists: *const GLuint);

/// [glDeleteFencesAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteFencesAPPLE.xhtml)
/// * `fences` group: FenceNV
/// * `fences` len: n
pub type glDeleteFencesAPPLE_t = unsafe extern "system" fn(n: GLsizei, fences: *const GLuint);

/// [glDeleteFencesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteFencesNV.xhtml)
/// * `fences` group: FenceNV
/// * `fences` len: n
pub type glDeleteFencesNV_t = unsafe extern "system" fn(n: GLsizei, fences: *const GLuint);

/// [glDeleteFragmentShaderATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteFragmentShaderATI.xhtml)
pub type glDeleteFragmentShaderATI_t = unsafe extern "system" fn(id: GLuint);

/// [glDeleteFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteFramebuffers.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glDeleteFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

/// [glDeleteFramebuffersEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteFramebuffersEXT.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glDeleteFramebuffersEXT_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

/// [glDeleteFramebuffersOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteFramebuffersOES.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glDeleteFramebuffersOES_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

/// [glDeleteLists](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteLists.xhtml)
/// * `list` group: List
/// * `list` class: display list
pub type glDeleteLists_t = unsafe extern "system" fn(list: GLuint, range: GLsizei);

/// [glDeleteMemoryObjectsEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteMemoryObjectsEXT.xhtml)
/// * `memoryObjects` len: n
pub type glDeleteMemoryObjectsEXT_t = unsafe extern "system" fn(n: GLsizei, memoryObjects: *const GLuint);

/// [glDeleteNamedStringARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteNamedStringARB.xhtml)
/// * `name` len: namelen
pub type glDeleteNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar);

/// [glDeleteNamesAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteNamesAMD.xhtml)
/// * `names` len: num
pub type glDeleteNamesAMD_t = unsafe extern "system" fn(identifier: GLenum, num: GLuint, names: *const GLuint);

/// [glDeleteObjectARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteObjectARB.xhtml)
/// * `obj` group: handleARB
pub type glDeleteObjectARB_t = unsafe extern "system" fn(obj: GLhandleARB);

/// [glDeleteOcclusionQueriesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteOcclusionQueriesNV.xhtml)
/// * `ids` len: n
pub type glDeleteOcclusionQueriesNV_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// [glDeletePathsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeletePathsNV.xhtml)
/// * `path` group: Path
pub type glDeletePathsNV_t = unsafe extern "system" fn(path: GLuint, range: GLsizei);

/// [glDeletePerfMonitorsAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeletePerfMonitorsAMD.xhtml)
/// * `monitors` len: n
pub type glDeletePerfMonitorsAMD_t = unsafe extern "system" fn(n: GLsizei, monitors: *mut GLuint);

/// [glDeletePerfQueryINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeletePerfQueryINTEL.xhtml)
pub type glDeletePerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

/// [glDeleteProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgram.xhtml)
/// * `program` class: program
pub type glDeleteProgram_t = unsafe extern "system" fn(program: GLuint);

/// [glDeleteProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgramPipelines.xhtml)
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glDeleteProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);

/// [glDeleteProgramPipelinesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgramPipelinesEXT.xhtml)
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glDeleteProgramPipelinesEXT_t = unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);

/// [glDeleteProgramsARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgramsARB.xhtml)
/// * `programs` class: program
/// * `programs` len: n
pub type glDeleteProgramsARB_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

/// [glDeleteProgramsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgramsNV.xhtml)
/// * `programs` class: program
/// * `programs` len: n
pub type glDeleteProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

/// [glDeleteQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteQueries.xhtml)
/// * `ids` class: query
/// * `ids` len: n
pub type glDeleteQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// [glDeleteQueriesARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteQueriesARB.xhtml)
/// * `ids` class: query
/// * `ids` len: n
pub type glDeleteQueriesARB_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// [glDeleteQueriesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteQueriesEXT.xhtml)
/// * `ids` class: query
/// * `ids` len: n
pub type glDeleteQueriesEXT_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// [glDeleteQueryResourceTagNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteQueryResourceTagNV.xhtml)
/// * `tagIds` len: n
pub type glDeleteQueryResourceTagNV_t = unsafe extern "system" fn(n: GLsizei, tagIds: *const GLint);

/// [glDeleteRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteRenderbuffers.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glDeleteRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

/// [glDeleteRenderbuffersEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteRenderbuffersEXT.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glDeleteRenderbuffersEXT_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

/// [glDeleteRenderbuffersOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteRenderbuffersOES.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glDeleteRenderbuffersOES_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

/// [glDeleteSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteSamplers.xhtml)
/// * `samplers` class: sampler
/// * `samplers` len: count
pub type glDeleteSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);

/// [glDeleteSemaphoresEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteSemaphoresEXT.xhtml)
/// * `semaphores` len: n
pub type glDeleteSemaphoresEXT_t = unsafe extern "system" fn(n: GLsizei, semaphores: *const GLuint);

/// [glDeleteShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml)
/// * `shader` class: shader
pub type glDeleteShader_t = unsafe extern "system" fn(shader: GLuint);

/// [glDeleteStatesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteStatesNV.xhtml)
/// * `states` len: n
pub type glDeleteStatesNV_t = unsafe extern "system" fn(n: GLsizei, states: *const GLuint);

/// [glDeleteSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteSync.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
pub type glDeleteSync_t = unsafe extern "system" fn(sync: GLsync);

/// [glDeleteSyncAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteSyncAPPLE.xhtml)
/// * `sync` class: sync
pub type glDeleteSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync);

/// [glDeleteTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteTextures.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
pub type glDeleteTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);

/// [glDeleteTexturesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteTexturesEXT.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
pub type glDeleteTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);

/// [glDeleteTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteTransformFeedbacks.xhtml)
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glDeleteTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// [glDeleteTransformFeedbacksNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteTransformFeedbacksNV.xhtml)
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glDeleteTransformFeedbacksNV_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// [glDeleteVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glDeleteVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

/// [glDeleteVertexArraysAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArraysAPPLE.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glDeleteVertexArraysAPPLE_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

/// [glDeleteVertexArraysOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArraysOES.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glDeleteVertexArraysOES_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

/// [glDeleteVertexShaderEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexShaderEXT.xhtml)
pub type glDeleteVertexShaderEXT_t = unsafe extern "system" fn(id: GLuint);

/// [glDepthBoundsEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthBoundsEXT.xhtml)
/// * `zmin` group: ClampedFloat64
/// * `zmax` group: ClampedFloat64
pub type glDepthBoundsEXT_t = unsafe extern "system" fn(zmin: GLclampd, zmax: GLclampd);

/// [glDepthBoundsdNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthBoundsdNV.xhtml)
pub type glDepthBoundsdNV_t = unsafe extern "system" fn(zmin: GLdouble, zmax: GLdouble);

/// [glDepthFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthFunc.xhtml)
/// * `func` group: DepthFunction
pub type glDepthFunc_t = unsafe extern "system" fn(func: DepthFunction);

/// [glDepthMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthMask.xhtml)
/// * `flag` group: Boolean
pub type glDepthMask_t = unsafe extern "system" fn(flag: GLboolean);

/// [glDepthRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRange.xhtml)
pub type glDepthRange_t = unsafe extern "system" fn(n: GLdouble, f: GLdouble);

/// [glDepthRangeArraydvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeArraydvNV.xhtml)
pub type glDepthRangeArraydvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);

/// [glDepthRangeArrayfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeArrayfvNV.xhtml)
pub type glDepthRangeArrayfvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// [glDepthRangeArrayfvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeArrayfvOES.xhtml)
pub type glDepthRangeArrayfvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// [glDepthRangeArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeArrayv.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glDepthRangeArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);

/// [glDepthRangeIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeIndexed.xhtml)
pub type glDepthRangeIndexed_t = unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);

/// [glDepthRangeIndexeddNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeIndexeddNV.xhtml)
pub type glDepthRangeIndexeddNV_t = unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);

/// [glDepthRangeIndexedfNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeIndexedfNV.xhtml)
pub type glDepthRangeIndexedfNV_t = unsafe extern "system" fn(index: GLuint, n: GLfloat, f: GLfloat);

/// [glDepthRangeIndexedfOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeIndexedfOES.xhtml)
pub type glDepthRangeIndexedfOES_t = unsafe extern "system" fn(index: GLuint, n: GLfloat, f: GLfloat);

/// [glDepthRangedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangedNV.xhtml)
pub type glDepthRangedNV_t = unsafe extern "system" fn(zNear: GLdouble, zFar: GLdouble);

/// [glDepthRangef](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangef.xhtml)
pub type glDepthRangef_t = unsafe extern "system" fn(n: GLfloat, f: GLfloat);

/// [glDepthRangefOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangefOES.xhtml)
/// * `n` group: ClampedFloat32
/// * `f` group: ClampedFloat32
pub type glDepthRangefOES_t = unsafe extern "system" fn(n: GLclampf, f: GLclampf);

/// [glDepthRangex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangex.xhtml)
pub type glDepthRangex_t = unsafe extern "system" fn(n: GLfixed, f: GLfixed);

/// [glDepthRangexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangexOES.xhtml)
/// * `n` group: ClampedFixed
/// * `f` group: ClampedFixed
pub type glDepthRangexOES_t = unsafe extern "system" fn(n: GLfixed, f: GLfixed);

/// [glDetachObjectARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDetachObjectARB.xhtml)
/// * `containerObj` group: handleARB
/// * `attachedObj` group: handleARB
pub type glDetachObjectARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, attachedObj: GLhandleARB);

/// [glDetachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDetachShader.xhtml)
/// * `program` class: program
/// * `shader` class: shader
pub type glDetachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);

/// [glDetailTexFuncSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDetailTexFuncSGIS.xhtml)
/// * `target` group: TextureTarget
/// * `points` len: n*2
pub type glDetailTexFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, n: GLsizei, points: *const GLfloat);

/// [glDisable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisable.xhtml)
/// * `cap` group: EnableCap
pub type glDisable_t = unsafe extern "system" fn(cap: EnableCap);

/// [glDisableClientState](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableClientState.xhtml)
/// * `array` group: EnableCap
pub type glDisableClientState_t = unsafe extern "system" fn(array: EnableCap);

/// [glDisableClientStateIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableClientStateIndexedEXT.xhtml)
/// * `array` group: EnableCap
pub type glDisableClientStateIndexedEXT_t = unsafe extern "system" fn(array: EnableCap, index: GLuint);

/// [glDisableClientStateiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableClientStateiEXT.xhtml)
/// * `array` group: EnableCap
pub type glDisableClientStateiEXT_t = unsafe extern "system" fn(array: EnableCap, index: GLuint);

/// [glDisableDriverControlQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableDriverControlQCOM.xhtml)
pub type glDisableDriverControlQCOM_t = unsafe extern "system" fn(driverControl: GLuint);

/// [glDisableIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableIndexedEXT.xhtml)
/// * `target` group: EnableCap
pub type glDisableIndexedEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glDisableVariantClientStateEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVariantClientStateEXT.xhtml)
pub type glDisableVariantClientStateEXT_t = unsafe extern "system" fn(id: GLuint);

/// [glDisableVertexArrayAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexArrayAttrib.xhtml)
/// * `vaobj` class: vertex array
pub type glDisableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

/// [glDisableVertexArrayAttribEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexArrayAttribEXT.xhtml)
/// * `vaobj` class: vertex array
pub type glDisableVertexArrayAttribEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

/// [glDisableVertexArrayEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexArrayEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `array` group: EnableCap
pub type glDisableVertexArrayEXT_t = unsafe extern "system" fn(vaobj: GLuint, array: EnableCap);

/// [glDisableVertexAttribAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexAttribAPPLE.xhtml)
pub type glDisableVertexAttribAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum);

/// [glDisableVertexAttribArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexAttribArray.xhtml)
pub type glDisableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

/// [glDisableVertexAttribArrayARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexAttribArrayARB.xhtml)
pub type glDisableVertexAttribArrayARB_t = unsafe extern "system" fn(index: GLuint);

/// [glDisablei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisablei.xhtml)
/// * `target` group: EnableCap
pub type glDisablei_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glDisableiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableiEXT.xhtml)
/// * `target` group: EnableCap
pub type glDisableiEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glDisableiNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableiNV.xhtml)
/// * `target` group: EnableCap
pub type glDisableiNV_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glDisableiOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableiOES.xhtml)
/// * `target` group: EnableCap
pub type glDisableiOES_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glDiscardFramebufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDiscardFramebufferEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachments` group: InvalidateFramebufferAttachment
/// * `attachments` len: numAttachments
pub type glDiscardFramebufferEXT_t = unsafe extern "system" fn(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment);

/// [glDispatchCompute](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchCompute.xhtml)
pub type glDispatchCompute_t = unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);

/// [glDispatchComputeGroupSizeARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchComputeGroupSizeARB.xhtml)
pub type glDispatchComputeGroupSizeARB_t = unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint, group_size_x: GLuint, group_size_y: GLuint, group_size_z: GLuint);

/// [glDispatchComputeIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchComputeIndirect.xhtml)
/// * `indirect` group: BufferOffset
pub type glDispatchComputeIndirect_t = unsafe extern "system" fn(indirect: GLintptr);

/// [glDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArrays_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei);

/// [glDrawArraysEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysEXT.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysEXT_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei);

/// [glDrawArraysIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysIndirect.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void);

/// [glDrawArraysInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstanced.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei);

/// [glDrawArraysInstancedANGLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstancedANGLE.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedANGLE_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, primcount: GLsizei);

/// [glDrawArraysInstancedARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstancedARB.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedARB_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, primcount: GLsizei);

/// [glDrawArraysInstancedBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstancedBaseInstance.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);

/// [glDrawArraysInstancedBaseInstanceEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstancedBaseInstanceEXT.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedBaseInstanceEXT_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);

/// [glDrawArraysInstancedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstancedEXT.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedEXT_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLint, count: GLsizei, primcount: GLsizei);

/// [glDrawArraysInstancedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstancedNV.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedNV_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, primcount: GLsizei);

/// [glDrawBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffer.xhtml)
/// * `buf` group: DrawBufferMode
pub type glDrawBuffer_t = unsafe extern "system" fn(buf: DrawBufferMode);

/// [glDrawBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffers.xhtml)
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
pub type glDrawBuffers_t = unsafe extern "system" fn(n: GLsizei, bufs: *const DrawBufferMode);

/// [glDrawBuffersARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffersARB.xhtml)
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
pub type glDrawBuffersARB_t = unsafe extern "system" fn(n: GLsizei, bufs: *const DrawBufferMode);

/// [glDrawBuffersATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffersATI.xhtml)
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
pub type glDrawBuffersATI_t = unsafe extern "system" fn(n: GLsizei, bufs: *const DrawBufferMode);

/// [glDrawBuffersEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffersEXT.xhtml)
/// * `bufs` len: n
pub type glDrawBuffersEXT_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

/// [glDrawBuffersIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffersIndexedEXT.xhtml)
/// * `location` len: n
/// * `indices` len: n
pub type glDrawBuffersIndexedEXT_t = unsafe extern "system" fn(n: GLint, location: *const GLenum, indices: *const GLint);

/// [glDrawBuffersNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffersNV.xhtml)
/// * `bufs` len: n
pub type glDrawBuffersNV_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

/// [glDrawCommandsAddressNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawCommandsAddressNV.xhtml)
pub type glDrawCommandsAddressNV_t = unsafe extern "system" fn(primitiveMode: GLenum, indirects: *const GLuint64, sizes: *const GLsizei, count: GLuint);

/// [glDrawCommandsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawCommandsNV.xhtml)
pub type glDrawCommandsNV_t = unsafe extern "system" fn(primitiveMode: GLenum, buffer: GLuint, indirects: *const GLintptr, sizes: *const GLsizei, count: GLuint);

/// [glDrawCommandsStatesAddressNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawCommandsStatesAddressNV.xhtml)
pub type glDrawCommandsStatesAddressNV_t = unsafe extern "system" fn(indirects: *const GLuint64, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

/// [glDrawCommandsStatesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawCommandsStatesNV.xhtml)
/// * `buffer` class: buffer
pub type glDrawCommandsStatesNV_t = unsafe extern "system" fn(buffer: GLuint, indirects: *const GLintptr, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

/// [glDrawElementArrayAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementArrayAPPLE.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawElementArrayAPPLE_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei);

/// [glDrawElementArrayATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementArrayATI.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawElementArrayATI_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei);

/// [glDrawElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElements.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElements_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void);

/// [glDrawElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsBaseVertex.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// [glDrawElementsBaseVertexEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsBaseVertexEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// [glDrawElementsBaseVertexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsBaseVertexOES.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsBaseVertexOES_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// [glDrawElementsIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsIndirect.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glDrawElementsIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void);

/// [glDrawElementsInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstanced.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei);

/// [glDrawElementsInstancedANGLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedANGLE.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedANGLE_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, primcount: GLsizei);

/// [glDrawElementsInstancedARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedARB.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedARB_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, primcount: GLsizei);

/// [glDrawElementsInstancedBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseInstance.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: count
pub type glDrawElementsInstancedBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, instancecount: GLsizei, baseinstance: GLuint);

/// [glDrawElementsInstancedBaseInstanceEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseInstanceEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: count
pub type glDrawElementsInstancedBaseInstanceEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, instancecount: GLsizei, baseinstance: GLuint);

/// [glDrawElementsInstancedBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertex.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);

/// [glDrawElementsInstancedBaseVertexBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertexBaseInstance.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: count
pub type glDrawElementsInstancedBaseVertexBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);

/// [glDrawElementsInstancedBaseVertexBaseInstanceEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertexBaseInstanceEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: count
pub type glDrawElementsInstancedBaseVertexBaseInstanceEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);

/// [glDrawElementsInstancedBaseVertexEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertexEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedBaseVertexEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);

/// [glDrawElementsInstancedBaseVertexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertexOES.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedBaseVertexOES_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);

/// [glDrawElementsInstancedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, primcount: GLsizei);

/// [glDrawElementsInstancedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedNV.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedNV_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, primcount: GLsizei);

/// [glDrawMeshArraysSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawMeshArraysSUN.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawMeshArraysSUN_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, width: GLsizei);

/// [glDrawMeshTasksIndirectNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawMeshTasksIndirectNV.xhtml)
pub type glDrawMeshTasksIndirectNV_t = unsafe extern "system" fn(indirect: GLintptr);

/// [glDrawMeshTasksNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawMeshTasksNV.xhtml)
pub type glDrawMeshTasksNV_t = unsafe extern "system" fn(first: GLuint, count: GLuint);

/// [glDrawPixels](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawPixels.xhtml)
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glDrawPixels_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glDrawRangeElementArrayAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElementArrayAPPLE.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawRangeElementArrayAPPLE_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, first: GLint, count: GLsizei);

/// [glDrawRangeElementArrayATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElementArrayATI.xhtml)
/// * `mode` group: PrimitiveType
pub type glDrawRangeElementArrayATI_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei);

/// [glDrawRangeElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElements.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElements_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void);

/// [glDrawRangeElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElementsBaseVertex.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// [glDrawRangeElementsBaseVertexEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElementsBaseVertexEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// [glDrawRangeElementsBaseVertexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElementsBaseVertexOES.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElementsBaseVertexOES_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// [glDrawRangeElementsEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElementsEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElementsEXT_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void);

/// [glDrawTexfOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTexfOES.xhtml)
pub type glDrawTexfOES_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, width: GLfloat, height: GLfloat);

/// [glDrawTexfvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTexfvOES.xhtml)
pub type glDrawTexfvOES_t = unsafe extern "system" fn(coords: *const [GLfloat; 5]);

/// [glDrawTexiOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTexiOES.xhtml)
pub type glDrawTexiOES_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, width: GLint, height: GLint);

/// [glDrawTexivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTexivOES.xhtml)
pub type glDrawTexivOES_t = unsafe extern "system" fn(coords: *const [GLint; 5]);

/// [glDrawTexsOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTexsOES.xhtml)
pub type glDrawTexsOES_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, width: GLshort, height: GLshort);

/// [glDrawTexsvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTexsvOES.xhtml)
pub type glDrawTexsvOES_t = unsafe extern "system" fn(coords: *const [GLshort; 5]);

/// [glDrawTextureNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTextureNV.xhtml)
/// * `texture` class: texture
/// * `sampler` class: sampler
pub type glDrawTextureNV_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint, x0: GLfloat, y0: GLfloat, x1: GLfloat, y1: GLfloat, z: GLfloat, s0: GLfloat, t0: GLfloat, s1: GLfloat, t1: GLfloat);

/// [glDrawTexxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTexxOES.xhtml)
pub type glDrawTexxOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed, width: GLfixed, height: GLfixed);

/// [glDrawTexxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTexxvOES.xhtml)
pub type glDrawTexxvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 5]);

/// [glDrawTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedback.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedback_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint);

/// [glDrawTransformFeedbackEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackEXT_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint);

/// [glDrawTransformFeedbackInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackInstanced.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, instancecount: GLsizei);

/// [glDrawTransformFeedbackInstancedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackInstancedEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackInstancedEXT_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, instancecount: GLsizei);

/// [glDrawTransformFeedbackNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackNV.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackNV_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint);

/// [glDrawTransformFeedbackStream](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackStream.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackStream_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, stream: GLuint);

/// [glDrawTransformFeedbackStreamInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackStreamInstanced.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackStreamInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, stream: GLuint, instancecount: GLsizei);

/// [glDrawVkImageNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawVkImageNV.xhtml)
/// * `sampler` class: sampler
pub type glDrawVkImageNV_t = unsafe extern "system" fn(vkImage: GLuint64, sampler: GLuint, x0: GLfloat, y0: GLfloat, x1: GLfloat, y1: GLfloat, z: GLfloat, s0: GLfloat, t0: GLfloat, s1: GLfloat, t1: GLfloat);

/// [glEGLImageTargetRenderbufferStorageOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEGLImageTargetRenderbufferStorageOES.xhtml)
pub type glEGLImageTargetRenderbufferStorageOES_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES);

/// [glEGLImageTargetTexStorageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEGLImageTargetTexStorageEXT.xhtml)
pub type glEGLImageTargetTexStorageEXT_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES, attrib_list: *const GLint);

/// [glEGLImageTargetTexture2DOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEGLImageTargetTexture2DOES.xhtml)
pub type glEGLImageTargetTexture2DOES_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES);

/// [glEGLImageTargetTextureStorageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEGLImageTargetTextureStorageEXT.xhtml)
/// * `texture` class: texture
pub type glEGLImageTargetTextureStorageEXT_t = unsafe extern "system" fn(texture: GLuint, image: GLeglImageOES, attrib_list: *const GLint);

/// [glEdgeFlag](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEdgeFlag.xhtml)
/// * `flag` group: Boolean
pub type glEdgeFlag_t = unsafe extern "system" fn(flag: GLboolean);

/// [glEdgeFlagFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEdgeFlagFormatNV.xhtml)
pub type glEdgeFlagFormatNV_t = unsafe extern "system" fn(stride: GLsizei);

/// [glEdgeFlagPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEdgeFlagPointer.xhtml)
/// * `pointer` len: COMPSIZE(stride)
pub type glEdgeFlagPointer_t = unsafe extern "system" fn(stride: GLsizei, pointer: *const void);

/// [glEdgeFlagPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEdgeFlagPointerEXT.xhtml)
/// * `pointer` group: Boolean
/// * `pointer` len: COMPSIZE(stride,count)
pub type glEdgeFlagPointerEXT_t = unsafe extern "system" fn(stride: GLsizei, count: GLsizei, pointer: *const GLboolean);

/// [glEdgeFlagPointerListIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEdgeFlagPointerListIBM.xhtml)
/// * `pointer` group: BooleanPointer
/// * `pointer` len: COMPSIZE(stride)
pub type glEdgeFlagPointerListIBM_t = unsafe extern "system" fn(stride: GLint, pointer: *const *mut GLboolean, ptrstride: GLint);

/// [glEdgeFlagv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEdgeFlagv.xhtml)
/// * `flag` group: Boolean
pub type glEdgeFlagv_t = unsafe extern "system" fn(flag: *const GLboolean);

/// [glElementPointerAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glElementPointerAPPLE.xhtml)
/// * `type` group: ElementPointerTypeATI
/// * `pointer` len: COMPSIZE(type)
pub type glElementPointerAPPLE_t = unsafe extern "system" fn(type_: ElementPointerTypeATI, pointer: *const void);

/// [glElementPointerATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glElementPointerATI.xhtml)
/// * `type` group: ElementPointerTypeATI
/// * `pointer` len: COMPSIZE(type)
pub type glElementPointerATI_t = unsafe extern "system" fn(type_: ElementPointerTypeATI, pointer: *const void);

/// [glEnable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml)
/// * `cap` group: EnableCap
pub type glEnable_t = unsafe extern "system" fn(cap: EnableCap);

/// [glEnableClientState](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableClientState.xhtml)
/// * `array` group: EnableCap
pub type glEnableClientState_t = unsafe extern "system" fn(array: EnableCap);

/// [glEnableClientStateIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableClientStateIndexedEXT.xhtml)
/// * `array` group: EnableCap
pub type glEnableClientStateIndexedEXT_t = unsafe extern "system" fn(array: EnableCap, index: GLuint);

/// [glEnableClientStateiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableClientStateiEXT.xhtml)
/// * `array` group: EnableCap
pub type glEnableClientStateiEXT_t = unsafe extern "system" fn(array: EnableCap, index: GLuint);

/// [glEnableDriverControlQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableDriverControlQCOM.xhtml)
pub type glEnableDriverControlQCOM_t = unsafe extern "system" fn(driverControl: GLuint);

/// [glEnableIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableIndexedEXT.xhtml)
/// * `target` group: EnableCap
pub type glEnableIndexedEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glEnableVariantClientStateEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVariantClientStateEXT.xhtml)
pub type glEnableVariantClientStateEXT_t = unsafe extern "system" fn(id: GLuint);

/// [glEnableVertexArrayAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexArrayAttrib.xhtml)
/// * `vaobj` class: vertex array
pub type glEnableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

/// [glEnableVertexArrayAttribEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexArrayAttribEXT.xhtml)
/// * `vaobj` class: vertex array
pub type glEnableVertexArrayAttribEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

/// [glEnableVertexArrayEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexArrayEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `array` group: EnableCap
pub type glEnableVertexArrayEXT_t = unsafe extern "system" fn(vaobj: GLuint, array: EnableCap);

/// [glEnableVertexAttribAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribAPPLE.xhtml)
pub type glEnableVertexAttribAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum);

/// [glEnableVertexAttribArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml)
pub type glEnableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

/// [glEnableVertexAttribArrayARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArrayARB.xhtml)
pub type glEnableVertexAttribArrayARB_t = unsafe extern "system" fn(index: GLuint);

/// [glEnablei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnablei.xhtml)
/// * `target` group: EnableCap
pub type glEnablei_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glEnableiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableiEXT.xhtml)
/// * `target` group: EnableCap
pub type glEnableiEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glEnableiNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableiNV.xhtml)
/// * `target` group: EnableCap
pub type glEnableiNV_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glEnableiOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableiOES.xhtml)
/// * `target` group: EnableCap
pub type glEnableiOES_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// [glEnd](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnd.xhtml)
pub type glEnd_t = unsafe extern "system" fn();

/// [glEndConditionalRender](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndConditionalRender.xhtml)
pub type glEndConditionalRender_t = unsafe extern "system" fn();

/// [glEndConditionalRenderNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndConditionalRenderNV.xhtml)
pub type glEndConditionalRenderNV_t = unsafe extern "system" fn();

/// [glEndConditionalRenderNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndConditionalRenderNVX.xhtml)
pub type glEndConditionalRenderNVX_t = unsafe extern "system" fn();

/// [glEndFragmentShaderATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndFragmentShaderATI.xhtml)
pub type glEndFragmentShaderATI_t = unsafe extern "system" fn();

/// [glEndList](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndList.xhtml)
pub type glEndList_t = unsafe extern "system" fn();

/// [glEndOcclusionQueryNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndOcclusionQueryNV.xhtml)
pub type glEndOcclusionQueryNV_t = unsafe extern "system" fn();

/// [glEndPerfMonitorAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndPerfMonitorAMD.xhtml)
pub type glEndPerfMonitorAMD_t = unsafe extern "system" fn(monitor: GLuint);

/// [glEndPerfQueryINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndPerfQueryINTEL.xhtml)
pub type glEndPerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

/// [glEndQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndQuery.xhtml)
/// * `target` group: QueryTarget
pub type glEndQuery_t = unsafe extern "system" fn(target: QueryTarget);

/// [glEndQueryARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndQueryARB.xhtml)
/// * `target` group: QueryTarget
pub type glEndQueryARB_t = unsafe extern "system" fn(target: QueryTarget);

/// [glEndQueryEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndQueryEXT.xhtml)
/// * `target` group: QueryTarget
pub type glEndQueryEXT_t = unsafe extern "system" fn(target: QueryTarget);

/// [glEndQueryIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndQueryIndexed.xhtml)
/// * `target` group: QueryTarget
pub type glEndQueryIndexed_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint);

/// [glEndTilingQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndTilingQCOM.xhtml)
/// * `preserveMask` group: BufferBitQCOM
pub type glEndTilingQCOM_t = unsafe extern "system" fn(preserveMask: GLbitfield);

/// [glEndTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndTransformFeedback.xhtml)
pub type glEndTransformFeedback_t = unsafe extern "system" fn();

/// [glEndTransformFeedbackEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndTransformFeedbackEXT.xhtml)
pub type glEndTransformFeedbackEXT_t = unsafe extern "system" fn();

/// [glEndTransformFeedbackNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndTransformFeedbackNV.xhtml)
pub type glEndTransformFeedbackNV_t = unsafe extern "system" fn();

/// [glEndVertexShaderEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndVertexShaderEXT.xhtml)
pub type glEndVertexShaderEXT_t = unsafe extern "system" fn();

/// [glEndVideoCaptureNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndVideoCaptureNV.xhtml)
pub type glEndVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint);

/// [glEvalCoord1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord1d.xhtml)
/// * `u` group: CoordD
pub type glEvalCoord1d_t = unsafe extern "system" fn(u: GLdouble);

/// [glEvalCoord1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord1dv.xhtml)
/// * `u` group: CoordD
pub type glEvalCoord1dv_t = unsafe extern "system" fn(u: *const GLdouble);

/// [glEvalCoord1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord1f.xhtml)
/// * `u` group: CoordF
pub type glEvalCoord1f_t = unsafe extern "system" fn(u: GLfloat);

/// [glEvalCoord1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord1fv.xhtml)
/// * `u` group: CoordF
pub type glEvalCoord1fv_t = unsafe extern "system" fn(u: *const GLfloat);

/// [glEvalCoord1xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord1xOES.xhtml)
pub type glEvalCoord1xOES_t = unsafe extern "system" fn(u: GLfixed);

/// [glEvalCoord1xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord1xvOES.xhtml)
pub type glEvalCoord1xvOES_t = unsafe extern "system" fn(coords: *const GLfixed);

/// [glEvalCoord2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord2d.xhtml)
/// * `u` group: CoordD
/// * `v` group: CoordD
pub type glEvalCoord2d_t = unsafe extern "system" fn(u: GLdouble, v: GLdouble);

/// [glEvalCoord2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord2dv.xhtml)
/// * `u` group: CoordD
pub type glEvalCoord2dv_t = unsafe extern "system" fn(u: *const [GLdouble; 2]);

/// [glEvalCoord2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord2f.xhtml)
/// * `u` group: CoordF
/// * `v` group: CoordF
pub type glEvalCoord2f_t = unsafe extern "system" fn(u: GLfloat, v: GLfloat);

/// [glEvalCoord2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord2fv.xhtml)
/// * `u` group: CoordF
pub type glEvalCoord2fv_t = unsafe extern "system" fn(u: *const [GLfloat; 2]);

/// [glEvalCoord2xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord2xOES.xhtml)
pub type glEvalCoord2xOES_t = unsafe extern "system" fn(u: GLfixed, v: GLfixed);

/// [glEvalCoord2xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalCoord2xvOES.xhtml)
pub type glEvalCoord2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

/// [glEvalMapsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalMapsNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `mode` group: EvalMapsModeNV
pub type glEvalMapsNV_t = unsafe extern "system" fn(target: EvalTargetNV, mode: EvalMapsModeNV);

/// [glEvalMesh1](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalMesh1.xhtml)
/// * `mode` group: MeshMode1
/// * `i1` group: CheckedInt32
/// * `i2` group: CheckedInt32
pub type glEvalMesh1_t = unsafe extern "system" fn(mode: MeshMode1, i1: GLint, i2: GLint);

/// [glEvalMesh2](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalMesh2.xhtml)
/// * `mode` group: MeshMode2
/// * `i1` group: CheckedInt32
/// * `i2` group: CheckedInt32
/// * `j1` group: CheckedInt32
/// * `j2` group: CheckedInt32
pub type glEvalMesh2_t = unsafe extern "system" fn(mode: MeshMode2, i1: GLint, i2: GLint, j1: GLint, j2: GLint);

/// [glEvalPoint1](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalPoint1.xhtml)
pub type glEvalPoint1_t = unsafe extern "system" fn(i: GLint);

/// [glEvalPoint2](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvalPoint2.xhtml)
/// * `i` group: CheckedInt32
/// * `j` group: CheckedInt32
pub type glEvalPoint2_t = unsafe extern "system" fn(i: GLint, j: GLint);

/// [glEvaluateDepthValuesARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEvaluateDepthValuesARB.xhtml)
pub type glEvaluateDepthValuesARB_t = unsafe extern "system" fn();

/// [glExecuteProgramNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExecuteProgramNV.xhtml)
/// * `target` group: VertexAttribEnumNV
pub type glExecuteProgramNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, id: GLuint, params: *const [GLfloat; 4]);

/// [glExtGetBufferPointervQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetBufferPointervQCOM.xhtml)
pub type glExtGetBufferPointervQCOM_t = unsafe extern "system" fn(target: GLenum, params: *mut *mut void);

/// [glExtGetBuffersQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetBuffersQCOM.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: maxBuffers
pub type glExtGetBuffersQCOM_t = unsafe extern "system" fn(buffers: *mut GLuint, maxBuffers: GLint, numBuffers: *mut GLint);

/// [glExtGetFramebuffersQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetFramebuffersQCOM.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: maxFramebuffers
pub type glExtGetFramebuffersQCOM_t = unsafe extern "system" fn(framebuffers: *mut GLuint, maxFramebuffers: GLint, numFramebuffers: *mut GLint);

/// [glExtGetProgramBinarySourceQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetProgramBinarySourceQCOM.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
pub type glExtGetProgramBinarySourceQCOM_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, source: *mut GLchar, length: *mut GLint);

/// [glExtGetProgramsQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetProgramsQCOM.xhtml)
/// * `programs` class: program
/// * `programs` len: maxPrograms
pub type glExtGetProgramsQCOM_t = unsafe extern "system" fn(programs: *mut GLuint, maxPrograms: GLint, numPrograms: *mut GLint);

/// [glExtGetRenderbuffersQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetRenderbuffersQCOM.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: maxRenderbuffers
pub type glExtGetRenderbuffersQCOM_t = unsafe extern "system" fn(renderbuffers: *mut GLuint, maxRenderbuffers: GLint, numRenderbuffers: *mut GLint);

/// [glExtGetShadersQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetShadersQCOM.xhtml)
/// * `shaders` class: shader
/// * `shaders` len: maxShaders
pub type glExtGetShadersQCOM_t = unsafe extern "system" fn(shaders: *mut GLuint, maxShaders: GLint, numShaders: *mut GLint);

/// [glExtGetTexLevelParameterivQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetTexLevelParameterivQCOM.xhtml)
/// * `texture` class: texture
pub type glExtGetTexLevelParameterivQCOM_t = unsafe extern "system" fn(texture: GLuint, face: GLenum, level: GLint, pname: GLenum, params: *mut GLint);

/// [glExtGetTexSubImageQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetTexSubImageQCOM.xhtml)
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glExtGetTexSubImageQCOM_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, texels: *mut void);

/// [glExtGetTexturesQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtGetTexturesQCOM.xhtml)
/// * `textures` class: texture
pub type glExtGetTexturesQCOM_t = unsafe extern "system" fn(textures: *mut GLuint, maxTextures: GLint, numTextures: *mut GLint);

/// [glExtIsProgramBinaryQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtIsProgramBinaryQCOM.xhtml)
/// * `program` class: program
pub type glExtIsProgramBinaryQCOM_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

/// [glExtTexObjectStateOverrideiQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtTexObjectStateOverrideiQCOM.xhtml)
pub type glExtTexObjectStateOverrideiQCOM_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

/// [glExtractComponentEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtractComponentEXT.xhtml)
pub type glExtractComponentEXT_t = unsafe extern "system" fn(res: GLuint, src: GLuint, num: GLuint);

/// [glExtrapolateTex2DQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glExtrapolateTex2DQCOM.xhtml)
/// * `src1` class: texture
/// * `src2` class: texture
/// * `output` class: texture
pub type glExtrapolateTex2DQCOM_t = unsafe extern "system" fn(src1: GLuint, src2: GLuint, output: GLuint, scaleFactor: GLfloat);

/// [glFeedbackBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFeedbackBuffer.xhtml)
/// * `type` group: FeedbackType
/// * `buffer` group: FeedbackElement
/// * `buffer` len: size
pub type glFeedbackBuffer_t = unsafe extern "system" fn(size: GLsizei, type_: FeedbackType, buffer: *mut GLfloat);

/// [glFeedbackBufferxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFeedbackBufferxOES.xhtml)
/// * `buffer` len: n
pub type glFeedbackBufferxOES_t = unsafe extern "system" fn(n: GLsizei, type_: GLenum, buffer: *const GLfixed);

/// [glFenceSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFenceSync.xhtml)
/// * `condition` group: SyncCondition
/// * `flags` group: SyncBehaviorFlags
pub type glFenceSync_t = unsafe extern "system" fn(condition: SyncCondition, flags: GLbitfield) -> GLsync;

/// [glFenceSyncAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFenceSyncAPPLE.xhtml)
/// * `condition` group: SyncCondition
/// * `flags` group: SyncBehaviorFlags
pub type glFenceSyncAPPLE_t = unsafe extern "system" fn(condition: SyncCondition, flags: GLbitfield) -> GLsync;

/// [glFinalCombinerInputNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinalCombinerInputNV.xhtml)
/// * `variable` group: CombinerVariableNV
/// * `input` group: CombinerRegisterNV
/// * `mapping` group: CombinerMappingNV
/// * `componentUsage` group: CombinerComponentUsageNV
pub type glFinalCombinerInputNV_t = unsafe extern "system" fn(variable: CombinerVariableNV, input: CombinerRegisterNV, mapping: CombinerMappingNV, componentUsage: CombinerComponentUsageNV);

/// [glFinish](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinish.xhtml)
pub type glFinish_t = unsafe extern "system" fn();

/// [glFinishAsyncSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinishAsyncSGIX.xhtml)
pub type glFinishAsyncSGIX_t = unsafe extern "system" fn(markerp: *mut GLuint) -> GLint;

/// [glFinishFenceAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinishFenceAPPLE.xhtml)
/// * `fence` group: FenceNV
pub type glFinishFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint);

/// [glFinishFenceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinishFenceNV.xhtml)
/// * `fence` group: FenceNV
pub type glFinishFenceNV_t = unsafe extern "system" fn(fence: GLuint);

/// [glFinishObjectAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinishObjectAPPLE.xhtml)
/// * `object` group: ObjectTypeAPPLE
pub type glFinishObjectAPPLE_t = unsafe extern "system" fn(object: ObjectTypeAPPLE, name: GLint);

/// [glFinishTextureSUNX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinishTextureSUNX.xhtml)
pub type glFinishTextureSUNX_t = unsafe extern "system" fn();

/// [glFlush](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlush.xhtml)
pub type glFlush_t = unsafe extern "system" fn();

/// [glFlushMappedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedBufferRange.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
pub type glFlushMappedBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr);

/// [glFlushMappedBufferRangeAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedBufferRangeAPPLE.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glFlushMappedBufferRangeAPPLE_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr);

/// [glFlushMappedBufferRangeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedBufferRangeEXT.xhtml)
/// * `target` group: BufferTargetARB
pub type glFlushMappedBufferRangeEXT_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr);

/// [glFlushMappedNamedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedNamedBufferRange.xhtml)
/// * `buffer` class: buffer
/// * `length` group: BufferSize
pub type glFlushMappedNamedBufferRange_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

/// [glFlushMappedNamedBufferRangeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedNamedBufferRangeEXT.xhtml)
/// * `buffer` class: buffer
pub type glFlushMappedNamedBufferRangeEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

/// [glFlushPixelDataRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushPixelDataRangeNV.xhtml)
/// * `target` group: PixelDataRangeTargetNV
pub type glFlushPixelDataRangeNV_t = unsafe extern "system" fn(target: PixelDataRangeTargetNV);

/// [glFlushRasterSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushRasterSGIX.xhtml)
pub type glFlushRasterSGIX_t = unsafe extern "system" fn();

/// [glFlushStaticDataIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushStaticDataIBM.xhtml)
pub type glFlushStaticDataIBM_t = unsafe extern "system" fn(target: GLenum);

/// [glFlushVertexArrayRangeAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushVertexArrayRangeAPPLE.xhtml)
/// * `pointer` len: length
pub type glFlushVertexArrayRangeAPPLE_t = unsafe extern "system" fn(length: GLsizei, pointer: *mut void);

/// [glFlushVertexArrayRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushVertexArrayRangeNV.xhtml)
pub type glFlushVertexArrayRangeNV_t = unsafe extern "system" fn();

/// [glFogCoordFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordFormatNV.xhtml)
pub type glFogCoordFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

/// [glFogCoordPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordPointer.xhtml)
/// * `type` group: FogPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glFogCoordPointer_t = unsafe extern "system" fn(type_: FogPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// [glFogCoordPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordPointerEXT.xhtml)
/// * `type` group: FogPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glFogCoordPointerEXT_t = unsafe extern "system" fn(type_: FogPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// [glFogCoordPointerListIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordPointerListIBM.xhtml)
/// * `type` group: FogPointerTypeIBM
/// * `pointer` len: COMPSIZE(type,stride)
pub type glFogCoordPointerListIBM_t = unsafe extern "system" fn(type_: FogPointerTypeIBM, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// [glFogCoordd](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordd.xhtml)
/// * `coord` group: CoordD
pub type glFogCoordd_t = unsafe extern "system" fn(coord: GLdouble);

/// [glFogCoorddEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoorddEXT.xhtml)
/// * `coord` group: CoordD
pub type glFogCoorddEXT_t = unsafe extern "system" fn(coord: GLdouble);

/// [glFogCoorddv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoorddv.xhtml)
/// * `coord` group: CoordD
pub type glFogCoorddv_t = unsafe extern "system" fn(coord: *const GLdouble);

/// [glFogCoorddvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoorddvEXT.xhtml)
/// * `coord` group: CoordD
pub type glFogCoorddvEXT_t = unsafe extern "system" fn(coord: *const GLdouble);

/// [glFogCoordf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordf.xhtml)
/// * `coord` group: CoordF
pub type glFogCoordf_t = unsafe extern "system" fn(coord: GLfloat);

/// [glFogCoordfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordfEXT.xhtml)
/// * `coord` group: CoordF
pub type glFogCoordfEXT_t = unsafe extern "system" fn(coord: GLfloat);

/// [glFogCoordfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordfv.xhtml)
/// * `coord` group: CoordF
pub type glFogCoordfv_t = unsafe extern "system" fn(coord: *const GLfloat);

/// [glFogCoordfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordfvEXT.xhtml)
/// * `coord` group: CoordF
pub type glFogCoordfvEXT_t = unsafe extern "system" fn(coord: *const GLfloat);

/// [glFogCoordhNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordhNV.xhtml)
/// * `fog` group: Half16NV
pub type glFogCoordhNV_t = unsafe extern "system" fn(fog: GLhalfNV);

/// [glFogCoordhvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogCoordhvNV.xhtml)
/// * `fog` group: Half16NV
pub type glFogCoordhvNV_t = unsafe extern "system" fn(fog: *const GLhalfNV);

/// [glFogFuncSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogFuncSGIS.xhtml)
/// * `points` len: n*2
pub type glFogFuncSGIS_t = unsafe extern "system" fn(n: GLsizei, points: *const GLfloat);

/// [glFogf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogf.xhtml)
/// * `pname` group: FogParameter
/// * `param` group: CheckedFloat32
pub type glFogf_t = unsafe extern "system" fn(pname: FogParameter, param: GLfloat);

/// [glFogfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogfv.xhtml)
/// * `pname` group: FogParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glFogfv_t = unsafe extern "system" fn(pname: FogParameter, params: *const GLfloat);

/// [glFogi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogi.xhtml)
/// * `pname` group: FogParameter
/// * `param` group: CheckedInt32
pub type glFogi_t = unsafe extern "system" fn(pname: FogParameter, param: GLint);

/// [glFogiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogiv.xhtml)
/// * `pname` group: FogParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glFogiv_t = unsafe extern "system" fn(pname: FogParameter, params: *const GLint);

/// [glFogx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogx.xhtml)
/// * `pname` group: FogPName
pub type glFogx_t = unsafe extern "system" fn(pname: FogPName, param: GLfixed);

/// [glFogxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogxOES.xhtml)
/// * `pname` group: FogPName
pub type glFogxOES_t = unsafe extern "system" fn(pname: FogPName, param: GLfixed);

/// [glFogxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogxv.xhtml)
/// * `pname` group: FogPName
/// * `param` len: COMPSIZE(pname)
pub type glFogxv_t = unsafe extern "system" fn(pname: FogPName, param: *const GLfixed);

/// [glFogxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFogxvOES.xhtml)
/// * `pname` group: FogPName
/// * `param` len: COMPSIZE(pname)
pub type glFogxvOES_t = unsafe extern "system" fn(pname: FogPName, param: *const GLfixed);

/// [glFragmentColorMaterialSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentColorMaterialSGIX.xhtml)
/// * `face` group: MaterialFace
/// * `mode` group: MaterialParameter
pub type glFragmentColorMaterialSGIX_t = unsafe extern "system" fn(face: MaterialFace, mode: MaterialParameter);

/// [glFragmentCoverageColorNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentCoverageColorNV.xhtml)
pub type glFragmentCoverageColorNV_t = unsafe extern "system" fn(color: GLuint);

/// [glFragmentLightModelfSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentLightModelfSGIX.xhtml)
/// * `pname` group: FragmentLightModelParameterSGIX
/// * `param` group: CheckedFloat32
pub type glFragmentLightModelfSGIX_t = unsafe extern "system" fn(pname: FragmentLightModelParameterSGIX, param: GLfloat);

/// [glFragmentLightModelfvSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentLightModelfvSGIX.xhtml)
/// * `pname` group: FragmentLightModelParameterSGIX
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentLightModelfvSGIX_t = unsafe extern "system" fn(pname: FragmentLightModelParameterSGIX, params: *const GLfloat);

/// [glFragmentLightModeliSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentLightModeliSGIX.xhtml)
/// * `pname` group: FragmentLightModelParameterSGIX
/// * `param` group: CheckedInt32
pub type glFragmentLightModeliSGIX_t = unsafe extern "system" fn(pname: FragmentLightModelParameterSGIX, param: GLint);

/// [glFragmentLightModelivSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentLightModelivSGIX.xhtml)
/// * `pname` group: FragmentLightModelParameterSGIX
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentLightModelivSGIX_t = unsafe extern "system" fn(pname: FragmentLightModelParameterSGIX, params: *const GLint);

/// [glFragmentLightfSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentLightfSGIX.xhtml)
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `param` group: CheckedFloat32
pub type glFragmentLightfSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, param: GLfloat);

/// [glFragmentLightfvSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentLightfvSGIX.xhtml)
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentLightfvSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, params: *const GLfloat);

/// [glFragmentLightiSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentLightiSGIX.xhtml)
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `param` group: CheckedInt32
pub type glFragmentLightiSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, param: GLint);

/// [glFragmentLightivSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentLightivSGIX.xhtml)
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentLightivSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, params: *const GLint);

/// [glFragmentMaterialfSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentMaterialfSGIX.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` group: CheckedFloat32
pub type glFragmentMaterialfSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfloat);

/// [glFragmentMaterialfvSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentMaterialfvSGIX.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentMaterialfvSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *const GLfloat);

/// [glFragmentMaterialiSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentMaterialiSGIX.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` group: CheckedInt32
pub type glFragmentMaterialiSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLint);

/// [glFragmentMaterialivSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFragmentMaterialivSGIX.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentMaterialivSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *const GLint);

/// [glFrameTerminatorGREMEDY](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrameTerminatorGREMEDY.xhtml)
pub type glFrameTerminatorGREMEDY_t = unsafe extern "system" fn();

/// [glFrameZoomSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrameZoomSGIX.xhtml)
/// * `factor` group: CheckedInt32
pub type glFrameZoomSGIX_t = unsafe extern "system" fn(factor: GLint);

/// [glFramebufferDrawBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferDrawBufferEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `mode` group: DrawBufferMode
pub type glFramebufferDrawBufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, mode: DrawBufferMode);

/// [glFramebufferDrawBuffersEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferDrawBuffersEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
pub type glFramebufferDrawBuffersEXT_t = unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const DrawBufferMode);

/// [glFramebufferFetchBarrierEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferFetchBarrierEXT.xhtml)
pub type glFramebufferFetchBarrierEXT_t = unsafe extern "system" fn();

/// [glFramebufferFetchBarrierQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferFetchBarrierQCOM.xhtml)
pub type glFramebufferFetchBarrierQCOM_t = unsafe extern "system" fn();

/// [glFramebufferFoveationConfigQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferFoveationConfigQCOM.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
pub type glFramebufferFoveationConfigQCOM_t = unsafe extern "system" fn(framebuffer: GLuint, numLayers: GLuint, focalPointsPerLayer: GLuint, requestedFeatures: GLuint, providedFeatures: *mut GLuint);

/// [glFramebufferFoveationParametersQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferFoveationParametersQCOM.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `focalX` group: CheckedFloat32
/// * `focalY` group: CheckedFloat32
/// * `gainX` group: CheckedFloat32
/// * `gainY` group: CheckedFloat32
/// * `foveaArea` group: CheckedFloat32
pub type glFramebufferFoveationParametersQCOM_t = unsafe extern "system" fn(framebuffer: GLuint, layer: GLuint, focalPoint: GLuint, focalX: GLfloat, focalY: GLfloat, gainX: GLfloat, gainY: GLfloat, foveaArea: GLfloat);

/// [glFramebufferParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferParameteri.xhtml)
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferParameterName
pub type glFramebufferParameteri_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferParameterName, param: GLint);

/// [glFramebufferParameteriMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferParameteriMESA.xhtml)
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferParameterName
pub type glFramebufferParameteriMESA_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferParameterName, param: GLint);

/// [glFramebufferPixelLocalStorageSizeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferPixelLocalStorageSizeEXT.xhtml)
pub type glFramebufferPixelLocalStorageSizeEXT_t = unsafe extern "system" fn(target: GLuint, size: GLsizei);

/// [glFramebufferReadBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferReadBufferEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `mode` group: ReadBufferMode
pub type glFramebufferReadBufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, mode: ReadBufferMode);

/// [glFramebufferRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferRenderbuffer.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glFramebufferRenderbuffer_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// [glFramebufferRenderbufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferRenderbufferEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glFramebufferRenderbufferEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// [glFramebufferRenderbufferOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferRenderbufferOES.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glFramebufferRenderbufferOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// [glFramebufferSampleLocationsfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferSampleLocationsfvARB.xhtml)
/// * `target` group: FramebufferTarget
pub type glFramebufferSampleLocationsfvARB_t = unsafe extern "system" fn(target: FramebufferTarget, start: GLuint, count: GLsizei, v: *const GLfloat);

/// [glFramebufferSampleLocationsfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferSampleLocationsfvNV.xhtml)
/// * `target` group: FramebufferTarget
pub type glFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(target: FramebufferTarget, start: GLuint, count: GLsizei, v: *const GLfloat);

/// [glFramebufferSamplePositionsfvAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferSamplePositionsfvAMD.xhtml)
/// * `target` group: FramebufferTarget
pub type glFramebufferSamplePositionsfvAMD_t = unsafe extern "system" fn(target: FramebufferTarget, numsamples: GLuint, pixelindex: GLuint, values: *const GLfloat);

/// [glFramebufferTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
pub type glFramebufferTexture_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// [glFramebufferTexture1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture1D.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture1D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// [glFramebufferTexture1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture1DEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture1DEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// [glFramebufferTexture2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture2D.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// [glFramebufferTexture2DDownsampleIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture2DDownsampleIMG.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DDownsampleIMG_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, xscale: GLint, yscale: GLint);

/// [glFramebufferTexture2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture2DEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// [glFramebufferTexture2DMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture2DMultisampleEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DMultisampleEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, samples: GLsizei);

/// [glFramebufferTexture2DMultisampleIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture2DMultisampleIMG.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DMultisampleIMG_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, samples: GLsizei);

/// [glFramebufferTexture2DOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture2DOES.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// [glFramebufferTexture3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture3D.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture3D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

/// [glFramebufferTexture3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture3DEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture3DEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

/// [glFramebufferTexture3DOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture3DOES.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture3DOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

/// [glFramebufferTextureARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureARB.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureARB_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// [glFramebufferTextureEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// [glFramebufferTextureFaceARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureFaceARB.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `face` group: TextureTarget
pub type glFramebufferTextureFaceARB_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, face: TextureTarget);

/// [glFramebufferTextureFaceEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureFaceEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `face` group: TextureTarget
pub type glFramebufferTextureFaceEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, face: TextureTarget);

/// [glFramebufferTextureLayer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureLayer.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glFramebufferTextureLayer_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// [glFramebufferTextureLayerARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureLayerARB.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glFramebufferTextureLayerARB_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// [glFramebufferTextureLayerDownsampleIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureLayerDownsampleIMG.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glFramebufferTextureLayerDownsampleIMG_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint, xscale: GLint, yscale: GLint);

/// [glFramebufferTextureLayerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureLayerEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glFramebufferTextureLayerEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// [glFramebufferTextureMultisampleMultiviewOVR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureMultisampleMultiviewOVR.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureMultisampleMultiviewOVR_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, samples: GLsizei, baseViewIndex: GLint, numViews: GLsizei);

/// [glFramebufferTextureMultiviewOVR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureMultiviewOVR.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureMultiviewOVR_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, baseViewIndex: GLint, numViews: GLsizei);

/// [glFramebufferTextureOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureOES.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// [glFreeObjectBufferATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFreeObjectBufferATI.xhtml)
/// * `buffer` class: buffer
pub type glFreeObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint);

/// [glFrontFace](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrontFace.xhtml)
/// * `mode` group: FrontFaceDirection
pub type glFrontFace_t = unsafe extern "system" fn(mode: FrontFaceDirection);

/// [glFrustum](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrustum.xhtml)
pub type glFrustum_t = unsafe extern "system" fn(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

/// [glFrustumf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrustumf.xhtml)
pub type glFrustumf_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

/// [glFrustumfOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrustumfOES.xhtml)
pub type glFrustumfOES_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

/// [glFrustumx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrustumx.xhtml)
pub type glFrustumx_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

/// [glFrustumxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrustumxOES.xhtml)
pub type glFrustumxOES_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

/// [glGenAsyncMarkersSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenAsyncMarkersSGIX.xhtml)
pub type glGenAsyncMarkersSGIX_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

/// [glGenBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glGenBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

/// [glGenBuffersARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffersARB.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glGenBuffersARB_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

/// [glGenFencesAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenFencesAPPLE.xhtml)
/// * `fences` group: FenceNV
/// * `fences` len: n
pub type glGenFencesAPPLE_t = unsafe extern "system" fn(n: GLsizei, fences: *mut GLuint);

/// [glGenFencesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenFencesNV.xhtml)
/// * `fences` group: FenceNV
/// * `fences` len: n
pub type glGenFencesNV_t = unsafe extern "system" fn(n: GLsizei, fences: *mut GLuint);

/// [glGenFragmentShadersATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenFragmentShadersATI.xhtml)
pub type glGenFragmentShadersATI_t = unsafe extern "system" fn(range: GLuint) -> GLuint;

/// [glGenFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenFramebuffers.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glGenFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

/// [glGenFramebuffersEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenFramebuffersEXT.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glGenFramebuffersEXT_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

/// [glGenFramebuffersOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenFramebuffersOES.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glGenFramebuffersOES_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

/// [glGenLists](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenLists.xhtml)
pub type glGenLists_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

/// [glGenNamesAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenNamesAMD.xhtml)
/// * `names` len: num
pub type glGenNamesAMD_t = unsafe extern "system" fn(identifier: GLenum, num: GLuint, names: *mut GLuint);

/// [glGenOcclusionQueriesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenOcclusionQueriesNV.xhtml)
/// * `ids` len: n
pub type glGenOcclusionQueriesNV_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// [glGenPathsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenPathsNV.xhtml)
pub type glGenPathsNV_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

/// [glGenPerfMonitorsAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenPerfMonitorsAMD.xhtml)
/// * `monitors` len: n
pub type glGenPerfMonitorsAMD_t = unsafe extern "system" fn(n: GLsizei, monitors: *mut GLuint);

/// [glGenProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenProgramPipelines.xhtml)
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glGenProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

/// [glGenProgramPipelinesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenProgramPipelinesEXT.xhtml)
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glGenProgramPipelinesEXT_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

/// [glGenProgramsARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenProgramsARB.xhtml)
/// * `programs` class: program
/// * `programs` len: n
pub type glGenProgramsARB_t = unsafe extern "system" fn(n: GLsizei, programs: *mut GLuint);

/// [glGenProgramsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenProgramsNV.xhtml)
/// * `programs` class: program
/// * `programs` len: n
pub type glGenProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *mut GLuint);

/// [glGenQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenQueries.xhtml)
/// * `ids` class: query
/// * `ids` len: n
pub type glGenQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// [glGenQueriesARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenQueriesARB.xhtml)
/// * `ids` class: query
/// * `ids` len: n
pub type glGenQueriesARB_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// [glGenQueriesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenQueriesEXT.xhtml)
/// * `ids` class: query
/// * `ids` len: n
pub type glGenQueriesEXT_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// [glGenQueryResourceTagNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenQueryResourceTagNV.xhtml)
/// * `tagIds` len: n
pub type glGenQueryResourceTagNV_t = unsafe extern "system" fn(n: GLsizei, tagIds: *mut GLint);

/// [glGenRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenRenderbuffers.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glGenRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

/// [glGenRenderbuffersEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenRenderbuffersEXT.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glGenRenderbuffersEXT_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

/// [glGenRenderbuffersOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenRenderbuffersOES.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glGenRenderbuffersOES_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

/// [glGenSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenSamplers.xhtml)
/// * `samplers` class: sampler
/// * `samplers` len: count
pub type glGenSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);

/// [glGenSemaphoresEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenSemaphoresEXT.xhtml)
/// * `semaphores` len: n
pub type glGenSemaphoresEXT_t = unsafe extern "system" fn(n: GLsizei, semaphores: *mut GLuint);

/// [glGenSymbolsEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenSymbolsEXT.xhtml)
/// * `datatype` group: DataTypeEXT
/// * `storagetype` group: VertexShaderStorageTypeEXT
/// * `range` group: ParameterRangeEXT
pub type glGenSymbolsEXT_t = unsafe extern "system" fn(datatype: DataTypeEXT, storagetype: VertexShaderStorageTypeEXT, range: ParameterRangeEXT, components: GLuint) -> GLuint;

/// [glGenTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTextures.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
pub type glGenTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);

/// [glGenTexturesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTexturesEXT.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
pub type glGenTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);

/// [glGenTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTransformFeedbacks.xhtml)
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glGenTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// [glGenTransformFeedbacksNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTransformFeedbacksNV.xhtml)
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glGenTransformFeedbacksNV_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// [glGenVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glGenVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

/// [glGenVertexArraysAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArraysAPPLE.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glGenVertexArraysAPPLE_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

/// [glGenVertexArraysOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArraysOES.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glGenVertexArraysOES_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

/// [glGenVertexShadersEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexShadersEXT.xhtml)
pub type glGenVertexShadersEXT_t = unsafe extern "system" fn(range: GLuint) -> GLuint;

/// [glGenerateMipmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateMipmap.xhtml)
/// * `target` group: TextureTarget
pub type glGenerateMipmap_t = unsafe extern "system" fn(target: TextureTarget);

/// [glGenerateMipmapEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateMipmapEXT.xhtml)
/// * `target` group: TextureTarget
pub type glGenerateMipmapEXT_t = unsafe extern "system" fn(target: TextureTarget);

/// [glGenerateMipmapOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateMipmapOES.xhtml)
/// * `target` group: TextureTarget
pub type glGenerateMipmapOES_t = unsafe extern "system" fn(target: TextureTarget);

/// [glGenerateMultiTexMipmapEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateMultiTexMipmapEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
pub type glGenerateMultiTexMipmapEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget);

/// [glGenerateTextureMipmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateTextureMipmap.xhtml)
/// * `texture` class: texture
pub type glGenerateTextureMipmap_t = unsafe extern "system" fn(texture: GLuint);

/// [glGenerateTextureMipmapEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateTextureMipmapEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
pub type glGenerateTextureMipmapEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget);

/// [glGetActiveAtomicCounterBufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveAtomicCounterBufferiv.xhtml)
/// * `program` class: program
/// * `pname` group: AtomicCounterBufferPName
/// * `params` len: COMPSIZE(pname)
pub type glGetActiveAtomicCounterBufferiv_t = unsafe extern "system" fn(program: GLuint, bufferIndex: GLuint, pname: AtomicCounterBufferPName, params: *mut GLint);

/// [glGetActiveAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveAttrib.xhtml)
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
pub type glGetActiveAttrib_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar);

/// [glGetActiveAttribARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveAttribARB.xhtml)
/// * `programObj` group: handleARB
/// * `type` group: AttributeType
/// * `name` len: maxLength
pub type glGetActiveAttribARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLcharARB);

/// [glGetActiveSubroutineName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineName.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `name` len: bufSize
pub type glGetActiveSubroutineName_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

/// [glGetActiveSubroutineUniformName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineUniformName.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `name` len: bufSize
pub type glGetActiveSubroutineUniformName_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

/// [glGetActiveSubroutineUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineUniformiv.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `pname` group: SubroutineParameterName
/// * `values` len: COMPSIZE(pname)
pub type glGetActiveSubroutineUniformiv_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, pname: SubroutineParameterName, values: *mut GLint);

/// [glGetActiveUniform](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniform.xhtml)
/// * `program` class: program
/// * `type` group: UniformType
/// * `name` len: bufSize
pub type glGetActiveUniform_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar);

/// [glGetActiveUniformARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformARB.xhtml)
/// * `programObj` group: handleARB
/// * `type` group: UniformType
/// * `name` len: maxLength
pub type glGetActiveUniformARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLcharARB);

/// [glGetActiveUniformBlockName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformBlockName.xhtml)
/// * `program` class: program
/// * `uniformBlockName` len: bufSize
pub type glGetActiveUniformBlockName_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);

/// [glGetActiveUniformBlockiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformBlockiv.xhtml)
/// * `program` class: program
/// * `pname` group: UniformBlockPName
/// * `params` len: COMPSIZE(program,uniformBlockIndex,pname)
pub type glGetActiveUniformBlockiv_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint);

/// [glGetActiveUniformName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformName.xhtml)
/// * `program` class: program
/// * `uniformName` len: bufSize
pub type glGetActiveUniformName_t = unsafe extern "system" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);

/// [glGetActiveUniformsiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformsiv.xhtml)
/// * `program` class: program
/// * `uniformIndices` len: uniformCount
/// * `pname` group: UniformPName
/// * `params` len: COMPSIZE(uniformCount,pname)
pub type glGetActiveUniformsiv_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint);

/// [glGetActiveVaryingNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveVaryingNV.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(program,index,bufSize)
pub type glGetActiveVaryingNV_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);

/// [glGetArrayObjectfvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetArrayObjectfvATI.xhtml)
/// * `array` group: EnableCap
/// * `pname` group: ArrayObjectPNameATI
pub type glGetArrayObjectfvATI_t = unsafe extern "system" fn(array: EnableCap, pname: ArrayObjectPNameATI, params: *mut GLfloat);

/// [glGetArrayObjectivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetArrayObjectivATI.xhtml)
/// * `array` group: EnableCap
/// * `pname` group: ArrayObjectPNameATI
pub type glGetArrayObjectivATI_t = unsafe extern "system" fn(array: EnableCap, pname: ArrayObjectPNameATI, params: *mut GLint);

/// [glGetAttachedObjectsARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttachedObjectsARB.xhtml)
/// * `containerObj` group: handleARB
/// * `obj` group: handleARB
/// * `obj` len: maxCount
pub type glGetAttachedObjectsARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, maxCount: GLsizei, count: *mut GLsizei, obj: *mut GLhandleARB);

/// [glGetAttachedShaders](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttachedShaders.xhtml)
/// * `program` class: program
/// * `shaders` class: shader
/// * `shaders` len: maxCount
pub type glGetAttachedShaders_t = unsafe extern "system" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);

/// [glGetAttribLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttribLocation.xhtml)
/// * `program` class: program
pub type glGetAttribLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// [glGetAttribLocationARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttribLocationARB.xhtml)
/// * `programObj` group: handleARB
pub type glGetAttribLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, name: *const GLcharARB) -> GLint;

/// [glGetBooleanIndexedvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBooleanIndexedvEXT.xhtml)
/// * `target` group: BufferTargetARB
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(target)
pub type glGetBooleanIndexedvEXT_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, data: *mut GLboolean);

/// [glGetBooleani_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBooleani_v.xhtml)
/// * `target` group: BufferTargetARB
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(target)
pub type glGetBooleani_v_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, data: *mut GLboolean);

/// [glGetBooleanv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBooleanv.xhtml)
/// * `pname` group: GetPName
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(pname)
pub type glGetBooleanv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLboolean);

/// [glGetBufferParameteri64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferParameteri64v.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetBufferParameteri64v_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64);

/// [glGetBufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferParameteriv.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetBufferParameteriv_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint);

/// [glGetBufferParameterivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferParameterivARB.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetBufferParameterivARB_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint);

/// [glGetBufferParameterui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferParameterui64vNV.xhtml)
/// * `target` group: BufferTargetARB
/// * `params` len: COMPSIZE(pname)
pub type glGetBufferParameterui64vNV_t = unsafe extern "system" fn(target: BufferTargetARB, pname: GLenum, params: *mut GLuint64EXT);

/// [glGetBufferPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferPointerv.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPointerNameARB
pub type glGetBufferPointerv_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);

/// [glGetBufferPointervARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferPointervARB.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPointerNameARB
pub type glGetBufferPointervARB_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);

/// [glGetBufferPointervOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferPointervOES.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPointerNameARB
pub type glGetBufferPointervOES_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);

/// [glGetBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferSubData.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `data` len: size
pub type glGetBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void);

/// [glGetBufferSubDataARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferSubDataARB.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffsetARB
/// * `size` group: BufferSizeARB
/// * `data` len: size
pub type glGetBufferSubDataARB_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptrARB, size: GLsizeiptrARB, data: *mut void);

/// [glGetClipPlane](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetClipPlane.xhtml)
/// * `plane` group: ClipPlaneName
pub type glGetClipPlane_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLdouble; 4]);

/// [glGetClipPlanef](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetClipPlanef.xhtml)
/// * `plane` group: ClipPlaneName
pub type glGetClipPlanef_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLfloat; 4]);

/// [glGetClipPlanefOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetClipPlanefOES.xhtml)
/// * `plane` group: ClipPlaneName
pub type glGetClipPlanefOES_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLfloat; 4]);

/// [glGetClipPlanex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetClipPlanex.xhtml)
/// * `plane` group: ClipPlaneName
pub type glGetClipPlanex_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLfixed; 4]);

/// [glGetClipPlanexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetClipPlanexOES.xhtml)
/// * `plane` group: ClipPlaneName
pub type glGetClipPlanexOES_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLfixed; 4]);

/// [glGetColorTable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTable.xhtml)
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(target,format,type)
pub type glGetColorTable_t = unsafe extern "system" fn(target: ColorTableTarget, format: PixelFormat, type_: PixelType, table: *mut void);

/// [glGetColorTableEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTableEXT.xhtml)
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(target,format,type)
pub type glGetColorTableEXT_t = unsafe extern "system" fn(target: ColorTableTarget, format: PixelFormat, type_: PixelType, data: *mut void);

/// [glGetColorTableParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTableParameterfv.xhtml)
/// * `target` group: ColorTableTarget
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterfv_t = unsafe extern "system" fn(target: ColorTableTarget, pname: GetColorTableParameterPNameSGI, params: *mut GLfloat);

/// [glGetColorTableParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTableParameterfvEXT.xhtml)
/// * `target` group: ColorTableTarget
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterfvEXT_t = unsafe extern "system" fn(target: ColorTableTarget, pname: GetColorTableParameterPNameSGI, params: *mut GLfloat);

/// [glGetColorTableParameterfvSGI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTableParameterfvSGI.xhtml)
/// * `target` group: ColorTableTargetSGI
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterfvSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, pname: GetColorTableParameterPNameSGI, params: *mut GLfloat);

/// [glGetColorTableParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTableParameteriv.xhtml)
/// * `target` group: ColorTableTarget
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameteriv_t = unsafe extern "system" fn(target: ColorTableTarget, pname: GetColorTableParameterPNameSGI, params: *mut GLint);

/// [glGetColorTableParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTableParameterivEXT.xhtml)
/// * `target` group: ColorTableTarget
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterivEXT_t = unsafe extern "system" fn(target: ColorTableTarget, pname: GetColorTableParameterPNameSGI, params: *mut GLint);

/// [glGetColorTableParameterivSGI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTableParameterivSGI.xhtml)
/// * `target` group: ColorTableTargetSGI
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterivSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, pname: GetColorTableParameterPNameSGI, params: *mut GLint);

/// [glGetColorTableSGI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetColorTableSGI.xhtml)
/// * `target` group: ColorTableTargetSGI
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(target,format,type)
pub type glGetColorTableSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, format: PixelFormat, type_: PixelType, table: *mut void);

/// [glGetCombinerInputParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCombinerInputParameterfvNV.xhtml)
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `variable` group: CombinerVariableNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerInputParameterfvNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, variable: CombinerVariableNV, pname: CombinerParameterNV, params: *mut GLfloat);

/// [glGetCombinerInputParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCombinerInputParameterivNV.xhtml)
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `variable` group: CombinerVariableNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerInputParameterivNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, variable: CombinerVariableNV, pname: CombinerParameterNV, params: *mut GLint);

/// [glGetCombinerOutputParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCombinerOutputParameterfvNV.xhtml)
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerOutputParameterfvNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, pname: CombinerParameterNV, params: *mut GLfloat);

/// [glGetCombinerOutputParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCombinerOutputParameterivNV.xhtml)
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerOutputParameterivNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, pname: CombinerParameterNV, params: *mut GLint);

/// [glGetCombinerStageParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCombinerStageParameterfvNV.xhtml)
/// * `stage` group: CombinerStageNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerStageParameterfvNV_t = unsafe extern "system" fn(stage: CombinerStageNV, pname: CombinerParameterNV, params: *mut GLfloat);

/// [glGetCommandHeaderNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCommandHeaderNV.xhtml)
pub type glGetCommandHeaderNV_t = unsafe extern "system" fn(tokenID: GLenum, size: GLuint) -> GLuint;

/// [glGetCompressedMultiTexImageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedMultiTexImageEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `lod` group: CheckedInt32
/// * `img` len: COMPSIZE(target,lod)
pub type glGetCompressedMultiTexImageEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, lod: GLint, img: *mut void);

/// [glGetCompressedTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTexImage.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `img` group: CompressedTextureARB
/// * `img` len: COMPSIZE(target,level)
pub type glGetCompressedTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, img: *mut void);

/// [glGetCompressedTexImageARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTexImageARB.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `img` group: CompressedTextureARB
/// * `img` len: COMPSIZE(target,level)
pub type glGetCompressedTexImageARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, img: *mut void);

/// [glGetCompressedTextureImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTextureImage.xhtml)
/// * `texture` class: texture
pub type glGetCompressedTextureImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut void);

/// [glGetCompressedTextureImageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTextureImageEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `lod` group: CheckedInt32
/// * `img` len: COMPSIZE(target,lod)
pub type glGetCompressedTextureImageEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, lod: GLint, img: *mut void);

/// [glGetCompressedTextureSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTextureSubImage.xhtml)
/// * `texture` class: texture
pub type glGetCompressedTextureSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut void);

/// [glGetConvolutionFilter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetConvolutionFilter.xhtml)
/// * `target` group: ConvolutionTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(target,format,type)
pub type glGetConvolutionFilter_t = unsafe extern "system" fn(target: ConvolutionTarget, format: PixelFormat, type_: PixelType, image: *mut void);

/// [glGetConvolutionFilterEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetConvolutionFilterEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(target,format,type)
pub type glGetConvolutionFilterEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, format: PixelFormat, type_: PixelType, image: *mut void);

/// [glGetConvolutionParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetConvolutionParameterfv.xhtml)
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameterfv_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: *mut GLfloat);

/// [glGetConvolutionParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetConvolutionParameterfvEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameterfvEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *mut GLfloat);

/// [glGetConvolutionParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetConvolutionParameteriv.xhtml)
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameteriv_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: *mut GLint);

/// [glGetConvolutionParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetConvolutionParameterivEXT.xhtml)
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameterivEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *mut GLint);

/// [glGetConvolutionParameterxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetConvolutionParameterxvOES.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameterxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);

/// [glGetCoverageModulationTableNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCoverageModulationTableNV.xhtml)
pub type glGetCoverageModulationTableNV_t = unsafe extern "system" fn(bufSize: GLsizei, v: *mut GLfloat);

/// [glGetDebugMessageLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDebugMessageLog.xhtml)
/// * `sources` group: DebugSource
/// * `sources` len: count
/// * `types` group: DebugType
/// * `types` len: count
/// * `ids` len: count
/// * `severities` group: DebugSeverity
/// * `severities` len: count
/// * `lengths` len: count
/// * `messageLog` len: bufSize
pub type glGetDebugMessageLog_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;

/// [glGetDebugMessageLogAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDebugMessageLogAMD.xhtml)
/// * `categories` len: count
/// * `severities` group: DebugSeverity
/// * `severities` len: count
/// * `ids` len: count
/// * `lengths` len: count
/// * `message` len: bufSize
pub type glGetDebugMessageLogAMD_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, categories: *mut GLenum, severities: *mut GLuint, ids: *mut GLuint, lengths: *mut GLsizei, message: *mut GLchar) -> GLuint;

/// [glGetDebugMessageLogARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDebugMessageLogARB.xhtml)
/// * `sources` group: DebugSource
/// * `sources` len: count
/// * `types` group: DebugType
/// * `types` len: count
/// * `ids` len: count
/// * `severities` group: DebugSeverity
/// * `severities` len: count
/// * `lengths` len: count
/// * `messageLog` len: bufSize
pub type glGetDebugMessageLogARB_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;

/// [glGetDebugMessageLogKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDebugMessageLogKHR.xhtml)
/// * `sources` group: DebugSource
/// * `sources` len: count
/// * `types` group: DebugType
/// * `types` len: count
/// * `ids` len: count
/// * `severities` group: DebugSeverity
/// * `severities` len: count
/// * `lengths` len: count
/// * `messageLog` len: bufSize
pub type glGetDebugMessageLogKHR_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;

/// [glGetDetailTexFuncSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDetailTexFuncSGIS.xhtml)
/// * `target` group: TextureTarget
/// * `points` len: COMPSIZE(target)
pub type glGetDetailTexFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, points: *mut GLfloat);

/// [glGetDoubleIndexedvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDoubleIndexedvEXT.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetDoubleIndexedvEXT_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLdouble);

/// [glGetDoublei_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDoublei_v.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetDoublei_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLdouble);

/// [glGetDoublei_vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDoublei_vEXT.xhtml)
/// * `pname` group: GetPName
/// * `params` len: COMPSIZE(pname)
pub type glGetDoublei_vEXT_t = unsafe extern "system" fn(pname: GetPName, index: GLuint, params: *mut GLdouble);

/// [glGetDoublev](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDoublev.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetDoublev_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLdouble);

/// [glGetDriverControlStringQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDriverControlStringQCOM.xhtml)
/// * `driverControlString` len: bufSize
pub type glGetDriverControlStringQCOM_t = unsafe extern "system" fn(driverControl: GLuint, bufSize: GLsizei, length: *mut GLsizei, driverControlString: *mut GLchar);

/// [glGetDriverControlsQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDriverControlsQCOM.xhtml)
/// * `driverControls` len: size
pub type glGetDriverControlsQCOM_t = unsafe extern "system" fn(num: *mut GLint, size: GLsizei, driverControls: *mut GLuint);

/// [glGetError](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml)
pub type glGetError_t = unsafe extern "system" fn() -> ErrorCode;

/// [glGetFenceivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFenceivNV.xhtml)
/// * `fence` group: FenceNV
/// * `pname` group: FenceParameterNameNV
/// * `params` len: COMPSIZE(pname)
pub type glGetFenceivNV_t = unsafe extern "system" fn(fence: GLuint, pname: FenceParameterNameNV, params: *mut GLint);

/// [glGetFinalCombinerInputParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFinalCombinerInputParameterfvNV.xhtml)
/// * `variable` group: CombinerVariableNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetFinalCombinerInputParameterfvNV_t = unsafe extern "system" fn(variable: CombinerVariableNV, pname: CombinerParameterNV, params: *mut GLfloat);

/// [glGetFinalCombinerInputParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFinalCombinerInputParameterivNV.xhtml)
/// * `variable` group: CombinerVariableNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetFinalCombinerInputParameterivNV_t = unsafe extern "system" fn(variable: CombinerVariableNV, pname: CombinerParameterNV, params: *mut GLint);

/// [glGetFirstPerfQueryIdINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFirstPerfQueryIdINTEL.xhtml)
pub type glGetFirstPerfQueryIdINTEL_t = unsafe extern "system" fn(queryId: *mut GLuint);

/// [glGetFixedv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFixedv.xhtml)
/// * `pname` group: GetPName
pub type glGetFixedv_t = unsafe extern "system" fn(pname: GetPName, params: *mut GLfixed);

/// [glGetFixedvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFixedvOES.xhtml)
/// * `pname` group: GetPName
/// * `params` len: COMPSIZE(pname)
pub type glGetFixedvOES_t = unsafe extern "system" fn(pname: GetPName, params: *mut GLfixed);

/// [glGetFloatIndexedvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloatIndexedvEXT.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetFloatIndexedvEXT_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

/// [glGetFloati_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloati_v.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetFloati_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

/// [glGetFloati_vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloati_vEXT.xhtml)
/// * `pname` group: GetPName
/// * `params` len: COMPSIZE(pname)
pub type glGetFloati_vEXT_t = unsafe extern "system" fn(pname: GetPName, index: GLuint, params: *mut GLfloat);

/// [glGetFloati_vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloati_vNV.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetFloati_vNV_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

/// [glGetFloati_vOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloati_vOES.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetFloati_vOES_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

/// [glGetFloatv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloatv.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetFloatv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLfloat);

/// [glGetFogFuncSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFogFuncSGIS.xhtml)
/// * `points` len: COMPSIZE()
pub type glGetFogFuncSGIS_t = unsafe extern "system" fn(points: *mut GLfloat);

/// [glGetFragDataIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragDataIndex.xhtml)
/// * `program` class: program
pub type glGetFragDataIndex_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// [glGetFragDataIndexEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragDataIndexEXT.xhtml)
/// * `program` class: program
pub type glGetFragDataIndexEXT_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// [glGetFragDataLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragDataLocation.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glGetFragDataLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// [glGetFragDataLocationEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragDataLocationEXT.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glGetFragDataLocationEXT_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// [glGetFragmentLightfvSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragmentLightfvSGIX.xhtml)
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `params` len: COMPSIZE(pname)
pub type glGetFragmentLightfvSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, params: *mut GLfloat);

/// [glGetFragmentLightivSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragmentLightivSGIX.xhtml)
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `params` len: COMPSIZE(pname)
pub type glGetFragmentLightivSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, params: *mut GLint);

/// [glGetFragmentMaterialfvSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragmentMaterialfvSGIX.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetFragmentMaterialfvSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLfloat);

/// [glGetFragmentMaterialivSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragmentMaterialivSGIX.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetFragmentMaterialivSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLint);

/// [glGetFramebufferAttachmentParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferAttachmentParameteriv.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// [glGetFramebufferAttachmentParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferAttachmentParameterivEXT.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferAttachmentParameterivEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// [glGetFramebufferAttachmentParameterivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferAttachmentParameterivOES.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferAttachmentParameterivOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// [glGetFramebufferParameterfvAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferParameterfvAMD.xhtml)
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferAttachmentParameterName
pub type glGetFramebufferParameterfvAMD_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, numsamples: GLuint, pixelindex: GLuint, size: GLsizei, values: *mut GLfloat);

/// [glGetFramebufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferParameteriv.xhtml)
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferParameteriv_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// [glGetFramebufferParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferParameterivEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `pname` group: GetFramebufferParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GetFramebufferParameter, params: *mut GLint);

/// [glGetFramebufferParameterivMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferParameterivMESA.xhtml)
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferParameterivMESA_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// [glGetFramebufferPixelLocalStorageSizeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferPixelLocalStorageSizeEXT.xhtml)
/// * `target` group: FramebufferTarget
pub type glGetFramebufferPixelLocalStorageSizeEXT_t = unsafe extern "system" fn(target: GLuint) -> GLsizei;

/// [glGetGraphicsResetStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetGraphicsResetStatus.xhtml)
pub type glGetGraphicsResetStatus_t = unsafe extern "system" fn() -> GraphicsResetStatus;

/// [glGetGraphicsResetStatusARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetGraphicsResetStatusARB.xhtml)
pub type glGetGraphicsResetStatusARB_t = unsafe extern "system" fn() -> GraphicsResetStatus;

/// [glGetGraphicsResetStatusEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetGraphicsResetStatusEXT.xhtml)
pub type glGetGraphicsResetStatusEXT_t = unsafe extern "system" fn() -> GraphicsResetStatus;

/// [glGetGraphicsResetStatusKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetGraphicsResetStatusKHR.xhtml)
pub type glGetGraphicsResetStatusKHR_t = unsafe extern "system" fn() -> GraphicsResetStatus;

/// [glGetHandleARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetHandleARB.xhtml)
pub type glGetHandleARB_t = unsafe extern "system" fn(pname: GLenum) -> GLhandleARB;

/// [glGetHistogram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetHistogram.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: COMPSIZE(target,format,type)
pub type glGetHistogram_t = unsafe extern "system" fn(target: HistogramTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, values: *mut void);

/// [glGetHistogramEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetHistogramEXT.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: COMPSIZE(target,format,type)
pub type glGetHistogramEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, values: *mut void);

/// [glGetHistogramParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetHistogramParameterfv.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameterfv_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLfloat);

/// [glGetHistogramParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetHistogramParameterfvEXT.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameterfvEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLfloat);

/// [glGetHistogramParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetHistogramParameteriv.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameteriv_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLint);

/// [glGetHistogramParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetHistogramParameterivEXT.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameterivEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLint);

/// [glGetHistogramParameterxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetHistogramParameterxvOES.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameterxvOES_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLfixed);

/// [glGetImageHandleARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetImageHandleARB.xhtml)
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `format` group: PixelFormat
pub type glGetImageHandleARB_t = unsafe extern "system" fn(texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: PixelFormat) -> GLuint64;

/// [glGetImageHandleNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetImageHandleNV.xhtml)
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `format` group: PixelFormat
pub type glGetImageHandleNV_t = unsafe extern "system" fn(texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: PixelFormat) -> GLuint64;

/// [glGetImageTransformParameterfvHP](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetImageTransformParameterfvHP.xhtml)
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
/// * `params` len: COMPSIZE(pname)
pub type glGetImageTransformParameterfvHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, params: *mut GLfloat);

/// [glGetImageTransformParameterivHP](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetImageTransformParameterivHP.xhtml)
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
/// * `params` len: COMPSIZE(pname)
pub type glGetImageTransformParameterivHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, params: *mut GLint);

/// [glGetInfoLogARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInfoLogARB.xhtml)
/// * `obj` group: handleARB
/// * `infoLog` len: maxLength
pub type glGetInfoLogARB_t = unsafe extern "system" fn(obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLcharARB);

/// [glGetInstrumentsSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInstrumentsSGIX.xhtml)
pub type glGetInstrumentsSGIX_t = unsafe extern "system" fn() -> GLint;

/// [glGetInteger64i_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInteger64i_v.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetInteger64i_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint64);

/// [glGetInteger64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInteger64v.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetInteger64v_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint64);

/// [glGetInteger64vAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInteger64vAPPLE.xhtml)
/// * `pname` group: GetPName
pub type glGetInteger64vAPPLE_t = unsafe extern "system" fn(pname: GetPName, params: *mut GLint64);

/// [glGetInteger64vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInteger64vEXT.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetInteger64vEXT_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint64);

/// [glGetIntegerIndexedvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegerIndexedvEXT.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetIntegerIndexedvEXT_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint);

/// [glGetIntegeri_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegeri_v.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetIntegeri_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint);

/// [glGetIntegeri_vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegeri_vEXT.xhtml)
/// * `target` group: GetPName
pub type glGetIntegeri_vEXT_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint);

/// [glGetIntegerui64i_vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegerui64i_vNV.xhtml)
/// * `result` len: COMPSIZE(value)
pub type glGetIntegerui64i_vNV_t = unsafe extern "system" fn(value: GLenum, index: GLuint, result: *mut GLuint64EXT);

/// [glGetIntegerui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegerui64vNV.xhtml)
/// * `result` len: COMPSIZE(value)
pub type glGetIntegerui64vNV_t = unsafe extern "system" fn(value: GLenum, result: *mut GLuint64EXT);

/// [glGetIntegerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegerv.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetIntegerv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint);

/// [glGetInternalformatSampleivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInternalformatSampleivNV.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `pname` group: InternalFormatPName
/// * `params` len: count
pub type glGetInternalformatSampleivNV_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, samples: GLsizei, pname: InternalFormatPName, count: GLsizei, params: *mut GLint);

/// [glGetInternalformati64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInternalformati64v.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `pname` group: InternalFormatPName
/// * `params` len: count
pub type glGetInternalformati64v_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint64);

/// [glGetInternalformativ](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInternalformativ.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `pname` group: InternalFormatPName
/// * `params` len: count
pub type glGetInternalformativ_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint);

/// [glGetInvariantBooleanvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInvariantBooleanvEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(id)
pub type glGetInvariantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLboolean);

/// [glGetInvariantFloatvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInvariantFloatvEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetInvariantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLfloat);

/// [glGetInvariantIntegervEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInvariantIntegervEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetInvariantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLint);

/// [glGetLightfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetLightfv.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightfv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLfloat);

/// [glGetLightiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetLightiv.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightiv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLint);

/// [glGetLightxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetLightxOES.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightxOES_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLfixed);

/// [glGetLightxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetLightxv.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightxv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLfixed);

/// [glGetLightxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetLightxvOES.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightxvOES_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLfixed);

/// [glGetListParameterfvSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetListParameterfvSGIX.xhtml)
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glGetListParameterfvSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, params: *mut GLfloat);

/// [glGetListParameterivSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetListParameterivSGIX.xhtml)
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glGetListParameterivSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, params: *mut GLint);

/// [glGetLocalConstantBooleanvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetLocalConstantBooleanvEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(id)
pub type glGetLocalConstantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLboolean);

/// [glGetLocalConstantFloatvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetLocalConstantFloatvEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetLocalConstantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLfloat);

/// [glGetLocalConstantIntegervEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetLocalConstantIntegervEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetLocalConstantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLint);

/// [glGetMapAttribParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapAttribParameterfvNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `pname` group: MapAttribParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetMapAttribParameterfvNV_t = unsafe extern "system" fn(target: EvalTargetNV, index: GLuint, pname: MapAttribParameterNV, params: *mut GLfloat);

/// [glGetMapAttribParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapAttribParameterivNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `pname` group: MapAttribParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetMapAttribParameterivNV_t = unsafe extern "system" fn(target: EvalTargetNV, index: GLuint, pname: MapAttribParameterNV, params: *mut GLint);

/// [glGetMapControlPointsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapControlPointsNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `type` group: MapTypeNV
/// * `packed` group: Boolean
/// * `points` len: COMPSIZE(target)
pub type glGetMapControlPointsNV_t = unsafe extern "system" fn(target: EvalTargetNV, index: GLuint, type_: MapTypeNV, ustride: GLsizei, vstride: GLsizei, packed: GLboolean, points: *mut void);

/// [glGetMapParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapParameterfvNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `pname` group: MapParameterNV
/// * `params` len: COMPSIZE(target,pname)
pub type glGetMapParameterfvNV_t = unsafe extern "system" fn(target: EvalTargetNV, pname: MapParameterNV, params: *mut GLfloat);

/// [glGetMapParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapParameterivNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `pname` group: MapParameterNV
/// * `params` len: COMPSIZE(target,pname)
pub type glGetMapParameterivNV_t = unsafe extern "system" fn(target: EvalTargetNV, pname: MapParameterNV, params: *mut GLint);

/// [glGetMapdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapdv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: GetMapQuery
/// * `v` len: COMPSIZE(target,query)
pub type glGetMapdv_t = unsafe extern "system" fn(target: MapTarget, query: GetMapQuery, v: *mut GLdouble);

/// [glGetMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapfv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: GetMapQuery
/// * `v` len: COMPSIZE(target,query)
pub type glGetMapfv_t = unsafe extern "system" fn(target: MapTarget, query: GetMapQuery, v: *mut GLfloat);

/// [glGetMapiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapiv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: GetMapQuery
/// * `v` len: COMPSIZE(target,query)
pub type glGetMapiv_t = unsafe extern "system" fn(target: MapTarget, query: GetMapQuery, v: *mut GLint);

/// [glGetMapxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMapxvOES.xhtml)
/// * `target` group: MapTarget
/// * `query` group: GetMapQuery
/// * `v` len: COMPSIZE(query)
pub type glGetMapxvOES_t = unsafe extern "system" fn(target: MapTarget, query: GetMapQuery, v: *mut GLfixed);

/// [glGetMaterialfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMaterialfv.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMaterialfv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLfloat);

/// [glGetMaterialiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMaterialiv.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMaterialiv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLint);

/// [glGetMaterialxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMaterialxOES.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
pub type glGetMaterialxOES_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfixed);

/// [glGetMaterialxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMaterialxv.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMaterialxv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLfixed);

/// [glGetMaterialxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMaterialxvOES.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMaterialxvOES_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLfixed);

/// [glGetMemoryObjectDetachedResourcesuivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMemoryObjectDetachedResourcesuivNV.xhtml)
pub type glGetMemoryObjectDetachedResourcesuivNV_t = unsafe extern "system" fn(memory: GLuint, pname: GLenum, first: GLint, count: GLsizei, params: *mut GLuint);

/// [glGetMemoryObjectParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMemoryObjectParameterivEXT.xhtml)
/// * `pname` group: MemoryObjectParameterName
pub type glGetMemoryObjectParameterivEXT_t = unsafe extern "system" fn(memoryObject: GLuint, pname: MemoryObjectParameterName, params: *mut GLint);

/// [glGetMinmax](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMinmax.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: COMPSIZE(target,format,type)
pub type glGetMinmax_t = unsafe extern "system" fn(target: MinmaxTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, values: *mut void);

/// [glGetMinmaxEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMinmaxEXT.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: COMPSIZE(target,format,type)
pub type glGetMinmaxEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, values: *mut void);

/// [glGetMinmaxParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMinmaxParameterfv.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `pname` group: GetMinmaxParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetMinmaxParameterfv_t = unsafe extern "system" fn(target: MinmaxTargetEXT, pname: GetMinmaxParameterPNameEXT, params: *mut GLfloat);

/// [glGetMinmaxParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMinmaxParameterfvEXT.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `pname` group: GetMinmaxParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetMinmaxParameterfvEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT, pname: GetMinmaxParameterPNameEXT, params: *mut GLfloat);

/// [glGetMinmaxParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMinmaxParameteriv.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `pname` group: GetMinmaxParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetMinmaxParameteriv_t = unsafe extern "system" fn(target: MinmaxTargetEXT, pname: GetMinmaxParameterPNameEXT, params: *mut GLint);

/// [glGetMinmaxParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMinmaxParameterivEXT.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `pname` group: GetMinmaxParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetMinmaxParameterivEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT, pname: GetMinmaxParameterPNameEXT, params: *mut GLint);

/// [glGetMultiTexEnvfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexEnvfvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexEnvfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLfloat);

/// [glGetMultiTexEnvivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexEnvivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexEnvivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLint);

/// [glGetMultiTexGendvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexGendvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexGendvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLdouble);

/// [glGetMultiTexGenfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexGenfvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexGenfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLfloat);

/// [glGetMultiTexGenivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexGenivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexGenivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLint);

/// [glGetMultiTexImageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexImageEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(target,level,format,type)
pub type glGetMultiTexImageEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);

/// [glGetMultiTexLevelParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexLevelParameterfvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexLevelParameterfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

/// [glGetMultiTexLevelParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexLevelParameterivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexLevelParameterivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);

/// [glGetMultiTexParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexParameterIivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexParameterIivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// [glGetMultiTexParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexParameterIuivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexParameterIuivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// [glGetMultiTexParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexParameterfvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexParameterfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);

/// [glGetMultiTexParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultiTexParameterivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexParameterivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// [glGetMultisamplefv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultisamplefv.xhtml)
/// * `pname` group: GetMultisamplePNameNV
/// * `val` len: COMPSIZE(pname)
pub type glGetMultisamplefv_t = unsafe extern "system" fn(pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat);

/// [glGetMultisamplefvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultisamplefvNV.xhtml)
/// * `pname` group: GetMultisamplePNameNV
pub type glGetMultisamplefvNV_t = unsafe extern "system" fn(pname: GetMultisamplePNameNV, index: GLuint, val: *mut [GLfloat; 2]);

/// [glGetNamedBufferParameteri64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferParameteri64v.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
pub type glGetNamedBufferParameteri64v_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint64);

/// [glGetNamedBufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferParameteriv.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
pub type glGetNamedBufferParameteriv_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint);

/// [glGetNamedBufferParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferParameterivEXT.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedBufferParameterivEXT_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint);

/// [glGetNamedBufferParameterui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferParameterui64vNV.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedBufferParameterui64vNV_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLuint64EXT);

/// [glGetNamedBufferPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferPointerv.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPointerNameARB
pub type glGetNamedBufferPointerv_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPointerNameARB, params: *mut *mut void);

/// [glGetNamedBufferPointervEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferPointervEXT.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPointerNameARB
pub type glGetNamedBufferPointervEXT_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPointerNameARB, params: *mut *mut void);

/// [glGetNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferSubData.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
pub type glGetNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void);

/// [glGetNamedBufferSubDataEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferSubDataEXT.xhtml)
/// * `buffer` class: buffer
/// * `data` len: COMPSIZE(size)
pub type glGetNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void);

/// [glGetNamedFramebufferAttachmentParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferAttachmentParameteriv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
pub type glGetNamedFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// [glGetNamedFramebufferAttachmentParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferAttachmentParameterivEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedFramebufferAttachmentParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// [glGetNamedFramebufferParameterfvAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferParameterfvAMD.xhtml)
/// * `framebuffer` class: framebuffer
pub type glGetNamedFramebufferParameterfvAMD_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, numsamples: GLuint, pixelindex: GLuint, size: GLsizei, values: *mut GLfloat);

/// [glGetNamedFramebufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferParameteriv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `pname` group: GetFramebufferParameter
pub type glGetNamedFramebufferParameteriv_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GetFramebufferParameter, param: *mut GLint);

/// [glGetNamedFramebufferParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferParameterivEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `pname` group: GetFramebufferParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedFramebufferParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GetFramebufferParameter, params: *mut GLint);

/// [glGetNamedProgramLocalParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedProgramLocalParameterIivEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glGetNamedProgramLocalParameterIivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *mut [GLint; 4]);

/// [glGetNamedProgramLocalParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedProgramLocalParameterIuivEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glGetNamedProgramLocalParameterIuivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *mut [GLuint; 4]);

/// [glGetNamedProgramLocalParameterdvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedProgramLocalParameterdvEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glGetNamedProgramLocalParameterdvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *mut [GLdouble; 4]);

/// [glGetNamedProgramLocalParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedProgramLocalParameterfvEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glGetNamedProgramLocalParameterfvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *mut [GLfloat; 4]);

/// [glGetNamedProgramStringEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedProgramStringEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `pname` group: ProgramStringProperty
/// * `string` len: COMPSIZE(program,pname)
pub type glGetNamedProgramStringEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, pname: ProgramStringProperty, string: *mut void);

/// [glGetNamedProgramivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedProgramivEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `pname` group: ProgramPropertyARB
pub type glGetNamedProgramivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, pname: ProgramPropertyARB, params: *mut GLint);

/// [glGetNamedRenderbufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedRenderbufferParameteriv.xhtml)
/// * `renderbuffer` class: renderbuffer
/// * `pname` group: RenderbufferParameterName
pub type glGetNamedRenderbufferParameteriv_t = unsafe extern "system" fn(renderbuffer: GLuint, pname: RenderbufferParameterName, params: *mut GLint);

/// [glGetNamedRenderbufferParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedRenderbufferParameterivEXT.xhtml)
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedRenderbufferParameterivEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, pname: RenderbufferParameterName, params: *mut GLint);

/// [glGetNamedStringARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedStringARB.xhtml)
/// * `name` len: namelen
/// * `string` len: bufSize
pub type glGetNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar, bufSize: GLsizei, stringlen: *mut GLint, string: *mut GLchar);

/// [glGetNamedStringivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedStringivARB.xhtml)
/// * `name` len: namelen
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedStringivARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar, pname: GLenum, params: *mut GLint);

/// [glGetNextPerfQueryIdINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNextPerfQueryIdINTEL.xhtml)
pub type glGetNextPerfQueryIdINTEL_t = unsafe extern "system" fn(queryId: GLuint, nextQueryId: *mut GLuint);

/// [glGetObjectBufferfvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectBufferfvATI.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: ArrayObjectPNameATI
pub type glGetObjectBufferfvATI_t = unsafe extern "system" fn(buffer: GLuint, pname: ArrayObjectPNameATI, params: *mut GLfloat);

/// [glGetObjectBufferivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectBufferivATI.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: ArrayObjectPNameATI
pub type glGetObjectBufferivATI_t = unsafe extern "system" fn(buffer: GLuint, pname: ArrayObjectPNameATI, params: *mut GLint);

/// [glGetObjectLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectLabel.xhtml)
/// * `identifier` group: ObjectIdentifier
/// * `label` len: bufSize
pub type glGetObjectLabel_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// [glGetObjectLabelEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectLabelEXT.xhtml)
/// * `label` len: bufSize
pub type glGetObjectLabelEXT_t = unsafe extern "system" fn(type_: GLenum, object: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// [glGetObjectLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectLabelKHR.xhtml)
/// * `label` len: bufSize
pub type glGetObjectLabelKHR_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// [glGetObjectParameterfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectParameterfvARB.xhtml)
/// * `obj` group: handleARB
/// * `params` len: COMPSIZE(pname)
pub type glGetObjectParameterfvARB_t = unsafe extern "system" fn(obj: GLhandleARB, pname: GLenum, params: *mut GLfloat);

/// [glGetObjectParameterivAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectParameterivAPPLE.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetObjectParameterivAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, pname: GLenum, params: *mut GLint);

/// [glGetObjectParameterivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectParameterivARB.xhtml)
/// * `obj` group: handleARB
/// * `params` len: COMPSIZE(pname)
pub type glGetObjectParameterivARB_t = unsafe extern "system" fn(obj: GLhandleARB, pname: GLenum, params: *mut GLint);

/// [glGetObjectPtrLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectPtrLabel.xhtml)
/// * `label` len: bufSize
pub type glGetObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// [glGetObjectPtrLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectPtrLabelKHR.xhtml)
/// * `label` len: bufSize
pub type glGetObjectPtrLabelKHR_t = unsafe extern "system" fn(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// [glGetOcclusionQueryivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetOcclusionQueryivNV.xhtml)
/// * `pname` group: OcclusionQueryParameterNameNV
/// * `params` len: COMPSIZE(pname)
pub type glGetOcclusionQueryivNV_t = unsafe extern "system" fn(id: GLuint, pname: OcclusionQueryParameterNameNV, params: *mut GLint);

/// [glGetOcclusionQueryuivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetOcclusionQueryuivNV.xhtml)
/// * `pname` group: OcclusionQueryParameterNameNV
/// * `params` len: COMPSIZE(pname)
pub type glGetOcclusionQueryuivNV_t = unsafe extern "system" fn(id: GLuint, pname: OcclusionQueryParameterNameNV, params: *mut GLuint);

/// [glGetPathColorGenfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathColorGenfvNV.xhtml)
/// * `color` group: PathColor
/// * `pname` group: PathGenMode
/// * `value` len: COMPSIZE(pname)
pub type glGetPathColorGenfvNV_t = unsafe extern "system" fn(color: PathColor, pname: PathGenMode, value: *mut GLfloat);

/// [glGetPathColorGenivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathColorGenivNV.xhtml)
/// * `color` group: PathColor
/// * `pname` group: PathGenMode
/// * `value` len: COMPSIZE(pname)
pub type glGetPathColorGenivNV_t = unsafe extern "system" fn(color: PathColor, pname: PathGenMode, value: *mut GLint);

/// [glGetPathCommandsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathCommandsNV.xhtml)
/// * `path` group: Path
/// * `commands` group: PathCommand
/// * `commands` len: COMPSIZE(path)
pub type glGetPathCommandsNV_t = unsafe extern "system" fn(path: GLuint, commands: *mut GLubyte);

/// [glGetPathCoordsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathCoordsNV.xhtml)
/// * `path` group: Path
/// * `coords` len: COMPSIZE(path)
pub type glGetPathCoordsNV_t = unsafe extern "system" fn(path: GLuint, coords: *mut GLfloat);

/// [glGetPathDashArrayNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathDashArrayNV.xhtml)
/// * `path` group: Path
/// * `dashArray` len: COMPSIZE(path)
pub type glGetPathDashArrayNV_t = unsafe extern "system" fn(path: GLuint, dashArray: *mut GLfloat);

/// [glGetPathLengthNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathLengthNV.xhtml)
/// * `path` group: Path
pub type glGetPathLengthNV_t = unsafe extern "system" fn(path: GLuint, startSegment: GLsizei, numSegments: GLsizei) -> GLfloat;

/// [glGetPathMetricRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathMetricRangeNV.xhtml)
/// * `metricQueryMask` group: PathMetricMask
/// * `firstPathName` group: Path
/// * `metrics` len: COMPSIZE(metricQueryMask,numPaths,stride)
pub type glGetPathMetricRangeNV_t = unsafe extern "system" fn(metricQueryMask: GLbitfield, firstPathName: GLuint, numPaths: GLsizei, stride: GLsizei, metrics: *mut GLfloat);

/// [glGetPathMetricsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathMetricsNV.xhtml)
/// * `metricQueryMask` group: PathMetricMask
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `metrics` len: COMPSIZE(metricQueryMask,numPaths,stride)
pub type glGetPathMetricsNV_t = unsafe extern "system" fn(metricQueryMask: GLbitfield, numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, stride: GLsizei, metrics: *mut GLfloat);

/// [glGetPathParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathParameterfvNV.xhtml)
/// * `path` group: Path
/// * `pname` group: PathParameter
pub type glGetPathParameterfvNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: *mut [GLfloat; 4]);

/// [glGetPathParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathParameterivNV.xhtml)
/// * `path` group: Path
/// * `pname` group: PathParameter
pub type glGetPathParameterivNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: *mut [GLint; 4]);

/// [glGetPathSpacingNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathSpacingNV.xhtml)
/// * `pathListMode` group: PathListMode
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `transformType` group: PathTransformType
/// * `returnedSpacing` len: COMPSIZE(pathListMode,numPaths)
pub type glGetPathSpacingNV_t = unsafe extern "system" fn(pathListMode: PathListMode, numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, advanceScale: GLfloat, kerningScale: GLfloat, transformType: PathTransformType, returnedSpacing: *mut GLfloat);

/// [glGetPathTexGenfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathTexGenfvNV.xhtml)
/// * `texCoordSet` group: TextureUnit
/// * `pname` group: PathGenMode
/// * `value` len: COMPSIZE(pname)
pub type glGetPathTexGenfvNV_t = unsafe extern "system" fn(texCoordSet: TextureUnit, pname: PathGenMode, value: *mut GLfloat);

/// [glGetPathTexGenivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPathTexGenivNV.xhtml)
/// * `texCoordSet` group: TextureUnit
/// * `pname` group: PathGenMode
/// * `value` len: COMPSIZE(pname)
pub type glGetPathTexGenivNV_t = unsafe extern "system" fn(texCoordSet: TextureUnit, pname: PathGenMode, value: *mut GLint);

/// [glGetPerfCounterInfoINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfCounterInfoINTEL.xhtml)
pub type glGetPerfCounterInfoINTEL_t = unsafe extern "system" fn(queryId: GLuint, counterId: GLuint, counterNameLength: GLuint, counterName: *mut GLchar, counterDescLength: GLuint, counterDesc: *mut GLchar, counterOffset: *mut GLuint, counterDataSize: *mut GLuint, counterTypeEnum: *mut GLuint, counterDataTypeEnum: *mut GLuint, rawCounterMaxValue: *mut GLuint64);

/// [glGetPerfMonitorCounterDataAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfMonitorCounterDataAMD.xhtml)
/// * `data` len: dataSize / 4
pub type glGetPerfMonitorCounterDataAMD_t = unsafe extern "system" fn(monitor: GLuint, pname: GLenum, dataSize: GLsizei, data: *mut GLuint, bytesWritten: *mut GLint);

/// [glGetPerfMonitorCounterInfoAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfMonitorCounterInfoAMD.xhtml)
/// * `data` len: COMPSIZE(pname)
pub type glGetPerfMonitorCounterInfoAMD_t = unsafe extern "system" fn(group: GLuint, counter: GLuint, pname: GLenum, data: *mut void);

/// [glGetPerfMonitorCounterStringAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfMonitorCounterStringAMD.xhtml)
/// * `counterString` len: bufSize
pub type glGetPerfMonitorCounterStringAMD_t = unsafe extern "system" fn(group: GLuint, counter: GLuint, bufSize: GLsizei, length: *mut GLsizei, counterString: *mut GLchar);

/// [glGetPerfMonitorCountersAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfMonitorCountersAMD.xhtml)
/// * `counters` len: counterSize
pub type glGetPerfMonitorCountersAMD_t = unsafe extern "system" fn(group: GLuint, numCounters: *mut GLint, maxActiveCounters: *mut GLint, counterSize: GLsizei, counters: *mut GLuint);

/// [glGetPerfMonitorGroupStringAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfMonitorGroupStringAMD.xhtml)
/// * `groupString` len: bufSize
pub type glGetPerfMonitorGroupStringAMD_t = unsafe extern "system" fn(group: GLuint, bufSize: GLsizei, length: *mut GLsizei, groupString: *mut GLchar);

/// [glGetPerfMonitorGroupsAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfMonitorGroupsAMD.xhtml)
/// * `groups` len: groupsSize
pub type glGetPerfMonitorGroupsAMD_t = unsafe extern "system" fn(numGroups: *mut GLint, groupsSize: GLsizei, groups: *mut GLuint);

/// [glGetPerfQueryDataINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfQueryDataINTEL.xhtml)
pub type glGetPerfQueryDataINTEL_t = unsafe extern "system" fn(queryHandle: GLuint, flags: GLuint, dataSize: GLsizei, data: *mut void, bytesWritten: *mut GLuint);

/// [glGetPerfQueryIdByNameINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfQueryIdByNameINTEL.xhtml)
pub type glGetPerfQueryIdByNameINTEL_t = unsafe extern "system" fn(queryName: *mut GLchar, queryId: *mut GLuint);

/// [glGetPerfQueryInfoINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPerfQueryInfoINTEL.xhtml)
pub type glGetPerfQueryInfoINTEL_t = unsafe extern "system" fn(queryId: GLuint, queryNameLength: GLuint, queryName: *mut GLchar, dataSize: *mut GLuint, noCounters: *mut GLuint, noInstances: *mut GLuint, capsMask: *mut GLuint);

/// [glGetPixelMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPixelMapfv.xhtml)
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(map)
pub type glGetPixelMapfv_t = unsafe extern "system" fn(map: PixelMap, values: *mut GLfloat);

/// [glGetPixelMapuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPixelMapuiv.xhtml)
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(map)
pub type glGetPixelMapuiv_t = unsafe extern "system" fn(map: PixelMap, values: *mut GLuint);

/// [glGetPixelMapusv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPixelMapusv.xhtml)
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(map)
pub type glGetPixelMapusv_t = unsafe extern "system" fn(map: PixelMap, values: *mut GLushort);

/// [glGetPixelMapxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPixelMapxv.xhtml)
/// * `map` group: PixelMap
/// * `values` len: size
pub type glGetPixelMapxv_t = unsafe extern "system" fn(map: PixelMap, size: GLint, values: *mut GLfixed);

/// [glGetPixelTexGenParameterfvSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPixelTexGenParameterfvSGIS.xhtml)
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glGetPixelTexGenParameterfvSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, params: *mut GLfloat);

/// [glGetPixelTexGenParameterivSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPixelTexGenParameterivSGIS.xhtml)
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glGetPixelTexGenParameterivSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, params: *mut GLint);

/// [glGetPixelTransformParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPixelTransformParameterfvEXT.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetPixelTransformParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

/// [glGetPixelTransformParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPixelTransformParameterivEXT.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetPixelTransformParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

/// [glGetPointerIndexedvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointerIndexedvEXT.xhtml)
pub type glGetPointerIndexedvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut *mut void);

/// [glGetPointeri_vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointeri_vEXT.xhtml)
pub type glGetPointeri_vEXT_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, params: *mut *mut void);

/// [glGetPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointerv.xhtml)
/// * `pname` group: GetPointervPName
pub type glGetPointerv_t = unsafe extern "system" fn(pname: GetPointervPName, params: *mut *mut void);

/// [glGetPointervEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointervEXT.xhtml)
/// * `pname` group: GetPointervPName
pub type glGetPointervEXT_t = unsafe extern "system" fn(pname: GetPointervPName, params: *mut *mut void);

/// [glGetPointervKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointervKHR.xhtml)
pub type glGetPointervKHR_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut void);

/// [glGetPolygonStipple](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPolygonStipple.xhtml)
/// * `mask` len: COMPSIZE()
pub type glGetPolygonStipple_t = unsafe extern "system" fn(mask: *mut GLubyte);

/// [glGetProgramBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramBinary.xhtml)
/// * `program` class: program
/// * `binary` len: bufSize
pub type glGetProgramBinary_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void);

/// [glGetProgramBinaryOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramBinaryOES.xhtml)
/// * `program` class: program
/// * `binary` len: bufSize
pub type glGetProgramBinaryOES_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void);

/// [glGetProgramEnvParameterIivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramEnvParameterIivNV.xhtml)
/// * `target` group: ProgramTarget
pub type glGetProgramEnvParameterIivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLint; 4]);

/// [glGetProgramEnvParameterIuivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramEnvParameterIuivNV.xhtml)
/// * `target` group: ProgramTarget
pub type glGetProgramEnvParameterIuivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLuint; 4]);

/// [glGetProgramEnvParameterdvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramEnvParameterdvARB.xhtml)
/// * `target` group: ProgramTarget
pub type glGetProgramEnvParameterdvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLdouble; 4]);

/// [glGetProgramEnvParameterfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramEnvParameterfvARB.xhtml)
/// * `target` group: ProgramTarget
pub type glGetProgramEnvParameterfvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLfloat; 4]);

/// [glGetProgramInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInfoLog.xhtml)
/// * `program` class: program
/// * `infoLog` len: bufSize
pub type glGetProgramInfoLog_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

/// [glGetProgramInterfaceiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInterfaceiv.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `pname` group: ProgramInterfacePName
/// * `params` len: COMPSIZE(pname)
pub type glGetProgramInterfaceiv_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, pname: ProgramInterfacePName, params: *mut GLint);

/// [glGetProgramLocalParameterIivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramLocalParameterIivNV.xhtml)
/// * `target` group: ProgramTarget
pub type glGetProgramLocalParameterIivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLint; 4]);

/// [glGetProgramLocalParameterIuivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramLocalParameterIuivNV.xhtml)
/// * `target` group: ProgramTarget
pub type glGetProgramLocalParameterIuivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLuint; 4]);

/// [glGetProgramLocalParameterdvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramLocalParameterdvARB.xhtml)
/// * `target` group: ProgramTarget
pub type glGetProgramLocalParameterdvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLdouble; 4]);

/// [glGetProgramLocalParameterfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramLocalParameterfvARB.xhtml)
/// * `target` group: ProgramTarget
pub type glGetProgramLocalParameterfvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLfloat; 4]);

/// [glGetProgramNamedParameterdvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramNamedParameterdvNV.xhtml)
/// * `id` class: program
pub type glGetProgramNamedParameterdvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, params: *mut [GLdouble; 4]);

/// [glGetProgramNamedParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramNamedParameterfvNV.xhtml)
/// * `id` class: program
pub type glGetProgramNamedParameterfvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, params: *mut [GLfloat; 4]);

/// [glGetProgramParameterdvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramParameterdvNV.xhtml)
/// * `target` group: VertexAttribEnumNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetProgramParameterdvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, pname: VertexAttribEnumNV, params: *mut [GLdouble; 4]);

/// [glGetProgramParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramParameterfvNV.xhtml)
/// * `target` group: VertexAttribEnumNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetProgramParameterfvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, pname: VertexAttribEnumNV, params: *mut [GLfloat; 4]);

/// [glGetProgramPipelineInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramPipelineInfoLog.xhtml)
/// * `pipeline` class: program pipeline
/// * `infoLog` len: bufSize
pub type glGetProgramPipelineInfoLog_t = unsafe extern "system" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

/// [glGetProgramPipelineInfoLogEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramPipelineInfoLogEXT.xhtml)
/// * `pipeline` class: program pipeline
/// * `infoLog` len: bufSize
pub type glGetProgramPipelineInfoLogEXT_t = unsafe extern "system" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

/// [glGetProgramPipelineiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramPipelineiv.xhtml)
/// * `pipeline` class: program pipeline
/// * `pname` group: PipelineParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetProgramPipelineiv_t = unsafe extern "system" fn(pipeline: GLuint, pname: PipelineParameterName, params: *mut GLint);

/// [glGetProgramPipelineivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramPipelineivEXT.xhtml)
/// * `pipeline` class: program pipeline
/// * `pname` group: PipelineParameterName
pub type glGetProgramPipelineivEXT_t = unsafe extern "system" fn(pipeline: GLuint, pname: PipelineParameterName, params: *mut GLint);

/// [glGetProgramResourceIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceIndex.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
pub type glGetProgramResourceIndex_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLuint;

/// [glGetProgramResourceLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceLocation.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
pub type glGetProgramResourceLocation_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;

/// [glGetProgramResourceLocationIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceLocationIndex.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
pub type glGetProgramResourceLocationIndex_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;

/// [glGetProgramResourceLocationIndexEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceLocationIndexEXT.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
pub type glGetProgramResourceLocationIndexEXT_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;

/// [glGetProgramResourceName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceName.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: bufSize
pub type glGetProgramResourceName_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

/// [glGetProgramResourcefvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourcefvNV.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `params` len: count
pub type glGetProgramResourcefvNV_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLfloat);

/// [glGetProgramResourceiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceiv.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `props` group: ProgramResourceProperty
/// * `props` len: propCount
/// * `params` len: count
pub type glGetProgramResourceiv_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, index: GLuint, propCount: GLsizei, props: *const ProgramResourceProperty, count: GLsizei, length: *mut GLsizei, params: *mut GLint);

/// [glGetProgramStageiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramStageiv.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `pname` group: ProgramStagePName
pub type glGetProgramStageiv_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, pname: ProgramStagePName, values: *mut GLint);

/// [glGetProgramStringARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramStringARB.xhtml)
/// * `target` group: ProgramTarget
/// * `pname` group: ProgramStringProperty
/// * `string` len: COMPSIZE(target,pname)
pub type glGetProgramStringARB_t = unsafe extern "system" fn(target: ProgramTarget, pname: ProgramStringProperty, string: *mut void);

/// [glGetProgramStringNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramStringNV.xhtml)
/// * `id` class: program
/// * `pname` group: VertexAttribEnumNV
/// * `program` group: ProgramCharacterNV
/// * `program` len: COMPSIZE(id,pname)
pub type glGetProgramStringNV_t = unsafe extern "system" fn(id: GLuint, pname: VertexAttribEnumNV, program: *mut GLubyte);

/// [glGetProgramSubroutineParameteruivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramSubroutineParameteruivNV.xhtml)
/// * `param` len: COMPSIZE(target)
pub type glGetProgramSubroutineParameteruivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, param: *mut GLuint);

/// [glGetProgramiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramiv.xhtml)
/// * `program` class: program
/// * `pname` group: ProgramPropertyARB
/// * `params` len: COMPSIZE(pname)
pub type glGetProgramiv_t = unsafe extern "system" fn(program: GLuint, pname: ProgramPropertyARB, params: *mut GLint);

/// [glGetProgramivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramivARB.xhtml)
/// * `target` group: ProgramTarget
/// * `pname` group: ProgramPropertyARB
pub type glGetProgramivARB_t = unsafe extern "system" fn(target: ProgramTarget, pname: ProgramPropertyARB, params: *mut GLint);

/// [glGetProgramivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramivNV.xhtml)
/// * `id` class: program
/// * `pname` group: VertexAttribEnumNV
pub type glGetProgramivNV_t = unsafe extern "system" fn(id: GLuint, pname: VertexAttribEnumNV, params: *mut [GLint; 4]);

/// [glGetQueryBufferObjecti64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjecti64v.xhtml)
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
pub type glGetQueryBufferObjecti64v_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

/// [glGetQueryBufferObjectiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectiv.xhtml)
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
pub type glGetQueryBufferObjectiv_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

/// [glGetQueryBufferObjectui64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectui64v.xhtml)
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
pub type glGetQueryBufferObjectui64v_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

/// [glGetQueryBufferObjectuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectuiv.xhtml)
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
pub type glGetQueryBufferObjectuiv_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

/// [glGetQueryIndexediv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryIndexediv.xhtml)
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryIndexediv_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint, pname: QueryParameterName, params: *mut GLint);

/// [glGetQueryObjecti64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjecti64v.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjecti64v_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64);

/// [glGetQueryObjecti64vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjecti64vEXT.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjecti64vEXT_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64);

/// [glGetQueryObjectiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectiv.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectiv_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);

/// [glGetQueryObjectivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectivARB.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectivARB_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);

/// [glGetQueryObjectivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectivEXT.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectivEXT_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);

/// [glGetQueryObjectui64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectui64v.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectui64v_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64);

/// [glGetQueryObjectui64vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectui64vEXT.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectui64vEXT_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64);

/// [glGetQueryObjectuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectuiv.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectuiv_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);

/// [glGetQueryObjectuivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectuivARB.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectuivARB_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);

/// [glGetQueryObjectuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectuivEXT.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectuivEXT_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);

/// [glGetQueryiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryiv.xhtml)
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryiv_t = unsafe extern "system" fn(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);

/// [glGetQueryivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryivARB.xhtml)
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryivARB_t = unsafe extern "system" fn(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);

/// [glGetQueryivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryivEXT.xhtml)
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryivEXT_t = unsafe extern "system" fn(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);

/// [glGetRenderbufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetRenderbufferParameteriv.xhtml)
/// * `target` group: RenderbufferTarget
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetRenderbufferParameteriv_t = unsafe extern "system" fn(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);

/// [glGetRenderbufferParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetRenderbufferParameterivEXT.xhtml)
/// * `target` group: RenderbufferTarget
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetRenderbufferParameterivEXT_t = unsafe extern "system" fn(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);

/// [glGetRenderbufferParameterivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetRenderbufferParameterivOES.xhtml)
/// * `target` group: RenderbufferTarget
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetRenderbufferParameterivOES_t = unsafe extern "system" fn(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);

/// [glGetSamplerParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIiv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

/// [glGetSamplerParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIivEXT.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

/// [glGetSamplerParameterIivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIivOES.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

/// [glGetSamplerParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIuiv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);

/// [glGetSamplerParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIuivEXT.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIuivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);

/// [glGetSamplerParameterIuivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIuivOES.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIuivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);

/// [glGetSamplerParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterfv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat);

/// [glGetSamplerParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameteriv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

/// [glGetSemaphoreParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSemaphoreParameterivNV.xhtml)
/// * `pname` group: SemaphoreParameterName
pub type glGetSemaphoreParameterivNV_t = unsafe extern "system" fn(semaphore: GLuint, pname: SemaphoreParameterName, params: *mut GLint);

/// [glGetSemaphoreParameterui64vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSemaphoreParameterui64vEXT.xhtml)
/// * `pname` group: SemaphoreParameterName
pub type glGetSemaphoreParameterui64vEXT_t = unsafe extern "system" fn(semaphore: GLuint, pname: SemaphoreParameterName, params: *mut GLuint64);

/// [glGetSeparableFilter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSeparableFilter.xhtml)
/// * `target` group: SeparableTargetEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: COMPSIZE(target,format,type)
/// * `column` len: COMPSIZE(target,format,type)
/// * `span` len: COMPSIZE(target,format,type)
pub type glGetSeparableFilter_t = unsafe extern "system" fn(target: SeparableTargetEXT, format: PixelFormat, type_: PixelType, row: *mut void, column: *mut void, span: *mut void);

/// [glGetSeparableFilterEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSeparableFilterEXT.xhtml)
/// * `target` group: SeparableTargetEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: COMPSIZE(target,format,type)
/// * `column` len: COMPSIZE(target,format,type)
/// * `span` len: COMPSIZE(target,format,type)
pub type glGetSeparableFilterEXT_t = unsafe extern "system" fn(target: SeparableTargetEXT, format: PixelFormat, type_: PixelType, row: *mut void, column: *mut void, span: *mut void);

/// [glGetShaderInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderInfoLog.xhtml)
/// * `shader` class: shader
/// * `infoLog` len: bufSize
pub type glGetShaderInfoLog_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

/// [glGetShaderPrecisionFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderPrecisionFormat.xhtml)
/// * `shadertype` group: ShaderType
/// * `precisiontype` group: PrecisionType
pub type glGetShaderPrecisionFormat_t = unsafe extern "system" fn(shadertype: ShaderType, precisiontype: PrecisionType, range: *mut [GLint; 2], precision: *mut GLint);

/// [glGetShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderSource.xhtml)
/// * `shader` class: shader
/// * `source` len: bufSize
pub type glGetShaderSource_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);

/// [glGetShaderSourceARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderSourceARB.xhtml)
/// * `obj` group: handleARB
/// * `source` len: maxLength
pub type glGetShaderSourceARB_t = unsafe extern "system" fn(obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, source: *mut GLcharARB);

/// [glGetShaderiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderiv.xhtml)
/// * `shader` class: shader
/// * `pname` group: ShaderParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetShaderiv_t = unsafe extern "system" fn(shader: GLuint, pname: ShaderParameterName, params: *mut GLint);

/// [glGetShadingRateImagePaletteNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShadingRateImagePaletteNV.xhtml)
pub type glGetShadingRateImagePaletteNV_t = unsafe extern "system" fn(viewport: GLuint, entry: GLuint, rate: *mut GLenum);

/// [glGetShadingRateSampleLocationivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShadingRateSampleLocationivNV.xhtml)
pub type glGetShadingRateSampleLocationivNV_t = unsafe extern "system" fn(rate: GLenum, samples: GLuint, index: GLuint, location: *mut [GLint; 3]);

/// [glGetSharpenTexFuncSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSharpenTexFuncSGIS.xhtml)
/// * `target` group: TextureTarget
/// * `points` len: COMPSIZE(target)
pub type glGetSharpenTexFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, points: *mut GLfloat);

/// [glGetStageIndexNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetStageIndexNV.xhtml)
/// * `shadertype` group: ShaderType
pub type glGetStageIndexNV_t = unsafe extern "system" fn(shadertype: ShaderType) -> GLushort;

/// [glGetString](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetString.xhtml)
/// * `name` group: StringName
pub type glGetString_t = unsafe extern "system" fn(name: StringName) -> GLubyte;

/// [glGetStringi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetStringi.xhtml)
/// * `name` group: StringName
pub type glGetStringi_t = unsafe extern "system" fn(name: StringName, index: GLuint) -> GLubyte;

/// [glGetSubroutineIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSubroutineIndex.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
pub type glGetSubroutineIndex_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLuint;

/// [glGetSubroutineUniformLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSubroutineUniformLocation.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
pub type glGetSubroutineUniformLocation_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLint;

/// [glGetSynciv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSynciv.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
/// * `pname` group: SyncParameterName
/// * `values` len: count
pub type glGetSynciv_t = unsafe extern "system" fn(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

/// [glGetSyncivAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSyncivAPPLE.xhtml)
/// * `sync` class: sync
/// * `pname` group: SyncParameterName
/// * `values` len: count
pub type glGetSyncivAPPLE_t = unsafe extern "system" fn(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

/// [glGetTexBumpParameterfvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexBumpParameterfvATI.xhtml)
/// * `pname` group: GetTexBumpParameterATI
/// * `param` len: COMPSIZE(pname)
pub type glGetTexBumpParameterfvATI_t = unsafe extern "system" fn(pname: GetTexBumpParameterATI, param: *mut GLfloat);

/// [glGetTexBumpParameterivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexBumpParameterivATI.xhtml)
/// * `pname` group: GetTexBumpParameterATI
/// * `param` len: COMPSIZE(pname)
pub type glGetTexBumpParameterivATI_t = unsafe extern "system" fn(pname: GetTexBumpParameterATI, param: *mut GLint);

/// [glGetTexEnvfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexEnvfv.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexEnvfv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLfloat);

/// [glGetTexEnviv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexEnviv.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexEnviv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLint);

/// [glGetTexEnvxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexEnvxv.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexEnvxv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLfixed);

/// [glGetTexEnvxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexEnvxvOES.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexEnvxvOES_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLfixed);

/// [glGetTexFilterFuncSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexFilterFuncSGIS.xhtml)
/// * `target` group: TextureTarget
/// * `filter` group: TextureFilterSGIS
/// * `weights` len: COMPSIZE(target,filter)
pub type glGetTexFilterFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, filter: TextureFilterSGIS, weights: *mut GLfloat);

/// [glGetTexGendv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexGendv.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGendv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLdouble);

/// [glGetTexGenfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexGenfv.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGenfv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLfloat);

/// [glGetTexGenfvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexGenfvOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGenfvOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLfloat);

/// [glGetTexGeniv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexGeniv.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGeniv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLint);

/// [glGetTexGenivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexGenivOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGenivOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLint);

/// [glGetTexGenxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexGenxvOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGenxvOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLfixed);

/// [glGetTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexImage.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(target,level,format,type)
pub type glGetTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);

/// [glGetTexLevelParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexLevelParameterfv.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexLevelParameterfv_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

/// [glGetTexLevelParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexLevelParameteriv.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexLevelParameteriv_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTexLevelParameterxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexLevelParameterxvOES.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexLevelParameterxvOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfixed);

/// [glGetTexParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIiv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIiv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTexParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIivEXT.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIivEXT_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTexParameterIivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIivOES.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIivOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTexParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIuiv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIuiv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// [glGetTexParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIuivEXT.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIuivEXT_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// [glGetTexParameterIuivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIuivOES.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIuivOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// [glGetTexParameterPointervAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterPointervAPPLE.xhtml)
pub type glGetTexParameterPointervAPPLE_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut void);

/// [glGetTexParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterfv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterfv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);

/// [glGetTexParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameteriv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameteriv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTexParameterxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterxv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterxv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfixed);

/// [glGetTexParameterxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterxvOES.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterxvOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfixed);

/// [glGetTextureHandleARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureHandleARB.xhtml)
/// * `texture` class: texture
pub type glGetTextureHandleARB_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

/// [glGetTextureHandleIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureHandleIMG.xhtml)
/// * `texture` class: texture
pub type glGetTextureHandleIMG_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

/// [glGetTextureHandleNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureHandleNV.xhtml)
/// * `texture` class: texture
pub type glGetTextureHandleNV_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

/// [glGetTextureImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureImage.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glGetTextureImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

/// [glGetTextureImageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureImageEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(target,level,format,type)
pub type glGetTextureImageEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);

/// [glGetTextureLevelParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureLevelParameterfv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureLevelParameterfv_t = unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

/// [glGetTextureLevelParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureLevelParameterfvEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureLevelParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

/// [glGetTextureLevelParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureLevelParameteriv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureLevelParameteriv_t = unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTextureLevelParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureLevelParameterivEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureLevelParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTextureParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterIiv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureParameterIiv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTextureParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterIivEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureParameterIivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTextureParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterIuiv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureParameterIuiv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLuint);

/// [glGetTextureParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterIuivEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureParameterIuivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// [glGetTextureParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterfv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureParameterfv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLfloat);

/// [glGetTextureParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterfvEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);

/// [glGetTextureParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameteriv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureParameteriv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTextureParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterivEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// [glGetTextureSamplerHandleARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureSamplerHandleARB.xhtml)
/// * `texture` class: texture
/// * `sampler` class: sampler
pub type glGetTextureSamplerHandleARB_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

/// [glGetTextureSamplerHandleIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureSamplerHandleIMG.xhtml)
/// * `texture` class: texture
/// * `sampler` class: sampler
pub type glGetTextureSamplerHandleIMG_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

/// [glGetTextureSamplerHandleNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureSamplerHandleNV.xhtml)
/// * `texture` class: texture
/// * `sampler` class: sampler
pub type glGetTextureSamplerHandleNV_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

/// [glGetTextureSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureSubImage.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glGetTextureSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

/// [glGetTrackMatrixivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTrackMatrixivNV.xhtml)
/// * `target` group: VertexAttribEnumNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetTrackMatrixivNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, address: GLuint, pname: VertexAttribEnumNV, params: *mut GLint);

/// [glGetTransformFeedbackVarying](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbackVarying.xhtml)
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
pub type glGetTransformFeedbackVarying_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar);

/// [glGetTransformFeedbackVaryingEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbackVaryingEXT.xhtml)
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
pub type glGetTransformFeedbackVaryingEXT_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar);

/// [glGetTransformFeedbackVaryingNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbackVaryingNV.xhtml)
/// * `program` class: program
pub type glGetTransformFeedbackVaryingNV_t = unsafe extern "system" fn(program: GLuint, index: GLuint, location: *mut GLint);

/// [glGetTransformFeedbacki64_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbacki64_v.xhtml)
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
pub type glGetTransformFeedbacki64_v_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint64);

/// [glGetTransformFeedbacki_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbacki_v.xhtml)
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
pub type glGetTransformFeedbacki_v_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint);

/// [glGetTransformFeedbackiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbackiv.xhtml)
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
pub type glGetTransformFeedbackiv_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, param: *mut GLint);

/// [glGetTranslatedShaderSourceANGLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTranslatedShaderSourceANGLE.xhtml)
/// * `shader` class: shader
pub type glGetTranslatedShaderSourceANGLE_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);

/// [glGetUniformBlockIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformBlockIndex.xhtml)
/// * `program` class: program
/// * `uniformBlockName` len: COMPSIZE()
pub type glGetUniformBlockIndex_t = unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;

/// [glGetUniformBufferSizeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformBufferSizeEXT.xhtml)
/// * `program` class: program
pub type glGetUniformBufferSizeEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint) -> GLint;

/// [glGetUniformIndices](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformIndices.xhtml)
/// * `program` class: program
/// * `uniformNames` len: COMPSIZE(uniformCount)
/// * `uniformIndices` len: COMPSIZE(uniformCount)
pub type glGetUniformIndices_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);

/// [glGetUniformLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformLocation.xhtml)
/// * `program` class: program
pub type glGetUniformLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// [glGetUniformLocationARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformLocationARB.xhtml)
/// * `programObj` group: handleARB
pub type glGetUniformLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, name: *const GLcharARB) -> GLint;

/// [glGetUniformOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformOffsetEXT.xhtml)
/// * `program` class: program
pub type glGetUniformOffsetEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint) -> GLintptr;

/// [glGetUniformSubroutineuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformSubroutineuiv.xhtml)
/// * `shadertype` group: ShaderType
pub type glGetUniformSubroutineuiv_t = unsafe extern "system" fn(shadertype: ShaderType, location: GLint, params: *mut GLuint);

/// [glGetUniformdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformdv.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformdv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLdouble);

/// [glGetUniformfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformfv.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);

/// [glGetUniformfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformfvARB.xhtml)
/// * `programObj` group: handleARB
/// * `params` len: COMPSIZE(programObj,location)
pub type glGetUniformfvARB_t = unsafe extern "system" fn(programObj: GLhandleARB, location: GLint, params: *mut GLfloat);

/// [glGetUniformi64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformi64vARB.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformi64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint64);

/// [glGetUniformi64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformi64vNV.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformi64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint64EXT);

/// [glGetUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformiv.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);

/// [glGetUniformivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformivARB.xhtml)
/// * `programObj` group: handleARB
/// * `params` len: COMPSIZE(programObj,location)
pub type glGetUniformivARB_t = unsafe extern "system" fn(programObj: GLhandleARB, location: GLint, params: *mut GLint);

/// [glGetUniformui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformui64vARB.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint64);

/// [glGetUniformui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformui64vNV.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint64EXT);

/// [glGetUniformuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformuiv.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);

/// [glGetUniformuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformuivEXT.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformuivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);

/// [glGetUnsignedBytei_vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUnsignedBytei_vEXT.xhtml)
/// * `data` len: COMPSIZE(target)
pub type glGetUnsignedBytei_vEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLubyte);

/// [glGetUnsignedBytevEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUnsignedBytevEXT.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetUnsignedBytevEXT_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLubyte);

/// [glGetVariantArrayObjectfvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVariantArrayObjectfvATI.xhtml)
/// * `pname` group: ArrayObjectPNameATI
pub type glGetVariantArrayObjectfvATI_t = unsafe extern "system" fn(id: GLuint, pname: ArrayObjectPNameATI, params: *mut GLfloat);

/// [glGetVariantArrayObjectivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVariantArrayObjectivATI.xhtml)
/// * `pname` group: ArrayObjectPNameATI
pub type glGetVariantArrayObjectivATI_t = unsafe extern "system" fn(id: GLuint, pname: ArrayObjectPNameATI, params: *mut GLint);

/// [glGetVariantBooleanvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVariantBooleanvEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(id)
pub type glGetVariantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLboolean);

/// [glGetVariantFloatvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVariantFloatvEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetVariantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLfloat);

/// [glGetVariantIntegervEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVariantIntegervEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetVariantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLint);

/// [glGetVariantPointervEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVariantPointervEXT.xhtml)
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetVariantPointervEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut *mut void);

/// [glGetVaryingLocationNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVaryingLocationNV.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glGetVaryingLocationNV_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// [glGetVertexArrayIndexed64iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayIndexed64iv.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayIndexed64iv_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint64);

/// [glGetVertexArrayIndexediv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayIndexediv.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayIndexediv_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint);

/// [glGetVertexArrayIntegeri_vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayIntegeri_vEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayIntegeri_vEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint);

/// [glGetVertexArrayIntegervEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayIntegervEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayIntegervEXT_t = unsafe extern "system" fn(vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint);

/// [glGetVertexArrayPointeri_vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayPointeri_vEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayPointeri_vEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut *mut void);

/// [glGetVertexArrayPointervEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayPointervEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayPointervEXT_t = unsafe extern "system" fn(vaobj: GLuint, pname: VertexArrayPName, param: *mut *mut void);

/// [glGetVertexArrayiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayiv.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayiv_t = unsafe extern "system" fn(vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint);

/// [glGetVertexAttribArrayObjectfvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribArrayObjectfvATI.xhtml)
/// * `pname` group: ArrayObjectPNameATI
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribArrayObjectfvATI_t = unsafe extern "system" fn(index: GLuint, pname: ArrayObjectPNameATI, params: *mut GLfloat);

/// [glGetVertexAttribArrayObjectivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribArrayObjectivATI.xhtml)
/// * `pname` group: ArrayObjectPNameATI
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribArrayObjectivATI_t = unsafe extern "system" fn(index: GLuint, pname: ArrayObjectPNameATI, params: *mut GLint);

/// [glGetVertexAttribIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribIiv.xhtml)
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribIiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLint);

/// [glGetVertexAttribIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribIivEXT.xhtml)
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribIivEXT_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLint);

/// [glGetVertexAttribIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribIuiv.xhtml)
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribIuiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint);

/// [glGetVertexAttribIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribIuivEXT.xhtml)
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribIuivEXT_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint);

/// [glGetVertexAttribLdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribLdv.xhtml)
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribLdv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLdouble);

/// [glGetVertexAttribLdvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribLdvEXT.xhtml)
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribLdvEXT_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLdouble);

/// [glGetVertexAttribLi64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribLi64vNV.xhtml)
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribLi64vNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLint64EXT);

/// [glGetVertexAttribLui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribLui64vARB.xhtml)
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribLui64vARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint64EXT);

/// [glGetVertexAttribLui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribLui64vNV.xhtml)
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribLui64vNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint64EXT);

/// [glGetVertexAttribPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribPointerv.xhtml)
/// * `pname` group: VertexAttribPointerPropertyARB
pub type glGetVertexAttribPointerv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void);

/// [glGetVertexAttribPointervARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribPointervARB.xhtml)
/// * `pname` group: VertexAttribPointerPropertyARB
pub type glGetVertexAttribPointervARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void);

/// [glGetVertexAttribPointervNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribPointervNV.xhtml)
/// * `pname` group: VertexAttribEnumNV
pub type glGetVertexAttribPointervNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnumNV, pointer: *mut *mut void);

/// [glGetVertexAttribdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribdv.xhtml)
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribdv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]);

/// [glGetVertexAttribdvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribdvARB.xhtml)
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribdvARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]);

/// [glGetVertexAttribdvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribdvNV.xhtml)
/// * `pname` group: VertexAttribEnumNV
pub type glGetVertexAttribdvNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnumNV, params: *mut GLdouble);

/// [glGetVertexAttribfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribfv.xhtml)
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribfv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]);

/// [glGetVertexAttribfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribfvARB.xhtml)
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribfvARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]);

/// [glGetVertexAttribfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribfvNV.xhtml)
/// * `pname` group: VertexAttribEnumNV
pub type glGetVertexAttribfvNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnumNV, params: *mut GLfloat);

/// [glGetVertexAttribiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribiv.xhtml)
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]);

/// [glGetVertexAttribivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribivARB.xhtml)
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribivARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]);

/// [glGetVertexAttribivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribivNV.xhtml)
/// * `pname` group: VertexAttribEnumNV
pub type glGetVertexAttribivNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnumNV, params: *mut GLint);

/// [glGetVideoCaptureStreamdvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVideoCaptureStreamdvNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoCaptureStreamdvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLdouble);

/// [glGetVideoCaptureStreamfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVideoCaptureStreamfvNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoCaptureStreamfvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLfloat);

/// [glGetVideoCaptureStreamivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVideoCaptureStreamivNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoCaptureStreamivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLint);

/// [glGetVideoCaptureivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVideoCaptureivNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoCaptureivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, pname: GLenum, params: *mut GLint);

/// [glGetVideoi64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVideoi64vNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoi64vNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLint64EXT);

/// [glGetVideoivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVideoivNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoivNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLint);

/// [glGetVideoui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVideoui64vNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoui64vNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLuint64EXT);

/// [glGetVideouivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVideouivNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glGetVideouivNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLuint);

/// [glGetVkProcAddrNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVkProcAddrNV.xhtml)
/// * `name` len: COMPSIZE(name)
pub type glGetVkProcAddrNV_t = unsafe extern "system" fn(name: *const GLchar) -> GLVULKANPROCNV;

/// [glGetnColorTable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnColorTable.xhtml)
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: bufSize
pub type glGetnColorTable_t = unsafe extern "system" fn(target: ColorTableTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, table: *mut void);

/// [glGetnColorTableARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnColorTableARB.xhtml)
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: bufSize
pub type glGetnColorTableARB_t = unsafe extern "system" fn(target: ColorTableTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, table: *mut void);

/// [glGetnCompressedTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnCompressedTexImage.xhtml)
/// * `target` group: TextureTarget
/// * `pixels` len: bufSize
pub type glGetnCompressedTexImage_t = unsafe extern "system" fn(target: TextureTarget, lod: GLint, bufSize: GLsizei, pixels: *mut void);

/// [glGetnCompressedTexImageARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnCompressedTexImageARB.xhtml)
/// * `target` group: TextureTarget
/// * `img` len: bufSize
pub type glGetnCompressedTexImageARB_t = unsafe extern "system" fn(target: TextureTarget, lod: GLint, bufSize: GLsizei, img: *mut void);

/// [glGetnConvolutionFilter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnConvolutionFilter.xhtml)
/// * `target` group: ConvolutionTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: bufSize
pub type glGetnConvolutionFilter_t = unsafe extern "system" fn(target: ConvolutionTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, image: *mut void);

/// [glGetnConvolutionFilterARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnConvolutionFilterARB.xhtml)
/// * `target` group: ConvolutionTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: bufSize
pub type glGetnConvolutionFilterARB_t = unsafe extern "system" fn(target: ConvolutionTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, image: *mut void);

/// [glGetnHistogram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnHistogram.xhtml)
/// * `target` group: HistogramTarget
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
pub type glGetnHistogram_t = unsafe extern "system" fn(target: HistogramTarget, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void);

/// [glGetnHistogramARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnHistogramARB.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
pub type glGetnHistogramARB_t = unsafe extern "system" fn(target: HistogramTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void);

/// [glGetnMapdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapdv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: COMPSIZE(bufSize)
pub type glGetnMapdv_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLdouble);

/// [glGetnMapdvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapdvARB.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: bufSize / 8
pub type glGetnMapdvARB_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLdouble);

/// [glGetnMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapfv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
pub type glGetnMapfv_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLfloat);

/// [glGetnMapfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapfvARB.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: bufSize
pub type glGetnMapfvARB_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLfloat);

/// [glGetnMapiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapiv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
pub type glGetnMapiv_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLint);

/// [glGetnMapivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapivARB.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: bufSize
pub type glGetnMapivARB_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLint);

/// [glGetnMinmax](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMinmax.xhtml)
/// * `target` group: MinmaxTarget
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
pub type glGetnMinmax_t = unsafe extern "system" fn(target: MinmaxTarget, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void);

/// [glGetnMinmaxARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMinmaxARB.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
pub type glGetnMinmaxARB_t = unsafe extern "system" fn(target: MinmaxTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void);

/// [glGetnPixelMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapfv.xhtml)
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(bufSize)
pub type glGetnPixelMapfv_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLfloat);

/// [glGetnPixelMapfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapfvARB.xhtml)
/// * `map` group: PixelMap
/// * `values` len: bufSize / 4
pub type glGetnPixelMapfvARB_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLfloat);

/// [glGetnPixelMapuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapuiv.xhtml)
/// * `map` group: PixelMap
pub type glGetnPixelMapuiv_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLuint);

/// [glGetnPixelMapuivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapuivARB.xhtml)
/// * `map` group: PixelMap
/// * `values` len: bufSize
pub type glGetnPixelMapuivARB_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLuint);

/// [glGetnPixelMapusv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapusv.xhtml)
/// * `map` group: PixelMap
pub type glGetnPixelMapusv_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLushort);

/// [glGetnPixelMapusvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapusvARB.xhtml)
/// * `map` group: PixelMap
/// * `values` len: bufSize
pub type glGetnPixelMapusvARB_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLushort);

/// [glGetnPolygonStipple](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPolygonStipple.xhtml)
/// * `pattern` len: bufSize
pub type glGetnPolygonStipple_t = unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);

/// [glGetnPolygonStippleARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPolygonStippleARB.xhtml)
/// * `pattern` len: bufSize
pub type glGetnPolygonStippleARB_t = unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);

/// [glGetnSeparableFilter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnSeparableFilter.xhtml)
/// * `target` group: SeparableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: rowBufSize
/// * `column` len: columnBufSize
pub type glGetnSeparableFilter_t = unsafe extern "system" fn(target: SeparableTarget, format: PixelFormat, type_: PixelType, rowBufSize: GLsizei, row: *mut void, columnBufSize: GLsizei, column: *mut void, span: *mut void);

/// [glGetnSeparableFilterARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnSeparableFilterARB.xhtml)
/// * `target` group: SeparableTargetEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: rowBufSize
/// * `column` len: columnBufSize
pub type glGetnSeparableFilterARB_t = unsafe extern "system" fn(target: SeparableTargetEXT, format: PixelFormat, type_: PixelType, rowBufSize: GLsizei, row: *mut void, columnBufSize: GLsizei, column: *mut void, span: *mut void);

/// [glGetnTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnTexImage.xhtml)
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: bufSize
pub type glGetnTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

/// [glGetnTexImageARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnTexImageARB.xhtml)
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `img` len: bufSize
pub type glGetnTexImageARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, img: *mut void);

/// [glGetnUniformdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformdv.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 8
pub type glGetnUniformdv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);

/// [glGetnUniformdvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformdvARB.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 8
pub type glGetnUniformdvARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);

/// [glGetnUniformfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformfv.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

/// [glGetnUniformfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformfvARB.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformfvARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

/// [glGetnUniformfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformfvEXT.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformfvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

/// [glGetnUniformfvKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformfvKHR.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformfvKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

/// [glGetnUniformi64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformi64vARB.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 8
pub type glGetnUniformi64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint64);

/// [glGetnUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformiv.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

/// [glGetnUniformivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformivARB.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformivARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

/// [glGetnUniformivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformivEXT.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

/// [glGetnUniformivKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformivKHR.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformivKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

/// [glGetnUniformui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformui64vARB.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 8
pub type glGetnUniformui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint64);

/// [glGetnUniformuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformuiv.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

/// [glGetnUniformuivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformuivARB.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformuivARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

/// [glGetnUniformuivKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformuivKHR.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformuivKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

/// [glGlobalAlphaFactorbSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGlobalAlphaFactorbSUN.xhtml)
pub type glGlobalAlphaFactorbSUN_t = unsafe extern "system" fn(factor: GLbyte);

/// [glGlobalAlphaFactordSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGlobalAlphaFactordSUN.xhtml)
pub type glGlobalAlphaFactordSUN_t = unsafe extern "system" fn(factor: GLdouble);

/// [glGlobalAlphaFactorfSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGlobalAlphaFactorfSUN.xhtml)
pub type glGlobalAlphaFactorfSUN_t = unsafe extern "system" fn(factor: GLfloat);

/// [glGlobalAlphaFactoriSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGlobalAlphaFactoriSUN.xhtml)
pub type glGlobalAlphaFactoriSUN_t = unsafe extern "system" fn(factor: GLint);

/// [glGlobalAlphaFactorsSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGlobalAlphaFactorsSUN.xhtml)
pub type glGlobalAlphaFactorsSUN_t = unsafe extern "system" fn(factor: GLshort);

/// [glGlobalAlphaFactorubSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGlobalAlphaFactorubSUN.xhtml)
pub type glGlobalAlphaFactorubSUN_t = unsafe extern "system" fn(factor: GLubyte);

/// [glGlobalAlphaFactoruiSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGlobalAlphaFactoruiSUN.xhtml)
pub type glGlobalAlphaFactoruiSUN_t = unsafe extern "system" fn(factor: GLuint);

/// [glGlobalAlphaFactorusSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGlobalAlphaFactorusSUN.xhtml)
pub type glGlobalAlphaFactorusSUN_t = unsafe extern "system" fn(factor: GLushort);

/// [glHint](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glHint.xhtml)
/// * `target` group: HintTarget
/// * `mode` group: HintMode
pub type glHint_t = unsafe extern "system" fn(target: HintTarget, mode: HintMode);

/// [glHintPGI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glHintPGI.xhtml)
/// * `target` group: HintTargetPGI
pub type glHintPGI_t = unsafe extern "system" fn(target: HintTargetPGI, mode: GLint);

/// [glHistogram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glHistogram.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `internalformat` group: InternalFormat
/// * `sink` group: Boolean
pub type glHistogram_t = unsafe extern "system" fn(target: HistogramTargetEXT, width: GLsizei, internalformat: InternalFormat, sink: GLboolean);

/// [glHistogramEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glHistogramEXT.xhtml)
/// * `target` group: HistogramTargetEXT
/// * `internalformat` group: InternalFormat
/// * `sink` group: Boolean
pub type glHistogramEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT, width: GLsizei, internalformat: InternalFormat, sink: GLboolean);

/// [glIglooInterfaceSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIglooInterfaceSGIX.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glIglooInterfaceSGIX_t = unsafe extern "system" fn(pname: GLenum, params: *const void);

/// [glImageTransformParameterfHP](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImageTransformParameterfHP.xhtml)
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
pub type glImageTransformParameterfHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, param: GLfloat);

/// [glImageTransformParameterfvHP](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImageTransformParameterfvHP.xhtml)
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
/// * `params` len: COMPSIZE(pname)
pub type glImageTransformParameterfvHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, params: *const GLfloat);

/// [glImageTransformParameteriHP](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImageTransformParameteriHP.xhtml)
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
pub type glImageTransformParameteriHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, param: GLint);

/// [glImageTransformParameterivHP](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImageTransformParameterivHP.xhtml)
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
/// * `params` len: COMPSIZE(pname)
pub type glImageTransformParameterivHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, params: *const GLint);

/// [glImportMemoryFdEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImportMemoryFdEXT.xhtml)
/// * `handleType` group: ExternalHandleType
pub type glImportMemoryFdEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: ExternalHandleType, fd: GLint);

/// [glImportMemoryWin32HandleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImportMemoryWin32HandleEXT.xhtml)
/// * `handleType` group: ExternalHandleType
pub type glImportMemoryWin32HandleEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: ExternalHandleType, handle: *mut void);

/// [glImportMemoryWin32NameEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImportMemoryWin32NameEXT.xhtml)
/// * `handleType` group: ExternalHandleType
pub type glImportMemoryWin32NameEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: ExternalHandleType, name: *const void);

/// [glImportSemaphoreFdEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImportSemaphoreFdEXT.xhtml)
/// * `handleType` group: ExternalHandleType
pub type glImportSemaphoreFdEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: ExternalHandleType, fd: GLint);

/// [glImportSemaphoreWin32HandleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImportSemaphoreWin32HandleEXT.xhtml)
/// * `handleType` group: ExternalHandleType
pub type glImportSemaphoreWin32HandleEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: ExternalHandleType, handle: *mut void);

/// [glImportSemaphoreWin32NameEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImportSemaphoreWin32NameEXT.xhtml)
/// * `handleType` group: ExternalHandleType
pub type glImportSemaphoreWin32NameEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: ExternalHandleType, name: *const void);

/// [glImportSyncEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glImportSyncEXT.xhtml)
pub type glImportSyncEXT_t = unsafe extern "system" fn(external_sync_type: GLenum, external_sync: GLintptr, flags: GLbitfield) -> GLsync;

/// [glIndexFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexFormatNV.xhtml)
pub type glIndexFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

/// [glIndexFuncEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexFuncEXT.xhtml)
/// * `func` group: IndexFunctionEXT
/// * `ref` group: ClampedFloat32
pub type glIndexFuncEXT_t = unsafe extern "system" fn(func: IndexFunctionEXT, ref_: GLclampf);

/// [glIndexMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexMask.xhtml)
/// * `mask` group: MaskedColorIndexValueI
pub type glIndexMask_t = unsafe extern "system" fn(mask: GLuint);

/// [glIndexMaterialEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexMaterialEXT.xhtml)
/// * `face` group: MaterialFace
/// * `mode` group: IndexMaterialParameterEXT
pub type glIndexMaterialEXT_t = unsafe extern "system" fn(face: MaterialFace, mode: IndexMaterialParameterEXT);

/// [glIndexPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexPointer.xhtml)
/// * `type` group: IndexPointerType
/// * `pointer` len: COMPSIZE(type,stride)
pub type glIndexPointer_t = unsafe extern "system" fn(type_: IndexPointerType, stride: GLsizei, pointer: *const void);

/// [glIndexPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexPointerEXT.xhtml)
/// * `type` group: IndexPointerType
/// * `pointer` len: COMPSIZE(type,stride,count)
pub type glIndexPointerEXT_t = unsafe extern "system" fn(type_: IndexPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// [glIndexPointerListIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexPointerListIBM.xhtml)
/// * `type` group: IndexPointerType
/// * `pointer` len: COMPSIZE(type,stride)
pub type glIndexPointerListIBM_t = unsafe extern "system" fn(type_: IndexPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// [glIndexd](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexd.xhtml)
/// * `c` group: ColorIndexValueD
pub type glIndexd_t = unsafe extern "system" fn(c: GLdouble);

/// [glIndexdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexdv.xhtml)
/// * `c` group: ColorIndexValueD
pub type glIndexdv_t = unsafe extern "system" fn(c: *const GLdouble);

/// [glIndexf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexf.xhtml)
/// * `c` group: ColorIndexValueF
pub type glIndexf_t = unsafe extern "system" fn(c: GLfloat);

/// [glIndexfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexfv.xhtml)
/// * `c` group: ColorIndexValueF
pub type glIndexfv_t = unsafe extern "system" fn(c: *const GLfloat);

/// [glIndexi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexi.xhtml)
/// * `c` group: ColorIndexValueI
pub type glIndexi_t = unsafe extern "system" fn(c: GLint);

/// [glIndexiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexiv.xhtml)
/// * `c` group: ColorIndexValueI
pub type glIndexiv_t = unsafe extern "system" fn(c: *const GLint);

/// [glIndexs](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexs.xhtml)
/// * `c` group: ColorIndexValueS
pub type glIndexs_t = unsafe extern "system" fn(c: GLshort);

/// [glIndexsv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexsv.xhtml)
/// * `c` group: ColorIndexValueS
pub type glIndexsv_t = unsafe extern "system" fn(c: *const GLshort);

/// [glIndexub](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexub.xhtml)
/// * `c` group: ColorIndexValueUB
pub type glIndexub_t = unsafe extern "system" fn(c: GLubyte);

/// [glIndexubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexubv.xhtml)
/// * `c` group: ColorIndexValueUB
pub type glIndexubv_t = unsafe extern "system" fn(c: *const GLubyte);

/// [glIndexxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexxOES.xhtml)
pub type glIndexxOES_t = unsafe extern "system" fn(component: GLfixed);

/// [glIndexxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIndexxvOES.xhtml)
pub type glIndexxvOES_t = unsafe extern "system" fn(component: *const GLfixed);

/// [glInitNames](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInitNames.xhtml)
pub type glInitNames_t = unsafe extern "system" fn();

/// [glInsertComponentEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInsertComponentEXT.xhtml)
pub type glInsertComponentEXT_t = unsafe extern "system" fn(res: GLuint, src: GLuint, num: GLuint);

/// [glInsertEventMarkerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInsertEventMarkerEXT.xhtml)
pub type glInsertEventMarkerEXT_t = unsafe extern "system" fn(length: GLsizei, marker: *const GLchar);

/// [glInstrumentsBufferSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInstrumentsBufferSGIX.xhtml)
/// * `buffer` len: size
pub type glInstrumentsBufferSGIX_t = unsafe extern "system" fn(size: GLsizei, buffer: *mut GLint);

/// [glInterleavedArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInterleavedArrays.xhtml)
/// * `format` group: InterleavedArrayFormat
/// * `pointer` len: COMPSIZE(format,stride)
pub type glInterleavedArrays_t = unsafe extern "system" fn(format: InterleavedArrayFormat, stride: GLsizei, pointer: *const void);

/// [glInterpolatePathsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInterpolatePathsNV.xhtml)
/// * `resultPath` group: Path
/// * `pathA` group: Path
/// * `pathB` group: Path
pub type glInterpolatePathsNV_t = unsafe extern "system" fn(resultPath: GLuint, pathA: GLuint, pathB: GLuint, weight: GLfloat);

/// [glInvalidateBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateBufferData.xhtml)
/// * `buffer` class: buffer
pub type glInvalidateBufferData_t = unsafe extern "system" fn(buffer: GLuint);

/// [glInvalidateBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateBufferSubData.xhtml)
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
pub type glInvalidateBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

/// [glInvalidateFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateFramebuffer.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachments` group: InvalidateFramebufferAttachment
/// * `attachments` len: numAttachments
pub type glInvalidateFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment);

/// [glInvalidateNamedFramebufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateNamedFramebufferData.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachments` group: FramebufferAttachment
pub type glInvalidateNamedFramebufferData_t = unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment);

/// [glInvalidateNamedFramebufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateNamedFramebufferSubData.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachments` group: FramebufferAttachment
pub type glInvalidateNamedFramebufferSubData_t = unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glInvalidateSubFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateSubFramebuffer.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachments` group: InvalidateFramebufferAttachment
/// * `attachments` len: numAttachments
pub type glInvalidateSubFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glInvalidateTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateTexImage.xhtml)
/// * `texture` class: texture
pub type glInvalidateTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint);

/// [glInvalidateTexSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateTexSubImage.xhtml)
/// * `texture` class: texture
pub type glInvalidateTexSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

/// [glIsAsyncMarkerSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsAsyncMarkerSGIX.xhtml)
pub type glIsAsyncMarkerSGIX_t = unsafe extern "system" fn(marker: GLuint) -> GLboolean;

/// [glIsBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsBuffer.xhtml)
/// * `buffer` class: buffer
pub type glIsBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// [glIsBufferARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsBufferARB.xhtml)
/// * `buffer` class: buffer
pub type glIsBufferARB_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// [glIsBufferResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsBufferResidentNV.xhtml)
pub type glIsBufferResidentNV_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;

/// [glIsCommandListNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsCommandListNV.xhtml)
pub type glIsCommandListNV_t = unsafe extern "system" fn(list: GLuint) -> GLboolean;

/// [glIsEnabled](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnabled.xhtml)
/// * `cap` group: EnableCap
pub type glIsEnabled_t = unsafe extern "system" fn(cap: EnableCap) -> GLboolean;

/// [glIsEnabledIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnabledIndexedEXT.xhtml)
/// * `target` group: EnableCap
pub type glIsEnabledIndexedEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// [glIsEnabledi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnabledi.xhtml)
/// * `target` group: EnableCap
pub type glIsEnabledi_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// [glIsEnablediEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnablediEXT.xhtml)
/// * `target` group: EnableCap
pub type glIsEnablediEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// [glIsEnablediNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnablediNV.xhtml)
/// * `target` group: EnableCap
pub type glIsEnablediNV_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// [glIsEnablediOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnablediOES.xhtml)
/// * `target` group: EnableCap
pub type glIsEnablediOES_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// [glIsFenceAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsFenceAPPLE.xhtml)
/// * `fence` group: FenceNV
pub type glIsFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

/// [glIsFenceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsFenceNV.xhtml)
/// * `fence` group: FenceNV
pub type glIsFenceNV_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

/// [glIsFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsFramebuffer.xhtml)
/// * `framebuffer` class: framebuffer
pub type glIsFramebuffer_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

/// [glIsFramebufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsFramebufferEXT.xhtml)
/// * `framebuffer` class: framebuffer
pub type glIsFramebufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

/// [glIsFramebufferOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsFramebufferOES.xhtml)
/// * `framebuffer` class: framebuffer
pub type glIsFramebufferOES_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

/// [glIsImageHandleResidentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsImageHandleResidentARB.xhtml)
pub type glIsImageHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

/// [glIsImageHandleResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsImageHandleResidentNV.xhtml)
pub type glIsImageHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

/// [glIsList](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsList.xhtml)
/// * `list` group: List
/// * `list` class: display list
pub type glIsList_t = unsafe extern "system" fn(list: GLuint) -> GLboolean;

/// [glIsMemoryObjectEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsMemoryObjectEXT.xhtml)
pub type glIsMemoryObjectEXT_t = unsafe extern "system" fn(memoryObject: GLuint) -> GLboolean;

/// [glIsNameAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsNameAMD.xhtml)
pub type glIsNameAMD_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint) -> GLboolean;

/// [glIsNamedBufferResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsNamedBufferResidentNV.xhtml)
/// * `buffer` class: buffer
pub type glIsNamedBufferResidentNV_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// [glIsNamedStringARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsNamedStringARB.xhtml)
/// * `name` len: namelen
pub type glIsNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar) -> GLboolean;

/// [glIsObjectBufferATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsObjectBufferATI.xhtml)
/// * `buffer` class: buffer
pub type glIsObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// [glIsOcclusionQueryNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsOcclusionQueryNV.xhtml)
pub type glIsOcclusionQueryNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// [glIsPathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsPathNV.xhtml)
/// * `path` group: Path
pub type glIsPathNV_t = unsafe extern "system" fn(path: GLuint) -> GLboolean;

/// [glIsPointInFillPathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsPointInFillPathNV.xhtml)
/// * `path` group: Path
/// * `mask` group: MaskedStencilValue
pub type glIsPointInFillPathNV_t = unsafe extern "system" fn(path: GLuint, mask: GLuint, x: GLfloat, y: GLfloat) -> GLboolean;

/// [glIsPointInStrokePathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsPointInStrokePathNV.xhtml)
/// * `path` group: Path
pub type glIsPointInStrokePathNV_t = unsafe extern "system" fn(path: GLuint, x: GLfloat, y: GLfloat) -> GLboolean;

/// [glIsProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgram.xhtml)
/// * `program` class: program
pub type glIsProgram_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

/// [glIsProgramARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgramARB.xhtml)
/// * `program` class: program
pub type glIsProgramARB_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

/// [glIsProgramNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgramNV.xhtml)
/// * `id` class: program
pub type glIsProgramNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// [glIsProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgramPipeline.xhtml)
/// * `pipeline` class: program pipeline
pub type glIsProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;

/// [glIsProgramPipelineEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgramPipelineEXT.xhtml)
/// * `pipeline` class: program pipeline
pub type glIsProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;

/// [glIsQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsQuery.xhtml)
/// * `id` class: query
pub type glIsQuery_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// [glIsQueryARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsQueryARB.xhtml)
/// * `id` class: query
pub type glIsQueryARB_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// [glIsQueryEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsQueryEXT.xhtml)
/// * `id` class: query
pub type glIsQueryEXT_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// [glIsRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsRenderbuffer.xhtml)
/// * `renderbuffer` class: renderbuffer
pub type glIsRenderbuffer_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

/// [glIsRenderbufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsRenderbufferEXT.xhtml)
/// * `renderbuffer` class: renderbuffer
pub type glIsRenderbufferEXT_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

/// [glIsRenderbufferOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsRenderbufferOES.xhtml)
/// * `renderbuffer` class: renderbuffer
pub type glIsRenderbufferOES_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

/// [glIsSampler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsSampler.xhtml)
/// * `sampler` class: sampler
pub type glIsSampler_t = unsafe extern "system" fn(sampler: GLuint) -> GLboolean;

/// [glIsSemaphoreEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsSemaphoreEXT.xhtml)
pub type glIsSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint) -> GLboolean;

/// [glIsShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsShader.xhtml)
/// * `shader` class: shader
pub type glIsShader_t = unsafe extern "system" fn(shader: GLuint) -> GLboolean;

/// [glIsStateNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsStateNV.xhtml)
pub type glIsStateNV_t = unsafe extern "system" fn(state: GLuint) -> GLboolean;

/// [glIsSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsSync.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
pub type glIsSync_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;

/// [glIsSyncAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsSyncAPPLE.xhtml)
/// * `sync` class: sync
pub type glIsSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;

/// [glIsTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTexture.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glIsTexture_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

/// [glIsTextureEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTextureEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glIsTextureEXT_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

/// [glIsTextureHandleResidentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTextureHandleResidentARB.xhtml)
pub type glIsTextureHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

/// [glIsTextureHandleResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTextureHandleResidentNV.xhtml)
pub type glIsTextureHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

/// [glIsTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTransformFeedback.xhtml)
/// * `id` class: transform feedback
pub type glIsTransformFeedback_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// [glIsTransformFeedbackNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTransformFeedbackNV.xhtml)
/// * `id` class: transform feedback
pub type glIsTransformFeedbackNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// [glIsVariantEnabledEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsVariantEnabledEXT.xhtml)
/// * `cap` group: VariantCapEXT
pub type glIsVariantEnabledEXT_t = unsafe extern "system" fn(id: GLuint, cap: VariantCapEXT) -> GLboolean;

/// [glIsVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsVertexArray.xhtml)
/// * `array` class: vertex array
pub type glIsVertexArray_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

/// [glIsVertexArrayAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsVertexArrayAPPLE.xhtml)
/// * `array` class: vertex array
pub type glIsVertexArrayAPPLE_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

/// [glIsVertexArrayOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsVertexArrayOES.xhtml)
/// * `array` class: vertex array
pub type glIsVertexArrayOES_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

/// [glIsVertexAttribEnabledAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsVertexAttribEnabledAPPLE.xhtml)
pub type glIsVertexAttribEnabledAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum) -> GLboolean;

/// [glLGPUCopyImageSubDataNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLGPUCopyImageSubDataNVX.xhtml)
pub type glLGPUCopyImageSubDataNVX_t = unsafe extern "system" fn(sourceGpu: GLuint, destinationGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srxY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

/// [glLGPUInterlockNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLGPUInterlockNVX.xhtml)
pub type glLGPUInterlockNVX_t = unsafe extern "system" fn();

/// [glLGPUNamedBufferSubDataNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLGPUNamedBufferSubDataNVX.xhtml)
/// * `buffer` class: buffer
pub type glLGPUNamedBufferSubDataNVX_t = unsafe extern "system" fn(gpuMask: GLbitfield, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// [glLabelObjectEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLabelObjectEXT.xhtml)
pub type glLabelObjectEXT_t = unsafe extern "system" fn(type_: GLenum, object: GLuint, length: GLsizei, label: *const GLchar);

/// [glLightEnviSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightEnviSGIX.xhtml)
/// * `pname` group: LightEnvParameterSGIX
/// * `param` group: CheckedInt32
pub type glLightEnviSGIX_t = unsafe extern "system" fn(pname: LightEnvParameterSGIX, param: GLint);

/// [glLightModelf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightModelf.xhtml)
/// * `pname` group: LightModelParameter
pub type glLightModelf_t = unsafe extern "system" fn(pname: LightModelParameter, param: GLfloat);

/// [glLightModelfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightModelfv.xhtml)
/// * `pname` group: LightModelParameter
/// * `params` len: COMPSIZE(pname)
pub type glLightModelfv_t = unsafe extern "system" fn(pname: LightModelParameter, params: *const GLfloat);

/// [glLightModeli](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightModeli.xhtml)
/// * `pname` group: LightModelParameter
pub type glLightModeli_t = unsafe extern "system" fn(pname: LightModelParameter, param: GLint);

/// [glLightModeliv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightModeliv.xhtml)
/// * `pname` group: LightModelParameter
/// * `params` len: COMPSIZE(pname)
pub type glLightModeliv_t = unsafe extern "system" fn(pname: LightModelParameter, params: *const GLint);

/// [glLightModelx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightModelx.xhtml)
/// * `pname` group: LightModelParameter
pub type glLightModelx_t = unsafe extern "system" fn(pname: LightModelParameter, param: GLfixed);

/// [glLightModelxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightModelxOES.xhtml)
/// * `pname` group: LightModelParameter
pub type glLightModelxOES_t = unsafe extern "system" fn(pname: LightModelParameter, param: GLfixed);

/// [glLightModelxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightModelxv.xhtml)
/// * `pname` group: LightModelParameter
/// * `param` len: COMPSIZE(pname)
pub type glLightModelxv_t = unsafe extern "system" fn(pname: LightModelParameter, param: *const GLfixed);

/// [glLightModelxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightModelxvOES.xhtml)
/// * `pname` group: LightModelParameter
/// * `param` len: COMPSIZE(pname)
pub type glLightModelxvOES_t = unsafe extern "system" fn(pname: LightModelParameter, param: *const GLfixed);

/// [glLightf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightf.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `param` group: CheckedFloat32
pub type glLightf_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, param: GLfloat);

/// [glLightfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightfv.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glLightfv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *const GLfloat);

/// [glLighti](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLighti.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `param` group: CheckedInt32
pub type glLighti_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, param: GLint);

/// [glLightiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightiv.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glLightiv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *const GLint);

/// [glLightx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightx.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
pub type glLightx_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, param: GLfixed);

/// [glLightxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightxOES.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
pub type glLightxOES_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, param: GLfixed);

/// [glLightxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightxv.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glLightxv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *const GLfixed);

/// [glLightxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLightxvOES.xhtml)
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glLightxvOES_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *const GLfixed);

/// [glLineStipple](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLineStipple.xhtml)
/// * `factor` group: CheckedInt32
/// * `pattern` group: LineStipple
pub type glLineStipple_t = unsafe extern "system" fn(factor: GLint, pattern: GLushort);

/// [glLineWidth](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLineWidth.xhtml)
/// * `width` group: CheckedFloat32
pub type glLineWidth_t = unsafe extern "system" fn(width: GLfloat);

/// [glLineWidthx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLineWidthx.xhtml)
pub type glLineWidthx_t = unsafe extern "system" fn(width: GLfixed);

/// [glLineWidthxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLineWidthxOES.xhtml)
pub type glLineWidthxOES_t = unsafe extern "system" fn(width: GLfixed);

/// [glLinkProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml)
/// * `program` class: program
pub type glLinkProgram_t = unsafe extern "system" fn(program: GLuint);

/// [glLinkProgramARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgramARB.xhtml)
/// * `programObj` group: handleARB
pub type glLinkProgramARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

/// [glListBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glListBase.xhtml)
/// * `base` group: List
pub type glListBase_t = unsafe extern "system" fn(base: GLuint);

/// [glListDrawCommandsStatesClientNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glListDrawCommandsStatesClientNV.xhtml)
/// * `indirects` len: count
/// * `sizes` len: count
/// * `states` len: count
/// * `fbos` len: count
pub type glListDrawCommandsStatesClientNV_t = unsafe extern "system" fn(list: GLuint, segment: GLuint, indirects: *const *mut void, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

/// [glListParameterfSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glListParameterfSGIX.xhtml)
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `param` group: CheckedFloat32
pub type glListParameterfSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, param: GLfloat);

/// [glListParameterfvSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glListParameterfvSGIX.xhtml)
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glListParameterfvSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, params: *const GLfloat);

/// [glListParameteriSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glListParameteriSGIX.xhtml)
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `param` group: CheckedInt32
pub type glListParameteriSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, param: GLint);

/// [glListParameterivSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glListParameterivSGIX.xhtml)
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glListParameterivSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, params: *const GLint);

/// [glLoadIdentity](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadIdentity.xhtml)
pub type glLoadIdentity_t = unsafe extern "system" fn();

/// [glLoadIdentityDeformationMapSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadIdentityDeformationMapSGIX.xhtml)
/// * `mask` group: FfdMaskSGIX
pub type glLoadIdentityDeformationMapSGIX_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glLoadMatrixd](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadMatrixd.xhtml)
pub type glLoadMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// [glLoadMatrixf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadMatrixf.xhtml)
pub type glLoadMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// [glLoadMatrixx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadMatrixx.xhtml)
pub type glLoadMatrixx_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// [glLoadMatrixxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadMatrixxOES.xhtml)
pub type glLoadMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// [glLoadName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadName.xhtml)
/// * `name` group: SelectName
pub type glLoadName_t = unsafe extern "system" fn(name: GLuint);

/// [glLoadPaletteFromModelViewMatrixOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadPaletteFromModelViewMatrixOES.xhtml)
pub type glLoadPaletteFromModelViewMatrixOES_t = unsafe extern "system" fn();

/// [glLoadProgramNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadProgramNV.xhtml)
/// * `target` group: VertexAttribEnumNV
/// * `program` len: len
pub type glLoadProgramNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, id: GLuint, len: GLsizei, program: *const GLubyte);

/// [glLoadTransposeMatrixd](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadTransposeMatrixd.xhtml)
pub type glLoadTransposeMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// [glLoadTransposeMatrixdARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadTransposeMatrixdARB.xhtml)
pub type glLoadTransposeMatrixdARB_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// [glLoadTransposeMatrixf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadTransposeMatrixf.xhtml)
pub type glLoadTransposeMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// [glLoadTransposeMatrixfARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadTransposeMatrixfARB.xhtml)
pub type glLoadTransposeMatrixfARB_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// [glLoadTransposeMatrixxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLoadTransposeMatrixxOES.xhtml)
pub type glLoadTransposeMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// [glLockArraysEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLockArraysEXT.xhtml)
pub type glLockArraysEXT_t = unsafe extern "system" fn(first: GLint, count: GLsizei);

/// [glLogicOp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLogicOp.xhtml)
/// * `opcode` group: LogicOp
pub type glLogicOp_t = unsafe extern "system" fn(opcode: LogicOp);

/// [glMakeBufferNonResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeBufferNonResidentNV.xhtml)
pub type glMakeBufferNonResidentNV_t = unsafe extern "system" fn(target: GLenum);

/// [glMakeBufferResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeBufferResidentNV.xhtml)
pub type glMakeBufferResidentNV_t = unsafe extern "system" fn(target: GLenum, access: GLenum);

/// [glMakeImageHandleNonResidentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeImageHandleNonResidentARB.xhtml)
pub type glMakeImageHandleNonResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

/// [glMakeImageHandleNonResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeImageHandleNonResidentNV.xhtml)
pub type glMakeImageHandleNonResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

/// [glMakeImageHandleResidentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeImageHandleResidentARB.xhtml)
pub type glMakeImageHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64, access: GLenum);

/// [glMakeImageHandleResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeImageHandleResidentNV.xhtml)
pub type glMakeImageHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64, access: GLenum);

/// [glMakeNamedBufferNonResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeNamedBufferNonResidentNV.xhtml)
/// * `buffer` class: buffer
pub type glMakeNamedBufferNonResidentNV_t = unsafe extern "system" fn(buffer: GLuint);

/// [glMakeNamedBufferResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeNamedBufferResidentNV.xhtml)
/// * `buffer` class: buffer
pub type glMakeNamedBufferResidentNV_t = unsafe extern "system" fn(buffer: GLuint, access: GLenum);

/// [glMakeTextureHandleNonResidentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeTextureHandleNonResidentARB.xhtml)
pub type glMakeTextureHandleNonResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

/// [glMakeTextureHandleNonResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeTextureHandleNonResidentNV.xhtml)
pub type glMakeTextureHandleNonResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

/// [glMakeTextureHandleResidentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeTextureHandleResidentARB.xhtml)
pub type glMakeTextureHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

/// [glMakeTextureHandleResidentNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMakeTextureHandleResidentNV.xhtml)
pub type glMakeTextureHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

/// [glMap1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMap1d.xhtml)
/// * `target` group: MapTarget
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `order` group: CheckedInt32
/// * `points` group: CoordD
/// * `points` len: COMPSIZE(target,stride,order)
pub type glMap1d_t = unsafe extern "system" fn(target: MapTarget, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble);

/// [glMap1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMap1f.xhtml)
/// * `target` group: MapTarget
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `order` group: CheckedInt32
/// * `points` group: CoordF
/// * `points` len: COMPSIZE(target,stride,order)
pub type glMap1f_t = unsafe extern "system" fn(target: MapTarget, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat);

/// [glMap1xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMap1xOES.xhtml)
/// * `target` group: MapTarget
pub type glMap1xOES_t = unsafe extern "system" fn(target: MapTarget, u1: GLfixed, u2: GLfixed, stride: GLint, order: GLint, points: GLfixed);

/// [glMap2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMap2d.xhtml)
/// * `target` group: MapTarget
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `uorder` group: CheckedInt32
/// * `v1` group: CoordD
/// * `v2` group: CoordD
/// * `vorder` group: CheckedInt32
/// * `points` group: CoordD
/// * `points` len: COMPSIZE(target,ustride,uorder,vstride,vorder)
pub type glMap2d_t = unsafe extern "system" fn(target: MapTarget, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble);

/// [glMap2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMap2f.xhtml)
/// * `target` group: MapTarget
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `uorder` group: CheckedInt32
/// * `v1` group: CoordF
/// * `v2` group: CoordF
/// * `vorder` group: CheckedInt32
/// * `points` group: CoordF
/// * `points` len: COMPSIZE(target,ustride,uorder,vstride,vorder)
pub type glMap2f_t = unsafe extern "system" fn(target: MapTarget, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat);

/// [glMap2xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMap2xOES.xhtml)
/// * `target` group: MapTarget
pub type glMap2xOES_t = unsafe extern "system" fn(target: MapTarget, u1: GLfixed, u2: GLfixed, ustride: GLint, uorder: GLint, v1: GLfixed, v2: GLfixed, vstride: GLint, vorder: GLint, points: GLfixed);

/// [glMapBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBuffer.xhtml)
/// * `target` group: BufferTargetARB
/// * `access` group: BufferAccessARB
pub type glMapBuffer_t = unsafe extern "system" fn(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;

/// [glMapBufferARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBufferARB.xhtml)
/// * `target` group: BufferTargetARB
/// * `access` group: BufferAccessARB
pub type glMapBufferARB_t = unsafe extern "system" fn(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;

/// [glMapBufferOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBufferOES.xhtml)
/// * `target` group: BufferTargetARB
/// * `access` group: BufferAccessARB
pub type glMapBufferOES_t = unsafe extern "system" fn(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;

/// [glMapBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBufferRange.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
/// * `access` group: MapBufferAccessMask
pub type glMapBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

/// [glMapBufferRangeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBufferRangeEXT.xhtml)
/// * `target` group: BufferTargetARB
/// * `access` group: MapBufferAccessMask
pub type glMapBufferRangeEXT_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

/// [glMapControlPointsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapControlPointsNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `type` group: MapTypeNV
/// * `uorder` group: CheckedInt32
/// * `vorder` group: CheckedInt32
/// * `packed` group: Boolean
/// * `points` len: COMPSIZE(target,uorder,vorder)
pub type glMapControlPointsNV_t = unsafe extern "system" fn(target: EvalTargetNV, index: GLuint, type_: MapTypeNV, ustride: GLsizei, vstride: GLsizei, uorder: GLint, vorder: GLint, packed: GLboolean, points: *const void);

/// [glMapGrid1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapGrid1d.xhtml)
/// * `u1` group: CoordD
/// * `u2` group: CoordD
pub type glMapGrid1d_t = unsafe extern "system" fn(un: GLint, u1: GLdouble, u2: GLdouble);

/// [glMapGrid1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapGrid1f.xhtml)
/// * `u1` group: CoordF
/// * `u2` group: CoordF
pub type glMapGrid1f_t = unsafe extern "system" fn(un: GLint, u1: GLfloat, u2: GLfloat);

/// [glMapGrid1xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapGrid1xOES.xhtml)
pub type glMapGrid1xOES_t = unsafe extern "system" fn(n: GLint, u1: GLfixed, u2: GLfixed);

/// [glMapGrid2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapGrid2d.xhtml)
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `v1` group: CoordD
/// * `v2` group: CoordD
pub type glMapGrid2d_t = unsafe extern "system" fn(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble);

/// [glMapGrid2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapGrid2f.xhtml)
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `v1` group: CoordF
/// * `v2` group: CoordF
pub type glMapGrid2f_t = unsafe extern "system" fn(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat);

/// [glMapGrid2xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapGrid2xOES.xhtml)
pub type glMapGrid2xOES_t = unsafe extern "system" fn(n: GLint, u1: GLfixed, u2: GLfixed, v1: GLfixed, v2: GLfixed);

/// [glMapNamedBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapNamedBuffer.xhtml)
/// * `buffer` class: buffer
/// * `access` group: BufferAccessARB
pub type glMapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint, access: BufferAccessARB) -> *mut void;

/// [glMapNamedBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapNamedBufferEXT.xhtml)
/// * `buffer` class: buffer
/// * `access` group: BufferAccessARB
pub type glMapNamedBufferEXT_t = unsafe extern "system" fn(buffer: GLuint, access: BufferAccessARB) -> *mut void;

/// [glMapNamedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapNamedBufferRange.xhtml)
/// * `buffer` class: buffer
/// * `length` group: BufferSize
/// * `access` group: MapBufferAccessMask
pub type glMapNamedBufferRange_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

/// [glMapNamedBufferRangeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapNamedBufferRangeEXT.xhtml)
/// * `buffer` class: buffer
/// * `access` group: MapBufferAccessMask
pub type glMapNamedBufferRangeEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

/// [glMapObjectBufferATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapObjectBufferATI.xhtml)
/// * `buffer` class: buffer
pub type glMapObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint) -> *mut void;

/// [glMapParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapParameterfvNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `pname` group: MapParameterNV
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(target,pname)
pub type glMapParameterfvNV_t = unsafe extern "system" fn(target: EvalTargetNV, pname: MapParameterNV, params: *const GLfloat);

/// [glMapParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapParameterivNV.xhtml)
/// * `target` group: EvalTargetNV
/// * `pname` group: MapParameterNV
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(target,pname)
pub type glMapParameterivNV_t = unsafe extern "system" fn(target: EvalTargetNV, pname: MapParameterNV, params: *const GLint);

/// [glMapTexture2DINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapTexture2DINTEL.xhtml)
/// * `texture` class: texture
pub type glMapTexture2DINTEL_t = unsafe extern "system" fn(texture: GLuint, level: GLint, access: GLbitfield, stride: *mut GLint, layout: *mut GLenum) -> *mut void;

/// [glMapVertexAttrib1dAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapVertexAttrib1dAPPLE.xhtml)
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `order` group: CheckedInt32
/// * `points` group: CoordD
/// * `points` len: COMPSIZE(size,stride,order)
pub type glMapVertexAttrib1dAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble);

/// [glMapVertexAttrib1fAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapVertexAttrib1fAPPLE.xhtml)
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `order` group: CheckedInt32
/// * `points` group: CoordF
/// * `points` len: COMPSIZE(size,stride,order)
pub type glMapVertexAttrib1fAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat);

/// [glMapVertexAttrib2dAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapVertexAttrib2dAPPLE.xhtml)
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `uorder` group: CheckedInt32
/// * `v1` group: CoordD
/// * `v2` group: CoordD
/// * `vorder` group: CheckedInt32
/// * `points` group: CoordD
/// * `points` len: COMPSIZE(size,ustride,uorder,vstride,vorder)
pub type glMapVertexAttrib2dAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble);

/// [glMapVertexAttrib2fAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapVertexAttrib2fAPPLE.xhtml)
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `uorder` group: CheckedInt32
/// * `v1` group: CoordF
/// * `v2` group: CoordF
/// * `vorder` group: CheckedInt32
/// * `points` group: CoordF
/// * `points` len: COMPSIZE(size,ustride,uorder,vstride,vorder)
pub type glMapVertexAttrib2fAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat);

/// [glMaterialf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaterialf.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` group: CheckedFloat32
pub type glMaterialf_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfloat);

/// [glMaterialfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaterialfv.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glMaterialfv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *const GLfloat);

/// [glMateriali](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMateriali.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` group: CheckedInt32
pub type glMateriali_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLint);

/// [glMaterialiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaterialiv.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMaterialiv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *const GLint);

/// [glMaterialx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaterialx.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
pub type glMaterialx_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfixed);

/// [glMaterialxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaterialxOES.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
pub type glMaterialxOES_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfixed);

/// [glMaterialxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaterialxv.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` len: COMPSIZE(pname)
pub type glMaterialxv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: *const GLfixed);

/// [glMaterialxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaterialxvOES.xhtml)
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` len: COMPSIZE(pname)
pub type glMaterialxvOES_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: *const GLfixed);

/// [glMatrixFrustumEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixFrustumEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixFrustumEXT_t = unsafe extern "system" fn(mode: MatrixMode, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

/// [glMatrixIndexPointerARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixIndexPointerARB.xhtml)
/// * `type` group: MatrixIndexPointerTypeARB
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glMatrixIndexPointerARB_t = unsafe extern "system" fn(size: GLint, type_: MatrixIndexPointerTypeARB, stride: GLsizei, pointer: *const void);

/// [glMatrixIndexPointerOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixIndexPointerOES.xhtml)
/// * `type` group: MatrixIndexPointerTypeARB
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glMatrixIndexPointerOES_t = unsafe extern "system" fn(size: GLint, type_: MatrixIndexPointerTypeARB, stride: GLsizei, pointer: *const void);

/// [glMatrixIndexubvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixIndexubvARB.xhtml)
/// * `indices` len: size
pub type glMatrixIndexubvARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLubyte);

/// [glMatrixIndexuivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixIndexuivARB.xhtml)
/// * `indices` len: size
pub type glMatrixIndexuivARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLuint);

/// [glMatrixIndexusvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixIndexusvARB.xhtml)
/// * `indices` len: size
pub type glMatrixIndexusvARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLushort);

/// [glMatrixLoad3x2fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixLoad3x2fNV.xhtml)
pub type glMatrixLoad3x2fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// [glMatrixLoad3x3fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixLoad3x3fNV.xhtml)
pub type glMatrixLoad3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// [glMatrixLoadIdentityEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixLoadIdentityEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixLoadIdentityEXT_t = unsafe extern "system" fn(mode: MatrixMode);

/// [glMatrixLoadTranspose3x3fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixLoadTranspose3x3fNV.xhtml)
pub type glMatrixLoadTranspose3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// [glMatrixLoadTransposedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixLoadTransposedEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixLoadTransposedEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLdouble; 16]);

/// [glMatrixLoadTransposefEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixLoadTransposefEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixLoadTransposefEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLfloat; 16]);

/// [glMatrixLoaddEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixLoaddEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixLoaddEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLdouble; 16]);

/// [glMatrixLoadfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixLoadfEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixLoadfEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLfloat; 16]);

/// [glMatrixMode](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixMode.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixMode_t = unsafe extern "system" fn(mode: MatrixMode);

/// [glMatrixMult3x2fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixMult3x2fNV.xhtml)
pub type glMatrixMult3x2fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// [glMatrixMult3x3fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixMult3x3fNV.xhtml)
pub type glMatrixMult3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// [glMatrixMultTranspose3x3fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixMultTranspose3x3fNV.xhtml)
pub type glMatrixMultTranspose3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// [glMatrixMultTransposedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixMultTransposedEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixMultTransposedEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLdouble; 16]);

/// [glMatrixMultTransposefEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixMultTransposefEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixMultTransposefEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLfloat; 16]);

/// [glMatrixMultdEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixMultdEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixMultdEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLdouble; 16]);

/// [glMatrixMultfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixMultfEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixMultfEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLfloat; 16]);

/// [glMatrixOrthoEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixOrthoEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixOrthoEXT_t = unsafe extern "system" fn(mode: MatrixMode, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

/// [glMatrixPopEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixPopEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixPopEXT_t = unsafe extern "system" fn(mode: MatrixMode);

/// [glMatrixPushEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixPushEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixPushEXT_t = unsafe extern "system" fn(mode: MatrixMode);

/// [glMatrixRotatedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixRotatedEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixRotatedEXT_t = unsafe extern "system" fn(mode: MatrixMode, angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glMatrixRotatefEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixRotatefEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixRotatefEXT_t = unsafe extern "system" fn(mode: MatrixMode, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glMatrixScaledEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixScaledEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixScaledEXT_t = unsafe extern "system" fn(mode: MatrixMode, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glMatrixScalefEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixScalefEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixScalefEXT_t = unsafe extern "system" fn(mode: MatrixMode, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glMatrixTranslatedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixTranslatedEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixTranslatedEXT_t = unsafe extern "system" fn(mode: MatrixMode, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glMatrixTranslatefEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMatrixTranslatefEXT.xhtml)
/// * `mode` group: MatrixMode
pub type glMatrixTranslatefEXT_t = unsafe extern "system" fn(mode: MatrixMode, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glMaxShaderCompilerThreadsARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaxShaderCompilerThreadsARB.xhtml)
pub type glMaxShaderCompilerThreadsARB_t = unsafe extern "system" fn(count: GLuint);

/// [glMaxShaderCompilerThreadsKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMaxShaderCompilerThreadsKHR.xhtml)
pub type glMaxShaderCompilerThreadsKHR_t = unsafe extern "system" fn(count: GLuint);

/// [glMemoryBarrier](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml)
/// * `barriers` group: MemoryBarrierMask
pub type glMemoryBarrier_t = unsafe extern "system" fn(barriers: GLbitfield);

/// [glMemoryBarrierByRegion](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrierByRegion.xhtml)
/// * `barriers` group: MemoryBarrierMask
pub type glMemoryBarrierByRegion_t = unsafe extern "system" fn(barriers: GLbitfield);

/// [glMemoryBarrierEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrierEXT.xhtml)
/// * `barriers` group: MemoryBarrierMask
pub type glMemoryBarrierEXT_t = unsafe extern "system" fn(barriers: GLbitfield);

/// [glMemoryObjectParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryObjectParameterivEXT.xhtml)
/// * `pname` group: MemoryObjectParameterName
pub type glMemoryObjectParameterivEXT_t = unsafe extern "system" fn(memoryObject: GLuint, pname: MemoryObjectParameterName, params: *const GLint);

/// [glMinSampleShading](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMinSampleShading.xhtml)
/// * `value` group: ColorF
pub type glMinSampleShading_t = unsafe extern "system" fn(value: GLfloat);

/// [glMinSampleShadingARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMinSampleShadingARB.xhtml)
/// * `value` group: ColorF
pub type glMinSampleShadingARB_t = unsafe extern "system" fn(value: GLfloat);

/// [glMinSampleShadingOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMinSampleShadingOES.xhtml)
/// * `value` group: ColorF
pub type glMinSampleShadingOES_t = unsafe extern "system" fn(value: GLfloat);

/// [glMinmax](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMinmax.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `internalformat` group: InternalFormat
/// * `sink` group: Boolean
pub type glMinmax_t = unsafe extern "system" fn(target: MinmaxTargetEXT, internalformat: InternalFormat, sink: GLboolean);

/// [glMinmaxEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMinmaxEXT.xhtml)
/// * `target` group: MinmaxTargetEXT
/// * `internalformat` group: InternalFormat
/// * `sink` group: Boolean
pub type glMinmaxEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT, internalformat: InternalFormat, sink: GLboolean);

/// [glMultMatrixd](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultMatrixd.xhtml)
pub type glMultMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// [glMultMatrixf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultMatrixf.xhtml)
pub type glMultMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// [glMultMatrixx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultMatrixx.xhtml)
pub type glMultMatrixx_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// [glMultMatrixxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultMatrixxOES.xhtml)
pub type glMultMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// [glMultTransposeMatrixd](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultTransposeMatrixd.xhtml)
pub type glMultTransposeMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// [glMultTransposeMatrixdARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultTransposeMatrixdARB.xhtml)
pub type glMultTransposeMatrixdARB_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// [glMultTransposeMatrixf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultTransposeMatrixf.xhtml)
pub type glMultTransposeMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// [glMultTransposeMatrixfARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultTransposeMatrixfARB.xhtml)
pub type glMultTransposeMatrixfARB_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// [glMultTransposeMatrixxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultTransposeMatrixxOES.xhtml)
pub type glMultTransposeMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// [glMultiDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArrays.xhtml)
/// * `mode` group: PrimitiveType
/// * `first` len: COMPSIZE(drawcount)
/// * `count` len: COMPSIZE(drawcount)
pub type glMultiDrawArrays_t = unsafe extern "system" fn(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);

/// [glMultiDrawArraysEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `first` len: COMPSIZE(primcount)
/// * `count` len: COMPSIZE(primcount)
pub type glMultiDrawArraysEXT_t = unsafe extern "system" fn(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

/// [glMultiDrawArraysIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirect.xhtml)
/// * `mode` group: PrimitiveType
/// * `indirect` len: COMPSIZE(drawcount,stride)
pub type glMultiDrawArraysIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawArraysIndirectAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirectAMD.xhtml)
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectAMD_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, primcount: GLsizei, stride: GLsizei);

/// [glMultiDrawArraysIndirectBindlessCountNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirectBindlessCountNV.xhtml)
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectBindlessCountNV_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawCount: GLsizei, maxDrawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

/// [glMultiDrawArraysIndirectBindlessNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirectBindlessNV.xhtml)
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectBindlessNV_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

/// [glMultiDrawArraysIndirectCount](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirectCount.xhtml)
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectCount_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawArraysIndirectCountARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirectCountARB.xhtml)
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectCountARB_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawArraysIndirectEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirectEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `indirect` len: COMPSIZE(drawcount,stride)
pub type glMultiDrawArraysIndirectEXT_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawElementArrayAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementArrayAPPLE.xhtml)
/// * `mode` group: PrimitiveType
/// * `first` len: primcount
/// * `count` len: primcount
pub type glMultiDrawElementArrayAPPLE_t = unsafe extern "system" fn(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

/// [glMultiDrawElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElements.xhtml)
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
pub type glMultiDrawElements_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei);

/// [glMultiDrawElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsBaseVertex.xhtml)
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
/// * `basevertex` len: COMPSIZE(drawcount)
pub type glMultiDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);

/// [glMultiDrawElementsBaseVertexEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsBaseVertexEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
/// * `basevertex` len: COMPSIZE(drawcount)
pub type glMultiDrawElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);

/// [glMultiDrawElementsEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(primcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(primcount)
pub type glMultiDrawElementsEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, primcount: GLsizei);

/// [glMultiDrawElementsIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirect.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indirect` len: COMPSIZE(drawcount,stride)
pub type glMultiDrawElementsIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawElementsIndirectAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirectAMD.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectAMD_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, primcount: GLsizei, stride: GLsizei);

/// [glMultiDrawElementsIndirectBindlessCountNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirectBindlessCountNV.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectBindlessCountNV_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawCount: GLsizei, maxDrawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

/// [glMultiDrawElementsIndirectBindlessNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirectBindlessNV.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectBindlessNV_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

/// [glMultiDrawElementsIndirectCount](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirectCount.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectCount_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawElementsIndirectCountARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirectCountARB.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectCountARB_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawElementsIndirectEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirectEXT.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indirect` len: COMPSIZE(drawcount,stride)
pub type glMultiDrawElementsIndirectEXT_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawMeshTasksIndirectCountNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawMeshTasksIndirectCountNV.xhtml)
pub type glMultiDrawMeshTasksIndirectCountNV_t = unsafe extern "system" fn(indirect: GLintptr, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawMeshTasksIndirectNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawMeshTasksIndirectNV.xhtml)
pub type glMultiDrawMeshTasksIndirectNV_t = unsafe extern "system" fn(indirect: GLintptr, drawcount: GLsizei, stride: GLsizei);

/// [glMultiDrawRangeElementArrayAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawRangeElementArrayAPPLE.xhtml)
/// * `mode` group: PrimitiveType
/// * `first` len: primcount
/// * `count` len: primcount
pub type glMultiDrawRangeElementArrayAPPLE_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

/// [glMultiModeDrawArraysIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiModeDrawArraysIBM.xhtml)
/// * `mode` group: PrimitiveType
/// * `mode` len: COMPSIZE(primcount)
/// * `first` len: COMPSIZE(primcount)
/// * `count` len: COMPSIZE(primcount)
pub type glMultiModeDrawArraysIBM_t = unsafe extern "system" fn(mode: *const PrimitiveType, first: *const GLint, count: *const GLsizei, primcount: GLsizei, modestride: GLint);

/// [glMultiModeDrawElementsIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiModeDrawElementsIBM.xhtml)
/// * `mode` group: PrimitiveType
/// * `mode` len: COMPSIZE(primcount)
/// * `count` len: COMPSIZE(primcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(primcount)
pub type glMultiModeDrawElementsIBM_t = unsafe extern "system" fn(mode: *const PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, primcount: GLsizei, modestride: GLint);

/// [glMultiTexBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexBufferEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `buffer` class: buffer
pub type glMultiTexBufferEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, internalformat: GLenum, buffer: GLuint);

/// [glMultiTexCoord1bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1bOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord1bOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLbyte);

/// [glMultiTexCoord1bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1bvOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord1bvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const GLbyte);

/// [glMultiTexCoord1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1d.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordD
pub type glMultiTexCoord1d_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble);

/// [glMultiTexCoord1dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1dARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordD
pub type glMultiTexCoord1dARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble);

/// [glMultiTexCoord1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1dv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord1dv_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLdouble);

/// [glMultiTexCoord1dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1dvARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord1dvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLdouble);

/// [glMultiTexCoord1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1f.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordF
pub type glMultiTexCoord1f_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat);

/// [glMultiTexCoord1fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1fARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordF
pub type glMultiTexCoord1fARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat);

/// [glMultiTexCoord1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1fv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord1fv_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLfloat);

/// [glMultiTexCoord1fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1fvARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord1fvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLfloat);

/// [glMultiTexCoord1hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1hNV.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: Half16NV
pub type glMultiTexCoord1hNV_t = unsafe extern "system" fn(target: TextureUnit, s: GLhalfNV);

/// [glMultiTexCoord1hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1hvNV.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: Half16NV
pub type glMultiTexCoord1hvNV_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLhalfNV);

/// [glMultiTexCoord1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1i.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordI
pub type glMultiTexCoord1i_t = unsafe extern "system" fn(target: TextureUnit, s: GLint);

/// [glMultiTexCoord1iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1iARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordI
pub type glMultiTexCoord1iARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLint);

/// [glMultiTexCoord1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1iv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord1iv_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLint);

/// [glMultiTexCoord1ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1ivARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord1ivARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLint);

/// [glMultiTexCoord1s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1s.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordS
pub type glMultiTexCoord1s_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort);

/// [glMultiTexCoord1sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1sARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordS
pub type glMultiTexCoord1sARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort);

/// [glMultiTexCoord1sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1sv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord1sv_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLshort);

/// [glMultiTexCoord1svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1svARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord1svARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLshort);

/// [glMultiTexCoord1xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1xOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord1xOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed);

/// [glMultiTexCoord1xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord1xvOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord1xvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const GLfixed);

/// [glMultiTexCoord2bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2bOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord2bOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLbyte, t: GLbyte);

/// [glMultiTexCoord2bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2bvOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord2bvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLbyte; 2]);

/// [glMultiTexCoord2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2d.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
pub type glMultiTexCoord2d_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble);

/// [glMultiTexCoord2dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2dARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
pub type glMultiTexCoord2dARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble);

/// [glMultiTexCoord2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2dv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord2dv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 2]);

/// [glMultiTexCoord2dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2dvARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord2dvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 2]);

/// [glMultiTexCoord2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2f.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
pub type glMultiTexCoord2f_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat);

/// [glMultiTexCoord2fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2fARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
pub type glMultiTexCoord2fARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat);

/// [glMultiTexCoord2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2fv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord2fv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 2]);

/// [glMultiTexCoord2fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2fvARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord2fvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 2]);

/// [glMultiTexCoord2hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2hNV.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: Half16NV
/// * `t` group: Half16NV
pub type glMultiTexCoord2hNV_t = unsafe extern "system" fn(target: TextureUnit, s: GLhalfNV, t: GLhalfNV);

/// [glMultiTexCoord2hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2hvNV.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: Half16NV
pub type glMultiTexCoord2hvNV_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLhalfNV; 2]);

/// [glMultiTexCoord2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2i.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
pub type glMultiTexCoord2i_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint);

/// [glMultiTexCoord2iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2iARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
pub type glMultiTexCoord2iARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint);

/// [glMultiTexCoord2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2iv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord2iv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 2]);

/// [glMultiTexCoord2ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2ivARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord2ivARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 2]);

/// [glMultiTexCoord2s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2s.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
pub type glMultiTexCoord2s_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort);

/// [glMultiTexCoord2sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2sARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
pub type glMultiTexCoord2sARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort);

/// [glMultiTexCoord2sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2sv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord2sv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 2]);

/// [glMultiTexCoord2svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2svARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord2svARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 2]);

/// [glMultiTexCoord2xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2xOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord2xOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed, t: GLfixed);

/// [glMultiTexCoord2xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord2xvOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord2xvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLfixed; 2]);

/// [glMultiTexCoord3bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3bOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord3bOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLbyte, t: GLbyte, r: GLbyte);

/// [glMultiTexCoord3bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3bvOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord3bvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLbyte; 3]);

/// [glMultiTexCoord3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3d.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
pub type glMultiTexCoord3d_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble, r: GLdouble);

/// [glMultiTexCoord3dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3dARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
pub type glMultiTexCoord3dARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble, r: GLdouble);

/// [glMultiTexCoord3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3dv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord3dv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 3]);

/// [glMultiTexCoord3dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3dvARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord3dvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 3]);

/// [glMultiTexCoord3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3f.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
pub type glMultiTexCoord3f_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat, r: GLfloat);

/// [glMultiTexCoord3fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3fARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
pub type glMultiTexCoord3fARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat, r: GLfloat);

/// [glMultiTexCoord3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3fv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord3fv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 3]);

/// [glMultiTexCoord3fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3fvARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord3fvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 3]);

/// [glMultiTexCoord3hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3hNV.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: Half16NV
/// * `t` group: Half16NV
/// * `r` group: Half16NV
pub type glMultiTexCoord3hNV_t = unsafe extern "system" fn(target: TextureUnit, s: GLhalfNV, t: GLhalfNV, r: GLhalfNV);

/// [glMultiTexCoord3hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3hvNV.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: Half16NV
pub type glMultiTexCoord3hvNV_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLhalfNV; 3]);

/// [glMultiTexCoord3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3i.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
pub type glMultiTexCoord3i_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint, r: GLint);

/// [glMultiTexCoord3iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3iARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
pub type glMultiTexCoord3iARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint, r: GLint);

/// [glMultiTexCoord3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3iv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord3iv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 3]);

/// [glMultiTexCoord3ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3ivARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord3ivARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 3]);

/// [glMultiTexCoord3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3s.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
pub type glMultiTexCoord3s_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort, r: GLshort);

/// [glMultiTexCoord3sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3sARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
pub type glMultiTexCoord3sARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort, r: GLshort);

/// [glMultiTexCoord3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3sv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord3sv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 3]);

/// [glMultiTexCoord3svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3svARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord3svARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 3]);

/// [glMultiTexCoord3xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3xOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord3xOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed, t: GLfixed, r: GLfixed);

/// [glMultiTexCoord3xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord3xvOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord3xvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLfixed; 3]);

/// [glMultiTexCoord4bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4bOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4bOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLbyte, t: GLbyte, r: GLbyte, q: GLbyte);

/// [glMultiTexCoord4bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4bvOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4bvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLbyte; 4]);

/// [glMultiTexCoord4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4d.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
/// * `q` group: CoordD
pub type glMultiTexCoord4d_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

/// [glMultiTexCoord4dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4dARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
/// * `q` group: CoordD
pub type glMultiTexCoord4dARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

/// [glMultiTexCoord4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4dv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord4dv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 4]);

/// [glMultiTexCoord4dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4dvARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord4dvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 4]);

/// [glMultiTexCoord4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4f.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
/// * `q` group: CoordF
pub type glMultiTexCoord4f_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

/// [glMultiTexCoord4fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4fARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
/// * `q` group: CoordF
pub type glMultiTexCoord4fARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

/// [glMultiTexCoord4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4fv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord4fv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 4]);

/// [glMultiTexCoord4fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4fvARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord4fvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 4]);

/// [glMultiTexCoord4hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4hNV.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: Half16NV
/// * `t` group: Half16NV
/// * `r` group: Half16NV
/// * `q` group: Half16NV
pub type glMultiTexCoord4hNV_t = unsafe extern "system" fn(target: TextureUnit, s: GLhalfNV, t: GLhalfNV, r: GLhalfNV, q: GLhalfNV);

/// [glMultiTexCoord4hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4hvNV.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: Half16NV
pub type glMultiTexCoord4hvNV_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLhalfNV; 4]);

/// [glMultiTexCoord4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4i.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
/// * `q` group: CoordI
pub type glMultiTexCoord4i_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint, r: GLint, q: GLint);

/// [glMultiTexCoord4iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4iARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
/// * `q` group: CoordI
pub type glMultiTexCoord4iARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint, r: GLint, q: GLint);

/// [glMultiTexCoord4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4iv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord4iv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 4]);

/// [glMultiTexCoord4ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4ivARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord4ivARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 4]);

/// [glMultiTexCoord4s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4s.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
/// * `q` group: CoordS
pub type glMultiTexCoord4s_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort, r: GLshort, q: GLshort);

/// [glMultiTexCoord4sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4sARB.xhtml)
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
/// * `q` group: CoordS
pub type glMultiTexCoord4sARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort, r: GLshort, q: GLshort);

/// [glMultiTexCoord4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4sv.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord4sv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 4]);

/// [glMultiTexCoord4svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4svARB.xhtml)
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord4svARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 4]);

/// [glMultiTexCoord4x](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4x.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4x_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

/// [glMultiTexCoord4xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4xOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4xOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

/// [glMultiTexCoord4xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoord4xvOES.xhtml)
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4xvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLfixed; 4]);

/// [glMultiTexCoordP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP1ui.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP1ui_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint);

/// [glMultiTexCoordP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP1uiv.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP1uiv_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint);

/// [glMultiTexCoordP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP2ui.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP2ui_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint);

/// [glMultiTexCoordP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP2uiv.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP2uiv_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint);

/// [glMultiTexCoordP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP3ui.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP3ui_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint);

/// [glMultiTexCoordP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP3uiv.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP3uiv_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint);

/// [glMultiTexCoordP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP4ui.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP4ui_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint);

/// [glMultiTexCoordP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP4uiv.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP4uiv_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint);

/// [glMultiTexCoordPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordPointerEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `type` group: TexCoordPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glMultiTexCoordPointerEXT_t = unsafe extern "system" fn(texunit: TextureUnit, size: GLint, type_: TexCoordPointerType, stride: GLsizei, pointer: *const void);

/// [glMultiTexEnvfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexEnvfEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `param` group: CheckedFloat32
pub type glMultiTexEnvfEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, param: GLfloat);

/// [glMultiTexEnvfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexEnvfvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexEnvfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLfloat);

/// [glMultiTexEnviEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexEnviEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `param` group: CheckedInt32
pub type glMultiTexEnviEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, param: GLint);

/// [glMultiTexEnvivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexEnvivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexEnvivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLint);

/// [glMultiTexGendEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexGendEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glMultiTexGendEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, param: GLdouble);

/// [glMultiTexGendvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexGendvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexGendvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *const GLdouble);

/// [glMultiTexGenfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexGenfEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `param` group: CheckedFloat32
pub type glMultiTexGenfEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, param: GLfloat);

/// [glMultiTexGenfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexGenfvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexGenfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *const GLfloat);

/// [glMultiTexGeniEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexGeniEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `param` group: CheckedInt32
pub type glMultiTexGeniEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, param: GLint);

/// [glMultiTexGenivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexGenivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexGenivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *const GLint);

/// [glMultiTexImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexImage1DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glMultiTexImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexImage2DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glMultiTexImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexImage3DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glMultiTexImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glMultiTexParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexParameterIivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexParameterIivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// [glMultiTexParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexParameterIuivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexParameterIuivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// [glMultiTexParameterfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexParameterfEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedFloat32
pub type glMultiTexParameterfEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, param: GLfloat);

/// [glMultiTexParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexParameterfvEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexParameterfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);

/// [glMultiTexParameteriEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexParameteriEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedInt32
pub type glMultiTexParameteriEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, param: GLint);

/// [glMultiTexParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexParameterivEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexParameterivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// [glMultiTexRenderbufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexRenderbufferEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `renderbuffer` class: renderbuffer
pub type glMultiTexRenderbufferEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, renderbuffer: GLuint);

/// [glMultiTexSubImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexSubImage1DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glMultiTexSubImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexSubImage2DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glMultiTexSubImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexSubImage3DEXT.xhtml)
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glMultiTexSubImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glMulticastBarrierNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastBarrierNV.xhtml)
pub type glMulticastBarrierNV_t = unsafe extern "system" fn();

/// [glMulticastBlitFramebufferNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastBlitFramebufferNV.xhtml)
/// * `mask` group: ClearBufferMask
pub type glMulticastBlitFramebufferNV_t = unsafe extern "system" fn(srcGpu: GLuint, dstGpu: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);

/// [glMulticastBufferSubDataNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastBufferSubDataNV.xhtml)
/// * `buffer` class: buffer
pub type glMulticastBufferSubDataNV_t = unsafe extern "system" fn(gpuMask: GLbitfield, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// [glMulticastCopyBufferSubDataNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastCopyBufferSubDataNV.xhtml)
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
pub type glMulticastCopyBufferSubDataNV_t = unsafe extern "system" fn(readGpu: GLuint, writeGpuMask: GLbitfield, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// [glMulticastCopyImageSubDataNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastCopyImageSubDataNV.xhtml)
pub type glMulticastCopyImageSubDataNV_t = unsafe extern "system" fn(srcGpu: GLuint, dstGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

/// [glMulticastFramebufferSampleLocationsfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastFramebufferSampleLocationsfvNV.xhtml)
/// * `framebuffer` class: framebuffer
pub type glMulticastFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(gpu: GLuint, framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

/// [glMulticastGetQueryObjecti64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastGetQueryObjecti64vNV.xhtml)
pub type glMulticastGetQueryObjecti64vNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLint64);

/// [glMulticastGetQueryObjectivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastGetQueryObjectivNV.xhtml)
pub type glMulticastGetQueryObjectivNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLint);

/// [glMulticastGetQueryObjectui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastGetQueryObjectui64vNV.xhtml)
pub type glMulticastGetQueryObjectui64vNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLuint64);

/// [glMulticastGetQueryObjectuivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastGetQueryObjectuivNV.xhtml)
pub type glMulticastGetQueryObjectuivNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLuint);

/// [glMulticastScissorArrayvNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastScissorArrayvNVX.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glMulticastScissorArrayvNVX_t = unsafe extern "system" fn(gpu: GLuint, first: GLuint, count: GLsizei, v: *const GLint);

/// [glMulticastViewportArrayvNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastViewportArrayvNVX.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glMulticastViewportArrayvNVX_t = unsafe extern "system" fn(gpu: GLuint, first: GLuint, count: GLsizei, v: *const GLfloat);

/// [glMulticastViewportPositionWScaleNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastViewportPositionWScaleNVX.xhtml)
pub type glMulticastViewportPositionWScaleNVX_t = unsafe extern "system" fn(gpu: GLuint, index: GLuint, xcoeff: GLfloat, ycoeff: GLfloat);

/// [glMulticastWaitSyncNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMulticastWaitSyncNV.xhtml)
pub type glMulticastWaitSyncNV_t = unsafe extern "system" fn(signalGpu: GLuint, waitGpuMask: GLbitfield);

/// [glNamedBufferAttachMemoryNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferAttachMemoryNV.xhtml)
/// * `buffer` class: buffer
pub type glNamedBufferAttachMemoryNV_t = unsafe extern "system" fn(buffer: GLuint, memory: GLuint, offset: GLuint64);

/// [glNamedBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferData.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `usage` group: VertexBufferObjectUsage
pub type glNamedBufferData_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: VertexBufferObjectUsage);

/// [glNamedBufferDataEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferDataEXT.xhtml)
/// * `buffer` class: buffer
/// * `data` len: COMPSIZE(size)
/// * `usage` group: VertexBufferObjectUsage
pub type glNamedBufferDataEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: VertexBufferObjectUsage);

/// [glNamedBufferPageCommitmentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferPageCommitmentARB.xhtml)
/// * `buffer` class: buffer
/// * `commit` group: Boolean
pub type glNamedBufferPageCommitmentARB_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

/// [glNamedBufferPageCommitmentEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferPageCommitmentEXT.xhtml)
/// * `buffer` class: buffer
/// * `commit` group: Boolean
pub type glNamedBufferPageCommitmentEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

/// [glNamedBufferPageCommitmentMemNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferPageCommitmentMemNV.xhtml)
/// * `buffer` class: buffer
/// * `commit` group: Boolean
pub type glNamedBufferPageCommitmentMemNV_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, memory: GLuint, memOffset: GLuint64, commit: GLboolean);

/// [glNamedBufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferStorage.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: size
/// * `flags` group: BufferStorageMask
pub type glNamedBufferStorage_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield);

/// [glNamedBufferStorageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferStorageEXT.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: size
/// * `flags` group: BufferStorageMask
pub type glNamedBufferStorageEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield);

/// [glNamedBufferStorageExternalEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferStorageExternalEXT.xhtml)
/// * `buffer` class: buffer
/// * `flags` group: BufferStorageMask
pub type glNamedBufferStorageExternalEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, clientBuffer: GLeglClientBufferEXT, flags: GLbitfield);

/// [glNamedBufferStorageMemEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferStorageMemEXT.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
pub type glNamedBufferStorageMemEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, memory: GLuint, offset: GLuint64);

/// [glNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferSubData.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: COMPSIZE(size)
pub type glNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// [glNamedBufferSubDataEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferSubDataEXT.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: COMPSIZE(size)
pub type glNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// [glNamedCopyBufferSubDataEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedCopyBufferSubDataEXT.xhtml)
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
pub type glNamedCopyBufferSubDataEXT_t = unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// [glNamedFramebufferDrawBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferDrawBuffer.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buf` group: ColorBuffer
pub type glNamedFramebufferDrawBuffer_t = unsafe extern "system" fn(framebuffer: GLuint, buf: ColorBuffer);

/// [glNamedFramebufferDrawBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferDrawBuffers.xhtml)
/// * `framebuffer` class: framebuffer
/// * `bufs` group: ColorBuffer
pub type glNamedFramebufferDrawBuffers_t = unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const ColorBuffer);

/// [glNamedFramebufferParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferParameteri.xhtml)
/// * `framebuffer` class: framebuffer
/// * `pname` group: FramebufferParameterName
pub type glNamedFramebufferParameteri_t = unsafe extern "system" fn(framebuffer: GLuint, pname: FramebufferParameterName, param: GLint);

/// [glNamedFramebufferParameteriEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferParameteriEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `pname` group: FramebufferParameterName
pub type glNamedFramebufferParameteriEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: FramebufferParameterName, param: GLint);

/// [glNamedFramebufferReadBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferReadBuffer.xhtml)
/// * `framebuffer` class: framebuffer
/// * `src` group: ColorBuffer
pub type glNamedFramebufferReadBuffer_t = unsafe extern "system" fn(framebuffer: GLuint, src: ColorBuffer);

/// [glNamedFramebufferRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferRenderbuffer.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glNamedFramebufferRenderbuffer_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// [glNamedFramebufferRenderbufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferRenderbufferEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
pub type glNamedFramebufferRenderbufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// [glNamedFramebufferSampleLocationsfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferSampleLocationsfvARB.xhtml)
/// * `framebuffer` class: framebuffer
pub type glNamedFramebufferSampleLocationsfvARB_t = unsafe extern "system" fn(framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

/// [glNamedFramebufferSampleLocationsfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferSampleLocationsfvNV.xhtml)
/// * `framebuffer` class: framebuffer
pub type glNamedFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

/// [glNamedFramebufferSamplePositionsfvAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferSamplePositionsfvAMD.xhtml)
/// * `framebuffer` class: framebuffer
pub type glNamedFramebufferSamplePositionsfvAMD_t = unsafe extern "system" fn(framebuffer: GLuint, numsamples: GLuint, pixelindex: GLuint, values: *const GLfloat);

/// [glNamedFramebufferTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTexture.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
pub type glNamedFramebufferTexture_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// [glNamedFramebufferTexture1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTexture1DEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glNamedFramebufferTexture1DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// [glNamedFramebufferTexture2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTexture2DEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glNamedFramebufferTexture2DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// [glNamedFramebufferTexture3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTexture3DEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
pub type glNamedFramebufferTexture3DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

/// [glNamedFramebufferTextureEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTextureEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glNamedFramebufferTextureEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// [glNamedFramebufferTextureFaceEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTextureFaceEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `face` group: TextureTarget
pub type glNamedFramebufferTextureFaceEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, face: TextureTarget);

/// [glNamedFramebufferTextureLayer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTextureLayer.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
pub type glNamedFramebufferTextureLayer_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// [glNamedFramebufferTextureLayerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTextureLayerEXT.xhtml)
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glNamedFramebufferTextureLayerEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// [glNamedProgramLocalParameter4dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameter4dEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameter4dEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glNamedProgramLocalParameter4dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameter4dvEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameter4dvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *const [GLdouble; 4]);

/// [glNamedProgramLocalParameter4fEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameter4fEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameter4fEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glNamedProgramLocalParameter4fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameter4fvEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameter4fvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *const [GLfloat; 4]);

/// [glNamedProgramLocalParameterI4iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameterI4iEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameterI4iEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// [glNamedProgramLocalParameterI4ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameterI4ivEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameterI4ivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *const [GLint; 4]);

/// [glNamedProgramLocalParameterI4uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameterI4uiEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameterI4uiEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// [glNamedProgramLocalParameterI4uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameterI4uivEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameterI4uivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *const [GLuint; 4]);

/// [glNamedProgramLocalParameters4fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParameters4fvEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glNamedProgramLocalParameters4fvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLfloat);

/// [glNamedProgramLocalParametersI4ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParametersI4ivEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glNamedProgramLocalParametersI4ivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLint);

/// [glNamedProgramLocalParametersI4uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramLocalParametersI4uivEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glNamedProgramLocalParametersI4uivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLuint);

/// [glNamedProgramStringEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedProgramStringEXT.xhtml)
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `format` group: ProgramFormat
/// * `string` len: len
pub type glNamedProgramStringEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, format: ProgramFormat, len: GLsizei, string: *const void);

/// [glNamedRenderbufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorage.xhtml)
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorage_t = unsafe extern "system" fn(renderbuffer: GLuint, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glNamedRenderbufferStorageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorageEXT.xhtml)
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glNamedRenderbufferStorageMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorageMultisample.xhtml)
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageMultisample_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glNamedRenderbufferStorageMultisampleAdvancedAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorageMultisampleAdvancedAMD.xhtml)
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageMultisampleAdvancedAMD_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, storageSamples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glNamedRenderbufferStorageMultisampleCoverageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorageMultisampleCoverageEXT.xhtml)
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageMultisampleCoverageEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glNamedRenderbufferStorageMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorageMultisampleEXT.xhtml)
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageMultisampleEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glNamedStringARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedStringARB.xhtml)
/// * `name` len: namelen
/// * `string` len: stringlen
pub type glNamedStringARB_t = unsafe extern "system" fn(type_: GLenum, namelen: GLint, name: *const GLchar, stringlen: GLint, string: *const GLchar);

/// [glNewList](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNewList.xhtml)
/// * `list` group: List
/// * `mode` group: ListMode
pub type glNewList_t = unsafe extern "system" fn(list: GLuint, mode: ListMode);

/// [glNewObjectBufferATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNewObjectBufferATI.xhtml)
/// * `pointer` len: size
/// * `usage` group: ArrayObjectUsageATI
pub type glNewObjectBufferATI_t = unsafe extern "system" fn(size: GLsizei, pointer: *const void, usage: ArrayObjectUsageATI) -> GLuint;

/// [glNormal3b](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3b.xhtml)
pub type glNormal3b_t = unsafe extern "system" fn(nx: GLbyte, ny: GLbyte, nz: GLbyte);

/// [glNormal3bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3bv.xhtml)
pub type glNormal3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// [glNormal3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3d.xhtml)
/// * `nx` group: CoordD
/// * `ny` group: CoordD
/// * `nz` group: CoordD
pub type glNormal3d_t = unsafe extern "system" fn(nx: GLdouble, ny: GLdouble, nz: GLdouble);

/// [glNormal3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3dv.xhtml)
/// * `v` group: CoordD
pub type glNormal3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glNormal3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3f.xhtml)
/// * `nx` group: CoordF
/// * `ny` group: CoordF
/// * `nz` group: CoordF
pub type glNormal3f_t = unsafe extern "system" fn(nx: GLfloat, ny: GLfloat, nz: GLfloat);

/// [glNormal3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3fVertex3fSUN.xhtml)
pub type glNormal3fVertex3fSUN_t = unsafe extern "system" fn(nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glNormal3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3fVertex3fvSUN.xhtml)
pub type glNormal3fVertex3fvSUN_t = unsafe extern "system" fn(n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glNormal3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3fv.xhtml)
/// * `v` group: CoordF
pub type glNormal3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glNormal3hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3hNV.xhtml)
/// * `nx` group: Half16NV
/// * `ny` group: Half16NV
/// * `nz` group: Half16NV
pub type glNormal3hNV_t = unsafe extern "system" fn(nx: GLhalfNV, ny: GLhalfNV, nz: GLhalfNV);

/// [glNormal3hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3hvNV.xhtml)
/// * `v` group: Half16NV
pub type glNormal3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// [glNormal3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3i.xhtml)
pub type glNormal3i_t = unsafe extern "system" fn(nx: GLint, ny: GLint, nz: GLint);

/// [glNormal3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3iv.xhtml)
pub type glNormal3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glNormal3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3s.xhtml)
pub type glNormal3s_t = unsafe extern "system" fn(nx: GLshort, ny: GLshort, nz: GLshort);

/// [glNormal3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3sv.xhtml)
pub type glNormal3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glNormal3x](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3x.xhtml)
pub type glNormal3x_t = unsafe extern "system" fn(nx: GLfixed, ny: GLfixed, nz: GLfixed);

/// [glNormal3xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3xOES.xhtml)
pub type glNormal3xOES_t = unsafe extern "system" fn(nx: GLfixed, ny: GLfixed, nz: GLfixed);

/// [glNormal3xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormal3xvOES.xhtml)
pub type glNormal3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

/// [glNormalFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalFormatNV.xhtml)
pub type glNormalFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

/// [glNormalP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalP3ui.xhtml)
/// * `type` group: NormalPointerType
pub type glNormalP3ui_t = unsafe extern "system" fn(type_: NormalPointerType, coords: GLuint);

/// [glNormalP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalP3uiv.xhtml)
/// * `type` group: NormalPointerType
pub type glNormalP3uiv_t = unsafe extern "system" fn(type_: NormalPointerType, coords: *const GLuint);

/// [glNormalPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalPointer.xhtml)
/// * `type` group: NormalPointerType
/// * `pointer` len: COMPSIZE(type,stride)
pub type glNormalPointer_t = unsafe extern "system" fn(type_: NormalPointerType, stride: GLsizei, pointer: *const void);

/// [glNormalPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalPointerEXT.xhtml)
/// * `type` group: NormalPointerType
/// * `pointer` len: COMPSIZE(type,stride,count)
pub type glNormalPointerEXT_t = unsafe extern "system" fn(type_: NormalPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// [glNormalPointerListIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalPointerListIBM.xhtml)
/// * `type` group: NormalPointerType
/// * `pointer` len: COMPSIZE(type,stride)
pub type glNormalPointerListIBM_t = unsafe extern "system" fn(type_: NormalPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// [glNormalPointervINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalPointervINTEL.xhtml)
/// * `type` group: NormalPointerType
pub type glNormalPointervINTEL_t = unsafe extern "system" fn(type_: NormalPointerType, pointer: *const [*mut void; 4]);

/// [glNormalStream3bATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3bATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3bATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLbyte, ny: GLbyte, nz: GLbyte);

/// [glNormalStream3bvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3bvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3bvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLbyte; 3]);

/// [glNormalStream3dATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3dATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLdouble, ny: GLdouble, nz: GLdouble);

/// [glNormalStream3dvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3dvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLdouble; 3]);

/// [glNormalStream3fATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3fATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLfloat, ny: GLfloat, nz: GLfloat);

/// [glNormalStream3fvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3fvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLfloat; 3]);

/// [glNormalStream3iATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3iATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLint, ny: GLint, nz: GLint);

/// [glNormalStream3ivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3ivATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLint; 3]);

/// [glNormalStream3sATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3sATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLshort, ny: GLshort, nz: GLshort);

/// [glNormalStream3svATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalStream3svATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glNormalStream3svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLshort; 3]);

/// [glObjectLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectLabel.xhtml)
/// * `identifier` group: ObjectIdentifier
/// * `label` len: COMPSIZE(label,length)
pub type glObjectLabel_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar);

/// [glObjectLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectLabelKHR.xhtml)
/// * `identifier` group: ObjectIdentifier
pub type glObjectLabelKHR_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar);

/// [glObjectPtrLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectPtrLabel.xhtml)
/// * `label` len: COMPSIZE(label,length)
pub type glObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, length: GLsizei, label: *const GLchar);

/// [glObjectPtrLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectPtrLabelKHR.xhtml)
pub type glObjectPtrLabelKHR_t = unsafe extern "system" fn(ptr: *const void, length: GLsizei, label: *const GLchar);

/// [glObjectPurgeableAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectPurgeableAPPLE.xhtml)
pub type glObjectPurgeableAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, option: GLenum) -> GLenum;

/// [glObjectUnpurgeableAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectUnpurgeableAPPLE.xhtml)
pub type glObjectUnpurgeableAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, option: GLenum) -> GLenum;

/// [glOrtho](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glOrtho.xhtml)
pub type glOrtho_t = unsafe extern "system" fn(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

/// [glOrthof](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glOrthof.xhtml)
pub type glOrthof_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

/// [glOrthofOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glOrthofOES.xhtml)
pub type glOrthofOES_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

/// [glOrthox](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glOrthox.xhtml)
pub type glOrthox_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

/// [glOrthoxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glOrthoxOES.xhtml)
pub type glOrthoxOES_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

/// [glPNTrianglesfATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPNTrianglesfATI.xhtml)
/// * `pname` group: PNTrianglesPNameATI
pub type glPNTrianglesfATI_t = unsafe extern "system" fn(pname: PNTrianglesPNameATI, param: GLfloat);

/// [glPNTrianglesiATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPNTrianglesiATI.xhtml)
/// * `pname` group: PNTrianglesPNameATI
pub type glPNTrianglesiATI_t = unsafe extern "system" fn(pname: PNTrianglesPNameATI, param: GLint);

/// [glPassTexCoordATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPassTexCoordATI.xhtml)
/// * `swizzle` group: SwizzleOpATI
pub type glPassTexCoordATI_t = unsafe extern "system" fn(dst: GLuint, coord: GLuint, swizzle: SwizzleOpATI);

/// [glPassThrough](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPassThrough.xhtml)
/// * `token` group: FeedbackElement
pub type glPassThrough_t = unsafe extern "system" fn(token: GLfloat);

/// [glPassThroughxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPassThroughxOES.xhtml)
pub type glPassThroughxOES_t = unsafe extern "system" fn(token: GLfixed);

/// [glPatchParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPatchParameterfv.xhtml)
/// * `pname` group: PatchParameterName
/// * `values` len: COMPSIZE(pname)
pub type glPatchParameterfv_t = unsafe extern "system" fn(pname: PatchParameterName, values: *const GLfloat);

/// [glPatchParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPatchParameteri.xhtml)
/// * `pname` group: PatchParameterName
pub type glPatchParameteri_t = unsafe extern "system" fn(pname: PatchParameterName, value: GLint);

/// [glPatchParameteriEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPatchParameteriEXT.xhtml)
/// * `pname` group: PatchParameterName
pub type glPatchParameteriEXT_t = unsafe extern "system" fn(pname: PatchParameterName, value: GLint);

/// [glPatchParameteriOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPatchParameteriOES.xhtml)
/// * `pname` group: PatchParameterName
pub type glPatchParameteriOES_t = unsafe extern "system" fn(pname: PatchParameterName, value: GLint);

/// [glPathColorGenNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathColorGenNV.xhtml)
/// * `color` group: PathColor
/// * `genMode` group: PathGenMode
/// * `colorFormat` group: PathColorFormat
/// * `coeffs` len: COMPSIZE(genMode,colorFormat)
pub type glPathColorGenNV_t = unsafe extern "system" fn(color: PathColor, genMode: PathGenMode, colorFormat: PathColorFormat, coeffs: *const GLfloat);

/// [glPathCommandsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathCommandsNV.xhtml)
/// * `path` group: Path
/// * `commands` group: PathCommand
/// * `commands` len: numCommands
/// * `coordType` group: PathCoordType
/// * `coords` len: COMPSIZE(numCoords,coordType)
pub type glPathCommandsNV_t = unsafe extern "system" fn(path: GLuint, numCommands: GLsizei, commands: *const GLubyte, numCoords: GLsizei, coordType: PathCoordType, coords: *const void);

/// [glPathCoordsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathCoordsNV.xhtml)
/// * `path` group: Path
/// * `coordType` group: PathCoordType
/// * `coords` len: COMPSIZE(numCoords,coordType)
pub type glPathCoordsNV_t = unsafe extern "system" fn(path: GLuint, numCoords: GLsizei, coordType: PathCoordType, coords: *const void);

/// [glPathCoverDepthFuncNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathCoverDepthFuncNV.xhtml)
/// * `func` group: DepthFunction
pub type glPathCoverDepthFuncNV_t = unsafe extern "system" fn(func: DepthFunction);

/// [glPathDashArrayNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathDashArrayNV.xhtml)
/// * `path` group: Path
/// * `dashArray` len: dashCount
pub type glPathDashArrayNV_t = unsafe extern "system" fn(path: GLuint, dashCount: GLsizei, dashArray: *const GLfloat);

/// [glPathFogGenNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathFogGenNV.xhtml)
/// * `genMode` group: PathGenMode
pub type glPathFogGenNV_t = unsafe extern "system" fn(genMode: PathGenMode);

/// [glPathGlyphIndexArrayNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathGlyphIndexArrayNV.xhtml)
/// * `fontStyle` group: PathFontStyle
pub type glPathGlyphIndexArrayNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: GLenum, fontName: *const void, fontStyle: GLbitfield, firstGlyphIndex: GLuint, numGlyphs: GLsizei, pathParameterTemplate: GLuint, emScale: GLfloat) -> GLenum;

/// [glPathGlyphIndexRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathGlyphIndexRangeNV.xhtml)
/// * `fontStyle` group: PathFontStyle
/// * `baseAndCount` len: [2]
pub type glPathGlyphIndexRangeNV_t = unsafe extern "system" fn(fontTarget: GLenum, fontName: *const void, fontStyle: GLbitfield, pathParameterTemplate: GLuint, emScale: GLfloat, baseAndCount: GLuint) -> GLenum;

/// [glPathGlyphRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathGlyphRangeNV.xhtml)
/// * `firstPathName` group: Path
/// * `fontTarget` group: PathFontTarget
/// * `fontName` len: COMPSIZE(fontTarget,fontName)
/// * `fontStyle` group: PathFontStyle
/// * `handleMissingGlyphs` group: PathHandleMissingGlyphs
/// * `pathParameterTemplate` group: Path
pub type glPathGlyphRangeNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: PathFontTarget, fontName: *const void, fontStyle: GLbitfield, firstGlyph: GLuint, numGlyphs: GLsizei, handleMissingGlyphs: PathHandleMissingGlyphs, pathParameterTemplate: GLuint, emScale: GLfloat);

/// [glPathGlyphsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathGlyphsNV.xhtml)
/// * `firstPathName` group: Path
/// * `fontTarget` group: PathFontTarget
/// * `fontName` len: COMPSIZE(fontTarget,fontName)
/// * `fontStyle` group: PathFontStyle
/// * `type` group: PathElementType
/// * `charcodes` len: COMPSIZE(numGlyphs,type,charcodes)
/// * `handleMissingGlyphs` group: PathHandleMissingGlyphs
/// * `pathParameterTemplate` group: Path
pub type glPathGlyphsNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: PathFontTarget, fontName: *const void, fontStyle: GLbitfield, numGlyphs: GLsizei, type_: PathElementType, charcodes: *const void, handleMissingGlyphs: PathHandleMissingGlyphs, pathParameterTemplate: GLuint, emScale: GLfloat);

/// [glPathMemoryGlyphIndexArrayNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathMemoryGlyphIndexArrayNV.xhtml)
pub type glPathMemoryGlyphIndexArrayNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: GLenum, fontSize: GLsizeiptr, fontData: *const void, faceIndex: GLsizei, firstGlyphIndex: GLuint, numGlyphs: GLsizei, pathParameterTemplate: GLuint, emScale: GLfloat) -> GLenum;

/// [glPathParameterfNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathParameterfNV.xhtml)
/// * `path` group: Path
/// * `pname` group: PathParameter
pub type glPathParameterfNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: GLfloat);

/// [glPathParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathParameterfvNV.xhtml)
/// * `path` group: Path
/// * `pname` group: PathParameter
/// * `value` len: COMPSIZE(pname)
pub type glPathParameterfvNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: *const GLfloat);

/// [glPathParameteriNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathParameteriNV.xhtml)
/// * `path` group: Path
/// * `pname` group: PathParameter
pub type glPathParameteriNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: GLint);

/// [glPathParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathParameterivNV.xhtml)
/// * `path` group: Path
/// * `pname` group: PathParameter
/// * `value` len: COMPSIZE(pname)
pub type glPathParameterivNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: *const GLint);

/// [glPathStencilDepthOffsetNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathStencilDepthOffsetNV.xhtml)
pub type glPathStencilDepthOffsetNV_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);

/// [glPathStencilFuncNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathStencilFuncNV.xhtml)
/// * `func` group: StencilFunction
/// * `ref` group: ClampedStencilValue
/// * `mask` group: MaskedStencilValue
pub type glPathStencilFuncNV_t = unsafe extern "system" fn(func: StencilFunction, ref_: GLint, mask: GLuint);

/// [glPathStringNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathStringNV.xhtml)
/// * `path` group: Path
/// * `format` group: PathStringFormat
/// * `pathString` len: length
pub type glPathStringNV_t = unsafe extern "system" fn(path: GLuint, format: PathStringFormat, length: GLsizei, pathString: *const void);

/// [glPathSubCommandsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathSubCommandsNV.xhtml)
/// * `path` group: Path
/// * `commands` group: PathCommand
/// * `commands` len: numCommands
/// * `coordType` group: PathCoordType
/// * `coords` len: COMPSIZE(numCoords,coordType)
pub type glPathSubCommandsNV_t = unsafe extern "system" fn(path: GLuint, commandStart: GLsizei, commandsToDelete: GLsizei, numCommands: GLsizei, commands: *const GLubyte, numCoords: GLsizei, coordType: PathCoordType, coords: *const void);

/// [glPathSubCoordsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathSubCoordsNV.xhtml)
/// * `path` group: Path
/// * `coordType` group: PathCoordType
/// * `coords` len: COMPSIZE(numCoords,coordType)
pub type glPathSubCoordsNV_t = unsafe extern "system" fn(path: GLuint, coordStart: GLsizei, numCoords: GLsizei, coordType: PathCoordType, coords: *const void);

/// [glPathTexGenNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPathTexGenNV.xhtml)
/// * `texCoordSet` group: PathColor
/// * `genMode` group: PathGenMode
/// * `coeffs` len: COMPSIZE(genMode,components)
pub type glPathTexGenNV_t = unsafe extern "system" fn(texCoordSet: PathColor, genMode: PathGenMode, components: GLint, coeffs: *const GLfloat);

/// [glPauseTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPauseTransformFeedback.xhtml)
pub type glPauseTransformFeedback_t = unsafe extern "system" fn();

/// [glPauseTransformFeedbackNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPauseTransformFeedbackNV.xhtml)
pub type glPauseTransformFeedbackNV_t = unsafe extern "system" fn();

/// [glPixelDataRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelDataRangeNV.xhtml)
/// * `target` group: PixelDataRangeTargetNV
/// * `pointer` len: length
pub type glPixelDataRangeNV_t = unsafe extern "system" fn(target: PixelDataRangeTargetNV, length: GLsizei, pointer: *const void);

/// [glPixelMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelMapfv.xhtml)
/// * `map` group: PixelMap
/// * `mapsize` group: CheckedInt32
/// * `values` len: mapsize
pub type glPixelMapfv_t = unsafe extern "system" fn(map: PixelMap, mapsize: GLsizei, values: *const GLfloat);

/// [glPixelMapuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelMapuiv.xhtml)
/// * `map` group: PixelMap
/// * `mapsize` group: CheckedInt32
/// * `values` len: mapsize
pub type glPixelMapuiv_t = unsafe extern "system" fn(map: PixelMap, mapsize: GLsizei, values: *const GLuint);

/// [glPixelMapusv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelMapusv.xhtml)
/// * `map` group: PixelMap
/// * `mapsize` group: CheckedInt32
/// * `values` len: mapsize
pub type glPixelMapusv_t = unsafe extern "system" fn(map: PixelMap, mapsize: GLsizei, values: *const GLushort);

/// [glPixelMapx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelMapx.xhtml)
/// * `map` group: PixelMap
/// * `values` len: size
pub type glPixelMapx_t = unsafe extern "system" fn(map: PixelMap, size: GLint, values: *const GLfixed);

/// [glPixelStoref](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelStoref.xhtml)
/// * `pname` group: PixelStoreParameter
/// * `param` group: CheckedFloat32
pub type glPixelStoref_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLfloat);

/// [glPixelStorei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelStorei.xhtml)
/// * `pname` group: PixelStoreParameter
/// * `param` group: CheckedInt32
pub type glPixelStorei_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLint);

/// [glPixelStorex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelStorex.xhtml)
/// * `pname` group: PixelStoreParameter
pub type glPixelStorex_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLfixed);

/// [glPixelTexGenParameterfSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTexGenParameterfSGIS.xhtml)
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `param` group: CheckedFloat32
pub type glPixelTexGenParameterfSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, param: GLfloat);

/// [glPixelTexGenParameterfvSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTexGenParameterfvSGIS.xhtml)
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPixelTexGenParameterfvSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, params: *const GLfloat);

/// [glPixelTexGenParameteriSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTexGenParameteriSGIS.xhtml)
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `param` group: CheckedInt32
pub type glPixelTexGenParameteriSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, param: GLint);

/// [glPixelTexGenParameterivSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTexGenParameterivSGIS.xhtml)
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glPixelTexGenParameterivSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, params: *const GLint);

/// [glPixelTexGenSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTexGenSGIX.xhtml)
/// * `mode` group: PixelTexGenModeSGIX
pub type glPixelTexGenSGIX_t = unsafe extern "system" fn(mode: PixelTexGenModeSGIX);

/// [glPixelTransferf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTransferf.xhtml)
/// * `pname` group: PixelTransferParameter
/// * `param` group: CheckedFloat32
pub type glPixelTransferf_t = unsafe extern "system" fn(pname: PixelTransferParameter, param: GLfloat);

/// [glPixelTransferi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTransferi.xhtml)
/// * `pname` group: PixelTransferParameter
/// * `param` group: CheckedInt32
pub type glPixelTransferi_t = unsafe extern "system" fn(pname: PixelTransferParameter, param: GLint);

/// [glPixelTransferxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTransferxOES.xhtml)
/// * `pname` group: PixelTransferParameter
pub type glPixelTransferxOES_t = unsafe extern "system" fn(pname: PixelTransferParameter, param: GLfixed);

/// [glPixelTransformParameterfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTransformParameterfEXT.xhtml)
/// * `target` group: PixelTransformTargetEXT
/// * `pname` group: PixelTransformPNameEXT
pub type glPixelTransformParameterfEXT_t = unsafe extern "system" fn(target: PixelTransformTargetEXT, pname: PixelTransformPNameEXT, param: GLfloat);

/// [glPixelTransformParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTransformParameterfvEXT.xhtml)
/// * `target` group: PixelTransformTargetEXT
/// * `pname` group: PixelTransformPNameEXT
pub type glPixelTransformParameterfvEXT_t = unsafe extern "system" fn(target: PixelTransformTargetEXT, pname: PixelTransformPNameEXT, params: *const GLfloat);

/// [glPixelTransformParameteriEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTransformParameteriEXT.xhtml)
/// * `target` group: PixelTransformTargetEXT
/// * `pname` group: PixelTransformPNameEXT
pub type glPixelTransformParameteriEXT_t = unsafe extern "system" fn(target: PixelTransformTargetEXT, pname: PixelTransformPNameEXT, param: GLint);

/// [glPixelTransformParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelTransformParameterivEXT.xhtml)
/// * `target` group: PixelTransformTargetEXT
/// * `pname` group: PixelTransformPNameEXT
pub type glPixelTransformParameterivEXT_t = unsafe extern "system" fn(target: PixelTransformTargetEXT, pname: PixelTransformPNameEXT, params: *const GLint);

/// [glPixelZoom](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelZoom.xhtml)
pub type glPixelZoom_t = unsafe extern "system" fn(xfactor: GLfloat, yfactor: GLfloat);

/// [glPixelZoomxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelZoomxOES.xhtml)
pub type glPixelZoomxOES_t = unsafe extern "system" fn(xfactor: GLfixed, yfactor: GLfixed);

/// [glPointAlongPathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointAlongPathNV.xhtml)
/// * `path` group: Path
pub type glPointAlongPathNV_t = unsafe extern "system" fn(path: GLuint, startSegment: GLsizei, numSegments: GLsizei, distance: GLfloat, x: *mut GLfloat, y: *mut GLfloat, tangentX: *mut GLfloat, tangentY: *mut GLfloat) -> GLboolean;

/// [glPointParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterf.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
pub type glPointParameterf_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

/// [glPointParameterfARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfARB.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
pub type glPointParameterfARB_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

/// [glPointParameterfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfEXT.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
pub type glPointParameterfEXT_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

/// [glPointParameterfSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfSGIS.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
pub type glPointParameterfSGIS_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

/// [glPointParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfv.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterfv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

/// [glPointParameterfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfvARB.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterfvARB_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

/// [glPointParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfvEXT.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterfvEXT_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

/// [glPointParameterfvSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfvSGIS.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterfvSGIS_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

/// [glPointParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameteri.xhtml)
/// * `pname` group: PointParameterNameARB
pub type glPointParameteri_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLint);

/// [glPointParameteriNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameteriNV.xhtml)
/// * `pname` group: PointParameterNameARB
pub type glPointParameteriNV_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLint);

/// [glPointParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameteriv.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
pub type glPointParameteriv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLint);

/// [glPointParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterivNV.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterivNV_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLint);

/// [glPointParameterx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterx.xhtml)
/// * `pname` group: PointParameterNameARB
pub type glPointParameterx_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfixed);

/// [glPointParameterxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterxOES.xhtml)
/// * `pname` group: PointParameterNameARB
pub type glPointParameterxOES_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfixed);

/// [glPointParameterxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterxv.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterxv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfixed);

/// [glPointParameterxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterxvOES.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterxvOES_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfixed);

/// [glPointSize](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSize.xhtml)
/// * `size` group: CheckedFloat32
pub type glPointSize_t = unsafe extern "system" fn(size: GLfloat);

/// [glPointSizePointerOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSizePointerOES.xhtml)
/// * `pointer` len: COMPSIZE(type,stride)
pub type glPointSizePointerOES_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

/// [glPointSizex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSizex.xhtml)
pub type glPointSizex_t = unsafe extern "system" fn(size: GLfixed);

/// [glPointSizexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSizexOES.xhtml)
pub type glPointSizexOES_t = unsafe extern "system" fn(size: GLfixed);

/// [glPollAsyncSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPollAsyncSGIX.xhtml)
pub type glPollAsyncSGIX_t = unsafe extern "system" fn(markerp: *mut GLuint) -> GLint;

/// [glPollInstrumentsSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPollInstrumentsSGIX.xhtml)
pub type glPollInstrumentsSGIX_t = unsafe extern "system" fn(marker_p: *mut GLint) -> GLint;

/// [glPolygonMode](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonMode.xhtml)
/// * `face` group: MaterialFace
/// * `mode` group: PolygonMode
pub type glPolygonMode_t = unsafe extern "system" fn(face: MaterialFace, mode: PolygonMode);

/// [glPolygonModeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonModeNV.xhtml)
/// * `face` group: MaterialFace
/// * `mode` group: PolygonMode
pub type glPolygonModeNV_t = unsafe extern "system" fn(face: MaterialFace, mode: PolygonMode);

/// [glPolygonOffset](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffset.xhtml)
pub type glPolygonOffset_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);

/// [glPolygonOffsetClamp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffsetClamp.xhtml)
pub type glPolygonOffsetClamp_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);

/// [glPolygonOffsetClampEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffsetClampEXT.xhtml)
pub type glPolygonOffsetClampEXT_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);

/// [glPolygonOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffsetEXT.xhtml)
pub type glPolygonOffsetEXT_t = unsafe extern "system" fn(factor: GLfloat, bias: GLfloat);

/// [glPolygonOffsetx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffsetx.xhtml)
pub type glPolygonOffsetx_t = unsafe extern "system" fn(factor: GLfixed, units: GLfixed);

/// [glPolygonOffsetxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffsetxOES.xhtml)
pub type glPolygonOffsetxOES_t = unsafe extern "system" fn(factor: GLfixed, units: GLfixed);

/// [glPolygonStipple](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonStipple.xhtml)
/// * `mask` len: COMPSIZE()
pub type glPolygonStipple_t = unsafe extern "system" fn(mask: *const GLubyte);

/// [glPopAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopAttrib.xhtml)
pub type glPopAttrib_t = unsafe extern "system" fn();

/// [glPopClientAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopClientAttrib.xhtml)
pub type glPopClientAttrib_t = unsafe extern "system" fn();

/// [glPopDebugGroup](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopDebugGroup.xhtml)
pub type glPopDebugGroup_t = unsafe extern "system" fn();

/// [glPopDebugGroupKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopDebugGroupKHR.xhtml)
pub type glPopDebugGroupKHR_t = unsafe extern "system" fn();

/// [glPopGroupMarkerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopGroupMarkerEXT.xhtml)
pub type glPopGroupMarkerEXT_t = unsafe extern "system" fn();

/// [glPopMatrix](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopMatrix.xhtml)
pub type glPopMatrix_t = unsafe extern "system" fn();

/// [glPopName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopName.xhtml)
pub type glPopName_t = unsafe extern "system" fn();

/// [glPresentFrameDualFillNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPresentFrameDualFillNV.xhtml)
pub type glPresentFrameDualFillNV_t = unsafe extern "system" fn(video_slot: GLuint, minPresentTime: GLuint64EXT, beginPresentTimeId: GLuint, presentDurationId: GLuint, type_: GLenum, target0: GLenum, fill0: GLuint, target1: GLenum, fill1: GLuint, target2: GLenum, fill2: GLuint, target3: GLenum, fill3: GLuint);

/// [glPresentFrameKeyedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPresentFrameKeyedNV.xhtml)
pub type glPresentFrameKeyedNV_t = unsafe extern "system" fn(video_slot: GLuint, minPresentTime: GLuint64EXT, beginPresentTimeId: GLuint, presentDurationId: GLuint, type_: GLenum, target0: GLenum, fill0: GLuint, key0: GLuint, target1: GLenum, fill1: GLuint, key1: GLuint);

/// [glPrimitiveBoundingBox](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveBoundingBox.xhtml)
pub type glPrimitiveBoundingBox_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

/// [glPrimitiveBoundingBoxARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveBoundingBoxARB.xhtml)
pub type glPrimitiveBoundingBoxARB_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

/// [glPrimitiveBoundingBoxEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveBoundingBoxEXT.xhtml)
pub type glPrimitiveBoundingBoxEXT_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

/// [glPrimitiveBoundingBoxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveBoundingBoxOES.xhtml)
pub type glPrimitiveBoundingBoxOES_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

/// [glPrimitiveRestartIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveRestartIndex.xhtml)
pub type glPrimitiveRestartIndex_t = unsafe extern "system" fn(index: GLuint);

/// [glPrimitiveRestartIndexNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveRestartIndexNV.xhtml)
pub type glPrimitiveRestartIndexNV_t = unsafe extern "system" fn(index: GLuint);

/// [glPrimitiveRestartNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveRestartNV.xhtml)
pub type glPrimitiveRestartNV_t = unsafe extern "system" fn();

/// [glPrioritizeTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrioritizeTextures.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
/// * `priorities` len: n
pub type glPrioritizeTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfloat);

/// [glPrioritizeTexturesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrioritizeTexturesEXT.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
/// * `priorities` group: ClampedFloat32
/// * `priorities` len: n
pub type glPrioritizeTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLclampf);

/// [glPrioritizeTexturesxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrioritizeTexturesxOES.xhtml)
/// * `textures` class: texture
/// * `textures` len: n
/// * `priorities` group: ClampedFixed
/// * `priorities` len: n
pub type glPrioritizeTexturesxOES_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfixed);

/// [glProgramBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramBinary.xhtml)
/// * `program` class: program
/// * `binary` len: length
pub type glProgramBinary_t = unsafe extern "system" fn(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLsizei);

/// [glProgramBinaryOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramBinaryOES.xhtml)
/// * `program` class: program
/// * `binary` len: length
pub type glProgramBinaryOES_t = unsafe extern "system" fn(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLint);

/// [glProgramBufferParametersIivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramBufferParametersIivNV.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count
pub type glProgramBufferParametersIivNV_t = unsafe extern "system" fn(target: ProgramTarget, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLint);

/// [glProgramBufferParametersIuivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramBufferParametersIuivNV.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count
pub type glProgramBufferParametersIuivNV_t = unsafe extern "system" fn(target: ProgramTarget, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLuint);

/// [glProgramBufferParametersfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramBufferParametersfvNV.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count
pub type glProgramBufferParametersfvNV_t = unsafe extern "system" fn(target: ProgramTarget, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLfloat);

/// [glProgramEnvParameter4dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameter4dARB.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramEnvParameter4dARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glProgramEnvParameter4dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameter4dvARB.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramEnvParameter4dvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLdouble; 4]);

/// [glProgramEnvParameter4fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameter4fARB.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramEnvParameter4fARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glProgramEnvParameter4fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameter4fvARB.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramEnvParameter4fvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLfloat; 4]);

/// [glProgramEnvParameterI4iNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameterI4iNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramEnvParameterI4iNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// [glProgramEnvParameterI4ivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameterI4ivNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramEnvParameterI4ivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLint; 4]);

/// [glProgramEnvParameterI4uiNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameterI4uiNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramEnvParameterI4uiNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// [glProgramEnvParameterI4uivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameterI4uivNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramEnvParameterI4uivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLuint; 4]);

/// [glProgramEnvParameters4fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParameters4fvEXT.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramEnvParameters4fvEXT_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLfloat);

/// [glProgramEnvParametersI4ivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParametersI4ivNV.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramEnvParametersI4ivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLint);

/// [glProgramEnvParametersI4uivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramEnvParametersI4uivNV.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramEnvParametersI4uivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLuint);

/// [glProgramLocalParameter4dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameter4dARB.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramLocalParameter4dARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glProgramLocalParameter4dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameter4dvARB.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramLocalParameter4dvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLdouble; 4]);

/// [glProgramLocalParameter4fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameter4fARB.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramLocalParameter4fARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glProgramLocalParameter4fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameter4fvARB.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramLocalParameter4fvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLfloat; 4]);

/// [glProgramLocalParameterI4iNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameterI4iNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramLocalParameterI4iNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// [glProgramLocalParameterI4ivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameterI4ivNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramLocalParameterI4ivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLint; 4]);

/// [glProgramLocalParameterI4uiNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameterI4uiNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramLocalParameterI4uiNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// [glProgramLocalParameterI4uivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameterI4uivNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramLocalParameterI4uivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLuint; 4]);

/// [glProgramLocalParameters4fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParameters4fvEXT.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramLocalParameters4fvEXT_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLfloat);

/// [glProgramLocalParametersI4ivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParametersI4ivNV.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramLocalParametersI4ivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLint);

/// [glProgramLocalParametersI4uivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramLocalParametersI4uivNV.xhtml)
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramLocalParametersI4uivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLuint);

/// [glProgramNamedParameter4dNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramNamedParameter4dNV.xhtml)
/// * `id` class: program
pub type glProgramNamedParameter4dNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glProgramNamedParameter4dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramNamedParameter4dvNV.xhtml)
/// * `id` class: program
pub type glProgramNamedParameter4dvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, v: *const [GLdouble; 4]);

/// [glProgramNamedParameter4fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramNamedParameter4fNV.xhtml)
/// * `id` class: program
pub type glProgramNamedParameter4fNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glProgramNamedParameter4fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramNamedParameter4fvNV.xhtml)
/// * `id` class: program
pub type glProgramNamedParameter4fvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, v: *const [GLfloat; 4]);

/// [glProgramParameter4dNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameter4dNV.xhtml)
/// * `target` group: VertexAttribEnumNV
pub type glProgramParameter4dNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glProgramParameter4dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameter4dvNV.xhtml)
/// * `target` group: VertexAttribEnumNV
pub type glProgramParameter4dvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, v: *const [GLdouble; 4]);

/// [glProgramParameter4fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameter4fNV.xhtml)
/// * `target` group: VertexAttribEnumNV
pub type glProgramParameter4fNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glProgramParameter4fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameter4fvNV.xhtml)
/// * `target` group: VertexAttribEnumNV
pub type glProgramParameter4fvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, v: *const [GLfloat; 4]);

/// [glProgramParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameteri.xhtml)
/// * `program` class: program
/// * `pname` group: ProgramParameterPName
pub type glProgramParameteri_t = unsafe extern "system" fn(program: GLuint, pname: ProgramParameterPName, value: GLint);

/// [glProgramParameteriARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameteriARB.xhtml)
/// * `program` class: program
/// * `pname` group: ProgramParameterPName
pub type glProgramParameteriARB_t = unsafe extern "system" fn(program: GLuint, pname: ProgramParameterPName, value: GLint);

/// [glProgramParameteriEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameteriEXT.xhtml)
/// * `program` class: program
/// * `pname` group: ProgramParameterPName
pub type glProgramParameteriEXT_t = unsafe extern "system" fn(program: GLuint, pname: ProgramParameterPName, value: GLint);

/// [glProgramParameters4dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameters4dvNV.xhtml)
/// * `target` group: VertexAttribEnumNV
/// * `v` len: count*4
pub type glProgramParameters4dvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, count: GLsizei, v: *const GLdouble);

/// [glProgramParameters4fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameters4fvNV.xhtml)
/// * `target` group: VertexAttribEnumNV
/// * `v` len: count*4
pub type glProgramParameters4fvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, count: GLsizei, v: *const GLfloat);

/// [glProgramPathFragmentInputGenNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramPathFragmentInputGenNV.xhtml)
/// * `program` class: program
pub type glProgramPathFragmentInputGenNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, genMode: GLenum, components: GLint, coeffs: *const GLfloat);

/// [glProgramStringARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramStringARB.xhtml)
/// * `target` group: ProgramTarget
/// * `format` group: ProgramFormat
/// * `string` len: len
pub type glProgramStringARB_t = unsafe extern "system" fn(target: ProgramTarget, format: ProgramFormat, len: GLsizei, string: *const void);

/// [glProgramSubroutineParametersuivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramSubroutineParametersuivNV.xhtml)
/// * `params` len: count
pub type glProgramSubroutineParametersuivNV_t = unsafe extern "system" fn(target: GLenum, count: GLsizei, params: *const GLuint);

/// [glProgramUniform1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1d.xhtml)
/// * `program` class: program
pub type glProgramUniform1d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble);

/// [glProgramUniform1dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1dEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform1dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble);

/// [glProgramUniform1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1dv.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// [glProgramUniform1dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1dvEXT.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// [glProgramUniform1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1f.xhtml)
/// * `program` class: program
pub type glProgramUniform1f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);

/// [glProgramUniform1fEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1fEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform1fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);

/// [glProgramUniform1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1fv.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// [glProgramUniform1fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1fvEXT.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// [glProgramUniform1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1i.xhtml)
/// * `program` class: program
pub type glProgramUniform1i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);

/// [glProgramUniform1i64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1i64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniform1i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64);

/// [glProgramUniform1i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1i64NV.xhtml)
/// * `program` class: program
pub type glProgramUniform1i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT);

/// [glProgramUniform1i64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1i64vARB.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

/// [glProgramUniform1i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1i64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

/// [glProgramUniform1iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1iEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform1iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);

/// [glProgramUniform1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1iv.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// [glProgramUniform1ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1ivEXT.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// [glProgramUniform1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1ui.xhtml)
/// * `program` class: program
pub type glProgramUniform1ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);

/// [glProgramUniform1ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1ui64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniform1ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64);

/// [glProgramUniform1ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1ui64NV.xhtml)
/// * `program` class: program
pub type glProgramUniform1ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT);

/// [glProgramUniform1ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1ui64vARB.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

/// [glProgramUniform1ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1ui64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glProgramUniform1uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1uiEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform1uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);

/// [glProgramUniform1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1uiv.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// [glProgramUniform1uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1uivEXT.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// [glProgramUniform2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2d.xhtml)
/// * `program` class: program
pub type glProgramUniform2d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);

/// [glProgramUniform2dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2dEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform2dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble);

/// [glProgramUniform2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2dv.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// [glProgramUniform2dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2dvEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// [glProgramUniform2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2f.xhtml)
/// * `program` class: program
pub type glProgramUniform2f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);

/// [glProgramUniform2fEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2fEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform2fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);

/// [glProgramUniform2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2fv.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// [glProgramUniform2fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2fvEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// [glProgramUniform2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2i.xhtml)
/// * `program` class: program
pub type glProgramUniform2i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);

/// [glProgramUniform2i64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2i64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniform2i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64);

/// [glProgramUniform2i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2i64NV.xhtml)
/// * `program` class: program
pub type glProgramUniform2i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT);

/// [glProgramUniform2i64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2i64vARB.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

/// [glProgramUniform2i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2i64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

/// [glProgramUniform2iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2iEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform2iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);

/// [glProgramUniform2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2iv.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// [glProgramUniform2ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2ivEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// [glProgramUniform2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2ui.xhtml)
/// * `program` class: program
pub type glProgramUniform2ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);

/// [glProgramUniform2ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2ui64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniform2ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64);

/// [glProgramUniform2ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2ui64NV.xhtml)
/// * `program` class: program
pub type glProgramUniform2ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT);

/// [glProgramUniform2ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2ui64vARB.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

/// [glProgramUniform2ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2ui64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glProgramUniform2uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2uiEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform2uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);

/// [glProgramUniform2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2uiv.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// [glProgramUniform2uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2uivEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// [glProgramUniform3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3d.xhtml)
/// * `program` class: program
pub type glProgramUniform3d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);

/// [glProgramUniform3dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3dEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform3dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glProgramUniform3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3dv.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// [glProgramUniform3dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3dvEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// [glProgramUniform3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3f.xhtml)
/// * `program` class: program
pub type glProgramUniform3f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

/// [glProgramUniform3fEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3fEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform3fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

/// [glProgramUniform3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3fv.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// [glProgramUniform3fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3fvEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// [glProgramUniform3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3i.xhtml)
/// * `program` class: program
pub type glProgramUniform3i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);

/// [glProgramUniform3i64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3i64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniform3i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64);

/// [glProgramUniform3i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3i64NV.xhtml)
/// * `program` class: program
pub type glProgramUniform3i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

/// [glProgramUniform3i64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3i64vARB.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

/// [glProgramUniform3i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3i64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

/// [glProgramUniform3iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3iEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform3iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);

/// [glProgramUniform3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3iv.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// [glProgramUniform3ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3ivEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// [glProgramUniform3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3ui.xhtml)
/// * `program` class: program
pub type glProgramUniform3ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

/// [glProgramUniform3ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3ui64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniform3ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);

/// [glProgramUniform3ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3ui64NV.xhtml)
/// * `program` class: program
pub type glProgramUniform3ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

/// [glProgramUniform3ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3ui64vARB.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

/// [glProgramUniform3ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3ui64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glProgramUniform3uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3uiEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform3uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

/// [glProgramUniform3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3uiv.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// [glProgramUniform3uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3uivEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// [glProgramUniform4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4d.xhtml)
/// * `program` class: program
pub type glProgramUniform4d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);

/// [glProgramUniform4dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4dEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform4dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glProgramUniform4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4dv.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// [glProgramUniform4dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4dvEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// [glProgramUniform4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4f.xhtml)
/// * `program` class: program
pub type glProgramUniform4f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

/// [glProgramUniform4fEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4fEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform4fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

/// [glProgramUniform4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4fv.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// [glProgramUniform4fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4fvEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// [glProgramUniform4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4i.xhtml)
/// * `program` class: program
pub type glProgramUniform4i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

/// [glProgramUniform4i64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4i64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniform4i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);

/// [glProgramUniform4i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4i64NV.xhtml)
/// * `program` class: program
pub type glProgramUniform4i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

/// [glProgramUniform4i64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4i64vARB.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

/// [glProgramUniform4i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4i64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

/// [glProgramUniform4iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4iEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform4iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

/// [glProgramUniform4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4iv.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// [glProgramUniform4ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4ivEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// [glProgramUniform4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4ui.xhtml)
/// * `program` class: program
pub type glProgramUniform4ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

/// [glProgramUniform4ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4ui64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniform4ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);

/// [glProgramUniform4ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4ui64NV.xhtml)
/// * `program` class: program
pub type glProgramUniform4ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

/// [glProgramUniform4ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4ui64vARB.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

/// [glProgramUniform4ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4ui64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glProgramUniform4uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4uiEXT.xhtml)
/// * `program` class: program
pub type glProgramUniform4uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

/// [glProgramUniform4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4uiv.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// [glProgramUniform4uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4uivEXT.xhtml)
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// [glProgramUniformHandleui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformHandleui64ARB.xhtml)
/// * `program` class: program
pub type glProgramUniformHandleui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

/// [glProgramUniformHandleui64IMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformHandleui64IMG.xhtml)
/// * `program` class: program
pub type glProgramUniformHandleui64IMG_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

/// [glProgramUniformHandleui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformHandleui64NV.xhtml)
/// * `program` class: program
pub type glProgramUniformHandleui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

/// [glProgramUniformHandleui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformHandleui64vARB.xhtml)
/// * `program` class: program
/// * `values` len: count
pub type glProgramUniformHandleui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

/// [glProgramUniformHandleui64vIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformHandleui64vIMG.xhtml)
/// * `program` class: program
/// * `values` len: count
pub type glProgramUniformHandleui64vIMG_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

/// [glProgramUniformHandleui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformHandleui64vNV.xhtml)
/// * `program` class: program
/// * `values` len: count
pub type glProgramUniformHandleui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

/// [glProgramUniformMatrix2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glProgramUniformMatrix2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix2dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glProgramUniformMatrix2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glProgramUniformMatrix2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix2fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glProgramUniformMatrix2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix2x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x3dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix2x3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix2x3dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x3dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix2x3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix2x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x3fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix2x3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix2x3fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x3fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix2x3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix2x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x4dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix2x4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix2x4dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x4dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix2x4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix2x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x4fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix2x4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix2x4fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x4fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix2x4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glProgramUniformMatrix3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix3dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glProgramUniformMatrix3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glProgramUniformMatrix3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix3fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glProgramUniformMatrix3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix3x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x2dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix3x2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix3x2dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x2dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix3x2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix3x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x2fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix3x2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix3x2fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x2fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix3x2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix3x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x4dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix3x4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix3x4dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x4dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix3x4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix3x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x4fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix3x4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix3x4fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x4fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix3x4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glProgramUniformMatrix4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix4dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glProgramUniformMatrix4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glProgramUniformMatrix4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix4fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glProgramUniformMatrix4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix4x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x2dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix4x2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix4x2dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x2dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix4x2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix4x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x2fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix4x2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix4x2fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x2fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix4x2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix4x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x3dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix4x3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix4x3dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x3dvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix4x3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glProgramUniformMatrix4x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x3fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix4x3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformMatrix4x3fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x3fvEXT.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix4x3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glProgramUniformui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformui64NV.xhtml)
/// * `program` class: program
pub type glProgramUniformui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64EXT);

/// [glProgramUniformui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformui64vNV.xhtml)
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniformui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glProgramVertexLimitNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramVertexLimitNV.xhtml)
/// * `target` group: ProgramTarget
pub type glProgramVertexLimitNV_t = unsafe extern "system" fn(target: ProgramTarget, limit: GLint);

/// [glProvokingVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProvokingVertex.xhtml)
/// * `mode` group: VertexProvokingMode
pub type glProvokingVertex_t = unsafe extern "system" fn(mode: VertexProvokingMode);

/// [glProvokingVertexEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProvokingVertexEXT.xhtml)
/// * `mode` group: VertexProvokingMode
pub type glProvokingVertexEXT_t = unsafe extern "system" fn(mode: VertexProvokingMode);

/// [glPushAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushAttrib.xhtml)
/// * `mask` group: AttribMask
pub type glPushAttrib_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glPushClientAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushClientAttrib.xhtml)
/// * `mask` group: ClientAttribMask
pub type glPushClientAttrib_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glPushClientAttribDefaultEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushClientAttribDefaultEXT.xhtml)
/// * `mask` group: ClientAttribMask
pub type glPushClientAttribDefaultEXT_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glPushDebugGroup](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushDebugGroup.xhtml)
/// * `source` group: DebugSource
/// * `message` len: COMPSIZE(message,length)
pub type glPushDebugGroup_t = unsafe extern "system" fn(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar);

/// [glPushDebugGroupKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushDebugGroupKHR.xhtml)
/// * `source` group: DebugSource
pub type glPushDebugGroupKHR_t = unsafe extern "system" fn(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar);

/// [glPushGroupMarkerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushGroupMarkerEXT.xhtml)
pub type glPushGroupMarkerEXT_t = unsafe extern "system" fn(length: GLsizei, marker: *const GLchar);

/// [glPushMatrix](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushMatrix.xhtml)
pub type glPushMatrix_t = unsafe extern "system" fn();

/// [glPushName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushName.xhtml)
/// * `name` group: SelectName
pub type glPushName_t = unsafe extern "system" fn(name: GLuint);

/// [glQueryCounter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glQueryCounter.xhtml)
/// * `id` class: query
/// * `target` group: QueryCounterTarget
pub type glQueryCounter_t = unsafe extern "system" fn(id: GLuint, target: QueryCounterTarget);

/// [glQueryCounterEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glQueryCounterEXT.xhtml)
/// * `id` class: query
/// * `target` group: QueryCounterTarget
pub type glQueryCounterEXT_t = unsafe extern "system" fn(id: GLuint, target: QueryCounterTarget);

/// [glQueryMatrixxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glQueryMatrixxOES.xhtml)
pub type glQueryMatrixxOES_t = unsafe extern "system" fn(mantissa: *mut [GLfixed; 16], exponent: *mut [GLint; 16]) -> GLbitfield;

/// [glQueryObjectParameteruiAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glQueryObjectParameteruiAMD.xhtml)
/// * `target` group: QueryTarget
/// * `id` class: query
/// * `param` group: OcclusionQueryEventMaskAMD
pub type glQueryObjectParameteruiAMD_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint, pname: GLenum, param: GLuint);

/// [glQueryResourceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glQueryResourceNV.xhtml)
/// * `buffer` len: count
pub type glQueryResourceNV_t = unsafe extern "system" fn(queryType: GLenum, tagId: GLint, count: GLuint, buffer: *mut GLint) -> GLint;

/// [glQueryResourceTagNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glQueryResourceTagNV.xhtml)
pub type glQueryResourceTagNV_t = unsafe extern "system" fn(tagId: GLint, tagString: *const GLchar);

/// [glRasterPos2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2d.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glRasterPos2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// [glRasterPos2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2dv.xhtml)
/// * `v` group: CoordD
pub type glRasterPos2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// [glRasterPos2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2f.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glRasterPos2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// [glRasterPos2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2fv.xhtml)
/// * `v` group: CoordF
pub type glRasterPos2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// [glRasterPos2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2i.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glRasterPos2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// [glRasterPos2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2iv.xhtml)
/// * `v` group: CoordI
pub type glRasterPos2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// [glRasterPos2s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2s.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glRasterPos2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// [glRasterPos2sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2sv.xhtml)
/// * `v` group: CoordS
pub type glRasterPos2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// [glRasterPos2xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2xOES.xhtml)
pub type glRasterPos2xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed);

/// [glRasterPos2xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos2xvOES.xhtml)
pub type glRasterPos2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

/// [glRasterPos3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3d.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glRasterPos3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// [glRasterPos3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3dv.xhtml)
/// * `v` group: CoordD
pub type glRasterPos3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glRasterPos3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3f.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glRasterPos3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// [glRasterPos3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3fv.xhtml)
/// * `v` group: CoordF
pub type glRasterPos3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glRasterPos3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3i.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glRasterPos3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// [glRasterPos3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3iv.xhtml)
/// * `v` group: CoordI
pub type glRasterPos3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glRasterPos3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3s.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glRasterPos3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// [glRasterPos3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3sv.xhtml)
/// * `v` group: CoordS
pub type glRasterPos3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glRasterPos3xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3xOES.xhtml)
pub type glRasterPos3xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// [glRasterPos3xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos3xvOES.xhtml)
pub type glRasterPos3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

/// [glRasterPos4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4d.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
/// * `w` group: CoordD
pub type glRasterPos4d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glRasterPos4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4dv.xhtml)
/// * `v` group: CoordD
pub type glRasterPos4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// [glRasterPos4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4f.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
/// * `w` group: CoordF
pub type glRasterPos4f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glRasterPos4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4fv.xhtml)
/// * `v` group: CoordF
pub type glRasterPos4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// [glRasterPos4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4i.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
/// * `w` group: CoordI
pub type glRasterPos4i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

/// [glRasterPos4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4iv.xhtml)
/// * `v` group: CoordI
pub type glRasterPos4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// [glRasterPos4s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4s.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
/// * `w` group: CoordS
pub type glRasterPos4s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// [glRasterPos4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4sv.xhtml)
/// * `v` group: CoordS
pub type glRasterPos4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// [glRasterPos4xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4xOES.xhtml)
pub type glRasterPos4xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed, w: GLfixed);

/// [glRasterPos4xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterPos4xvOES.xhtml)
pub type glRasterPos4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

/// [glRasterSamplesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRasterSamplesEXT.xhtml)
/// * `fixedsamplelocations` group: Boolean
pub type glRasterSamplesEXT_t = unsafe extern "system" fn(samples: GLuint, fixedsamplelocations: GLboolean);

/// [glReadBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadBuffer.xhtml)
/// * `src` group: ReadBufferMode
pub type glReadBuffer_t = unsafe extern "system" fn(src: ReadBufferMode);

/// [glReadBufferIndexedEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadBufferIndexedEXT.xhtml)
/// * `src` group: ReadBufferMode
pub type glReadBufferIndexedEXT_t = unsafe extern "system" fn(src: ReadBufferMode, index: GLint);

/// [glReadBufferNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadBufferNV.xhtml)
pub type glReadBufferNV_t = unsafe extern "system" fn(mode: GLenum);

/// [glReadInstrumentsSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadInstrumentsSGIX.xhtml)
pub type glReadInstrumentsSGIX_t = unsafe extern "system" fn(marker: GLint);

/// [glReadPixels](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadPixels.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glReadPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void);

/// [glReadnPixels](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadnPixels.xhtml)
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
pub type glReadnPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

/// [glReadnPixelsARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadnPixelsARB.xhtml)
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
pub type glReadnPixelsARB_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

/// [glReadnPixelsEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadnPixelsEXT.xhtml)
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
pub type glReadnPixelsEXT_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

/// [glReadnPixelsKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadnPixelsKHR.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
pub type glReadnPixelsKHR_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

/// [glRectd](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRectd.xhtml)
/// * `x1` group: CoordD
/// * `y1` group: CoordD
/// * `x2` group: CoordD
/// * `y2` group: CoordD
pub type glRectd_t = unsafe extern "system" fn(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble);

/// [glRectdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRectdv.xhtml)
/// * `v1` group: CoordD
/// * `v2` group: CoordD
pub type glRectdv_t = unsafe extern "system" fn(v1: *const [GLdouble; 2], v2: *const [GLdouble; 2]);

/// [glRectf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRectf.xhtml)
/// * `x1` group: CoordF
/// * `y1` group: CoordF
/// * `x2` group: CoordF
/// * `y2` group: CoordF
pub type glRectf_t = unsafe extern "system" fn(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat);

/// [glRectfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRectfv.xhtml)
/// * `v1` group: CoordF
/// * `v2` group: CoordF
pub type glRectfv_t = unsafe extern "system" fn(v1: *const [GLfloat; 2], v2: *const [GLfloat; 2]);

/// [glRecti](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRecti.xhtml)
/// * `x1` group: CoordI
/// * `y1` group: CoordI
/// * `x2` group: CoordI
/// * `y2` group: CoordI
pub type glRecti_t = unsafe extern "system" fn(x1: GLint, y1: GLint, x2: GLint, y2: GLint);

/// [glRectiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRectiv.xhtml)
/// * `v1` group: CoordI
/// * `v2` group: CoordI
pub type glRectiv_t = unsafe extern "system" fn(v1: *const [GLint; 2], v2: *const [GLint; 2]);

/// [glRects](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRects.xhtml)
/// * `x1` group: CoordS
/// * `y1` group: CoordS
/// * `x2` group: CoordS
/// * `y2` group: CoordS
pub type glRects_t = unsafe extern "system" fn(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort);

/// [glRectsv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRectsv.xhtml)
/// * `v1` group: CoordS
/// * `v2` group: CoordS
pub type glRectsv_t = unsafe extern "system" fn(v1: *const [GLshort; 2], v2: *const [GLshort; 2]);

/// [glRectxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRectxOES.xhtml)
pub type glRectxOES_t = unsafe extern "system" fn(x1: GLfixed, y1: GLfixed, x2: GLfixed, y2: GLfixed);

/// [glRectxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRectxvOES.xhtml)
pub type glRectxvOES_t = unsafe extern "system" fn(v1: *const [GLfixed; 2], v2: *const [GLfixed; 2]);

/// [glReferencePlaneSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReferencePlaneSGIX.xhtml)
pub type glReferencePlaneSGIX_t = unsafe extern "system" fn(equation: *const [GLdouble; 4]);

/// [glReleaseKeyedMutexWin32EXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReleaseKeyedMutexWin32EXT.xhtml)
pub type glReleaseKeyedMutexWin32EXT_t = unsafe extern "system" fn(memory: GLuint, key: GLuint64) -> GLboolean;

/// [glReleaseShaderCompiler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReleaseShaderCompiler.xhtml)
pub type glReleaseShaderCompiler_t = unsafe extern "system" fn();

/// [glRenderGpuMaskNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderGpuMaskNV.xhtml)
pub type glRenderGpuMaskNV_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glRenderMode](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderMode.xhtml)
/// * `mode` group: RenderingMode
pub type glRenderMode_t = unsafe extern "system" fn(mode: RenderingMode) -> GLint;

/// [glRenderbufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorage.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorage_t = unsafe extern "system" fn(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageEXT.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageEXT_t = unsafe extern "system" fn(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisample.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisample_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageMultisampleANGLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisampleANGLE.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleANGLE_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageMultisampleAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisampleAPPLE.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleAPPLE_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageMultisampleAdvancedAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisampleAdvancedAMD.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleAdvancedAMD_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, storageSamples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageMultisampleCoverageNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisampleCoverageNV.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleCoverageNV_t = unsafe extern "system" fn(target: RenderbufferTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisampleEXT.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleEXT_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageMultisampleIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisampleIMG.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleIMG_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageMultisampleNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisampleNV.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleNV_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glRenderbufferStorageOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageOES.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageOES_t = unsafe extern "system" fn(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glReplacementCodePointerSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodePointerSUN.xhtml)
/// * `type` group: ReplacementCodeTypeSUN
/// * `pointer` len: COMPSIZE(type,stride)
pub type glReplacementCodePointerSUN_t = unsafe extern "system" fn(type_: ReplacementCodeTypeSUN, stride: GLsizei, pointer: *const *mut void);

/// [glReplacementCodeubSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeubSUN.xhtml)
pub type glReplacementCodeubSUN_t = unsafe extern "system" fn(code: GLubyte);

/// [glReplacementCodeubvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeubvSUN.xhtml)
/// * `code` len: COMPSIZE()
pub type glReplacementCodeubvSUN_t = unsafe extern "system" fn(code: *const GLubyte);

/// [glReplacementCodeuiColor3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiColor3fVertex3fSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glReplacementCodeuiColor3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiColor3fVertex3fvSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glReplacementCodeuiColor4fNormal3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiColor4fNormal3fVertex3fSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glReplacementCodeuiColor4fNormal3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiColor4fNormal3fVertex3fvSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glReplacementCodeuiColor4ubVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiColor4ubVertex3fSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor4ubVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glReplacementCodeuiColor4ubVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiColor4ubVertex3fvSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor4ubVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

/// [glReplacementCodeuiNormal3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiNormal3fVertex3fSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glReplacementCodeuiNormal3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiNormal3fVertex3fvSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glReplacementCodeuiSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiSUN.xhtml)
pub type glReplacementCodeuiSUN_t = unsafe extern "system" fn(code: GLuint);

/// [glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glReplacementCodeuiTexCoord2fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiTexCoord2fVertex3fSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glReplacementCodeuiTexCoord2fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiTexCoord2fVertex3fvSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], v: *const [GLfloat; 3]);

/// [glReplacementCodeuiVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiVertex3fSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glReplacementCodeuiVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuiVertex3fvSUN.xhtml)
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, v: *const [GLfloat; 3]);

/// [glReplacementCodeuivSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeuivSUN.xhtml)
/// * `code` len: COMPSIZE()
pub type glReplacementCodeuivSUN_t = unsafe extern "system" fn(code: *const GLuint);

/// [glReplacementCodeusSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeusSUN.xhtml)
pub type glReplacementCodeusSUN_t = unsafe extern "system" fn(code: GLushort);

/// [glReplacementCodeusvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReplacementCodeusvSUN.xhtml)
/// * `code` len: COMPSIZE()
pub type glReplacementCodeusvSUN_t = unsafe extern "system" fn(code: *const GLushort);

/// [glRequestResidentProgramsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRequestResidentProgramsNV.xhtml)
/// * `programs` class: program
/// * `programs` len: n
pub type glRequestResidentProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

/// [glResetHistogram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResetHistogram.xhtml)
/// * `target` group: HistogramTargetEXT
pub type glResetHistogram_t = unsafe extern "system" fn(target: HistogramTargetEXT);

/// [glResetHistogramEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResetHistogramEXT.xhtml)
/// * `target` group: HistogramTargetEXT
pub type glResetHistogramEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT);

/// [glResetMemoryObjectParameterNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResetMemoryObjectParameterNV.xhtml)
pub type glResetMemoryObjectParameterNV_t = unsafe extern "system" fn(memory: GLuint, pname: GLenum);

/// [glResetMinmax](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResetMinmax.xhtml)
/// * `target` group: MinmaxTargetEXT
pub type glResetMinmax_t = unsafe extern "system" fn(target: MinmaxTargetEXT);

/// [glResetMinmaxEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResetMinmaxEXT.xhtml)
/// * `target` group: MinmaxTargetEXT
pub type glResetMinmaxEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT);

/// [glResizeBuffersMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResizeBuffersMESA.xhtml)
pub type glResizeBuffersMESA_t = unsafe extern "system" fn();

/// [glResolveDepthValuesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResolveDepthValuesNV.xhtml)
pub type glResolveDepthValuesNV_t = unsafe extern "system" fn();

/// [glResolveMultisampleFramebufferAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResolveMultisampleFramebufferAPPLE.xhtml)
pub type glResolveMultisampleFramebufferAPPLE_t = unsafe extern "system" fn();

/// [glResumeTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResumeTransformFeedback.xhtml)
pub type glResumeTransformFeedback_t = unsafe extern "system" fn();

/// [glResumeTransformFeedbackNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResumeTransformFeedbackNV.xhtml)
pub type glResumeTransformFeedbackNV_t = unsafe extern "system" fn();

/// [glRotated](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRotated.xhtml)
pub type glRotated_t = unsafe extern "system" fn(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glRotatef](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRotatef.xhtml)
pub type glRotatef_t = unsafe extern "system" fn(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glRotatex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRotatex.xhtml)
pub type glRotatex_t = unsafe extern "system" fn(angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);

/// [glRotatexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRotatexOES.xhtml)
pub type glRotatexOES_t = unsafe extern "system" fn(angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);

/// [glSampleCoverage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleCoverage.xhtml)
/// * `invert` group: Boolean
pub type glSampleCoverage_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);

/// [glSampleCoverageARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleCoverageARB.xhtml)
/// * `invert` group: Boolean
pub type glSampleCoverageARB_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);

/// [glSampleCoveragex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleCoveragex.xhtml)
/// * `invert` group: Boolean
pub type glSampleCoveragex_t = unsafe extern "system" fn(value: GLclampx, invert: GLboolean);

/// [glSampleCoveragexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleCoveragexOES.xhtml)
/// * `invert` group: Boolean
pub type glSampleCoveragexOES_t = unsafe extern "system" fn(value: GLclampx, invert: GLboolean);

/// [glSampleMapATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleMapATI.xhtml)
/// * `swizzle` group: SwizzleOpATI
pub type glSampleMapATI_t = unsafe extern "system" fn(dst: GLuint, interp: GLuint, swizzle: SwizzleOpATI);

/// [glSampleMaskEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleMaskEXT.xhtml)
/// * `value` group: ClampedFloat32
/// * `invert` group: Boolean
pub type glSampleMaskEXT_t = unsafe extern "system" fn(value: GLclampf, invert: GLboolean);

/// [glSampleMaskIndexedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleMaskIndexedNV.xhtml)
/// * `mask` group: SampleMaskNV
pub type glSampleMaskIndexedNV_t = unsafe extern "system" fn(index: GLuint, mask: GLbitfield);

/// [glSampleMaskSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleMaskSGIS.xhtml)
/// * `value` group: ClampedFloat32
/// * `invert` group: Boolean
pub type glSampleMaskSGIS_t = unsafe extern "system" fn(value: GLclampf, invert: GLboolean);

/// [glSampleMaski](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleMaski.xhtml)
pub type glSampleMaski_t = unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield);

/// [glSamplePatternEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplePatternEXT.xhtml)
/// * `pattern` group: SamplePatternEXT
pub type glSamplePatternEXT_t = unsafe extern "system" fn(pattern: SamplePatternEXT);

/// [glSamplePatternSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplePatternSGIS.xhtml)
/// * `pattern` group: SamplePatternSGIS
pub type glSamplePatternSGIS_t = unsafe extern "system" fn(pattern: SamplePatternSGIS);

/// [glSamplerParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIiv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

/// [glSamplerParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIivEXT.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

/// [glSamplerParameterIivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIivOES.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

/// [glSamplerParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIuiv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);

/// [glSamplerParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIuivEXT.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIuivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);

/// [glSamplerParameterIuivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIuivOES.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIuivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);

/// [glSamplerParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterf.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
pub type glSamplerParameterf_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, param: GLfloat);

/// [glSamplerParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterfv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat);

/// [glSamplerParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameteri.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
pub type glSamplerParameteri_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: GLint);

/// [glSamplerParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameteriv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

/// [glScaled](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScaled.xhtml)
pub type glScaled_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// [glScalef](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScalef.xhtml)
pub type glScalef_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// [glScalex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScalex.xhtml)
pub type glScalex_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// [glScalexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScalexOES.xhtml)
pub type glScalexOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// [glScissor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissor.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glScissor_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glScissorArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorArrayv.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glScissorArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

/// [glScissorArrayvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorArrayvNV.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glScissorArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

/// [glScissorArrayvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorArrayvOES.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glScissorArrayvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

/// [glScissorExclusiveArrayvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorExclusiveArrayvNV.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glScissorExclusiveArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

/// [glScissorExclusiveNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorExclusiveNV.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glScissorExclusiveNV_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glScissorIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexed.xhtml)
pub type glScissorIndexed_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

/// [glScissorIndexedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexedNV.xhtml)
pub type glScissorIndexedNV_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

/// [glScissorIndexedOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexedOES.xhtml)
pub type glScissorIndexedOES_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

/// [glScissorIndexedv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexedv.xhtml)
pub type glScissorIndexedv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glScissorIndexedvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexedvNV.xhtml)
pub type glScissorIndexedvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glScissorIndexedvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexedvOES.xhtml)
pub type glScissorIndexedvOES_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glSecondaryColor3b](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3b.xhtml)
/// * `red` group: ColorB
/// * `green` group: ColorB
/// * `blue` group: ColorB
pub type glSecondaryColor3b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

/// [glSecondaryColor3bEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3bEXT.xhtml)
/// * `red` group: ColorB
/// * `green` group: ColorB
/// * `blue` group: ColorB
pub type glSecondaryColor3bEXT_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

/// [glSecondaryColor3bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3bv.xhtml)
/// * `v` group: ColorB
pub type glSecondaryColor3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// [glSecondaryColor3bvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3bvEXT.xhtml)
/// * `v` group: ColorB
pub type glSecondaryColor3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// [glSecondaryColor3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3d.xhtml)
/// * `red` group: ColorD
/// * `green` group: ColorD
/// * `blue` group: ColorD
pub type glSecondaryColor3d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

/// [glSecondaryColor3dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3dEXT.xhtml)
/// * `red` group: ColorD
/// * `green` group: ColorD
/// * `blue` group: ColorD
pub type glSecondaryColor3dEXT_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

/// [glSecondaryColor3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3dv.xhtml)
/// * `v` group: ColorD
pub type glSecondaryColor3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glSecondaryColor3dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3dvEXT.xhtml)
/// * `v` group: ColorD
pub type glSecondaryColor3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glSecondaryColor3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3f.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
pub type glSecondaryColor3f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

/// [glSecondaryColor3fEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3fEXT.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
pub type glSecondaryColor3fEXT_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

/// [glSecondaryColor3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3fv.xhtml)
/// * `v` group: ColorF
pub type glSecondaryColor3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glSecondaryColor3fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3fvEXT.xhtml)
/// * `v` group: ColorF
pub type glSecondaryColor3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glSecondaryColor3hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3hNV.xhtml)
/// * `red` group: Half16NV
/// * `green` group: Half16NV
/// * `blue` group: Half16NV
pub type glSecondaryColor3hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV);

/// [glSecondaryColor3hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3hvNV.xhtml)
/// * `v` group: Half16NV
pub type glSecondaryColor3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// [glSecondaryColor3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3i.xhtml)
/// * `red` group: ColorI
/// * `green` group: ColorI
/// * `blue` group: ColorI
pub type glSecondaryColor3i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

/// [glSecondaryColor3iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3iEXT.xhtml)
/// * `red` group: ColorI
/// * `green` group: ColorI
/// * `blue` group: ColorI
pub type glSecondaryColor3iEXT_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

/// [glSecondaryColor3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3iv.xhtml)
/// * `v` group: ColorI
pub type glSecondaryColor3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glSecondaryColor3ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3ivEXT.xhtml)
/// * `v` group: ColorI
pub type glSecondaryColor3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glSecondaryColor3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3s.xhtml)
/// * `red` group: ColorS
/// * `green` group: ColorS
/// * `blue` group: ColorS
pub type glSecondaryColor3s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

/// [glSecondaryColor3sEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3sEXT.xhtml)
/// * `red` group: ColorS
/// * `green` group: ColorS
/// * `blue` group: ColorS
pub type glSecondaryColor3sEXT_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

/// [glSecondaryColor3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3sv.xhtml)
/// * `v` group: ColorS
pub type glSecondaryColor3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glSecondaryColor3svEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3svEXT.xhtml)
/// * `v` group: ColorS
pub type glSecondaryColor3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glSecondaryColor3ub](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3ub.xhtml)
/// * `red` group: ColorUB
/// * `green` group: ColorUB
/// * `blue` group: ColorUB
pub type glSecondaryColor3ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

/// [glSecondaryColor3ubEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3ubEXT.xhtml)
/// * `red` group: ColorUB
/// * `green` group: ColorUB
/// * `blue` group: ColorUB
pub type glSecondaryColor3ubEXT_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

/// [glSecondaryColor3ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3ubv.xhtml)
/// * `v` group: ColorUB
pub type glSecondaryColor3ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

/// [glSecondaryColor3ubvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3ubvEXT.xhtml)
/// * `v` group: ColorUB
pub type glSecondaryColor3ubvEXT_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

/// [glSecondaryColor3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3ui.xhtml)
/// * `red` group: ColorUI
/// * `green` group: ColorUI
/// * `blue` group: ColorUI
pub type glSecondaryColor3ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

/// [glSecondaryColor3uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3uiEXT.xhtml)
/// * `red` group: ColorUI
/// * `green` group: ColorUI
/// * `blue` group: ColorUI
pub type glSecondaryColor3uiEXT_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

/// [glSecondaryColor3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3uiv.xhtml)
/// * `v` group: ColorUI
pub type glSecondaryColor3uiv_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

/// [glSecondaryColor3uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3uivEXT.xhtml)
/// * `v` group: ColorUI
pub type glSecondaryColor3uivEXT_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

/// [glSecondaryColor3us](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3us.xhtml)
/// * `red` group: ColorUS
/// * `green` group: ColorUS
/// * `blue` group: ColorUS
pub type glSecondaryColor3us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

/// [glSecondaryColor3usEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3usEXT.xhtml)
/// * `red` group: ColorUS
/// * `green` group: ColorUS
/// * `blue` group: ColorUS
pub type glSecondaryColor3usEXT_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

/// [glSecondaryColor3usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3usv.xhtml)
/// * `v` group: ColorUS
pub type glSecondaryColor3usv_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

/// [glSecondaryColor3usvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColor3usvEXT.xhtml)
/// * `v` group: ColorUS
pub type glSecondaryColor3usvEXT_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

/// [glSecondaryColorFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorFormatNV.xhtml)
/// * `type` group: ColorPointerType
pub type glSecondaryColorFormatNV_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei);

/// [glSecondaryColorP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorP3ui.xhtml)
/// * `type` group: ColorPointerType
pub type glSecondaryColorP3ui_t = unsafe extern "system" fn(type_: ColorPointerType, color: GLuint);

/// [glSecondaryColorP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorP3uiv.xhtml)
/// * `type` group: ColorPointerType
pub type glSecondaryColorP3uiv_t = unsafe extern "system" fn(type_: ColorPointerType, color: *const GLuint);

/// [glSecondaryColorPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorPointer.xhtml)
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glSecondaryColorPointer_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei, pointer: *const void);

/// [glSecondaryColorPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorPointerEXT.xhtml)
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glSecondaryColorPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei, pointer: *const void);

/// [glSecondaryColorPointerListIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorPointerListIBM.xhtml)
/// * `type` group: SecondaryColorPointerTypeIBM
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glSecondaryColorPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: SecondaryColorPointerTypeIBM, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// [glSelectBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSelectBuffer.xhtml)
/// * `buffer` group: SelectName
/// * `buffer` len: size
pub type glSelectBuffer_t = unsafe extern "system" fn(size: GLsizei, buffer: *mut GLuint);

/// [glSelectPerfMonitorCountersAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSelectPerfMonitorCountersAMD.xhtml)
/// * `enable` group: Boolean
/// * `counterList` len: numCounters
pub type glSelectPerfMonitorCountersAMD_t = unsafe extern "system" fn(monitor: GLuint, enable: GLboolean, group: GLuint, numCounters: GLint, counterList: *mut GLuint);

/// [glSemaphoreParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSemaphoreParameterivNV.xhtml)
/// * `pname` group: SemaphoreParameterName
pub type glSemaphoreParameterivNV_t = unsafe extern "system" fn(semaphore: GLuint, pname: SemaphoreParameterName, params: *const GLint);

/// [glSemaphoreParameterui64vEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSemaphoreParameterui64vEXT.xhtml)
/// * `pname` group: SemaphoreParameterName
pub type glSemaphoreParameterui64vEXT_t = unsafe extern "system" fn(semaphore: GLuint, pname: SemaphoreParameterName, params: *const GLuint64);

/// [glSeparableFilter2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSeparableFilter2D.xhtml)
/// * `target` group: SeparableTargetEXT
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: COMPSIZE(target,format,type,width)
/// * `column` len: COMPSIZE(target,format,type,height)
pub type glSeparableFilter2D_t = unsafe extern "system" fn(target: SeparableTargetEXT, internalformat: InternalFormat, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, row: *const void, column: *const void);

/// [glSeparableFilter2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSeparableFilter2DEXT.xhtml)
/// * `target` group: SeparableTargetEXT
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: COMPSIZE(target,format,type,width)
/// * `column` len: COMPSIZE(target,format,type,height)
pub type glSeparableFilter2DEXT_t = unsafe extern "system" fn(target: SeparableTargetEXT, internalformat: InternalFormat, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, row: *const void, column: *const void);

/// [glSetFenceAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSetFenceAPPLE.xhtml)
/// * `fence` group: FenceNV
pub type glSetFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint);

/// [glSetFenceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSetFenceNV.xhtml)
/// * `fence` group: FenceNV
/// * `condition` group: FenceConditionNV
pub type glSetFenceNV_t = unsafe extern "system" fn(fence: GLuint, condition: FenceConditionNV);

/// [glSetFragmentShaderConstantATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSetFragmentShaderConstantATI.xhtml)
pub type glSetFragmentShaderConstantATI_t = unsafe extern "system" fn(dst: GLuint, value: *const [GLfloat; 4]);

/// [glSetInvariantEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSetInvariantEXT.xhtml)
/// * `type` group: ScalarType
/// * `addr` len: COMPSIZE(id,type)
pub type glSetInvariantEXT_t = unsafe extern "system" fn(id: GLuint, type_: ScalarType, addr: *const void);

/// [glSetLocalConstantEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSetLocalConstantEXT.xhtml)
/// * `type` group: ScalarType
/// * `addr` len: COMPSIZE(id,type)
pub type glSetLocalConstantEXT_t = unsafe extern "system" fn(id: GLuint, type_: ScalarType, addr: *const void);

/// [glSetMultisamplefvAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSetMultisamplefvAMD.xhtml)
pub type glSetMultisamplefvAMD_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *const [GLfloat; 2]);

/// [glShadeModel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShadeModel.xhtml)
/// * `mode` group: ShadingModel
pub type glShadeModel_t = unsafe extern "system" fn(mode: ShadingModel);

/// [glShaderBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderBinary.xhtml)
/// * `shaders` class: shader
/// * `shaders` len: count
/// * `binaryFormat` group: ShaderBinaryFormat
/// * `binary` len: length
pub type glShaderBinary_t = unsafe extern "system" fn(count: GLsizei, shaders: *const GLuint, binaryFormat: ShaderBinaryFormat, binary: *const void, length: GLsizei);

/// [glShaderOp1EXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderOp1EXT.xhtml)
/// * `op` group: VertexShaderOpEXT
pub type glShaderOp1EXT_t = unsafe extern "system" fn(op: VertexShaderOpEXT, res: GLuint, arg1: GLuint);

/// [glShaderOp2EXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderOp2EXT.xhtml)
/// * `op` group: VertexShaderOpEXT
pub type glShaderOp2EXT_t = unsafe extern "system" fn(op: VertexShaderOpEXT, res: GLuint, arg1: GLuint, arg2: GLuint);

/// [glShaderOp3EXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderOp3EXT.xhtml)
/// * `op` group: VertexShaderOpEXT
pub type glShaderOp3EXT_t = unsafe extern "system" fn(op: VertexShaderOpEXT, res: GLuint, arg1: GLuint, arg2: GLuint, arg3: GLuint);

/// [glShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml)
/// * `shader` class: shader
/// * `string` len: count
/// * `length` len: count
pub type glShaderSource_t = unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);

/// [glShaderSourceARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSourceARB.xhtml)
/// * `shaderObj` group: handleARB
/// * `string` len: count
/// * `length` len: count
pub type glShaderSourceARB_t = unsafe extern "system" fn(shaderObj: GLhandleARB, count: GLsizei, string: *const *mut GLcharARB, length: *const GLint);

/// [glShaderStorageBlockBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderStorageBlockBinding.xhtml)
/// * `program` class: program
pub type glShaderStorageBlockBinding_t = unsafe extern "system" fn(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);

/// [glShadingRateImageBarrierNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShadingRateImageBarrierNV.xhtml)
/// * `synchronize` group: Boolean
pub type glShadingRateImageBarrierNV_t = unsafe extern "system" fn(synchronize: GLboolean);

/// [glShadingRateImagePaletteNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShadingRateImagePaletteNV.xhtml)
/// * `rates` len: count
pub type glShadingRateImagePaletteNV_t = unsafe extern "system" fn(viewport: GLuint, first: GLuint, count: GLsizei, rates: *const GLenum);

/// [glShadingRateQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShadingRateQCOM.xhtml)
/// * `rate` group: ShadingRateQCOM
pub type glShadingRateQCOM_t = unsafe extern "system" fn(rate: ShadingRateQCOM);

/// [glShadingRateSampleOrderCustomNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShadingRateSampleOrderCustomNV.xhtml)
/// * `locations` len: COMPSIZE(rate,samples)
pub type glShadingRateSampleOrderCustomNV_t = unsafe extern "system" fn(rate: GLenum, samples: GLuint, locations: *const GLint);

/// [glShadingRateSampleOrderNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShadingRateSampleOrderNV.xhtml)
pub type glShadingRateSampleOrderNV_t = unsafe extern "system" fn(order: GLenum);

/// [glSharpenTexFuncSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSharpenTexFuncSGIS.xhtml)
/// * `target` group: TextureTarget
/// * `points` len: n*2
pub type glSharpenTexFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, n: GLsizei, points: *const GLfloat);

/// [glSignalSemaphoreEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSignalSemaphoreEXT.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: COMPSIZE(numBufferBarriers)
/// * `textures` class: texture
/// * `textures` len: COMPSIZE(numTextureBarriers)
/// * `dstLayouts` group: TextureLayout
/// * `dstLayouts` len: COMPSIZE(numTextureBarriers)
pub type glSignalSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint, numBufferBarriers: GLuint, buffers: *const GLuint, numTextureBarriers: GLuint, textures: *const GLuint, dstLayouts: *const TextureLayout);

/// [glSignalSemaphoreui64NVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSignalSemaphoreui64NVX.xhtml)
/// * `semaphoreArray` len: fenceObjectCount
/// * `fenceValueArray` len: fenceObjectCount
pub type glSignalSemaphoreui64NVX_t = unsafe extern "system" fn(signalGpu: GLuint, fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

/// [glSignalVkFenceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSignalVkFenceNV.xhtml)
pub type glSignalVkFenceNV_t = unsafe extern "system" fn(vkFence: GLuint64);

/// [glSignalVkSemaphoreNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSignalVkSemaphoreNV.xhtml)
pub type glSignalVkSemaphoreNV_t = unsafe extern "system" fn(vkSemaphore: GLuint64);

/// [glSpecializeShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSpecializeShader.xhtml)
/// * `shader` class: shader
pub type glSpecializeShader_t = unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);

/// [glSpecializeShaderARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSpecializeShaderARB.xhtml)
/// * `shader` class: shader
pub type glSpecializeShaderARB_t = unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);

/// [glSpriteParameterfSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSpriteParameterfSGIX.xhtml)
/// * `pname` group: SpriteParameterNameSGIX
/// * `param` group: CheckedFloat32
pub type glSpriteParameterfSGIX_t = unsafe extern "system" fn(pname: SpriteParameterNameSGIX, param: GLfloat);

/// [glSpriteParameterfvSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSpriteParameterfvSGIX.xhtml)
/// * `pname` group: SpriteParameterNameSGIX
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glSpriteParameterfvSGIX_t = unsafe extern "system" fn(pname: SpriteParameterNameSGIX, params: *const GLfloat);

/// [glSpriteParameteriSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSpriteParameteriSGIX.xhtml)
/// * `pname` group: SpriteParameterNameSGIX
/// * `param` group: CheckedInt32
pub type glSpriteParameteriSGIX_t = unsafe extern "system" fn(pname: SpriteParameterNameSGIX, param: GLint);

/// [glSpriteParameterivSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSpriteParameterivSGIX.xhtml)
/// * `pname` group: SpriteParameterNameSGIX
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glSpriteParameterivSGIX_t = unsafe extern "system" fn(pname: SpriteParameterNameSGIX, params: *const GLint);

/// [glStartInstrumentsSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStartInstrumentsSGIX.xhtml)
pub type glStartInstrumentsSGIX_t = unsafe extern "system" fn();

/// [glStartTilingQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStartTilingQCOM.xhtml)
/// * `preserveMask` group: BufferBitQCOM
pub type glStartTilingQCOM_t = unsafe extern "system" fn(x: GLuint, y: GLuint, width: GLuint, height: GLuint, preserveMask: GLbitfield);

/// [glStateCaptureNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStateCaptureNV.xhtml)
pub type glStateCaptureNV_t = unsafe extern "system" fn(state: GLuint, mode: GLenum);

/// [glStencilClearTagEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilClearTagEXT.xhtml)
pub type glStencilClearTagEXT_t = unsafe extern "system" fn(stencilTagBits: GLsizei, stencilClearTag: GLuint);

/// [glStencilFillPathInstancedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFillPathInstancedNV.xhtml)
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `fillMode` group: PathFillMode
/// * `mask` group: MaskedStencilValue
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(numPaths,transformType)
pub type glStencilFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, fillMode: PathFillMode, mask: GLuint, transformType: PathTransformType, transformValues: *const GLfloat);

/// [glStencilFillPathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFillPathNV.xhtml)
/// * `path` group: Path
/// * `fillMode` group: PathFillMode
/// * `mask` group: MaskedStencilValue
pub type glStencilFillPathNV_t = unsafe extern "system" fn(path: GLuint, fillMode: PathFillMode, mask: GLuint);

/// [glStencilFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFunc.xhtml)
/// * `func` group: StencilFunction
/// * `ref` group: StencilValue
/// * `mask` group: MaskedStencilValue
pub type glStencilFunc_t = unsafe extern "system" fn(func: StencilFunction, ref_: GLint, mask: GLuint);

/// [glStencilFuncSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFuncSeparate.xhtml)
/// * `face` group: StencilFaceDirection
/// * `func` group: StencilFunction
/// * `ref` group: StencilValue
/// * `mask` group: MaskedStencilValue
pub type glStencilFuncSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint);

/// [glStencilFuncSeparateATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFuncSeparateATI.xhtml)
/// * `frontfunc` group: StencilFunction
/// * `backfunc` group: StencilFunction
/// * `ref` group: ClampedStencilValue
/// * `mask` group: MaskedStencilValue
pub type glStencilFuncSeparateATI_t = unsafe extern "system" fn(frontfunc: StencilFunction, backfunc: StencilFunction, ref_: GLint, mask: GLuint);

/// [glStencilMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilMask.xhtml)
/// * `mask` group: MaskedStencilValue
pub type glStencilMask_t = unsafe extern "system" fn(mask: GLuint);

/// [glStencilMaskSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilMaskSeparate.xhtml)
/// * `face` group: StencilFaceDirection
/// * `mask` group: MaskedStencilValue
pub type glStencilMaskSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, mask: GLuint);

/// [glStencilOp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilOp.xhtml)
/// * `fail` group: StencilOp
/// * `zfail` group: StencilOp
/// * `zpass` group: StencilOp
pub type glStencilOp_t = unsafe extern "system" fn(fail: StencilOp, zfail: StencilOp, zpass: StencilOp);

/// [glStencilOpSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilOpSeparate.xhtml)
/// * `face` group: StencilFaceDirection
/// * `sfail` group: StencilOp
/// * `dpfail` group: StencilOp
/// * `dppass` group: StencilOp
pub type glStencilOpSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp);

/// [glStencilOpSeparateATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilOpSeparateATI.xhtml)
/// * `face` group: StencilFaceDirection
/// * `sfail` group: StencilOp
/// * `dpfail` group: StencilOp
/// * `dppass` group: StencilOp
pub type glStencilOpSeparateATI_t = unsafe extern "system" fn(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp);

/// [glStencilOpValueAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilOpValueAMD.xhtml)
/// * `face` group: StencilFaceDirection
pub type glStencilOpValueAMD_t = unsafe extern "system" fn(face: StencilFaceDirection, value: GLuint);

/// [glStencilStrokePathInstancedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilStrokePathInstancedNV.xhtml)
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `reference` group: StencilValue
/// * `mask` group: MaskedStencilValue
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(numPaths,transformType)
pub type glStencilStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, reference: GLint, mask: GLuint, transformType: PathTransformType, transformValues: *const GLfloat);

/// [glStencilStrokePathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilStrokePathNV.xhtml)
/// * `path` group: Path
/// * `reference` group: StencilValue
/// * `mask` group: MaskedStencilValue
pub type glStencilStrokePathNV_t = unsafe extern "system" fn(path: GLuint, reference: GLint, mask: GLuint);

/// [glStencilThenCoverFillPathInstancedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilThenCoverFillPathInstancedNV.xhtml)
pub type glStencilThenCoverFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, fillMode: GLenum, mask: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);

/// [glStencilThenCoverFillPathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilThenCoverFillPathNV.xhtml)
pub type glStencilThenCoverFillPathNV_t = unsafe extern "system" fn(path: GLuint, fillMode: GLenum, mask: GLuint, coverMode: GLenum);

/// [glStencilThenCoverStrokePathInstancedNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilThenCoverStrokePathInstancedNV.xhtml)
pub type glStencilThenCoverStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, reference: GLint, mask: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);

/// [glStencilThenCoverStrokePathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilThenCoverStrokePathNV.xhtml)
pub type glStencilThenCoverStrokePathNV_t = unsafe extern "system" fn(path: GLuint, reference: GLint, mask: GLuint, coverMode: GLenum);

/// [glStopInstrumentsSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStopInstrumentsSGIX.xhtml)
pub type glStopInstrumentsSGIX_t = unsafe extern "system" fn(marker: GLint);

/// [glStringMarkerGREMEDY](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStringMarkerGREMEDY.xhtml)
/// * `string` len: len
pub type glStringMarkerGREMEDY_t = unsafe extern "system" fn(len: GLsizei, string: *const void);

/// [glSubpixelPrecisionBiasNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSubpixelPrecisionBiasNV.xhtml)
pub type glSubpixelPrecisionBiasNV_t = unsafe extern "system" fn(xbits: GLuint, ybits: GLuint);

/// [glSwizzleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSwizzleEXT.xhtml)
/// * `outX` group: VertexShaderCoordOutEXT
/// * `outY` group: VertexShaderCoordOutEXT
/// * `outZ` group: VertexShaderCoordOutEXT
/// * `outW` group: VertexShaderCoordOutEXT
pub type glSwizzleEXT_t = unsafe extern "system" fn(res: GLuint, in_: GLuint, outX: VertexShaderCoordOutEXT, outY: VertexShaderCoordOutEXT, outZ: VertexShaderCoordOutEXT, outW: VertexShaderCoordOutEXT);

/// [glSyncTextureINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSyncTextureINTEL.xhtml)
/// * `texture` class: texture
pub type glSyncTextureINTEL_t = unsafe extern "system" fn(texture: GLuint);

/// [glTagSampleBufferSGIX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTagSampleBufferSGIX.xhtml)
pub type glTagSampleBufferSGIX_t = unsafe extern "system" fn();

/// [glTangent3bEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3bEXT.xhtml)
pub type glTangent3bEXT_t = unsafe extern "system" fn(tx: GLbyte, ty: GLbyte, tz: GLbyte);

/// [glTangent3bvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3bvEXT.xhtml)
pub type glTangent3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// [glTangent3dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3dEXT.xhtml)
/// * `tx` group: CoordD
/// * `ty` group: CoordD
/// * `tz` group: CoordD
pub type glTangent3dEXT_t = unsafe extern "system" fn(tx: GLdouble, ty: GLdouble, tz: GLdouble);

/// [glTangent3dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3dvEXT.xhtml)
/// * `v` group: CoordD
pub type glTangent3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glTangent3fEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3fEXT.xhtml)
/// * `tx` group: CoordF
/// * `ty` group: CoordF
/// * `tz` group: CoordF
pub type glTangent3fEXT_t = unsafe extern "system" fn(tx: GLfloat, ty: GLfloat, tz: GLfloat);

/// [glTangent3fvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3fvEXT.xhtml)
/// * `v` group: CoordF
pub type glTangent3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glTangent3iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3iEXT.xhtml)
pub type glTangent3iEXT_t = unsafe extern "system" fn(tx: GLint, ty: GLint, tz: GLint);

/// [glTangent3ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3ivEXT.xhtml)
pub type glTangent3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glTangent3sEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3sEXT.xhtml)
pub type glTangent3sEXT_t = unsafe extern "system" fn(tx: GLshort, ty: GLshort, tz: GLshort);

/// [glTangent3svEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangent3svEXT.xhtml)
pub type glTangent3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glTangentPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTangentPointerEXT.xhtml)
/// * `type` group: TangentPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glTangentPointerEXT_t = unsafe extern "system" fn(type_: TangentPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// [glTbufferMask3DFX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTbufferMask3DFX.xhtml)
pub type glTbufferMask3DFX_t = unsafe extern "system" fn(mask: GLuint);

/// [glTessellationFactorAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTessellationFactorAMD.xhtml)
pub type glTessellationFactorAMD_t = unsafe extern "system" fn(factor: GLfloat);

/// [glTessellationModeAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTessellationModeAMD.xhtml)
pub type glTessellationModeAMD_t = unsafe extern "system" fn(mode: GLenum);

/// [glTestFenceAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTestFenceAPPLE.xhtml)
/// * `fence` group: FenceNV
pub type glTestFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

/// [glTestFenceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTestFenceNV.xhtml)
/// * `fence` group: FenceNV
pub type glTestFenceNV_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

/// [glTestObjectAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTestObjectAPPLE.xhtml)
/// * `object` group: ObjectTypeAPPLE
pub type glTestObjectAPPLE_t = unsafe extern "system" fn(object: ObjectTypeAPPLE, name: GLuint) -> GLboolean;

/// [glTexAttachMemoryNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexAttachMemoryNV.xhtml)
/// * `target` group: TextureTarget
pub type glTexAttachMemoryNV_t = unsafe extern "system" fn(target: TextureTarget, memory: GLuint, offset: GLuint64);

/// [glTexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBuffer.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTexBuffer_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// [glTexBufferARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBufferARB.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTexBufferARB_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// [glTexBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBufferEXT.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTexBufferEXT_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// [glTexBufferOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBufferOES.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTexBufferOES_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// [glTexBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBufferRange.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glTexBufferRange_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glTexBufferRangeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBufferRangeEXT.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glTexBufferRangeEXT_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glTexBufferRangeOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBufferRangeOES.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glTexBufferRangeOES_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glTexBumpParameterfvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBumpParameterfvATI.xhtml)
/// * `pname` group: TexBumpParameterATI
/// * `param` len: COMPSIZE(pname)
pub type glTexBumpParameterfvATI_t = unsafe extern "system" fn(pname: TexBumpParameterATI, param: *const GLfloat);

/// [glTexBumpParameterivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBumpParameterivATI.xhtml)
/// * `pname` group: TexBumpParameterATI
/// * `param` len: COMPSIZE(pname)
pub type glTexBumpParameterivATI_t = unsafe extern "system" fn(pname: TexBumpParameterATI, param: *const GLint);

/// [glTexCoord1bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1bOES.xhtml)
pub type glTexCoord1bOES_t = unsafe extern "system" fn(s: GLbyte);

/// [glTexCoord1bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1bvOES.xhtml)
pub type glTexCoord1bvOES_t = unsafe extern "system" fn(coords: *const GLbyte);

/// [glTexCoord1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1d.xhtml)
/// * `s` group: CoordD
pub type glTexCoord1d_t = unsafe extern "system" fn(s: GLdouble);

/// [glTexCoord1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1dv.xhtml)
/// * `v` group: CoordD
pub type glTexCoord1dv_t = unsafe extern "system" fn(v: *const GLdouble);

/// [glTexCoord1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1f.xhtml)
/// * `s` group: CoordF
pub type glTexCoord1f_t = unsafe extern "system" fn(s: GLfloat);

/// [glTexCoord1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1fv.xhtml)
/// * `v` group: CoordF
pub type glTexCoord1fv_t = unsafe extern "system" fn(v: *const GLfloat);

/// [glTexCoord1hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1hNV.xhtml)
/// * `s` group: Half16NV
pub type glTexCoord1hNV_t = unsafe extern "system" fn(s: GLhalfNV);

/// [glTexCoord1hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1hvNV.xhtml)
/// * `v` group: Half16NV
pub type glTexCoord1hvNV_t = unsafe extern "system" fn(v: *const GLhalfNV);

/// [glTexCoord1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1i.xhtml)
/// * `s` group: CoordI
pub type glTexCoord1i_t = unsafe extern "system" fn(s: GLint);

/// [glTexCoord1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1iv.xhtml)
/// * `v` group: CoordI
pub type glTexCoord1iv_t = unsafe extern "system" fn(v: *const GLint);

/// [glTexCoord1s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1s.xhtml)
/// * `s` group: CoordS
pub type glTexCoord1s_t = unsafe extern "system" fn(s: GLshort);

/// [glTexCoord1sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1sv.xhtml)
/// * `v` group: CoordS
pub type glTexCoord1sv_t = unsafe extern "system" fn(v: *const GLshort);

/// [glTexCoord1xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1xOES.xhtml)
pub type glTexCoord1xOES_t = unsafe extern "system" fn(s: GLfixed);

/// [glTexCoord1xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord1xvOES.xhtml)
pub type glTexCoord1xvOES_t = unsafe extern "system" fn(coords: *const GLfixed);

/// [glTexCoord2bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2bOES.xhtml)
pub type glTexCoord2bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte);

/// [glTexCoord2bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2bvOES.xhtml)
pub type glTexCoord2bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 2]);

/// [glTexCoord2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2d.xhtml)
/// * `s` group: CoordD
/// * `t` group: CoordD
pub type glTexCoord2d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble);

/// [glTexCoord2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2dv.xhtml)
/// * `v` group: CoordD
pub type glTexCoord2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// [glTexCoord2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2f.xhtml)
/// * `s` group: CoordF
/// * `t` group: CoordF
pub type glTexCoord2f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat);

/// [glTexCoord2fColor3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fColor3fVertex3fSUN.xhtml)
pub type glTexCoord2fColor3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glTexCoord2fColor3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fColor3fVertex3fvSUN.xhtml)
pub type glTexCoord2fColor3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glTexCoord2fColor4fNormal3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fColor4fNormal3fVertex3fSUN.xhtml)
pub type glTexCoord2fColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glTexCoord2fColor4fNormal3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fColor4fNormal3fVertex3fvSUN.xhtml)
pub type glTexCoord2fColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glTexCoord2fColor4ubVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fColor4ubVertex3fSUN.xhtml)
pub type glTexCoord2fColor4ubVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glTexCoord2fColor4ubVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fColor4ubVertex3fvSUN.xhtml)
pub type glTexCoord2fColor4ubVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

/// [glTexCoord2fNormal3fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fNormal3fVertex3fSUN.xhtml)
pub type glTexCoord2fNormal3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glTexCoord2fNormal3fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fNormal3fVertex3fvSUN.xhtml)
pub type glTexCoord2fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// [glTexCoord2fVertex3fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fVertex3fSUN.xhtml)
pub type glTexCoord2fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glTexCoord2fVertex3fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fVertex3fvSUN.xhtml)
pub type glTexCoord2fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], v: *const [GLfloat; 3]);

/// [glTexCoord2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2fv.xhtml)
/// * `v` group: CoordF
pub type glTexCoord2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// [glTexCoord2hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2hNV.xhtml)
/// * `s` group: Half16NV
/// * `t` group: Half16NV
pub type glTexCoord2hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV);

/// [glTexCoord2hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2hvNV.xhtml)
/// * `v` group: Half16NV
pub type glTexCoord2hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 2]);

/// [glTexCoord2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2i.xhtml)
/// * `s` group: CoordI
/// * `t` group: CoordI
pub type glTexCoord2i_t = unsafe extern "system" fn(s: GLint, t: GLint);

/// [glTexCoord2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2iv.xhtml)
/// * `v` group: CoordI
pub type glTexCoord2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// [glTexCoord2s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2s.xhtml)
/// * `s` group: CoordS
/// * `t` group: CoordS
pub type glTexCoord2s_t = unsafe extern "system" fn(s: GLshort, t: GLshort);

/// [glTexCoord2sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2sv.xhtml)
/// * `v` group: CoordS
pub type glTexCoord2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// [glTexCoord2xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2xOES.xhtml)
pub type glTexCoord2xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed);

/// [glTexCoord2xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord2xvOES.xhtml)
pub type glTexCoord2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

/// [glTexCoord3bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3bOES.xhtml)
pub type glTexCoord3bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte, r: GLbyte);

/// [glTexCoord3bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3bvOES.xhtml)
pub type glTexCoord3bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 3]);

/// [glTexCoord3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3d.xhtml)
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
pub type glTexCoord3d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble, r: GLdouble);

/// [glTexCoord3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3dv.xhtml)
/// * `v` group: CoordD
pub type glTexCoord3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glTexCoord3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3f.xhtml)
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
pub type glTexCoord3f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat);

/// [glTexCoord3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3fv.xhtml)
/// * `v` group: CoordF
pub type glTexCoord3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glTexCoord3hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3hNV.xhtml)
/// * `s` group: Half16NV
/// * `t` group: Half16NV
/// * `r` group: Half16NV
pub type glTexCoord3hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV, r: GLhalfNV);

/// [glTexCoord3hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3hvNV.xhtml)
/// * `v` group: Half16NV
pub type glTexCoord3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// [glTexCoord3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3i.xhtml)
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
pub type glTexCoord3i_t = unsafe extern "system" fn(s: GLint, t: GLint, r: GLint);

/// [glTexCoord3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3iv.xhtml)
/// * `v` group: CoordI
pub type glTexCoord3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glTexCoord3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3s.xhtml)
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
pub type glTexCoord3s_t = unsafe extern "system" fn(s: GLshort, t: GLshort, r: GLshort);

/// [glTexCoord3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3sv.xhtml)
/// * `v` group: CoordS
pub type glTexCoord3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glTexCoord3xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3xOES.xhtml)
pub type glTexCoord3xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed, r: GLfixed);

/// [glTexCoord3xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord3xvOES.xhtml)
pub type glTexCoord3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

/// [glTexCoord4bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4bOES.xhtml)
pub type glTexCoord4bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte, r: GLbyte, q: GLbyte);

/// [glTexCoord4bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4bvOES.xhtml)
pub type glTexCoord4bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 4]);

/// [glTexCoord4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4d.xhtml)
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
/// * `q` group: CoordD
pub type glTexCoord4d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

/// [glTexCoord4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4dv.xhtml)
/// * `v` group: CoordD
pub type glTexCoord4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// [glTexCoord4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4f.xhtml)
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
/// * `q` group: CoordF
pub type glTexCoord4f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

/// [glTexCoord4fColor4fNormal3fVertex4fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4fColor4fNormal3fVertex4fSUN.xhtml)
pub type glTexCoord4fColor4fNormal3fVertex4fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, p: GLfloat, q: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glTexCoord4fColor4fNormal3fVertex4fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4fColor4fNormal3fVertex4fvSUN.xhtml)
pub type glTexCoord4fColor4fNormal3fVertex4fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 4], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 4]);

/// [glTexCoord4fVertex4fSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4fVertex4fSUN.xhtml)
pub type glTexCoord4fVertex4fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, p: GLfloat, q: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glTexCoord4fVertex4fvSUN](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4fVertex4fvSUN.xhtml)
pub type glTexCoord4fVertex4fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 4], v: *const [GLfloat; 4]);

/// [glTexCoord4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4fv.xhtml)
/// * `v` group: CoordF
pub type glTexCoord4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// [glTexCoord4hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4hNV.xhtml)
/// * `s` group: Half16NV
/// * `t` group: Half16NV
/// * `r` group: Half16NV
/// * `q` group: Half16NV
pub type glTexCoord4hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV, r: GLhalfNV, q: GLhalfNV);

/// [glTexCoord4hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4hvNV.xhtml)
/// * `v` group: Half16NV
pub type glTexCoord4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

/// [glTexCoord4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4i.xhtml)
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
/// * `q` group: CoordI
pub type glTexCoord4i_t = unsafe extern "system" fn(s: GLint, t: GLint, r: GLint, q: GLint);

/// [glTexCoord4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4iv.xhtml)
/// * `v` group: CoordI
pub type glTexCoord4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// [glTexCoord4s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4s.xhtml)
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
/// * `q` group: CoordS
pub type glTexCoord4s_t = unsafe extern "system" fn(s: GLshort, t: GLshort, r: GLshort, q: GLshort);

/// [glTexCoord4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4sv.xhtml)
/// * `v` group: CoordS
pub type glTexCoord4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// [glTexCoord4xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4xOES.xhtml)
pub type glTexCoord4xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

/// [glTexCoord4xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoord4xvOES.xhtml)
pub type glTexCoord4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

/// [glTexCoordFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordFormatNV.xhtml)
pub type glTexCoordFormatNV_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei);

/// [glTexCoordP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP1ui.xhtml)
/// * `type` group: TexCoordPointerType
pub type glTexCoordP1ui_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: GLuint);

/// [glTexCoordP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP1uiv.xhtml)
/// * `type` group: TexCoordPointerType
pub type glTexCoordP1uiv_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: *const GLuint);

/// [glTexCoordP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP2ui.xhtml)
/// * `type` group: TexCoordPointerType
pub type glTexCoordP2ui_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: GLuint);

/// [glTexCoordP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP2uiv.xhtml)
/// * `type` group: TexCoordPointerType
pub type glTexCoordP2uiv_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: *const GLuint);

/// [glTexCoordP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP3ui.xhtml)
/// * `type` group: TexCoordPointerType
pub type glTexCoordP3ui_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: GLuint);

/// [glTexCoordP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP3uiv.xhtml)
/// * `type` group: TexCoordPointerType
pub type glTexCoordP3uiv_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: *const GLuint);

/// [glTexCoordP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP4ui.xhtml)
/// * `type` group: TexCoordPointerType
pub type glTexCoordP4ui_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: GLuint);

/// [glTexCoordP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP4uiv.xhtml)
/// * `type` group: TexCoordPointerType
pub type glTexCoordP4uiv_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: *const GLuint);

/// [glTexCoordPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordPointer.xhtml)
/// * `type` group: TexCoordPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glTexCoordPointer_t = unsafe extern "system" fn(size: GLint, type_: TexCoordPointerType, stride: GLsizei, pointer: *const void);

/// [glTexCoordPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordPointerEXT.xhtml)
/// * `type` group: TexCoordPointerType
/// * `pointer` len: COMPSIZE(size,type,stride,count)
pub type glTexCoordPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: TexCoordPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// [glTexCoordPointerListIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordPointerListIBM.xhtml)
/// * `type` group: TexCoordPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glTexCoordPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: TexCoordPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// [glTexCoordPointervINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordPointervINTEL.xhtml)
/// * `type` group: VertexPointerType
pub type glTexCoordPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, pointer: *const [*mut void; 4]);

/// [glTexEnvf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEnvf.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `param` group: CheckedFloat32
pub type glTexEnvf_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, param: GLfloat);

/// [glTexEnvfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEnvfv.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glTexEnvfv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLfloat);

/// [glTexEnvi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEnvi.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `param` group: CheckedInt32
pub type glTexEnvi_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, param: GLint);

/// [glTexEnviv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEnviv.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTexEnviv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLint);

/// [glTexEnvx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEnvx.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
pub type glTexEnvx_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, param: GLfixed);

/// [glTexEnvxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEnvxOES.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
pub type glTexEnvxOES_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, param: GLfixed);

/// [glTexEnvxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEnvxv.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexEnvxv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLfixed);

/// [glTexEnvxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEnvxvOES.xhtml)
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexEnvxvOES_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLfixed);

/// [glTexEstimateMotionQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEstimateMotionQCOM.xhtml)
/// * `ref` group: Texture
/// * `ref` class: texture
/// * `target` group: Texture
/// * `target` class: texture
/// * `output` group: Texture
/// * `output` class: texture
pub type glTexEstimateMotionQCOM_t = unsafe extern "system" fn(ref_: GLuint, target: GLuint, output: GLuint);

/// [glTexEstimateMotionRegionsQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexEstimateMotionRegionsQCOM.xhtml)
/// * `ref` group: Texture
/// * `ref` class: texture
/// * `target` group: Texture
/// * `target` class: texture
/// * `output` group: Texture
/// * `output` class: texture
/// * `mask` group: Texture
/// * `mask` class: texture
pub type glTexEstimateMotionRegionsQCOM_t = unsafe extern "system" fn(ref_: GLuint, target: GLuint, output: GLuint, mask: GLuint);

/// [glTexFilterFuncSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexFilterFuncSGIS.xhtml)
/// * `target` group: TextureTarget
/// * `filter` group: TextureFilterSGIS
/// * `weights` len: n
pub type glTexFilterFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, filter: TextureFilterSGIS, n: GLsizei, weights: *const GLfloat);

/// [glTexGend](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGend.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glTexGend_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLdouble);

/// [glTexGendv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGendv.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexGendv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLdouble);

/// [glTexGenf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGenf.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `param` group: CheckedFloat32
pub type glTexGenf_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLfloat);

/// [glTexGenfOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGenfOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glTexGenfOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLfloat);

/// [glTexGenfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGenfv.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glTexGenfv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLfloat);

/// [glTexGenfvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGenfvOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexGenfvOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLfloat);

/// [glTexGeni](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGeni.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `param` group: CheckedInt32
pub type glTexGeni_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLint);

/// [glTexGeniOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGeniOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glTexGeniOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLint);

/// [glTexGeniv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGeniv.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTexGeniv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLint);

/// [glTexGenivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGenivOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexGenivOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLint);

/// [glTexGenxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGenxOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glTexGenxOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLfixed);

/// [glTexGenxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexGenxvOES.xhtml)
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexGenxvOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLfixed);

/// [glTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexImage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2DMultisample.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexImage2DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

/// [glTexImage2DMultisampleCoverageNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2DMultisampleCoverageNV.xhtml)
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTexImage2DMultisampleCoverageNV_t = unsafe extern "system" fn(target: TextureTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

/// [glTexImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexImage3DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexImage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3DMultisample.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexImage3DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// [glTexImage3DMultisampleCoverageNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3DMultisampleCoverageNV.xhtml)
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTexImage3DMultisampleCoverageNV_t = unsafe extern "system" fn(target: TextureTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

/// [glTexImage3DOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3DOES.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexImage3DOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexImage4DSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage4DSGIS.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth,size4d)
pub type glTexImage4DSGIS_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, size4d: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexPageCommitmentARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexPageCommitmentARB.xhtml)
/// * `commit` group: Boolean
pub type glTexPageCommitmentARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

/// [glTexPageCommitmentEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexPageCommitmentEXT.xhtml)
/// * `commit` group: Boolean
pub type glTexPageCommitmentEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

/// [glTexPageCommitmentMemNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexPageCommitmentMemNV.xhtml)
/// * `target` group: TextureTarget
/// * `commit` group: Boolean
pub type glTexPageCommitmentMemNV_t = unsafe extern "system" fn(target: TextureTarget, layer: GLint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64, commit: GLboolean);

/// [glTexParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIiv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIiv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// [glTexParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIivEXT.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIivEXT_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// [glTexParameterIivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIivOES.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIivOES_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// [glTexParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIuiv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIuiv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// [glTexParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIuivEXT.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIuivEXT_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// [glTexParameterIuivOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIuivOES.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIuivOES_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// [glTexParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterf.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedFloat32
pub type glTexParameterf_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, param: GLfloat);

/// [glTexParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterfv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterfv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);

/// [glTexParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameteri.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedInt32
pub type glTexParameteri_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, param: GLint);

/// [glTexParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameteriv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTexParameteriv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// [glTexParameterx](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterx.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
pub type glTexParameterx_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, param: GLfixed);

/// [glTexParameterxOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterxOES.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
pub type glTexParameterxOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, param: GLfixed);

/// [glTexParameterxv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterxv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterxv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *const GLfixed);

/// [glTexParameterxvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterxvOES.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterxvOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *const GLfixed);

/// [glTexRenderbufferNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexRenderbufferNV.xhtml)
/// * `target` group: TextureTarget
/// * `renderbuffer` class: renderbuffer
pub type glTexRenderbufferNV_t = unsafe extern "system" fn(target: TextureTarget, renderbuffer: GLuint);

/// [glTexStorage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage1D.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage1D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

/// [glTexStorage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage1DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage1DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

/// [glTexStorage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage2D.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage2D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glTexStorage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage2DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage2DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glTexStorage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage2DMultisample.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexStorage2DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

/// [glTexStorage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage3D.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage3D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

/// [glTexStorage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage3DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage3DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

/// [glTexStorage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage3DMultisample.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexStorage3DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// [glTexStorage3DMultisampleOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage3DMultisampleOES.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexStorage3DMultisampleOES_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// [glTexStorageMem1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorageMem1DEXT.xhtml)
/// * `target` group: TextureTarget
pub type glTexStorageMem1DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalFormat: GLenum, width: GLsizei, memory: GLuint, offset: GLuint64);

/// [glTexStorageMem2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorageMem2DEXT.xhtml)
/// * `target` group: TextureTarget
pub type glTexStorageMem2DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, memory: GLuint, offset: GLuint64);

/// [glTexStorageMem2DMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorageMem2DMultisampleEXT.xhtml)
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTexStorageMem2DMultisampleEXT_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

/// [glTexStorageMem3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorageMem3DEXT.xhtml)
/// * `target` group: TextureTarget
pub type glTexStorageMem3DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64);

/// [glTexStorageMem3DMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorageMem3DMultisampleEXT.xhtml)
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTexStorageMem3DMultisampleEXT_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

/// [glTexStorageSparseAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorageSparseAMD.xhtml)
/// * `target` group: TextureTarget
/// * `internalFormat` group: InternalFormat
/// * `flags` group: TextureStorageMaskAMD
pub type glTexStorageSparseAMD_t = unsafe extern "system" fn(target: TextureTarget, internalFormat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, layers: GLsizei, flags: GLbitfield);

/// [glTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexSubImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage1DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTexSubImage1DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexSubImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage2DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glTexSubImage2DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexSubImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage3DEXT.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexSubImage3DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexSubImage3DOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage3DOES.xhtml)
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexSubImage3DOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTexSubImage4DSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage4DSGIS.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `woffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth,size4d)
pub type glTexSubImage4DSGIS_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, woffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, size4d: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureAttachMemoryNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureAttachMemoryNV.xhtml)
/// * `texture` class: texture
pub type glTextureAttachMemoryNV_t = unsafe extern "system" fn(texture: GLuint, memory: GLuint, offset: GLuint64);

/// [glTextureBarrier](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBarrier.xhtml)
pub type glTextureBarrier_t = unsafe extern "system" fn();

/// [glTextureBarrierNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBarrierNV.xhtml)
pub type glTextureBarrierNV_t = unsafe extern "system" fn();

/// [glTextureBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBuffer.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTextureBuffer_t = unsafe extern "system" fn(texture: GLuint, internalformat: InternalFormat, buffer: GLuint);

/// [glTextureBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBufferEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTextureBufferEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// [glTextureBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBufferRange.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `size` group: BufferSize
pub type glTextureBufferRange_t = unsafe extern "system" fn(texture: GLuint, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glTextureBufferRangeEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBufferRangeEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glTextureBufferRangeEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glTextureColorMaskSGIS](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureColorMaskSGIS.xhtml)
/// * `red` group: Boolean
/// * `green` group: Boolean
/// * `blue` group: Boolean
/// * `alpha` group: Boolean
pub type glTextureColorMaskSGIS_t = unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

/// [glTextureFoveationParametersQCOM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureFoveationParametersQCOM.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `focalX` group: CheckedFloat32
/// * `focalY` group: CheckedFloat32
/// * `gainX` group: CheckedFloat32
/// * `gainY` group: CheckedFloat32
/// * `foveaArea` group: CheckedFloat32
pub type glTextureFoveationParametersQCOM_t = unsafe extern "system" fn(texture: GLuint, layer: GLuint, focalPoint: GLuint, focalX: GLfloat, focalY: GLfloat, gainX: GLfloat, gainY: GLfloat, foveaArea: GLfloat);

/// [glTextureImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureImage1DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTextureImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureImage2DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glTextureImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureImage2DMultisampleCoverageNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureImage2DMultisampleCoverageNV.xhtml)
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTextureImage2DMultisampleCoverageNV_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

/// [glTextureImage2DMultisampleNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureImage2DMultisampleNV.xhtml)
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTextureImage2DMultisampleNV_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, samples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

/// [glTextureImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureImage3DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTextureImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureImage3DMultisampleCoverageNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureImage3DMultisampleCoverageNV.xhtml)
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTextureImage3DMultisampleCoverageNV_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

/// [glTextureImage3DMultisampleNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureImage3DMultisampleNV.xhtml)
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTextureImage3DMultisampleNV_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, samples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

/// [glTextureLightEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureLightEXT.xhtml)
/// * `pname` group: LightTexturePNameEXT
pub type glTextureLightEXT_t = unsafe extern "system" fn(pname: LightTexturePNameEXT);

/// [glTextureMaterialEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureMaterialEXT.xhtml)
/// * `face` group: MaterialFace
/// * `mode` group: MaterialParameter
pub type glTextureMaterialEXT_t = unsafe extern "system" fn(face: MaterialFace, mode: MaterialParameter);

/// [glTextureNormalEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureNormalEXT.xhtml)
/// * `mode` group: TextureNormalModeEXT
pub type glTextureNormalEXT_t = unsafe extern "system" fn(mode: TextureNormalModeEXT);

/// [glTexturePageCommitmentEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexturePageCommitmentEXT.xhtml)
/// * `texture` class: texture
/// * `commit` group: Boolean
pub type glTexturePageCommitmentEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

/// [glTexturePageCommitmentMemNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexturePageCommitmentMemNV.xhtml)
/// * `texture` class: texture
/// * `commit` group: Boolean
pub type glTexturePageCommitmentMemNV_t = unsafe extern "system" fn(texture: GLuint, layer: GLint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64, commit: GLboolean);

/// [glTextureParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterIiv.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameterIiv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, params: *const GLint);

/// [glTextureParameterIivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterIivEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTextureParameterIivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// [glTextureParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterIuiv.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameterIuiv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, params: *const GLuint);

/// [glTextureParameterIuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterIuivEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTextureParameterIuivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// [glTextureParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterf.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameterf_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: GLfloat);

/// [glTextureParameterfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterfEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedFloat32
pub type glTextureParameterfEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, param: GLfloat);

/// [glTextureParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterfv.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameterfv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: *const GLfloat);

/// [glTextureParameterfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterfvEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glTextureParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);

/// [glTextureParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameteri.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameteri_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: GLint);

/// [glTextureParameteriEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameteriEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedInt32
pub type glTextureParameteriEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, param: GLint);

/// [glTextureParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameteriv.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameteriv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: *const GLint);

/// [glTextureParameterivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterivEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTextureParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// [glTextureRangeAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureRangeAPPLE.xhtml)
/// * `pointer` len: length
pub type glTextureRangeAPPLE_t = unsafe extern "system" fn(target: GLenum, length: GLsizei, pointer: *const void);

/// [glTextureRenderbufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureRenderbufferEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `renderbuffer` class: renderbuffer
pub type glTextureRenderbufferEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, renderbuffer: GLuint);

/// [glTextureStorage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage1D.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage1D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

/// [glTextureStorage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage1DEXT.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

/// [glTextureStorage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage2D.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage2D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glTextureStorage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage2DEXT.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// [glTextureStorage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage2DMultisample.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTextureStorage2DMultisample_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

/// [glTextureStorage2DMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage2DMultisampleEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTextureStorage2DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

/// [glTextureStorage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage3D.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage3D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

/// [glTextureStorage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage3DEXT.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

/// [glTextureStorage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage3DMultisample.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTextureStorage3DMultisample_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// [glTextureStorage3DMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage3DMultisampleEXT.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTextureStorage3DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// [glTextureStorageMem1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorageMem1DEXT.xhtml)
/// * `texture` class: texture
pub type glTextureStorageMem1DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, memory: GLuint, offset: GLuint64);

/// [glTextureStorageMem2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorageMem2DEXT.xhtml)
/// * `texture` class: texture
pub type glTextureStorageMem2DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, memory: GLuint, offset: GLuint64);

/// [glTextureStorageMem2DMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorageMem2DMultisampleEXT.xhtml)
/// * `texture` class: texture
/// * `fixedSampleLocations` group: Boolean
pub type glTextureStorageMem2DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

/// [glTextureStorageMem3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorageMem3DEXT.xhtml)
/// * `texture` class: texture
pub type glTextureStorageMem3DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64);

/// [glTextureStorageMem3DMultisampleEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorageMem3DMultisampleEXT.xhtml)
/// * `texture` class: texture
/// * `fixedSampleLocations` group: Boolean
pub type glTextureStorageMem3DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

/// [glTextureStorageSparseAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorageSparseAMD.xhtml)
/// * `texture` class: texture
/// * `internalFormat` group: InternalFormat
/// * `flags` group: TextureStorageMaskAMD
pub type glTextureStorageSparseAMD_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, internalFormat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, layers: GLsizei, flags: GLbitfield);

/// [glTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage1D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureSubImage1DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage1DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage2D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureSubImage2DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage2DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glTextureSubImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage3D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureSubImage3DEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage3DEXT.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTextureSubImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// [glTextureView](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureView.xhtml)
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `origtexture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureView_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

/// [glTextureViewEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureViewEXT.xhtml)
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `origtexture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureViewEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

/// [glTextureViewOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureViewOES.xhtml)
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `origtexture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureViewOES_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

/// [glTrackMatrixNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTrackMatrixNV.xhtml)
/// * `target` group: VertexAttribEnumNV
/// * `matrix` group: VertexAttribEnumNV
/// * `transform` group: VertexAttribEnumNV
pub type glTrackMatrixNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, address: GLuint, matrix: VertexAttribEnumNV, transform: VertexAttribEnumNV);

/// [glTransformFeedbackAttribsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackAttribsNV.xhtml)
/// * `attribs` len: COMPSIZE(count)
pub type glTransformFeedbackAttribsNV_t = unsafe extern "system" fn(count: GLsizei, attribs: *const GLint, bufferMode: GLenum);

/// [glTransformFeedbackBufferBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackBufferBase.xhtml)
/// * `xfb` class: transform feedback
/// * `buffer` class: buffer
pub type glTransformFeedbackBufferBase_t = unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint);

/// [glTransformFeedbackBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackBufferRange.xhtml)
/// * `xfb` class: transform feedback
/// * `buffer` class: buffer
/// * `size` group: BufferSize
pub type glTransformFeedbackBufferRange_t = unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// [glTransformFeedbackStreamAttribsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackStreamAttribsNV.xhtml)
/// * `attribs` len: count
/// * `bufstreams` len: nbuffers
pub type glTransformFeedbackStreamAttribsNV_t = unsafe extern "system" fn(count: GLsizei, attribs: *const GLint, nbuffers: GLsizei, bufstreams: *const GLint, bufferMode: GLenum);

/// [glTransformFeedbackVaryings](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackVaryings.xhtml)
/// * `program` class: program
/// * `varyings` len: count
/// * `bufferMode` group: TransformFeedbackBufferMode
pub type glTransformFeedbackVaryings_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode);

/// [glTransformFeedbackVaryingsEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackVaryingsEXT.xhtml)
/// * `program` class: program
/// * `varyings` len: count
pub type glTransformFeedbackVaryingsEXT_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);

/// [glTransformFeedbackVaryingsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackVaryingsNV.xhtml)
/// * `program` class: program
/// * `locations` len: count
pub type glTransformFeedbackVaryingsNV_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, locations: *const GLint, bufferMode: GLenum);

/// [glTransformPathNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformPathNV.xhtml)
/// * `resultPath` group: Path
/// * `srcPath` group: Path
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(transformType)
pub type glTransformPathNV_t = unsafe extern "system" fn(resultPath: GLuint, srcPath: GLuint, transformType: PathTransformType, transformValues: *const GLfloat);

/// [glTranslated](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTranslated.xhtml)
pub type glTranslated_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// [glTranslatef](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTranslatef.xhtml)
pub type glTranslatef_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// [glTranslatex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTranslatex.xhtml)
pub type glTranslatex_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// [glTranslatexOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTranslatexOES.xhtml)
pub type glTranslatexOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// [glUniform1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1d.xhtml)
pub type glUniform1d_t = unsafe extern "system" fn(location: GLint, x: GLdouble);

/// [glUniform1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1dv.xhtml)
/// * `value` len: count*1
pub type glUniform1dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

/// [glUniform1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1f.xhtml)
pub type glUniform1f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);

/// [glUniform1fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1fARB.xhtml)
pub type glUniform1fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);

/// [glUniform1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1fv.xhtml)
/// * `value` len: count*1
pub type glUniform1fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// [glUniform1fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1fvARB.xhtml)
/// * `value` len: count*1
pub type glUniform1fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// [glUniform1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1i.xhtml)
pub type glUniform1i_t = unsafe extern "system" fn(location: GLint, v0: GLint);

/// [glUniform1i64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1i64ARB.xhtml)
pub type glUniform1i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64);

/// [glUniform1i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1i64NV.xhtml)
pub type glUniform1i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT);

/// [glUniform1i64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1i64vARB.xhtml)
/// * `value` len: count*1
pub type glUniform1i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

/// [glUniform1i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1i64vNV.xhtml)
/// * `value` len: count*1
pub type glUniform1i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

/// [glUniform1iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1iARB.xhtml)
pub type glUniform1iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint);

/// [glUniform1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1iv.xhtml)
/// * `value` len: count*1
pub type glUniform1iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// [glUniform1ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1ivARB.xhtml)
/// * `value` len: count*1
pub type glUniform1ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// [glUniform1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1ui.xhtml)
pub type glUniform1ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint);

/// [glUniform1ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1ui64ARB.xhtml)
pub type glUniform1ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64);

/// [glUniform1ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1ui64NV.xhtml)
pub type glUniform1ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT);

/// [glUniform1ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1ui64vARB.xhtml)
/// * `value` len: count*1
pub type glUniform1ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// [glUniform1ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1ui64vNV.xhtml)
/// * `value` len: count*1
pub type glUniform1ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glUniform1uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1uiEXT.xhtml)
pub type glUniform1uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint);

/// [glUniform1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1uiv.xhtml)
/// * `value` len: count*1
pub type glUniform1uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// [glUniform1uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1uivEXT.xhtml)
/// * `value` len: count*1
pub type glUniform1uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// [glUniform2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2d.xhtml)
pub type glUniform2d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble);

/// [glUniform2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2dv.xhtml)
/// * `value` len: count*2
pub type glUniform2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

/// [glUniform2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2f.xhtml)
pub type glUniform2f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);

/// [glUniform2fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2fARB.xhtml)
pub type glUniform2fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);

/// [glUniform2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2fv.xhtml)
/// * `value` len: count*2
pub type glUniform2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// [glUniform2fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2fvARB.xhtml)
/// * `value` len: count*2
pub type glUniform2fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// [glUniform2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2i.xhtml)
pub type glUniform2i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);

/// [glUniform2i64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2i64ARB.xhtml)
pub type glUniform2i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64);

/// [glUniform2i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2i64NV.xhtml)
pub type glUniform2i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT);

/// [glUniform2i64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2i64vARB.xhtml)
/// * `value` len: count*2
pub type glUniform2i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

/// [glUniform2i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2i64vNV.xhtml)
/// * `value` len: count*2
pub type glUniform2i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

/// [glUniform2iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2iARB.xhtml)
pub type glUniform2iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);

/// [glUniform2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2iv.xhtml)
/// * `value` len: count*2
pub type glUniform2iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// [glUniform2ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2ivARB.xhtml)
/// * `value` len: count*2
pub type glUniform2ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// [glUniform2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2ui.xhtml)
pub type glUniform2ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);

/// [glUniform2ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2ui64ARB.xhtml)
pub type glUniform2ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64);

/// [glUniform2ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2ui64NV.xhtml)
pub type glUniform2ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT);

/// [glUniform2ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2ui64vARB.xhtml)
/// * `value` len: count*2
pub type glUniform2ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// [glUniform2ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2ui64vNV.xhtml)
/// * `value` len: count*2
pub type glUniform2ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glUniform2uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2uiEXT.xhtml)
pub type glUniform2uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);

/// [glUniform2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2uiv.xhtml)
/// * `value` len: count*2
pub type glUniform2uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// [glUniform2uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2uivEXT.xhtml)
/// * `value` len: count*2
pub type glUniform2uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// [glUniform3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3d.xhtml)
pub type glUniform3d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glUniform3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3dv.xhtml)
/// * `value` len: count*3
pub type glUniform3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

/// [glUniform3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3f.xhtml)
pub type glUniform3f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

/// [glUniform3fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3fARB.xhtml)
pub type glUniform3fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

/// [glUniform3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3fv.xhtml)
/// * `value` len: count*3
pub type glUniform3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// [glUniform3fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3fvARB.xhtml)
/// * `value` len: count*3
pub type glUniform3fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// [glUniform3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3i.xhtml)
pub type glUniform3i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);

/// [glUniform3i64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3i64ARB.xhtml)
pub type glUniform3i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64, z: GLint64);

/// [glUniform3i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3i64NV.xhtml)
pub type glUniform3i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

/// [glUniform3i64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3i64vARB.xhtml)
/// * `value` len: count*3
pub type glUniform3i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

/// [glUniform3i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3i64vNV.xhtml)
/// * `value` len: count*3
pub type glUniform3i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

/// [glUniform3iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3iARB.xhtml)
pub type glUniform3iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);

/// [glUniform3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3iv.xhtml)
/// * `value` len: count*3
pub type glUniform3iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// [glUniform3ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3ivARB.xhtml)
/// * `value` len: count*3
pub type glUniform3ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// [glUniform3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3ui.xhtml)
pub type glUniform3ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

/// [glUniform3ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3ui64ARB.xhtml)
pub type glUniform3ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);

/// [glUniform3ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3ui64NV.xhtml)
pub type glUniform3ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

/// [glUniform3ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3ui64vARB.xhtml)
/// * `value` len: count*3
pub type glUniform3ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// [glUniform3ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3ui64vNV.xhtml)
/// * `value` len: count*3
pub type glUniform3ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glUniform3uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3uiEXT.xhtml)
pub type glUniform3uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

/// [glUniform3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3uiv.xhtml)
/// * `value` len: count*3
pub type glUniform3uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// [glUniform3uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3uivEXT.xhtml)
/// * `value` len: count*3
pub type glUniform3uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// [glUniform4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4d.xhtml)
pub type glUniform4d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glUniform4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4dv.xhtml)
/// * `value` len: count*4
pub type glUniform4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

/// [glUniform4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4f.xhtml)
pub type glUniform4f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

/// [glUniform4fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4fARB.xhtml)
pub type glUniform4fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

/// [glUniform4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4fv.xhtml)
/// * `value` len: count*4
pub type glUniform4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// [glUniform4fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4fvARB.xhtml)
/// * `value` len: count*4
pub type glUniform4fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// [glUniform4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4i.xhtml)
pub type glUniform4i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

/// [glUniform4i64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4i64ARB.xhtml)
pub type glUniform4i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);

/// [glUniform4i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4i64NV.xhtml)
pub type glUniform4i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

/// [glUniform4i64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4i64vARB.xhtml)
/// * `value` len: count*4
pub type glUniform4i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

/// [glUniform4i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4i64vNV.xhtml)
/// * `value` len: count*4
pub type glUniform4i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

/// [glUniform4iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4iARB.xhtml)
pub type glUniform4iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

/// [glUniform4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4iv.xhtml)
/// * `value` len: count*4
pub type glUniform4iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// [glUniform4ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4ivARB.xhtml)
/// * `value` len: count*4
pub type glUniform4ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// [glUniform4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4ui.xhtml)
pub type glUniform4ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

/// [glUniform4ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4ui64ARB.xhtml)
pub type glUniform4ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);

/// [glUniform4ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4ui64NV.xhtml)
pub type glUniform4ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

/// [glUniform4ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4ui64vARB.xhtml)
/// * `value` len: count*4
pub type glUniform4ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// [glUniform4ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4ui64vNV.xhtml)
/// * `value` len: count*4
pub type glUniform4ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glUniform4uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4uiEXT.xhtml)
pub type glUniform4uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

/// [glUniform4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4uiv.xhtml)
/// * `value` len: count*4
pub type glUniform4uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// [glUniform4uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4uivEXT.xhtml)
/// * `value` len: count*4
pub type glUniform4uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// [glUniformBlockBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformBlockBinding.xhtml)
/// * `program` class: program
pub type glUniformBlockBinding_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);

/// [glUniformBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformBufferEXT.xhtml)
/// * `program` class: program
/// * `buffer` class: buffer
pub type glUniformBufferEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, buffer: GLuint);

/// [glUniformHandleui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformHandleui64ARB.xhtml)
pub type glUniformHandleui64ARB_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

/// [glUniformHandleui64IMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformHandleui64IMG.xhtml)
pub type glUniformHandleui64IMG_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

/// [glUniformHandleui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformHandleui64NV.xhtml)
pub type glUniformHandleui64NV_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

/// [glUniformHandleui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformHandleui64vARB.xhtml)
/// * `value` len: count
pub type glUniformHandleui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// [glUniformHandleui64vIMG](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformHandleui64vIMG.xhtml)
/// * `value` len: count
pub type glUniformHandleui64vIMG_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// [glUniformHandleui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformHandleui64vNV.xhtml)
/// * `value` len: count
pub type glUniformHandleui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// [glUniformMatrix2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glUniformMatrix2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glUniformMatrix2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix2fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2fvARB.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glUniformMatrix2fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix2x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x3dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix2x3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix2x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x3fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix2x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix2x3fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x3fvNV.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix2x3fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix2x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x4dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix2x4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix2x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x4fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix2x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix2x4fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x4fvNV.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix2x4fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glUniformMatrix3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glUniformMatrix3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix3fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3fvARB.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glUniformMatrix3fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix3x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x2dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix3x2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix3x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x2fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix3x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix3x2fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x2fvNV.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix3x2fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix3x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x4dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix3x4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix3x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x4fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix3x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix3x4fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x4fvNV.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix3x4fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glUniformMatrix4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glUniformMatrix4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix4fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4fvARB.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glUniformMatrix4fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix4x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x2dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix4x2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix4x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x2fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix4x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix4x2fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x2fvNV.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix4x2fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix4x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x3dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix4x3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// [glUniformMatrix4x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x3fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix4x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformMatrix4x3fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x3fvNV.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix4x3fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// [glUniformSubroutinesuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformSubroutinesuiv.xhtml)
/// * `shadertype` group: ShaderType
/// * `indices` len: count
pub type glUniformSubroutinesuiv_t = unsafe extern "system" fn(shadertype: ShaderType, count: GLsizei, indices: *const GLuint);

/// [glUniformui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformui64NV.xhtml)
pub type glUniformui64NV_t = unsafe extern "system" fn(location: GLint, value: GLuint64EXT);

/// [glUniformui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformui64vNV.xhtml)
/// * `value` len: count*1
pub type glUniformui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// [glUnlockArraysEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnlockArraysEXT.xhtml)
pub type glUnlockArraysEXT_t = unsafe extern "system" fn();

/// [glUnmapBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapBuffer.xhtml)
/// * `target` group: BufferTargetARB
pub type glUnmapBuffer_t = unsafe extern "system" fn(target: BufferTargetARB) -> GLboolean;

/// [glUnmapBufferARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapBufferARB.xhtml)
/// * `target` group: BufferTargetARB
pub type glUnmapBufferARB_t = unsafe extern "system" fn(target: BufferTargetARB) -> GLboolean;

/// [glUnmapBufferOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapBufferOES.xhtml)
pub type glUnmapBufferOES_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;

/// [glUnmapNamedBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapNamedBuffer.xhtml)
/// * `buffer` class: buffer
pub type glUnmapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// [glUnmapNamedBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapNamedBufferEXT.xhtml)
/// * `buffer` class: buffer
pub type glUnmapNamedBufferEXT_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// [glUnmapObjectBufferATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapObjectBufferATI.xhtml)
/// * `buffer` class: buffer
pub type glUnmapObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint);

/// [glUnmapTexture2DINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapTexture2DINTEL.xhtml)
/// * `texture` class: texture
pub type glUnmapTexture2DINTEL_t = unsafe extern "system" fn(texture: GLuint, level: GLint);

/// [glUpdateObjectBufferATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUpdateObjectBufferATI.xhtml)
/// * `buffer` class: buffer
/// * `pointer` len: size
/// * `preserve` group: PreserveModeATI
pub type glUpdateObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint, offset: GLuint, size: GLsizei, pointer: *const void, preserve: PreserveModeATI);

/// [glUploadGpuMaskNVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUploadGpuMaskNVX.xhtml)
pub type glUploadGpuMaskNVX_t = unsafe extern "system" fn(mask: GLbitfield);

/// [glUseProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml)
/// * `program` class: program
pub type glUseProgram_t = unsafe extern "system" fn(program: GLuint);

/// [glUseProgramObjectARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgramObjectARB.xhtml)
/// * `programObj` group: handleARB
pub type glUseProgramObjectARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

/// [glUseProgramStages](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgramStages.xhtml)
/// * `pipeline` class: program pipeline
/// * `stages` group: UseProgramStageMask
/// * `program` class: program
pub type glUseProgramStages_t = unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);

/// [glUseProgramStagesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgramStagesEXT.xhtml)
/// * `pipeline` class: program pipeline
/// * `stages` group: UseProgramStageMask
/// * `program` class: program
pub type glUseProgramStagesEXT_t = unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);

/// [glUseShaderProgramEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseShaderProgramEXT.xhtml)
/// * `program` class: program
pub type glUseShaderProgramEXT_t = unsafe extern "system" fn(type_: GLenum, program: GLuint);

/// [glVDPAUFiniNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAUFiniNV.xhtml)
pub type glVDPAUFiniNV_t = unsafe extern "system" fn();

/// [glVDPAUGetSurfaceivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAUGetSurfaceivNV.xhtml)
/// * `surface` group: vdpauSurfaceNV
/// * `values` len: count
pub type glVDPAUGetSurfaceivNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

/// [glVDPAUInitNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAUInitNV.xhtml)
pub type glVDPAUInitNV_t = unsafe extern "system" fn(vdpDevice: *const void, getProcAddress: *const void);

/// [glVDPAUIsSurfaceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAUIsSurfaceNV.xhtml)
/// * `surface` group: vdpauSurfaceNV
pub type glVDPAUIsSurfaceNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV) -> GLboolean;

/// [glVDPAUMapSurfacesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAUMapSurfacesNV.xhtml)
/// * `surfaces` group: vdpauSurfaceNV
/// * `surfaces` len: numSurfaces
pub type glVDPAUMapSurfacesNV_t = unsafe extern "system" fn(numSurfaces: GLsizei, surfaces: *const GLvdpauSurfaceNV);

/// [glVDPAURegisterOutputSurfaceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAURegisterOutputSurfaceNV.xhtml)
/// * `textureNames` len: numTextureNames
pub type glVDPAURegisterOutputSurfaceNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint) -> GLvdpauSurfaceNV;

/// [glVDPAURegisterVideoSurfaceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAURegisterVideoSurfaceNV.xhtml)
/// * `textureNames` len: numTextureNames
pub type glVDPAURegisterVideoSurfaceNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint) -> GLvdpauSurfaceNV;

/// [glVDPAURegisterVideoSurfaceWithPictureStructureNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAURegisterVideoSurfaceWithPictureStructureNV.xhtml)
/// * `textureNames` len: numTextureNames
/// * `isFrameStructure` group: Boolean
pub type glVDPAURegisterVideoSurfaceWithPictureStructureNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint, isFrameStructure: GLboolean) -> GLvdpauSurfaceNV;

/// [glVDPAUSurfaceAccessNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAUSurfaceAccessNV.xhtml)
/// * `surface` group: vdpauSurfaceNV
pub type glVDPAUSurfaceAccessNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV, access: GLenum);

/// [glVDPAUUnmapSurfacesNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAUUnmapSurfacesNV.xhtml)
/// * `surfaces` group: vdpauSurfaceNV
/// * `surfaces` len: numSurface
pub type glVDPAUUnmapSurfacesNV_t = unsafe extern "system" fn(numSurface: GLsizei, surfaces: *const GLvdpauSurfaceNV);

/// [glVDPAUUnregisterSurfaceNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVDPAUUnregisterSurfaceNV.xhtml)
/// * `surface` group: vdpauSurfaceNV
pub type glVDPAUUnregisterSurfaceNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV);

/// [glValidateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glValidateProgram.xhtml)
/// * `program` class: program
pub type glValidateProgram_t = unsafe extern "system" fn(program: GLuint);

/// [glValidateProgramARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glValidateProgramARB.xhtml)
/// * `programObj` group: handleARB
pub type glValidateProgramARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

/// [glValidateProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glValidateProgramPipeline.xhtml)
/// * `pipeline` class: program pipeline
pub type glValidateProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);

/// [glValidateProgramPipelineEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glValidateProgramPipelineEXT.xhtml)
/// * `pipeline` class: program pipeline
pub type glValidateProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint);

/// [glVariantArrayObjectATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantArrayObjectATI.xhtml)
/// * `type` group: ScalarType
/// * `buffer` class: buffer
pub type glVariantArrayObjectATI_t = unsafe extern "system" fn(id: GLuint, type_: ScalarType, stride: GLsizei, buffer: GLuint, offset: GLuint);

/// [glVariantPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantPointerEXT.xhtml)
/// * `type` group: ScalarType
/// * `addr` len: COMPSIZE(id,type,stride)
pub type glVariantPointerEXT_t = unsafe extern "system" fn(id: GLuint, type_: ScalarType, stride: GLuint, addr: *const void);

/// [glVariantbvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantbvEXT.xhtml)
/// * `addr` len: COMPSIZE(id)
pub type glVariantbvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLbyte);

/// [glVariantdvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantdvEXT.xhtml)
/// * `addr` len: COMPSIZE(id)
pub type glVariantdvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLdouble);

/// [glVariantfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantfvEXT.xhtml)
/// * `addr` len: COMPSIZE(id)
pub type glVariantfvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLfloat);

/// [glVariantivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantivEXT.xhtml)
/// * `addr` len: COMPSIZE(id)
pub type glVariantivEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLint);

/// [glVariantsvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantsvEXT.xhtml)
/// * `addr` len: COMPSIZE(id)
pub type glVariantsvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLshort);

/// [glVariantubvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantubvEXT.xhtml)
/// * `addr` len: COMPSIZE(id)
pub type glVariantubvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLubyte);

/// [glVariantuivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantuivEXT.xhtml)
/// * `addr` len: COMPSIZE(id)
pub type glVariantuivEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLuint);

/// [glVariantusvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVariantusvEXT.xhtml)
/// * `addr` len: COMPSIZE(id)
pub type glVariantusvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLushort);

/// [glVertex2bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2bOES.xhtml)
pub type glVertex2bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte);

/// [glVertex2bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2bvOES.xhtml)
pub type glVertex2bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 2]);

/// [glVertex2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2d.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glVertex2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// [glVertex2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2dv.xhtml)
/// * `v` group: CoordD
pub type glVertex2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// [glVertex2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2f.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glVertex2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// [glVertex2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2fv.xhtml)
/// * `v` group: CoordF
pub type glVertex2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// [glVertex2hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2hNV.xhtml)
/// * `x` group: Half16NV
/// * `y` group: Half16NV
pub type glVertex2hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV);

/// [glVertex2hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2hvNV.xhtml)
/// * `v` group: Half16NV
pub type glVertex2hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 2]);

/// [glVertex2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2i.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glVertex2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// [glVertex2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2iv.xhtml)
/// * `v` group: CoordI
pub type glVertex2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// [glVertex2s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2s.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glVertex2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// [glVertex2sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2sv.xhtml)
/// * `v` group: CoordS
pub type glVertex2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// [glVertex2xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2xOES.xhtml)
pub type glVertex2xOES_t = unsafe extern "system" fn(x: GLfixed);

/// [glVertex2xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex2xvOES.xhtml)
pub type glVertex2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

/// [glVertex3bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3bOES.xhtml)
pub type glVertex3bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte, z: GLbyte);

/// [glVertex3bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3bvOES.xhtml)
pub type glVertex3bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 3]);

/// [glVertex3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3d.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glVertex3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// [glVertex3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3dv.xhtml)
/// * `v` group: CoordD
pub type glVertex3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glVertex3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3f.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glVertex3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// [glVertex3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3fv.xhtml)
/// * `v` group: CoordF
pub type glVertex3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glVertex3hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3hNV.xhtml)
/// * `x` group: Half16NV
/// * `y` group: Half16NV
/// * `z` group: Half16NV
pub type glVertex3hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV, z: GLhalfNV);

/// [glVertex3hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3hvNV.xhtml)
/// * `v` group: Half16NV
pub type glVertex3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// [glVertex3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3i.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glVertex3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// [glVertex3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3iv.xhtml)
/// * `v` group: CoordI
pub type glVertex3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glVertex3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3s.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glVertex3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// [glVertex3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3sv.xhtml)
/// * `v` group: CoordS
pub type glVertex3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glVertex3xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3xOES.xhtml)
pub type glVertex3xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed);

/// [glVertex3xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex3xvOES.xhtml)
pub type glVertex3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

/// [glVertex4bOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4bOES.xhtml)
pub type glVertex4bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte, z: GLbyte, w: GLbyte);

/// [glVertex4bvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4bvOES.xhtml)
pub type glVertex4bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 4]);

/// [glVertex4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4d.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
/// * `w` group: CoordD
pub type glVertex4d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glVertex4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4dv.xhtml)
/// * `v` group: CoordD
pub type glVertex4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// [glVertex4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4f.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
/// * `w` group: CoordF
pub type glVertex4f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glVertex4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4fv.xhtml)
/// * `v` group: CoordF
pub type glVertex4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// [glVertex4hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4hNV.xhtml)
/// * `x` group: Half16NV
/// * `y` group: Half16NV
/// * `z` group: Half16NV
/// * `w` group: Half16NV
pub type glVertex4hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV, z: GLhalfNV, w: GLhalfNV);

/// [glVertex4hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4hvNV.xhtml)
/// * `v` group: Half16NV
pub type glVertex4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

/// [glVertex4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4i.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
/// * `w` group: CoordI
pub type glVertex4i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

/// [glVertex4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4iv.xhtml)
/// * `v` group: CoordI
pub type glVertex4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// [glVertex4s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4s.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
/// * `w` group: CoordS
pub type glVertex4s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// [glVertex4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4sv.xhtml)
/// * `v` group: CoordS
pub type glVertex4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// [glVertex4xOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4xOES.xhtml)
pub type glVertex4xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// [glVertex4xvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertex4xvOES.xhtml)
pub type glVertex4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

/// [glVertexArrayAttribBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribBinding.xhtml)
/// * `vaobj` class: vertex array
pub type glVertexArrayAttribBinding_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);

/// [glVertexArrayAttribFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribFormat.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
pub type glVertexArrayAttribFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);

/// [glVertexArrayAttribIFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribIFormat.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribIType
pub type glVertexArrayAttribIFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);

/// [glVertexArrayAttribLFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribLFormat.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribLType
pub type glVertexArrayAttribLFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);

/// [glVertexArrayBindVertexBufferEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayBindVertexBufferEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
pub type glVertexArrayBindVertexBufferEXT_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

/// [glVertexArrayBindingDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayBindingDivisor.xhtml)
/// * `vaobj` class: vertex array
pub type glVertexArrayBindingDivisor_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);

/// [glVertexArrayColorOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayColorOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: ColorPointerType
pub type glVertexArrayColorOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: ColorPointerType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayEdgeFlagOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayEdgeFlagOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
pub type glVertexArrayEdgeFlagOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayElementBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayElementBuffer.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
pub type glVertexArrayElementBuffer_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint);

/// [glVertexArrayFogCoordOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayFogCoordOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: FogCoordinatePointerType
pub type glVertexArrayFogCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: FogCoordinatePointerType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayIndexOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayIndexOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: IndexPointerType
pub type glVertexArrayIndexOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: IndexPointerType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayMultiTexCoordOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayMultiTexCoordOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: TexCoordPointerType
pub type glVertexArrayMultiTexCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, texunit: GLenum, size: GLint, type_: TexCoordPointerType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayNormalOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayNormalOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: NormalPointerType
pub type glVertexArrayNormalOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: NormalPointerType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayParameteriAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayParameteriAPPLE.xhtml)
/// * `pname` group: VertexArrayPNameAPPLE
pub type glVertexArrayParameteriAPPLE_t = unsafe extern "system" fn(pname: VertexArrayPNameAPPLE, param: GLint);

/// [glVertexArrayRangeAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayRangeAPPLE.xhtml)
/// * `pointer` len: length
pub type glVertexArrayRangeAPPLE_t = unsafe extern "system" fn(length: GLsizei, pointer: *mut void);

/// [glVertexArrayRangeNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayRangeNV.xhtml)
/// * `pointer` len: COMPSIZE(length)
pub type glVertexArrayRangeNV_t = unsafe extern "system" fn(length: GLsizei, pointer: *const void);

/// [glVertexArraySecondaryColorOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArraySecondaryColorOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: ColorPointerType
pub type glVertexArraySecondaryColorOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: ColorPointerType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayTexCoordOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayTexCoordOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: TexCoordPointerType
pub type glVertexArrayTexCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: TexCoordPointerType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayVertexAttribBindingEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexAttribBindingEXT.xhtml)
/// * `vaobj` class: vertex array
pub type glVertexArrayVertexAttribBindingEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);

/// [glVertexArrayVertexAttribDivisorEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexAttribDivisorEXT.xhtml)
/// * `vaobj` class: vertex array
pub type glVertexArrayVertexAttribDivisorEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, divisor: GLuint);

/// [glVertexArrayVertexAttribFormatEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexAttribFormatEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
pub type glVertexArrayVertexAttribFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);

/// [glVertexArrayVertexAttribIFormatEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexAttribIFormatEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribIType
pub type glVertexArrayVertexAttribIFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);

/// [glVertexArrayVertexAttribIOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexAttribIOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: VertexAttribType
pub type glVertexArrayVertexAttribIOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: VertexAttribType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayVertexAttribLFormatEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexAttribLFormatEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribLType
pub type glVertexArrayVertexAttribLFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);

/// [glVertexArrayVertexAttribLOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexAttribLOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: VertexAttribLType
/// * `offset` group: BufferOffset
pub type glVertexArrayVertexAttribLOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayVertexAttribOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexAttribOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexArrayVertexAttribOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, offset: GLintptr);

/// [glVertexArrayVertexBindingDivisorEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexBindingDivisorEXT.xhtml)
/// * `vaobj` class: vertex array
pub type glVertexArrayVertexBindingDivisorEXT_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);

/// [glVertexArrayVertexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexBuffer.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
pub type glVertexArrayVertexBuffer_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

/// [glVertexArrayVertexBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexBuffers.xhtml)
/// * `vaobj` class: vertex array
/// * `buffers` class: buffer
pub type glVertexArrayVertexBuffers_t = unsafe extern "system" fn(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);

/// [glVertexArrayVertexOffsetEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexOffsetEXT.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: VertexPointerType
pub type glVertexArrayVertexOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: VertexPointerType, stride: GLsizei, offset: GLintptr);

/// [glVertexAttrib1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1d.xhtml)
pub type glVertexAttrib1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// [glVertexAttrib1dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1dARB.xhtml)
pub type glVertexAttrib1dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// [glVertexAttrib1dNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1dNV.xhtml)
pub type glVertexAttrib1dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// [glVertexAttrib1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1dv.xhtml)
pub type glVertexAttrib1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// [glVertexAttrib1dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1dvARB.xhtml)
pub type glVertexAttrib1dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// [glVertexAttrib1dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1dvNV.xhtml)
pub type glVertexAttrib1dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// [glVertexAttrib1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1f.xhtml)
pub type glVertexAttrib1f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

/// [glVertexAttrib1fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1fARB.xhtml)
pub type glVertexAttrib1fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

/// [glVertexAttrib1fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1fNV.xhtml)
pub type glVertexAttrib1fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

/// [glVertexAttrib1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1fv.xhtml)
pub type glVertexAttrib1fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

/// [glVertexAttrib1fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1fvARB.xhtml)
pub type glVertexAttrib1fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

/// [glVertexAttrib1fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1fvNV.xhtml)
pub type glVertexAttrib1fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

/// [glVertexAttrib1hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1hNV.xhtml)
/// * `x` group: Half16NV
pub type glVertexAttrib1hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV);

/// [glVertexAttrib1hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1hvNV.xhtml)
/// * `v` group: Half16NV
pub type glVertexAttrib1hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLhalfNV);

/// [glVertexAttrib1s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1s.xhtml)
pub type glVertexAttrib1s_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

/// [glVertexAttrib1sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1sARB.xhtml)
pub type glVertexAttrib1sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

/// [glVertexAttrib1sNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1sNV.xhtml)
pub type glVertexAttrib1sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

/// [glVertexAttrib1sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1sv.xhtml)
pub type glVertexAttrib1sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

/// [glVertexAttrib1svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1svARB.xhtml)
pub type glVertexAttrib1svARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

/// [glVertexAttrib1svNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1svNV.xhtml)
pub type glVertexAttrib1svNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

/// [glVertexAttrib2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2d.xhtml)
pub type glVertexAttrib2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// [glVertexAttrib2dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2dARB.xhtml)
pub type glVertexAttrib2dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// [glVertexAttrib2dNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2dNV.xhtml)
pub type glVertexAttrib2dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// [glVertexAttrib2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2dv.xhtml)
pub type glVertexAttrib2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// [glVertexAttrib2dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2dvARB.xhtml)
pub type glVertexAttrib2dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// [glVertexAttrib2dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2dvNV.xhtml)
pub type glVertexAttrib2dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// [glVertexAttrib2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2f.xhtml)
pub type glVertexAttrib2f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

/// [glVertexAttrib2fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2fARB.xhtml)
pub type glVertexAttrib2fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

/// [glVertexAttrib2fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2fNV.xhtml)
pub type glVertexAttrib2fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

/// [glVertexAttrib2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2fv.xhtml)
pub type glVertexAttrib2fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

/// [glVertexAttrib2fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2fvARB.xhtml)
pub type glVertexAttrib2fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

/// [glVertexAttrib2fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2fvNV.xhtml)
pub type glVertexAttrib2fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

/// [glVertexAttrib2hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2hNV.xhtml)
/// * `x` group: Half16NV
/// * `y` group: Half16NV
pub type glVertexAttrib2hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV);

/// [glVertexAttrib2hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2hvNV.xhtml)
/// * `v` group: Half16NV
pub type glVertexAttrib2hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 2]);

/// [glVertexAttrib2s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2s.xhtml)
pub type glVertexAttrib2s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

/// [glVertexAttrib2sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2sARB.xhtml)
pub type glVertexAttrib2sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

/// [glVertexAttrib2sNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2sNV.xhtml)
pub type glVertexAttrib2sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

/// [glVertexAttrib2sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2sv.xhtml)
pub type glVertexAttrib2sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

/// [glVertexAttrib2svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2svARB.xhtml)
pub type glVertexAttrib2svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

/// [glVertexAttrib2svNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2svNV.xhtml)
pub type glVertexAttrib2svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

/// [glVertexAttrib3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3d.xhtml)
pub type glVertexAttrib3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glVertexAttrib3dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3dARB.xhtml)
pub type glVertexAttrib3dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glVertexAttrib3dNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3dNV.xhtml)
pub type glVertexAttrib3dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glVertexAttrib3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3dv.xhtml)
pub type glVertexAttrib3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// [glVertexAttrib3dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3dvARB.xhtml)
pub type glVertexAttrib3dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// [glVertexAttrib3dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3dvNV.xhtml)
pub type glVertexAttrib3dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// [glVertexAttrib3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3f.xhtml)
pub type glVertexAttrib3f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glVertexAttrib3fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3fARB.xhtml)
pub type glVertexAttrib3fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glVertexAttrib3fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3fNV.xhtml)
pub type glVertexAttrib3fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glVertexAttrib3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3fv.xhtml)
pub type glVertexAttrib3fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

/// [glVertexAttrib3fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3fvARB.xhtml)
pub type glVertexAttrib3fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

/// [glVertexAttrib3fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3fvNV.xhtml)
pub type glVertexAttrib3fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

/// [glVertexAttrib3hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3hNV.xhtml)
/// * `x` group: Half16NV
/// * `y` group: Half16NV
/// * `z` group: Half16NV
pub type glVertexAttrib3hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV, z: GLhalfNV);

/// [glVertexAttrib3hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3hvNV.xhtml)
/// * `v` group: Half16NV
pub type glVertexAttrib3hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 3]);

/// [glVertexAttrib3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3s.xhtml)
pub type glVertexAttrib3s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

/// [glVertexAttrib3sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3sARB.xhtml)
pub type glVertexAttrib3sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

/// [glVertexAttrib3sNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3sNV.xhtml)
pub type glVertexAttrib3sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

/// [glVertexAttrib3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3sv.xhtml)
pub type glVertexAttrib3sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

/// [glVertexAttrib3svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3svARB.xhtml)
pub type glVertexAttrib3svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

/// [glVertexAttrib3svNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3svNV.xhtml)
pub type glVertexAttrib3svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

/// [glVertexAttrib4Nbv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nbv.xhtml)
pub type glVertexAttrib4Nbv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// [glVertexAttrib4NbvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4NbvARB.xhtml)
pub type glVertexAttrib4NbvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// [glVertexAttrib4Niv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Niv.xhtml)
pub type glVertexAttrib4Niv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glVertexAttrib4NivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4NivARB.xhtml)
pub type glVertexAttrib4NivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glVertexAttrib4Nsv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nsv.xhtml)
pub type glVertexAttrib4Nsv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// [glVertexAttrib4NsvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4NsvARB.xhtml)
pub type glVertexAttrib4NsvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// [glVertexAttrib4Nub](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nub.xhtml)
pub type glVertexAttrib4Nub_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

/// [glVertexAttrib4NubARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4NubARB.xhtml)
pub type glVertexAttrib4NubARB_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

/// [glVertexAttrib4Nubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nubv.xhtml)
pub type glVertexAttrib4Nubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// [glVertexAttrib4NubvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4NubvARB.xhtml)
pub type glVertexAttrib4NubvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// [glVertexAttrib4Nuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nuiv.xhtml)
pub type glVertexAttrib4Nuiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// [glVertexAttrib4NuivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4NuivARB.xhtml)
pub type glVertexAttrib4NuivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// [glVertexAttrib4Nusv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nusv.xhtml)
pub type glVertexAttrib4Nusv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// [glVertexAttrib4NusvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4NusvARB.xhtml)
pub type glVertexAttrib4NusvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// [glVertexAttrib4bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4bv.xhtml)
pub type glVertexAttrib4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// [glVertexAttrib4bvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4bvARB.xhtml)
pub type glVertexAttrib4bvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// [glVertexAttrib4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4d.xhtml)
pub type glVertexAttrib4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glVertexAttrib4dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4dARB.xhtml)
pub type glVertexAttrib4dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glVertexAttrib4dNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4dNV.xhtml)
pub type glVertexAttrib4dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glVertexAttrib4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4dv.xhtml)
pub type glVertexAttrib4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// [glVertexAttrib4dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4dvARB.xhtml)
pub type glVertexAttrib4dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// [glVertexAttrib4dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4dvNV.xhtml)
pub type glVertexAttrib4dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// [glVertexAttrib4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4f.xhtml)
pub type glVertexAttrib4f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glVertexAttrib4fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4fARB.xhtml)
pub type glVertexAttrib4fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glVertexAttrib4fNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4fNV.xhtml)
pub type glVertexAttrib4fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glVertexAttrib4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4fv.xhtml)
pub type glVertexAttrib4fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// [glVertexAttrib4fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4fvARB.xhtml)
pub type glVertexAttrib4fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// [glVertexAttrib4fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4fvNV.xhtml)
pub type glVertexAttrib4fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// [glVertexAttrib4hNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4hNV.xhtml)
/// * `x` group: Half16NV
/// * `y` group: Half16NV
/// * `z` group: Half16NV
/// * `w` group: Half16NV
pub type glVertexAttrib4hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV, z: GLhalfNV, w: GLhalfNV);

/// [glVertexAttrib4hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4hvNV.xhtml)
/// * `v` group: Half16NV
pub type glVertexAttrib4hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 4]);

/// [glVertexAttrib4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4iv.xhtml)
pub type glVertexAttrib4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glVertexAttrib4ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4ivARB.xhtml)
pub type glVertexAttrib4ivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glVertexAttrib4s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4s.xhtml)
pub type glVertexAttrib4s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// [glVertexAttrib4sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4sARB.xhtml)
pub type glVertexAttrib4sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// [glVertexAttrib4sNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4sNV.xhtml)
pub type glVertexAttrib4sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// [glVertexAttrib4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4sv.xhtml)
pub type glVertexAttrib4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// [glVertexAttrib4svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4svARB.xhtml)
pub type glVertexAttrib4svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// [glVertexAttrib4svNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4svNV.xhtml)
pub type glVertexAttrib4svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// [glVertexAttrib4ubNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4ubNV.xhtml)
/// * `x` group: ColorUB
/// * `y` group: ColorUB
/// * `z` group: ColorUB
/// * `w` group: ColorUB
pub type glVertexAttrib4ubNV_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

/// [glVertexAttrib4ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4ubv.xhtml)
pub type glVertexAttrib4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// [glVertexAttrib4ubvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4ubvARB.xhtml)
pub type glVertexAttrib4ubvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// [glVertexAttrib4ubvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4ubvNV.xhtml)
/// * `v` group: ColorUB
pub type glVertexAttrib4ubvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// [glVertexAttrib4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4uiv.xhtml)
pub type glVertexAttrib4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// [glVertexAttrib4uivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4uivARB.xhtml)
pub type glVertexAttrib4uivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// [glVertexAttrib4usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4usv.xhtml)
pub type glVertexAttrib4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// [glVertexAttrib4usvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4usvARB.xhtml)
pub type glVertexAttrib4usvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// [glVertexAttribArrayObjectATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribArrayObjectATI.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
/// * `buffer` class: buffer
pub type glVertexAttribArrayObjectATI_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, buffer: GLuint, offset: GLuint);

/// [glVertexAttribBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribBinding.xhtml)
pub type glVertexAttribBinding_t = unsafe extern "system" fn(attribindex: GLuint, bindingindex: GLuint);

/// [glVertexAttribDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribDivisor.xhtml)
pub type glVertexAttribDivisor_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// [glVertexAttribDivisorANGLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribDivisorANGLE.xhtml)
pub type glVertexAttribDivisorANGLE_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// [glVertexAttribDivisorARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribDivisorARB.xhtml)
pub type glVertexAttribDivisorARB_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// [glVertexAttribDivisorEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribDivisorEXT.xhtml)
pub type glVertexAttribDivisorEXT_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// [glVertexAttribDivisorNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribDivisorNV.xhtml)
pub type glVertexAttribDivisorNV_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// [glVertexAttribFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribFormat.xhtml)
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
pub type glVertexAttribFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);

/// [glVertexAttribFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribFormatNV.xhtml)
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
pub type glVertexAttribFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, stride: GLsizei);

/// [glVertexAttribI1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1i.xhtml)
pub type glVertexAttribI1i_t = unsafe extern "system" fn(index: GLuint, x: GLint);

/// [glVertexAttribI1iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1iEXT.xhtml)
pub type glVertexAttribI1iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint);

/// [glVertexAttribI1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1iv.xhtml)
pub type glVertexAttribI1iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);

/// [glVertexAttribI1ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1ivEXT.xhtml)
pub type glVertexAttribI1ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);

/// [glVertexAttribI1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1ui.xhtml)
pub type glVertexAttribI1ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint);

/// [glVertexAttribI1uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1uiEXT.xhtml)
pub type glVertexAttribI1uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint);

/// [glVertexAttribI1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1uiv.xhtml)
pub type glVertexAttribI1uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);

/// [glVertexAttribI1uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1uivEXT.xhtml)
pub type glVertexAttribI1uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);

/// [glVertexAttribI2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2i.xhtml)
pub type glVertexAttribI2i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);

/// [glVertexAttribI2iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2iEXT.xhtml)
pub type glVertexAttribI2iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);

/// [glVertexAttribI2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2iv.xhtml)
pub type glVertexAttribI2iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 2]);

/// [glVertexAttribI2ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2ivEXT.xhtml)
pub type glVertexAttribI2ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 2]);

/// [glVertexAttribI2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2ui.xhtml)
pub type glVertexAttribI2ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);

/// [glVertexAttribI2uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2uiEXT.xhtml)
pub type glVertexAttribI2uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);

/// [glVertexAttribI2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2uiv.xhtml)
pub type glVertexAttribI2uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 2]);

/// [glVertexAttribI2uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2uivEXT.xhtml)
pub type glVertexAttribI2uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 2]);

/// [glVertexAttribI3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3i.xhtml)
pub type glVertexAttribI3i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);

/// [glVertexAttribI3iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3iEXT.xhtml)
pub type glVertexAttribI3iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);

/// [glVertexAttribI3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3iv.xhtml)
pub type glVertexAttribI3iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 3]);

/// [glVertexAttribI3ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3ivEXT.xhtml)
pub type glVertexAttribI3ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 3]);

/// [glVertexAttribI3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3ui.xhtml)
pub type glVertexAttribI3ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);

/// [glVertexAttribI3uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3uiEXT.xhtml)
pub type glVertexAttribI3uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);

/// [glVertexAttribI3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3uiv.xhtml)
pub type glVertexAttribI3uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 3]);

/// [glVertexAttribI3uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3uivEXT.xhtml)
pub type glVertexAttribI3uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 3]);

/// [glVertexAttribI4bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4bv.xhtml)
pub type glVertexAttribI4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// [glVertexAttribI4bvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4bvEXT.xhtml)
pub type glVertexAttribI4bvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// [glVertexAttribI4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4i.xhtml)
pub type glVertexAttribI4i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// [glVertexAttribI4iEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4iEXT.xhtml)
pub type glVertexAttribI4iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// [glVertexAttribI4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4iv.xhtml)
pub type glVertexAttribI4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glVertexAttribI4ivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4ivEXT.xhtml)
pub type glVertexAttribI4ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// [glVertexAttribI4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4sv.xhtml)
pub type glVertexAttribI4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// [glVertexAttribI4svEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4svEXT.xhtml)
pub type glVertexAttribI4svEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// [glVertexAttribI4ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4ubv.xhtml)
pub type glVertexAttribI4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// [glVertexAttribI4ubvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4ubvEXT.xhtml)
pub type glVertexAttribI4ubvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// [glVertexAttribI4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4ui.xhtml)
pub type glVertexAttribI4ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// [glVertexAttribI4uiEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4uiEXT.xhtml)
pub type glVertexAttribI4uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// [glVertexAttribI4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4uiv.xhtml)
pub type glVertexAttribI4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// [glVertexAttribI4uivEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4uivEXT.xhtml)
pub type glVertexAttribI4uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// [glVertexAttribI4usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4usv.xhtml)
pub type glVertexAttribI4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// [glVertexAttribI4usvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4usvEXT.xhtml)
pub type glVertexAttribI4usvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// [glVertexAttribIFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribIFormat.xhtml)
/// * `type` group: VertexAttribIType
pub type glVertexAttribIFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);

/// [glVertexAttribIFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribIFormatNV.xhtml)
/// * `type` group: VertexAttribIType
pub type glVertexAttribIFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei);

/// [glVertexAttribIPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribIPointer.xhtml)
/// * `type` group: VertexAttribIType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexAttribIPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void);

/// [glVertexAttribIPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribIPointerEXT.xhtml)
/// * `type` group: VertexAttribIType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexAttribIPointerEXT_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void);

/// [glVertexAttribL1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1d.xhtml)
pub type glVertexAttribL1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// [glVertexAttribL1dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1dEXT.xhtml)
pub type glVertexAttribL1dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// [glVertexAttribL1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1dv.xhtml)
pub type glVertexAttribL1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// [glVertexAttribL1dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1dvEXT.xhtml)
pub type glVertexAttribL1dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// [glVertexAttribL1i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1i64NV.xhtml)
pub type glVertexAttribL1i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT);

/// [glVertexAttribL1i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1i64vNV.xhtml)
pub type glVertexAttribL1i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLint64EXT);

/// [glVertexAttribL1ui64ARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1ui64ARB.xhtml)
pub type glVertexAttribL1ui64ARB_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT);

/// [glVertexAttribL1ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1ui64NV.xhtml)
pub type glVertexAttribL1ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT);

/// [glVertexAttribL1ui64vARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1ui64vARB.xhtml)
pub type glVertexAttribL1ui64vARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint64EXT);

/// [glVertexAttribL1ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1ui64vNV.xhtml)
pub type glVertexAttribL1ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint64EXT);

/// [glVertexAttribL2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2d.xhtml)
pub type glVertexAttribL2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// [glVertexAttribL2dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2dEXT.xhtml)
pub type glVertexAttribL2dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// [glVertexAttribL2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2dv.xhtml)
pub type glVertexAttribL2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// [glVertexAttribL2dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2dvEXT.xhtml)
pub type glVertexAttribL2dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// [glVertexAttribL2i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2i64NV.xhtml)
pub type glVertexAttribL2i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT);

/// [glVertexAttribL2i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2i64vNV.xhtml)
pub type glVertexAttribL2i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 2]);

/// [glVertexAttribL2ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2ui64NV.xhtml)
pub type glVertexAttribL2ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT);

/// [glVertexAttribL2ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2ui64vNV.xhtml)
pub type glVertexAttribL2ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 2]);

/// [glVertexAttribL3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3d.xhtml)
pub type glVertexAttribL3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glVertexAttribL3dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3dEXT.xhtml)
pub type glVertexAttribL3dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glVertexAttribL3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3dv.xhtml)
pub type glVertexAttribL3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// [glVertexAttribL3dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3dvEXT.xhtml)
pub type glVertexAttribL3dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// [glVertexAttribL3i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3i64NV.xhtml)
pub type glVertexAttribL3i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

/// [glVertexAttribL3i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3i64vNV.xhtml)
pub type glVertexAttribL3i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 3]);

/// [glVertexAttribL3ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3ui64NV.xhtml)
pub type glVertexAttribL3ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

/// [glVertexAttribL3ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3ui64vNV.xhtml)
pub type glVertexAttribL3ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 3]);

/// [glVertexAttribL4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4d.xhtml)
pub type glVertexAttribL4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glVertexAttribL4dEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4dEXT.xhtml)
pub type glVertexAttribL4dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glVertexAttribL4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4dv.xhtml)
pub type glVertexAttribL4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// [glVertexAttribL4dvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4dvEXT.xhtml)
pub type glVertexAttribL4dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// [glVertexAttribL4i64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4i64NV.xhtml)
pub type glVertexAttribL4i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

/// [glVertexAttribL4i64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4i64vNV.xhtml)
pub type glVertexAttribL4i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 4]);

/// [glVertexAttribL4ui64NV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4ui64NV.xhtml)
pub type glVertexAttribL4ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

/// [glVertexAttribL4ui64vNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4ui64vNV.xhtml)
pub type glVertexAttribL4ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 4]);

/// [glVertexAttribLFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribLFormat.xhtml)
/// * `type` group: VertexAttribLType
pub type glVertexAttribLFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);

/// [glVertexAttribLFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribLFormatNV.xhtml)
/// * `type` group: VertexAttribLType
pub type glVertexAttribLFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei);

/// [glVertexAttribLPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribLPointer.xhtml)
/// * `type` group: VertexAttribLType
/// * `pointer` len: size
pub type glVertexAttribLPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, pointer: *const void);

/// [glVertexAttribLPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribLPointerEXT.xhtml)
/// * `type` group: VertexAttribLType
/// * `pointer` len: size
pub type glVertexAttribLPointerEXT_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, pointer: *const void);

/// [glVertexAttribP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP1ui.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP1ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

/// [glVertexAttribP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP1uiv.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP1uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

/// [glVertexAttribP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP2ui.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP2ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

/// [glVertexAttribP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP2uiv.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP2uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

/// [glVertexAttribP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP3ui.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP3ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

/// [glVertexAttribP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP3uiv.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP3uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

/// [glVertexAttribP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP4ui.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP4ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

/// [glVertexAttribP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP4uiv.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP4uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

/// [glVertexAttribParameteriAMD](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribParameteriAMD.xhtml)
pub type glVertexAttribParameteriAMD_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, param: GLint);

/// [glVertexAttribPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexAttribPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void);

/// [glVertexAttribPointerARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointerARB.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexAttribPointerARB_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void);

/// [glVertexAttribPointerNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointerNV.xhtml)
/// * `type` group: VertexAttribEnumNV
/// * `pointer` len: COMPSIZE(fsize,type,stride)
pub type glVertexAttribPointerNV_t = unsafe extern "system" fn(index: GLuint, fsize: GLint, type_: VertexAttribEnumNV, stride: GLsizei, pointer: *const void);

/// [glVertexAttribs1dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs1dvNV.xhtml)
/// * `v` len: count
pub type glVertexAttribs1dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

/// [glVertexAttribs1fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs1fvNV.xhtml)
/// * `v` len: count
pub type glVertexAttribs1fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

/// [glVertexAttribs1hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs1hvNV.xhtml)
/// * `v` group: Half16NV
/// * `v` len: n
pub type glVertexAttribs1hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

/// [glVertexAttribs1svNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs1svNV.xhtml)
/// * `v` len: count
pub type glVertexAttribs1svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

/// [glVertexAttribs2dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs2dvNV.xhtml)
/// * `v` len: count*2
pub type glVertexAttribs2dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

/// [glVertexAttribs2fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs2fvNV.xhtml)
/// * `v` len: count*2
pub type glVertexAttribs2fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

/// [glVertexAttribs2hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs2hvNV.xhtml)
/// * `v` group: Half16NV
/// * `v` len: n
pub type glVertexAttribs2hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

/// [glVertexAttribs2svNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs2svNV.xhtml)
/// * `v` len: count*2
pub type glVertexAttribs2svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

/// [glVertexAttribs3dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs3dvNV.xhtml)
/// * `v` len: count*3
pub type glVertexAttribs3dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

/// [glVertexAttribs3fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs3fvNV.xhtml)
/// * `v` len: count*3
pub type glVertexAttribs3fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

/// [glVertexAttribs3hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs3hvNV.xhtml)
/// * `v` group: Half16NV
/// * `v` len: n
pub type glVertexAttribs3hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

/// [glVertexAttribs3svNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs3svNV.xhtml)
/// * `v` len: count*3
pub type glVertexAttribs3svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

/// [glVertexAttribs4dvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs4dvNV.xhtml)
/// * `v` len: count*4
pub type glVertexAttribs4dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

/// [glVertexAttribs4fvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs4fvNV.xhtml)
/// * `v` len: count*4
pub type glVertexAttribs4fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

/// [glVertexAttribs4hvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs4hvNV.xhtml)
/// * `v` group: Half16NV
/// * `v` len: n
pub type glVertexAttribs4hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

/// [glVertexAttribs4svNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs4svNV.xhtml)
/// * `v` len: count*4
pub type glVertexAttribs4svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

/// [glVertexAttribs4ubvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribs4ubvNV.xhtml)
/// * `v` group: ColorUB
/// * `v` len: count*4
pub type glVertexAttribs4ubvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLubyte);

/// [glVertexBindingDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexBindingDivisor.xhtml)
pub type glVertexBindingDivisor_t = unsafe extern "system" fn(bindingindex: GLuint, divisor: GLuint);

/// [glVertexBlendARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexBlendARB.xhtml)
pub type glVertexBlendARB_t = unsafe extern "system" fn(count: GLint);

/// [glVertexBlendEnvfATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexBlendEnvfATI.xhtml)
/// * `pname` group: VertexStreamATI
pub type glVertexBlendEnvfATI_t = unsafe extern "system" fn(pname: VertexStreamATI, param: GLfloat);

/// [glVertexBlendEnviATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexBlendEnviATI.xhtml)
/// * `pname` group: VertexStreamATI
pub type glVertexBlendEnviATI_t = unsafe extern "system" fn(pname: VertexStreamATI, param: GLint);

/// [glVertexFormatNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexFormatNV.xhtml)
/// * `type` group: VertexPointerType
pub type glVertexFormatNV_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, stride: GLsizei);

/// [glVertexP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP2ui.xhtml)
/// * `type` group: VertexPointerType
pub type glVertexP2ui_t = unsafe extern "system" fn(type_: VertexPointerType, value: GLuint);

/// [glVertexP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP2uiv.xhtml)
/// * `type` group: VertexPointerType
pub type glVertexP2uiv_t = unsafe extern "system" fn(type_: VertexPointerType, value: *const GLuint);

/// [glVertexP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP3ui.xhtml)
/// * `type` group: VertexPointerType
pub type glVertexP3ui_t = unsafe extern "system" fn(type_: VertexPointerType, value: GLuint);

/// [glVertexP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP3uiv.xhtml)
/// * `type` group: VertexPointerType
pub type glVertexP3uiv_t = unsafe extern "system" fn(type_: VertexPointerType, value: *const GLuint);

/// [glVertexP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP4ui.xhtml)
/// * `type` group: VertexPointerType
pub type glVertexP4ui_t = unsafe extern "system" fn(type_: VertexPointerType, value: GLuint);

/// [glVertexP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP4uiv.xhtml)
/// * `type` group: VertexPointerType
pub type glVertexP4uiv_t = unsafe extern "system" fn(type_: VertexPointerType, value: *const GLuint);

/// [glVertexPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexPointer.xhtml)
/// * `type` group: VertexPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexPointer_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, stride: GLsizei, pointer: *const void);

/// [glVertexPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexPointerEXT.xhtml)
/// * `type` group: VertexPointerType
/// * `pointer` len: COMPSIZE(size,type,stride,count)
pub type glVertexPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// [glVertexPointerListIBM](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexPointerListIBM.xhtml)
/// * `type` group: VertexPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// [glVertexPointervINTEL](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexPointervINTEL.xhtml)
/// * `type` group: VertexPointerType
pub type glVertexPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, pointer: *const [*mut void; 4]);

/// [glVertexStream1dATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream1dATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream1dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLdouble);

/// [glVertexStream1dvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream1dvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream1dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const GLdouble);

/// [glVertexStream1fATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream1fATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream1fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLfloat);

/// [glVertexStream1fvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream1fvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream1fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const GLfloat);

/// [glVertexStream1iATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream1iATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream1iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLint);

/// [glVertexStream1ivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream1ivATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream1ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const GLint);

/// [glVertexStream1sATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream1sATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream1sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLshort);

/// [glVertexStream1svATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream1svATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream1svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const GLshort);

/// [glVertexStream2dATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream2dATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream2dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLdouble, y: GLdouble);

/// [glVertexStream2dvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream2dvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream2dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLdouble; 2]);

/// [glVertexStream2fATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream2fATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream2fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLfloat, y: GLfloat);

/// [glVertexStream2fvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream2fvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream2fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLfloat; 2]);

/// [glVertexStream2iATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream2iATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream2iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLint, y: GLint);

/// [glVertexStream2ivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream2ivATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream2ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLint; 2]);

/// [glVertexStream2sATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream2sATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream2sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLshort, y: GLshort);

/// [glVertexStream2svATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream2svATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream2svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLshort; 2]);

/// [glVertexStream3dATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream3dATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream3dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLdouble, y: GLdouble, z: GLdouble);

/// [glVertexStream3dvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream3dvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream3dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLdouble; 3]);

/// [glVertexStream3fATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream3fATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream3fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLfloat, y: GLfloat, z: GLfloat);

/// [glVertexStream3fvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream3fvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream3fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLfloat; 3]);

/// [glVertexStream3iATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream3iATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream3iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLint, y: GLint, z: GLint);

/// [glVertexStream3ivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream3ivATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream3ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLint; 3]);

/// [glVertexStream3sATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream3sATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream3sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLshort, y: GLshort, z: GLshort);

/// [glVertexStream3svATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream3svATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream3svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLshort; 3]);

/// [glVertexStream4dATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream4dATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream4dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glVertexStream4dvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream4dvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream4dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLdouble; 4]);

/// [glVertexStream4fATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream4fATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream4fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glVertexStream4fvATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream4fvATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream4fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLfloat; 4]);

/// [glVertexStream4iATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream4iATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream4iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLint, y: GLint, z: GLint, w: GLint);

/// [glVertexStream4ivATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream4ivATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream4ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLint; 4]);

/// [glVertexStream4sATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream4sATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream4sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// [glVertexStream4svATI](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexStream4svATI.xhtml)
/// * `stream` group: VertexStreamATI
pub type glVertexStream4svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLshort; 4]);

/// [glVertexWeightPointerEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexWeightPointerEXT.xhtml)
/// * `type` group: VertexWeightPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glVertexWeightPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: VertexWeightPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// [glVertexWeightfEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexWeightfEXT.xhtml)
pub type glVertexWeightfEXT_t = unsafe extern "system" fn(weight: GLfloat);

/// [glVertexWeightfvEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexWeightfvEXT.xhtml)
pub type glVertexWeightfvEXT_t = unsafe extern "system" fn(weight: *const GLfloat);

/// [glVertexWeighthNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexWeighthNV.xhtml)
/// * `weight` group: Half16NV
pub type glVertexWeighthNV_t = unsafe extern "system" fn(weight: GLhalfNV);

/// [glVertexWeighthvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexWeighthvNV.xhtml)
/// * `weight` group: Half16NV
pub type glVertexWeighthvNV_t = unsafe extern "system" fn(weight: *const GLhalfNV);

/// [glVideoCaptureNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVideoCaptureNV.xhtml)
pub type glVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, sequence_num: *mut GLuint, capture_time: *mut GLuint64EXT) -> GLenum;

/// [glVideoCaptureStreamParameterdvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVideoCaptureStreamParameterdvNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glVideoCaptureStreamParameterdvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLdouble);

/// [glVideoCaptureStreamParameterfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVideoCaptureStreamParameterfvNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glVideoCaptureStreamParameterfvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLfloat);

/// [glVideoCaptureStreamParameterivNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVideoCaptureStreamParameterivNV.xhtml)
/// * `params` len: COMPSIZE(pname)
pub type glVideoCaptureStreamParameterivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLint);

/// [glViewport](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glViewport_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// [glViewportArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportArrayv.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glViewportArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// [glViewportArrayvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportArrayvNV.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glViewportArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// [glViewportArrayvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportArrayvOES.xhtml)
/// * `v` len: COMPSIZE(count)
pub type glViewportArrayvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// [glViewportIndexedf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedf.xhtml)
pub type glViewportIndexedf_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

/// [glViewportIndexedfNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedfNV.xhtml)
pub type glViewportIndexedfNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

/// [glViewportIndexedfOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedfOES.xhtml)
pub type glViewportIndexedfOES_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

/// [glViewportIndexedfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedfv.xhtml)
pub type glViewportIndexedfv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// [glViewportIndexedfvNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedfvNV.xhtml)
pub type glViewportIndexedfvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// [glViewportIndexedfvOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedfvOES.xhtml)
pub type glViewportIndexedfvOES_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// [glViewportPositionWScaleNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportPositionWScaleNV.xhtml)
pub type glViewportPositionWScaleNV_t = unsafe extern "system" fn(index: GLuint, xcoeff: GLfloat, ycoeff: GLfloat);

/// [glViewportSwizzleNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportSwizzleNV.xhtml)
pub type glViewportSwizzleNV_t = unsafe extern "system" fn(index: GLuint, swizzlex: GLenum, swizzley: GLenum, swizzlez: GLenum, swizzlew: GLenum);

/// [glWaitSemaphoreEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWaitSemaphoreEXT.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: COMPSIZE(numBufferBarriers)
/// * `textures` class: texture
/// * `textures` len: COMPSIZE(numTextureBarriers)
/// * `srcLayouts` group: TextureLayout
/// * `srcLayouts` len: COMPSIZE(numTextureBarriers)
pub type glWaitSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint, numBufferBarriers: GLuint, buffers: *const GLuint, numTextureBarriers: GLuint, textures: *const GLuint, srcLayouts: *const TextureLayout);

/// [glWaitSemaphoreui64NVX](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWaitSemaphoreui64NVX.xhtml)
/// * `semaphoreArray` len: fenceObjectCount
/// * `fenceValueArray` len: fenceObjectCount
pub type glWaitSemaphoreui64NVX_t = unsafe extern "system" fn(waitGpu: GLuint, fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

/// [glWaitSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWaitSync.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncBehaviorFlags
pub type glWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

/// [glWaitSyncAPPLE](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWaitSyncAPPLE.xhtml)
/// * `sync` class: sync
/// * `flags` group: SyncBehaviorFlags
pub type glWaitSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

/// [glWaitVkSemaphoreNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWaitVkSemaphoreNV.xhtml)
pub type glWaitVkSemaphoreNV_t = unsafe extern "system" fn(vkSemaphore: GLuint64);

/// [glWeightPathsNV](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightPathsNV.xhtml)
/// * `resultPath` group: Path
/// * `paths` group: Path
/// * `paths` len: numPaths
/// * `weights` len: numPaths
pub type glWeightPathsNV_t = unsafe extern "system" fn(resultPath: GLuint, numPaths: GLsizei, paths: *const GLuint, weights: *const GLfloat);

/// [glWeightPointerARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightPointerARB.xhtml)
/// * `type` group: WeightPointerTypeARB
/// * `pointer` len: COMPSIZE(type,stride)
pub type glWeightPointerARB_t = unsafe extern "system" fn(size: GLint, type_: WeightPointerTypeARB, stride: GLsizei, pointer: *const void);

/// [glWeightPointerOES](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightPointerOES.xhtml)
/// * `pointer` len: COMPSIZE(type,stride)
pub type glWeightPointerOES_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

/// [glWeightbvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightbvARB.xhtml)
/// * `weights` len: size
pub type glWeightbvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLbyte);

/// [glWeightdvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightdvARB.xhtml)
/// * `weights` len: size
pub type glWeightdvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLdouble);

/// [glWeightfvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightfvARB.xhtml)
/// * `weights` len: size
pub type glWeightfvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLfloat);

/// [glWeightivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightivARB.xhtml)
/// * `weights` len: size
pub type glWeightivARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLint);

/// [glWeightsvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightsvARB.xhtml)
/// * `weights` len: size
pub type glWeightsvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLshort);

/// [glWeightubvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightubvARB.xhtml)
/// * `weights` len: size
pub type glWeightubvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLubyte);

/// [glWeightuivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightuivARB.xhtml)
/// * `weights` len: size
pub type glWeightuivARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLuint);

/// [glWeightusvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWeightusvARB.xhtml)
/// * `weights` len: size
pub type glWeightusvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLushort);

/// [glWindowPos2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2d.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glWindowPos2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// [glWindowPos2dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2dARB.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glWindowPos2dARB_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// [glWindowPos2dMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2dMESA.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glWindowPos2dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// [glWindowPos2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2dv.xhtml)
/// * `v` group: CoordD
pub type glWindowPos2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// [glWindowPos2dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2dvARB.xhtml)
/// * `v` group: CoordD
pub type glWindowPos2dvARB_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// [glWindowPos2dvMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2dvMESA.xhtml)
/// * `v` group: CoordD
pub type glWindowPos2dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// [glWindowPos2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2f.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glWindowPos2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// [glWindowPos2fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2fARB.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glWindowPos2fARB_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// [glWindowPos2fMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2fMESA.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glWindowPos2fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// [glWindowPos2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2fv.xhtml)
/// * `v` group: CoordF
pub type glWindowPos2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// [glWindowPos2fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2fvARB.xhtml)
/// * `v` group: CoordF
pub type glWindowPos2fvARB_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// [glWindowPos2fvMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2fvMESA.xhtml)
/// * `v` group: CoordF
pub type glWindowPos2fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// [glWindowPos2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2i.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glWindowPos2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// [glWindowPos2iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2iARB.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glWindowPos2iARB_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// [glWindowPos2iMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2iMESA.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glWindowPos2iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// [glWindowPos2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2iv.xhtml)
/// * `v` group: CoordI
pub type glWindowPos2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// [glWindowPos2ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2ivARB.xhtml)
/// * `v` group: CoordI
pub type glWindowPos2ivARB_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// [glWindowPos2ivMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2ivMESA.xhtml)
/// * `v` group: CoordI
pub type glWindowPos2ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// [glWindowPos2s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2s.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glWindowPos2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// [glWindowPos2sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2sARB.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glWindowPos2sARB_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// [glWindowPos2sMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2sMESA.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glWindowPos2sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// [glWindowPos2sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2sv.xhtml)
/// * `v` group: CoordS
pub type glWindowPos2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// [glWindowPos2svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2svARB.xhtml)
/// * `v` group: CoordS
pub type glWindowPos2svARB_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// [glWindowPos2svMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos2svMESA.xhtml)
/// * `v` group: CoordS
pub type glWindowPos2svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// [glWindowPos3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3d.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glWindowPos3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// [glWindowPos3dARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3dARB.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glWindowPos3dARB_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// [glWindowPos3dMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3dMESA.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glWindowPos3dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// [glWindowPos3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3dv.xhtml)
/// * `v` group: CoordD
pub type glWindowPos3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glWindowPos3dvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3dvARB.xhtml)
/// * `v` group: CoordD
pub type glWindowPos3dvARB_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glWindowPos3dvMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3dvMESA.xhtml)
/// * `v` group: CoordD
pub type glWindowPos3dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// [glWindowPos3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3f.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glWindowPos3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// [glWindowPos3fARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3fARB.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glWindowPos3fARB_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// [glWindowPos3fMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3fMESA.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glWindowPos3fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// [glWindowPos3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3fv.xhtml)
/// * `v` group: CoordF
pub type glWindowPos3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glWindowPos3fvARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3fvARB.xhtml)
/// * `v` group: CoordF
pub type glWindowPos3fvARB_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glWindowPos3fvMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3fvMESA.xhtml)
/// * `v` group: CoordF
pub type glWindowPos3fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// [glWindowPos3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3i.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glWindowPos3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// [glWindowPos3iARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3iARB.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glWindowPos3iARB_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// [glWindowPos3iMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3iMESA.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glWindowPos3iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// [glWindowPos3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3iv.xhtml)
/// * `v` group: CoordI
pub type glWindowPos3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glWindowPos3ivARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3ivARB.xhtml)
/// * `v` group: CoordI
pub type glWindowPos3ivARB_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glWindowPos3ivMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3ivMESA.xhtml)
/// * `v` group: CoordI
pub type glWindowPos3ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// [glWindowPos3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3s.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glWindowPos3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// [glWindowPos3sARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3sARB.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glWindowPos3sARB_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// [glWindowPos3sMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3sMESA.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glWindowPos3sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// [glWindowPos3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3sv.xhtml)
/// * `v` group: CoordS
pub type glWindowPos3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glWindowPos3svARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3svARB.xhtml)
/// * `v` group: CoordS
pub type glWindowPos3svARB_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glWindowPos3svMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos3svMESA.xhtml)
/// * `v` group: CoordS
pub type glWindowPos3svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// [glWindowPos4dMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos4dMESA.xhtml)
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
/// * `w` group: CoordD
pub type glWindowPos4dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// [glWindowPos4dvMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos4dvMESA.xhtml)
/// * `v` group: CoordD
pub type glWindowPos4dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// [glWindowPos4fMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos4fMESA.xhtml)
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
/// * `w` group: CoordF
pub type glWindowPos4fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// [glWindowPos4fvMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos4fvMESA.xhtml)
/// * `v` group: CoordF
pub type glWindowPos4fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// [glWindowPos4iMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos4iMESA.xhtml)
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
/// * `w` group: CoordI
pub type glWindowPos4iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

/// [glWindowPos4ivMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos4ivMESA.xhtml)
/// * `v` group: CoordI
pub type glWindowPos4ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// [glWindowPos4sMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos4sMESA.xhtml)
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
/// * `w` group: CoordS
pub type glWindowPos4sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// [glWindowPos4svMESA](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowPos4svMESA.xhtml)
/// * `v` group: CoordS
pub type glWindowPos4svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// [glWindowRectanglesEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWindowRectanglesEXT.xhtml)
/// * `box` len: COMPSIZE(count)
pub type glWindowRectanglesEXT_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, box_: *const GLint);

/// [glWriteMaskEXT](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWriteMaskEXT.xhtml)
/// * `outX` group: VertexShaderWriteMaskEXT
/// * `outY` group: VertexShaderWriteMaskEXT
/// * `outZ` group: VertexShaderWriteMaskEXT
/// * `outW` group: VertexShaderWriteMaskEXT
pub type glWriteMaskEXT_t = unsafe extern "system" fn(res: GLuint, in_: GLuint, outX: VertexShaderWriteMaskEXT, outY: VertexShaderWriteMaskEXT, outZ: VertexShaderWriteMaskEXT, outW: VertexShaderWriteMaskEXT);
