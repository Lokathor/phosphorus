use super::*;

/// glAccum
/// * `op` group: AccumOp
/// * `value` group: CoordF
pub type glAccum_t = unsafe extern "system" fn(op: AccumOp, value: GLfloat);

/// glAccumxOES
pub type glAccumxOES_t = unsafe extern "system" fn(op: GLenum, value: GLfixed);

/// glAcquireKeyedMutexWin32EXT
pub type glAcquireKeyedMutexWin32EXT_t = unsafe extern "system" fn(memory: GLuint, key: GLuint64, timeout: GLuint) -> GLboolean;

/// glActiveProgramEXT
/// * `program` class: program
pub type glActiveProgramEXT_t = unsafe extern "system" fn(program: GLuint);

/// glActiveShaderProgram
/// * `pipeline` class: program pipeline
/// * `program` class: program
pub type glActiveShaderProgram_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);

/// glActiveShaderProgramEXT
/// * `pipeline` class: program pipeline
/// * `program` class: program
pub type glActiveShaderProgramEXT_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);

/// glActiveStencilFaceEXT
/// * `face` group: StencilFaceDirection
pub type glActiveStencilFaceEXT_t = unsafe extern "system" fn(face: StencilFaceDirection);

/// glActiveTexture
/// * `texture` group: TextureUnit
pub type glActiveTexture_t = unsafe extern "system" fn(texture: TextureUnit);

/// glActiveTextureARB
/// * `texture` group: TextureUnit
pub type glActiveTextureARB_t = unsafe extern "system" fn(texture: TextureUnit);

/// glActiveVaryingNV
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glActiveVaryingNV_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar);

/// glAlphaFragmentOp1ATI
/// * `op` group: FragmentOpATI
pub type glAlphaFragmentOp1ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint);

/// glAlphaFragmentOp2ATI
/// * `op` group: FragmentOpATI
pub type glAlphaFragmentOp2ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint);

/// glAlphaFragmentOp3ATI
/// * `op` group: FragmentOpATI
pub type glAlphaFragmentOp3ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint, arg3: GLuint, arg3Rep: GLuint, arg3Mod: GLuint);

/// glAlphaFunc
/// * `func` group: AlphaFunction
pub type glAlphaFunc_t = unsafe extern "system" fn(func: AlphaFunction, ref_: GLfloat);

/// glAlphaFuncQCOM
pub type glAlphaFuncQCOM_t = unsafe extern "system" fn(func: GLenum, ref_: GLclampf);

/// glAlphaFuncx
/// * `func` group: AlphaFunction
pub type glAlphaFuncx_t = unsafe extern "system" fn(func: AlphaFunction, ref_: GLfixed);

/// glAlphaFuncxOES
/// * `func` group: AlphaFunction
/// * `ref` group: ClampedFixed
pub type glAlphaFuncxOES_t = unsafe extern "system" fn(func: AlphaFunction, ref_: GLfixed);

/// glAlphaToCoverageDitherControlNV
pub type glAlphaToCoverageDitherControlNV_t = unsafe extern "system" fn(mode: GLenum);

/// glApplyFramebufferAttachmentCMAAINTEL
pub type glApplyFramebufferAttachmentCMAAINTEL_t = unsafe extern "system" fn();

/// glApplyTextureEXT
/// * `mode` group: LightTextureModeEXT
pub type glApplyTextureEXT_t = unsafe extern "system" fn(mode: LightTextureModeEXT);

/// glAreProgramsResidentNV
/// * `programs` class: program
/// * `programs` len: n
/// * `residences` group: Boolean
/// * `residences` len: n
pub type glAreProgramsResidentNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint, residences: *mut GLboolean) -> GLboolean;

/// glAreTexturesResident
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
/// * `residences` group: Boolean
/// * `residences` len: n
pub type glAreTexturesResident_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean;

/// glAreTexturesResidentEXT
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
/// * `residences` group: Boolean
/// * `residences` len: n
pub type glAreTexturesResidentEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean;

/// glArrayElement
pub type glArrayElement_t = unsafe extern "system" fn(i: GLint);

/// glArrayElementEXT
pub type glArrayElementEXT_t = unsafe extern "system" fn(i: GLint);

/// glArrayObjectATI
/// * `array` group: EnableCap
/// * `type` group: ScalarType
/// * `buffer` class: buffer
pub type glArrayObjectATI_t = unsafe extern "system" fn(array: EnableCap, size: GLint, type_: ScalarType, stride: GLsizei, buffer: GLuint, offset: GLuint);

/// glAsyncCopyBufferSubDataNVX
/// * `waitSemaphoreArray` len: waitSemaphoreCount
/// * `fenceValueArray` len: waitSemaphoreCount
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
/// * `signalSemaphoreArray` len: signalSemaphoreCount
/// * `signalValueArray` len: signalSemaphoreCount
pub type glAsyncCopyBufferSubDataNVX_t = unsafe extern "system" fn(waitSemaphoreCount: GLsizei, waitSemaphoreArray: *const GLuint, fenceValueArray: *const GLuint64, readGpu: GLuint, writeGpuMask: GLbitfield, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr, signalSemaphoreCount: GLsizei, signalSemaphoreArray: *const GLuint, signalValueArray: *const GLuint64) -> GLuint;

/// glAsyncCopyImageSubDataNVX
/// * `waitSemaphoreArray` len: waitSemaphoreCount
/// * `waitValueArray` len: waitSemaphoreCount
/// * `signalSemaphoreArray` len: signalSemaphoreCount
/// * `signalValueArray` len: signalSemaphoreCount
pub type glAsyncCopyImageSubDataNVX_t = unsafe extern "system" fn(waitSemaphoreCount: GLsizei, waitSemaphoreArray: *const GLuint, waitValueArray: *const GLuint64, srcGpu: GLuint, dstGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei, signalSemaphoreCount: GLsizei, signalSemaphoreArray: *const GLuint, signalValueArray: *const GLuint64) -> GLuint;

/// glAsyncMarkerSGIX
pub type glAsyncMarkerSGIX_t = unsafe extern "system" fn(marker: GLuint);

/// glAttachObjectARB
/// * `containerObj` group: handleARB
/// * `obj` group: handleARB
pub type glAttachObjectARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, obj: GLhandleARB);

/// glAttachShader
/// * `program` class: program
/// * `shader` class: shader
pub type glAttachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);

/// glBegin
/// * `mode` group: PrimitiveType
pub type glBegin_t = unsafe extern "system" fn(mode: PrimitiveType);

/// glBeginConditionalRender
/// * `mode` group: ConditionalRenderMode
pub type glBeginConditionalRender_t = unsafe extern "system" fn(id: GLuint, mode: ConditionalRenderMode);

/// glBeginConditionalRenderNV
/// * `mode` group: ConditionalRenderMode
pub type glBeginConditionalRenderNV_t = unsafe extern "system" fn(id: GLuint, mode: ConditionalRenderMode);

/// glBeginConditionalRenderNVX
pub type glBeginConditionalRenderNVX_t = unsafe extern "system" fn(id: GLuint);

/// glBeginFragmentShaderATI
pub type glBeginFragmentShaderATI_t = unsafe extern "system" fn();

/// glBeginOcclusionQueryNV
pub type glBeginOcclusionQueryNV_t = unsafe extern "system" fn(id: GLuint);

/// glBeginPerfMonitorAMD
pub type glBeginPerfMonitorAMD_t = unsafe extern "system" fn(monitor: GLuint);

/// glBeginPerfQueryINTEL
pub type glBeginPerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

/// glBeginQuery
/// * `target` group: QueryTarget
/// * `id` class: query
pub type glBeginQuery_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint);

/// glBeginQueryARB
/// * `target` group: QueryTarget
/// * `id` class: query
pub type glBeginQueryARB_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint);

/// glBeginQueryEXT
/// * `target` group: QueryTarget
/// * `id` class: query
pub type glBeginQueryEXT_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint);

/// glBeginQueryIndexed
/// * `target` group: QueryTarget
/// * `id` class: query
pub type glBeginQueryIndexed_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint, id: GLuint);

/// glBeginTransformFeedback
/// * `primitiveMode` group: PrimitiveType
pub type glBeginTransformFeedback_t = unsafe extern "system" fn(primitiveMode: PrimitiveType);

/// glBeginTransformFeedbackEXT
/// * `primitiveMode` group: PrimitiveType
pub type glBeginTransformFeedbackEXT_t = unsafe extern "system" fn(primitiveMode: PrimitiveType);

/// glBeginTransformFeedbackNV
/// * `primitiveMode` group: PrimitiveType
pub type glBeginTransformFeedbackNV_t = unsafe extern "system" fn(primitiveMode: PrimitiveType);

/// glBeginVertexShaderEXT
pub type glBeginVertexShaderEXT_t = unsafe extern "system" fn();

/// glBeginVideoCaptureNV
pub type glBeginVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint);

/// glBindAttribLocation
/// * `program` class: program
pub type glBindAttribLocation_t = unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);

/// glBindAttribLocationARB
/// * `programObj` group: handleARB
pub type glBindAttribLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, name: *const GLcharARB);

/// glBindBuffer
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBuffer_t = unsafe extern "system" fn(target: BufferTargetARB, buffer: GLuint);

/// glBindBufferARB
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBufferARB_t = unsafe extern "system" fn(target: BufferTargetARB, buffer: GLuint);

/// glBindBufferBase
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBufferBase_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint);

/// glBindBufferBaseEXT
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBufferBaseEXT_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint);

/// glBindBufferBaseNV
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
pub type glBindBufferBaseNV_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint);

/// glBindBufferOffsetEXT
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
pub type glBindBufferOffsetEXT_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr);

/// glBindBufferOffsetNV
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
pub type glBindBufferOffsetNV_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr);

/// glBindBufferRange
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glBindBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glBindBufferRangeEXT
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glBindBufferRangeEXT_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glBindBufferRangeNV
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glBindBufferRangeNV_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glBindBuffersBase
/// * `target` group: BufferTargetARB
/// * `buffers` class: buffer
/// * `buffers` len: count
pub type glBindBuffersBase_t = unsafe extern "system" fn(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint);

/// glBindBuffersRange
/// * `target` group: BufferTargetARB
/// * `buffers` class: buffer
/// * `buffers` len: count
/// * `offsets` len: count
/// * `sizes` len: count
pub type glBindBuffersRange_t = unsafe extern "system" fn(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);

/// glBindFragDataLocation
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glBindFragDataLocation_t = unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);

/// glBindFragDataLocationEXT
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glBindFragDataLocationEXT_t = unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);

/// glBindFragDataLocationIndexed
/// * `program` class: program
pub type glBindFragDataLocationIndexed_t = unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);

/// glBindFragDataLocationIndexedEXT
/// * `program` class: program
pub type glBindFragDataLocationIndexedEXT_t = unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);

/// glBindFragmentShaderATI
pub type glBindFragmentShaderATI_t = unsafe extern "system" fn(id: GLuint);

/// glBindFramebuffer
/// * `target` group: FramebufferTarget
/// * `framebuffer` class: framebuffer
pub type glBindFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, framebuffer: GLuint);

/// glBindFramebufferEXT
/// * `target` group: FramebufferTarget
/// * `framebuffer` class: framebuffer
pub type glBindFramebufferEXT_t = unsafe extern "system" fn(target: FramebufferTarget, framebuffer: GLuint);

/// glBindFramebufferOES
/// * `target` group: FramebufferTarget
/// * `framebuffer` class: framebuffer
pub type glBindFramebufferOES_t = unsafe extern "system" fn(target: FramebufferTarget, framebuffer: GLuint);

/// glBindImageTexture
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `access` group: BufferAccessARB
/// * `format` group: InternalFormat
pub type glBindImageTexture_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: BufferAccessARB, format: InternalFormat);

/// glBindImageTextureEXT
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `access` group: BufferAccessARB
pub type glBindImageTextureEXT_t = unsafe extern "system" fn(index: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: BufferAccessARB, format: GLint);

/// glBindImageTextures
/// * `textures` class: texture
/// * `textures` len: count
pub type glBindImageTextures_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);

/// glBindLightParameterEXT
/// * `light` group: LightName
/// * `value` group: LightParameter
pub type glBindLightParameterEXT_t = unsafe extern "system" fn(light: LightName, value: LightParameter) -> GLuint;

/// glBindMaterialParameterEXT
/// * `face` group: MaterialFace
/// * `value` group: MaterialParameter
pub type glBindMaterialParameterEXT_t = unsafe extern "system" fn(face: MaterialFace, value: MaterialParameter) -> GLuint;

/// glBindMultiTextureEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glBindMultiTextureEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, texture: GLuint);

/// glBindParameterEXT
/// * `value` group: VertexShaderParameterEXT
pub type glBindParameterEXT_t = unsafe extern "system" fn(value: VertexShaderParameterEXT) -> GLuint;

/// glBindProgramARB
/// * `target` group: ProgramTarget
/// * `program` class: program
pub type glBindProgramARB_t = unsafe extern "system" fn(target: ProgramTarget, program: GLuint);

/// glBindProgramNV
/// * `target` group: VertexAttribEnumNV
/// * `id` class: program
pub type glBindProgramNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, id: GLuint);

/// glBindProgramPipeline
/// * `pipeline` class: program pipeline
pub type glBindProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);

/// glBindProgramPipelineEXT
/// * `pipeline` class: program pipeline
pub type glBindProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint);

/// glBindRenderbuffer
/// * `target` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glBindRenderbuffer_t = unsafe extern "system" fn(target: RenderbufferTarget, renderbuffer: GLuint);

/// glBindRenderbufferEXT
/// * `target` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glBindRenderbufferEXT_t = unsafe extern "system" fn(target: RenderbufferTarget, renderbuffer: GLuint);

/// glBindRenderbufferOES
/// * `target` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glBindRenderbufferOES_t = unsafe extern "system" fn(target: RenderbufferTarget, renderbuffer: GLuint);

/// glBindSampler
/// * `sampler` class: sampler
pub type glBindSampler_t = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);

/// glBindSamplers
/// * `samplers` class: sampler
/// * `samplers` len: count
pub type glBindSamplers_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint);

/// glBindShadingRateImageNV
/// * `texture` class: texture
pub type glBindShadingRateImageNV_t = unsafe extern "system" fn(texture: GLuint);

/// glBindTexGenParameterEXT
/// * `unit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `value` group: TextureGenParameter
pub type glBindTexGenParameterEXT_t = unsafe extern "system" fn(unit: TextureUnit, coord: TextureCoordName, value: TextureGenParameter) -> GLuint;

/// glBindTexture
/// * `target` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glBindTexture_t = unsafe extern "system" fn(target: TextureTarget, texture: GLuint);

/// glBindTextureEXT
/// * `target` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glBindTextureEXT_t = unsafe extern "system" fn(target: TextureTarget, texture: GLuint);

/// glBindTextureUnit
/// * `texture` class: texture
pub type glBindTextureUnit_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint);

/// glBindTextureUnitParameterEXT
/// * `unit` group: TextureUnit
/// * `value` group: VertexShaderTextureUnitParameter
pub type glBindTextureUnitParameterEXT_t = unsafe extern "system" fn(unit: TextureUnit, value: VertexShaderTextureUnitParameter) -> GLuint;

/// glBindTextures
/// * `textures` class: texture
/// * `textures` len: count
pub type glBindTextures_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);

/// glBindTransformFeedback
/// * `target` group: BindTransformFeedbackTarget
/// * `id` class: transform feedback
pub type glBindTransformFeedback_t = unsafe extern "system" fn(target: BindTransformFeedbackTarget, id: GLuint);

/// glBindTransformFeedbackNV
/// * `target` group: BufferTargetARB
/// * `id` class: transform feedback
pub type glBindTransformFeedbackNV_t = unsafe extern "system" fn(target: BufferTargetARB, id: GLuint);

/// glBindVertexArray
/// * `array` class: vertex array
pub type glBindVertexArray_t = unsafe extern "system" fn(array: GLuint);

/// glBindVertexArrayAPPLE
/// * `array` class: vertex array
pub type glBindVertexArrayAPPLE_t = unsafe extern "system" fn(array: GLuint);

/// glBindVertexArrayOES
/// * `array` class: vertex array
pub type glBindVertexArrayOES_t = unsafe extern "system" fn(array: GLuint);

/// glBindVertexBuffer
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
pub type glBindVertexBuffer_t = unsafe extern "system" fn(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

/// glBindVertexBuffers
/// * `buffers` class: buffer
/// * `buffers` len: count
/// * `offsets` len: count
/// * `strides` len: count
pub type glBindVertexBuffers_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);

/// glBindVertexShaderEXT
pub type glBindVertexShaderEXT_t = unsafe extern "system" fn(id: GLuint);

/// glBindVideoCaptureStreamBufferNV
/// * `offset` group: BufferOffsetARB
pub type glBindVideoCaptureStreamBufferNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, frame_region: GLenum, offset: GLintptrARB);

/// glBindVideoCaptureStreamTextureNV
/// * `texture` class: texture
pub type glBindVideoCaptureStreamTextureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, frame_region: GLenum, target: GLenum, texture: GLuint);

/// glBinormal3bEXT
pub type glBinormal3bEXT_t = unsafe extern "system" fn(bx: GLbyte, by: GLbyte, bz: GLbyte);

/// glBinormal3bvEXT
pub type glBinormal3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// glBinormal3dEXT
/// * `bx` group: CoordD
/// * `by` group: CoordD
/// * `bz` group: CoordD
pub type glBinormal3dEXT_t = unsafe extern "system" fn(bx: GLdouble, by: GLdouble, bz: GLdouble);

/// glBinormal3dvEXT
/// * `v` group: CoordD
pub type glBinormal3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glBinormal3fEXT
/// * `bx` group: CoordF
/// * `by` group: CoordF
/// * `bz` group: CoordF
pub type glBinormal3fEXT_t = unsafe extern "system" fn(bx: GLfloat, by: GLfloat, bz: GLfloat);

/// glBinormal3fvEXT
/// * `v` group: CoordF
pub type glBinormal3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glBinormal3iEXT
pub type glBinormal3iEXT_t = unsafe extern "system" fn(bx: GLint, by: GLint, bz: GLint);

/// glBinormal3ivEXT
pub type glBinormal3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glBinormal3sEXT
pub type glBinormal3sEXT_t = unsafe extern "system" fn(bx: GLshort, by: GLshort, bz: GLshort);

/// glBinormal3svEXT
pub type glBinormal3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glBinormalPointerEXT
/// * `type` group: BinormalPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glBinormalPointerEXT_t = unsafe extern "system" fn(type_: BinormalPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// glBitmap
/// * `xorig` group: CoordF
/// * `yorig` group: CoordF
/// * `xmove` group: CoordF
/// * `ymove` group: CoordF
/// * `bitmap` len: COMPSIZE(width,height)
pub type glBitmap_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *const GLubyte);

/// glBitmapxOES
/// * `bitmap` len: COMPSIZE(width,height)
pub type glBitmapxOES_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, xorig: GLfixed, yorig: GLfixed, xmove: GLfixed, ymove: GLfixed, bitmap: *const GLubyte);

/// glBlendBarrier
pub type glBlendBarrier_t = unsafe extern "system" fn();

/// glBlendBarrierKHR
pub type glBlendBarrierKHR_t = unsafe extern "system" fn();

/// glBlendBarrierNV
pub type glBlendBarrierNV_t = unsafe extern "system" fn();

/// glBlendColor
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
pub type glBlendColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// glBlendColorEXT
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
pub type glBlendColorEXT_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// glBlendColorxOES
/// * `red` group: ClampedFixed
/// * `green` group: ClampedFixed
/// * `blue` group: ClampedFixed
/// * `alpha` group: ClampedFixed
pub type glBlendColorxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// glBlendEquation
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquation_t = unsafe extern "system" fn(mode: BlendEquationModeEXT);

/// glBlendEquationEXT
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationEXT_t = unsafe extern "system" fn(mode: BlendEquationModeEXT);

/// glBlendEquationIndexedAMD
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// glBlendEquationOES
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationOES_t = unsafe extern "system" fn(mode: BlendEquationModeEXT);

/// glBlendEquationSeparate
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparate_t = unsafe extern "system" fn(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// glBlendEquationSeparateEXT
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateEXT_t = unsafe extern "system" fn(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// glBlendEquationSeparateIndexedAMD
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// glBlendEquationSeparateOES
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateOES_t = unsafe extern "system" fn(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// glBlendEquationSeparatei
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparatei_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// glBlendEquationSeparateiARB
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateiARB_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// glBlendEquationSeparateiEXT
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateiEXT_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// glBlendEquationSeparateiOES
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
pub type glBlendEquationSeparateiOES_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

/// glBlendEquationi
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationi_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// glBlendEquationiARB
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationiARB_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// glBlendEquationiEXT
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationiEXT_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// glBlendEquationiOES
/// * `mode` group: BlendEquationModeEXT
pub type glBlendEquationiOES_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

/// glBlendFunc
/// * `sfactor` group: BlendingFactor
/// * `dfactor` group: BlendingFactor
pub type glBlendFunc_t = unsafe extern "system" fn(sfactor: BlendingFactor, dfactor: BlendingFactor);

/// glBlendFuncIndexedAMD
pub type glBlendFuncIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);

/// glBlendFuncSeparate
/// * `sfactorRGB` group: BlendingFactor
/// * `dfactorRGB` group: BlendingFactor
/// * `sfactorAlpha` group: BlendingFactor
/// * `dfactorAlpha` group: BlendingFactor
pub type glBlendFuncSeparate_t = unsafe extern "system" fn(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);

/// glBlendFuncSeparateEXT
/// * `sfactorRGB` group: BlendingFactor
/// * `dfactorRGB` group: BlendingFactor
/// * `sfactorAlpha` group: BlendingFactor
/// * `dfactorAlpha` group: BlendingFactor
pub type glBlendFuncSeparateEXT_t = unsafe extern "system" fn(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);

/// glBlendFuncSeparateINGR
/// * `sfactorRGB` group: BlendingFactor
/// * `dfactorRGB` group: BlendingFactor
/// * `sfactorAlpha` group: BlendingFactor
/// * `dfactorAlpha` group: BlendingFactor
pub type glBlendFuncSeparateINGR_t = unsafe extern "system" fn(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);

/// glBlendFuncSeparateIndexedAMD
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// glBlendFuncSeparateOES
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateOES_t = unsafe extern "system" fn(srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// glBlendFuncSeparatei
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparatei_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// glBlendFuncSeparateiARB
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateiARB_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// glBlendFuncSeparateiEXT
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateiEXT_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// glBlendFuncSeparateiOES
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
pub type glBlendFuncSeparateiOES_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

/// glBlendFunci
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
pub type glBlendFunci_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

/// glBlendFunciARB
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
pub type glBlendFunciARB_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

/// glBlendFunciEXT
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
pub type glBlendFunciEXT_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

/// glBlendFunciOES
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
pub type glBlendFunciOES_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

/// glBlendParameteriNV
pub type glBlendParameteriNV_t = unsafe extern "system" fn(pname: GLenum, value: GLint);

/// glBlitFramebuffer
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitFramebuffer_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// glBlitFramebufferANGLE
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitFramebufferANGLE_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// glBlitFramebufferEXT
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitFramebufferEXT_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// glBlitFramebufferNV
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitFramebufferNV_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// glBlitNamedFramebuffer
/// * `readFramebuffer` class: framebuffer
/// * `drawFramebuffer` class: framebuffer
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
pub type glBlitNamedFramebuffer_t = unsafe extern "system" fn(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

/// glBufferAddressRangeNV
/// * `length` group: BufferSize
pub type glBufferAddressRangeNV_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, address: GLuint64EXT, length: GLsizeiptr);

/// glBufferAttachMemoryNV
/// * `target` group: BufferTargetARB
pub type glBufferAttachMemoryNV_t = unsafe extern "system" fn(target: BufferTargetARB, memory: GLuint, offset: GLuint64);

/// glBufferData
/// * `target` group: BufferTargetARB
/// * `size` group: BufferSize
/// * `data` len: size
/// * `usage` group: BufferUsageARB
pub type glBufferData_t = unsafe extern "system" fn(target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB);

/// glBufferDataARB
/// * `target` group: BufferTargetARB
/// * `size` group: BufferSizeARB
/// * `data` len: size
/// * `usage` group: BufferUsageARB
pub type glBufferDataARB_t = unsafe extern "system" fn(target: BufferTargetARB, size: GLsizeiptrARB, data: *const void, usage: BufferUsageARB);

/// glBufferPageCommitmentARB
/// * `commit` group: Boolean
pub type glBufferPageCommitmentARB_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

/// glBufferPageCommitmentMemNV
/// * `target` group: BufferStorageTarget
/// * `commit` group: Boolean
pub type glBufferPageCommitmentMemNV_t = unsafe extern "system" fn(target: BufferStorageTarget, offset: GLintptr, size: GLsizeiptr, memory: GLuint, memOffset: GLuint64, commit: GLboolean);

/// glBufferParameteriAPPLE
pub type glBufferParameteriAPPLE_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

/// glBufferStorage
/// * `target` group: BufferStorageTarget
/// * `data` len: size
/// * `flags` group: BufferStorageMask
pub type glBufferStorage_t = unsafe extern "system" fn(target: BufferStorageTarget, size: GLsizeiptr, data: *const void, flags: GLbitfield);

/// glBufferStorageEXT
/// * `target` group: BufferStorageTarget
/// * `data` len: size
/// * `flags` group: BufferStorageMask
pub type glBufferStorageEXT_t = unsafe extern "system" fn(target: BufferStorageTarget, size: GLsizeiptr, data: *const void, flags: GLbitfield);

/// glBufferStorageExternalEXT
/// * `flags` group: BufferStorageMask
pub type glBufferStorageExternalEXT_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, clientBuffer: GLeglClientBufferEXT, flags: GLbitfield);

/// glBufferStorageMemEXT
/// * `target` group: BufferTargetARB
/// * `size` group: BufferSize
pub type glBufferStorageMemEXT_t = unsafe extern "system" fn(target: BufferTargetARB, size: GLsizeiptr, memory: GLuint, offset: GLuint64);

/// glBufferSubData
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `data` len: size
pub type glBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// glBufferSubDataARB
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffsetARB
/// * `size` group: BufferSizeARB
/// * `data` len: size
pub type glBufferSubDataARB_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptrARB, size: GLsizeiptrARB, data: *const void);

/// glCallCommandListNV
pub type glCallCommandListNV_t = unsafe extern "system" fn(list: GLuint);

/// glCallList
/// * `list` group: List
pub type glCallList_t = unsafe extern "system" fn(list: GLuint);

/// glCallLists
/// * `type` group: ListNameType
/// * `lists` len: COMPSIZE(n,type)
pub type glCallLists_t = unsafe extern "system" fn(n: GLsizei, type_: ListNameType, lists: *const void);

/// glCheckFramebufferStatus
/// * `target` group: FramebufferTarget
pub type glCheckFramebufferStatus_t = unsafe extern "system" fn(target: FramebufferTarget) -> FramebufferStatus;

/// glCheckFramebufferStatusEXT
/// * `target` group: FramebufferTarget
pub type glCheckFramebufferStatusEXT_t = unsafe extern "system" fn(target: FramebufferTarget) -> FramebufferStatus;

/// glCheckFramebufferStatusOES
/// * `target` group: FramebufferTarget
pub type glCheckFramebufferStatusOES_t = unsafe extern "system" fn(target: FramebufferTarget) -> FramebufferStatus;

/// glCheckNamedFramebufferStatus
/// * `framebuffer` class: framebuffer
/// * `target` group: FramebufferTarget
pub type glCheckNamedFramebufferStatus_t = unsafe extern "system" fn(framebuffer: GLuint, target: FramebufferTarget) -> FramebufferStatus;

/// glCheckNamedFramebufferStatusEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `target` group: FramebufferTarget
pub type glCheckNamedFramebufferStatusEXT_t = unsafe extern "system" fn(framebuffer: GLuint, target: FramebufferTarget) -> FramebufferStatus;

/// glClampColor
/// * `target` group: ClampColorTargetARB
/// * `clamp` group: ClampColorModeARB
pub type glClampColor_t = unsafe extern "system" fn(target: ClampColorTargetARB, clamp: ClampColorModeARB);

/// glClampColorARB
/// * `target` group: ClampColorTargetARB
/// * `clamp` group: ClampColorModeARB
pub type glClampColorARB_t = unsafe extern "system" fn(target: ClampColorTargetARB, clamp: ClampColorModeARB);

/// glClear
/// * `mask` group: ClearBufferMask
pub type glClear_t = unsafe extern "system" fn(mask: GLbitfield);

/// glClearAccum
pub type glClearAccum_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// glClearAccumxOES
/// * `red` group: ClampedFixed
/// * `green` group: ClampedFixed
/// * `blue` group: ClampedFixed
/// * `alpha` group: ClampedFixed
pub type glClearAccumxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// glClearBufferData
/// * `target` group: BufferStorageTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearBufferData_t = unsafe extern "system" fn(target: BufferStorageTarget, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearBufferSubData
/// * `target` group: BufferTargetARB
/// * `internalformat` group: InternalFormat
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearBufferfi
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
pub type glClearBufferfi_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

/// glClearBufferfv
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
pub type glClearBufferfv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);

/// glClearBufferiv
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
pub type glClearBufferiv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLint);

/// glClearBufferuiv
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
pub type glClearBufferuiv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLuint);

/// glClearColor
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
pub type glClearColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// glClearColorIiEXT
pub type glClearColorIiEXT_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint, alpha: GLint);

/// glClearColorIuiEXT
pub type glClearColorIuiEXT_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);

/// glClearColorx
pub type glClearColorx_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// glClearColorxOES
/// * `red` group: ClampedFixed
/// * `green` group: ClampedFixed
/// * `blue` group: ClampedFixed
/// * `alpha` group: ClampedFixed
pub type glClearColorxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// glClearDepth
pub type glClearDepth_t = unsafe extern "system" fn(depth: GLdouble);

/// glClearDepthdNV
pub type glClearDepthdNV_t = unsafe extern "system" fn(depth: GLdouble);

/// glClearDepthf
pub type glClearDepthf_t = unsafe extern "system" fn(d: GLfloat);

/// glClearDepthfOES
/// * `depth` group: ClampedFloat32
pub type glClearDepthfOES_t = unsafe extern "system" fn(depth: GLclampf);

/// glClearDepthx
pub type glClearDepthx_t = unsafe extern "system" fn(depth: GLfixed);

/// glClearDepthxOES
/// * `depth` group: ClampedFixed
pub type glClearDepthxOES_t = unsafe extern "system" fn(depth: GLfixed);

/// glClearIndex
/// * `c` group: MaskedColorIndexValueF
pub type glClearIndex_t = unsafe extern "system" fn(c: GLfloat);

/// glClearNamedBufferData
/// * `buffer` class: buffer
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glClearNamedBufferData_t = unsafe extern "system" fn(buffer: GLuint, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearNamedBufferDataEXT
/// * `buffer` class: buffer
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearNamedBufferDataEXT_t = unsafe extern "system" fn(buffer: GLuint, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearNamedBufferSubData
/// * `buffer` class: buffer
/// * `internalformat` group: InternalFormat
/// * `size` group: BufferSize
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glClearNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearNamedBufferSubDataEXT
/// * `buffer` class: buffer
/// * `offset` group: BufferSize
/// * `size` group: BufferSize
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, offset: GLsizeiptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearNamedFramebufferfi
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
pub type glClearNamedFramebufferfi_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

/// glClearNamedFramebufferfv
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
pub type glClearNamedFramebufferfv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);

/// glClearNamedFramebufferiv
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
pub type glClearNamedFramebufferiv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLint);

/// glClearNamedFramebufferuiv
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
pub type glClearNamedFramebufferuiv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLuint);

/// glClearPixelLocalStorageuiEXT
/// * `values` len: n
pub type glClearPixelLocalStorageuiEXT_t = unsafe extern "system" fn(offset: GLsizei, n: GLsizei, values: *const GLuint);

/// glClearStencil
/// * `s` group: StencilValue
pub type glClearStencil_t = unsafe extern "system" fn(s: GLint);

/// glClearTexImage
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearTexImageEXT
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearTexImageEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearTexSubImage
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearTexSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

/// glClearTexSubImageEXT
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
pub type glClearTexSubImageEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

/// glClientActiveTexture
/// * `texture` group: TextureUnit
pub type glClientActiveTexture_t = unsafe extern "system" fn(texture: TextureUnit);

/// glClientActiveTextureARB
/// * `texture` group: TextureUnit
pub type glClientActiveTextureARB_t = unsafe extern "system" fn(texture: TextureUnit);

/// glClientActiveVertexStreamATI
/// * `stream` group: VertexStreamATI
pub type glClientActiveVertexStreamATI_t = unsafe extern "system" fn(stream: VertexStreamATI);

/// glClientAttribDefaultEXT
/// * `mask` group: ClientAttribMask
pub type glClientAttribDefaultEXT_t = unsafe extern "system" fn(mask: GLbitfield);

/// glClientWaitSemaphoreui64NVX
/// * `semaphoreArray` len: fenceObjectCount
/// * `fenceValueArray` len: fenceObjectCount
pub type glClientWaitSemaphoreui64NVX_t = unsafe extern "system" fn(fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

/// glClientWaitSync
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncObjectMask
pub type glClientWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus;

/// glClientWaitSyncAPPLE
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncObjectMask
pub type glClientWaitSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus;

/// glClipControl
/// * `origin` group: ClipControlOrigin
/// * `depth` group: ClipControlDepth
pub type glClipControl_t = unsafe extern "system" fn(origin: ClipControlOrigin, depth: ClipControlDepth);

/// glClipControlEXT
pub type glClipControlEXT_t = unsafe extern "system" fn(origin: GLenum, depth: GLenum);

/// glClipPlane
/// * `plane` group: ClipPlaneName
pub type glClipPlane_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: &[GLdouble; 4]);

/// glClipPlanef
/// * `p` group: ClipPlaneName
pub type glClipPlanef_t = unsafe extern "system" fn(p: ClipPlaneName, eqn: *const [GLfloat; 4]);

/// glClipPlanefIMG
/// * `p` group: ClipPlaneName
pub type glClipPlanefIMG_t = unsafe extern "system" fn(p: ClipPlaneName, eqn: &[GLfloat; 4]);

/// glClipPlanefOES
/// * `plane` group: ClipPlaneName
pub type glClipPlanefOES_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *const [GLfloat; 4]);

/// glClipPlanex
/// * `plane` group: ClipPlaneName
pub type glClipPlanex_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *const [GLfixed; 4]);

/// glClipPlanexIMG
/// * `p` group: ClipPlaneName
pub type glClipPlanexIMG_t = unsafe extern "system" fn(p: ClipPlaneName, eqn: *const [GLfixed; 4]);

/// glClipPlanexOES
/// * `plane` group: ClipPlaneName
pub type glClipPlanexOES_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *const [GLfixed; 4]);

/// glColor3b
/// * `red` group: ColorB
/// * `green` group: ColorB
/// * `blue` group: ColorB
pub type glColor3b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

/// glColor3bv
/// * `v` group: ColorB
pub type glColor3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// glColor3d
/// * `red` group: ColorD
/// * `green` group: ColorD
/// * `blue` group: ColorD
pub type glColor3d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

/// glColor3dv
/// * `v` group: ColorD
pub type glColor3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glColor3f
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
pub type glColor3f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

/// glColor3fVertex3fSUN
pub type glColor3fVertex3fSUN_t = unsafe extern "system" fn(r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glColor3fVertex3fvSUN
pub type glColor3fVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glColor3fv
/// * `v` group: ColorF
pub type glColor3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glColor3hNV
/// * `red` group: Half16NV
/// * `green` group: Half16NV
/// * `blue` group: Half16NV
pub type glColor3hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV);

/// glColor3hvNV
/// * `v` group: Half16NV
pub type glColor3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// glColor3i
/// * `red` group: ColorI
/// * `green` group: ColorI
/// * `blue` group: ColorI
pub type glColor3i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

/// glColor3iv
/// * `v` group: ColorI
pub type glColor3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glColor3s
/// * `red` group: ColorS
/// * `green` group: ColorS
/// * `blue` group: ColorS
pub type glColor3s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

/// glColor3sv
/// * `v` group: ColorS
pub type glColor3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glColor3ub
/// * `red` group: ColorUB
/// * `green` group: ColorUB
/// * `blue` group: ColorUB
pub type glColor3ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

/// glColor3ubv
/// * `v` group: ColorUB
pub type glColor3ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

/// glColor3ui
/// * `red` group: ColorUI
/// * `green` group: ColorUI
/// * `blue` group: ColorUI
pub type glColor3ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

/// glColor3uiv
/// * `v` group: ColorUI
pub type glColor3uiv_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

/// glColor3us
/// * `red` group: ColorUS
/// * `green` group: ColorUS
/// * `blue` group: ColorUS
pub type glColor3us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

/// glColor3usv
/// * `v` group: ColorUS
pub type glColor3usv_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

/// glColor3xOES
pub type glColor3xOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed);

/// glColor3xvOES
pub type glColor3xvOES_t = unsafe extern "system" fn(components: *const [GLfixed; 3]);

/// glColor4b
/// * `red` group: ColorB
/// * `green` group: ColorB
/// * `blue` group: ColorB
/// * `alpha` group: ColorB
pub type glColor4b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte);

/// glColor4bv
/// * `v` group: ColorB
pub type glColor4bv_t = unsafe extern "system" fn(v: *const [GLbyte; 4]);

/// glColor4d
/// * `red` group: ColorD
/// * `green` group: ColorD
/// * `blue` group: ColorD
/// * `alpha` group: ColorD
pub type glColor4d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble);

/// glColor4dv
/// * `v` group: ColorD
pub type glColor4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// glColor4f
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
pub type glColor4f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

/// glColor4fNormal3fVertex3fSUN
pub type glColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glColor4fNormal3fVertex3fvSUN
pub type glColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glColor4fv
/// * `v` group: ColorF
pub type glColor4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// glColor4hNV
/// * `red` group: Half16NV
/// * `green` group: Half16NV
/// * `blue` group: Half16NV
/// * `alpha` group: Half16NV
pub type glColor4hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV, alpha: GLhalfNV);

/// glColor4hvNV
/// * `v` group: Half16NV
pub type glColor4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

/// glColor4i
/// * `red` group: ColorI
/// * `green` group: ColorI
/// * `blue` group: ColorI
/// * `alpha` group: ColorI
pub type glColor4i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint, alpha: GLint);

/// glColor4iv
/// * `v` group: ColorI
pub type glColor4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// glColor4s
/// * `red` group: ColorS
/// * `green` group: ColorS
/// * `blue` group: ColorS
/// * `alpha` group: ColorS
pub type glColor4s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort);

/// glColor4sv
/// * `v` group: ColorS
pub type glColor4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// glColor4ub
/// * `red` group: ColorUB
/// * `green` group: ColorUB
/// * `blue` group: ColorUB
/// * `alpha` group: ColorUB
pub type glColor4ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte);

/// glColor4ubVertex2fSUN
pub type glColor4ubVertex2fSUN_t = unsafe extern "system" fn(r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat);

/// glColor4ubVertex2fvSUN
pub type glColor4ubVertex2fvSUN_t = unsafe extern "system" fn(c: *const [GLubyte; 4], v: *const [GLfloat; 2]);

/// glColor4ubVertex3fSUN
pub type glColor4ubVertex3fSUN_t = unsafe extern "system" fn(r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

/// glColor4ubVertex3fvSUN
pub type glColor4ubVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

/// glColor4ubv
/// * `v` group: ColorUB
pub type glColor4ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 4]);

/// glColor4ui
/// * `red` group: ColorUI
/// * `green` group: ColorUI
/// * `blue` group: ColorUI
/// * `alpha` group: ColorUI
pub type glColor4ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);

/// glColor4uiv
/// * `v` group: ColorUI
pub type glColor4uiv_t = unsafe extern "system" fn(v: *const [GLuint; 4]);

/// glColor4us
/// * `red` group: ColorUS
/// * `green` group: ColorUS
/// * `blue` group: ColorUS
/// * `alpha` group: ColorUS
pub type glColor4us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort);

/// glColor4usv
/// * `v` group: ColorUS
pub type glColor4usv_t = unsafe extern "system" fn(v: *const [GLushort; 4]);

/// glColor4x
pub type glColor4x_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// glColor4xOES
pub type glColor4xOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

/// glColor4xvOES
pub type glColor4xvOES_t = unsafe extern "system" fn(components: *const [GLfixed; 4]);

/// glColorFormatNV
pub type glColorFormatNV_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei);

/// glColorFragmentOp1ATI
/// * `op` group: FragmentOpATI
pub type glColorFragmentOp1ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint);

/// glColorFragmentOp2ATI
/// * `op` group: FragmentOpATI
pub type glColorFragmentOp2ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint);

/// glColorFragmentOp3ATI
/// * `op` group: FragmentOpATI
pub type glColorFragmentOp3ATI_t = unsafe extern "system" fn(op: FragmentOpATI, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint, arg3: GLuint, arg3Rep: GLuint, arg3Mod: GLuint);

/// glColorMask
/// * `red` group: Boolean
/// * `green` group: Boolean
/// * `blue` group: Boolean
/// * `alpha` group: Boolean
pub type glColorMask_t = unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

/// glColorMaskIndexedEXT
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
pub type glColorMaskIndexedEXT_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

/// glColorMaski
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
pub type glColorMaski_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

/// glColorMaskiEXT
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
pub type glColorMaskiEXT_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

/// glColorMaskiOES
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
pub type glColorMaskiOES_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

/// glColorMaterial
/// * `face` group: MaterialFace
/// * `mode` group: ColorMaterialParameter
pub type glColorMaterial_t = unsafe extern "system" fn(face: MaterialFace, mode: ColorMaterialParameter);

/// glColorP3ui
/// * `type` group: ColorPointerType
pub type glColorP3ui_t = unsafe extern "system" fn(type_: ColorPointerType, color: GLuint);

/// glColorP3uiv
/// * `type` group: ColorPointerType
pub type glColorP3uiv_t = unsafe extern "system" fn(type_: ColorPointerType, color: *const GLuint);

/// glColorP4ui
/// * `type` group: ColorPointerType
pub type glColorP4ui_t = unsafe extern "system" fn(type_: ColorPointerType, color: GLuint);

/// glColorP4uiv
/// * `type` group: ColorPointerType
pub type glColorP4uiv_t = unsafe extern "system" fn(type_: ColorPointerType, color: *const GLuint);

/// glColorPointer
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glColorPointer_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei, pointer: *const void);

/// glColorPointerEXT
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride,count)
pub type glColorPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// glColorPointerListIBM
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glColorPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// glColorPointervINTEL
/// * `type` group: VertexPointerType
pub type glColorPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, pointer: *const [*mut void; 4]);

/// glColorSubTable
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type,count)
pub type glColorSubTable_t = unsafe extern "system" fn(target: ColorTableTarget, start: GLsizei, count: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

/// glColorSubTableEXT
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type,count)
pub type glColorSubTableEXT_t = unsafe extern "system" fn(target: ColorTableTarget, start: GLsizei, count: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

/// glColorTable
/// * `target` group: ColorTableTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(format,type,width)
pub type glColorTable_t = unsafe extern "system" fn(target: ColorTableTarget, internalformat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, table: *const void);

/// glColorTableEXT
/// * `target` group: ColorTableTarget
/// * `internalFormat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(format,type,width)
pub type glColorTableEXT_t = unsafe extern "system" fn(target: ColorTableTarget, internalFormat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, table: *const void);

/// glColorTableParameterfv
/// * `target` group: ColorTableTarget
/// * `pname` group: ColorTableParameterPNameSGI
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glColorTableParameterfv_t = unsafe extern "system" fn(target: ColorTableTarget, pname: ColorTableParameterPNameSGI, params: *const GLfloat);

/// glColorTableParameterfvSGI
/// * `target` group: ColorTableTargetSGI
/// * `pname` group: ColorTableParameterPNameSGI
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glColorTableParameterfvSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, pname: ColorTableParameterPNameSGI, params: *const GLfloat);

/// glColorTableParameteriv
/// * `target` group: ColorTableTarget
/// * `pname` group: ColorTableParameterPNameSGI
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glColorTableParameteriv_t = unsafe extern "system" fn(target: ColorTableTarget, pname: ColorTableParameterPNameSGI, params: *const GLint);

/// glColorTableParameterivSGI
/// * `target` group: ColorTableTargetSGI
/// * `pname` group: ColorTableParameterPNameSGI
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glColorTableParameterivSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, pname: ColorTableParameterPNameSGI, params: *const GLint);

/// glColorTableSGI
/// * `target` group: ColorTableTargetSGI
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(format,type,width)
pub type glColorTableSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, internalformat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, table: *const void);

/// glCombinerInputNV
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `variable` group: CombinerVariableNV
/// * `input` group: CombinerRegisterNV
/// * `mapping` group: CombinerMappingNV
/// * `componentUsage` group: CombinerComponentUsageNV
pub type glCombinerInputNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, variable: CombinerVariableNV, input: CombinerRegisterNV, mapping: CombinerMappingNV, componentUsage: CombinerComponentUsageNV);

/// glCombinerOutputNV
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

/// glCombinerParameterfNV
/// * `pname` group: CombinerParameterNV
pub type glCombinerParameterfNV_t = unsafe extern "system" fn(pname: CombinerParameterNV, param: GLfloat);

/// glCombinerParameterfvNV
/// * `pname` group: CombinerParameterNV
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glCombinerParameterfvNV_t = unsafe extern "system" fn(pname: CombinerParameterNV, params: *const GLfloat);

/// glCombinerParameteriNV
/// * `pname` group: CombinerParameterNV
pub type glCombinerParameteriNV_t = unsafe extern "system" fn(pname: CombinerParameterNV, param: GLint);

/// glCombinerParameterivNV
/// * `pname` group: CombinerParameterNV
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glCombinerParameterivNV_t = unsafe extern "system" fn(pname: CombinerParameterNV, params: *const GLint);

/// glCombinerStageParameterfvNV
/// * `stage` group: CombinerStageNV
/// * `pname` group: CombinerParameterNV
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glCombinerStageParameterfvNV_t = unsafe extern "system" fn(stage: CombinerStageNV, pname: CombinerParameterNV, params: *const GLfloat);

/// glCommandListSegmentsNV
pub type glCommandListSegmentsNV_t = unsafe extern "system" fn(list: GLuint, segments: GLuint);

/// glCompileCommandListNV
pub type glCompileCommandListNV_t = unsafe extern "system" fn(list: GLuint);

/// glCompileShader
/// * `shader` class: shader
pub type glCompileShader_t = unsafe extern "system" fn(shader: GLuint);

/// glCompileShaderARB
/// * `shaderObj` group: handleARB
pub type glCompileShaderARB_t = unsafe extern "system" fn(shaderObj: GLhandleARB);

/// glCompileShaderIncludeARB
/// * `shader` class: shader
/// * `path` len: count
/// * `length` len: count
pub type glCompileShaderIncludeARB_t = unsafe extern "system" fn(shader: GLuint, count: GLsizei, path: *const *const GLchar, length: *const GLint);

/// glCompressedMultiTexImage1DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// glCompressedMultiTexImage2DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// glCompressedMultiTexImage3DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedMultiTexImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// glCompressedMultiTexSubImage1DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// glCompressedMultiTexSubImage2DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// glCompressedMultiTexSubImage3DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedMultiTexSubImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// glCompressedTexImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// glCompressedTexImage1DARB
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage1DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// glCompressedTexImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// glCompressedTexImage2DARB
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage2DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// glCompressedTexImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// glCompressedTexImage3DARB
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexImage3DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// glCompressedTexImage3DOES
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `data` len: imageSize
pub type glCompressedTexImage3DOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

/// glCompressedTexSubImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTexSubImage1DARB
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage1DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTexSubImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTexSubImage2DARB
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage2DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTexSubImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTexSubImage3DARB
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
pub type glCompressedTexSubImage3DARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTexSubImage3DOES
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `data` len: imageSize
pub type glCompressedTexSubImage3DOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTextureImage1DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedTextureImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// glCompressedTextureImage2DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedTextureImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// glCompressedTextureImage3DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `bits` len: imageSize
pub type glCompressedTextureImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

/// glCompressedTextureSubImage1D
/// * `texture` class: texture
/// * `format` group: PixelFormat
pub type glCompressedTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTextureSubImage1DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// glCompressedTextureSubImage2D
/// * `texture` class: texture
/// * `format` group: PixelFormat
pub type glCompressedTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTextureSubImage2DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `bits` len: imageSize
pub type glCompressedTextureSubImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, bits: *const void);

/// glCompressedTextureSubImage3D
/// * `texture` class: texture
/// * `format` group: PixelFormat
pub type glCompressedTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

/// glCompressedTextureSubImage3DEXT
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

/// glConservativeRasterParameterfNV
pub type glConservativeRasterParameterfNV_t = unsafe extern "system" fn(pname: GLenum, value: GLfloat);

/// glConservativeRasterParameteriNV
pub type glConservativeRasterParameteriNV_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

/// glConvolutionFilter1D
/// * `target` group: ConvolutionTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(format,type,width)
pub type glConvolutionFilter1D_t = unsafe extern "system" fn(target: ConvolutionTarget, internalformat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, image: *const void);

/// glConvolutionFilter1DEXT
/// * `target` group: ConvolutionTargetEXT
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(format,type,width)
pub type glConvolutionFilter1DEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, internalformat: InternalFormat, width: GLsizei, format: PixelFormat, type_: PixelType, image: *const void);

/// glConvolutionFilter2D
/// * `target` group: ConvolutionTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(format,type,width,height)
pub type glConvolutionFilter2D_t = unsafe extern "system" fn(target: ConvolutionTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, image: *const void);

/// glConvolutionFilter2DEXT
/// * `target` group: ConvolutionTargetEXT
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(format,type,width,height)
pub type glConvolutionFilter2DEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, internalformat: InternalFormat, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, image: *const void);

/// glConvolutionParameterf
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedFloat32
pub type glConvolutionParameterf_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: GLfloat);

/// glConvolutionParameterfEXT
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedFloat32
pub type glConvolutionParameterfEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: GLfloat);

/// glConvolutionParameterfv
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameterfv_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: *const GLfloat);

/// glConvolutionParameterfvEXT
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameterfvEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *const GLfloat);

/// glConvolutionParameteri
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedInt32
pub type glConvolutionParameteri_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: GLint);

/// glConvolutionParameteriEXT
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedInt32
pub type glConvolutionParameteriEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: GLint);

/// glConvolutionParameteriv
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameteriv_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: *const GLint);

/// glConvolutionParameterivEXT
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameterivEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *const GLint);

/// glConvolutionParameterxOES
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
pub type glConvolutionParameterxOES_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, param: GLfixed);

/// glConvolutionParameterxvOES
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glConvolutionParameterxvOES_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *const GLfixed);

/// glCopyBufferSubData
/// * `readTarget` group: CopyBufferSubDataTarget
/// * `writeTarget` group: CopyBufferSubDataTarget
/// * `readOffset` group: BufferOffset
/// * `writeOffset` group: BufferOffset
/// * `size` group: BufferSize
pub type glCopyBufferSubData_t = unsafe extern "system" fn(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// glCopyBufferSubDataNV
/// * `readTarget` group: CopyBufferSubDataTarget
/// * `writeTarget` group: CopyBufferSubDataTarget
/// * `readOffset` group: BufferOffset
/// * `writeOffset` group: BufferOffset
/// * `size` group: BufferSize
pub type glCopyBufferSubDataNV_t = unsafe extern "system" fn(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// glCopyColorSubTable
/// * `target` group: ColorTableTarget
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyColorSubTable_t = unsafe extern "system" fn(target: ColorTableTarget, start: GLsizei, x: GLint, y: GLint, width: GLsizei);

/// glCopyColorSubTableEXT
/// * `target` group: ColorTableTarget
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyColorSubTableEXT_t = unsafe extern "system" fn(target: ColorTableTarget, start: GLsizei, x: GLint, y: GLint, width: GLsizei);

/// glCopyColorTable
/// * `target` group: ColorTableTarget
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyColorTable_t = unsafe extern "system" fn(target: ColorTableTarget, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei);

/// glCopyColorTableSGI
/// * `target` group: ColorTableTargetSGI
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyColorTableSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei);

/// glCopyConvolutionFilter1D
/// * `target` group: ConvolutionTarget
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyConvolutionFilter1D_t = unsafe extern "system" fn(target: ConvolutionTarget, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei);

/// glCopyConvolutionFilter1DEXT
/// * `target` group: ConvolutionTargetEXT
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyConvolutionFilter1DEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei);

/// glCopyConvolutionFilter2D
/// * `target` group: ConvolutionTarget
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyConvolutionFilter2D_t = unsafe extern "system" fn(target: ConvolutionTarget, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyConvolutionFilter2DEXT
/// * `target` group: ConvolutionTargetEXT
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyConvolutionFilter2DEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyImageSubData
/// * `srcTarget` group: CopyImageSubDataTarget
/// * `dstTarget` group: CopyImageSubDataTarget
pub type glCopyImageSubData_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyImageSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyImageSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

/// glCopyImageSubDataEXT
/// * `srcTarget` group: CopyBufferSubDataTarget
/// * `dstTarget` group: CopyBufferSubDataTarget
pub type glCopyImageSubDataEXT_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyBufferSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyBufferSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

/// glCopyImageSubDataNV
/// * `srcTarget` group: CopyBufferSubDataTarget
/// * `dstTarget` group: CopyBufferSubDataTarget
pub type glCopyImageSubDataNV_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyBufferSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyBufferSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

/// glCopyImageSubDataOES
/// * `srcTarget` group: CopyBufferSubDataTarget
/// * `dstTarget` group: CopyBufferSubDataTarget
pub type glCopyImageSubDataOES_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyBufferSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyBufferSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

/// glCopyMultiTexImage1DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

/// glCopyMultiTexImage2DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

/// glCopyMultiTexSubImage1DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// glCopyMultiTexSubImage2DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyMultiTexSubImage3DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyMultiTexSubImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyNamedBufferSubData
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
/// * `size` group: BufferSize
pub type glCopyNamedBufferSubData_t = unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// glCopyPathNV
/// * `resultPath` group: Path
/// * `srcPath` group: Path
pub type glCopyPathNV_t = unsafe extern "system" fn(resultPath: GLuint, srcPath: GLuint);

/// glCopyPixels
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `type` group: PixelCopyType
pub type glCopyPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_: PixelCopyType);

/// glCopyTexImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

/// glCopyTexImage1DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTexImage1DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

/// glCopyTexImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

/// glCopyTexImage2DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTexImage2DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

/// glCopyTexSubImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// glCopyTexSubImage1DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage1DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// glCopyTexSubImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyTexSubImage2DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage2DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyTexSubImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyTexSubImage3DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTexSubImage3DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyTexSubImage3DOES
pub type glCopyTexSubImage3DOES_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyTextureImage1DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTextureImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

/// glCopyTextureImage2DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
pub type glCopyTextureImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

/// glCopyTextureLevelsAPPLE
pub type glCopyTextureLevelsAPPLE_t = unsafe extern "system" fn(destinationTexture: GLuint, sourceTexture: GLuint, sourceBaseLevel: GLint, sourceLevelCount: GLsizei);

/// glCopyTextureSubImage1D
/// * `texture` class: texture
pub type glCopyTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// glCopyTextureSubImage1DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

/// glCopyTextureSubImage2D
/// * `texture` class: texture
pub type glCopyTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyTextureSubImage2DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glCopyTextureSubImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyTextureSubImage3D
/// * `texture` class: texture
pub type glCopyTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glCopyTextureSubImage3DEXT
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

/// glCoverFillPathInstancedNV
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `coverMode` group: PathCoverMode
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(numPaths,transformType)
pub type glCoverFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, coverMode: PathCoverMode, transformType: PathTransformType, transformValues: *const GLfloat);

/// glCoverFillPathNV
/// * `path` group: Path
/// * `coverMode` group: PathCoverMode
pub type glCoverFillPathNV_t = unsafe extern "system" fn(path: GLuint, coverMode: PathCoverMode);

/// glCoverStrokePathInstancedNV
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `coverMode` group: PathCoverMode
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(numPaths,transformType)
pub type glCoverStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, coverMode: PathCoverMode, transformType: PathTransformType, transformValues: *const GLfloat);

/// glCoverStrokePathNV
/// * `path` group: Path
/// * `coverMode` group: PathCoverMode
pub type glCoverStrokePathNV_t = unsafe extern "system" fn(path: GLuint, coverMode: PathCoverMode);

/// glCoverageMaskNV
/// * `mask` group: Boolean
pub type glCoverageMaskNV_t = unsafe extern "system" fn(mask: GLboolean);

/// glCoverageModulationNV
pub type glCoverageModulationNV_t = unsafe extern "system" fn(components: GLenum);

/// glCoverageModulationTableNV
/// * `v` len: n
pub type glCoverageModulationTableNV_t = unsafe extern "system" fn(n: GLsizei, v: *const GLfloat);

/// glCoverageOperationNV
pub type glCoverageOperationNV_t = unsafe extern "system" fn(operation: GLenum);

/// glCreateBuffers
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glCreateBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

/// glCreateCommandListsNV
/// * `lists` len: n
pub type glCreateCommandListsNV_t = unsafe extern "system" fn(n: GLsizei, lists: *mut GLuint);

/// glCreateFramebuffers
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glCreateFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

/// glCreateMemoryObjectsEXT
pub type glCreateMemoryObjectsEXT_t = unsafe extern "system" fn(n: GLsizei, memoryObjects: *mut GLuint);

/// glCreatePerfQueryINTEL
pub type glCreatePerfQueryINTEL_t = unsafe extern "system" fn(queryId: GLuint, queryHandle: *mut GLuint);

/// glCreateProgram
pub type glCreateProgram_t = unsafe extern "system" fn() -> GLuint;

/// glCreateProgramObjectARB
pub type glCreateProgramObjectARB_t = unsafe extern "system" fn() -> GLhandleARB;

/// glCreateProgramPipelines
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glCreateProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

/// glCreateProgressFenceNVX
pub type glCreateProgressFenceNVX_t = unsafe extern "system" fn() -> GLuint;

/// glCreateQueries
/// * `target` group: QueryTarget
/// * `ids` class: query
/// * `ids` len: n
pub type glCreateQueries_t = unsafe extern "system" fn(target: QueryTarget, n: GLsizei, ids: *mut GLuint);

/// glCreateRenderbuffers
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glCreateRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

/// glCreateSamplers
/// * `samplers` class: sampler
/// * `samplers` len: n
pub type glCreateSamplers_t = unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint);

/// glCreateSemaphoresNV
/// * `semaphores` len: n
pub type glCreateSemaphoresNV_t = unsafe extern "system" fn(n: GLsizei, semaphores: *mut GLuint);

/// glCreateShader
/// * `type` group: ShaderType
pub type glCreateShader_t = unsafe extern "system" fn(type_: ShaderType) -> GLuint;

/// glCreateShaderObjectARB
/// * `shaderType` group: ShaderType
pub type glCreateShaderObjectARB_t = unsafe extern "system" fn(shaderType: ShaderType) -> GLhandleARB;

/// glCreateShaderProgramEXT
/// * `type` group: ShaderType
pub type glCreateShaderProgramEXT_t = unsafe extern "system" fn(type_: ShaderType, string: *const GLchar) -> GLuint;

/// glCreateShaderProgramv
/// * `type` group: ShaderType
/// * `strings` len: count
pub type glCreateShaderProgramv_t = unsafe extern "system" fn(type_: ShaderType, count: GLsizei, strings: *const *const GLchar) -> GLuint;

/// glCreateShaderProgramvEXT
/// * `type` group: ShaderType
/// * `strings` len: count
pub type glCreateShaderProgramvEXT_t = unsafe extern "system" fn(type_: ShaderType, count: GLsizei, strings: *const *mut GLchar) -> GLuint;

/// glCreateStatesNV
/// * `states` len: n
pub type glCreateStatesNV_t = unsafe extern "system" fn(n: GLsizei, states: *mut GLuint);

/// glCreateSyncFromCLeventARB
/// * `context` group: cl_context
/// * `event` group: cl_event
pub type glCreateSyncFromCLeventARB_t = unsafe extern "system" fn(context: *mut _cl_context, event: *mut _cl_event, flags: GLbitfield) -> GLsync;

/// glCreateTextures
/// * `target` group: TextureTarget
/// * `textures` class: texture
/// * `textures` len: n
pub type glCreateTextures_t = unsafe extern "system" fn(target: TextureTarget, n: GLsizei, textures: *mut GLuint);

/// glCreateTransformFeedbacks
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glCreateTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// glCreateVertexArrays
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glCreateVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

/// glCullFace
/// * `mode` group: CullFaceMode
pub type glCullFace_t = unsafe extern "system" fn(mode: CullFaceMode);

/// glCullParameterdvEXT
/// * `pname` group: CullParameterEXT
pub type glCullParameterdvEXT_t = unsafe extern "system" fn(pname: CullParameterEXT, params: *mut [GLdouble; 4]);

/// glCullParameterfvEXT
/// * `pname` group: CullParameterEXT
pub type glCullParameterfvEXT_t = unsafe extern "system" fn(pname: CullParameterEXT, params: *mut [GLfloat; 4]);

/// glCurrentPaletteMatrixARB
pub type glCurrentPaletteMatrixARB_t = unsafe extern "system" fn(index: GLint);

/// glCurrentPaletteMatrixOES
pub type glCurrentPaletteMatrixOES_t = unsafe extern "system" fn(matrixpaletteindex: GLuint);

/// glDebugMessageCallback
pub type glDebugMessageCallback_t = unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const void);

/// glDebugMessageCallbackAMD
pub type glDebugMessageCallbackAMD_t = unsafe extern "system" fn(callback: GLDEBUGPROCAMD, userParam: *mut void);

/// glDebugMessageCallbackARB
/// * `userParam` len: COMPSIZE(callback)
pub type glDebugMessageCallbackARB_t = unsafe extern "system" fn(callback: GLDEBUGPROCARB, userParam: *const void);

/// glDebugMessageCallbackKHR
pub type glDebugMessageCallbackKHR_t = unsafe extern "system" fn(callback: GLDEBUGPROCKHR, userParam: *const void);

/// glDebugMessageControl
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
pub type glDebugMessageControl_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

/// glDebugMessageControlARB
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
pub type glDebugMessageControlARB_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

/// glDebugMessageControlKHR
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `enabled` group: Boolean
pub type glDebugMessageControlKHR_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

/// glDebugMessageEnableAMD
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
pub type glDebugMessageEnableAMD_t = unsafe extern "system" fn(category: GLenum, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

/// glDebugMessageInsert
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `buf` len: COMPSIZE(buf,length)
pub type glDebugMessageInsert_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);

/// glDebugMessageInsertAMD
/// * `severity` group: DebugSeverity
/// * `buf` len: length
pub type glDebugMessageInsertAMD_t = unsafe extern "system" fn(category: GLenum, severity: DebugSeverity, id: GLuint, length: GLsizei, buf: *const GLchar);

/// glDebugMessageInsertARB
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `buf` len: length
pub type glDebugMessageInsertARB_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);

/// glDebugMessageInsertKHR
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
pub type glDebugMessageInsertKHR_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);

/// glDeformSGIX
/// * `mask` group: FfdMaskSGIX
pub type glDeformSGIX_t = unsafe extern "system" fn(mask: GLbitfield);

/// glDeformationMap3dSGIX
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

/// glDeformationMap3fSGIX
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

/// glDeleteAsyncMarkersSGIX
pub type glDeleteAsyncMarkersSGIX_t = unsafe extern "system" fn(marker: GLuint, range: GLsizei);

/// glDeleteBuffers
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glDeleteBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);

/// glDeleteBuffersARB
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glDeleteBuffersARB_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);

/// glDeleteCommandListsNV
/// * `lists` len: n
pub type glDeleteCommandListsNV_t = unsafe extern "system" fn(n: GLsizei, lists: *const GLuint);

/// glDeleteFencesAPPLE
/// * `fences` group: FenceNV
/// * `fences` len: n
pub type glDeleteFencesAPPLE_t = unsafe extern "system" fn(n: GLsizei, fences: *const GLuint);

/// glDeleteFencesNV
/// * `fences` group: FenceNV
/// * `fences` len: n
pub type glDeleteFencesNV_t = unsafe extern "system" fn(n: GLsizei, fences: *const GLuint);

/// glDeleteFragmentShaderATI
pub type glDeleteFragmentShaderATI_t = unsafe extern "system" fn(id: GLuint);

/// glDeleteFramebuffers
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glDeleteFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

/// glDeleteFramebuffersEXT
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glDeleteFramebuffersEXT_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

/// glDeleteFramebuffersOES
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glDeleteFramebuffersOES_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

/// glDeleteLists
/// * `list` group: List
/// * `list` class: display list
pub type glDeleteLists_t = unsafe extern "system" fn(list: GLuint, range: GLsizei);

/// glDeleteMemoryObjectsEXT
/// * `memoryObjects` len: n
pub type glDeleteMemoryObjectsEXT_t = unsafe extern "system" fn(n: GLsizei, memoryObjects: *const GLuint);

/// glDeleteNamedStringARB
/// * `name` len: namelen
pub type glDeleteNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar);

/// glDeleteNamesAMD
/// * `names` len: num
pub type glDeleteNamesAMD_t = unsafe extern "system" fn(identifier: GLenum, num: GLuint, names: *const GLuint);

/// glDeleteObjectARB
/// * `obj` group: handleARB
pub type glDeleteObjectARB_t = unsafe extern "system" fn(obj: GLhandleARB);

/// glDeleteOcclusionQueriesNV
/// * `ids` len: n
pub type glDeleteOcclusionQueriesNV_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// glDeletePathsNV
/// * `path` group: Path
pub type glDeletePathsNV_t = unsafe extern "system" fn(path: GLuint, range: GLsizei);

/// glDeletePerfMonitorsAMD
/// * `monitors` len: n
pub type glDeletePerfMonitorsAMD_t = unsafe extern "system" fn(n: GLsizei, monitors: *mut GLuint);

/// glDeletePerfQueryINTEL
pub type glDeletePerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

/// glDeleteProgram
/// * `program` class: program
pub type glDeleteProgram_t = unsafe extern "system" fn(program: GLuint);

/// glDeleteProgramPipelines
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glDeleteProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);

/// glDeleteProgramPipelinesEXT
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glDeleteProgramPipelinesEXT_t = unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);

/// glDeleteProgramsARB
/// * `programs` class: program
/// * `programs` len: n
pub type glDeleteProgramsARB_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

/// glDeleteProgramsNV
/// * `programs` class: program
/// * `programs` len: n
pub type glDeleteProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

/// glDeleteQueries
/// * `ids` class: query
/// * `ids` len: n
pub type glDeleteQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// glDeleteQueriesARB
/// * `ids` class: query
/// * `ids` len: n
pub type glDeleteQueriesARB_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// glDeleteQueriesEXT
/// * `ids` class: query
/// * `ids` len: n
pub type glDeleteQueriesEXT_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// glDeleteQueryResourceTagNV
/// * `tagIds` len: n
pub type glDeleteQueryResourceTagNV_t = unsafe extern "system" fn(n: GLsizei, tagIds: *const GLint);

/// glDeleteRenderbuffers
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glDeleteRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

/// glDeleteRenderbuffersEXT
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glDeleteRenderbuffersEXT_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

/// glDeleteRenderbuffersOES
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glDeleteRenderbuffersOES_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

/// glDeleteSamplers
/// * `samplers` class: sampler
/// * `samplers` len: count
pub type glDeleteSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);

/// glDeleteSemaphoresEXT
/// * `semaphores` len: n
pub type glDeleteSemaphoresEXT_t = unsafe extern "system" fn(n: GLsizei, semaphores: *const GLuint);

/// glDeleteShader
/// * `shader` class: shader
pub type glDeleteShader_t = unsafe extern "system" fn(shader: GLuint);

/// glDeleteStatesNV
/// * `states` len: n
pub type glDeleteStatesNV_t = unsafe extern "system" fn(n: GLsizei, states: *const GLuint);

/// glDeleteSync
/// * `sync` group: sync
/// * `sync` class: sync
pub type glDeleteSync_t = unsafe extern "system" fn(sync: GLsync);

/// glDeleteSyncAPPLE
/// * `sync` class: sync
pub type glDeleteSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync);

/// glDeleteTextures
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
pub type glDeleteTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);

/// glDeleteTexturesEXT
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
pub type glDeleteTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);

/// glDeleteTransformFeedbacks
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glDeleteTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// glDeleteTransformFeedbacksNV
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glDeleteTransformFeedbacksNV_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

/// glDeleteVertexArrays
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glDeleteVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

/// glDeleteVertexArraysAPPLE
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glDeleteVertexArraysAPPLE_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

/// glDeleteVertexArraysOES
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glDeleteVertexArraysOES_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

/// glDeleteVertexShaderEXT
pub type glDeleteVertexShaderEXT_t = unsafe extern "system" fn(id: GLuint);

/// glDepthBoundsEXT
/// * `zmin` group: ClampedFloat64
/// * `zmax` group: ClampedFloat64
pub type glDepthBoundsEXT_t = unsafe extern "system" fn(zmin: GLclampd, zmax: GLclampd);

/// glDepthBoundsdNV
pub type glDepthBoundsdNV_t = unsafe extern "system" fn(zmin: GLdouble, zmax: GLdouble);

/// glDepthFunc
/// * `func` group: DepthFunction
pub type glDepthFunc_t = unsafe extern "system" fn(func: DepthFunction);

/// glDepthMask
/// * `flag` group: Boolean
pub type glDepthMask_t = unsafe extern "system" fn(flag: GLboolean);

/// glDepthRange
pub type glDepthRange_t = unsafe extern "system" fn(n: GLdouble, f: GLdouble);

/// glDepthRangeArraydvNV
pub type glDepthRangeArraydvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);

/// glDepthRangeArrayfvNV
pub type glDepthRangeArrayfvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// glDepthRangeArrayfvOES
pub type glDepthRangeArrayfvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// glDepthRangeArrayv
/// * `v` len: COMPSIZE(count)
pub type glDepthRangeArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);

/// glDepthRangeIndexed
pub type glDepthRangeIndexed_t = unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);

/// glDepthRangeIndexeddNV
pub type glDepthRangeIndexeddNV_t = unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);

/// glDepthRangeIndexedfNV
pub type glDepthRangeIndexedfNV_t = unsafe extern "system" fn(index: GLuint, n: GLfloat, f: GLfloat);

/// glDepthRangeIndexedfOES
pub type glDepthRangeIndexedfOES_t = unsafe extern "system" fn(index: GLuint, n: GLfloat, f: GLfloat);

/// glDepthRangedNV
pub type glDepthRangedNV_t = unsafe extern "system" fn(zNear: GLdouble, zFar: GLdouble);

/// glDepthRangef
pub type glDepthRangef_t = unsafe extern "system" fn(n: GLfloat, f: GLfloat);

/// glDepthRangefOES
/// * `n` group: ClampedFloat32
/// * `f` group: ClampedFloat32
pub type glDepthRangefOES_t = unsafe extern "system" fn(n: GLclampf, f: GLclampf);

/// glDepthRangex
pub type glDepthRangex_t = unsafe extern "system" fn(n: GLfixed, f: GLfixed);

/// glDepthRangexOES
/// * `n` group: ClampedFixed
/// * `f` group: ClampedFixed
pub type glDepthRangexOES_t = unsafe extern "system" fn(n: GLfixed, f: GLfixed);

/// glDetachObjectARB
/// * `containerObj` group: handleARB
/// * `attachedObj` group: handleARB
pub type glDetachObjectARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, attachedObj: GLhandleARB);

/// glDetachShader
/// * `program` class: program
/// * `shader` class: shader
pub type glDetachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);

/// glDetailTexFuncSGIS
/// * `target` group: TextureTarget
/// * `points` len: n*2
pub type glDetailTexFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, n: GLsizei, points: *const GLfloat);

/// glDisable
/// * `cap` group: EnableCap
pub type glDisable_t = unsafe extern "system" fn(cap: EnableCap);

/// glDisableClientState
/// * `array` group: EnableCap
pub type glDisableClientState_t = unsafe extern "system" fn(array: EnableCap);

/// glDisableClientStateIndexedEXT
/// * `array` group: EnableCap
pub type glDisableClientStateIndexedEXT_t = unsafe extern "system" fn(array: EnableCap, index: GLuint);

/// glDisableClientStateiEXT
/// * `array` group: EnableCap
pub type glDisableClientStateiEXT_t = unsafe extern "system" fn(array: EnableCap, index: GLuint);

/// glDisableDriverControlQCOM
pub type glDisableDriverControlQCOM_t = unsafe extern "system" fn(driverControl: GLuint);

/// glDisableIndexedEXT
/// * `target` group: EnableCap
pub type glDisableIndexedEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glDisableVariantClientStateEXT
pub type glDisableVariantClientStateEXT_t = unsafe extern "system" fn(id: GLuint);

/// glDisableVertexArrayAttrib
/// * `vaobj` class: vertex array
pub type glDisableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

/// glDisableVertexArrayAttribEXT
/// * `vaobj` class: vertex array
pub type glDisableVertexArrayAttribEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

/// glDisableVertexArrayEXT
/// * `vaobj` class: vertex array
/// * `array` group: EnableCap
pub type glDisableVertexArrayEXT_t = unsafe extern "system" fn(vaobj: GLuint, array: EnableCap);

/// glDisableVertexAttribAPPLE
pub type glDisableVertexAttribAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum);

/// glDisableVertexAttribArray
pub type glDisableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

/// glDisableVertexAttribArrayARB
pub type glDisableVertexAttribArrayARB_t = unsafe extern "system" fn(index: GLuint);

/// glDisablei
/// * `target` group: EnableCap
pub type glDisablei_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glDisableiEXT
/// * `target` group: EnableCap
pub type glDisableiEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glDisableiNV
/// * `target` group: EnableCap
pub type glDisableiNV_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glDisableiOES
/// * `target` group: EnableCap
pub type glDisableiOES_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glDiscardFramebufferEXT
/// * `target` group: FramebufferTarget
/// * `attachments` group: InvalidateFramebufferAttachment
/// * `attachments` len: numAttachments
pub type glDiscardFramebufferEXT_t = unsafe extern "system" fn(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment);

/// glDispatchCompute
pub type glDispatchCompute_t = unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);

/// glDispatchComputeGroupSizeARB
pub type glDispatchComputeGroupSizeARB_t = unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint, group_size_x: GLuint, group_size_y: GLuint, group_size_z: GLuint);

/// glDispatchComputeIndirect
/// * `indirect` group: BufferOffset
pub type glDispatchComputeIndirect_t = unsafe extern "system" fn(indirect: GLintptr);

/// glDrawArrays
/// * `mode` group: PrimitiveType
pub type glDrawArrays_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei);

/// glDrawArraysEXT
/// * `mode` group: PrimitiveType
pub type glDrawArraysEXT_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei);

/// glDrawArraysIndirect
/// * `mode` group: PrimitiveType
pub type glDrawArraysIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void);

/// glDrawArraysInstanced
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei);

/// glDrawArraysInstancedANGLE
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedANGLE_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, primcount: GLsizei);

/// glDrawArraysInstancedARB
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedARB_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, primcount: GLsizei);

/// glDrawArraysInstancedBaseInstance
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);

/// glDrawArraysInstancedBaseInstanceEXT
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedBaseInstanceEXT_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);

/// glDrawArraysInstancedEXT
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedEXT_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLint, count: GLsizei, primcount: GLsizei);

/// glDrawArraysInstancedNV
/// * `mode` group: PrimitiveType
pub type glDrawArraysInstancedNV_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, primcount: GLsizei);

/// glDrawBuffer
/// * `buf` group: DrawBufferMode
pub type glDrawBuffer_t = unsafe extern "system" fn(buf: DrawBufferMode);

/// glDrawBuffers
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
pub type glDrawBuffers_t = unsafe extern "system" fn(n: GLsizei, bufs: *const DrawBufferMode);

/// glDrawBuffersARB
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
pub type glDrawBuffersARB_t = unsafe extern "system" fn(n: GLsizei, bufs: *const DrawBufferMode);

/// glDrawBuffersATI
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
pub type glDrawBuffersATI_t = unsafe extern "system" fn(n: GLsizei, bufs: *const DrawBufferMode);

/// glDrawBuffersEXT
/// * `bufs` len: n
pub type glDrawBuffersEXT_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

/// glDrawBuffersIndexedEXT
/// * `location` len: n
/// * `indices` len: n
pub type glDrawBuffersIndexedEXT_t = unsafe extern "system" fn(n: GLint, location: *const GLenum, indices: *const GLint);

/// glDrawBuffersNV
/// * `bufs` len: n
pub type glDrawBuffersNV_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

/// glDrawCommandsAddressNV
pub type glDrawCommandsAddressNV_t = unsafe extern "system" fn(primitiveMode: GLenum, indirects: *const GLuint64, sizes: *const GLsizei, count: GLuint);

/// glDrawCommandsNV
pub type glDrawCommandsNV_t = unsafe extern "system" fn(primitiveMode: GLenum, buffer: GLuint, indirects: *const GLintptr, sizes: *const GLsizei, count: GLuint);

/// glDrawCommandsStatesAddressNV
pub type glDrawCommandsStatesAddressNV_t = unsafe extern "system" fn(indirects: *const GLuint64, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

/// glDrawCommandsStatesNV
/// * `buffer` class: buffer
pub type glDrawCommandsStatesNV_t = unsafe extern "system" fn(buffer: GLuint, indirects: *const GLintptr, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

/// glDrawElementArrayAPPLE
/// * `mode` group: PrimitiveType
pub type glDrawElementArrayAPPLE_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei);

/// glDrawElementArrayATI
/// * `mode` group: PrimitiveType
pub type glDrawElementArrayATI_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei);

/// glDrawElements
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElements_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void);

/// glDrawElementsBaseVertex
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// glDrawElementsBaseVertexEXT
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// glDrawElementsBaseVertexOES
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsBaseVertexOES_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// glDrawElementsIndirect
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glDrawElementsIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void);

/// glDrawElementsInstanced
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei);

/// glDrawElementsInstancedANGLE
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedANGLE_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, primcount: GLsizei);

/// glDrawElementsInstancedARB
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedARB_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, primcount: GLsizei);

/// glDrawElementsInstancedBaseInstance
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: count
pub type glDrawElementsInstancedBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, instancecount: GLsizei, baseinstance: GLuint);

/// glDrawElementsInstancedBaseInstanceEXT
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: count
pub type glDrawElementsInstancedBaseInstanceEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, instancecount: GLsizei, baseinstance: GLuint);

/// glDrawElementsInstancedBaseVertex
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);

/// glDrawElementsInstancedBaseVertexBaseInstance
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: count
pub type glDrawElementsInstancedBaseVertexBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);

/// glDrawElementsInstancedBaseVertexBaseInstanceEXT
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: count
pub type glDrawElementsInstancedBaseVertexBaseInstanceEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);

/// glDrawElementsInstancedBaseVertexEXT
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedBaseVertexEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);

/// glDrawElementsInstancedBaseVertexOES
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedBaseVertexOES_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);

/// glDrawElementsInstancedEXT
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, primcount: GLsizei);

/// glDrawElementsInstancedNV
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawElementsInstancedNV_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, primcount: GLsizei);

/// glDrawMeshArraysSUN
/// * `mode` group: PrimitiveType
pub type glDrawMeshArraysSUN_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, width: GLsizei);

/// glDrawMeshTasksIndirectNV
pub type glDrawMeshTasksIndirectNV_t = unsafe extern "system" fn(indirect: GLintptr);

/// glDrawMeshTasksNV
pub type glDrawMeshTasksNV_t = unsafe extern "system" fn(first: GLuint, count: GLuint);

/// glDrawPixels
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glDrawPixels_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glDrawRangeElementArrayAPPLE
/// * `mode` group: PrimitiveType
pub type glDrawRangeElementArrayAPPLE_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, first: GLint, count: GLsizei);

/// glDrawRangeElementArrayATI
/// * `mode` group: PrimitiveType
pub type glDrawRangeElementArrayATI_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei);

/// glDrawRangeElements
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElements_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void);

/// glDrawRangeElementsBaseVertex
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// glDrawRangeElementsBaseVertexEXT
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// glDrawRangeElementsBaseVertexOES
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElementsBaseVertexOES_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

/// glDrawRangeElementsEXT
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
pub type glDrawRangeElementsEXT_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void);

/// glDrawTexfOES
pub type glDrawTexfOES_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, width: GLfloat, height: GLfloat);

/// glDrawTexfvOES
pub type glDrawTexfvOES_t = unsafe extern "system" fn(coords: *const [GLfloat; 5]);

/// glDrawTexiOES
pub type glDrawTexiOES_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, width: GLint, height: GLint);

/// glDrawTexivOES
pub type glDrawTexivOES_t = unsafe extern "system" fn(coords: *const [GLint; 5]);

/// glDrawTexsOES
pub type glDrawTexsOES_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, width: GLshort, height: GLshort);

/// glDrawTexsvOES
pub type glDrawTexsvOES_t = unsafe extern "system" fn(coords: *const [GLshort; 5]);

/// glDrawTextureNV
/// * `texture` class: texture
/// * `sampler` class: sampler
pub type glDrawTextureNV_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint, x0: GLfloat, y0: GLfloat, x1: GLfloat, y1: GLfloat, z: GLfloat, s0: GLfloat, t0: GLfloat, s1: GLfloat, t1: GLfloat);

/// glDrawTexxOES
pub type glDrawTexxOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed, width: GLfixed, height: GLfixed);

/// glDrawTexxvOES
pub type glDrawTexxvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 5]);

/// glDrawTransformFeedback
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedback_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint);

/// glDrawTransformFeedbackEXT
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackEXT_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint);

/// glDrawTransformFeedbackInstanced
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, instancecount: GLsizei);

/// glDrawTransformFeedbackInstancedEXT
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackInstancedEXT_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, instancecount: GLsizei);

/// glDrawTransformFeedbackNV
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackNV_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint);

/// glDrawTransformFeedbackStream
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackStream_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, stream: GLuint);

/// glDrawTransformFeedbackStreamInstanced
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
pub type glDrawTransformFeedbackStreamInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, stream: GLuint, instancecount: GLsizei);

/// glDrawVkImageNV
/// * `sampler` class: sampler
pub type glDrawVkImageNV_t = unsafe extern "system" fn(vkImage: GLuint64, sampler: GLuint, x0: GLfloat, y0: GLfloat, x1: GLfloat, y1: GLfloat, z: GLfloat, s0: GLfloat, t0: GLfloat, s1: GLfloat, t1: GLfloat);

/// glEGLImageTargetRenderbufferStorageOES
pub type glEGLImageTargetRenderbufferStorageOES_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES);

/// glEGLImageTargetTexStorageEXT
pub type glEGLImageTargetTexStorageEXT_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES, attrib_list: *const GLint);

/// glEGLImageTargetTexture2DOES
pub type glEGLImageTargetTexture2DOES_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES);

/// glEGLImageTargetTextureStorageEXT
/// * `texture` class: texture
pub type glEGLImageTargetTextureStorageEXT_t = unsafe extern "system" fn(texture: GLuint, image: GLeglImageOES, attrib_list: *const GLint);

/// glEdgeFlag
/// * `flag` group: Boolean
pub type glEdgeFlag_t = unsafe extern "system" fn(flag: GLboolean);

/// glEdgeFlagFormatNV
pub type glEdgeFlagFormatNV_t = unsafe extern "system" fn(stride: GLsizei);

/// glEdgeFlagPointer
/// * `pointer` len: COMPSIZE(stride)
pub type glEdgeFlagPointer_t = unsafe extern "system" fn(stride: GLsizei, pointer: *const void);

/// glEdgeFlagPointerEXT
/// * `pointer` group: Boolean
/// * `pointer` len: COMPSIZE(stride,count)
pub type glEdgeFlagPointerEXT_t = unsafe extern "system" fn(stride: GLsizei, count: GLsizei, pointer: *const GLboolean);

/// glEdgeFlagPointerListIBM
/// * `pointer` group: BooleanPointer
/// * `pointer` len: COMPSIZE(stride)
pub type glEdgeFlagPointerListIBM_t = unsafe extern "system" fn(stride: GLint, pointer: *const *mut GLboolean, ptrstride: GLint);

/// glEdgeFlagv
/// * `flag` group: Boolean
pub type glEdgeFlagv_t = unsafe extern "system" fn(flag: *const GLboolean);

/// glElementPointerAPPLE
/// * `type` group: ElementPointerTypeATI
/// * `pointer` len: COMPSIZE(type)
pub type glElementPointerAPPLE_t = unsafe extern "system" fn(type_: ElementPointerTypeATI, pointer: *const void);

/// glElementPointerATI
/// * `type` group: ElementPointerTypeATI
/// * `pointer` len: COMPSIZE(type)
pub type glElementPointerATI_t = unsafe extern "system" fn(type_: ElementPointerTypeATI, pointer: *const void);

/// glEnable
/// * `cap` group: EnableCap
pub type glEnable_t = unsafe extern "system" fn(cap: EnableCap);

/// glEnableClientState
/// * `array` group: EnableCap
pub type glEnableClientState_t = unsafe extern "system" fn(array: EnableCap);

/// glEnableClientStateIndexedEXT
/// * `array` group: EnableCap
pub type glEnableClientStateIndexedEXT_t = unsafe extern "system" fn(array: EnableCap, index: GLuint);

/// glEnableClientStateiEXT
/// * `array` group: EnableCap
pub type glEnableClientStateiEXT_t = unsafe extern "system" fn(array: EnableCap, index: GLuint);

/// glEnableDriverControlQCOM
pub type glEnableDriverControlQCOM_t = unsafe extern "system" fn(driverControl: GLuint);

/// glEnableIndexedEXT
/// * `target` group: EnableCap
pub type glEnableIndexedEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glEnableVariantClientStateEXT
pub type glEnableVariantClientStateEXT_t = unsafe extern "system" fn(id: GLuint);

/// glEnableVertexArrayAttrib
/// * `vaobj` class: vertex array
pub type glEnableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

/// glEnableVertexArrayAttribEXT
/// * `vaobj` class: vertex array
pub type glEnableVertexArrayAttribEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

/// glEnableVertexArrayEXT
/// * `vaobj` class: vertex array
/// * `array` group: EnableCap
pub type glEnableVertexArrayEXT_t = unsafe extern "system" fn(vaobj: GLuint, array: EnableCap);

/// glEnableVertexAttribAPPLE
pub type glEnableVertexAttribAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum);

/// glEnableVertexAttribArray
pub type glEnableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

/// glEnableVertexAttribArrayARB
pub type glEnableVertexAttribArrayARB_t = unsafe extern "system" fn(index: GLuint);

/// glEnablei
/// * `target` group: EnableCap
pub type glEnablei_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glEnableiEXT
/// * `target` group: EnableCap
pub type glEnableiEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glEnableiNV
/// * `target` group: EnableCap
pub type glEnableiNV_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glEnableiOES
/// * `target` group: EnableCap
pub type glEnableiOES_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

/// glEnd
pub type glEnd_t = unsafe extern "system" fn();

/// glEndConditionalRender
pub type glEndConditionalRender_t = unsafe extern "system" fn();

/// glEndConditionalRenderNV
pub type glEndConditionalRenderNV_t = unsafe extern "system" fn();

/// glEndConditionalRenderNVX
pub type glEndConditionalRenderNVX_t = unsafe extern "system" fn();

/// glEndFragmentShaderATI
pub type glEndFragmentShaderATI_t = unsafe extern "system" fn();

/// glEndList
pub type glEndList_t = unsafe extern "system" fn();

/// glEndOcclusionQueryNV
pub type glEndOcclusionQueryNV_t = unsafe extern "system" fn();

/// glEndPerfMonitorAMD
pub type glEndPerfMonitorAMD_t = unsafe extern "system" fn(monitor: GLuint);

/// glEndPerfQueryINTEL
pub type glEndPerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

/// glEndQuery
/// * `target` group: QueryTarget
pub type glEndQuery_t = unsafe extern "system" fn(target: QueryTarget);

/// glEndQueryARB
/// * `target` group: QueryTarget
pub type glEndQueryARB_t = unsafe extern "system" fn(target: QueryTarget);

/// glEndQueryEXT
/// * `target` group: QueryTarget
pub type glEndQueryEXT_t = unsafe extern "system" fn(target: QueryTarget);

/// glEndQueryIndexed
/// * `target` group: QueryTarget
pub type glEndQueryIndexed_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint);

/// glEndTilingQCOM
/// * `preserveMask` group: BufferBitQCOM
pub type glEndTilingQCOM_t = unsafe extern "system" fn(preserveMask: GLbitfield);

/// glEndTransformFeedback
pub type glEndTransformFeedback_t = unsafe extern "system" fn();

/// glEndTransformFeedbackEXT
pub type glEndTransformFeedbackEXT_t = unsafe extern "system" fn();

/// glEndTransformFeedbackNV
pub type glEndTransformFeedbackNV_t = unsafe extern "system" fn();

/// glEndVertexShaderEXT
pub type glEndVertexShaderEXT_t = unsafe extern "system" fn();

/// glEndVideoCaptureNV
pub type glEndVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint);

/// glEvalCoord1d
/// * `u` group: CoordD
pub type glEvalCoord1d_t = unsafe extern "system" fn(u: GLdouble);

/// glEvalCoord1dv
/// * `u` group: CoordD
pub type glEvalCoord1dv_t = unsafe extern "system" fn(u: *const GLdouble);

/// glEvalCoord1f
/// * `u` group: CoordF
pub type glEvalCoord1f_t = unsafe extern "system" fn(u: GLfloat);

/// glEvalCoord1fv
/// * `u` group: CoordF
pub type glEvalCoord1fv_t = unsafe extern "system" fn(u: *const GLfloat);

/// glEvalCoord1xOES
pub type glEvalCoord1xOES_t = unsafe extern "system" fn(u: GLfixed);

/// glEvalCoord1xvOES
pub type glEvalCoord1xvOES_t = unsafe extern "system" fn(coords: *const GLfixed);

/// glEvalCoord2d
/// * `u` group: CoordD
/// * `v` group: CoordD
pub type glEvalCoord2d_t = unsafe extern "system" fn(u: GLdouble, v: GLdouble);

/// glEvalCoord2dv
/// * `u` group: CoordD
pub type glEvalCoord2dv_t = unsafe extern "system" fn(u: *const [GLdouble; 2]);

/// glEvalCoord2f
/// * `u` group: CoordF
/// * `v` group: CoordF
pub type glEvalCoord2f_t = unsafe extern "system" fn(u: GLfloat, v: GLfloat);

/// glEvalCoord2fv
/// * `u` group: CoordF
pub type glEvalCoord2fv_t = unsafe extern "system" fn(u: *const [GLfloat; 2]);

/// glEvalCoord2xOES
pub type glEvalCoord2xOES_t = unsafe extern "system" fn(u: GLfixed, v: GLfixed);

/// glEvalCoord2xvOES
pub type glEvalCoord2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

/// glEvalMapsNV
/// * `target` group: EvalTargetNV
/// * `mode` group: EvalMapsModeNV
pub type glEvalMapsNV_t = unsafe extern "system" fn(target: EvalTargetNV, mode: EvalMapsModeNV);

/// glEvalMesh1
/// * `mode` group: MeshMode1
/// * `i1` group: CheckedInt32
/// * `i2` group: CheckedInt32
pub type glEvalMesh1_t = unsafe extern "system" fn(mode: MeshMode1, i1: GLint, i2: GLint);

/// glEvalMesh2
/// * `mode` group: MeshMode2
/// * `i1` group: CheckedInt32
/// * `i2` group: CheckedInt32
/// * `j1` group: CheckedInt32
/// * `j2` group: CheckedInt32
pub type glEvalMesh2_t = unsafe extern "system" fn(mode: MeshMode2, i1: GLint, i2: GLint, j1: GLint, j2: GLint);

/// glEvalPoint1
pub type glEvalPoint1_t = unsafe extern "system" fn(i: GLint);

/// glEvalPoint2
/// * `i` group: CheckedInt32
/// * `j` group: CheckedInt32
pub type glEvalPoint2_t = unsafe extern "system" fn(i: GLint, j: GLint);

/// glEvaluateDepthValuesARB
pub type glEvaluateDepthValuesARB_t = unsafe extern "system" fn();

/// glExecuteProgramNV
/// * `target` group: VertexAttribEnumNV
pub type glExecuteProgramNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, id: GLuint, params: *const [GLfloat; 4]);

/// glExtGetBufferPointervQCOM
pub type glExtGetBufferPointervQCOM_t = unsafe extern "system" fn(target: GLenum, params: *mut *mut void);

/// glExtGetBuffersQCOM
/// * `buffers` class: buffer
/// * `buffers` len: maxBuffers
pub type glExtGetBuffersQCOM_t = unsafe extern "system" fn(buffers: *mut GLuint, maxBuffers: GLint, numBuffers: *mut GLint);

/// glExtGetFramebuffersQCOM
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: maxFramebuffers
pub type glExtGetFramebuffersQCOM_t = unsafe extern "system" fn(framebuffers: *mut GLuint, maxFramebuffers: GLint, numFramebuffers: *mut GLint);

/// glExtGetProgramBinarySourceQCOM
/// * `program` class: program
/// * `shadertype` group: ShaderType
pub type glExtGetProgramBinarySourceQCOM_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, source: *mut GLchar, length: *mut GLint);

/// glExtGetProgramsQCOM
/// * `programs` class: program
/// * `programs` len: maxPrograms
pub type glExtGetProgramsQCOM_t = unsafe extern "system" fn(programs: *mut GLuint, maxPrograms: GLint, numPrograms: *mut GLint);

/// glExtGetRenderbuffersQCOM
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: maxRenderbuffers
pub type glExtGetRenderbuffersQCOM_t = unsafe extern "system" fn(renderbuffers: *mut GLuint, maxRenderbuffers: GLint, numRenderbuffers: *mut GLint);

/// glExtGetShadersQCOM
/// * `shaders` class: shader
/// * `shaders` len: maxShaders
pub type glExtGetShadersQCOM_t = unsafe extern "system" fn(shaders: *mut GLuint, maxShaders: GLint, numShaders: *mut GLint);

/// glExtGetTexLevelParameterivQCOM
/// * `texture` class: texture
pub type glExtGetTexLevelParameterivQCOM_t = unsafe extern "system" fn(texture: GLuint, face: GLenum, level: GLint, pname: GLenum, params: *mut GLint);

/// glExtGetTexSubImageQCOM
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glExtGetTexSubImageQCOM_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, texels: *mut void);

/// glExtGetTexturesQCOM
/// * `textures` class: texture
pub type glExtGetTexturesQCOM_t = unsafe extern "system" fn(textures: *mut GLuint, maxTextures: GLint, numTextures: *mut GLint);

/// glExtIsProgramBinaryQCOM
/// * `program` class: program
pub type glExtIsProgramBinaryQCOM_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

/// glExtTexObjectStateOverrideiQCOM
pub type glExtTexObjectStateOverrideiQCOM_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

/// glExtractComponentEXT
pub type glExtractComponentEXT_t = unsafe extern "system" fn(res: GLuint, src: GLuint, num: GLuint);

/// glExtrapolateTex2DQCOM
/// * `src1` class: texture
/// * `src2` class: texture
/// * `output` class: texture
pub type glExtrapolateTex2DQCOM_t = unsafe extern "system" fn(src1: GLuint, src2: GLuint, output: GLuint, scaleFactor: GLfloat);

/// glFeedbackBuffer
/// * `type` group: FeedbackType
/// * `buffer` group: FeedbackElement
/// * `buffer` len: size
pub type glFeedbackBuffer_t = unsafe extern "system" fn(size: GLsizei, type_: FeedbackType, buffer: *mut GLfloat);

/// glFeedbackBufferxOES
/// * `buffer` len: n
pub type glFeedbackBufferxOES_t = unsafe extern "system" fn(n: GLsizei, type_: GLenum, buffer: *const GLfixed);

/// glFenceSync
/// * `condition` group: SyncCondition
/// * `flags` group: SyncBehaviorFlags
pub type glFenceSync_t = unsafe extern "system" fn(condition: SyncCondition, flags: GLbitfield) -> GLsync;

/// glFenceSyncAPPLE
/// * `condition` group: SyncCondition
/// * `flags` group: SyncBehaviorFlags
pub type glFenceSyncAPPLE_t = unsafe extern "system" fn(condition: SyncCondition, flags: GLbitfield) -> GLsync;

/// glFinalCombinerInputNV
/// * `variable` group: CombinerVariableNV
/// * `input` group: CombinerRegisterNV
/// * `mapping` group: CombinerMappingNV
/// * `componentUsage` group: CombinerComponentUsageNV
pub type glFinalCombinerInputNV_t = unsafe extern "system" fn(variable: CombinerVariableNV, input: CombinerRegisterNV, mapping: CombinerMappingNV, componentUsage: CombinerComponentUsageNV);

/// glFinish
pub type glFinish_t = unsafe extern "system" fn();

/// glFinishAsyncSGIX
pub type glFinishAsyncSGIX_t = unsafe extern "system" fn(markerp: *mut GLuint) -> GLint;

/// glFinishFenceAPPLE
/// * `fence` group: FenceNV
pub type glFinishFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint);

/// glFinishFenceNV
/// * `fence` group: FenceNV
pub type glFinishFenceNV_t = unsafe extern "system" fn(fence: GLuint);

/// glFinishObjectAPPLE
/// * `object` group: ObjectTypeAPPLE
pub type glFinishObjectAPPLE_t = unsafe extern "system" fn(object: ObjectTypeAPPLE, name: GLint);

/// glFinishTextureSUNX
pub type glFinishTextureSUNX_t = unsafe extern "system" fn();

/// glFlush
pub type glFlush_t = unsafe extern "system" fn();

/// glFlushMappedBufferRange
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
pub type glFlushMappedBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr);

/// glFlushMappedBufferRangeAPPLE
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glFlushMappedBufferRangeAPPLE_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr);

/// glFlushMappedBufferRangeEXT
/// * `target` group: BufferTargetARB
pub type glFlushMappedBufferRangeEXT_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr);

/// glFlushMappedNamedBufferRange
/// * `buffer` class: buffer
/// * `length` group: BufferSize
pub type glFlushMappedNamedBufferRange_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

/// glFlushMappedNamedBufferRangeEXT
/// * `buffer` class: buffer
pub type glFlushMappedNamedBufferRangeEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

/// glFlushPixelDataRangeNV
/// * `target` group: PixelDataRangeTargetNV
pub type glFlushPixelDataRangeNV_t = unsafe extern "system" fn(target: PixelDataRangeTargetNV);

/// glFlushRasterSGIX
pub type glFlushRasterSGIX_t = unsafe extern "system" fn();

/// glFlushStaticDataIBM
pub type glFlushStaticDataIBM_t = unsafe extern "system" fn(target: GLenum);

/// glFlushVertexArrayRangeAPPLE
/// * `pointer` len: length
pub type glFlushVertexArrayRangeAPPLE_t = unsafe extern "system" fn(length: GLsizei, pointer: *mut void);

/// glFlushVertexArrayRangeNV
pub type glFlushVertexArrayRangeNV_t = unsafe extern "system" fn();

/// glFogCoordFormatNV
pub type glFogCoordFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

/// glFogCoordPointer
/// * `type` group: FogPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glFogCoordPointer_t = unsafe extern "system" fn(type_: FogPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// glFogCoordPointerEXT
/// * `type` group: FogPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glFogCoordPointerEXT_t = unsafe extern "system" fn(type_: FogPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// glFogCoordPointerListIBM
/// * `type` group: FogPointerTypeIBM
/// * `pointer` len: COMPSIZE(type,stride)
pub type glFogCoordPointerListIBM_t = unsafe extern "system" fn(type_: FogPointerTypeIBM, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// glFogCoordd
/// * `coord` group: CoordD
pub type glFogCoordd_t = unsafe extern "system" fn(coord: GLdouble);

/// glFogCoorddEXT
/// * `coord` group: CoordD
pub type glFogCoorddEXT_t = unsafe extern "system" fn(coord: GLdouble);

/// glFogCoorddv
/// * `coord` group: CoordD
pub type glFogCoorddv_t = unsafe extern "system" fn(coord: *const GLdouble);

/// glFogCoorddvEXT
/// * `coord` group: CoordD
pub type glFogCoorddvEXT_t = unsafe extern "system" fn(coord: *const GLdouble);

/// glFogCoordf
/// * `coord` group: CoordF
pub type glFogCoordf_t = unsafe extern "system" fn(coord: GLfloat);

/// glFogCoordfEXT
/// * `coord` group: CoordF
pub type glFogCoordfEXT_t = unsafe extern "system" fn(coord: GLfloat);

/// glFogCoordfv
/// * `coord` group: CoordF
pub type glFogCoordfv_t = unsafe extern "system" fn(coord: *const GLfloat);

/// glFogCoordfvEXT
/// * `coord` group: CoordF
pub type glFogCoordfvEXT_t = unsafe extern "system" fn(coord: *const GLfloat);

/// glFogCoordhNV
/// * `fog` group: Half16NV
pub type glFogCoordhNV_t = unsafe extern "system" fn(fog: GLhalfNV);

/// glFogCoordhvNV
/// * `fog` group: Half16NV
pub type glFogCoordhvNV_t = unsafe extern "system" fn(fog: *const GLhalfNV);

/// glFogFuncSGIS
/// * `points` len: n*2
pub type glFogFuncSGIS_t = unsafe extern "system" fn(n: GLsizei, points: *const GLfloat);

/// glFogf
/// * `pname` group: FogParameter
/// * `param` group: CheckedFloat32
pub type glFogf_t = unsafe extern "system" fn(pname: FogParameter, param: GLfloat);

/// glFogfv
/// * `pname` group: FogParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glFogfv_t = unsafe extern "system" fn(pname: FogParameter, params: *const GLfloat);

/// glFogi
/// * `pname` group: FogParameter
/// * `param` group: CheckedInt32
pub type glFogi_t = unsafe extern "system" fn(pname: FogParameter, param: GLint);

/// glFogiv
/// * `pname` group: FogParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glFogiv_t = unsafe extern "system" fn(pname: FogParameter, params: *const GLint);

/// glFogx
/// * `pname` group: FogPName
pub type glFogx_t = unsafe extern "system" fn(pname: FogPName, param: GLfixed);

/// glFogxOES
/// * `pname` group: FogPName
pub type glFogxOES_t = unsafe extern "system" fn(pname: FogPName, param: GLfixed);

/// glFogxv
/// * `pname` group: FogPName
/// * `param` len: COMPSIZE(pname)
pub type glFogxv_t = unsafe extern "system" fn(pname: FogPName, param: *const GLfixed);

/// glFogxvOES
/// * `pname` group: FogPName
/// * `param` len: COMPSIZE(pname)
pub type glFogxvOES_t = unsafe extern "system" fn(pname: FogPName, param: *const GLfixed);

/// glFragmentColorMaterialSGIX
/// * `face` group: MaterialFace
/// * `mode` group: MaterialParameter
pub type glFragmentColorMaterialSGIX_t = unsafe extern "system" fn(face: MaterialFace, mode: MaterialParameter);

/// glFragmentCoverageColorNV
pub type glFragmentCoverageColorNV_t = unsafe extern "system" fn(color: GLuint);

/// glFragmentLightModelfSGIX
/// * `pname` group: FragmentLightModelParameterSGIX
/// * `param` group: CheckedFloat32
pub type glFragmentLightModelfSGIX_t = unsafe extern "system" fn(pname: FragmentLightModelParameterSGIX, param: GLfloat);

/// glFragmentLightModelfvSGIX
/// * `pname` group: FragmentLightModelParameterSGIX
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentLightModelfvSGIX_t = unsafe extern "system" fn(pname: FragmentLightModelParameterSGIX, params: *const GLfloat);

/// glFragmentLightModeliSGIX
/// * `pname` group: FragmentLightModelParameterSGIX
/// * `param` group: CheckedInt32
pub type glFragmentLightModeliSGIX_t = unsafe extern "system" fn(pname: FragmentLightModelParameterSGIX, param: GLint);

/// glFragmentLightModelivSGIX
/// * `pname` group: FragmentLightModelParameterSGIX
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentLightModelivSGIX_t = unsafe extern "system" fn(pname: FragmentLightModelParameterSGIX, params: *const GLint);

/// glFragmentLightfSGIX
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `param` group: CheckedFloat32
pub type glFragmentLightfSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, param: GLfloat);

/// glFragmentLightfvSGIX
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentLightfvSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, params: *const GLfloat);

/// glFragmentLightiSGIX
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `param` group: CheckedInt32
pub type glFragmentLightiSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, param: GLint);

/// glFragmentLightivSGIX
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentLightivSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, params: *const GLint);

/// glFragmentMaterialfSGIX
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` group: CheckedFloat32
pub type glFragmentMaterialfSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfloat);

/// glFragmentMaterialfvSGIX
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentMaterialfvSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *const GLfloat);

/// glFragmentMaterialiSGIX
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` group: CheckedInt32
pub type glFragmentMaterialiSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLint);

/// glFragmentMaterialivSGIX
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glFragmentMaterialivSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *const GLint);

/// glFrameTerminatorGREMEDY
pub type glFrameTerminatorGREMEDY_t = unsafe extern "system" fn();

/// glFrameZoomSGIX
/// * `factor` group: CheckedInt32
pub type glFrameZoomSGIX_t = unsafe extern "system" fn(factor: GLint);

/// glFramebufferDrawBufferEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `mode` group: DrawBufferMode
pub type glFramebufferDrawBufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, mode: DrawBufferMode);

/// glFramebufferDrawBuffersEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
pub type glFramebufferDrawBuffersEXT_t = unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const DrawBufferMode);

/// glFramebufferFetchBarrierEXT
pub type glFramebufferFetchBarrierEXT_t = unsafe extern "system" fn();

/// glFramebufferFetchBarrierQCOM
pub type glFramebufferFetchBarrierQCOM_t = unsafe extern "system" fn();

/// glFramebufferFoveationConfigQCOM
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
pub type glFramebufferFoveationConfigQCOM_t = unsafe extern "system" fn(framebuffer: GLuint, numLayers: GLuint, focalPointsPerLayer: GLuint, requestedFeatures: GLuint, providedFeatures: *mut GLuint);

/// glFramebufferFoveationParametersQCOM
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `focalX` group: CheckedFloat32
/// * `focalY` group: CheckedFloat32
/// * `gainX` group: CheckedFloat32
/// * `gainY` group: CheckedFloat32
/// * `foveaArea` group: CheckedFloat32
pub type glFramebufferFoveationParametersQCOM_t = unsafe extern "system" fn(framebuffer: GLuint, layer: GLuint, focalPoint: GLuint, focalX: GLfloat, focalY: GLfloat, gainX: GLfloat, gainY: GLfloat, foveaArea: GLfloat);

/// glFramebufferParameteri
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferParameterName
pub type glFramebufferParameteri_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferParameterName, param: GLint);

/// glFramebufferParameteriMESA
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferParameterName
pub type glFramebufferParameteriMESA_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferParameterName, param: GLint);

/// glFramebufferPixelLocalStorageSizeEXT
pub type glFramebufferPixelLocalStorageSizeEXT_t = unsafe extern "system" fn(target: GLuint, size: GLsizei);

/// glFramebufferReadBufferEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `mode` group: ReadBufferMode
pub type glFramebufferReadBufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, mode: ReadBufferMode);

/// glFramebufferRenderbuffer
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glFramebufferRenderbuffer_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// glFramebufferRenderbufferEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glFramebufferRenderbufferEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// glFramebufferRenderbufferOES
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glFramebufferRenderbufferOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// glFramebufferSampleLocationsfvARB
/// * `target` group: FramebufferTarget
pub type glFramebufferSampleLocationsfvARB_t = unsafe extern "system" fn(target: FramebufferTarget, start: GLuint, count: GLsizei, v: *const GLfloat);

/// glFramebufferSampleLocationsfvNV
/// * `target` group: FramebufferTarget
pub type glFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(target: FramebufferTarget, start: GLuint, count: GLsizei, v: *const GLfloat);

/// glFramebufferSamplePositionsfvAMD
/// * `target` group: FramebufferTarget
pub type glFramebufferSamplePositionsfvAMD_t = unsafe extern "system" fn(target: FramebufferTarget, numsamples: GLuint, pixelindex: GLuint, values: *const GLfloat);

/// glFramebufferTexture
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
pub type glFramebufferTexture_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// glFramebufferTexture1D
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture1D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// glFramebufferTexture1DEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture1DEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// glFramebufferTexture2D
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// glFramebufferTexture2DDownsampleIMG
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DDownsampleIMG_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, xscale: GLint, yscale: GLint);

/// glFramebufferTexture2DEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// glFramebufferTexture2DMultisampleEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DMultisampleEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, samples: GLsizei);

/// glFramebufferTexture2DMultisampleIMG
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DMultisampleIMG_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, samples: GLsizei);

/// glFramebufferTexture2DOES
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture2DOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// glFramebufferTexture3D
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture3D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

/// glFramebufferTexture3DEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture3DEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

/// glFramebufferTexture3DOES
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
pub type glFramebufferTexture3DOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

/// glFramebufferTextureARB
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureARB_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// glFramebufferTextureEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// glFramebufferTextureFaceARB
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `face` group: TextureTarget
pub type glFramebufferTextureFaceARB_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, face: TextureTarget);

/// glFramebufferTextureFaceEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `face` group: TextureTarget
pub type glFramebufferTextureFaceEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, face: TextureTarget);

/// glFramebufferTextureLayer
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glFramebufferTextureLayer_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// glFramebufferTextureLayerARB
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glFramebufferTextureLayerARB_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// glFramebufferTextureLayerDownsampleIMG
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glFramebufferTextureLayerDownsampleIMG_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint, xscale: GLint, yscale: GLint);

/// glFramebufferTextureLayerEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glFramebufferTextureLayerEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// glFramebufferTextureMultisampleMultiviewOVR
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureMultisampleMultiviewOVR_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, samples: GLsizei, baseViewIndex: GLint, numViews: GLsizei);

/// glFramebufferTextureMultiviewOVR
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureMultiviewOVR_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, baseViewIndex: GLint, numViews: GLsizei);

/// glFramebufferTextureOES
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glFramebufferTextureOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// glFreeObjectBufferATI
/// * `buffer` class: buffer
pub type glFreeObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint);

/// glFrontFace
/// * `mode` group: FrontFaceDirection
pub type glFrontFace_t = unsafe extern "system" fn(mode: FrontFaceDirection);

/// glFrustum
pub type glFrustum_t = unsafe extern "system" fn(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

/// glFrustumf
pub type glFrustumf_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

/// glFrustumfOES
pub type glFrustumfOES_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

/// glFrustumx
pub type glFrustumx_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

/// glFrustumxOES
pub type glFrustumxOES_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

/// glGenAsyncMarkersSGIX
pub type glGenAsyncMarkersSGIX_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

/// glGenBuffers
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glGenBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

/// glGenBuffersARB
/// * `buffers` class: buffer
/// * `buffers` len: n
pub type glGenBuffersARB_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

/// glGenFencesAPPLE
/// * `fences` group: FenceNV
/// * `fences` len: n
pub type glGenFencesAPPLE_t = unsafe extern "system" fn(n: GLsizei, fences: *mut GLuint);

/// glGenFencesNV
/// * `fences` group: FenceNV
/// * `fences` len: n
pub type glGenFencesNV_t = unsafe extern "system" fn(n: GLsizei, fences: *mut GLuint);

/// glGenFragmentShadersATI
pub type glGenFragmentShadersATI_t = unsafe extern "system" fn(range: GLuint) -> GLuint;

/// glGenFramebuffers
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glGenFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

/// glGenFramebuffersEXT
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glGenFramebuffersEXT_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

/// glGenFramebuffersOES
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
pub type glGenFramebuffersOES_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

/// glGenLists
pub type glGenLists_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

/// glGenNamesAMD
/// * `names` len: num
pub type glGenNamesAMD_t = unsafe extern "system" fn(identifier: GLenum, num: GLuint, names: *mut GLuint);

/// glGenOcclusionQueriesNV
/// * `ids` len: n
pub type glGenOcclusionQueriesNV_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// glGenPathsNV
pub type glGenPathsNV_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

/// glGenPerfMonitorsAMD
/// * `monitors` len: n
pub type glGenPerfMonitorsAMD_t = unsafe extern "system" fn(n: GLsizei, monitors: *mut GLuint);

/// glGenProgramPipelines
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glGenProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

/// glGenProgramPipelinesEXT
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
pub type glGenProgramPipelinesEXT_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

/// glGenProgramsARB
/// * `programs` class: program
/// * `programs` len: n
pub type glGenProgramsARB_t = unsafe extern "system" fn(n: GLsizei, programs: *mut GLuint);

/// glGenProgramsNV
/// * `programs` class: program
/// * `programs` len: n
pub type glGenProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *mut GLuint);

/// glGenQueries
/// * `ids` class: query
/// * `ids` len: n
pub type glGenQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// glGenQueriesARB
/// * `ids` class: query
/// * `ids` len: n
pub type glGenQueriesARB_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// glGenQueriesEXT
/// * `ids` class: query
/// * `ids` len: n
pub type glGenQueriesEXT_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// glGenQueryResourceTagNV
/// * `tagIds` len: n
pub type glGenQueryResourceTagNV_t = unsafe extern "system" fn(n: GLsizei, tagIds: *mut GLint);

/// glGenRenderbuffers
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glGenRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

/// glGenRenderbuffersEXT
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glGenRenderbuffersEXT_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

/// glGenRenderbuffersOES
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
pub type glGenRenderbuffersOES_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

/// glGenSamplers
/// * `samplers` class: sampler
/// * `samplers` len: count
pub type glGenSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);

/// glGenSemaphoresEXT
/// * `semaphores` len: n
pub type glGenSemaphoresEXT_t = unsafe extern "system" fn(n: GLsizei, semaphores: *mut GLuint);

/// glGenSymbolsEXT
/// * `datatype` group: DataTypeEXT
/// * `storagetype` group: VertexShaderStorageTypeEXT
/// * `range` group: ParameterRangeEXT
pub type glGenSymbolsEXT_t = unsafe extern "system" fn(datatype: DataTypeEXT, storagetype: VertexShaderStorageTypeEXT, range: ParameterRangeEXT, components: GLuint) -> GLuint;

/// glGenTextures
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
pub type glGenTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);

/// glGenTexturesEXT
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
pub type glGenTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);

/// glGenTransformFeedbacks
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glGenTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// glGenTransformFeedbacksNV
/// * `ids` class: transform feedback
/// * `ids` len: n
pub type glGenTransformFeedbacksNV_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

/// glGenVertexArrays
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glGenVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

/// glGenVertexArraysAPPLE
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glGenVertexArraysAPPLE_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

/// glGenVertexArraysOES
/// * `arrays` class: vertex array
/// * `arrays` len: n
pub type glGenVertexArraysOES_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

/// glGenVertexShadersEXT
pub type glGenVertexShadersEXT_t = unsafe extern "system" fn(range: GLuint) -> GLuint;

/// glGenerateMipmap
/// * `target` group: TextureTarget
pub type glGenerateMipmap_t = unsafe extern "system" fn(target: TextureTarget);

/// glGenerateMipmapEXT
/// * `target` group: TextureTarget
pub type glGenerateMipmapEXT_t = unsafe extern "system" fn(target: TextureTarget);

/// glGenerateMipmapOES
/// * `target` group: TextureTarget
pub type glGenerateMipmapOES_t = unsafe extern "system" fn(target: TextureTarget);

/// glGenerateMultiTexMipmapEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
pub type glGenerateMultiTexMipmapEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget);

/// glGenerateTextureMipmap
/// * `texture` class: texture
pub type glGenerateTextureMipmap_t = unsafe extern "system" fn(texture: GLuint);

/// glGenerateTextureMipmapEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
pub type glGenerateTextureMipmapEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget);

/// glGetActiveAtomicCounterBufferiv
/// * `program` class: program
/// * `pname` group: AtomicCounterBufferPName
/// * `params` len: COMPSIZE(pname)
pub type glGetActiveAtomicCounterBufferiv_t = unsafe extern "system" fn(program: GLuint, bufferIndex: GLuint, pname: AtomicCounterBufferPName, params: *mut GLint);

/// glGetActiveAttrib
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
pub type glGetActiveAttrib_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar);

/// glGetActiveAttribARB
/// * `programObj` group: handleARB
/// * `type` group: AttributeType
/// * `name` len: maxLength
pub type glGetActiveAttribARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLcharARB);

/// glGetActiveSubroutineName
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `name` len: bufSize
pub type glGetActiveSubroutineName_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

/// glGetActiveSubroutineUniformName
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `name` len: bufSize
pub type glGetActiveSubroutineUniformName_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

/// glGetActiveSubroutineUniformiv
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `pname` group: SubroutineParameterName
/// * `values` len: COMPSIZE(pname)
pub type glGetActiveSubroutineUniformiv_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, pname: SubroutineParameterName, values: *mut GLint);

/// glGetActiveUniform
/// * `program` class: program
/// * `type` group: UniformType
/// * `name` len: bufSize
pub type glGetActiveUniform_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar);

/// glGetActiveUniformARB
/// * `programObj` group: handleARB
/// * `type` group: UniformType
/// * `name` len: maxLength
pub type glGetActiveUniformARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLcharARB);

/// glGetActiveUniformBlockName
/// * `program` class: program
/// * `uniformBlockName` len: bufSize
pub type glGetActiveUniformBlockName_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);

/// glGetActiveUniformBlockiv
/// * `program` class: program
/// * `pname` group: UniformBlockPName
/// * `params` len: COMPSIZE(program,uniformBlockIndex,pname)
pub type glGetActiveUniformBlockiv_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint);

/// glGetActiveUniformName
/// * `program` class: program
/// * `uniformName` len: bufSize
pub type glGetActiveUniformName_t = unsafe extern "system" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);

/// glGetActiveUniformsiv
/// * `program` class: program
/// * `uniformIndices` len: uniformCount
/// * `pname` group: UniformPName
/// * `params` len: COMPSIZE(uniformCount,pname)
pub type glGetActiveUniformsiv_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint);

/// glGetActiveVaryingNV
/// * `program` class: program
/// * `name` len: COMPSIZE(program,index,bufSize)
pub type glGetActiveVaryingNV_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);

/// glGetArrayObjectfvATI
/// * `array` group: EnableCap
/// * `pname` group: ArrayObjectPNameATI
pub type glGetArrayObjectfvATI_t = unsafe extern "system" fn(array: EnableCap, pname: ArrayObjectPNameATI, params: *mut GLfloat);

/// glGetArrayObjectivATI
/// * `array` group: EnableCap
/// * `pname` group: ArrayObjectPNameATI
pub type glGetArrayObjectivATI_t = unsafe extern "system" fn(array: EnableCap, pname: ArrayObjectPNameATI, params: *mut GLint);

/// glGetAttachedObjectsARB
/// * `containerObj` group: handleARB
/// * `obj` group: handleARB
/// * `obj` len: maxCount
pub type glGetAttachedObjectsARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, maxCount: GLsizei, count: *mut GLsizei, obj: *mut GLhandleARB);

/// glGetAttachedShaders
/// * `program` class: program
/// * `shaders` class: shader
/// * `shaders` len: maxCount
pub type glGetAttachedShaders_t = unsafe extern "system" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);

/// glGetAttribLocation
/// * `program` class: program
pub type glGetAttribLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// glGetAttribLocationARB
/// * `programObj` group: handleARB
pub type glGetAttribLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, name: *const GLcharARB) -> GLint;

/// glGetBooleanIndexedvEXT
/// * `target` group: BufferTargetARB
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(target)
pub type glGetBooleanIndexedvEXT_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, data: *mut GLboolean);

/// glGetBooleani_v
/// * `target` group: BufferTargetARB
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(target)
pub type glGetBooleani_v_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, data: *mut GLboolean);

/// glGetBooleanv
/// * `pname` group: GetPName
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(pname)
pub type glGetBooleanv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLboolean);

/// glGetBufferParameteri64v
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetBufferParameteri64v_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64);

/// glGetBufferParameteriv
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetBufferParameteriv_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint);

/// glGetBufferParameterivARB
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetBufferParameterivARB_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint);

/// glGetBufferParameterui64vNV
/// * `target` group: BufferTargetARB
/// * `params` len: COMPSIZE(pname)
pub type glGetBufferParameterui64vNV_t = unsafe extern "system" fn(target: BufferTargetARB, pname: GLenum, params: *mut GLuint64EXT);

/// glGetBufferPointerv
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPointerNameARB
pub type glGetBufferPointerv_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);

/// glGetBufferPointervARB
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPointerNameARB
pub type glGetBufferPointervARB_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);

/// glGetBufferPointervOES
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPointerNameARB
pub type glGetBufferPointervOES_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);

/// glGetBufferSubData
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `data` len: size
pub type glGetBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void);

/// glGetBufferSubDataARB
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffsetARB
/// * `size` group: BufferSizeARB
/// * `data` len: size
pub type glGetBufferSubDataARB_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptrARB, size: GLsizeiptrARB, data: *mut void);

/// glGetClipPlane
/// * `plane` group: ClipPlaneName
pub type glGetClipPlane_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLdouble; 4]);

/// glGetClipPlanef
/// * `plane` group: ClipPlaneName
pub type glGetClipPlanef_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLfloat; 4]);

/// glGetClipPlanefOES
/// * `plane` group: ClipPlaneName
pub type glGetClipPlanefOES_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLfloat; 4]);

/// glGetClipPlanex
/// * `plane` group: ClipPlaneName
pub type glGetClipPlanex_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLfixed; 4]);

/// glGetClipPlanexOES
/// * `plane` group: ClipPlaneName
pub type glGetClipPlanexOES_t = unsafe extern "system" fn(plane: ClipPlaneName, equation: *mut [GLfixed; 4]);

/// glGetColorTable
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(target,format,type)
pub type glGetColorTable_t = unsafe extern "system" fn(target: ColorTableTarget, format: PixelFormat, type_: PixelType, table: *mut void);

/// glGetColorTableEXT
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(target,format,type)
pub type glGetColorTableEXT_t = unsafe extern "system" fn(target: ColorTableTarget, format: PixelFormat, type_: PixelType, data: *mut void);

/// glGetColorTableParameterfv
/// * `target` group: ColorTableTarget
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterfv_t = unsafe extern "system" fn(target: ColorTableTarget, pname: GetColorTableParameterPNameSGI, params: *mut GLfloat);

/// glGetColorTableParameterfvEXT
/// * `target` group: ColorTableTarget
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterfvEXT_t = unsafe extern "system" fn(target: ColorTableTarget, pname: GetColorTableParameterPNameSGI, params: *mut GLfloat);

/// glGetColorTableParameterfvSGI
/// * `target` group: ColorTableTargetSGI
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterfvSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, pname: GetColorTableParameterPNameSGI, params: *mut GLfloat);

/// glGetColorTableParameteriv
/// * `target` group: ColorTableTarget
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameteriv_t = unsafe extern "system" fn(target: ColorTableTarget, pname: GetColorTableParameterPNameSGI, params: *mut GLint);

/// glGetColorTableParameterivEXT
/// * `target` group: ColorTableTarget
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterivEXT_t = unsafe extern "system" fn(target: ColorTableTarget, pname: GetColorTableParameterPNameSGI, params: *mut GLint);

/// glGetColorTableParameterivSGI
/// * `target` group: ColorTableTargetSGI
/// * `pname` group: GetColorTableParameterPNameSGI
/// * `params` len: COMPSIZE(pname)
pub type glGetColorTableParameterivSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, pname: GetColorTableParameterPNameSGI, params: *mut GLint);

/// glGetColorTableSGI
/// * `target` group: ColorTableTargetSGI
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: COMPSIZE(target,format,type)
pub type glGetColorTableSGI_t = unsafe extern "system" fn(target: ColorTableTargetSGI, format: PixelFormat, type_: PixelType, table: *mut void);

/// glGetCombinerInputParameterfvNV
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `variable` group: CombinerVariableNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerInputParameterfvNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, variable: CombinerVariableNV, pname: CombinerParameterNV, params: *mut GLfloat);

/// glGetCombinerInputParameterivNV
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `variable` group: CombinerVariableNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerInputParameterivNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, variable: CombinerVariableNV, pname: CombinerParameterNV, params: *mut GLint);

/// glGetCombinerOutputParameterfvNV
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerOutputParameterfvNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, pname: CombinerParameterNV, params: *mut GLfloat);

/// glGetCombinerOutputParameterivNV
/// * `stage` group: CombinerStageNV
/// * `portion` group: CombinerPortionNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerOutputParameterivNV_t = unsafe extern "system" fn(stage: CombinerStageNV, portion: CombinerPortionNV, pname: CombinerParameterNV, params: *mut GLint);

/// glGetCombinerStageParameterfvNV
/// * `stage` group: CombinerStageNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetCombinerStageParameterfvNV_t = unsafe extern "system" fn(stage: CombinerStageNV, pname: CombinerParameterNV, params: *mut GLfloat);

/// glGetCommandHeaderNV
pub type glGetCommandHeaderNV_t = unsafe extern "system" fn(tokenID: GLenum, size: GLuint) -> GLuint;

/// glGetCompressedMultiTexImageEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `lod` group: CheckedInt32
/// * `img` len: COMPSIZE(target,lod)
pub type glGetCompressedMultiTexImageEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, lod: GLint, img: *mut void);

/// glGetCompressedTexImage
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `img` group: CompressedTextureARB
/// * `img` len: COMPSIZE(target,level)
pub type glGetCompressedTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, img: *mut void);

/// glGetCompressedTexImageARB
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `img` group: CompressedTextureARB
/// * `img` len: COMPSIZE(target,level)
pub type glGetCompressedTexImageARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, img: *mut void);

/// glGetCompressedTextureImage
/// * `texture` class: texture
pub type glGetCompressedTextureImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut void);

/// glGetCompressedTextureImageEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `lod` group: CheckedInt32
/// * `img` len: COMPSIZE(target,lod)
pub type glGetCompressedTextureImageEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, lod: GLint, img: *mut void);

/// glGetCompressedTextureSubImage
/// * `texture` class: texture
pub type glGetCompressedTextureSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut void);

/// glGetConvolutionFilter
/// * `target` group: ConvolutionTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(target,format,type)
pub type glGetConvolutionFilter_t = unsafe extern "system" fn(target: ConvolutionTarget, format: PixelFormat, type_: PixelType, image: *mut void);

/// glGetConvolutionFilterEXT
/// * `target` group: ConvolutionTargetEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: COMPSIZE(target,format,type)
pub type glGetConvolutionFilterEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, format: PixelFormat, type_: PixelType, image: *mut void);

/// glGetConvolutionParameterfv
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameterfv_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: *mut GLfloat);

/// glGetConvolutionParameterfvEXT
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameterfvEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *mut GLfloat);

/// glGetConvolutionParameteriv
/// * `target` group: ConvolutionTarget
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameteriv_t = unsafe extern "system" fn(target: ConvolutionTarget, pname: ConvolutionParameterEXT, params: *mut GLint);

/// glGetConvolutionParameterivEXT
/// * `target` group: ConvolutionTargetEXT
/// * `pname` group: ConvolutionParameterEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameterivEXT_t = unsafe extern "system" fn(target: ConvolutionTargetEXT, pname: ConvolutionParameterEXT, params: *mut GLint);

/// glGetConvolutionParameterxvOES
/// * `params` len: COMPSIZE(pname)
pub type glGetConvolutionParameterxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);

/// glGetCoverageModulationTableNV
pub type glGetCoverageModulationTableNV_t = unsafe extern "system" fn(bufSize: GLsizei, v: *mut GLfloat);

/// glGetDebugMessageLog
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

/// glGetDebugMessageLogAMD
/// * `categories` len: count
/// * `severities` group: DebugSeverity
/// * `severities` len: count
/// * `ids` len: count
/// * `lengths` len: count
/// * `message` len: bufSize
pub type glGetDebugMessageLogAMD_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, categories: *mut GLenum, severities: *mut GLuint, ids: *mut GLuint, lengths: *mut GLsizei, message: *mut GLchar) -> GLuint;

/// glGetDebugMessageLogARB
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

/// glGetDebugMessageLogKHR
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

/// glGetDetailTexFuncSGIS
/// * `target` group: TextureTarget
/// * `points` len: COMPSIZE(target)
pub type glGetDetailTexFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, points: *mut GLfloat);

/// glGetDoubleIndexedvEXT
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetDoubleIndexedvEXT_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLdouble);

/// glGetDoublei_v
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetDoublei_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLdouble);

/// glGetDoublei_vEXT
/// * `pname` group: GetPName
/// * `params` len: COMPSIZE(pname)
pub type glGetDoublei_vEXT_t = unsafe extern "system" fn(pname: GetPName, index: GLuint, params: *mut GLdouble);

/// glGetDoublev
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetDoublev_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLdouble);

/// glGetDriverControlStringQCOM
/// * `driverControlString` len: bufSize
pub type glGetDriverControlStringQCOM_t = unsafe extern "system" fn(driverControl: GLuint, bufSize: GLsizei, length: *mut GLsizei, driverControlString: *mut GLchar);

/// glGetDriverControlsQCOM
/// * `driverControls` len: size
pub type glGetDriverControlsQCOM_t = unsafe extern "system" fn(num: *mut GLint, size: GLsizei, driverControls: *mut GLuint);

/// glGetError
pub type glGetError_t = unsafe extern "system" fn() -> ErrorCode;

/// glGetFenceivNV
/// * `fence` group: FenceNV
/// * `pname` group: FenceParameterNameNV
/// * `params` len: COMPSIZE(pname)
pub type glGetFenceivNV_t = unsafe extern "system" fn(fence: GLuint, pname: FenceParameterNameNV, params: *mut GLint);

/// glGetFinalCombinerInputParameterfvNV
/// * `variable` group: CombinerVariableNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetFinalCombinerInputParameterfvNV_t = unsafe extern "system" fn(variable: CombinerVariableNV, pname: CombinerParameterNV, params: *mut GLfloat);

/// glGetFinalCombinerInputParameterivNV
/// * `variable` group: CombinerVariableNV
/// * `pname` group: CombinerParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetFinalCombinerInputParameterivNV_t = unsafe extern "system" fn(variable: CombinerVariableNV, pname: CombinerParameterNV, params: *mut GLint);

/// glGetFirstPerfQueryIdINTEL
pub type glGetFirstPerfQueryIdINTEL_t = unsafe extern "system" fn(queryId: *mut GLuint);

/// glGetFixedv
/// * `pname` group: GetPName
pub type glGetFixedv_t = unsafe extern "system" fn(pname: GetPName, params: *mut GLfixed);

/// glGetFixedvOES
/// * `pname` group: GetPName
/// * `params` len: COMPSIZE(pname)
pub type glGetFixedvOES_t = unsafe extern "system" fn(pname: GetPName, params: *mut GLfixed);

/// glGetFloatIndexedvEXT
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetFloatIndexedvEXT_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

/// glGetFloati_v
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetFloati_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

/// glGetFloati_vEXT
/// * `pname` group: GetPName
/// * `params` len: COMPSIZE(pname)
pub type glGetFloati_vEXT_t = unsafe extern "system" fn(pname: GetPName, index: GLuint, params: *mut GLfloat);

/// glGetFloati_vNV
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetFloati_vNV_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

/// glGetFloati_vOES
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetFloati_vOES_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

/// glGetFloatv
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetFloatv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLfloat);

/// glGetFogFuncSGIS
/// * `points` len: COMPSIZE()
pub type glGetFogFuncSGIS_t = unsafe extern "system" fn(points: *mut GLfloat);

/// glGetFragDataIndex
/// * `program` class: program
pub type glGetFragDataIndex_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// glGetFragDataIndexEXT
/// * `program` class: program
pub type glGetFragDataIndexEXT_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// glGetFragDataLocation
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glGetFragDataLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// glGetFragDataLocationEXT
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glGetFragDataLocationEXT_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// glGetFragmentLightfvSGIX
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `params` len: COMPSIZE(pname)
pub type glGetFragmentLightfvSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, params: *mut GLfloat);

/// glGetFragmentLightivSGIX
/// * `light` group: FragmentLightNameSGIX
/// * `pname` group: FragmentLightParameterSGIX
/// * `params` len: COMPSIZE(pname)
pub type glGetFragmentLightivSGIX_t = unsafe extern "system" fn(light: FragmentLightNameSGIX, pname: FragmentLightParameterSGIX, params: *mut GLint);

/// glGetFragmentMaterialfvSGIX
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetFragmentMaterialfvSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLfloat);

/// glGetFragmentMaterialivSGIX
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetFragmentMaterialivSGIX_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLint);

/// glGetFramebufferAttachmentParameteriv
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// glGetFramebufferAttachmentParameterivEXT
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferAttachmentParameterivEXT_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// glGetFramebufferAttachmentParameterivOES
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferAttachmentParameterivOES_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// glGetFramebufferParameterfvAMD
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferAttachmentParameterName
pub type glGetFramebufferParameterfvAMD_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, numsamples: GLuint, pixelindex: GLuint, size: GLsizei, values: *mut GLfloat);

/// glGetFramebufferParameteriv
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferParameteriv_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// glGetFramebufferParameterivEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `pname` group: GetFramebufferParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GetFramebufferParameter, params: *mut GLint);

/// glGetFramebufferParameterivMESA
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetFramebufferParameterivMESA_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// glGetFramebufferPixelLocalStorageSizeEXT
/// * `target` group: FramebufferTarget
pub type glGetFramebufferPixelLocalStorageSizeEXT_t = unsafe extern "system" fn(target: GLuint) -> GLsizei;

/// glGetGraphicsResetStatus
pub type glGetGraphicsResetStatus_t = unsafe extern "system" fn() -> GraphicsResetStatus;

/// glGetGraphicsResetStatusARB
pub type glGetGraphicsResetStatusARB_t = unsafe extern "system" fn() -> GraphicsResetStatus;

/// glGetGraphicsResetStatusEXT
pub type glGetGraphicsResetStatusEXT_t = unsafe extern "system" fn() -> GraphicsResetStatus;

/// glGetGraphicsResetStatusKHR
pub type glGetGraphicsResetStatusKHR_t = unsafe extern "system" fn() -> GraphicsResetStatus;

/// glGetHandleARB
pub type glGetHandleARB_t = unsafe extern "system" fn(pname: GLenum) -> GLhandleARB;

/// glGetHistogram
/// * `target` group: HistogramTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: COMPSIZE(target,format,type)
pub type glGetHistogram_t = unsafe extern "system" fn(target: HistogramTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, values: *mut void);

/// glGetHistogramEXT
/// * `target` group: HistogramTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: COMPSIZE(target,format,type)
pub type glGetHistogramEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, values: *mut void);

/// glGetHistogramParameterfv
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameterfv_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLfloat);

/// glGetHistogramParameterfvEXT
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameterfvEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLfloat);

/// glGetHistogramParameteriv
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameteriv_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLint);

/// glGetHistogramParameterivEXT
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameterivEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLint);

/// glGetHistogramParameterxvOES
/// * `target` group: HistogramTargetEXT
/// * `pname` group: GetHistogramParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetHistogramParameterxvOES_t = unsafe extern "system" fn(target: HistogramTargetEXT, pname: GetHistogramParameterPNameEXT, params: *mut GLfixed);

/// glGetImageHandleARB
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `format` group: PixelFormat
pub type glGetImageHandleARB_t = unsafe extern "system" fn(texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: PixelFormat) -> GLuint64;

/// glGetImageHandleNV
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `format` group: PixelFormat
pub type glGetImageHandleNV_t = unsafe extern "system" fn(texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: PixelFormat) -> GLuint64;

/// glGetImageTransformParameterfvHP
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
/// * `params` len: COMPSIZE(pname)
pub type glGetImageTransformParameterfvHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, params: *mut GLfloat);

/// glGetImageTransformParameterivHP
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
/// * `params` len: COMPSIZE(pname)
pub type glGetImageTransformParameterivHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, params: *mut GLint);

/// glGetInfoLogARB
/// * `obj` group: handleARB
/// * `infoLog` len: maxLength
pub type glGetInfoLogARB_t = unsafe extern "system" fn(obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLcharARB);

/// glGetInstrumentsSGIX
pub type glGetInstrumentsSGIX_t = unsafe extern "system" fn() -> GLint;

/// glGetInteger64i_v
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetInteger64i_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint64);

/// glGetInteger64v
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetInteger64v_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint64);

/// glGetInteger64vAPPLE
/// * `pname` group: GetPName
pub type glGetInteger64vAPPLE_t = unsafe extern "system" fn(pname: GetPName, params: *mut GLint64);

/// glGetInteger64vEXT
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetInteger64vEXT_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint64);

/// glGetIntegerIndexedvEXT
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetIntegerIndexedvEXT_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint);

/// glGetIntegeri_v
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
pub type glGetIntegeri_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint);

/// glGetIntegeri_vEXT
/// * `target` group: GetPName
pub type glGetIntegeri_vEXT_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint);

/// glGetIntegerui64i_vNV
/// * `result` len: COMPSIZE(value)
pub type glGetIntegerui64i_vNV_t = unsafe extern "system" fn(value: GLenum, index: GLuint, result: *mut GLuint64EXT);

/// glGetIntegerui64vNV
/// * `result` len: COMPSIZE(value)
pub type glGetIntegerui64vNV_t = unsafe extern "system" fn(value: GLenum, result: *mut GLuint64EXT);

/// glGetIntegerv
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetIntegerv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint);

/// glGetInternalformatSampleivNV
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `pname` group: InternalFormatPName
/// * `params` len: count
pub type glGetInternalformatSampleivNV_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, samples: GLsizei, pname: InternalFormatPName, count: GLsizei, params: *mut GLint);

/// glGetInternalformati64v
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `pname` group: InternalFormatPName
/// * `params` len: count
pub type glGetInternalformati64v_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint64);

/// glGetInternalformativ
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `pname` group: InternalFormatPName
/// * `params` len: count
pub type glGetInternalformativ_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint);

/// glGetInvariantBooleanvEXT
/// * `value` group: GetVariantValueEXT
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(id)
pub type glGetInvariantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLboolean);

/// glGetInvariantFloatvEXT
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetInvariantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLfloat);

/// glGetInvariantIntegervEXT
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetInvariantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLint);

/// glGetLightfv
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightfv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLfloat);

/// glGetLightiv
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightiv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLint);

/// glGetLightxOES
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightxOES_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLfixed);

/// glGetLightxv
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightxv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLfixed);

/// glGetLightxvOES
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetLightxvOES_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *mut GLfixed);

/// glGetListParameterfvSGIX
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glGetListParameterfvSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, params: *mut GLfloat);

/// glGetListParameterivSGIX
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glGetListParameterivSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, params: *mut GLint);

/// glGetLocalConstantBooleanvEXT
/// * `value` group: GetVariantValueEXT
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(id)
pub type glGetLocalConstantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLboolean);

/// glGetLocalConstantFloatvEXT
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetLocalConstantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLfloat);

/// glGetLocalConstantIntegervEXT
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetLocalConstantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLint);

/// glGetMapAttribParameterfvNV
/// * `target` group: EvalTargetNV
/// * `pname` group: MapAttribParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetMapAttribParameterfvNV_t = unsafe extern "system" fn(target: EvalTargetNV, index: GLuint, pname: MapAttribParameterNV, params: *mut GLfloat);

/// glGetMapAttribParameterivNV
/// * `target` group: EvalTargetNV
/// * `pname` group: MapAttribParameterNV
/// * `params` len: COMPSIZE(pname)
pub type glGetMapAttribParameterivNV_t = unsafe extern "system" fn(target: EvalTargetNV, index: GLuint, pname: MapAttribParameterNV, params: *mut GLint);

/// glGetMapControlPointsNV
/// * `target` group: EvalTargetNV
/// * `type` group: MapTypeNV
/// * `packed` group: Boolean
/// * `points` len: COMPSIZE(target)
pub type glGetMapControlPointsNV_t = unsafe extern "system" fn(target: EvalTargetNV, index: GLuint, type_: MapTypeNV, ustride: GLsizei, vstride: GLsizei, packed: GLboolean, points: *mut void);

/// glGetMapParameterfvNV
/// * `target` group: EvalTargetNV
/// * `pname` group: MapParameterNV
/// * `params` len: COMPSIZE(target,pname)
pub type glGetMapParameterfvNV_t = unsafe extern "system" fn(target: EvalTargetNV, pname: MapParameterNV, params: *mut GLfloat);

/// glGetMapParameterivNV
/// * `target` group: EvalTargetNV
/// * `pname` group: MapParameterNV
/// * `params` len: COMPSIZE(target,pname)
pub type glGetMapParameterivNV_t = unsafe extern "system" fn(target: EvalTargetNV, pname: MapParameterNV, params: *mut GLint);

/// glGetMapdv
/// * `target` group: MapTarget
/// * `query` group: GetMapQuery
/// * `v` len: COMPSIZE(target,query)
pub type glGetMapdv_t = unsafe extern "system" fn(target: MapTarget, query: GetMapQuery, v: *mut GLdouble);

/// glGetMapfv
/// * `target` group: MapTarget
/// * `query` group: GetMapQuery
/// * `v` len: COMPSIZE(target,query)
pub type glGetMapfv_t = unsafe extern "system" fn(target: MapTarget, query: GetMapQuery, v: *mut GLfloat);

/// glGetMapiv
/// * `target` group: MapTarget
/// * `query` group: GetMapQuery
/// * `v` len: COMPSIZE(target,query)
pub type glGetMapiv_t = unsafe extern "system" fn(target: MapTarget, query: GetMapQuery, v: *mut GLint);

/// glGetMapxvOES
/// * `target` group: MapTarget
/// * `query` group: GetMapQuery
/// * `v` len: COMPSIZE(query)
pub type glGetMapxvOES_t = unsafe extern "system" fn(target: MapTarget, query: GetMapQuery, v: *mut GLfixed);

/// glGetMaterialfv
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMaterialfv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLfloat);

/// glGetMaterialiv
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMaterialiv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLint);

/// glGetMaterialxOES
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
pub type glGetMaterialxOES_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfixed);

/// glGetMaterialxv
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMaterialxv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLfixed);

/// glGetMaterialxvOES
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMaterialxvOES_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *mut GLfixed);

/// glGetMemoryObjectDetachedResourcesuivNV
pub type glGetMemoryObjectDetachedResourcesuivNV_t = unsafe extern "system" fn(memory: GLuint, pname: GLenum, first: GLint, count: GLsizei, params: *mut GLuint);

/// glGetMemoryObjectParameterivEXT
/// * `pname` group: MemoryObjectParameterName
pub type glGetMemoryObjectParameterivEXT_t = unsafe extern "system" fn(memoryObject: GLuint, pname: MemoryObjectParameterName, params: *mut GLint);

/// glGetMinmax
/// * `target` group: MinmaxTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: COMPSIZE(target,format,type)
pub type glGetMinmax_t = unsafe extern "system" fn(target: MinmaxTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, values: *mut void);

/// glGetMinmaxEXT
/// * `target` group: MinmaxTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: COMPSIZE(target,format,type)
pub type glGetMinmaxEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, values: *mut void);

/// glGetMinmaxParameterfv
/// * `target` group: MinmaxTargetEXT
/// * `pname` group: GetMinmaxParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetMinmaxParameterfv_t = unsafe extern "system" fn(target: MinmaxTargetEXT, pname: GetMinmaxParameterPNameEXT, params: *mut GLfloat);

/// glGetMinmaxParameterfvEXT
/// * `target` group: MinmaxTargetEXT
/// * `pname` group: GetMinmaxParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetMinmaxParameterfvEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT, pname: GetMinmaxParameterPNameEXT, params: *mut GLfloat);

/// glGetMinmaxParameteriv
/// * `target` group: MinmaxTargetEXT
/// * `pname` group: GetMinmaxParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetMinmaxParameteriv_t = unsafe extern "system" fn(target: MinmaxTargetEXT, pname: GetMinmaxParameterPNameEXT, params: *mut GLint);

/// glGetMinmaxParameterivEXT
/// * `target` group: MinmaxTargetEXT
/// * `pname` group: GetMinmaxParameterPNameEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetMinmaxParameterivEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT, pname: GetMinmaxParameterPNameEXT, params: *mut GLint);

/// glGetMultiTexEnvfvEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexEnvfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLfloat);

/// glGetMultiTexEnvivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexEnvivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLint);

/// glGetMultiTexGendvEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexGendvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLdouble);

/// glGetMultiTexGenfvEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexGenfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLfloat);

/// glGetMultiTexGenivEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexGenivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLint);

/// glGetMultiTexImageEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(target,level,format,type)
pub type glGetMultiTexImageEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);

/// glGetMultiTexLevelParameterfvEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexLevelParameterfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

/// glGetMultiTexLevelParameterivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexLevelParameterivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);

/// glGetMultiTexParameterIivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexParameterIivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// glGetMultiTexParameterIuivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexParameterIuivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// glGetMultiTexParameterfvEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexParameterfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);

/// glGetMultiTexParameterivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetMultiTexParameterivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// glGetMultisamplefv
/// * `pname` group: GetMultisamplePNameNV
/// * `val` len: COMPSIZE(pname)
pub type glGetMultisamplefv_t = unsafe extern "system" fn(pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat);

/// glGetMultisamplefvNV
/// * `pname` group: GetMultisamplePNameNV
pub type glGetMultisamplefvNV_t = unsafe extern "system" fn(pname: GetMultisamplePNameNV, index: GLuint, val: *mut [GLfloat; 2]);

/// glGetNamedBufferParameteri64v
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
pub type glGetNamedBufferParameteri64v_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint64);

/// glGetNamedBufferParameteriv
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
pub type glGetNamedBufferParameteriv_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint);

/// glGetNamedBufferParameterivEXT
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedBufferParameterivEXT_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint);

/// glGetNamedBufferParameterui64vNV
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedBufferParameterui64vNV_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLuint64EXT);

/// glGetNamedBufferPointerv
/// * `buffer` class: buffer
/// * `pname` group: BufferPointerNameARB
pub type glGetNamedBufferPointerv_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPointerNameARB, params: *mut *mut void);

/// glGetNamedBufferPointervEXT
/// * `buffer` class: buffer
/// * `pname` group: BufferPointerNameARB
pub type glGetNamedBufferPointervEXT_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPointerNameARB, params: *mut *mut void);

/// glGetNamedBufferSubData
/// * `buffer` class: buffer
/// * `size` group: BufferSize
pub type glGetNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void);

/// glGetNamedBufferSubDataEXT
/// * `buffer` class: buffer
/// * `data` len: COMPSIZE(size)
pub type glGetNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void);

/// glGetNamedFramebufferAttachmentParameteriv
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
pub type glGetNamedFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// glGetNamedFramebufferAttachmentParameterivEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedFramebufferAttachmentParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

/// glGetNamedFramebufferParameterfvAMD
/// * `framebuffer` class: framebuffer
pub type glGetNamedFramebufferParameterfvAMD_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, numsamples: GLuint, pixelindex: GLuint, size: GLsizei, values: *mut GLfloat);

/// glGetNamedFramebufferParameteriv
/// * `framebuffer` class: framebuffer
/// * `pname` group: GetFramebufferParameter
pub type glGetNamedFramebufferParameteriv_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GetFramebufferParameter, param: *mut GLint);

/// glGetNamedFramebufferParameterivEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `pname` group: GetFramebufferParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedFramebufferParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GetFramebufferParameter, params: *mut GLint);

/// glGetNamedProgramLocalParameterIivEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glGetNamedProgramLocalParameterIivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *mut [GLint; 4]);

/// glGetNamedProgramLocalParameterIuivEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glGetNamedProgramLocalParameterIuivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *mut [GLuint; 4]);

/// glGetNamedProgramLocalParameterdvEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glGetNamedProgramLocalParameterdvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *mut [GLdouble; 4]);

/// glGetNamedProgramLocalParameterfvEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glGetNamedProgramLocalParameterfvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *mut [GLfloat; 4]);

/// glGetNamedProgramStringEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `pname` group: ProgramStringProperty
/// * `string` len: COMPSIZE(program,pname)
pub type glGetNamedProgramStringEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, pname: ProgramStringProperty, string: *mut void);

/// glGetNamedProgramivEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `pname` group: ProgramPropertyARB
pub type glGetNamedProgramivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, pname: ProgramPropertyARB, params: *mut GLint);

/// glGetNamedRenderbufferParameteriv
/// * `renderbuffer` class: renderbuffer
/// * `pname` group: RenderbufferParameterName
pub type glGetNamedRenderbufferParameteriv_t = unsafe extern "system" fn(renderbuffer: GLuint, pname: RenderbufferParameterName, params: *mut GLint);

/// glGetNamedRenderbufferParameterivEXT
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedRenderbufferParameterivEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, pname: RenderbufferParameterName, params: *mut GLint);

/// glGetNamedStringARB
/// * `name` len: namelen
/// * `string` len: bufSize
pub type glGetNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar, bufSize: GLsizei, stringlen: *mut GLint, string: *mut GLchar);

/// glGetNamedStringivARB
/// * `name` len: namelen
/// * `params` len: COMPSIZE(pname)
pub type glGetNamedStringivARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar, pname: GLenum, params: *mut GLint);

/// glGetNextPerfQueryIdINTEL
pub type glGetNextPerfQueryIdINTEL_t = unsafe extern "system" fn(queryId: GLuint, nextQueryId: *mut GLuint);

/// glGetObjectBufferfvATI
/// * `buffer` class: buffer
/// * `pname` group: ArrayObjectPNameATI
pub type glGetObjectBufferfvATI_t = unsafe extern "system" fn(buffer: GLuint, pname: ArrayObjectPNameATI, params: *mut GLfloat);

/// glGetObjectBufferivATI
/// * `buffer` class: buffer
/// * `pname` group: ArrayObjectPNameATI
pub type glGetObjectBufferivATI_t = unsafe extern "system" fn(buffer: GLuint, pname: ArrayObjectPNameATI, params: *mut GLint);

/// glGetObjectLabel
/// * `identifier` group: ObjectIdentifier
/// * `label` len: bufSize
pub type glGetObjectLabel_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// glGetObjectLabelEXT
/// * `label` len: bufSize
pub type glGetObjectLabelEXT_t = unsafe extern "system" fn(type_: GLenum, object: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// glGetObjectLabelKHR
/// * `label` len: bufSize
pub type glGetObjectLabelKHR_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// glGetObjectParameterfvARB
/// * `obj` group: handleARB
/// * `params` len: COMPSIZE(pname)
pub type glGetObjectParameterfvARB_t = unsafe extern "system" fn(obj: GLhandleARB, pname: GLenum, params: *mut GLfloat);

/// glGetObjectParameterivAPPLE
/// * `params` len: COMPSIZE(pname)
pub type glGetObjectParameterivAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, pname: GLenum, params: *mut GLint);

/// glGetObjectParameterivARB
/// * `obj` group: handleARB
/// * `params` len: COMPSIZE(pname)
pub type glGetObjectParameterivARB_t = unsafe extern "system" fn(obj: GLhandleARB, pname: GLenum, params: *mut GLint);

/// glGetObjectPtrLabel
/// * `label` len: bufSize
pub type glGetObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// glGetObjectPtrLabelKHR
/// * `label` len: bufSize
pub type glGetObjectPtrLabelKHR_t = unsafe extern "system" fn(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

/// glGetOcclusionQueryivNV
/// * `pname` group: OcclusionQueryParameterNameNV
/// * `params` len: COMPSIZE(pname)
pub type glGetOcclusionQueryivNV_t = unsafe extern "system" fn(id: GLuint, pname: OcclusionQueryParameterNameNV, params: *mut GLint);

/// glGetOcclusionQueryuivNV
/// * `pname` group: OcclusionQueryParameterNameNV
/// * `params` len: COMPSIZE(pname)
pub type glGetOcclusionQueryuivNV_t = unsafe extern "system" fn(id: GLuint, pname: OcclusionQueryParameterNameNV, params: *mut GLuint);

/// glGetPathColorGenfvNV
/// * `color` group: PathColor
/// * `pname` group: PathGenMode
/// * `value` len: COMPSIZE(pname)
pub type glGetPathColorGenfvNV_t = unsafe extern "system" fn(color: PathColor, pname: PathGenMode, value: *mut GLfloat);

/// glGetPathColorGenivNV
/// * `color` group: PathColor
/// * `pname` group: PathGenMode
/// * `value` len: COMPSIZE(pname)
pub type glGetPathColorGenivNV_t = unsafe extern "system" fn(color: PathColor, pname: PathGenMode, value: *mut GLint);

/// glGetPathCommandsNV
/// * `path` group: Path
/// * `commands` group: PathCommand
/// * `commands` len: COMPSIZE(path)
pub type glGetPathCommandsNV_t = unsafe extern "system" fn(path: GLuint, commands: *mut GLubyte);

/// glGetPathCoordsNV
/// * `path` group: Path
/// * `coords` len: COMPSIZE(path)
pub type glGetPathCoordsNV_t = unsafe extern "system" fn(path: GLuint, coords: *mut GLfloat);

/// glGetPathDashArrayNV
/// * `path` group: Path
/// * `dashArray` len: COMPSIZE(path)
pub type glGetPathDashArrayNV_t = unsafe extern "system" fn(path: GLuint, dashArray: *mut GLfloat);

/// glGetPathLengthNV
/// * `path` group: Path
pub type glGetPathLengthNV_t = unsafe extern "system" fn(path: GLuint, startSegment: GLsizei, numSegments: GLsizei) -> GLfloat;

/// glGetPathMetricRangeNV
/// * `metricQueryMask` group: PathMetricMask
/// * `firstPathName` group: Path
/// * `metrics` len: COMPSIZE(metricQueryMask,numPaths,stride)
pub type glGetPathMetricRangeNV_t = unsafe extern "system" fn(metricQueryMask: GLbitfield, firstPathName: GLuint, numPaths: GLsizei, stride: GLsizei, metrics: *mut GLfloat);

/// glGetPathMetricsNV
/// * `metricQueryMask` group: PathMetricMask
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `metrics` len: COMPSIZE(metricQueryMask,numPaths,stride)
pub type glGetPathMetricsNV_t = unsafe extern "system" fn(metricQueryMask: GLbitfield, numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, stride: GLsizei, metrics: *mut GLfloat);

/// glGetPathParameterfvNV
/// * `path` group: Path
/// * `pname` group: PathParameter
pub type glGetPathParameterfvNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: *mut [GLfloat; 4]);

/// glGetPathParameterivNV
/// * `path` group: Path
/// * `pname` group: PathParameter
pub type glGetPathParameterivNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: *mut [GLint; 4]);

/// glGetPathSpacingNV
/// * `pathListMode` group: PathListMode
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `transformType` group: PathTransformType
/// * `returnedSpacing` len: COMPSIZE(pathListMode,numPaths)
pub type glGetPathSpacingNV_t = unsafe extern "system" fn(pathListMode: PathListMode, numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, advanceScale: GLfloat, kerningScale: GLfloat, transformType: PathTransformType, returnedSpacing: *mut GLfloat);

/// glGetPathTexGenfvNV
/// * `texCoordSet` group: TextureUnit
/// * `pname` group: PathGenMode
/// * `value` len: COMPSIZE(pname)
pub type glGetPathTexGenfvNV_t = unsafe extern "system" fn(texCoordSet: TextureUnit, pname: PathGenMode, value: *mut GLfloat);

/// glGetPathTexGenivNV
/// * `texCoordSet` group: TextureUnit
/// * `pname` group: PathGenMode
/// * `value` len: COMPSIZE(pname)
pub type glGetPathTexGenivNV_t = unsafe extern "system" fn(texCoordSet: TextureUnit, pname: PathGenMode, value: *mut GLint);

/// glGetPerfCounterInfoINTEL
pub type glGetPerfCounterInfoINTEL_t = unsafe extern "system" fn(queryId: GLuint, counterId: GLuint, counterNameLength: GLuint, counterName: *mut GLchar, counterDescLength: GLuint, counterDesc: *mut GLchar, counterOffset: *mut GLuint, counterDataSize: *mut GLuint, counterTypeEnum: *mut GLuint, counterDataTypeEnum: *mut GLuint, rawCounterMaxValue: *mut GLuint64);

/// glGetPerfMonitorCounterDataAMD
/// * `data` len: dataSize / 4
pub type glGetPerfMonitorCounterDataAMD_t = unsafe extern "system" fn(monitor: GLuint, pname: GLenum, dataSize: GLsizei, data: *mut GLuint, bytesWritten: *mut GLint);

/// glGetPerfMonitorCounterInfoAMD
/// * `data` len: COMPSIZE(pname)
pub type glGetPerfMonitorCounterInfoAMD_t = unsafe extern "system" fn(group: GLuint, counter: GLuint, pname: GLenum, data: *mut void);

/// glGetPerfMonitorCounterStringAMD
/// * `counterString` len: bufSize
pub type glGetPerfMonitorCounterStringAMD_t = unsafe extern "system" fn(group: GLuint, counter: GLuint, bufSize: GLsizei, length: *mut GLsizei, counterString: *mut GLchar);

/// glGetPerfMonitorCountersAMD
/// * `counters` len: counterSize
pub type glGetPerfMonitorCountersAMD_t = unsafe extern "system" fn(group: GLuint, numCounters: *mut GLint, maxActiveCounters: *mut GLint, counterSize: GLsizei, counters: *mut GLuint);

/// glGetPerfMonitorGroupStringAMD
/// * `groupString` len: bufSize
pub type glGetPerfMonitorGroupStringAMD_t = unsafe extern "system" fn(group: GLuint, bufSize: GLsizei, length: *mut GLsizei, groupString: *mut GLchar);

/// glGetPerfMonitorGroupsAMD
/// * `groups` len: groupsSize
pub type glGetPerfMonitorGroupsAMD_t = unsafe extern "system" fn(numGroups: *mut GLint, groupsSize: GLsizei, groups: *mut GLuint);

/// glGetPerfQueryDataINTEL
pub type glGetPerfQueryDataINTEL_t = unsafe extern "system" fn(queryHandle: GLuint, flags: GLuint, dataSize: GLsizei, data: *mut void, bytesWritten: *mut GLuint);

/// glGetPerfQueryIdByNameINTEL
pub type glGetPerfQueryIdByNameINTEL_t = unsafe extern "system" fn(queryName: *mut GLchar, queryId: *mut GLuint);

/// glGetPerfQueryInfoINTEL
pub type glGetPerfQueryInfoINTEL_t = unsafe extern "system" fn(queryId: GLuint, queryNameLength: GLuint, queryName: *mut GLchar, dataSize: *mut GLuint, noCounters: *mut GLuint, noInstances: *mut GLuint, capsMask: *mut GLuint);

/// glGetPixelMapfv
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(map)
pub type glGetPixelMapfv_t = unsafe extern "system" fn(map: PixelMap, values: *mut GLfloat);

/// glGetPixelMapuiv
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(map)
pub type glGetPixelMapuiv_t = unsafe extern "system" fn(map: PixelMap, values: *mut GLuint);

/// glGetPixelMapusv
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(map)
pub type glGetPixelMapusv_t = unsafe extern "system" fn(map: PixelMap, values: *mut GLushort);

/// glGetPixelMapxv
/// * `map` group: PixelMap
/// * `values` len: size
pub type glGetPixelMapxv_t = unsafe extern "system" fn(map: PixelMap, size: GLint, values: *mut GLfixed);

/// glGetPixelTexGenParameterfvSGIS
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glGetPixelTexGenParameterfvSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, params: *mut GLfloat);

/// glGetPixelTexGenParameterivSGIS
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glGetPixelTexGenParameterivSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, params: *mut GLint);

/// glGetPixelTransformParameterfvEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetPixelTransformParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

/// glGetPixelTransformParameterivEXT
/// * `params` len: COMPSIZE(pname)
pub type glGetPixelTransformParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

/// glGetPointerIndexedvEXT
pub type glGetPointerIndexedvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut *mut void);

/// glGetPointeri_vEXT
pub type glGetPointeri_vEXT_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, params: *mut *mut void);

/// glGetPointerv
/// * `pname` group: GetPointervPName
pub type glGetPointerv_t = unsafe extern "system" fn(pname: GetPointervPName, params: *mut *mut void);

/// glGetPointervEXT
/// * `pname` group: GetPointervPName
pub type glGetPointervEXT_t = unsafe extern "system" fn(pname: GetPointervPName, params: *mut *mut void);

/// glGetPointervKHR
pub type glGetPointervKHR_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut void);

/// glGetPolygonStipple
/// * `mask` len: COMPSIZE()
pub type glGetPolygonStipple_t = unsafe extern "system" fn(mask: *mut GLubyte);

/// glGetProgramBinary
/// * `program` class: program
/// * `binary` len: bufSize
pub type glGetProgramBinary_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void);

/// glGetProgramBinaryOES
/// * `program` class: program
/// * `binary` len: bufSize
pub type glGetProgramBinaryOES_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void);

/// glGetProgramEnvParameterIivNV
/// * `target` group: ProgramTarget
pub type glGetProgramEnvParameterIivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLint; 4]);

/// glGetProgramEnvParameterIuivNV
/// * `target` group: ProgramTarget
pub type glGetProgramEnvParameterIuivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLuint; 4]);

/// glGetProgramEnvParameterdvARB
/// * `target` group: ProgramTarget
pub type glGetProgramEnvParameterdvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLdouble; 4]);

/// glGetProgramEnvParameterfvARB
/// * `target` group: ProgramTarget
pub type glGetProgramEnvParameterfvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLfloat; 4]);

/// glGetProgramInfoLog
/// * `program` class: program
/// * `infoLog` len: bufSize
pub type glGetProgramInfoLog_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

/// glGetProgramInterfaceiv
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `pname` group: ProgramInterfacePName
/// * `params` len: COMPSIZE(pname)
pub type glGetProgramInterfaceiv_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, pname: ProgramInterfacePName, params: *mut GLint);

/// glGetProgramLocalParameterIivNV
/// * `target` group: ProgramTarget
pub type glGetProgramLocalParameterIivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLint; 4]);

/// glGetProgramLocalParameterIuivNV
/// * `target` group: ProgramTarget
pub type glGetProgramLocalParameterIuivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLuint; 4]);

/// glGetProgramLocalParameterdvARB
/// * `target` group: ProgramTarget
pub type glGetProgramLocalParameterdvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLdouble; 4]);

/// glGetProgramLocalParameterfvARB
/// * `target` group: ProgramTarget
pub type glGetProgramLocalParameterfvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *mut [GLfloat; 4]);

/// glGetProgramNamedParameterdvNV
/// * `id` class: program
pub type glGetProgramNamedParameterdvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, params: *mut [GLdouble; 4]);

/// glGetProgramNamedParameterfvNV
/// * `id` class: program
pub type glGetProgramNamedParameterfvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, params: *mut [GLfloat; 4]);

/// glGetProgramParameterdvNV
/// * `target` group: VertexAttribEnumNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetProgramParameterdvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, pname: VertexAttribEnumNV, params: *mut [GLdouble; 4]);

/// glGetProgramParameterfvNV
/// * `target` group: VertexAttribEnumNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetProgramParameterfvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, pname: VertexAttribEnumNV, params: *mut [GLfloat; 4]);

/// glGetProgramPipelineInfoLog
/// * `pipeline` class: program pipeline
/// * `infoLog` len: bufSize
pub type glGetProgramPipelineInfoLog_t = unsafe extern "system" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

/// glGetProgramPipelineInfoLogEXT
/// * `pipeline` class: program pipeline
/// * `infoLog` len: bufSize
pub type glGetProgramPipelineInfoLogEXT_t = unsafe extern "system" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

/// glGetProgramPipelineiv
/// * `pipeline` class: program pipeline
/// * `pname` group: PipelineParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetProgramPipelineiv_t = unsafe extern "system" fn(pipeline: GLuint, pname: PipelineParameterName, params: *mut GLint);

/// glGetProgramPipelineivEXT
/// * `pipeline` class: program pipeline
/// * `pname` group: PipelineParameterName
pub type glGetProgramPipelineivEXT_t = unsafe extern "system" fn(pipeline: GLuint, pname: PipelineParameterName, params: *mut GLint);

/// glGetProgramResourceIndex
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
pub type glGetProgramResourceIndex_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLuint;

/// glGetProgramResourceLocation
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
pub type glGetProgramResourceLocation_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;

/// glGetProgramResourceLocationIndex
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
pub type glGetProgramResourceLocationIndex_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;

/// glGetProgramResourceLocationIndexEXT
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
pub type glGetProgramResourceLocationIndexEXT_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;

/// glGetProgramResourceName
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: bufSize
pub type glGetProgramResourceName_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

/// glGetProgramResourcefvNV
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `params` len: count
pub type glGetProgramResourcefvNV_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLfloat);

/// glGetProgramResourceiv
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `props` group: ProgramResourceProperty
/// * `props` len: propCount
/// * `params` len: count
pub type glGetProgramResourceiv_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, index: GLuint, propCount: GLsizei, props: *const ProgramResourceProperty, count: GLsizei, length: *mut GLsizei, params: *mut GLint);

/// glGetProgramStageiv
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `pname` group: ProgramStagePName
pub type glGetProgramStageiv_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, pname: ProgramStagePName, values: *mut GLint);

/// glGetProgramStringARB
/// * `target` group: ProgramTarget
/// * `pname` group: ProgramStringProperty
/// * `string` len: COMPSIZE(target,pname)
pub type glGetProgramStringARB_t = unsafe extern "system" fn(target: ProgramTarget, pname: ProgramStringProperty, string: *mut void);

/// glGetProgramStringNV
/// * `id` class: program
/// * `pname` group: VertexAttribEnumNV
/// * `program` group: ProgramCharacterNV
/// * `program` len: COMPSIZE(id,pname)
pub type glGetProgramStringNV_t = unsafe extern "system" fn(id: GLuint, pname: VertexAttribEnumNV, program: *mut GLubyte);

/// glGetProgramSubroutineParameteruivNV
/// * `param` len: COMPSIZE(target)
pub type glGetProgramSubroutineParameteruivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, param: *mut GLuint);

/// glGetProgramiv
/// * `program` class: program
/// * `pname` group: ProgramPropertyARB
/// * `params` len: COMPSIZE(pname)
pub type glGetProgramiv_t = unsafe extern "system" fn(program: GLuint, pname: ProgramPropertyARB, params: *mut GLint);

/// glGetProgramivARB
/// * `target` group: ProgramTarget
/// * `pname` group: ProgramPropertyARB
pub type glGetProgramivARB_t = unsafe extern "system" fn(target: ProgramTarget, pname: ProgramPropertyARB, params: *mut GLint);

/// glGetProgramivNV
/// * `id` class: program
/// * `pname` group: VertexAttribEnumNV
pub type glGetProgramivNV_t = unsafe extern "system" fn(id: GLuint, pname: VertexAttribEnumNV, params: *mut [GLint; 4]);

/// glGetQueryBufferObjecti64v
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
pub type glGetQueryBufferObjecti64v_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

/// glGetQueryBufferObjectiv
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
pub type glGetQueryBufferObjectiv_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

/// glGetQueryBufferObjectui64v
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
pub type glGetQueryBufferObjectui64v_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

/// glGetQueryBufferObjectuiv
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
pub type glGetQueryBufferObjectuiv_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

/// glGetQueryIndexediv
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryIndexediv_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint, pname: QueryParameterName, params: *mut GLint);

/// glGetQueryObjecti64v
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjecti64v_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64);

/// glGetQueryObjecti64vEXT
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjecti64vEXT_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64);

/// glGetQueryObjectiv
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectiv_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);

/// glGetQueryObjectivARB
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectivARB_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);

/// glGetQueryObjectivEXT
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectivEXT_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);

/// glGetQueryObjectui64v
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectui64v_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64);

/// glGetQueryObjectui64vEXT
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectui64vEXT_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64);

/// glGetQueryObjectuiv
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectuiv_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);

/// glGetQueryObjectuivARB
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectuivARB_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);

/// glGetQueryObjectuivEXT
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryObjectuivEXT_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);

/// glGetQueryiv
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryiv_t = unsafe extern "system" fn(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);

/// glGetQueryivARB
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryivARB_t = unsafe extern "system" fn(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);

/// glGetQueryivEXT
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetQueryivEXT_t = unsafe extern "system" fn(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);

/// glGetRenderbufferParameteriv
/// * `target` group: RenderbufferTarget
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetRenderbufferParameteriv_t = unsafe extern "system" fn(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);

/// glGetRenderbufferParameterivEXT
/// * `target` group: RenderbufferTarget
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetRenderbufferParameterivEXT_t = unsafe extern "system" fn(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);

/// glGetRenderbufferParameterivOES
/// * `target` group: RenderbufferTarget
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetRenderbufferParameterivOES_t = unsafe extern "system" fn(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);

/// glGetSamplerParameterIiv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

/// glGetSamplerParameterIivEXT
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

/// glGetSamplerParameterIivOES
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

/// glGetSamplerParameterIuiv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);

/// glGetSamplerParameterIuivEXT
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIuivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);

/// glGetSamplerParameterIuivOES
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterIuivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);

/// glGetSamplerParameterfv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat);

/// glGetSamplerParameteriv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
pub type glGetSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

/// glGetSemaphoreParameterivNV
/// * `pname` group: SemaphoreParameterName
pub type glGetSemaphoreParameterivNV_t = unsafe extern "system" fn(semaphore: GLuint, pname: SemaphoreParameterName, params: *mut GLint);

/// glGetSemaphoreParameterui64vEXT
/// * `pname` group: SemaphoreParameterName
pub type glGetSemaphoreParameterui64vEXT_t = unsafe extern "system" fn(semaphore: GLuint, pname: SemaphoreParameterName, params: *mut GLuint64);

/// glGetSeparableFilter
/// * `target` group: SeparableTargetEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: COMPSIZE(target,format,type)
/// * `column` len: COMPSIZE(target,format,type)
/// * `span` len: COMPSIZE(target,format,type)
pub type glGetSeparableFilter_t = unsafe extern "system" fn(target: SeparableTargetEXT, format: PixelFormat, type_: PixelType, row: *mut void, column: *mut void, span: *mut void);

/// glGetSeparableFilterEXT
/// * `target` group: SeparableTargetEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: COMPSIZE(target,format,type)
/// * `column` len: COMPSIZE(target,format,type)
/// * `span` len: COMPSIZE(target,format,type)
pub type glGetSeparableFilterEXT_t = unsafe extern "system" fn(target: SeparableTargetEXT, format: PixelFormat, type_: PixelType, row: *mut void, column: *mut void, span: *mut void);

/// glGetShaderInfoLog
/// * `shader` class: shader
/// * `infoLog` len: bufSize
pub type glGetShaderInfoLog_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

/// glGetShaderPrecisionFormat
/// * `shadertype` group: ShaderType
/// * `precisiontype` group: PrecisionType
pub type glGetShaderPrecisionFormat_t = unsafe extern "system" fn(shadertype: ShaderType, precisiontype: PrecisionType, range: *mut [GLint; 2], precision: *mut GLint);

/// glGetShaderSource
/// * `shader` class: shader
/// * `source` len: bufSize
pub type glGetShaderSource_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);

/// glGetShaderSourceARB
/// * `obj` group: handleARB
/// * `source` len: maxLength
pub type glGetShaderSourceARB_t = unsafe extern "system" fn(obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, source: *mut GLcharARB);

/// glGetShaderiv
/// * `shader` class: shader
/// * `pname` group: ShaderParameterName
/// * `params` len: COMPSIZE(pname)
pub type glGetShaderiv_t = unsafe extern "system" fn(shader: GLuint, pname: ShaderParameterName, params: *mut GLint);

/// glGetShadingRateImagePaletteNV
pub type glGetShadingRateImagePaletteNV_t = unsafe extern "system" fn(viewport: GLuint, entry: GLuint, rate: *mut GLenum);

/// glGetShadingRateSampleLocationivNV
pub type glGetShadingRateSampleLocationivNV_t = unsafe extern "system" fn(rate: GLenum, samples: GLuint, index: GLuint, location: *mut [GLint; 3]);

/// glGetSharpenTexFuncSGIS
/// * `target` group: TextureTarget
/// * `points` len: COMPSIZE(target)
pub type glGetSharpenTexFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, points: *mut GLfloat);

/// glGetStageIndexNV
/// * `shadertype` group: ShaderType
pub type glGetStageIndexNV_t = unsafe extern "system" fn(shadertype: ShaderType) -> GLushort;

/// glGetString
/// * `name` group: StringName
pub type glGetString_t = unsafe extern "system" fn(name: StringName) -> GLubyte;

/// glGetStringi
/// * `name` group: StringName
pub type glGetStringi_t = unsafe extern "system" fn(name: StringName, index: GLuint) -> GLubyte;

/// glGetSubroutineIndex
/// * `program` class: program
/// * `shadertype` group: ShaderType
pub type glGetSubroutineIndex_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLuint;

/// glGetSubroutineUniformLocation
/// * `program` class: program
/// * `shadertype` group: ShaderType
pub type glGetSubroutineUniformLocation_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLint;

/// glGetSynciv
/// * `sync` group: sync
/// * `sync` class: sync
/// * `pname` group: SyncParameterName
/// * `values` len: count
pub type glGetSynciv_t = unsafe extern "system" fn(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

/// glGetSyncivAPPLE
/// * `sync` class: sync
/// * `pname` group: SyncParameterName
/// * `values` len: count
pub type glGetSyncivAPPLE_t = unsafe extern "system" fn(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

/// glGetTexBumpParameterfvATI
/// * `pname` group: GetTexBumpParameterATI
/// * `param` len: COMPSIZE(pname)
pub type glGetTexBumpParameterfvATI_t = unsafe extern "system" fn(pname: GetTexBumpParameterATI, param: *mut GLfloat);

/// glGetTexBumpParameterivATI
/// * `pname` group: GetTexBumpParameterATI
/// * `param` len: COMPSIZE(pname)
pub type glGetTexBumpParameterivATI_t = unsafe extern "system" fn(pname: GetTexBumpParameterATI, param: *mut GLint);

/// glGetTexEnvfv
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexEnvfv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLfloat);

/// glGetTexEnviv
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexEnviv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLint);

/// glGetTexEnvxv
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexEnvxv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLfixed);

/// glGetTexEnvxvOES
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexEnvxvOES_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *mut GLfixed);

/// glGetTexFilterFuncSGIS
/// * `target` group: TextureTarget
/// * `filter` group: TextureFilterSGIS
/// * `weights` len: COMPSIZE(target,filter)
pub type glGetTexFilterFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, filter: TextureFilterSGIS, weights: *mut GLfloat);

/// glGetTexGendv
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGendv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLdouble);

/// glGetTexGenfv
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGenfv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLfloat);

/// glGetTexGenfvOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGenfvOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLfloat);

/// glGetTexGeniv
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGeniv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLint);

/// glGetTexGenivOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGenivOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLint);

/// glGetTexGenxvOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexGenxvOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *mut GLfixed);

/// glGetTexImage
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(target,level,format,type)
pub type glGetTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);

/// glGetTexLevelParameterfv
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexLevelParameterfv_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

/// glGetTexLevelParameteriv
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexLevelParameteriv_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);

/// glGetTexLevelParameterxvOES
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexLevelParameterxvOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfixed);

/// glGetTexParameterIiv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIiv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// glGetTexParameterIivEXT
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIivEXT_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// glGetTexParameterIivOES
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIivOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// glGetTexParameterIuiv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIuiv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// glGetTexParameterIuivEXT
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIuivEXT_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// glGetTexParameterIuivOES
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterIuivOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// glGetTexParameterPointervAPPLE
pub type glGetTexParameterPointervAPPLE_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut void);

/// glGetTexParameterfv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterfv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);

/// glGetTexParameteriv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameteriv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// glGetTexParameterxv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterxv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfixed);

/// glGetTexParameterxvOES
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTexParameterxvOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfixed);

/// glGetTextureHandleARB
/// * `texture` class: texture
pub type glGetTextureHandleARB_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

/// glGetTextureHandleIMG
/// * `texture` class: texture
pub type glGetTextureHandleIMG_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

/// glGetTextureHandleNV
/// * `texture` class: texture
pub type glGetTextureHandleNV_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

/// glGetTextureImage
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glGetTextureImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

/// glGetTextureImageEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(target,level,format,type)
pub type glGetTextureImageEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);

/// glGetTextureLevelParameterfv
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureLevelParameterfv_t = unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

/// glGetTextureLevelParameterfvEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureLevelParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

/// glGetTextureLevelParameteriv
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureLevelParameteriv_t = unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLint);

/// glGetTextureLevelParameterivEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureLevelParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);

/// glGetTextureParameterIiv
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureParameterIiv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLint);

/// glGetTextureParameterIivEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureParameterIivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// glGetTextureParameterIuiv
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureParameterIuiv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLuint);

/// glGetTextureParameterIuivEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureParameterIuivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

/// glGetTextureParameterfv
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureParameterfv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLfloat);

/// glGetTextureParameterfvEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);

/// glGetTextureParameteriv
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
pub type glGetTextureParameteriv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLint);

/// glGetTextureParameterivEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glGetTextureParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

/// glGetTextureSamplerHandleARB
/// * `texture` class: texture
/// * `sampler` class: sampler
pub type glGetTextureSamplerHandleARB_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

/// glGetTextureSamplerHandleIMG
/// * `texture` class: texture
/// * `sampler` class: sampler
pub type glGetTextureSamplerHandleIMG_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

/// glGetTextureSamplerHandleNV
/// * `texture` class: texture
/// * `sampler` class: sampler
pub type glGetTextureSamplerHandleNV_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

/// glGetTextureSubImage
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glGetTextureSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

/// glGetTrackMatrixivNV
/// * `target` group: VertexAttribEnumNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetTrackMatrixivNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, address: GLuint, pname: VertexAttribEnumNV, params: *mut GLint);

/// glGetTransformFeedbackVarying
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
pub type glGetTransformFeedbackVarying_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar);

/// glGetTransformFeedbackVaryingEXT
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
pub type glGetTransformFeedbackVaryingEXT_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar);

/// glGetTransformFeedbackVaryingNV
/// * `program` class: program
pub type glGetTransformFeedbackVaryingNV_t = unsafe extern "system" fn(program: GLuint, index: GLuint, location: *mut GLint);

/// glGetTransformFeedbacki64_v
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
pub type glGetTransformFeedbacki64_v_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint64);

/// glGetTransformFeedbacki_v
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
pub type glGetTransformFeedbacki_v_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint);

/// glGetTransformFeedbackiv
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
pub type glGetTransformFeedbackiv_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, param: *mut GLint);

/// glGetTranslatedShaderSourceANGLE
/// * `shader` class: shader
pub type glGetTranslatedShaderSourceANGLE_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);

/// glGetUniformBlockIndex
/// * `program` class: program
/// * `uniformBlockName` len: COMPSIZE()
pub type glGetUniformBlockIndex_t = unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;

/// glGetUniformBufferSizeEXT
/// * `program` class: program
pub type glGetUniformBufferSizeEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint) -> GLint;

/// glGetUniformIndices
/// * `program` class: program
/// * `uniformNames` len: COMPSIZE(uniformCount)
/// * `uniformIndices` len: COMPSIZE(uniformCount)
pub type glGetUniformIndices_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);

/// glGetUniformLocation
/// * `program` class: program
pub type glGetUniformLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// glGetUniformLocationARB
/// * `programObj` group: handleARB
pub type glGetUniformLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, name: *const GLcharARB) -> GLint;

/// glGetUniformOffsetEXT
/// * `program` class: program
pub type glGetUniformOffsetEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint) -> GLintptr;

/// glGetUniformSubroutineuiv
/// * `shadertype` group: ShaderType
pub type glGetUniformSubroutineuiv_t = unsafe extern "system" fn(shadertype: ShaderType, location: GLint, params: *mut GLuint);

/// glGetUniformdv
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformdv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLdouble);

/// glGetUniformfv
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);

/// glGetUniformfvARB
/// * `programObj` group: handleARB
/// * `params` len: COMPSIZE(programObj,location)
pub type glGetUniformfvARB_t = unsafe extern "system" fn(programObj: GLhandleARB, location: GLint, params: *mut GLfloat);

/// glGetUniformi64vARB
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformi64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint64);

/// glGetUniformi64vNV
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformi64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint64EXT);

/// glGetUniformiv
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);

/// glGetUniformivARB
/// * `programObj` group: handleARB
/// * `params` len: COMPSIZE(programObj,location)
pub type glGetUniformivARB_t = unsafe extern "system" fn(programObj: GLhandleARB, location: GLint, params: *mut GLint);

/// glGetUniformui64vARB
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint64);

/// glGetUniformui64vNV
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint64EXT);

/// glGetUniformuiv
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);

/// glGetUniformuivEXT
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
pub type glGetUniformuivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);

/// glGetUnsignedBytei_vEXT
/// * `data` len: COMPSIZE(target)
pub type glGetUnsignedBytei_vEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLubyte);

/// glGetUnsignedBytevEXT
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
pub type glGetUnsignedBytevEXT_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLubyte);

/// glGetVariantArrayObjectfvATI
/// * `pname` group: ArrayObjectPNameATI
pub type glGetVariantArrayObjectfvATI_t = unsafe extern "system" fn(id: GLuint, pname: ArrayObjectPNameATI, params: *mut GLfloat);

/// glGetVariantArrayObjectivATI
/// * `pname` group: ArrayObjectPNameATI
pub type glGetVariantArrayObjectivATI_t = unsafe extern "system" fn(id: GLuint, pname: ArrayObjectPNameATI, params: *mut GLint);

/// glGetVariantBooleanvEXT
/// * `value` group: GetVariantValueEXT
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(id)
pub type glGetVariantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLboolean);

/// glGetVariantFloatvEXT
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetVariantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLfloat);

/// glGetVariantIntegervEXT
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetVariantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut GLint);

/// glGetVariantPointervEXT
/// * `value` group: GetVariantValueEXT
/// * `data` len: COMPSIZE(id)
pub type glGetVariantPointervEXT_t = unsafe extern "system" fn(id: GLuint, value: GetVariantValueEXT, data: *mut *mut void);

/// glGetVaryingLocationNV
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
pub type glGetVaryingLocationNV_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

/// glGetVertexArrayIndexed64iv
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayIndexed64iv_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint64);

/// glGetVertexArrayIndexediv
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayIndexediv_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint);

/// glGetVertexArrayIntegeri_vEXT
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayIntegeri_vEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint);

/// glGetVertexArrayIntegervEXT
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayIntegervEXT_t = unsafe extern "system" fn(vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint);

/// glGetVertexArrayPointeri_vEXT
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayPointeri_vEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut *mut void);

/// glGetVertexArrayPointervEXT
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayPointervEXT_t = unsafe extern "system" fn(vaobj: GLuint, pname: VertexArrayPName, param: *mut *mut void);

/// glGetVertexArrayiv
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
pub type glGetVertexArrayiv_t = unsafe extern "system" fn(vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint);

/// glGetVertexAttribArrayObjectfvATI
/// * `pname` group: ArrayObjectPNameATI
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribArrayObjectfvATI_t = unsafe extern "system" fn(index: GLuint, pname: ArrayObjectPNameATI, params: *mut GLfloat);

/// glGetVertexAttribArrayObjectivATI
/// * `pname` group: ArrayObjectPNameATI
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribArrayObjectivATI_t = unsafe extern "system" fn(index: GLuint, pname: ArrayObjectPNameATI, params: *mut GLint);

/// glGetVertexAttribIiv
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribIiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLint);

/// glGetVertexAttribIivEXT
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribIivEXT_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLint);

/// glGetVertexAttribIuiv
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribIuiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint);

/// glGetVertexAttribIuivEXT
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribIuivEXT_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint);

/// glGetVertexAttribLdv
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribLdv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLdouble);

/// glGetVertexAttribLdvEXT
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribLdvEXT_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLdouble);

/// glGetVertexAttribLi64vNV
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribLi64vNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLint64EXT);

/// glGetVertexAttribLui64vARB
/// * `pname` group: VertexAttribEnum
pub type glGetVertexAttribLui64vARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint64EXT);

/// glGetVertexAttribLui64vNV
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
pub type glGetVertexAttribLui64vNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint64EXT);

/// glGetVertexAttribPointerv
/// * `pname` group: VertexAttribPointerPropertyARB
pub type glGetVertexAttribPointerv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void);

/// glGetVertexAttribPointervARB
/// * `pname` group: VertexAttribPointerPropertyARB
pub type glGetVertexAttribPointervARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void);

/// glGetVertexAttribPointervNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetVertexAttribPointervNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnumNV, pointer: *mut *mut void);

/// glGetVertexAttribdv
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribdv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]);

/// glGetVertexAttribdvARB
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribdvARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]);

/// glGetVertexAttribdvNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetVertexAttribdvNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnumNV, params: *mut GLdouble);

/// glGetVertexAttribfv
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribfv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]);

/// glGetVertexAttribfvARB
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribfvARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]);

/// glGetVertexAttribfvNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetVertexAttribfvNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnumNV, params: *mut GLfloat);

/// glGetVertexAttribiv
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]);

/// glGetVertexAttribivARB
/// * `pname` group: VertexAttribPropertyARB
pub type glGetVertexAttribivARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]);

/// glGetVertexAttribivNV
/// * `pname` group: VertexAttribEnumNV
pub type glGetVertexAttribivNV_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnumNV, params: *mut GLint);

/// glGetVideoCaptureStreamdvNV
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoCaptureStreamdvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLdouble);

/// glGetVideoCaptureStreamfvNV
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoCaptureStreamfvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLfloat);

/// glGetVideoCaptureStreamivNV
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoCaptureStreamivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLint);

/// glGetVideoCaptureivNV
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoCaptureivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, pname: GLenum, params: *mut GLint);

/// glGetVideoi64vNV
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoi64vNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLint64EXT);

/// glGetVideoivNV
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoivNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLint);

/// glGetVideoui64vNV
/// * `params` len: COMPSIZE(pname)
pub type glGetVideoui64vNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLuint64EXT);

/// glGetVideouivNV
/// * `params` len: COMPSIZE(pname)
pub type glGetVideouivNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLuint);

/// glGetVkProcAddrNV
/// * `name` len: COMPSIZE(name)
pub type glGetVkProcAddrNV_t = unsafe extern "system" fn(name: *const GLchar) -> GLVULKANPROCNV;

/// glGetnColorTable
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: bufSize
pub type glGetnColorTable_t = unsafe extern "system" fn(target: ColorTableTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, table: *mut void);

/// glGetnColorTableARB
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: bufSize
pub type glGetnColorTableARB_t = unsafe extern "system" fn(target: ColorTableTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, table: *mut void);

/// glGetnCompressedTexImage
/// * `target` group: TextureTarget
/// * `pixels` len: bufSize
pub type glGetnCompressedTexImage_t = unsafe extern "system" fn(target: TextureTarget, lod: GLint, bufSize: GLsizei, pixels: *mut void);

/// glGetnCompressedTexImageARB
/// * `target` group: TextureTarget
/// * `img` len: bufSize
pub type glGetnCompressedTexImageARB_t = unsafe extern "system" fn(target: TextureTarget, lod: GLint, bufSize: GLsizei, img: *mut void);

/// glGetnConvolutionFilter
/// * `target` group: ConvolutionTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: bufSize
pub type glGetnConvolutionFilter_t = unsafe extern "system" fn(target: ConvolutionTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, image: *mut void);

/// glGetnConvolutionFilterARB
/// * `target` group: ConvolutionTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: bufSize
pub type glGetnConvolutionFilterARB_t = unsafe extern "system" fn(target: ConvolutionTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, image: *mut void);

/// glGetnHistogram
/// * `target` group: HistogramTarget
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
pub type glGetnHistogram_t = unsafe extern "system" fn(target: HistogramTarget, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void);

/// glGetnHistogramARB
/// * `target` group: HistogramTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
pub type glGetnHistogramARB_t = unsafe extern "system" fn(target: HistogramTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void);

/// glGetnMapdv
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: COMPSIZE(bufSize)
pub type glGetnMapdv_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLdouble);

/// glGetnMapdvARB
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: bufSize / 8
pub type glGetnMapdvARB_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLdouble);

/// glGetnMapfv
/// * `target` group: MapTarget
/// * `query` group: MapQuery
pub type glGetnMapfv_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLfloat);

/// glGetnMapfvARB
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: bufSize
pub type glGetnMapfvARB_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLfloat);

/// glGetnMapiv
/// * `target` group: MapTarget
/// * `query` group: MapQuery
pub type glGetnMapiv_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLint);

/// glGetnMapivARB
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: bufSize
pub type glGetnMapivARB_t = unsafe extern "system" fn(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLint);

/// glGetnMinmax
/// * `target` group: MinmaxTarget
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
pub type glGetnMinmax_t = unsafe extern "system" fn(target: MinmaxTarget, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void);

/// glGetnMinmaxARB
/// * `target` group: MinmaxTargetEXT
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
pub type glGetnMinmaxARB_t = unsafe extern "system" fn(target: MinmaxTargetEXT, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void);

/// glGetnPixelMapfv
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(bufSize)
pub type glGetnPixelMapfv_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLfloat);

/// glGetnPixelMapfvARB
/// * `map` group: PixelMap
/// * `values` len: bufSize / 4
pub type glGetnPixelMapfvARB_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLfloat);

/// glGetnPixelMapuiv
/// * `map` group: PixelMap
pub type glGetnPixelMapuiv_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLuint);

/// glGetnPixelMapuivARB
/// * `map` group: PixelMap
/// * `values` len: bufSize
pub type glGetnPixelMapuivARB_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLuint);

/// glGetnPixelMapusv
/// * `map` group: PixelMap
pub type glGetnPixelMapusv_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLushort);

/// glGetnPixelMapusvARB
/// * `map` group: PixelMap
/// * `values` len: bufSize
pub type glGetnPixelMapusvARB_t = unsafe extern "system" fn(map: PixelMap, bufSize: GLsizei, values: *mut GLushort);

/// glGetnPolygonStipple
/// * `pattern` len: bufSize
pub type glGetnPolygonStipple_t = unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);

/// glGetnPolygonStippleARB
/// * `pattern` len: bufSize
pub type glGetnPolygonStippleARB_t = unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);

/// glGetnSeparableFilter
/// * `target` group: SeparableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: rowBufSize
/// * `column` len: columnBufSize
pub type glGetnSeparableFilter_t = unsafe extern "system" fn(target: SeparableTarget, format: PixelFormat, type_: PixelType, rowBufSize: GLsizei, row: *mut void, columnBufSize: GLsizei, column: *mut void, span: *mut void);

/// glGetnSeparableFilterARB
/// * `target` group: SeparableTargetEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: rowBufSize
/// * `column` len: columnBufSize
pub type glGetnSeparableFilterARB_t = unsafe extern "system" fn(target: SeparableTargetEXT, format: PixelFormat, type_: PixelType, rowBufSize: GLsizei, row: *mut void, columnBufSize: GLsizei, column: *mut void, span: *mut void);

/// glGetnTexImage
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: bufSize
pub type glGetnTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

/// glGetnTexImageARB
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `img` len: bufSize
pub type glGetnTexImageARB_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, img: *mut void);

/// glGetnUniformdv
/// * `program` class: program
/// * `params` len: bufSize / 8
pub type glGetnUniformdv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);

/// glGetnUniformdvARB
/// * `program` class: program
/// * `params` len: bufSize / 8
pub type glGetnUniformdvARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);

/// glGetnUniformfv
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

/// glGetnUniformfvARB
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformfvARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

/// glGetnUniformfvEXT
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformfvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

/// glGetnUniformfvKHR
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformfvKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

/// glGetnUniformi64vARB
/// * `program` class: program
/// * `params` len: bufSize / 8
pub type glGetnUniformi64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint64);

/// glGetnUniformiv
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

/// glGetnUniformivARB
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformivARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

/// glGetnUniformivEXT
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

/// glGetnUniformivKHR
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformivKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

/// glGetnUniformui64vARB
/// * `program` class: program
/// * `params` len: bufSize / 8
pub type glGetnUniformui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint64);

/// glGetnUniformuiv
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

/// glGetnUniformuivARB
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformuivARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

/// glGetnUniformuivKHR
/// * `program` class: program
/// * `params` len: bufSize / 4
pub type glGetnUniformuivKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

/// glGlobalAlphaFactorbSUN
pub type glGlobalAlphaFactorbSUN_t = unsafe extern "system" fn(factor: GLbyte);

/// glGlobalAlphaFactordSUN
pub type glGlobalAlphaFactordSUN_t = unsafe extern "system" fn(factor: GLdouble);

/// glGlobalAlphaFactorfSUN
pub type glGlobalAlphaFactorfSUN_t = unsafe extern "system" fn(factor: GLfloat);

/// glGlobalAlphaFactoriSUN
pub type glGlobalAlphaFactoriSUN_t = unsafe extern "system" fn(factor: GLint);

/// glGlobalAlphaFactorsSUN
pub type glGlobalAlphaFactorsSUN_t = unsafe extern "system" fn(factor: GLshort);

/// glGlobalAlphaFactorubSUN
pub type glGlobalAlphaFactorubSUN_t = unsafe extern "system" fn(factor: GLubyte);

/// glGlobalAlphaFactoruiSUN
pub type glGlobalAlphaFactoruiSUN_t = unsafe extern "system" fn(factor: GLuint);

/// glGlobalAlphaFactorusSUN
pub type glGlobalAlphaFactorusSUN_t = unsafe extern "system" fn(factor: GLushort);

/// glHint
/// * `target` group: HintTarget
/// * `mode` group: HintMode
pub type glHint_t = unsafe extern "system" fn(target: HintTarget, mode: HintMode);

/// glHintPGI
/// * `target` group: HintTargetPGI
pub type glHintPGI_t = unsafe extern "system" fn(target: HintTargetPGI, mode: GLint);

/// glHistogram
/// * `target` group: HistogramTargetEXT
/// * `internalformat` group: InternalFormat
/// * `sink` group: Boolean
pub type glHistogram_t = unsafe extern "system" fn(target: HistogramTargetEXT, width: GLsizei, internalformat: InternalFormat, sink: GLboolean);

/// glHistogramEXT
/// * `target` group: HistogramTargetEXT
/// * `internalformat` group: InternalFormat
/// * `sink` group: Boolean
pub type glHistogramEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT, width: GLsizei, internalformat: InternalFormat, sink: GLboolean);

/// glIglooInterfaceSGIX
/// * `params` len: COMPSIZE(pname)
pub type glIglooInterfaceSGIX_t = unsafe extern "system" fn(pname: GLenum, params: *const void);

/// glImageTransformParameterfHP
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
pub type glImageTransformParameterfHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, param: GLfloat);

/// glImageTransformParameterfvHP
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
/// * `params` len: COMPSIZE(pname)
pub type glImageTransformParameterfvHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, params: *const GLfloat);

/// glImageTransformParameteriHP
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
pub type glImageTransformParameteriHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, param: GLint);

/// glImageTransformParameterivHP
/// * `target` group: ImageTransformTargetHP
/// * `pname` group: ImageTransformPNameHP
/// * `params` len: COMPSIZE(pname)
pub type glImageTransformParameterivHP_t = unsafe extern "system" fn(target: ImageTransformTargetHP, pname: ImageTransformPNameHP, params: *const GLint);

/// glImportMemoryFdEXT
/// * `handleType` group: ExternalHandleType
pub type glImportMemoryFdEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: ExternalHandleType, fd: GLint);

/// glImportMemoryWin32HandleEXT
/// * `handleType` group: ExternalHandleType
pub type glImportMemoryWin32HandleEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: ExternalHandleType, handle: *mut void);

/// glImportMemoryWin32NameEXT
/// * `handleType` group: ExternalHandleType
pub type glImportMemoryWin32NameEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: ExternalHandleType, name: *const void);

/// glImportSemaphoreFdEXT
/// * `handleType` group: ExternalHandleType
pub type glImportSemaphoreFdEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: ExternalHandleType, fd: GLint);

/// glImportSemaphoreWin32HandleEXT
/// * `handleType` group: ExternalHandleType
pub type glImportSemaphoreWin32HandleEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: ExternalHandleType, handle: *mut void);

/// glImportSemaphoreWin32NameEXT
/// * `handleType` group: ExternalHandleType
pub type glImportSemaphoreWin32NameEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: ExternalHandleType, name: *const void);

/// glImportSyncEXT
pub type glImportSyncEXT_t = unsafe extern "system" fn(external_sync_type: GLenum, external_sync: GLintptr, flags: GLbitfield) -> GLsync;

/// glIndexFormatNV
pub type glIndexFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

/// glIndexFuncEXT
/// * `func` group: IndexFunctionEXT
/// * `ref` group: ClampedFloat32
pub type glIndexFuncEXT_t = unsafe extern "system" fn(func: IndexFunctionEXT, ref_: GLclampf);

/// glIndexMask
/// * `mask` group: MaskedColorIndexValueI
pub type glIndexMask_t = unsafe extern "system" fn(mask: GLuint);

/// glIndexMaterialEXT
/// * `face` group: MaterialFace
/// * `mode` group: IndexMaterialParameterEXT
pub type glIndexMaterialEXT_t = unsafe extern "system" fn(face: MaterialFace, mode: IndexMaterialParameterEXT);

/// glIndexPointer
/// * `type` group: IndexPointerType
/// * `pointer` len: COMPSIZE(type,stride)
pub type glIndexPointer_t = unsafe extern "system" fn(type_: IndexPointerType, stride: GLsizei, pointer: *const void);

/// glIndexPointerEXT
/// * `type` group: IndexPointerType
/// * `pointer` len: COMPSIZE(type,stride,count)
pub type glIndexPointerEXT_t = unsafe extern "system" fn(type_: IndexPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// glIndexPointerListIBM
/// * `type` group: IndexPointerType
/// * `pointer` len: COMPSIZE(type,stride)
pub type glIndexPointerListIBM_t = unsafe extern "system" fn(type_: IndexPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// glIndexd
/// * `c` group: ColorIndexValueD
pub type glIndexd_t = unsafe extern "system" fn(c: GLdouble);

/// glIndexdv
/// * `c` group: ColorIndexValueD
pub type glIndexdv_t = unsafe extern "system" fn(c: *const GLdouble);

/// glIndexf
/// * `c` group: ColorIndexValueF
pub type glIndexf_t = unsafe extern "system" fn(c: GLfloat);

/// glIndexfv
/// * `c` group: ColorIndexValueF
pub type glIndexfv_t = unsafe extern "system" fn(c: *const GLfloat);

/// glIndexi
/// * `c` group: ColorIndexValueI
pub type glIndexi_t = unsafe extern "system" fn(c: GLint);

/// glIndexiv
/// * `c` group: ColorIndexValueI
pub type glIndexiv_t = unsafe extern "system" fn(c: *const GLint);

/// glIndexs
/// * `c` group: ColorIndexValueS
pub type glIndexs_t = unsafe extern "system" fn(c: GLshort);

/// glIndexsv
/// * `c` group: ColorIndexValueS
pub type glIndexsv_t = unsafe extern "system" fn(c: *const GLshort);

/// glIndexub
/// * `c` group: ColorIndexValueUB
pub type glIndexub_t = unsafe extern "system" fn(c: GLubyte);

/// glIndexubv
/// * `c` group: ColorIndexValueUB
pub type glIndexubv_t = unsafe extern "system" fn(c: *const GLubyte);

/// glIndexxOES
pub type glIndexxOES_t = unsafe extern "system" fn(component: GLfixed);

/// glIndexxvOES
pub type glIndexxvOES_t = unsafe extern "system" fn(component: *const GLfixed);

/// glInitNames
pub type glInitNames_t = unsafe extern "system" fn();

/// glInsertComponentEXT
pub type glInsertComponentEXT_t = unsafe extern "system" fn(res: GLuint, src: GLuint, num: GLuint);

/// glInsertEventMarkerEXT
pub type glInsertEventMarkerEXT_t = unsafe extern "system" fn(length: GLsizei, marker: *const GLchar);

/// glInstrumentsBufferSGIX
/// * `buffer` len: size
pub type glInstrumentsBufferSGIX_t = unsafe extern "system" fn(size: GLsizei, buffer: *mut GLint);

/// glInterleavedArrays
/// * `format` group: InterleavedArrayFormat
/// * `pointer` len: COMPSIZE(format,stride)
pub type glInterleavedArrays_t = unsafe extern "system" fn(format: InterleavedArrayFormat, stride: GLsizei, pointer: *const void);

/// glInterpolatePathsNV
/// * `resultPath` group: Path
/// * `pathA` group: Path
/// * `pathB` group: Path
pub type glInterpolatePathsNV_t = unsafe extern "system" fn(resultPath: GLuint, pathA: GLuint, pathB: GLuint, weight: GLfloat);

/// glInvalidateBufferData
/// * `buffer` class: buffer
pub type glInvalidateBufferData_t = unsafe extern "system" fn(buffer: GLuint);

/// glInvalidateBufferSubData
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
pub type glInvalidateBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

/// glInvalidateFramebuffer
/// * `target` group: FramebufferTarget
/// * `attachments` group: InvalidateFramebufferAttachment
/// * `attachments` len: numAttachments
pub type glInvalidateFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment);

/// glInvalidateNamedFramebufferData
/// * `framebuffer` class: framebuffer
/// * `attachments` group: FramebufferAttachment
pub type glInvalidateNamedFramebufferData_t = unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment);

/// glInvalidateNamedFramebufferSubData
/// * `framebuffer` class: framebuffer
/// * `attachments` group: FramebufferAttachment
pub type glInvalidateNamedFramebufferSubData_t = unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glInvalidateSubFramebuffer
/// * `target` group: FramebufferTarget
/// * `attachments` group: InvalidateFramebufferAttachment
/// * `attachments` len: numAttachments
pub type glInvalidateSubFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glInvalidateTexImage
/// * `texture` class: texture
pub type glInvalidateTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint);

/// glInvalidateTexSubImage
/// * `texture` class: texture
pub type glInvalidateTexSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

/// glIsAsyncMarkerSGIX
pub type glIsAsyncMarkerSGIX_t = unsafe extern "system" fn(marker: GLuint) -> GLboolean;

/// glIsBuffer
/// * `buffer` class: buffer
pub type glIsBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// glIsBufferARB
/// * `buffer` class: buffer
pub type glIsBufferARB_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// glIsBufferResidentNV
pub type glIsBufferResidentNV_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;

/// glIsCommandListNV
pub type glIsCommandListNV_t = unsafe extern "system" fn(list: GLuint) -> GLboolean;

/// glIsEnabled
/// * `cap` group: EnableCap
pub type glIsEnabled_t = unsafe extern "system" fn(cap: EnableCap) -> GLboolean;

/// glIsEnabledIndexedEXT
/// * `target` group: EnableCap
pub type glIsEnabledIndexedEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// glIsEnabledi
/// * `target` group: EnableCap
pub type glIsEnabledi_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// glIsEnablediEXT
/// * `target` group: EnableCap
pub type glIsEnablediEXT_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// glIsEnablediNV
/// * `target` group: EnableCap
pub type glIsEnablediNV_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// glIsEnablediOES
/// * `target` group: EnableCap
pub type glIsEnablediOES_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

/// glIsFenceAPPLE
/// * `fence` group: FenceNV
pub type glIsFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

/// glIsFenceNV
/// * `fence` group: FenceNV
pub type glIsFenceNV_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

/// glIsFramebuffer
/// * `framebuffer` class: framebuffer
pub type glIsFramebuffer_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

/// glIsFramebufferEXT
/// * `framebuffer` class: framebuffer
pub type glIsFramebufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

/// glIsFramebufferOES
/// * `framebuffer` class: framebuffer
pub type glIsFramebufferOES_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

/// glIsImageHandleResidentARB
pub type glIsImageHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

/// glIsImageHandleResidentNV
pub type glIsImageHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

/// glIsList
/// * `list` group: List
/// * `list` class: display list
pub type glIsList_t = unsafe extern "system" fn(list: GLuint) -> GLboolean;

/// glIsMemoryObjectEXT
pub type glIsMemoryObjectEXT_t = unsafe extern "system" fn(memoryObject: GLuint) -> GLboolean;

/// glIsNameAMD
pub type glIsNameAMD_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint) -> GLboolean;

/// glIsNamedBufferResidentNV
/// * `buffer` class: buffer
pub type glIsNamedBufferResidentNV_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// glIsNamedStringARB
/// * `name` len: namelen
pub type glIsNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar) -> GLboolean;

/// glIsObjectBufferATI
/// * `buffer` class: buffer
pub type glIsObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// glIsOcclusionQueryNV
pub type glIsOcclusionQueryNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// glIsPathNV
/// * `path` group: Path
pub type glIsPathNV_t = unsafe extern "system" fn(path: GLuint) -> GLboolean;

/// glIsPointInFillPathNV
/// * `path` group: Path
/// * `mask` group: MaskedStencilValue
pub type glIsPointInFillPathNV_t = unsafe extern "system" fn(path: GLuint, mask: GLuint, x: GLfloat, y: GLfloat) -> GLboolean;

/// glIsPointInStrokePathNV
/// * `path` group: Path
pub type glIsPointInStrokePathNV_t = unsafe extern "system" fn(path: GLuint, x: GLfloat, y: GLfloat) -> GLboolean;

/// glIsProgram
/// * `program` class: program
pub type glIsProgram_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

/// glIsProgramARB
/// * `program` class: program
pub type glIsProgramARB_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

/// glIsProgramNV
/// * `id` class: program
pub type glIsProgramNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// glIsProgramPipeline
/// * `pipeline` class: program pipeline
pub type glIsProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;

/// glIsProgramPipelineEXT
/// * `pipeline` class: program pipeline
pub type glIsProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;

/// glIsQuery
/// * `id` class: query
pub type glIsQuery_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// glIsQueryARB
/// * `id` class: query
pub type glIsQueryARB_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// glIsQueryEXT
/// * `id` class: query
pub type glIsQueryEXT_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// glIsRenderbuffer
/// * `renderbuffer` class: renderbuffer
pub type glIsRenderbuffer_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

/// glIsRenderbufferEXT
/// * `renderbuffer` class: renderbuffer
pub type glIsRenderbufferEXT_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

/// glIsRenderbufferOES
/// * `renderbuffer` class: renderbuffer
pub type glIsRenderbufferOES_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

/// glIsSampler
/// * `sampler` class: sampler
pub type glIsSampler_t = unsafe extern "system" fn(sampler: GLuint) -> GLboolean;

/// glIsSemaphoreEXT
pub type glIsSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint) -> GLboolean;

/// glIsShader
/// * `shader` class: shader
pub type glIsShader_t = unsafe extern "system" fn(shader: GLuint) -> GLboolean;

/// glIsStateNV
pub type glIsStateNV_t = unsafe extern "system" fn(state: GLuint) -> GLboolean;

/// glIsSync
/// * `sync` group: sync
/// * `sync` class: sync
pub type glIsSync_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;

/// glIsSyncAPPLE
/// * `sync` class: sync
pub type glIsSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;

/// glIsTexture
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glIsTexture_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

/// glIsTextureEXT
/// * `texture` group: Texture
/// * `texture` class: texture
pub type glIsTextureEXT_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

/// glIsTextureHandleResidentARB
pub type glIsTextureHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

/// glIsTextureHandleResidentNV
pub type glIsTextureHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

/// glIsTransformFeedback
/// * `id` class: transform feedback
pub type glIsTransformFeedback_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// glIsTransformFeedbackNV
/// * `id` class: transform feedback
pub type glIsTransformFeedbackNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

/// glIsVariantEnabledEXT
/// * `cap` group: VariantCapEXT
pub type glIsVariantEnabledEXT_t = unsafe extern "system" fn(id: GLuint, cap: VariantCapEXT) -> GLboolean;

/// glIsVertexArray
/// * `array` class: vertex array
pub type glIsVertexArray_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

/// glIsVertexArrayAPPLE
/// * `array` class: vertex array
pub type glIsVertexArrayAPPLE_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

/// glIsVertexArrayOES
/// * `array` class: vertex array
pub type glIsVertexArrayOES_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

/// glIsVertexAttribEnabledAPPLE
pub type glIsVertexAttribEnabledAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum) -> GLboolean;

/// glLGPUCopyImageSubDataNVX
pub type glLGPUCopyImageSubDataNVX_t = unsafe extern "system" fn(sourceGpu: GLuint, destinationGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srxY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

/// glLGPUInterlockNVX
pub type glLGPUInterlockNVX_t = unsafe extern "system" fn();

/// glLGPUNamedBufferSubDataNVX
/// * `buffer` class: buffer
pub type glLGPUNamedBufferSubDataNVX_t = unsafe extern "system" fn(gpuMask: GLbitfield, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// glLabelObjectEXT
pub type glLabelObjectEXT_t = unsafe extern "system" fn(type_: GLenum, object: GLuint, length: GLsizei, label: *const GLchar);

/// glLightEnviSGIX
/// * `pname` group: LightEnvParameterSGIX
/// * `param` group: CheckedInt32
pub type glLightEnviSGIX_t = unsafe extern "system" fn(pname: LightEnvParameterSGIX, param: GLint);

/// glLightModelf
/// * `pname` group: LightModelParameter
pub type glLightModelf_t = unsafe extern "system" fn(pname: LightModelParameter, param: GLfloat);

/// glLightModelfv
/// * `pname` group: LightModelParameter
/// * `params` len: COMPSIZE(pname)
pub type glLightModelfv_t = unsafe extern "system" fn(pname: LightModelParameter, params: *const GLfloat);

/// glLightModeli
/// * `pname` group: LightModelParameter
pub type glLightModeli_t = unsafe extern "system" fn(pname: LightModelParameter, param: GLint);

/// glLightModeliv
/// * `pname` group: LightModelParameter
/// * `params` len: COMPSIZE(pname)
pub type glLightModeliv_t = unsafe extern "system" fn(pname: LightModelParameter, params: *const GLint);

/// glLightModelx
/// * `pname` group: LightModelParameter
pub type glLightModelx_t = unsafe extern "system" fn(pname: LightModelParameter, param: GLfixed);

/// glLightModelxOES
/// * `pname` group: LightModelParameter
pub type glLightModelxOES_t = unsafe extern "system" fn(pname: LightModelParameter, param: GLfixed);

/// glLightModelxv
/// * `pname` group: LightModelParameter
/// * `param` len: COMPSIZE(pname)
pub type glLightModelxv_t = unsafe extern "system" fn(pname: LightModelParameter, param: *const GLfixed);

/// glLightModelxvOES
/// * `pname` group: LightModelParameter
/// * `param` len: COMPSIZE(pname)
pub type glLightModelxvOES_t = unsafe extern "system" fn(pname: LightModelParameter, param: *const GLfixed);

/// glLightf
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `param` group: CheckedFloat32
pub type glLightf_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, param: GLfloat);

/// glLightfv
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glLightfv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *const GLfloat);

/// glLighti
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `param` group: CheckedInt32
pub type glLighti_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, param: GLint);

/// glLightiv
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glLightiv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *const GLint);

/// glLightx
/// * `light` group: LightName
/// * `pname` group: LightParameter
pub type glLightx_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, param: GLfixed);

/// glLightxOES
/// * `light` group: LightName
/// * `pname` group: LightParameter
pub type glLightxOES_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, param: GLfixed);

/// glLightxv
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glLightxv_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *const GLfixed);

/// glLightxvOES
/// * `light` group: LightName
/// * `pname` group: LightParameter
/// * `params` len: COMPSIZE(pname)
pub type glLightxvOES_t = unsafe extern "system" fn(light: LightName, pname: LightParameter, params: *const GLfixed);

/// glLineStipple
/// * `factor` group: CheckedInt32
/// * `pattern` group: LineStipple
pub type glLineStipple_t = unsafe extern "system" fn(factor: GLint, pattern: GLushort);

/// glLineWidth
/// * `width` group: CheckedFloat32
pub type glLineWidth_t = unsafe extern "system" fn(width: GLfloat);

/// glLineWidthx
pub type glLineWidthx_t = unsafe extern "system" fn(width: GLfixed);

/// glLineWidthxOES
pub type glLineWidthxOES_t = unsafe extern "system" fn(width: GLfixed);

/// glLinkProgram
/// * `program` class: program
pub type glLinkProgram_t = unsafe extern "system" fn(program: GLuint);

/// glLinkProgramARB
/// * `programObj` group: handleARB
pub type glLinkProgramARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

/// glListBase
/// * `base` group: List
pub type glListBase_t = unsafe extern "system" fn(base: GLuint);

/// glListDrawCommandsStatesClientNV
/// * `indirects` len: count
/// * `sizes` len: count
/// * `states` len: count
/// * `fbos` len: count
pub type glListDrawCommandsStatesClientNV_t = unsafe extern "system" fn(list: GLuint, segment: GLuint, indirects: *const *mut void, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

/// glListParameterfSGIX
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `param` group: CheckedFloat32
pub type glListParameterfSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, param: GLfloat);

/// glListParameterfvSGIX
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glListParameterfvSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, params: *const GLfloat);

/// glListParameteriSGIX
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `param` group: CheckedInt32
pub type glListParameteriSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, param: GLint);

/// glListParameterivSGIX
/// * `list` group: List
/// * `pname` group: ListParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glListParameterivSGIX_t = unsafe extern "system" fn(list: GLuint, pname: ListParameterName, params: *const GLint);

/// glLoadIdentity
pub type glLoadIdentity_t = unsafe extern "system" fn();

/// glLoadIdentityDeformationMapSGIX
/// * `mask` group: FfdMaskSGIX
pub type glLoadIdentityDeformationMapSGIX_t = unsafe extern "system" fn(mask: GLbitfield);

/// glLoadMatrixd
pub type glLoadMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// glLoadMatrixf
pub type glLoadMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// glLoadMatrixx
pub type glLoadMatrixx_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// glLoadMatrixxOES
pub type glLoadMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// glLoadName
/// * `name` group: SelectName
pub type glLoadName_t = unsafe extern "system" fn(name: GLuint);

/// glLoadPaletteFromModelViewMatrixOES
pub type glLoadPaletteFromModelViewMatrixOES_t = unsafe extern "system" fn();

/// glLoadProgramNV
/// * `target` group: VertexAttribEnumNV
/// * `program` len: len
pub type glLoadProgramNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, id: GLuint, len: GLsizei, program: *const GLubyte);

/// glLoadTransposeMatrixd
pub type glLoadTransposeMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// glLoadTransposeMatrixdARB
pub type glLoadTransposeMatrixdARB_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// glLoadTransposeMatrixf
pub type glLoadTransposeMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// glLoadTransposeMatrixfARB
pub type glLoadTransposeMatrixfARB_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// glLoadTransposeMatrixxOES
pub type glLoadTransposeMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// glLockArraysEXT
pub type glLockArraysEXT_t = unsafe extern "system" fn(first: GLint, count: GLsizei);

/// glLogicOp
/// * `opcode` group: LogicOp
pub type glLogicOp_t = unsafe extern "system" fn(opcode: LogicOp);

/// glMakeBufferNonResidentNV
pub type glMakeBufferNonResidentNV_t = unsafe extern "system" fn(target: GLenum);

/// glMakeBufferResidentNV
pub type glMakeBufferResidentNV_t = unsafe extern "system" fn(target: GLenum, access: GLenum);

/// glMakeImageHandleNonResidentARB
pub type glMakeImageHandleNonResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

/// glMakeImageHandleNonResidentNV
pub type glMakeImageHandleNonResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

/// glMakeImageHandleResidentARB
pub type glMakeImageHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64, access: GLenum);

/// glMakeImageHandleResidentNV
pub type glMakeImageHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64, access: GLenum);

/// glMakeNamedBufferNonResidentNV
/// * `buffer` class: buffer
pub type glMakeNamedBufferNonResidentNV_t = unsafe extern "system" fn(buffer: GLuint);

/// glMakeNamedBufferResidentNV
/// * `buffer` class: buffer
pub type glMakeNamedBufferResidentNV_t = unsafe extern "system" fn(buffer: GLuint, access: GLenum);

/// glMakeTextureHandleNonResidentARB
pub type glMakeTextureHandleNonResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

/// glMakeTextureHandleNonResidentNV
pub type glMakeTextureHandleNonResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

/// glMakeTextureHandleResidentARB
pub type glMakeTextureHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

/// glMakeTextureHandleResidentNV
pub type glMakeTextureHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

/// glMap1d
/// * `target` group: MapTarget
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `order` group: CheckedInt32
/// * `points` group: CoordD
/// * `points` len: COMPSIZE(target,stride,order)
pub type glMap1d_t = unsafe extern "system" fn(target: MapTarget, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble);

/// glMap1f
/// * `target` group: MapTarget
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `order` group: CheckedInt32
/// * `points` group: CoordF
/// * `points` len: COMPSIZE(target,stride,order)
pub type glMap1f_t = unsafe extern "system" fn(target: MapTarget, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat);

/// glMap1xOES
/// * `target` group: MapTarget
pub type glMap1xOES_t = unsafe extern "system" fn(target: MapTarget, u1: GLfixed, u2: GLfixed, stride: GLint, order: GLint, points: GLfixed);

/// glMap2d
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

/// glMap2f
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

/// glMap2xOES
/// * `target` group: MapTarget
pub type glMap2xOES_t = unsafe extern "system" fn(target: MapTarget, u1: GLfixed, u2: GLfixed, ustride: GLint, uorder: GLint, v1: GLfixed, v2: GLfixed, vstride: GLint, vorder: GLint, points: GLfixed);

/// glMapBuffer
/// * `target` group: BufferTargetARB
/// * `access` group: BufferAccessARB
pub type glMapBuffer_t = unsafe extern "system" fn(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;

/// glMapBufferARB
/// * `target` group: BufferTargetARB
/// * `access` group: BufferAccessARB
pub type glMapBufferARB_t = unsafe extern "system" fn(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;

/// glMapBufferOES
/// * `target` group: BufferTargetARB
/// * `access` group: BufferAccessARB
pub type glMapBufferOES_t = unsafe extern "system" fn(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;

/// glMapBufferRange
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
/// * `access` group: MapBufferAccessMask
pub type glMapBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

/// glMapBufferRangeEXT
/// * `target` group: BufferTargetARB
/// * `access` group: MapBufferAccessMask
pub type glMapBufferRangeEXT_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

/// glMapControlPointsNV
/// * `target` group: EvalTargetNV
/// * `type` group: MapTypeNV
/// * `uorder` group: CheckedInt32
/// * `vorder` group: CheckedInt32
/// * `packed` group: Boolean
/// * `points` len: COMPSIZE(target,uorder,vorder)
pub type glMapControlPointsNV_t = unsafe extern "system" fn(target: EvalTargetNV, index: GLuint, type_: MapTypeNV, ustride: GLsizei, vstride: GLsizei, uorder: GLint, vorder: GLint, packed: GLboolean, points: *const void);

/// glMapGrid1d
/// * `u1` group: CoordD
/// * `u2` group: CoordD
pub type glMapGrid1d_t = unsafe extern "system" fn(un: GLint, u1: GLdouble, u2: GLdouble);

/// glMapGrid1f
/// * `u1` group: CoordF
/// * `u2` group: CoordF
pub type glMapGrid1f_t = unsafe extern "system" fn(un: GLint, u1: GLfloat, u2: GLfloat);

/// glMapGrid1xOES
pub type glMapGrid1xOES_t = unsafe extern "system" fn(n: GLint, u1: GLfixed, u2: GLfixed);

/// glMapGrid2d
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `v1` group: CoordD
/// * `v2` group: CoordD
pub type glMapGrid2d_t = unsafe extern "system" fn(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble);

/// glMapGrid2f
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `v1` group: CoordF
/// * `v2` group: CoordF
pub type glMapGrid2f_t = unsafe extern "system" fn(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat);

/// glMapGrid2xOES
pub type glMapGrid2xOES_t = unsafe extern "system" fn(n: GLint, u1: GLfixed, u2: GLfixed, v1: GLfixed, v2: GLfixed);

/// glMapNamedBuffer
/// * `buffer` class: buffer
/// * `access` group: BufferAccessARB
pub type glMapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint, access: BufferAccessARB) -> *mut void;

/// glMapNamedBufferEXT
/// * `buffer` class: buffer
/// * `access` group: BufferAccessARB
pub type glMapNamedBufferEXT_t = unsafe extern "system" fn(buffer: GLuint, access: BufferAccessARB) -> *mut void;

/// glMapNamedBufferRange
/// * `buffer` class: buffer
/// * `length` group: BufferSize
/// * `access` group: MapBufferAccessMask
pub type glMapNamedBufferRange_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

/// glMapNamedBufferRangeEXT
/// * `buffer` class: buffer
/// * `access` group: MapBufferAccessMask
pub type glMapNamedBufferRangeEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

/// glMapObjectBufferATI
/// * `buffer` class: buffer
pub type glMapObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint) -> *mut void;

/// glMapParameterfvNV
/// * `target` group: EvalTargetNV
/// * `pname` group: MapParameterNV
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(target,pname)
pub type glMapParameterfvNV_t = unsafe extern "system" fn(target: EvalTargetNV, pname: MapParameterNV, params: *const GLfloat);

/// glMapParameterivNV
/// * `target` group: EvalTargetNV
/// * `pname` group: MapParameterNV
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(target,pname)
pub type glMapParameterivNV_t = unsafe extern "system" fn(target: EvalTargetNV, pname: MapParameterNV, params: *const GLint);

/// glMapTexture2DINTEL
/// * `texture` class: texture
pub type glMapTexture2DINTEL_t = unsafe extern "system" fn(texture: GLuint, level: GLint, access: GLbitfield, stride: *mut GLint, layout: *mut GLenum) -> *mut void;

/// glMapVertexAttrib1dAPPLE
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `order` group: CheckedInt32
/// * `points` group: CoordD
/// * `points` len: COMPSIZE(size,stride,order)
pub type glMapVertexAttrib1dAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble);

/// glMapVertexAttrib1fAPPLE
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `order` group: CheckedInt32
/// * `points` group: CoordF
/// * `points` len: COMPSIZE(size,stride,order)
pub type glMapVertexAttrib1fAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat);

/// glMapVertexAttrib2dAPPLE
/// * `u1` group: CoordD
/// * `u2` group: CoordD
/// * `uorder` group: CheckedInt32
/// * `v1` group: CoordD
/// * `v2` group: CoordD
/// * `vorder` group: CheckedInt32
/// * `points` group: CoordD
/// * `points` len: COMPSIZE(size,ustride,uorder,vstride,vorder)
pub type glMapVertexAttrib2dAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble);

/// glMapVertexAttrib2fAPPLE
/// * `u1` group: CoordF
/// * `u2` group: CoordF
/// * `uorder` group: CheckedInt32
/// * `v1` group: CoordF
/// * `v2` group: CoordF
/// * `vorder` group: CheckedInt32
/// * `points` group: CoordF
/// * `points` len: COMPSIZE(size,ustride,uorder,vstride,vorder)
pub type glMapVertexAttrib2fAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat);

/// glMaterialf
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` group: CheckedFloat32
pub type glMaterialf_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfloat);

/// glMaterialfv
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glMaterialfv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *const GLfloat);

/// glMateriali
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` group: CheckedInt32
pub type glMateriali_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLint);

/// glMaterialiv
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMaterialiv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, params: *const GLint);

/// glMaterialx
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
pub type glMaterialx_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfixed);

/// glMaterialxOES
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
pub type glMaterialxOES_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: GLfixed);

/// glMaterialxv
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` len: COMPSIZE(pname)
pub type glMaterialxv_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: *const GLfixed);

/// glMaterialxvOES
/// * `face` group: MaterialFace
/// * `pname` group: MaterialParameter
/// * `param` len: COMPSIZE(pname)
pub type glMaterialxvOES_t = unsafe extern "system" fn(face: MaterialFace, pname: MaterialParameter, param: *const GLfixed);

/// glMatrixFrustumEXT
/// * `mode` group: MatrixMode
pub type glMatrixFrustumEXT_t = unsafe extern "system" fn(mode: MatrixMode, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

/// glMatrixIndexPointerARB
/// * `type` group: MatrixIndexPointerTypeARB
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glMatrixIndexPointerARB_t = unsafe extern "system" fn(size: GLint, type_: MatrixIndexPointerTypeARB, stride: GLsizei, pointer: *const void);

/// glMatrixIndexPointerOES
/// * `type` group: MatrixIndexPointerTypeARB
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glMatrixIndexPointerOES_t = unsafe extern "system" fn(size: GLint, type_: MatrixIndexPointerTypeARB, stride: GLsizei, pointer: *const void);

/// glMatrixIndexubvARB
/// * `indices` len: size
pub type glMatrixIndexubvARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLubyte);

/// glMatrixIndexuivARB
/// * `indices` len: size
pub type glMatrixIndexuivARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLuint);

/// glMatrixIndexusvARB
/// * `indices` len: size
pub type glMatrixIndexusvARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLushort);

/// glMatrixLoad3x2fNV
pub type glMatrixLoad3x2fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// glMatrixLoad3x3fNV
pub type glMatrixLoad3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// glMatrixLoadIdentityEXT
/// * `mode` group: MatrixMode
pub type glMatrixLoadIdentityEXT_t = unsafe extern "system" fn(mode: MatrixMode);

/// glMatrixLoadTranspose3x3fNV
pub type glMatrixLoadTranspose3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// glMatrixLoadTransposedEXT
/// * `mode` group: MatrixMode
pub type glMatrixLoadTransposedEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLdouble; 16]);

/// glMatrixLoadTransposefEXT
/// * `mode` group: MatrixMode
pub type glMatrixLoadTransposefEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLfloat; 16]);

/// glMatrixLoaddEXT
/// * `mode` group: MatrixMode
pub type glMatrixLoaddEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLdouble; 16]);

/// glMatrixLoadfEXT
/// * `mode` group: MatrixMode
pub type glMatrixLoadfEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLfloat; 16]);

/// glMatrixMode
/// * `mode` group: MatrixMode
pub type glMatrixMode_t = unsafe extern "system" fn(mode: MatrixMode);

/// glMatrixMult3x2fNV
pub type glMatrixMult3x2fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// glMatrixMult3x3fNV
pub type glMatrixMult3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// glMatrixMultTranspose3x3fNV
pub type glMatrixMultTranspose3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

/// glMatrixMultTransposedEXT
/// * `mode` group: MatrixMode
pub type glMatrixMultTransposedEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLdouble; 16]);

/// glMatrixMultTransposefEXT
/// * `mode` group: MatrixMode
pub type glMatrixMultTransposefEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLfloat; 16]);

/// glMatrixMultdEXT
/// * `mode` group: MatrixMode
pub type glMatrixMultdEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLdouble; 16]);

/// glMatrixMultfEXT
/// * `mode` group: MatrixMode
pub type glMatrixMultfEXT_t = unsafe extern "system" fn(mode: MatrixMode, m: *const [GLfloat; 16]);

/// glMatrixOrthoEXT
/// * `mode` group: MatrixMode
pub type glMatrixOrthoEXT_t = unsafe extern "system" fn(mode: MatrixMode, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

/// glMatrixPopEXT
/// * `mode` group: MatrixMode
pub type glMatrixPopEXT_t = unsafe extern "system" fn(mode: MatrixMode);

/// glMatrixPushEXT
/// * `mode` group: MatrixMode
pub type glMatrixPushEXT_t = unsafe extern "system" fn(mode: MatrixMode);

/// glMatrixRotatedEXT
/// * `mode` group: MatrixMode
pub type glMatrixRotatedEXT_t = unsafe extern "system" fn(mode: MatrixMode, angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);

/// glMatrixRotatefEXT
/// * `mode` group: MatrixMode
pub type glMatrixRotatefEXT_t = unsafe extern "system" fn(mode: MatrixMode, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glMatrixScaledEXT
/// * `mode` group: MatrixMode
pub type glMatrixScaledEXT_t = unsafe extern "system" fn(mode: MatrixMode, x: GLdouble, y: GLdouble, z: GLdouble);

/// glMatrixScalefEXT
/// * `mode` group: MatrixMode
pub type glMatrixScalefEXT_t = unsafe extern "system" fn(mode: MatrixMode, x: GLfloat, y: GLfloat, z: GLfloat);

/// glMatrixTranslatedEXT
/// * `mode` group: MatrixMode
pub type glMatrixTranslatedEXT_t = unsafe extern "system" fn(mode: MatrixMode, x: GLdouble, y: GLdouble, z: GLdouble);

/// glMatrixTranslatefEXT
/// * `mode` group: MatrixMode
pub type glMatrixTranslatefEXT_t = unsafe extern "system" fn(mode: MatrixMode, x: GLfloat, y: GLfloat, z: GLfloat);

/// glMaxShaderCompilerThreadsARB
pub type glMaxShaderCompilerThreadsARB_t = unsafe extern "system" fn(count: GLuint);

/// glMaxShaderCompilerThreadsKHR
pub type glMaxShaderCompilerThreadsKHR_t = unsafe extern "system" fn(count: GLuint);

/// glMemoryBarrier
/// * `barriers` group: MemoryBarrierMask
pub type glMemoryBarrier_t = unsafe extern "system" fn(barriers: GLbitfield);

/// glMemoryBarrierByRegion
/// * `barriers` group: MemoryBarrierMask
pub type glMemoryBarrierByRegion_t = unsafe extern "system" fn(barriers: GLbitfield);

/// glMemoryBarrierEXT
/// * `barriers` group: MemoryBarrierMask
pub type glMemoryBarrierEXT_t = unsafe extern "system" fn(barriers: GLbitfield);

/// glMemoryObjectParameterivEXT
/// * `pname` group: MemoryObjectParameterName
pub type glMemoryObjectParameterivEXT_t = unsafe extern "system" fn(memoryObject: GLuint, pname: MemoryObjectParameterName, params: *const GLint);

/// glMinSampleShading
/// * `value` group: ColorF
pub type glMinSampleShading_t = unsafe extern "system" fn(value: GLfloat);

/// glMinSampleShadingARB
/// * `value` group: ColorF
pub type glMinSampleShadingARB_t = unsafe extern "system" fn(value: GLfloat);

/// glMinSampleShadingOES
/// * `value` group: ColorF
pub type glMinSampleShadingOES_t = unsafe extern "system" fn(value: GLfloat);

/// glMinmax
/// * `target` group: MinmaxTargetEXT
/// * `internalformat` group: InternalFormat
/// * `sink` group: Boolean
pub type glMinmax_t = unsafe extern "system" fn(target: MinmaxTargetEXT, internalformat: InternalFormat, sink: GLboolean);

/// glMinmaxEXT
/// * `target` group: MinmaxTargetEXT
/// * `internalformat` group: InternalFormat
/// * `sink` group: Boolean
pub type glMinmaxEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT, internalformat: InternalFormat, sink: GLboolean);

/// glMultMatrixd
pub type glMultMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// glMultMatrixf
pub type glMultMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// glMultMatrixx
pub type glMultMatrixx_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// glMultMatrixxOES
pub type glMultMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// glMultTransposeMatrixd
pub type glMultTransposeMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// glMultTransposeMatrixdARB
pub type glMultTransposeMatrixdARB_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

/// glMultTransposeMatrixf
pub type glMultTransposeMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// glMultTransposeMatrixfARB
pub type glMultTransposeMatrixfARB_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

/// glMultTransposeMatrixxOES
pub type glMultTransposeMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

/// glMultiDrawArrays
/// * `mode` group: PrimitiveType
/// * `first` len: COMPSIZE(drawcount)
/// * `count` len: COMPSIZE(drawcount)
pub type glMultiDrawArrays_t = unsafe extern "system" fn(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);

/// glMultiDrawArraysEXT
/// * `mode` group: PrimitiveType
/// * `first` len: COMPSIZE(primcount)
/// * `count` len: COMPSIZE(primcount)
pub type glMultiDrawArraysEXT_t = unsafe extern "system" fn(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

/// glMultiDrawArraysIndirect
/// * `mode` group: PrimitiveType
/// * `indirect` len: COMPSIZE(drawcount,stride)
pub type glMultiDrawArraysIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

/// glMultiDrawArraysIndirectAMD
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectAMD_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, primcount: GLsizei, stride: GLsizei);

/// glMultiDrawArraysIndirectBindlessCountNV
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectBindlessCountNV_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawCount: GLsizei, maxDrawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

/// glMultiDrawArraysIndirectBindlessNV
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectBindlessNV_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

/// glMultiDrawArraysIndirectCount
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectCount_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// glMultiDrawArraysIndirectCountARB
/// * `mode` group: PrimitiveType
pub type glMultiDrawArraysIndirectCountARB_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// glMultiDrawArraysIndirectEXT
/// * `mode` group: PrimitiveType
/// * `indirect` len: COMPSIZE(drawcount,stride)
pub type glMultiDrawArraysIndirectEXT_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

/// glMultiDrawElementArrayAPPLE
/// * `mode` group: PrimitiveType
/// * `first` len: primcount
/// * `count` len: primcount
pub type glMultiDrawElementArrayAPPLE_t = unsafe extern "system" fn(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

/// glMultiDrawElements
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
pub type glMultiDrawElements_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei);

/// glMultiDrawElementsBaseVertex
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
/// * `basevertex` len: COMPSIZE(drawcount)
pub type glMultiDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);

/// glMultiDrawElementsBaseVertexEXT
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
/// * `basevertex` len: COMPSIZE(drawcount)
pub type glMultiDrawElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);

/// glMultiDrawElementsEXT
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(primcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(primcount)
pub type glMultiDrawElementsEXT_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, primcount: GLsizei);

/// glMultiDrawElementsIndirect
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indirect` len: COMPSIZE(drawcount,stride)
pub type glMultiDrawElementsIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

/// glMultiDrawElementsIndirectAMD
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectAMD_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, primcount: GLsizei, stride: GLsizei);

/// glMultiDrawElementsIndirectBindlessCountNV
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectBindlessCountNV_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawCount: GLsizei, maxDrawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

/// glMultiDrawElementsIndirectBindlessNV
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectBindlessNV_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

/// glMultiDrawElementsIndirectCount
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectCount_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// glMultiDrawElementsIndirectCountARB
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
pub type glMultiDrawElementsIndirectCountARB_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// glMultiDrawElementsIndirectEXT
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indirect` len: COMPSIZE(drawcount,stride)
pub type glMultiDrawElementsIndirectEXT_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

/// glMultiDrawMeshTasksIndirectCountNV
pub type glMultiDrawMeshTasksIndirectCountNV_t = unsafe extern "system" fn(indirect: GLintptr, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

/// glMultiDrawMeshTasksIndirectNV
pub type glMultiDrawMeshTasksIndirectNV_t = unsafe extern "system" fn(indirect: GLintptr, drawcount: GLsizei, stride: GLsizei);

/// glMultiDrawRangeElementArrayAPPLE
/// * `mode` group: PrimitiveType
/// * `first` len: primcount
/// * `count` len: primcount
pub type glMultiDrawRangeElementArrayAPPLE_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

/// glMultiModeDrawArraysIBM
/// * `mode` group: PrimitiveType
/// * `mode` len: COMPSIZE(primcount)
/// * `first` len: COMPSIZE(primcount)
/// * `count` len: COMPSIZE(primcount)
pub type glMultiModeDrawArraysIBM_t = unsafe extern "system" fn(mode: *const PrimitiveType, first: *const GLint, count: *const GLsizei, primcount: GLsizei, modestride: GLint);

/// glMultiModeDrawElementsIBM
/// * `mode` group: PrimitiveType
/// * `mode` len: COMPSIZE(primcount)
/// * `count` len: COMPSIZE(primcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(primcount)
pub type glMultiModeDrawElementsIBM_t = unsafe extern "system" fn(mode: *const PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, primcount: GLsizei, modestride: GLint);

/// glMultiTexBufferEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `buffer` class: buffer
pub type glMultiTexBufferEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, internalformat: GLenum, buffer: GLuint);

/// glMultiTexCoord1bOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord1bOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLbyte);

/// glMultiTexCoord1bvOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord1bvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const GLbyte);

/// glMultiTexCoord1d
/// * `target` group: TextureUnit
/// * `s` group: CoordD
pub type glMultiTexCoord1d_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble);

/// glMultiTexCoord1dARB
/// * `target` group: TextureUnit
/// * `s` group: CoordD
pub type glMultiTexCoord1dARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble);

/// glMultiTexCoord1dv
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord1dv_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLdouble);

/// glMultiTexCoord1dvARB
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord1dvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLdouble);

/// glMultiTexCoord1f
/// * `target` group: TextureUnit
/// * `s` group: CoordF
pub type glMultiTexCoord1f_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat);

/// glMultiTexCoord1fARB
/// * `target` group: TextureUnit
/// * `s` group: CoordF
pub type glMultiTexCoord1fARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat);

/// glMultiTexCoord1fv
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord1fv_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLfloat);

/// glMultiTexCoord1fvARB
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord1fvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLfloat);

/// glMultiTexCoord1hNV
/// * `target` group: TextureUnit
/// * `s` group: Half16NV
pub type glMultiTexCoord1hNV_t = unsafe extern "system" fn(target: TextureUnit, s: GLhalfNV);

/// glMultiTexCoord1hvNV
/// * `target` group: TextureUnit
/// * `v` group: Half16NV
pub type glMultiTexCoord1hvNV_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLhalfNV);

/// glMultiTexCoord1i
/// * `target` group: TextureUnit
/// * `s` group: CoordI
pub type glMultiTexCoord1i_t = unsafe extern "system" fn(target: TextureUnit, s: GLint);

/// glMultiTexCoord1iARB
/// * `target` group: TextureUnit
/// * `s` group: CoordI
pub type glMultiTexCoord1iARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLint);

/// glMultiTexCoord1iv
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord1iv_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLint);

/// glMultiTexCoord1ivARB
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord1ivARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLint);

/// glMultiTexCoord1s
/// * `target` group: TextureUnit
/// * `s` group: CoordS
pub type glMultiTexCoord1s_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort);

/// glMultiTexCoord1sARB
/// * `target` group: TextureUnit
/// * `s` group: CoordS
pub type glMultiTexCoord1sARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort);

/// glMultiTexCoord1sv
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord1sv_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLshort);

/// glMultiTexCoord1svARB
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord1svARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const GLshort);

/// glMultiTexCoord1xOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord1xOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed);

/// glMultiTexCoord1xvOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord1xvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const GLfixed);

/// glMultiTexCoord2bOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord2bOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLbyte, t: GLbyte);

/// glMultiTexCoord2bvOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord2bvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLbyte; 2]);

/// glMultiTexCoord2d
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
pub type glMultiTexCoord2d_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble);

/// glMultiTexCoord2dARB
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
pub type glMultiTexCoord2dARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble);

/// glMultiTexCoord2dv
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord2dv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 2]);

/// glMultiTexCoord2dvARB
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord2dvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 2]);

/// glMultiTexCoord2f
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
pub type glMultiTexCoord2f_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat);

/// glMultiTexCoord2fARB
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
pub type glMultiTexCoord2fARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat);

/// glMultiTexCoord2fv
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord2fv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 2]);

/// glMultiTexCoord2fvARB
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord2fvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 2]);

/// glMultiTexCoord2hNV
/// * `target` group: TextureUnit
/// * `s` group: Half16NV
/// * `t` group: Half16NV
pub type glMultiTexCoord2hNV_t = unsafe extern "system" fn(target: TextureUnit, s: GLhalfNV, t: GLhalfNV);

/// glMultiTexCoord2hvNV
/// * `target` group: TextureUnit
/// * `v` group: Half16NV
pub type glMultiTexCoord2hvNV_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLhalfNV; 2]);

/// glMultiTexCoord2i
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
pub type glMultiTexCoord2i_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint);

/// glMultiTexCoord2iARB
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
pub type glMultiTexCoord2iARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint);

/// glMultiTexCoord2iv
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord2iv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 2]);

/// glMultiTexCoord2ivARB
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord2ivARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 2]);

/// glMultiTexCoord2s
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
pub type glMultiTexCoord2s_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort);

/// glMultiTexCoord2sARB
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
pub type glMultiTexCoord2sARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort);

/// glMultiTexCoord2sv
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord2sv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 2]);

/// glMultiTexCoord2svARB
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord2svARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 2]);

/// glMultiTexCoord2xOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord2xOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed, t: GLfixed);

/// glMultiTexCoord2xvOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord2xvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLfixed; 2]);

/// glMultiTexCoord3bOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord3bOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLbyte, t: GLbyte, r: GLbyte);

/// glMultiTexCoord3bvOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord3bvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLbyte; 3]);

/// glMultiTexCoord3d
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
pub type glMultiTexCoord3d_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble, r: GLdouble);

/// glMultiTexCoord3dARB
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
pub type glMultiTexCoord3dARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble, r: GLdouble);

/// glMultiTexCoord3dv
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord3dv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 3]);

/// glMultiTexCoord3dvARB
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord3dvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 3]);

/// glMultiTexCoord3f
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
pub type glMultiTexCoord3f_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat, r: GLfloat);

/// glMultiTexCoord3fARB
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
pub type glMultiTexCoord3fARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat, r: GLfloat);

/// glMultiTexCoord3fv
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord3fv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 3]);

/// glMultiTexCoord3fvARB
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord3fvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 3]);

/// glMultiTexCoord3hNV
/// * `target` group: TextureUnit
/// * `s` group: Half16NV
/// * `t` group: Half16NV
/// * `r` group: Half16NV
pub type glMultiTexCoord3hNV_t = unsafe extern "system" fn(target: TextureUnit, s: GLhalfNV, t: GLhalfNV, r: GLhalfNV);

/// glMultiTexCoord3hvNV
/// * `target` group: TextureUnit
/// * `v` group: Half16NV
pub type glMultiTexCoord3hvNV_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLhalfNV; 3]);

/// glMultiTexCoord3i
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
pub type glMultiTexCoord3i_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint, r: GLint);

/// glMultiTexCoord3iARB
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
pub type glMultiTexCoord3iARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint, r: GLint);

/// glMultiTexCoord3iv
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord3iv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 3]);

/// glMultiTexCoord3ivARB
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord3ivARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 3]);

/// glMultiTexCoord3s
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
pub type glMultiTexCoord3s_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort, r: GLshort);

/// glMultiTexCoord3sARB
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
pub type glMultiTexCoord3sARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort, r: GLshort);

/// glMultiTexCoord3sv
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord3sv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 3]);

/// glMultiTexCoord3svARB
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord3svARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 3]);

/// glMultiTexCoord3xOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord3xOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed, t: GLfixed, r: GLfixed);

/// glMultiTexCoord3xvOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord3xvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLfixed; 3]);

/// glMultiTexCoord4bOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4bOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLbyte, t: GLbyte, r: GLbyte, q: GLbyte);

/// glMultiTexCoord4bvOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4bvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLbyte; 4]);

/// glMultiTexCoord4d
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
/// * `q` group: CoordD
pub type glMultiTexCoord4d_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

/// glMultiTexCoord4dARB
/// * `target` group: TextureUnit
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
/// * `q` group: CoordD
pub type glMultiTexCoord4dARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

/// glMultiTexCoord4dv
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord4dv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 4]);

/// glMultiTexCoord4dvARB
/// * `target` group: TextureUnit
/// * `v` group: CoordD
pub type glMultiTexCoord4dvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLdouble; 4]);

/// glMultiTexCoord4f
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
/// * `q` group: CoordF
pub type glMultiTexCoord4f_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

/// glMultiTexCoord4fARB
/// * `target` group: TextureUnit
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
/// * `q` group: CoordF
pub type glMultiTexCoord4fARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

/// glMultiTexCoord4fv
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord4fv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 4]);

/// glMultiTexCoord4fvARB
/// * `target` group: TextureUnit
/// * `v` group: CoordF
pub type glMultiTexCoord4fvARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLfloat; 4]);

/// glMultiTexCoord4hNV
/// * `target` group: TextureUnit
/// * `s` group: Half16NV
/// * `t` group: Half16NV
/// * `r` group: Half16NV
/// * `q` group: Half16NV
pub type glMultiTexCoord4hNV_t = unsafe extern "system" fn(target: TextureUnit, s: GLhalfNV, t: GLhalfNV, r: GLhalfNV, q: GLhalfNV);

/// glMultiTexCoord4hvNV
/// * `target` group: TextureUnit
/// * `v` group: Half16NV
pub type glMultiTexCoord4hvNV_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLhalfNV; 4]);

/// glMultiTexCoord4i
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
/// * `q` group: CoordI
pub type glMultiTexCoord4i_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint, r: GLint, q: GLint);

/// glMultiTexCoord4iARB
/// * `target` group: TextureUnit
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
/// * `q` group: CoordI
pub type glMultiTexCoord4iARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLint, t: GLint, r: GLint, q: GLint);

/// glMultiTexCoord4iv
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord4iv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 4]);

/// glMultiTexCoord4ivARB
/// * `target` group: TextureUnit
/// * `v` group: CoordI
pub type glMultiTexCoord4ivARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLint; 4]);

/// glMultiTexCoord4s
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
/// * `q` group: CoordS
pub type glMultiTexCoord4s_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort, r: GLshort, q: GLshort);

/// glMultiTexCoord4sARB
/// * `target` group: TextureUnit
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
/// * `q` group: CoordS
pub type glMultiTexCoord4sARB_t = unsafe extern "system" fn(target: TextureUnit, s: GLshort, t: GLshort, r: GLshort, q: GLshort);

/// glMultiTexCoord4sv
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord4sv_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 4]);

/// glMultiTexCoord4svARB
/// * `target` group: TextureUnit
/// * `v` group: CoordS
pub type glMultiTexCoord4svARB_t = unsafe extern "system" fn(target: TextureUnit, v: *const [GLshort; 4]);

/// glMultiTexCoord4x
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4x_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

/// glMultiTexCoord4xOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4xOES_t = unsafe extern "system" fn(texture: TextureUnit, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

/// glMultiTexCoord4xvOES
/// * `texture` group: TextureUnit
pub type glMultiTexCoord4xvOES_t = unsafe extern "system" fn(texture: TextureUnit, coords: *const [GLfixed; 4]);

/// glMultiTexCoordP1ui
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP1ui_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint);

/// glMultiTexCoordP1uiv
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP1uiv_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint);

/// glMultiTexCoordP2ui
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP2ui_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint);

/// glMultiTexCoordP2uiv
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP2uiv_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint);

/// glMultiTexCoordP3ui
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP3ui_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint);

/// glMultiTexCoordP3uiv
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP3uiv_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint);

/// glMultiTexCoordP4ui
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP4ui_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint);

/// glMultiTexCoordP4uiv
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
pub type glMultiTexCoordP4uiv_t = unsafe extern "system" fn(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint);

/// glMultiTexCoordPointerEXT
/// * `texunit` group: TextureUnit
/// * `type` group: TexCoordPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glMultiTexCoordPointerEXT_t = unsafe extern "system" fn(texunit: TextureUnit, size: GLint, type_: TexCoordPointerType, stride: GLsizei, pointer: *const void);

/// glMultiTexEnvfEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `param` group: CheckedFloat32
pub type glMultiTexEnvfEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, param: GLfloat);

/// glMultiTexEnvfvEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexEnvfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLfloat);

/// glMultiTexEnviEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `param` group: CheckedInt32
pub type glMultiTexEnviEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, param: GLint);

/// glMultiTexEnvivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexEnvivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLint);

/// glMultiTexGendEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glMultiTexGendEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, param: GLdouble);

/// glMultiTexGendvEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexGendvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *const GLdouble);

/// glMultiTexGenfEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `param` group: CheckedFloat32
pub type glMultiTexGenfEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, param: GLfloat);

/// glMultiTexGenfvEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexGenfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *const GLfloat);

/// glMultiTexGeniEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `param` group: CheckedInt32
pub type glMultiTexGeniEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, param: GLint);

/// glMultiTexGenivEXT
/// * `texunit` group: TextureUnit
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexGenivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, coord: TextureCoordName, pname: TextureGenParameter, params: *const GLint);

/// glMultiTexImage1DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glMultiTexImage2DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glMultiTexImage3DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glMultiTexImage3DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glMultiTexParameterIivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexParameterIivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// glMultiTexParameterIuivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexParameterIuivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// glMultiTexParameterfEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedFloat32
pub type glMultiTexParameterfEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, param: GLfloat);

/// glMultiTexParameterfvEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexParameterfvEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);

/// glMultiTexParameteriEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedInt32
pub type glMultiTexParameteriEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, param: GLint);

/// glMultiTexParameterivEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glMultiTexParameterivEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// glMultiTexRenderbufferEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `renderbuffer` class: renderbuffer
pub type glMultiTexRenderbufferEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, renderbuffer: GLuint);

/// glMultiTexSubImage1DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glMultiTexSubImage2DEXT
/// * `texunit` group: TextureUnit
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: TextureUnit, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glMultiTexSubImage3DEXT
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

/// glMulticastBarrierNV
pub type glMulticastBarrierNV_t = unsafe extern "system" fn();

/// glMulticastBlitFramebufferNV
/// * `mask` group: ClearBufferMask
pub type glMulticastBlitFramebufferNV_t = unsafe extern "system" fn(srcGpu: GLuint, dstGpu: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);

/// glMulticastBufferSubDataNV
/// * `buffer` class: buffer
pub type glMulticastBufferSubDataNV_t = unsafe extern "system" fn(gpuMask: GLbitfield, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// glMulticastCopyBufferSubDataNV
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
pub type glMulticastCopyBufferSubDataNV_t = unsafe extern "system" fn(readGpu: GLuint, writeGpuMask: GLbitfield, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// glMulticastCopyImageSubDataNV
pub type glMulticastCopyImageSubDataNV_t = unsafe extern "system" fn(srcGpu: GLuint, dstGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

/// glMulticastFramebufferSampleLocationsfvNV
/// * `framebuffer` class: framebuffer
pub type glMulticastFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(gpu: GLuint, framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

/// glMulticastGetQueryObjecti64vNV
pub type glMulticastGetQueryObjecti64vNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLint64);

/// glMulticastGetQueryObjectivNV
pub type glMulticastGetQueryObjectivNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLint);

/// glMulticastGetQueryObjectui64vNV
pub type glMulticastGetQueryObjectui64vNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLuint64);

/// glMulticastGetQueryObjectuivNV
pub type glMulticastGetQueryObjectuivNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLuint);

/// glMulticastScissorArrayvNVX
/// * `v` len: COMPSIZE(count)
pub type glMulticastScissorArrayvNVX_t = unsafe extern "system" fn(gpu: GLuint, first: GLuint, count: GLsizei, v: *const GLint);

/// glMulticastViewportArrayvNVX
/// * `v` len: COMPSIZE(count)
pub type glMulticastViewportArrayvNVX_t = unsafe extern "system" fn(gpu: GLuint, first: GLuint, count: GLsizei, v: *const GLfloat);

/// glMulticastViewportPositionWScaleNVX
pub type glMulticastViewportPositionWScaleNVX_t = unsafe extern "system" fn(gpu: GLuint, index: GLuint, xcoeff: GLfloat, ycoeff: GLfloat);

/// glMulticastWaitSyncNV
pub type glMulticastWaitSyncNV_t = unsafe extern "system" fn(signalGpu: GLuint, waitGpuMask: GLbitfield);

/// glNamedBufferAttachMemoryNV
/// * `buffer` class: buffer
pub type glNamedBufferAttachMemoryNV_t = unsafe extern "system" fn(buffer: GLuint, memory: GLuint, offset: GLuint64);

/// glNamedBufferData
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `usage` group: VertexBufferObjectUsage
pub type glNamedBufferData_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: VertexBufferObjectUsage);

/// glNamedBufferDataEXT
/// * `buffer` class: buffer
/// * `data` len: COMPSIZE(size)
/// * `usage` group: VertexBufferObjectUsage
pub type glNamedBufferDataEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: VertexBufferObjectUsage);

/// glNamedBufferPageCommitmentARB
/// * `buffer` class: buffer
/// * `commit` group: Boolean
pub type glNamedBufferPageCommitmentARB_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

/// glNamedBufferPageCommitmentEXT
/// * `buffer` class: buffer
/// * `commit` group: Boolean
pub type glNamedBufferPageCommitmentEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

/// glNamedBufferPageCommitmentMemNV
/// * `buffer` class: buffer
/// * `commit` group: Boolean
pub type glNamedBufferPageCommitmentMemNV_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, memory: GLuint, memOffset: GLuint64, commit: GLboolean);

/// glNamedBufferStorage
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: size
/// * `flags` group: BufferStorageMask
pub type glNamedBufferStorage_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield);

/// glNamedBufferStorageEXT
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: size
/// * `flags` group: BufferStorageMask
pub type glNamedBufferStorageEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield);

/// glNamedBufferStorageExternalEXT
/// * `buffer` class: buffer
/// * `flags` group: BufferStorageMask
pub type glNamedBufferStorageExternalEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, clientBuffer: GLeglClientBufferEXT, flags: GLbitfield);

/// glNamedBufferStorageMemEXT
/// * `buffer` class: buffer
/// * `size` group: BufferSize
pub type glNamedBufferStorageMemEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, memory: GLuint, offset: GLuint64);

/// glNamedBufferSubData
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: COMPSIZE(size)
pub type glNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// glNamedBufferSubDataEXT
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: COMPSIZE(size)
pub type glNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

/// glNamedCopyBufferSubDataEXT
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
pub type glNamedCopyBufferSubDataEXT_t = unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

/// glNamedFramebufferDrawBuffer
/// * `framebuffer` class: framebuffer
/// * `buf` group: ColorBuffer
pub type glNamedFramebufferDrawBuffer_t = unsafe extern "system" fn(framebuffer: GLuint, buf: ColorBuffer);

/// glNamedFramebufferDrawBuffers
/// * `framebuffer` class: framebuffer
/// * `bufs` group: ColorBuffer
pub type glNamedFramebufferDrawBuffers_t = unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const ColorBuffer);

/// glNamedFramebufferParameteri
/// * `framebuffer` class: framebuffer
/// * `pname` group: FramebufferParameterName
pub type glNamedFramebufferParameteri_t = unsafe extern "system" fn(framebuffer: GLuint, pname: FramebufferParameterName, param: GLint);

/// glNamedFramebufferParameteriEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `pname` group: FramebufferParameterName
pub type glNamedFramebufferParameteriEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: FramebufferParameterName, param: GLint);

/// glNamedFramebufferReadBuffer
/// * `framebuffer` class: framebuffer
/// * `src` group: ColorBuffer
pub type glNamedFramebufferReadBuffer_t = unsafe extern "system" fn(framebuffer: GLuint, src: ColorBuffer);

/// glNamedFramebufferRenderbuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
pub type glNamedFramebufferRenderbuffer_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// glNamedFramebufferRenderbufferEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
pub type glNamedFramebufferRenderbufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

/// glNamedFramebufferSampleLocationsfvARB
/// * `framebuffer` class: framebuffer
pub type glNamedFramebufferSampleLocationsfvARB_t = unsafe extern "system" fn(framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

/// glNamedFramebufferSampleLocationsfvNV
/// * `framebuffer` class: framebuffer
pub type glNamedFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

/// glNamedFramebufferSamplePositionsfvAMD
/// * `framebuffer` class: framebuffer
pub type glNamedFramebufferSamplePositionsfvAMD_t = unsafe extern "system" fn(framebuffer: GLuint, numsamples: GLuint, pixelindex: GLuint, values: *const GLfloat);

/// glNamedFramebufferTexture
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
pub type glNamedFramebufferTexture_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// glNamedFramebufferTexture1DEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glNamedFramebufferTexture1DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// glNamedFramebufferTexture2DEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glNamedFramebufferTexture2DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

/// glNamedFramebufferTexture3DEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
pub type glNamedFramebufferTexture3DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

/// glNamedFramebufferTextureEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
pub type glNamedFramebufferTextureEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

/// glNamedFramebufferTextureFaceEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `face` group: TextureTarget
pub type glNamedFramebufferTextureFaceEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, face: TextureTarget);

/// glNamedFramebufferTextureLayer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
pub type glNamedFramebufferTextureLayer_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// glNamedFramebufferTextureLayerEXT
/// * `framebuffer` group: Framebuffer
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
pub type glNamedFramebufferTextureLayerEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

/// glNamedProgramLocalParameter4dEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameter4dEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glNamedProgramLocalParameter4dvEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameter4dvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *const [GLdouble; 4]);

/// glNamedProgramLocalParameter4fEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameter4fEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glNamedProgramLocalParameter4fvEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameter4fvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *const [GLfloat; 4]);

/// glNamedProgramLocalParameterI4iEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameterI4iEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// glNamedProgramLocalParameterI4ivEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameterI4ivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *const [GLint; 4]);

/// glNamedProgramLocalParameterI4uiEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameterI4uiEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// glNamedProgramLocalParameterI4uivEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
pub type glNamedProgramLocalParameterI4uivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, params: *const [GLuint; 4]);

/// glNamedProgramLocalParameters4fvEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glNamedProgramLocalParameters4fvEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLfloat);

/// glNamedProgramLocalParametersI4ivEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glNamedProgramLocalParametersI4ivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLint);

/// glNamedProgramLocalParametersI4uivEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glNamedProgramLocalParametersI4uivEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLuint);

/// glNamedProgramStringEXT
/// * `program` class: program
/// * `target` group: ProgramTarget
/// * `format` group: ProgramFormat
/// * `string` len: len
pub type glNamedProgramStringEXT_t = unsafe extern "system" fn(program: GLuint, target: ProgramTarget, format: ProgramFormat, len: GLsizei, string: *const void);

/// glNamedRenderbufferStorage
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorage_t = unsafe extern "system" fn(renderbuffer: GLuint, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glNamedRenderbufferStorageEXT
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glNamedRenderbufferStorageMultisample
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageMultisample_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glNamedRenderbufferStorageMultisampleAdvancedAMD
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageMultisampleAdvancedAMD_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, storageSamples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glNamedRenderbufferStorageMultisampleCoverageEXT
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageMultisampleCoverageEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glNamedRenderbufferStorageMultisampleEXT
/// * `renderbuffer` group: Renderbuffer
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
pub type glNamedRenderbufferStorageMultisampleEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glNamedStringARB
/// * `name` len: namelen
/// * `string` len: stringlen
pub type glNamedStringARB_t = unsafe extern "system" fn(type_: GLenum, namelen: GLint, name: *const GLchar, stringlen: GLint, string: *const GLchar);

/// glNewList
/// * `list` group: List
/// * `mode` group: ListMode
pub type glNewList_t = unsafe extern "system" fn(list: GLuint, mode: ListMode);

/// glNewObjectBufferATI
/// * `pointer` len: size
/// * `usage` group: ArrayObjectUsageATI
pub type glNewObjectBufferATI_t = unsafe extern "system" fn(size: GLsizei, pointer: *const void, usage: ArrayObjectUsageATI) -> GLuint;

/// glNormal3b
pub type glNormal3b_t = unsafe extern "system" fn(nx: GLbyte, ny: GLbyte, nz: GLbyte);

/// glNormal3bv
pub type glNormal3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// glNormal3d
/// * `nx` group: CoordD
/// * `ny` group: CoordD
/// * `nz` group: CoordD
pub type glNormal3d_t = unsafe extern "system" fn(nx: GLdouble, ny: GLdouble, nz: GLdouble);

/// glNormal3dv
/// * `v` group: CoordD
pub type glNormal3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glNormal3f
/// * `nx` group: CoordF
/// * `ny` group: CoordF
/// * `nz` group: CoordF
pub type glNormal3f_t = unsafe extern "system" fn(nx: GLfloat, ny: GLfloat, nz: GLfloat);

/// glNormal3fVertex3fSUN
pub type glNormal3fVertex3fSUN_t = unsafe extern "system" fn(nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glNormal3fVertex3fvSUN
pub type glNormal3fVertex3fvSUN_t = unsafe extern "system" fn(n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glNormal3fv
/// * `v` group: CoordF
pub type glNormal3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glNormal3hNV
/// * `nx` group: Half16NV
/// * `ny` group: Half16NV
/// * `nz` group: Half16NV
pub type glNormal3hNV_t = unsafe extern "system" fn(nx: GLhalfNV, ny: GLhalfNV, nz: GLhalfNV);

/// glNormal3hvNV
/// * `v` group: Half16NV
pub type glNormal3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// glNormal3i
pub type glNormal3i_t = unsafe extern "system" fn(nx: GLint, ny: GLint, nz: GLint);

/// glNormal3iv
pub type glNormal3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glNormal3s
pub type glNormal3s_t = unsafe extern "system" fn(nx: GLshort, ny: GLshort, nz: GLshort);

/// glNormal3sv
pub type glNormal3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glNormal3x
pub type glNormal3x_t = unsafe extern "system" fn(nx: GLfixed, ny: GLfixed, nz: GLfixed);

/// glNormal3xOES
pub type glNormal3xOES_t = unsafe extern "system" fn(nx: GLfixed, ny: GLfixed, nz: GLfixed);

/// glNormal3xvOES
pub type glNormal3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

/// glNormalFormatNV
pub type glNormalFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

/// glNormalP3ui
/// * `type` group: NormalPointerType
pub type glNormalP3ui_t = unsafe extern "system" fn(type_: NormalPointerType, coords: GLuint);

/// glNormalP3uiv
/// * `type` group: NormalPointerType
pub type glNormalP3uiv_t = unsafe extern "system" fn(type_: NormalPointerType, coords: *const GLuint);

/// glNormalPointer
/// * `type` group: NormalPointerType
/// * `pointer` len: COMPSIZE(type,stride)
pub type glNormalPointer_t = unsafe extern "system" fn(type_: NormalPointerType, stride: GLsizei, pointer: *const void);

/// glNormalPointerEXT
/// * `type` group: NormalPointerType
/// * `pointer` len: COMPSIZE(type,stride,count)
pub type glNormalPointerEXT_t = unsafe extern "system" fn(type_: NormalPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// glNormalPointerListIBM
/// * `type` group: NormalPointerType
/// * `pointer` len: COMPSIZE(type,stride)
pub type glNormalPointerListIBM_t = unsafe extern "system" fn(type_: NormalPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// glNormalPointervINTEL
/// * `type` group: NormalPointerType
pub type glNormalPointervINTEL_t = unsafe extern "system" fn(type_: NormalPointerType, pointer: *const [*mut void; 4]);

/// glNormalStream3bATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3bATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLbyte, ny: GLbyte, nz: GLbyte);

/// glNormalStream3bvATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3bvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLbyte; 3]);

/// glNormalStream3dATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLdouble, ny: GLdouble, nz: GLdouble);

/// glNormalStream3dvATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLdouble; 3]);

/// glNormalStream3fATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLfloat, ny: GLfloat, nz: GLfloat);

/// glNormalStream3fvATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLfloat; 3]);

/// glNormalStream3iATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLint, ny: GLint, nz: GLint);

/// glNormalStream3ivATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLint; 3]);

/// glNormalStream3sATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, nx: GLshort, ny: GLshort, nz: GLshort);

/// glNormalStream3svATI
/// * `stream` group: VertexStreamATI
pub type glNormalStream3svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLshort; 3]);

/// glObjectLabel
/// * `identifier` group: ObjectIdentifier
/// * `label` len: COMPSIZE(label,length)
pub type glObjectLabel_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar);

/// glObjectLabelKHR
/// * `identifier` group: ObjectIdentifier
pub type glObjectLabelKHR_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar);

/// glObjectPtrLabel
/// * `label` len: COMPSIZE(label,length)
pub type glObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, length: GLsizei, label: *const GLchar);

/// glObjectPtrLabelKHR
pub type glObjectPtrLabelKHR_t = unsafe extern "system" fn(ptr: *const void, length: GLsizei, label: *const GLchar);

/// glObjectPurgeableAPPLE
pub type glObjectPurgeableAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, option: GLenum) -> GLenum;

/// glObjectUnpurgeableAPPLE
pub type glObjectUnpurgeableAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, option: GLenum) -> GLenum;

/// glOrtho
pub type glOrtho_t = unsafe extern "system" fn(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

/// glOrthof
pub type glOrthof_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

/// glOrthofOES
pub type glOrthofOES_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

/// glOrthox
pub type glOrthox_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

/// glOrthoxOES
pub type glOrthoxOES_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

/// glPNTrianglesfATI
/// * `pname` group: PNTrianglesPNameATI
pub type glPNTrianglesfATI_t = unsafe extern "system" fn(pname: PNTrianglesPNameATI, param: GLfloat);

/// glPNTrianglesiATI
/// * `pname` group: PNTrianglesPNameATI
pub type glPNTrianglesiATI_t = unsafe extern "system" fn(pname: PNTrianglesPNameATI, param: GLint);

/// glPassTexCoordATI
/// * `swizzle` group: SwizzleOpATI
pub type glPassTexCoordATI_t = unsafe extern "system" fn(dst: GLuint, coord: GLuint, swizzle: SwizzleOpATI);

/// glPassThrough
/// * `token` group: FeedbackElement
pub type glPassThrough_t = unsafe extern "system" fn(token: GLfloat);

/// glPassThroughxOES
pub type glPassThroughxOES_t = unsafe extern "system" fn(token: GLfixed);

/// glPatchParameterfv
/// * `pname` group: PatchParameterName
/// * `values` len: COMPSIZE(pname)
pub type glPatchParameterfv_t = unsafe extern "system" fn(pname: PatchParameterName, values: *const GLfloat);

/// glPatchParameteri
/// * `pname` group: PatchParameterName
pub type glPatchParameteri_t = unsafe extern "system" fn(pname: PatchParameterName, value: GLint);

/// glPatchParameteriEXT
/// * `pname` group: PatchParameterName
pub type glPatchParameteriEXT_t = unsafe extern "system" fn(pname: PatchParameterName, value: GLint);

/// glPatchParameteriOES
/// * `pname` group: PatchParameterName
pub type glPatchParameteriOES_t = unsafe extern "system" fn(pname: PatchParameterName, value: GLint);

/// glPathColorGenNV
/// * `color` group: PathColor
/// * `genMode` group: PathGenMode
/// * `colorFormat` group: PathColorFormat
/// * `coeffs` len: COMPSIZE(genMode,colorFormat)
pub type glPathColorGenNV_t = unsafe extern "system" fn(color: PathColor, genMode: PathGenMode, colorFormat: PathColorFormat, coeffs: *const GLfloat);

/// glPathCommandsNV
/// * `path` group: Path
/// * `commands` group: PathCommand
/// * `commands` len: numCommands
/// * `coordType` group: PathCoordType
/// * `coords` len: COMPSIZE(numCoords,coordType)
pub type glPathCommandsNV_t = unsafe extern "system" fn(path: GLuint, numCommands: GLsizei, commands: *const GLubyte, numCoords: GLsizei, coordType: PathCoordType, coords: *const void);

/// glPathCoordsNV
/// * `path` group: Path
/// * `coordType` group: PathCoordType
/// * `coords` len: COMPSIZE(numCoords,coordType)
pub type glPathCoordsNV_t = unsafe extern "system" fn(path: GLuint, numCoords: GLsizei, coordType: PathCoordType, coords: *const void);

/// glPathCoverDepthFuncNV
/// * `func` group: DepthFunction
pub type glPathCoverDepthFuncNV_t = unsafe extern "system" fn(func: DepthFunction);

/// glPathDashArrayNV
/// * `path` group: Path
/// * `dashArray` len: dashCount
pub type glPathDashArrayNV_t = unsafe extern "system" fn(path: GLuint, dashCount: GLsizei, dashArray: *const GLfloat);

/// glPathFogGenNV
/// * `genMode` group: PathGenMode
pub type glPathFogGenNV_t = unsafe extern "system" fn(genMode: PathGenMode);

/// glPathGlyphIndexArrayNV
/// * `fontStyle` group: PathFontStyle
pub type glPathGlyphIndexArrayNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: GLenum, fontName: *const void, fontStyle: GLbitfield, firstGlyphIndex: GLuint, numGlyphs: GLsizei, pathParameterTemplate: GLuint, emScale: GLfloat) -> GLenum;

/// glPathGlyphIndexRangeNV
/// * `fontStyle` group: PathFontStyle
/// * `baseAndCount` len: 2
pub type glPathGlyphIndexRangeNV_t = unsafe extern "system" fn(fontTarget: GLenum, fontName: *const void, fontStyle: GLbitfield, pathParameterTemplate: GLuint, emScale: GLfloat, baseAndCount: *mut [GLuint; 2]) -> GLenum;

/// glPathGlyphRangeNV
/// * `firstPathName` group: Path
/// * `fontTarget` group: PathFontTarget
/// * `fontName` len: COMPSIZE(fontTarget,fontName)
/// * `fontStyle` group: PathFontStyle
/// * `handleMissingGlyphs` group: PathHandleMissingGlyphs
/// * `pathParameterTemplate` group: Path
pub type glPathGlyphRangeNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: PathFontTarget, fontName: *const void, fontStyle: GLbitfield, firstGlyph: GLuint, numGlyphs: GLsizei, handleMissingGlyphs: PathHandleMissingGlyphs, pathParameterTemplate: GLuint, emScale: GLfloat);

/// glPathGlyphsNV
/// * `firstPathName` group: Path
/// * `fontTarget` group: PathFontTarget
/// * `fontName` len: COMPSIZE(fontTarget,fontName)
/// * `fontStyle` group: PathFontStyle
/// * `type` group: PathElementType
/// * `charcodes` len: COMPSIZE(numGlyphs,type,charcodes)
/// * `handleMissingGlyphs` group: PathHandleMissingGlyphs
/// * `pathParameterTemplate` group: Path
pub type glPathGlyphsNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: PathFontTarget, fontName: *const void, fontStyle: GLbitfield, numGlyphs: GLsizei, type_: PathElementType, charcodes: *const void, handleMissingGlyphs: PathHandleMissingGlyphs, pathParameterTemplate: GLuint, emScale: GLfloat);

/// glPathMemoryGlyphIndexArrayNV
pub type glPathMemoryGlyphIndexArrayNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: GLenum, fontSize: GLsizeiptr, fontData: *const void, faceIndex: GLsizei, firstGlyphIndex: GLuint, numGlyphs: GLsizei, pathParameterTemplate: GLuint, emScale: GLfloat) -> GLenum;

/// glPathParameterfNV
/// * `path` group: Path
/// * `pname` group: PathParameter
pub type glPathParameterfNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: GLfloat);

/// glPathParameterfvNV
/// * `path` group: Path
/// * `pname` group: PathParameter
/// * `value` len: COMPSIZE(pname)
pub type glPathParameterfvNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: *const GLfloat);

/// glPathParameteriNV
/// * `path` group: Path
/// * `pname` group: PathParameter
pub type glPathParameteriNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: GLint);

/// glPathParameterivNV
/// * `path` group: Path
/// * `pname` group: PathParameter
/// * `value` len: COMPSIZE(pname)
pub type glPathParameterivNV_t = unsafe extern "system" fn(path: GLuint, pname: PathParameter, value: *const GLint);

/// glPathStencilDepthOffsetNV
pub type glPathStencilDepthOffsetNV_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);

/// glPathStencilFuncNV
/// * `func` group: StencilFunction
/// * `ref` group: ClampedStencilValue
/// * `mask` group: MaskedStencilValue
pub type glPathStencilFuncNV_t = unsafe extern "system" fn(func: StencilFunction, ref_: GLint, mask: GLuint);

/// glPathStringNV
/// * `path` group: Path
/// * `format` group: PathStringFormat
/// * `pathString` len: length
pub type glPathStringNV_t = unsafe extern "system" fn(path: GLuint, format: PathStringFormat, length: GLsizei, pathString: *const void);

/// glPathSubCommandsNV
/// * `path` group: Path
/// * `commands` group: PathCommand
/// * `commands` len: numCommands
/// * `coordType` group: PathCoordType
/// * `coords` len: COMPSIZE(numCoords,coordType)
pub type glPathSubCommandsNV_t = unsafe extern "system" fn(path: GLuint, commandStart: GLsizei, commandsToDelete: GLsizei, numCommands: GLsizei, commands: *const GLubyte, numCoords: GLsizei, coordType: PathCoordType, coords: *const void);

/// glPathSubCoordsNV
/// * `path` group: Path
/// * `coordType` group: PathCoordType
/// * `coords` len: COMPSIZE(numCoords,coordType)
pub type glPathSubCoordsNV_t = unsafe extern "system" fn(path: GLuint, coordStart: GLsizei, numCoords: GLsizei, coordType: PathCoordType, coords: *const void);

/// glPathTexGenNV
/// * `texCoordSet` group: PathColor
/// * `genMode` group: PathGenMode
/// * `coeffs` len: COMPSIZE(genMode,components)
pub type glPathTexGenNV_t = unsafe extern "system" fn(texCoordSet: PathColor, genMode: PathGenMode, components: GLint, coeffs: *const GLfloat);

/// glPauseTransformFeedback
pub type glPauseTransformFeedback_t = unsafe extern "system" fn();

/// glPauseTransformFeedbackNV
pub type glPauseTransformFeedbackNV_t = unsafe extern "system" fn();

/// glPixelDataRangeNV
/// * `target` group: PixelDataRangeTargetNV
/// * `pointer` len: length
pub type glPixelDataRangeNV_t = unsafe extern "system" fn(target: PixelDataRangeTargetNV, length: GLsizei, pointer: *const void);

/// glPixelMapfv
/// * `map` group: PixelMap
/// * `mapsize` group: CheckedInt32
/// * `values` len: mapsize
pub type glPixelMapfv_t = unsafe extern "system" fn(map: PixelMap, mapsize: GLsizei, values: *const GLfloat);

/// glPixelMapuiv
/// * `map` group: PixelMap
/// * `mapsize` group: CheckedInt32
/// * `values` len: mapsize
pub type glPixelMapuiv_t = unsafe extern "system" fn(map: PixelMap, mapsize: GLsizei, values: *const GLuint);

/// glPixelMapusv
/// * `map` group: PixelMap
/// * `mapsize` group: CheckedInt32
/// * `values` len: mapsize
pub type glPixelMapusv_t = unsafe extern "system" fn(map: PixelMap, mapsize: GLsizei, values: *const GLushort);

/// glPixelMapx
/// * `map` group: PixelMap
/// * `values` len: size
pub type glPixelMapx_t = unsafe extern "system" fn(map: PixelMap, size: GLint, values: *const GLfixed);

/// glPixelStoref
/// * `pname` group: PixelStoreParameter
/// * `param` group: CheckedFloat32
pub type glPixelStoref_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLfloat);

/// glPixelStorei
/// * `pname` group: PixelStoreParameter
/// * `param` group: CheckedInt32
pub type glPixelStorei_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLint);

/// glPixelStorex
/// * `pname` group: PixelStoreParameter
pub type glPixelStorex_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLfixed);

/// glPixelTexGenParameterfSGIS
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `param` group: CheckedFloat32
pub type glPixelTexGenParameterfSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, param: GLfloat);

/// glPixelTexGenParameterfvSGIS
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPixelTexGenParameterfvSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, params: *const GLfloat);

/// glPixelTexGenParameteriSGIS
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `param` group: CheckedInt32
pub type glPixelTexGenParameteriSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, param: GLint);

/// glPixelTexGenParameterivSGIS
/// * `pname` group: PixelTexGenParameterNameSGIS
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glPixelTexGenParameterivSGIS_t = unsafe extern "system" fn(pname: PixelTexGenParameterNameSGIS, params: *const GLint);

/// glPixelTexGenSGIX
/// * `mode` group: PixelTexGenModeSGIX
pub type glPixelTexGenSGIX_t = unsafe extern "system" fn(mode: PixelTexGenModeSGIX);

/// glPixelTransferf
/// * `pname` group: PixelTransferParameter
/// * `param` group: CheckedFloat32
pub type glPixelTransferf_t = unsafe extern "system" fn(pname: PixelTransferParameter, param: GLfloat);

/// glPixelTransferi
/// * `pname` group: PixelTransferParameter
/// * `param` group: CheckedInt32
pub type glPixelTransferi_t = unsafe extern "system" fn(pname: PixelTransferParameter, param: GLint);

/// glPixelTransferxOES
/// * `pname` group: PixelTransferParameter
pub type glPixelTransferxOES_t = unsafe extern "system" fn(pname: PixelTransferParameter, param: GLfixed);

/// glPixelTransformParameterfEXT
/// * `target` group: PixelTransformTargetEXT
/// * `pname` group: PixelTransformPNameEXT
pub type glPixelTransformParameterfEXT_t = unsafe extern "system" fn(target: PixelTransformTargetEXT, pname: PixelTransformPNameEXT, param: GLfloat);

/// glPixelTransformParameterfvEXT
/// * `target` group: PixelTransformTargetEXT
/// * `pname` group: PixelTransformPNameEXT
pub type glPixelTransformParameterfvEXT_t = unsafe extern "system" fn(target: PixelTransformTargetEXT, pname: PixelTransformPNameEXT, params: *const GLfloat);

/// glPixelTransformParameteriEXT
/// * `target` group: PixelTransformTargetEXT
/// * `pname` group: PixelTransformPNameEXT
pub type glPixelTransformParameteriEXT_t = unsafe extern "system" fn(target: PixelTransformTargetEXT, pname: PixelTransformPNameEXT, param: GLint);

/// glPixelTransformParameterivEXT
/// * `target` group: PixelTransformTargetEXT
/// * `pname` group: PixelTransformPNameEXT
pub type glPixelTransformParameterivEXT_t = unsafe extern "system" fn(target: PixelTransformTargetEXT, pname: PixelTransformPNameEXT, params: *const GLint);

/// glPixelZoom
pub type glPixelZoom_t = unsafe extern "system" fn(xfactor: GLfloat, yfactor: GLfloat);

/// glPixelZoomxOES
pub type glPixelZoomxOES_t = unsafe extern "system" fn(xfactor: GLfixed, yfactor: GLfixed);

/// glPointAlongPathNV
/// * `path` group: Path
pub type glPointAlongPathNV_t = unsafe extern "system" fn(path: GLuint, startSegment: GLsizei, numSegments: GLsizei, distance: GLfloat, x: *mut GLfloat, y: *mut GLfloat, tangentX: *mut GLfloat, tangentY: *mut GLfloat) -> GLboolean;

/// glPointParameterf
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
pub type glPointParameterf_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

/// glPointParameterfARB
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
pub type glPointParameterfARB_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

/// glPointParameterfEXT
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
pub type glPointParameterfEXT_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

/// glPointParameterfSGIS
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
pub type glPointParameterfSGIS_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

/// glPointParameterfv
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterfv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

/// glPointParameterfvARB
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterfvARB_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

/// glPointParameterfvEXT
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterfvEXT_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

/// glPointParameterfvSGIS
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterfvSGIS_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

/// glPointParameteri
/// * `pname` group: PointParameterNameARB
pub type glPointParameteri_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLint);

/// glPointParameteriNV
/// * `pname` group: PointParameterNameARB
pub type glPointParameteriNV_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLint);

/// glPointParameteriv
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
pub type glPointParameteriv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLint);

/// glPointParameterivNV
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterivNV_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLint);

/// glPointParameterx
/// * `pname` group: PointParameterNameARB
pub type glPointParameterx_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfixed);

/// glPointParameterxOES
/// * `pname` group: PointParameterNameARB
pub type glPointParameterxOES_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfixed);

/// glPointParameterxv
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterxv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfixed);

/// glPointParameterxvOES
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
pub type glPointParameterxvOES_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfixed);

/// glPointSize
/// * `size` group: CheckedFloat32
pub type glPointSize_t = unsafe extern "system" fn(size: GLfloat);

/// glPointSizePointerOES
/// * `pointer` len: COMPSIZE(type,stride)
pub type glPointSizePointerOES_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

/// glPointSizex
pub type glPointSizex_t = unsafe extern "system" fn(size: GLfixed);

/// glPointSizexOES
pub type glPointSizexOES_t = unsafe extern "system" fn(size: GLfixed);

/// glPollAsyncSGIX
pub type glPollAsyncSGIX_t = unsafe extern "system" fn(markerp: *mut GLuint) -> GLint;

/// glPollInstrumentsSGIX
pub type glPollInstrumentsSGIX_t = unsafe extern "system" fn(marker_p: *mut GLint) -> GLint;

/// glPolygonMode
/// * `face` group: MaterialFace
/// * `mode` group: PolygonMode
pub type glPolygonMode_t = unsafe extern "system" fn(face: MaterialFace, mode: PolygonMode);

/// glPolygonModeNV
/// * `face` group: MaterialFace
/// * `mode` group: PolygonMode
pub type glPolygonModeNV_t = unsafe extern "system" fn(face: MaterialFace, mode: PolygonMode);

/// glPolygonOffset
pub type glPolygonOffset_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);

/// glPolygonOffsetClamp
pub type glPolygonOffsetClamp_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);

/// glPolygonOffsetClampEXT
pub type glPolygonOffsetClampEXT_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);

/// glPolygonOffsetEXT
pub type glPolygonOffsetEXT_t = unsafe extern "system" fn(factor: GLfloat, bias: GLfloat);

/// glPolygonOffsetx
pub type glPolygonOffsetx_t = unsafe extern "system" fn(factor: GLfixed, units: GLfixed);

/// glPolygonOffsetxOES
pub type glPolygonOffsetxOES_t = unsafe extern "system" fn(factor: GLfixed, units: GLfixed);

/// glPolygonStipple
/// * `mask` len: COMPSIZE()
pub type glPolygonStipple_t = unsafe extern "system" fn(mask: *const GLubyte);

/// glPopAttrib
pub type glPopAttrib_t = unsafe extern "system" fn();

/// glPopClientAttrib
pub type glPopClientAttrib_t = unsafe extern "system" fn();

/// glPopDebugGroup
pub type glPopDebugGroup_t = unsafe extern "system" fn();

/// glPopDebugGroupKHR
pub type glPopDebugGroupKHR_t = unsafe extern "system" fn();

/// glPopGroupMarkerEXT
pub type glPopGroupMarkerEXT_t = unsafe extern "system" fn();

/// glPopMatrix
pub type glPopMatrix_t = unsafe extern "system" fn();

/// glPopName
pub type glPopName_t = unsafe extern "system" fn();

/// glPresentFrameDualFillNV
pub type glPresentFrameDualFillNV_t = unsafe extern "system" fn(video_slot: GLuint, minPresentTime: GLuint64EXT, beginPresentTimeId: GLuint, presentDurationId: GLuint, type_: GLenum, target0: GLenum, fill0: GLuint, target1: GLenum, fill1: GLuint, target2: GLenum, fill2: GLuint, target3: GLenum, fill3: GLuint);

/// glPresentFrameKeyedNV
pub type glPresentFrameKeyedNV_t = unsafe extern "system" fn(video_slot: GLuint, minPresentTime: GLuint64EXT, beginPresentTimeId: GLuint, presentDurationId: GLuint, type_: GLenum, target0: GLenum, fill0: GLuint, key0: GLuint, target1: GLenum, fill1: GLuint, key1: GLuint);

/// glPrimitiveBoundingBox
pub type glPrimitiveBoundingBox_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

/// glPrimitiveBoundingBoxARB
pub type glPrimitiveBoundingBoxARB_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

/// glPrimitiveBoundingBoxEXT
pub type glPrimitiveBoundingBoxEXT_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

/// glPrimitiveBoundingBoxOES
pub type glPrimitiveBoundingBoxOES_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

/// glPrimitiveRestartIndex
pub type glPrimitiveRestartIndex_t = unsafe extern "system" fn(index: GLuint);

/// glPrimitiveRestartIndexNV
pub type glPrimitiveRestartIndexNV_t = unsafe extern "system" fn(index: GLuint);

/// glPrimitiveRestartNV
pub type glPrimitiveRestartNV_t = unsafe extern "system" fn();

/// glPrioritizeTextures
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
/// * `priorities` len: n
pub type glPrioritizeTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfloat);

/// glPrioritizeTexturesEXT
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
/// * `priorities` group: ClampedFloat32
/// * `priorities` len: n
pub type glPrioritizeTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLclampf);

/// glPrioritizeTexturesxOES
/// * `textures` class: texture
/// * `textures` len: n
/// * `priorities` group: ClampedFixed
/// * `priorities` len: n
pub type glPrioritizeTexturesxOES_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfixed);

/// glProgramBinary
/// * `program` class: program
/// * `binary` len: length
pub type glProgramBinary_t = unsafe extern "system" fn(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLsizei);

/// glProgramBinaryOES
/// * `program` class: program
/// * `binary` len: length
pub type glProgramBinaryOES_t = unsafe extern "system" fn(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLint);

/// glProgramBufferParametersIivNV
/// * `target` group: ProgramTarget
/// * `params` len: count
pub type glProgramBufferParametersIivNV_t = unsafe extern "system" fn(target: ProgramTarget, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLint);

/// glProgramBufferParametersIuivNV
/// * `target` group: ProgramTarget
/// * `params` len: count
pub type glProgramBufferParametersIuivNV_t = unsafe extern "system" fn(target: ProgramTarget, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLuint);

/// glProgramBufferParametersfvNV
/// * `target` group: ProgramTarget
/// * `params` len: count
pub type glProgramBufferParametersfvNV_t = unsafe extern "system" fn(target: ProgramTarget, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLfloat);

/// glProgramEnvParameter4dARB
/// * `target` group: ProgramTarget
pub type glProgramEnvParameter4dARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glProgramEnvParameter4dvARB
/// * `target` group: ProgramTarget
pub type glProgramEnvParameter4dvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLdouble; 4]);

/// glProgramEnvParameter4fARB
/// * `target` group: ProgramTarget
pub type glProgramEnvParameter4fARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glProgramEnvParameter4fvARB
/// * `target` group: ProgramTarget
pub type glProgramEnvParameter4fvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLfloat; 4]);

/// glProgramEnvParameterI4iNV
/// * `target` group: ProgramTarget
pub type glProgramEnvParameterI4iNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// glProgramEnvParameterI4ivNV
/// * `target` group: ProgramTarget
pub type glProgramEnvParameterI4ivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLint; 4]);

/// glProgramEnvParameterI4uiNV
/// * `target` group: ProgramTarget
pub type glProgramEnvParameterI4uiNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// glProgramEnvParameterI4uivNV
/// * `target` group: ProgramTarget
pub type glProgramEnvParameterI4uivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLuint; 4]);

/// glProgramEnvParameters4fvEXT
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramEnvParameters4fvEXT_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLfloat);

/// glProgramEnvParametersI4ivNV
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramEnvParametersI4ivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLint);

/// glProgramEnvParametersI4uivNV
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramEnvParametersI4uivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLuint);

/// glProgramLocalParameter4dARB
/// * `target` group: ProgramTarget
pub type glProgramLocalParameter4dARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glProgramLocalParameter4dvARB
/// * `target` group: ProgramTarget
pub type glProgramLocalParameter4dvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLdouble; 4]);

/// glProgramLocalParameter4fARB
/// * `target` group: ProgramTarget
pub type glProgramLocalParameter4fARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glProgramLocalParameter4fvARB
/// * `target` group: ProgramTarget
pub type glProgramLocalParameter4fvARB_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLfloat; 4]);

/// glProgramLocalParameterI4iNV
/// * `target` group: ProgramTarget
pub type glProgramLocalParameterI4iNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// glProgramLocalParameterI4ivNV
/// * `target` group: ProgramTarget
pub type glProgramLocalParameterI4ivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLint; 4]);

/// glProgramLocalParameterI4uiNV
/// * `target` group: ProgramTarget
pub type glProgramLocalParameterI4uiNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// glProgramLocalParameterI4uivNV
/// * `target` group: ProgramTarget
pub type glProgramLocalParameterI4uivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, params: *const [GLuint; 4]);

/// glProgramLocalParameters4fvEXT
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramLocalParameters4fvEXT_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLfloat);

/// glProgramLocalParametersI4ivNV
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramLocalParametersI4ivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLint);

/// glProgramLocalParametersI4uivNV
/// * `target` group: ProgramTarget
/// * `params` len: count*4
pub type glProgramLocalParametersI4uivNV_t = unsafe extern "system" fn(target: ProgramTarget, index: GLuint, count: GLsizei, params: *const GLuint);

/// glProgramNamedParameter4dNV
/// * `id` class: program
pub type glProgramNamedParameter4dNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glProgramNamedParameter4dvNV
/// * `id` class: program
pub type glProgramNamedParameter4dvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, v: *const [GLdouble; 4]);

/// glProgramNamedParameter4fNV
/// * `id` class: program
pub type glProgramNamedParameter4fNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glProgramNamedParameter4fvNV
/// * `id` class: program
pub type glProgramNamedParameter4fvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, v: *const [GLfloat; 4]);

/// glProgramParameter4dNV
/// * `target` group: VertexAttribEnumNV
pub type glProgramParameter4dNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glProgramParameter4dvNV
/// * `target` group: VertexAttribEnumNV
pub type glProgramParameter4dvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, v: *const [GLdouble; 4]);

/// glProgramParameter4fNV
/// * `target` group: VertexAttribEnumNV
pub type glProgramParameter4fNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glProgramParameter4fvNV
/// * `target` group: VertexAttribEnumNV
pub type glProgramParameter4fvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, v: *const [GLfloat; 4]);

/// glProgramParameteri
/// * `program` class: program
/// * `pname` group: ProgramParameterPName
pub type glProgramParameteri_t = unsafe extern "system" fn(program: GLuint, pname: ProgramParameterPName, value: GLint);

/// glProgramParameteriARB
/// * `program` class: program
/// * `pname` group: ProgramParameterPName
pub type glProgramParameteriARB_t = unsafe extern "system" fn(program: GLuint, pname: ProgramParameterPName, value: GLint);

/// glProgramParameteriEXT
/// * `program` class: program
/// * `pname` group: ProgramParameterPName
pub type glProgramParameteriEXT_t = unsafe extern "system" fn(program: GLuint, pname: ProgramParameterPName, value: GLint);

/// glProgramParameters4dvNV
/// * `target` group: VertexAttribEnumNV
/// * `v` len: count*4
pub type glProgramParameters4dvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, count: GLsizei, v: *const GLdouble);

/// glProgramParameters4fvNV
/// * `target` group: VertexAttribEnumNV
/// * `v` len: count*4
pub type glProgramParameters4fvNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, index: GLuint, count: GLsizei, v: *const GLfloat);

/// glProgramPathFragmentInputGenNV
/// * `program` class: program
pub type glProgramPathFragmentInputGenNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, genMode: GLenum, components: GLint, coeffs: *const GLfloat);

/// glProgramStringARB
/// * `target` group: ProgramTarget
/// * `format` group: ProgramFormat
/// * `string` len: len
pub type glProgramStringARB_t = unsafe extern "system" fn(target: ProgramTarget, format: ProgramFormat, len: GLsizei, string: *const void);

/// glProgramSubroutineParametersuivNV
/// * `params` len: count
pub type glProgramSubroutineParametersuivNV_t = unsafe extern "system" fn(target: GLenum, count: GLsizei, params: *const GLuint);

/// glProgramUniform1d
/// * `program` class: program
pub type glProgramUniform1d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble);

/// glProgramUniform1dEXT
/// * `program` class: program
pub type glProgramUniform1dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble);

/// glProgramUniform1dv
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// glProgramUniform1dvEXT
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// glProgramUniform1f
/// * `program` class: program
pub type glProgramUniform1f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);

/// glProgramUniform1fEXT
/// * `program` class: program
pub type glProgramUniform1fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);

/// glProgramUniform1fv
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// glProgramUniform1fvEXT
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// glProgramUniform1i
/// * `program` class: program
pub type glProgramUniform1i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);

/// glProgramUniform1i64ARB
/// * `program` class: program
pub type glProgramUniform1i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64);

/// glProgramUniform1i64NV
/// * `program` class: program
pub type glProgramUniform1i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT);

/// glProgramUniform1i64vARB
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

/// glProgramUniform1i64vNV
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

/// glProgramUniform1iEXT
/// * `program` class: program
pub type glProgramUniform1iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);

/// glProgramUniform1iv
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// glProgramUniform1ivEXT
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// glProgramUniform1ui
/// * `program` class: program
pub type glProgramUniform1ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);

/// glProgramUniform1ui64ARB
/// * `program` class: program
pub type glProgramUniform1ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64);

/// glProgramUniform1ui64NV
/// * `program` class: program
pub type glProgramUniform1ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT);

/// glProgramUniform1ui64vARB
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

/// glProgramUniform1ui64vNV
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glProgramUniform1uiEXT
/// * `program` class: program
pub type glProgramUniform1uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);

/// glProgramUniform1uiv
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// glProgramUniform1uivEXT
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniform1uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// glProgramUniform2d
/// * `program` class: program
pub type glProgramUniform2d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);

/// glProgramUniform2dEXT
/// * `program` class: program
pub type glProgramUniform2dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble);

/// glProgramUniform2dv
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// glProgramUniform2dvEXT
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// glProgramUniform2f
/// * `program` class: program
pub type glProgramUniform2f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);

/// glProgramUniform2fEXT
/// * `program` class: program
pub type glProgramUniform2fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);

/// glProgramUniform2fv
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// glProgramUniform2fvEXT
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// glProgramUniform2i
/// * `program` class: program
pub type glProgramUniform2i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);

/// glProgramUniform2i64ARB
/// * `program` class: program
pub type glProgramUniform2i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64);

/// glProgramUniform2i64NV
/// * `program` class: program
pub type glProgramUniform2i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT);

/// glProgramUniform2i64vARB
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

/// glProgramUniform2i64vNV
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

/// glProgramUniform2iEXT
/// * `program` class: program
pub type glProgramUniform2iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);

/// glProgramUniform2iv
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// glProgramUniform2ivEXT
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// glProgramUniform2ui
/// * `program` class: program
pub type glProgramUniform2ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);

/// glProgramUniform2ui64ARB
/// * `program` class: program
pub type glProgramUniform2ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64);

/// glProgramUniform2ui64NV
/// * `program` class: program
pub type glProgramUniform2ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT);

/// glProgramUniform2ui64vARB
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

/// glProgramUniform2ui64vNV
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glProgramUniform2uiEXT
/// * `program` class: program
pub type glProgramUniform2uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);

/// glProgramUniform2uiv
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// glProgramUniform2uivEXT
/// * `program` class: program
/// * `value` len: count*2
pub type glProgramUniform2uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// glProgramUniform3d
/// * `program` class: program
pub type glProgramUniform3d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);

/// glProgramUniform3dEXT
/// * `program` class: program
pub type glProgramUniform3dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);

/// glProgramUniform3dv
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// glProgramUniform3dvEXT
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// glProgramUniform3f
/// * `program` class: program
pub type glProgramUniform3f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

/// glProgramUniform3fEXT
/// * `program` class: program
pub type glProgramUniform3fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

/// glProgramUniform3fv
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// glProgramUniform3fvEXT
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// glProgramUniform3i
/// * `program` class: program
pub type glProgramUniform3i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);

/// glProgramUniform3i64ARB
/// * `program` class: program
pub type glProgramUniform3i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64);

/// glProgramUniform3i64NV
/// * `program` class: program
pub type glProgramUniform3i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

/// glProgramUniform3i64vARB
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

/// glProgramUniform3i64vNV
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

/// glProgramUniform3iEXT
/// * `program` class: program
pub type glProgramUniform3iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);

/// glProgramUniform3iv
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// glProgramUniform3ivEXT
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// glProgramUniform3ui
/// * `program` class: program
pub type glProgramUniform3ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

/// glProgramUniform3ui64ARB
/// * `program` class: program
pub type glProgramUniform3ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);

/// glProgramUniform3ui64NV
/// * `program` class: program
pub type glProgramUniform3ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

/// glProgramUniform3ui64vARB
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

/// glProgramUniform3ui64vNV
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glProgramUniform3uiEXT
/// * `program` class: program
pub type glProgramUniform3uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

/// glProgramUniform3uiv
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// glProgramUniform3uivEXT
/// * `program` class: program
/// * `value` len: count*3
pub type glProgramUniform3uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// glProgramUniform4d
/// * `program` class: program
pub type glProgramUniform4d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);

/// glProgramUniform4dEXT
/// * `program` class: program
pub type glProgramUniform4dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glProgramUniform4dv
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// glProgramUniform4dvEXT
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

/// glProgramUniform4f
/// * `program` class: program
pub type glProgramUniform4f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

/// glProgramUniform4fEXT
/// * `program` class: program
pub type glProgramUniform4fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

/// glProgramUniform4fv
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// glProgramUniform4fvEXT
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

/// glProgramUniform4i
/// * `program` class: program
pub type glProgramUniform4i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

/// glProgramUniform4i64ARB
/// * `program` class: program
pub type glProgramUniform4i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);

/// glProgramUniform4i64NV
/// * `program` class: program
pub type glProgramUniform4i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

/// glProgramUniform4i64vARB
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

/// glProgramUniform4i64vNV
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

/// glProgramUniform4iEXT
/// * `program` class: program
pub type glProgramUniform4iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

/// glProgramUniform4iv
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// glProgramUniform4ivEXT
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

/// glProgramUniform4ui
/// * `program` class: program
pub type glProgramUniform4ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

/// glProgramUniform4ui64ARB
/// * `program` class: program
pub type glProgramUniform4ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);

/// glProgramUniform4ui64NV
/// * `program` class: program
pub type glProgramUniform4ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

/// glProgramUniform4ui64vARB
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

/// glProgramUniform4ui64vNV
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glProgramUniform4uiEXT
/// * `program` class: program
pub type glProgramUniform4uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

/// glProgramUniform4uiv
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// glProgramUniform4uivEXT
/// * `program` class: program
/// * `value` len: count*4
pub type glProgramUniform4uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

/// glProgramUniformHandleui64ARB
/// * `program` class: program
pub type glProgramUniformHandleui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

/// glProgramUniformHandleui64IMG
/// * `program` class: program
pub type glProgramUniformHandleui64IMG_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

/// glProgramUniformHandleui64NV
/// * `program` class: program
pub type glProgramUniformHandleui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

/// glProgramUniformHandleui64vARB
/// * `program` class: program
/// * `values` len: count
pub type glProgramUniformHandleui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

/// glProgramUniformHandleui64vIMG
/// * `program` class: program
/// * `values` len: count
pub type glProgramUniformHandleui64vIMG_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

/// glProgramUniformHandleui64vNV
/// * `program` class: program
/// * `values` len: count
pub type glProgramUniformHandleui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

/// glProgramUniformMatrix2dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glProgramUniformMatrix2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix2dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glProgramUniformMatrix2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix2fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glProgramUniformMatrix2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix2fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glProgramUniformMatrix2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix2x3dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix2x3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix2x3dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix2x3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix2x3fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix2x3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix2x3fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix2x3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix2x4dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix2x4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix2x4dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix2x4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix2x4fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix2x4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix2x4fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix2x4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix3dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glProgramUniformMatrix3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix3dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glProgramUniformMatrix3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix3fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glProgramUniformMatrix3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix3fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glProgramUniformMatrix3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix3x2dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix3x2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix3x2dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix3x2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix3x2fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix3x2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix3x2fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glProgramUniformMatrix3x2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix3x4dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix3x4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix3x4dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix3x4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix3x4fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix3x4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix3x4fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix3x4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix4dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glProgramUniformMatrix4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix4dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glProgramUniformMatrix4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix4fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glProgramUniformMatrix4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix4fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glProgramUniformMatrix4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix4x2dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix4x2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix4x2dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix4x2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix4x2fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix4x2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix4x2fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glProgramUniformMatrix4x2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix4x3dv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix4x3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix4x3dvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix4x3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glProgramUniformMatrix4x3fv
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix4x3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformMatrix4x3fvEXT
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glProgramUniformMatrix4x3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glProgramUniformui64NV
/// * `program` class: program
pub type glProgramUniformui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64EXT);

/// glProgramUniformui64vNV
/// * `program` class: program
/// * `value` len: count
pub type glProgramUniformui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glProgramVertexLimitNV
/// * `target` group: ProgramTarget
pub type glProgramVertexLimitNV_t = unsafe extern "system" fn(target: ProgramTarget, limit: GLint);

/// glProvokingVertex
/// * `mode` group: VertexProvokingMode
pub type glProvokingVertex_t = unsafe extern "system" fn(mode: VertexProvokingMode);

/// glProvokingVertexEXT
/// * `mode` group: VertexProvokingMode
pub type glProvokingVertexEXT_t = unsafe extern "system" fn(mode: VertexProvokingMode);

/// glPushAttrib
/// * `mask` group: AttribMask
pub type glPushAttrib_t = unsafe extern "system" fn(mask: GLbitfield);

/// glPushClientAttrib
/// * `mask` group: ClientAttribMask
pub type glPushClientAttrib_t = unsafe extern "system" fn(mask: GLbitfield);

/// glPushClientAttribDefaultEXT
/// * `mask` group: ClientAttribMask
pub type glPushClientAttribDefaultEXT_t = unsafe extern "system" fn(mask: GLbitfield);

/// glPushDebugGroup
/// * `source` group: DebugSource
/// * `message` len: COMPSIZE(message,length)
pub type glPushDebugGroup_t = unsafe extern "system" fn(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar);

/// glPushDebugGroupKHR
/// * `source` group: DebugSource
pub type glPushDebugGroupKHR_t = unsafe extern "system" fn(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar);

/// glPushGroupMarkerEXT
pub type glPushGroupMarkerEXT_t = unsafe extern "system" fn(length: GLsizei, marker: *const GLchar);

/// glPushMatrix
pub type glPushMatrix_t = unsafe extern "system" fn();

/// glPushName
/// * `name` group: SelectName
pub type glPushName_t = unsafe extern "system" fn(name: GLuint);

/// glQueryCounter
/// * `id` class: query
/// * `target` group: QueryCounterTarget
pub type glQueryCounter_t = unsafe extern "system" fn(id: GLuint, target: QueryCounterTarget);

/// glQueryCounterEXT
/// * `id` class: query
/// * `target` group: QueryCounterTarget
pub type glQueryCounterEXT_t = unsafe extern "system" fn(id: GLuint, target: QueryCounterTarget);

/// glQueryMatrixxOES
pub type glQueryMatrixxOES_t = unsafe extern "system" fn(mantissa: *mut [GLfixed; 16], exponent: *mut [GLint; 16]) -> GLbitfield;

/// glQueryObjectParameteruiAMD
/// * `target` group: QueryTarget
/// * `id` class: query
/// * `param` group: OcclusionQueryEventMaskAMD
pub type glQueryObjectParameteruiAMD_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint, pname: GLenum, param: GLuint);

/// glQueryResourceNV
/// * `buffer` len: count
pub type glQueryResourceNV_t = unsafe extern "system" fn(queryType: GLenum, tagId: GLint, count: GLuint, buffer: *mut GLint) -> GLint;

/// glQueryResourceTagNV
pub type glQueryResourceTagNV_t = unsafe extern "system" fn(tagId: GLint, tagString: *const GLchar);

/// glRasterPos2d
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glRasterPos2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// glRasterPos2dv
/// * `v` group: CoordD
pub type glRasterPos2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// glRasterPos2f
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glRasterPos2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// glRasterPos2fv
/// * `v` group: CoordF
pub type glRasterPos2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// glRasterPos2i
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glRasterPos2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// glRasterPos2iv
/// * `v` group: CoordI
pub type glRasterPos2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// glRasterPos2s
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glRasterPos2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// glRasterPos2sv
/// * `v` group: CoordS
pub type glRasterPos2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// glRasterPos2xOES
pub type glRasterPos2xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed);

/// glRasterPos2xvOES
pub type glRasterPos2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

/// glRasterPos3d
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glRasterPos3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// glRasterPos3dv
/// * `v` group: CoordD
pub type glRasterPos3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glRasterPos3f
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glRasterPos3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// glRasterPos3fv
/// * `v` group: CoordF
pub type glRasterPos3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glRasterPos3i
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glRasterPos3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// glRasterPos3iv
/// * `v` group: CoordI
pub type glRasterPos3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glRasterPos3s
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glRasterPos3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// glRasterPos3sv
/// * `v` group: CoordS
pub type glRasterPos3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glRasterPos3xOES
pub type glRasterPos3xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// glRasterPos3xvOES
pub type glRasterPos3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

/// glRasterPos4d
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
/// * `w` group: CoordD
pub type glRasterPos4d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glRasterPos4dv
/// * `v` group: CoordD
pub type glRasterPos4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// glRasterPos4f
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
/// * `w` group: CoordF
pub type glRasterPos4f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glRasterPos4fv
/// * `v` group: CoordF
pub type glRasterPos4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// glRasterPos4i
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
/// * `w` group: CoordI
pub type glRasterPos4i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

/// glRasterPos4iv
/// * `v` group: CoordI
pub type glRasterPos4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// glRasterPos4s
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
/// * `w` group: CoordS
pub type glRasterPos4s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// glRasterPos4sv
/// * `v` group: CoordS
pub type glRasterPos4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// glRasterPos4xOES
pub type glRasterPos4xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed, w: GLfixed);

/// glRasterPos4xvOES
pub type glRasterPos4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

/// glRasterSamplesEXT
/// * `fixedsamplelocations` group: Boolean
pub type glRasterSamplesEXT_t = unsafe extern "system" fn(samples: GLuint, fixedsamplelocations: GLboolean);

/// glReadBuffer
/// * `src` group: ReadBufferMode
pub type glReadBuffer_t = unsafe extern "system" fn(src: ReadBufferMode);

/// glReadBufferIndexedEXT
/// * `src` group: ReadBufferMode
pub type glReadBufferIndexedEXT_t = unsafe extern "system" fn(src: ReadBufferMode, index: GLint);

/// glReadBufferNV
pub type glReadBufferNV_t = unsafe extern "system" fn(mode: GLenum);

/// glReadInstrumentsSGIX
pub type glReadInstrumentsSGIX_t = unsafe extern "system" fn(marker: GLint);

/// glReadPixels
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glReadPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void);

/// glReadnPixels
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
pub type glReadnPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

/// glReadnPixelsARB
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
pub type glReadnPixelsARB_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

/// glReadnPixelsEXT
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
pub type glReadnPixelsEXT_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

/// glReadnPixelsKHR
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
pub type glReadnPixelsKHR_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

/// glRectd
/// * `x1` group: CoordD
/// * `y1` group: CoordD
/// * `x2` group: CoordD
/// * `y2` group: CoordD
pub type glRectd_t = unsafe extern "system" fn(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble);

/// glRectdv
/// * `v1` group: CoordD
/// * `v2` group: CoordD
pub type glRectdv_t = unsafe extern "system" fn(v1: *const [GLdouble; 2], v2: *const [GLdouble; 2]);

/// glRectf
/// * `x1` group: CoordF
/// * `y1` group: CoordF
/// * `x2` group: CoordF
/// * `y2` group: CoordF
pub type glRectf_t = unsafe extern "system" fn(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat);

/// glRectfv
/// * `v1` group: CoordF
/// * `v2` group: CoordF
pub type glRectfv_t = unsafe extern "system" fn(v1: *const [GLfloat; 2], v2: *const [GLfloat; 2]);

/// glRecti
/// * `x1` group: CoordI
/// * `y1` group: CoordI
/// * `x2` group: CoordI
/// * `y2` group: CoordI
pub type glRecti_t = unsafe extern "system" fn(x1: GLint, y1: GLint, x2: GLint, y2: GLint);

/// glRectiv
/// * `v1` group: CoordI
/// * `v2` group: CoordI
pub type glRectiv_t = unsafe extern "system" fn(v1: *const [GLint; 2], v2: *const [GLint; 2]);

/// glRects
/// * `x1` group: CoordS
/// * `y1` group: CoordS
/// * `x2` group: CoordS
/// * `y2` group: CoordS
pub type glRects_t = unsafe extern "system" fn(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort);

/// glRectsv
/// * `v1` group: CoordS
/// * `v2` group: CoordS
pub type glRectsv_t = unsafe extern "system" fn(v1: *const [GLshort; 2], v2: *const [GLshort; 2]);

/// glRectxOES
pub type glRectxOES_t = unsafe extern "system" fn(x1: GLfixed, y1: GLfixed, x2: GLfixed, y2: GLfixed);

/// glRectxvOES
pub type glRectxvOES_t = unsafe extern "system" fn(v1: *const [GLfixed; 2], v2: *const [GLfixed; 2]);

/// glReferencePlaneSGIX
pub type glReferencePlaneSGIX_t = unsafe extern "system" fn(equation: *const [GLdouble; 4]);

/// glReleaseKeyedMutexWin32EXT
pub type glReleaseKeyedMutexWin32EXT_t = unsafe extern "system" fn(memory: GLuint, key: GLuint64) -> GLboolean;

/// glReleaseShaderCompiler
pub type glReleaseShaderCompiler_t = unsafe extern "system" fn();

/// glRenderGpuMaskNV
pub type glRenderGpuMaskNV_t = unsafe extern "system" fn(mask: GLbitfield);

/// glRenderMode
/// * `mode` group: RenderingMode
pub type glRenderMode_t = unsafe extern "system" fn(mode: RenderingMode) -> GLint;

/// glRenderbufferStorage
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorage_t = unsafe extern "system" fn(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageEXT
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageEXT_t = unsafe extern "system" fn(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageMultisample
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisample_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageMultisampleANGLE
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleANGLE_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageMultisampleAPPLE
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleAPPLE_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageMultisampleAdvancedAMD
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleAdvancedAMD_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, storageSamples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageMultisampleCoverageNV
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleCoverageNV_t = unsafe extern "system" fn(target: RenderbufferTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageMultisampleEXT
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleEXT_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageMultisampleIMG
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleIMG_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageMultisampleNV
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageMultisampleNV_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glRenderbufferStorageOES
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
pub type glRenderbufferStorageOES_t = unsafe extern "system" fn(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glReplacementCodePointerSUN
/// * `type` group: ReplacementCodeTypeSUN
/// * `pointer` len: COMPSIZE(type,stride)
pub type glReplacementCodePointerSUN_t = unsafe extern "system" fn(type_: ReplacementCodeTypeSUN, stride: GLsizei, pointer: *const *mut void);

/// glReplacementCodeubSUN
pub type glReplacementCodeubSUN_t = unsafe extern "system" fn(code: GLubyte);

/// glReplacementCodeubvSUN
/// * `code` len: COMPSIZE()
pub type glReplacementCodeubvSUN_t = unsafe extern "system" fn(code: *const GLubyte);

/// glReplacementCodeuiColor3fVertex3fSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glReplacementCodeuiColor3fVertex3fvSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glReplacementCodeuiColor4fNormal3fVertex3fSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glReplacementCodeuiColor4fNormal3fVertex3fvSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glReplacementCodeuiColor4ubVertex3fSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor4ubVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

/// glReplacementCodeuiColor4ubVertex3fvSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiColor4ubVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

/// glReplacementCodeuiNormal3fVertex3fSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glReplacementCodeuiNormal3fVertex3fvSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glReplacementCodeuiSUN
pub type glReplacementCodeuiSUN_t = unsafe extern "system" fn(code: GLuint);

/// glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glReplacementCodeuiTexCoord2fVertex3fSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glReplacementCodeuiTexCoord2fVertex3fvSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiTexCoord2fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], v: *const [GLfloat; 3]);

/// glReplacementCodeuiVertex3fSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

/// glReplacementCodeuiVertex3fvSUN
/// * `rc` group: ReplacementCodeSUN
pub type glReplacementCodeuiVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, v: *const [GLfloat; 3]);

/// glReplacementCodeuivSUN
/// * `code` len: COMPSIZE()
pub type glReplacementCodeuivSUN_t = unsafe extern "system" fn(code: *const GLuint);

/// glReplacementCodeusSUN
pub type glReplacementCodeusSUN_t = unsafe extern "system" fn(code: GLushort);

/// glReplacementCodeusvSUN
/// * `code` len: COMPSIZE()
pub type glReplacementCodeusvSUN_t = unsafe extern "system" fn(code: *const GLushort);

/// glRequestResidentProgramsNV
/// * `programs` class: program
/// * `programs` len: n
pub type glRequestResidentProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

/// glResetHistogram
/// * `target` group: HistogramTargetEXT
pub type glResetHistogram_t = unsafe extern "system" fn(target: HistogramTargetEXT);

/// glResetHistogramEXT
/// * `target` group: HistogramTargetEXT
pub type glResetHistogramEXT_t = unsafe extern "system" fn(target: HistogramTargetEXT);

/// glResetMemoryObjectParameterNV
pub type glResetMemoryObjectParameterNV_t = unsafe extern "system" fn(memory: GLuint, pname: GLenum);

/// glResetMinmax
/// * `target` group: MinmaxTargetEXT
pub type glResetMinmax_t = unsafe extern "system" fn(target: MinmaxTargetEXT);

/// glResetMinmaxEXT
/// * `target` group: MinmaxTargetEXT
pub type glResetMinmaxEXT_t = unsafe extern "system" fn(target: MinmaxTargetEXT);

/// glResizeBuffersMESA
pub type glResizeBuffersMESA_t = unsafe extern "system" fn();

/// glResolveDepthValuesNV
pub type glResolveDepthValuesNV_t = unsafe extern "system" fn();

/// glResolveMultisampleFramebufferAPPLE
pub type glResolveMultisampleFramebufferAPPLE_t = unsafe extern "system" fn();

/// glResumeTransformFeedback
pub type glResumeTransformFeedback_t = unsafe extern "system" fn();

/// glResumeTransformFeedbackNV
pub type glResumeTransformFeedbackNV_t = unsafe extern "system" fn();

/// glRotated
pub type glRotated_t = unsafe extern "system" fn(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);

/// glRotatef
pub type glRotatef_t = unsafe extern "system" fn(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glRotatex
pub type glRotatex_t = unsafe extern "system" fn(angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);

/// glRotatexOES
pub type glRotatexOES_t = unsafe extern "system" fn(angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);

/// glSampleCoverage
/// * `invert` group: Boolean
pub type glSampleCoverage_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);

/// glSampleCoverageARB
/// * `invert` group: Boolean
pub type glSampleCoverageARB_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);

/// glSampleCoveragex
/// * `invert` group: Boolean
pub type glSampleCoveragex_t = unsafe extern "system" fn(value: GLclampx, invert: GLboolean);

/// glSampleCoveragexOES
/// * `invert` group: Boolean
pub type glSampleCoveragexOES_t = unsafe extern "system" fn(value: GLclampx, invert: GLboolean);

/// glSampleMapATI
/// * `swizzle` group: SwizzleOpATI
pub type glSampleMapATI_t = unsafe extern "system" fn(dst: GLuint, interp: GLuint, swizzle: SwizzleOpATI);

/// glSampleMaskEXT
/// * `value` group: ClampedFloat32
/// * `invert` group: Boolean
pub type glSampleMaskEXT_t = unsafe extern "system" fn(value: GLclampf, invert: GLboolean);

/// glSampleMaskIndexedNV
/// * `mask` group: SampleMaskNV
pub type glSampleMaskIndexedNV_t = unsafe extern "system" fn(index: GLuint, mask: GLbitfield);

/// glSampleMaskSGIS
/// * `value` group: ClampedFloat32
/// * `invert` group: Boolean
pub type glSampleMaskSGIS_t = unsafe extern "system" fn(value: GLclampf, invert: GLboolean);

/// glSampleMaski
pub type glSampleMaski_t = unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield);

/// glSamplePatternEXT
/// * `pattern` group: SamplePatternEXT
pub type glSamplePatternEXT_t = unsafe extern "system" fn(pattern: SamplePatternEXT);

/// glSamplePatternSGIS
/// * `pattern` group: SamplePatternSGIS
pub type glSamplePatternSGIS_t = unsafe extern "system" fn(pattern: SamplePatternSGIS);

/// glSamplerParameterIiv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

/// glSamplerParameterIivEXT
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

/// glSamplerParameterIivOES
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

/// glSamplerParameterIuiv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);

/// glSamplerParameterIuivEXT
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIuivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);

/// glSamplerParameterIuivOES
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterIuivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);

/// glSamplerParameterf
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
pub type glSamplerParameterf_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, param: GLfloat);

/// glSamplerParameterfv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat);

/// glSamplerParameteri
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
pub type glSamplerParameteri_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: GLint);

/// glSamplerParameteriv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
pub type glSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

/// glScaled
pub type glScaled_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// glScalef
pub type glScalef_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// glScalex
pub type glScalex_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// glScalexOES
pub type glScalexOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// glScissor
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glScissor_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glScissorArrayv
/// * `v` len: COMPSIZE(count)
pub type glScissorArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

/// glScissorArrayvNV
/// * `v` len: COMPSIZE(count)
pub type glScissorArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

/// glScissorArrayvOES
/// * `v` len: COMPSIZE(count)
pub type glScissorArrayvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

/// glScissorExclusiveArrayvNV
/// * `v` len: COMPSIZE(count)
pub type glScissorExclusiveArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

/// glScissorExclusiveNV
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glScissorExclusiveNV_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glScissorIndexed
pub type glScissorIndexed_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

/// glScissorIndexedNV
pub type glScissorIndexedNV_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

/// glScissorIndexedOES
pub type glScissorIndexedOES_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

/// glScissorIndexedv
pub type glScissorIndexedv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glScissorIndexedvNV
pub type glScissorIndexedvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glScissorIndexedvOES
pub type glScissorIndexedvOES_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glSecondaryColor3b
/// * `red` group: ColorB
/// * `green` group: ColorB
/// * `blue` group: ColorB
pub type glSecondaryColor3b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

/// glSecondaryColor3bEXT
/// * `red` group: ColorB
/// * `green` group: ColorB
/// * `blue` group: ColorB
pub type glSecondaryColor3bEXT_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

/// glSecondaryColor3bv
/// * `v` group: ColorB
pub type glSecondaryColor3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// glSecondaryColor3bvEXT
/// * `v` group: ColorB
pub type glSecondaryColor3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// glSecondaryColor3d
/// * `red` group: ColorD
/// * `green` group: ColorD
/// * `blue` group: ColorD
pub type glSecondaryColor3d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

/// glSecondaryColor3dEXT
/// * `red` group: ColorD
/// * `green` group: ColorD
/// * `blue` group: ColorD
pub type glSecondaryColor3dEXT_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

/// glSecondaryColor3dv
/// * `v` group: ColorD
pub type glSecondaryColor3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glSecondaryColor3dvEXT
/// * `v` group: ColorD
pub type glSecondaryColor3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glSecondaryColor3f
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
pub type glSecondaryColor3f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

/// glSecondaryColor3fEXT
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
pub type glSecondaryColor3fEXT_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

/// glSecondaryColor3fv
/// * `v` group: ColorF
pub type glSecondaryColor3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glSecondaryColor3fvEXT
/// * `v` group: ColorF
pub type glSecondaryColor3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glSecondaryColor3hNV
/// * `red` group: Half16NV
/// * `green` group: Half16NV
/// * `blue` group: Half16NV
pub type glSecondaryColor3hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV);

/// glSecondaryColor3hvNV
/// * `v` group: Half16NV
pub type glSecondaryColor3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// glSecondaryColor3i
/// * `red` group: ColorI
/// * `green` group: ColorI
/// * `blue` group: ColorI
pub type glSecondaryColor3i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

/// glSecondaryColor3iEXT
/// * `red` group: ColorI
/// * `green` group: ColorI
/// * `blue` group: ColorI
pub type glSecondaryColor3iEXT_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

/// glSecondaryColor3iv
/// * `v` group: ColorI
pub type glSecondaryColor3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glSecondaryColor3ivEXT
/// * `v` group: ColorI
pub type glSecondaryColor3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glSecondaryColor3s
/// * `red` group: ColorS
/// * `green` group: ColorS
/// * `blue` group: ColorS
pub type glSecondaryColor3s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

/// glSecondaryColor3sEXT
/// * `red` group: ColorS
/// * `green` group: ColorS
/// * `blue` group: ColorS
pub type glSecondaryColor3sEXT_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

/// glSecondaryColor3sv
/// * `v` group: ColorS
pub type glSecondaryColor3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glSecondaryColor3svEXT
/// * `v` group: ColorS
pub type glSecondaryColor3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glSecondaryColor3ub
/// * `red` group: ColorUB
/// * `green` group: ColorUB
/// * `blue` group: ColorUB
pub type glSecondaryColor3ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

/// glSecondaryColor3ubEXT
/// * `red` group: ColorUB
/// * `green` group: ColorUB
/// * `blue` group: ColorUB
pub type glSecondaryColor3ubEXT_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

/// glSecondaryColor3ubv
/// * `v` group: ColorUB
pub type glSecondaryColor3ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

/// glSecondaryColor3ubvEXT
/// * `v` group: ColorUB
pub type glSecondaryColor3ubvEXT_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

/// glSecondaryColor3ui
/// * `red` group: ColorUI
/// * `green` group: ColorUI
/// * `blue` group: ColorUI
pub type glSecondaryColor3ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

/// glSecondaryColor3uiEXT
/// * `red` group: ColorUI
/// * `green` group: ColorUI
/// * `blue` group: ColorUI
pub type glSecondaryColor3uiEXT_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

/// glSecondaryColor3uiv
/// * `v` group: ColorUI
pub type glSecondaryColor3uiv_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

/// glSecondaryColor3uivEXT
/// * `v` group: ColorUI
pub type glSecondaryColor3uivEXT_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

/// glSecondaryColor3us
/// * `red` group: ColorUS
/// * `green` group: ColorUS
/// * `blue` group: ColorUS
pub type glSecondaryColor3us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

/// glSecondaryColor3usEXT
/// * `red` group: ColorUS
/// * `green` group: ColorUS
/// * `blue` group: ColorUS
pub type glSecondaryColor3usEXT_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

/// glSecondaryColor3usv
/// * `v` group: ColorUS
pub type glSecondaryColor3usv_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

/// glSecondaryColor3usvEXT
/// * `v` group: ColorUS
pub type glSecondaryColor3usvEXT_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

/// glSecondaryColorFormatNV
/// * `type` group: ColorPointerType
pub type glSecondaryColorFormatNV_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei);

/// glSecondaryColorP3ui
/// * `type` group: ColorPointerType
pub type glSecondaryColorP3ui_t = unsafe extern "system" fn(type_: ColorPointerType, color: GLuint);

/// glSecondaryColorP3uiv
/// * `type` group: ColorPointerType
pub type glSecondaryColorP3uiv_t = unsafe extern "system" fn(type_: ColorPointerType, color: *const GLuint);

/// glSecondaryColorPointer
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glSecondaryColorPointer_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei, pointer: *const void);

/// glSecondaryColorPointerEXT
/// * `type` group: ColorPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glSecondaryColorPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: ColorPointerType, stride: GLsizei, pointer: *const void);

/// glSecondaryColorPointerListIBM
/// * `type` group: SecondaryColorPointerTypeIBM
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glSecondaryColorPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: SecondaryColorPointerTypeIBM, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// glSelectBuffer
/// * `buffer` group: SelectName
/// * `buffer` len: size
pub type glSelectBuffer_t = unsafe extern "system" fn(size: GLsizei, buffer: *mut GLuint);

/// glSelectPerfMonitorCountersAMD
/// * `enable` group: Boolean
/// * `counterList` len: numCounters
pub type glSelectPerfMonitorCountersAMD_t = unsafe extern "system" fn(monitor: GLuint, enable: GLboolean, group: GLuint, numCounters: GLint, counterList: *mut GLuint);

/// glSemaphoreParameterivNV
/// * `pname` group: SemaphoreParameterName
pub type glSemaphoreParameterivNV_t = unsafe extern "system" fn(semaphore: GLuint, pname: SemaphoreParameterName, params: *const GLint);

/// glSemaphoreParameterui64vEXT
/// * `pname` group: SemaphoreParameterName
pub type glSemaphoreParameterui64vEXT_t = unsafe extern "system" fn(semaphore: GLuint, pname: SemaphoreParameterName, params: *const GLuint64);

/// glSeparableFilter2D
/// * `target` group: SeparableTargetEXT
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: COMPSIZE(target,format,type,width)
/// * `column` len: COMPSIZE(target,format,type,height)
pub type glSeparableFilter2D_t = unsafe extern "system" fn(target: SeparableTargetEXT, internalformat: InternalFormat, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, row: *const void, column: *const void);

/// glSeparableFilter2DEXT
/// * `target` group: SeparableTargetEXT
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: COMPSIZE(target,format,type,width)
/// * `column` len: COMPSIZE(target,format,type,height)
pub type glSeparableFilter2DEXT_t = unsafe extern "system" fn(target: SeparableTargetEXT, internalformat: InternalFormat, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, row: *const void, column: *const void);

/// glSetFenceAPPLE
/// * `fence` group: FenceNV
pub type glSetFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint);

/// glSetFenceNV
/// * `fence` group: FenceNV
/// * `condition` group: FenceConditionNV
pub type glSetFenceNV_t = unsafe extern "system" fn(fence: GLuint, condition: FenceConditionNV);

/// glSetFragmentShaderConstantATI
pub type glSetFragmentShaderConstantATI_t = unsafe extern "system" fn(dst: GLuint, value: *const [GLfloat; 4]);

/// glSetInvariantEXT
/// * `type` group: ScalarType
/// * `addr` len: COMPSIZE(id,type)
pub type glSetInvariantEXT_t = unsafe extern "system" fn(id: GLuint, type_: ScalarType, addr: *const void);

/// glSetLocalConstantEXT
/// * `type` group: ScalarType
/// * `addr` len: COMPSIZE(id,type)
pub type glSetLocalConstantEXT_t = unsafe extern "system" fn(id: GLuint, type_: ScalarType, addr: *const void);

/// glSetMultisamplefvAMD
pub type glSetMultisamplefvAMD_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *const [GLfloat; 2]);

/// glShadeModel
/// * `mode` group: ShadingModel
pub type glShadeModel_t = unsafe extern "system" fn(mode: ShadingModel);

/// glShaderBinary
/// * `shaders` class: shader
/// * `shaders` len: count
/// * `binaryFormat` group: ShaderBinaryFormat
/// * `binary` len: length
pub type glShaderBinary_t = unsafe extern "system" fn(count: GLsizei, shaders: *const GLuint, binaryFormat: ShaderBinaryFormat, binary: *const void, length: GLsizei);

/// glShaderOp1EXT
/// * `op` group: VertexShaderOpEXT
pub type glShaderOp1EXT_t = unsafe extern "system" fn(op: VertexShaderOpEXT, res: GLuint, arg1: GLuint);

/// glShaderOp2EXT
/// * `op` group: VertexShaderOpEXT
pub type glShaderOp2EXT_t = unsafe extern "system" fn(op: VertexShaderOpEXT, res: GLuint, arg1: GLuint, arg2: GLuint);

/// glShaderOp3EXT
/// * `op` group: VertexShaderOpEXT
pub type glShaderOp3EXT_t = unsafe extern "system" fn(op: VertexShaderOpEXT, res: GLuint, arg1: GLuint, arg2: GLuint, arg3: GLuint);

/// glShaderSource
/// * `shader` class: shader
/// * `string` len: count
/// * `length` len: count
pub type glShaderSource_t = unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);

/// glShaderSourceARB
/// * `shaderObj` group: handleARB
/// * `string` len: count
/// * `length` len: count
pub type glShaderSourceARB_t = unsafe extern "system" fn(shaderObj: GLhandleARB, count: GLsizei, string: *const *mut GLcharARB, length: *const GLint);

/// glShaderStorageBlockBinding
/// * `program` class: program
pub type glShaderStorageBlockBinding_t = unsafe extern "system" fn(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);

/// glShadingRateImageBarrierNV
/// * `synchronize` group: Boolean
pub type glShadingRateImageBarrierNV_t = unsafe extern "system" fn(synchronize: GLboolean);

/// glShadingRateImagePaletteNV
/// * `rates` len: count
pub type glShadingRateImagePaletteNV_t = unsafe extern "system" fn(viewport: GLuint, first: GLuint, count: GLsizei, rates: *const GLenum);

/// glShadingRateQCOM
/// * `rate` group: ShadingRateQCOM
pub type glShadingRateQCOM_t = unsafe extern "system" fn(rate: ShadingRateQCOM);

/// glShadingRateSampleOrderCustomNV
/// * `locations` len: COMPSIZE(rate,samples)
pub type glShadingRateSampleOrderCustomNV_t = unsafe extern "system" fn(rate: GLenum, samples: GLuint, locations: *const GLint);

/// glShadingRateSampleOrderNV
pub type glShadingRateSampleOrderNV_t = unsafe extern "system" fn(order: GLenum);

/// glSharpenTexFuncSGIS
/// * `target` group: TextureTarget
/// * `points` len: n*2
pub type glSharpenTexFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, n: GLsizei, points: *const GLfloat);

/// glSignalSemaphoreEXT
/// * `buffers` class: buffer
/// * `buffers` len: COMPSIZE(numBufferBarriers)
/// * `textures` class: texture
/// * `textures` len: COMPSIZE(numTextureBarriers)
/// * `dstLayouts` group: TextureLayout
/// * `dstLayouts` len: COMPSIZE(numTextureBarriers)
pub type glSignalSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint, numBufferBarriers: GLuint, buffers: *const GLuint, numTextureBarriers: GLuint, textures: *const GLuint, dstLayouts: *const TextureLayout);

/// glSignalSemaphoreui64NVX
/// * `semaphoreArray` len: fenceObjectCount
/// * `fenceValueArray` len: fenceObjectCount
pub type glSignalSemaphoreui64NVX_t = unsafe extern "system" fn(signalGpu: GLuint, fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

/// glSignalVkFenceNV
pub type glSignalVkFenceNV_t = unsafe extern "system" fn(vkFence: GLuint64);

/// glSignalVkSemaphoreNV
pub type glSignalVkSemaphoreNV_t = unsafe extern "system" fn(vkSemaphore: GLuint64);

/// glSpecializeShader
/// * `shader` class: shader
pub type glSpecializeShader_t = unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);

/// glSpecializeShaderARB
/// * `shader` class: shader
pub type glSpecializeShaderARB_t = unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);

/// glSpriteParameterfSGIX
/// * `pname` group: SpriteParameterNameSGIX
/// * `param` group: CheckedFloat32
pub type glSpriteParameterfSGIX_t = unsafe extern "system" fn(pname: SpriteParameterNameSGIX, param: GLfloat);

/// glSpriteParameterfvSGIX
/// * `pname` group: SpriteParameterNameSGIX
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glSpriteParameterfvSGIX_t = unsafe extern "system" fn(pname: SpriteParameterNameSGIX, params: *const GLfloat);

/// glSpriteParameteriSGIX
/// * `pname` group: SpriteParameterNameSGIX
/// * `param` group: CheckedInt32
pub type glSpriteParameteriSGIX_t = unsafe extern "system" fn(pname: SpriteParameterNameSGIX, param: GLint);

/// glSpriteParameterivSGIX
/// * `pname` group: SpriteParameterNameSGIX
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glSpriteParameterivSGIX_t = unsafe extern "system" fn(pname: SpriteParameterNameSGIX, params: *const GLint);

/// glStartInstrumentsSGIX
pub type glStartInstrumentsSGIX_t = unsafe extern "system" fn();

/// glStartTilingQCOM
/// * `preserveMask` group: BufferBitQCOM
pub type glStartTilingQCOM_t = unsafe extern "system" fn(x: GLuint, y: GLuint, width: GLuint, height: GLuint, preserveMask: GLbitfield);

/// glStateCaptureNV
pub type glStateCaptureNV_t = unsafe extern "system" fn(state: GLuint, mode: GLenum);

/// glStencilClearTagEXT
pub type glStencilClearTagEXT_t = unsafe extern "system" fn(stencilTagBits: GLsizei, stencilClearTag: GLuint);

/// glStencilFillPathInstancedNV
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `fillMode` group: PathFillMode
/// * `mask` group: MaskedStencilValue
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(numPaths,transformType)
pub type glStencilFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, fillMode: PathFillMode, mask: GLuint, transformType: PathTransformType, transformValues: *const GLfloat);

/// glStencilFillPathNV
/// * `path` group: Path
/// * `fillMode` group: PathFillMode
/// * `mask` group: MaskedStencilValue
pub type glStencilFillPathNV_t = unsafe extern "system" fn(path: GLuint, fillMode: PathFillMode, mask: GLuint);

/// glStencilFunc
/// * `func` group: StencilFunction
/// * `ref` group: StencilValue
/// * `mask` group: MaskedStencilValue
pub type glStencilFunc_t = unsafe extern "system" fn(func: StencilFunction, ref_: GLint, mask: GLuint);

/// glStencilFuncSeparate
/// * `face` group: StencilFaceDirection
/// * `func` group: StencilFunction
/// * `ref` group: StencilValue
/// * `mask` group: MaskedStencilValue
pub type glStencilFuncSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint);

/// glStencilFuncSeparateATI
/// * `frontfunc` group: StencilFunction
/// * `backfunc` group: StencilFunction
/// * `ref` group: ClampedStencilValue
/// * `mask` group: MaskedStencilValue
pub type glStencilFuncSeparateATI_t = unsafe extern "system" fn(frontfunc: StencilFunction, backfunc: StencilFunction, ref_: GLint, mask: GLuint);

/// glStencilMask
/// * `mask` group: MaskedStencilValue
pub type glStencilMask_t = unsafe extern "system" fn(mask: GLuint);

/// glStencilMaskSeparate
/// * `face` group: StencilFaceDirection
/// * `mask` group: MaskedStencilValue
pub type glStencilMaskSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, mask: GLuint);

/// glStencilOp
/// * `fail` group: StencilOp
/// * `zfail` group: StencilOp
/// * `zpass` group: StencilOp
pub type glStencilOp_t = unsafe extern "system" fn(fail: StencilOp, zfail: StencilOp, zpass: StencilOp);

/// glStencilOpSeparate
/// * `face` group: StencilFaceDirection
/// * `sfail` group: StencilOp
/// * `dpfail` group: StencilOp
/// * `dppass` group: StencilOp
pub type glStencilOpSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp);

/// glStencilOpSeparateATI
/// * `face` group: StencilFaceDirection
/// * `sfail` group: StencilOp
/// * `dpfail` group: StencilOp
/// * `dppass` group: StencilOp
pub type glStencilOpSeparateATI_t = unsafe extern "system" fn(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp);

/// glStencilOpValueAMD
/// * `face` group: StencilFaceDirection
pub type glStencilOpValueAMD_t = unsafe extern "system" fn(face: StencilFaceDirection, value: GLuint);

/// glStencilStrokePathInstancedNV
/// * `pathNameType` group: PathElementType
/// * `paths` group: PathElement
/// * `paths` len: COMPSIZE(numPaths,pathNameType,paths)
/// * `pathBase` group: Path
/// * `reference` group: StencilValue
/// * `mask` group: MaskedStencilValue
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(numPaths,transformType)
pub type glStencilStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: PathElementType, paths: *const void, pathBase: GLuint, reference: GLint, mask: GLuint, transformType: PathTransformType, transformValues: *const GLfloat);

/// glStencilStrokePathNV
/// * `path` group: Path
/// * `reference` group: StencilValue
/// * `mask` group: MaskedStencilValue
pub type glStencilStrokePathNV_t = unsafe extern "system" fn(path: GLuint, reference: GLint, mask: GLuint);

/// glStencilThenCoverFillPathInstancedNV
pub type glStencilThenCoverFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, fillMode: GLenum, mask: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);

/// glStencilThenCoverFillPathNV
pub type glStencilThenCoverFillPathNV_t = unsafe extern "system" fn(path: GLuint, fillMode: GLenum, mask: GLuint, coverMode: GLenum);

/// glStencilThenCoverStrokePathInstancedNV
pub type glStencilThenCoverStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, reference: GLint, mask: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);

/// glStencilThenCoverStrokePathNV
pub type glStencilThenCoverStrokePathNV_t = unsafe extern "system" fn(path: GLuint, reference: GLint, mask: GLuint, coverMode: GLenum);

/// glStopInstrumentsSGIX
pub type glStopInstrumentsSGIX_t = unsafe extern "system" fn(marker: GLint);

/// glStringMarkerGREMEDY
/// * `string` len: len
pub type glStringMarkerGREMEDY_t = unsafe extern "system" fn(len: GLsizei, string: *const void);

/// glSubpixelPrecisionBiasNV
pub type glSubpixelPrecisionBiasNV_t = unsafe extern "system" fn(xbits: GLuint, ybits: GLuint);

/// glSwizzleEXT
/// * `outX` group: VertexShaderCoordOutEXT
/// * `outY` group: VertexShaderCoordOutEXT
/// * `outZ` group: VertexShaderCoordOutEXT
/// * `outW` group: VertexShaderCoordOutEXT
pub type glSwizzleEXT_t = unsafe extern "system" fn(res: GLuint, in_: GLuint, outX: VertexShaderCoordOutEXT, outY: VertexShaderCoordOutEXT, outZ: VertexShaderCoordOutEXT, outW: VertexShaderCoordOutEXT);

/// glSyncTextureINTEL
/// * `texture` class: texture
pub type glSyncTextureINTEL_t = unsafe extern "system" fn(texture: GLuint);

/// glTagSampleBufferSGIX
pub type glTagSampleBufferSGIX_t = unsafe extern "system" fn();

/// glTangent3bEXT
pub type glTangent3bEXT_t = unsafe extern "system" fn(tx: GLbyte, ty: GLbyte, tz: GLbyte);

/// glTangent3bvEXT
pub type glTangent3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

/// glTangent3dEXT
/// * `tx` group: CoordD
/// * `ty` group: CoordD
/// * `tz` group: CoordD
pub type glTangent3dEXT_t = unsafe extern "system" fn(tx: GLdouble, ty: GLdouble, tz: GLdouble);

/// glTangent3dvEXT
/// * `v` group: CoordD
pub type glTangent3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glTangent3fEXT
/// * `tx` group: CoordF
/// * `ty` group: CoordF
/// * `tz` group: CoordF
pub type glTangent3fEXT_t = unsafe extern "system" fn(tx: GLfloat, ty: GLfloat, tz: GLfloat);

/// glTangent3fvEXT
/// * `v` group: CoordF
pub type glTangent3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glTangent3iEXT
pub type glTangent3iEXT_t = unsafe extern "system" fn(tx: GLint, ty: GLint, tz: GLint);

/// glTangent3ivEXT
pub type glTangent3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glTangent3sEXT
pub type glTangent3sEXT_t = unsafe extern "system" fn(tx: GLshort, ty: GLshort, tz: GLshort);

/// glTangent3svEXT
pub type glTangent3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glTangentPointerEXT
/// * `type` group: TangentPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glTangentPointerEXT_t = unsafe extern "system" fn(type_: TangentPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// glTbufferMask3DFX
pub type glTbufferMask3DFX_t = unsafe extern "system" fn(mask: GLuint);

/// glTessellationFactorAMD
pub type glTessellationFactorAMD_t = unsafe extern "system" fn(factor: GLfloat);

/// glTessellationModeAMD
pub type glTessellationModeAMD_t = unsafe extern "system" fn(mode: GLenum);

/// glTestFenceAPPLE
/// * `fence` group: FenceNV
pub type glTestFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

/// glTestFenceNV
/// * `fence` group: FenceNV
pub type glTestFenceNV_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

/// glTestObjectAPPLE
/// * `object` group: ObjectTypeAPPLE
pub type glTestObjectAPPLE_t = unsafe extern "system" fn(object: ObjectTypeAPPLE, name: GLuint) -> GLboolean;

/// glTexAttachMemoryNV
/// * `target` group: TextureTarget
pub type glTexAttachMemoryNV_t = unsafe extern "system" fn(target: TextureTarget, memory: GLuint, offset: GLuint64);

/// glTexBuffer
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTexBuffer_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// glTexBufferARB
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTexBufferARB_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// glTexBufferEXT
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTexBufferEXT_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// glTexBufferOES
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTexBufferOES_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// glTexBufferRange
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glTexBufferRange_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glTexBufferRangeEXT
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glTexBufferRangeEXT_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glTexBufferRangeOES
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glTexBufferRangeOES_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glTexBumpParameterfvATI
/// * `pname` group: TexBumpParameterATI
/// * `param` len: COMPSIZE(pname)
pub type glTexBumpParameterfvATI_t = unsafe extern "system" fn(pname: TexBumpParameterATI, param: *const GLfloat);

/// glTexBumpParameterivATI
/// * `pname` group: TexBumpParameterATI
/// * `param` len: COMPSIZE(pname)
pub type glTexBumpParameterivATI_t = unsafe extern "system" fn(pname: TexBumpParameterATI, param: *const GLint);

/// glTexCoord1bOES
pub type glTexCoord1bOES_t = unsafe extern "system" fn(s: GLbyte);

/// glTexCoord1bvOES
pub type glTexCoord1bvOES_t = unsafe extern "system" fn(coords: *const GLbyte);

/// glTexCoord1d
/// * `s` group: CoordD
pub type glTexCoord1d_t = unsafe extern "system" fn(s: GLdouble);

/// glTexCoord1dv
/// * `v` group: CoordD
pub type glTexCoord1dv_t = unsafe extern "system" fn(v: *const GLdouble);

/// glTexCoord1f
/// * `s` group: CoordF
pub type glTexCoord1f_t = unsafe extern "system" fn(s: GLfloat);

/// glTexCoord1fv
/// * `v` group: CoordF
pub type glTexCoord1fv_t = unsafe extern "system" fn(v: *const GLfloat);

/// glTexCoord1hNV
/// * `s` group: Half16NV
pub type glTexCoord1hNV_t = unsafe extern "system" fn(s: GLhalfNV);

/// glTexCoord1hvNV
/// * `v` group: Half16NV
pub type glTexCoord1hvNV_t = unsafe extern "system" fn(v: *const GLhalfNV);

/// glTexCoord1i
/// * `s` group: CoordI
pub type glTexCoord1i_t = unsafe extern "system" fn(s: GLint);

/// glTexCoord1iv
/// * `v` group: CoordI
pub type glTexCoord1iv_t = unsafe extern "system" fn(v: *const GLint);

/// glTexCoord1s
/// * `s` group: CoordS
pub type glTexCoord1s_t = unsafe extern "system" fn(s: GLshort);

/// glTexCoord1sv
/// * `v` group: CoordS
pub type glTexCoord1sv_t = unsafe extern "system" fn(v: *const GLshort);

/// glTexCoord1xOES
pub type glTexCoord1xOES_t = unsafe extern "system" fn(s: GLfixed);

/// glTexCoord1xvOES
pub type glTexCoord1xvOES_t = unsafe extern "system" fn(coords: *const GLfixed);

/// glTexCoord2bOES
pub type glTexCoord2bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte);

/// glTexCoord2bvOES
pub type glTexCoord2bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 2]);

/// glTexCoord2d
/// * `s` group: CoordD
/// * `t` group: CoordD
pub type glTexCoord2d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble);

/// glTexCoord2dv
/// * `v` group: CoordD
pub type glTexCoord2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// glTexCoord2f
/// * `s` group: CoordF
/// * `t` group: CoordF
pub type glTexCoord2f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat);

/// glTexCoord2fColor3fVertex3fSUN
pub type glTexCoord2fColor3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glTexCoord2fColor3fVertex3fvSUN
pub type glTexCoord2fColor3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glTexCoord2fColor4fNormal3fVertex3fSUN
pub type glTexCoord2fColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glTexCoord2fColor4fNormal3fVertex3fvSUN
pub type glTexCoord2fColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glTexCoord2fColor4ubVertex3fSUN
pub type glTexCoord2fColor4ubVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

/// glTexCoord2fColor4ubVertex3fvSUN
pub type glTexCoord2fColor4ubVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

/// glTexCoord2fNormal3fVertex3fSUN
pub type glTexCoord2fNormal3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glTexCoord2fNormal3fVertex3fvSUN
pub type glTexCoord2fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

/// glTexCoord2fVertex3fSUN
pub type glTexCoord2fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

/// glTexCoord2fVertex3fvSUN
pub type glTexCoord2fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], v: *const [GLfloat; 3]);

/// glTexCoord2fv
/// * `v` group: CoordF
pub type glTexCoord2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// glTexCoord2hNV
/// * `s` group: Half16NV
/// * `t` group: Half16NV
pub type glTexCoord2hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV);

/// glTexCoord2hvNV
/// * `v` group: Half16NV
pub type glTexCoord2hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 2]);

/// glTexCoord2i
/// * `s` group: CoordI
/// * `t` group: CoordI
pub type glTexCoord2i_t = unsafe extern "system" fn(s: GLint, t: GLint);

/// glTexCoord2iv
/// * `v` group: CoordI
pub type glTexCoord2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// glTexCoord2s
/// * `s` group: CoordS
/// * `t` group: CoordS
pub type glTexCoord2s_t = unsafe extern "system" fn(s: GLshort, t: GLshort);

/// glTexCoord2sv
/// * `v` group: CoordS
pub type glTexCoord2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// glTexCoord2xOES
pub type glTexCoord2xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed);

/// glTexCoord2xvOES
pub type glTexCoord2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

/// glTexCoord3bOES
pub type glTexCoord3bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte, r: GLbyte);

/// glTexCoord3bvOES
pub type glTexCoord3bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 3]);

/// glTexCoord3d
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
pub type glTexCoord3d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble, r: GLdouble);

/// glTexCoord3dv
/// * `v` group: CoordD
pub type glTexCoord3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glTexCoord3f
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
pub type glTexCoord3f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat);

/// glTexCoord3fv
/// * `v` group: CoordF
pub type glTexCoord3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glTexCoord3hNV
/// * `s` group: Half16NV
/// * `t` group: Half16NV
/// * `r` group: Half16NV
pub type glTexCoord3hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV, r: GLhalfNV);

/// glTexCoord3hvNV
/// * `v` group: Half16NV
pub type glTexCoord3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// glTexCoord3i
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
pub type glTexCoord3i_t = unsafe extern "system" fn(s: GLint, t: GLint, r: GLint);

/// glTexCoord3iv
/// * `v` group: CoordI
pub type glTexCoord3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glTexCoord3s
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
pub type glTexCoord3s_t = unsafe extern "system" fn(s: GLshort, t: GLshort, r: GLshort);

/// glTexCoord3sv
/// * `v` group: CoordS
pub type glTexCoord3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glTexCoord3xOES
pub type glTexCoord3xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed, r: GLfixed);

/// glTexCoord3xvOES
pub type glTexCoord3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

/// glTexCoord4bOES
pub type glTexCoord4bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte, r: GLbyte, q: GLbyte);

/// glTexCoord4bvOES
pub type glTexCoord4bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 4]);

/// glTexCoord4d
/// * `s` group: CoordD
/// * `t` group: CoordD
/// * `r` group: CoordD
/// * `q` group: CoordD
pub type glTexCoord4d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

/// glTexCoord4dv
/// * `v` group: CoordD
pub type glTexCoord4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// glTexCoord4f
/// * `s` group: CoordF
/// * `t` group: CoordF
/// * `r` group: CoordF
/// * `q` group: CoordF
pub type glTexCoord4f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

/// glTexCoord4fColor4fNormal3fVertex4fSUN
pub type glTexCoord4fColor4fNormal3fVertex4fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, p: GLfloat, q: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glTexCoord4fColor4fNormal3fVertex4fvSUN
pub type glTexCoord4fColor4fNormal3fVertex4fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 4], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 4]);

/// glTexCoord4fVertex4fSUN
pub type glTexCoord4fVertex4fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, p: GLfloat, q: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glTexCoord4fVertex4fvSUN
pub type glTexCoord4fVertex4fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 4], v: *const [GLfloat; 4]);

/// glTexCoord4fv
/// * `v` group: CoordF
pub type glTexCoord4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// glTexCoord4hNV
/// * `s` group: Half16NV
/// * `t` group: Half16NV
/// * `r` group: Half16NV
/// * `q` group: Half16NV
pub type glTexCoord4hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV, r: GLhalfNV, q: GLhalfNV);

/// glTexCoord4hvNV
/// * `v` group: Half16NV
pub type glTexCoord4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

/// glTexCoord4i
/// * `s` group: CoordI
/// * `t` group: CoordI
/// * `r` group: CoordI
/// * `q` group: CoordI
pub type glTexCoord4i_t = unsafe extern "system" fn(s: GLint, t: GLint, r: GLint, q: GLint);

/// glTexCoord4iv
/// * `v` group: CoordI
pub type glTexCoord4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// glTexCoord4s
/// * `s` group: CoordS
/// * `t` group: CoordS
/// * `r` group: CoordS
/// * `q` group: CoordS
pub type glTexCoord4s_t = unsafe extern "system" fn(s: GLshort, t: GLshort, r: GLshort, q: GLshort);

/// glTexCoord4sv
/// * `v` group: CoordS
pub type glTexCoord4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// glTexCoord4xOES
pub type glTexCoord4xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

/// glTexCoord4xvOES
pub type glTexCoord4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

/// glTexCoordFormatNV
pub type glTexCoordFormatNV_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei);

/// glTexCoordP1ui
/// * `type` group: TexCoordPointerType
pub type glTexCoordP1ui_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: GLuint);

/// glTexCoordP1uiv
/// * `type` group: TexCoordPointerType
pub type glTexCoordP1uiv_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: *const GLuint);

/// glTexCoordP2ui
/// * `type` group: TexCoordPointerType
pub type glTexCoordP2ui_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: GLuint);

/// glTexCoordP2uiv
/// * `type` group: TexCoordPointerType
pub type glTexCoordP2uiv_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: *const GLuint);

/// glTexCoordP3ui
/// * `type` group: TexCoordPointerType
pub type glTexCoordP3ui_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: GLuint);

/// glTexCoordP3uiv
/// * `type` group: TexCoordPointerType
pub type glTexCoordP3uiv_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: *const GLuint);

/// glTexCoordP4ui
/// * `type` group: TexCoordPointerType
pub type glTexCoordP4ui_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: GLuint);

/// glTexCoordP4uiv
/// * `type` group: TexCoordPointerType
pub type glTexCoordP4uiv_t = unsafe extern "system" fn(type_: TexCoordPointerType, coords: *const GLuint);

/// glTexCoordPointer
/// * `type` group: TexCoordPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glTexCoordPointer_t = unsafe extern "system" fn(size: GLint, type_: TexCoordPointerType, stride: GLsizei, pointer: *const void);

/// glTexCoordPointerEXT
/// * `type` group: TexCoordPointerType
/// * `pointer` len: COMPSIZE(size,type,stride,count)
pub type glTexCoordPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: TexCoordPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// glTexCoordPointerListIBM
/// * `type` group: TexCoordPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glTexCoordPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: TexCoordPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// glTexCoordPointervINTEL
/// * `type` group: VertexPointerType
pub type glTexCoordPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, pointer: *const [*mut void; 4]);

/// glTexEnvf
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `param` group: CheckedFloat32
pub type glTexEnvf_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, param: GLfloat);

/// glTexEnvfv
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glTexEnvfv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLfloat);

/// glTexEnvi
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `param` group: CheckedInt32
pub type glTexEnvi_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, param: GLint);

/// glTexEnviv
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTexEnviv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLint);

/// glTexEnvx
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
pub type glTexEnvx_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, param: GLfixed);

/// glTexEnvxOES
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
pub type glTexEnvxOES_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, param: GLfixed);

/// glTexEnvxv
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexEnvxv_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLfixed);

/// glTexEnvxvOES
/// * `target` group: TextureEnvTarget
/// * `pname` group: TextureEnvParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexEnvxvOES_t = unsafe extern "system" fn(target: TextureEnvTarget, pname: TextureEnvParameter, params: *const GLfixed);

/// glTexEstimateMotionQCOM
/// * `ref` group: Texture
/// * `ref` class: texture
/// * `target` group: Texture
/// * `target` class: texture
/// * `output` group: Texture
/// * `output` class: texture
pub type glTexEstimateMotionQCOM_t = unsafe extern "system" fn(ref_: GLuint, target: GLuint, output: GLuint);

/// glTexEstimateMotionRegionsQCOM
/// * `ref` group: Texture
/// * `ref` class: texture
/// * `target` group: Texture
/// * `target` class: texture
/// * `output` group: Texture
/// * `output` class: texture
/// * `mask` group: Texture
/// * `mask` class: texture
pub type glTexEstimateMotionRegionsQCOM_t = unsafe extern "system" fn(ref_: GLuint, target: GLuint, output: GLuint, mask: GLuint);

/// glTexFilterFuncSGIS
/// * `target` group: TextureTarget
/// * `filter` group: TextureFilterSGIS
/// * `weights` len: n
pub type glTexFilterFuncSGIS_t = unsafe extern "system" fn(target: TextureTarget, filter: TextureFilterSGIS, n: GLsizei, weights: *const GLfloat);

/// glTexGend
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glTexGend_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLdouble);

/// glTexGendv
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexGendv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLdouble);

/// glTexGenf
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `param` group: CheckedFloat32
pub type glTexGenf_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLfloat);

/// glTexGenfOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glTexGenfOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLfloat);

/// glTexGenfv
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glTexGenfv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLfloat);

/// glTexGenfvOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexGenfvOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLfloat);

/// glTexGeni
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `param` group: CheckedInt32
pub type glTexGeni_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLint);

/// glTexGeniOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glTexGeniOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLint);

/// glTexGeniv
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTexGeniv_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLint);

/// glTexGenivOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexGenivOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLint);

/// glTexGenxOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
pub type glTexGenxOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, param: GLfixed);

/// glTexGenxvOES
/// * `coord` group: TextureCoordName
/// * `pname` group: TextureGenParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexGenxvOES_t = unsafe extern "system" fn(coord: TextureCoordName, pname: TextureGenParameter, params: *const GLfixed);

/// glTexImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexImage2DMultisample
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexImage2DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

/// glTexImage2DMultisampleCoverageNV
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTexImage2DMultisampleCoverageNV_t = unsafe extern "system" fn(target: TextureTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

/// glTexImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexImage3DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexImage3DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexImage3DMultisample
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexImage3DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// glTexImage3DMultisampleCoverageNV
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTexImage3DMultisampleCoverageNV_t = unsafe extern "system" fn(target: TextureTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

/// glTexImage3DOES
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexImage3DOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexImage4DSGIS
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth,size4d)
pub type glTexImage4DSGIS_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, size4d: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexPageCommitmentARB
/// * `commit` group: Boolean
pub type glTexPageCommitmentARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

/// glTexPageCommitmentEXT
/// * `commit` group: Boolean
pub type glTexPageCommitmentEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

/// glTexPageCommitmentMemNV
/// * `target` group: TextureTarget
/// * `commit` group: Boolean
pub type glTexPageCommitmentMemNV_t = unsafe extern "system" fn(target: TextureTarget, layer: GLint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64, commit: GLboolean);

/// glTexParameterIiv
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIiv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// glTexParameterIivEXT
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIivEXT_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// glTexParameterIivOES
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIivOES_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// glTexParameterIuiv
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIuiv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// glTexParameterIuivEXT
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIuivEXT_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// glTexParameterIuivOES
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterIuivOES_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// glTexParameterf
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedFloat32
pub type glTexParameterf_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, param: GLfloat);

/// glTexParameterfv
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterfv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);

/// glTexParameteri
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedInt32
pub type glTexParameteri_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, param: GLint);

/// glTexParameteriv
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTexParameteriv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// glTexParameterx
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
pub type glTexParameterx_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, param: GLfixed);

/// glTexParameterxOES
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
pub type glTexParameterxOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, param: GLfixed);

/// glTexParameterxv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterxv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *const GLfixed);

/// glTexParameterxvOES
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
pub type glTexParameterxvOES_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *const GLfixed);

/// glTexRenderbufferNV
/// * `target` group: TextureTarget
/// * `renderbuffer` class: renderbuffer
pub type glTexRenderbufferNV_t = unsafe extern "system" fn(target: TextureTarget, renderbuffer: GLuint);

/// glTexStorage1D
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage1D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

/// glTexStorage1DEXT
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage1DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

/// glTexStorage2D
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage2D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glTexStorage2DEXT
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage2DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glTexStorage2DMultisample
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexStorage2DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

/// glTexStorage3D
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage3D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

/// glTexStorage3DEXT
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
pub type glTexStorage3DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

/// glTexStorage3DMultisample
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexStorage3DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// glTexStorage3DMultisampleOES
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTexStorage3DMultisampleOES_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// glTexStorageMem1DEXT
/// * `target` group: TextureTarget
pub type glTexStorageMem1DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalFormat: GLenum, width: GLsizei, memory: GLuint, offset: GLuint64);

/// glTexStorageMem2DEXT
/// * `target` group: TextureTarget
pub type glTexStorageMem2DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, memory: GLuint, offset: GLuint64);

/// glTexStorageMem2DMultisampleEXT
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTexStorageMem2DMultisampleEXT_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

/// glTexStorageMem3DEXT
/// * `target` group: TextureTarget
pub type glTexStorageMem3DEXT_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64);

/// glTexStorageMem3DMultisampleEXT
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTexStorageMem3DMultisampleEXT_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

/// glTexStorageSparseAMD
/// * `target` group: TextureTarget
/// * `internalFormat` group: InternalFormat
/// * `flags` group: TextureStorageMaskAMD
pub type glTexStorageSparseAMD_t = unsafe extern "system" fn(target: TextureTarget, internalFormat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, layers: GLsizei, flags: GLbitfield);

/// glTexSubImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexSubImage1DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTexSubImage1DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexSubImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexSubImage2DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
pub type glTexSubImage2DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexSubImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexSubImage3DEXT
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexSubImage3DEXT_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexSubImage3DOES
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
pub type glTexSubImage3DOES_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTexSubImage4DSGIS
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

/// glTextureAttachMemoryNV
/// * `texture` class: texture
pub type glTextureAttachMemoryNV_t = unsafe extern "system" fn(texture: GLuint, memory: GLuint, offset: GLuint64);

/// glTextureBarrier
pub type glTextureBarrier_t = unsafe extern "system" fn();

/// glTextureBarrierNV
pub type glTextureBarrierNV_t = unsafe extern "system" fn();

/// glTextureBuffer
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTextureBuffer_t = unsafe extern "system" fn(texture: GLuint, internalformat: InternalFormat, buffer: GLuint);

/// glTextureBufferEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
pub type glTextureBufferEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

/// glTextureBufferRange
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `size` group: BufferSize
pub type glTextureBufferRange_t = unsafe extern "system" fn(texture: GLuint, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glTextureBufferRangeEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
pub type glTextureBufferRangeEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glTextureColorMaskSGIS
/// * `red` group: Boolean
/// * `green` group: Boolean
/// * `blue` group: Boolean
/// * `alpha` group: Boolean
pub type glTextureColorMaskSGIS_t = unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

/// glTextureFoveationParametersQCOM
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `focalX` group: CheckedFloat32
/// * `focalY` group: CheckedFloat32
/// * `gainX` group: CheckedFloat32
/// * `gainY` group: CheckedFloat32
/// * `foveaArea` group: CheckedFloat32
pub type glTextureFoveationParametersQCOM_t = unsafe extern "system" fn(texture: GLuint, layer: GLuint, focalPoint: GLuint, focalX: GLfloat, focalY: GLfloat, gainX: GLfloat, gainY: GLfloat, foveaArea: GLfloat);

/// glTextureImage1DEXT
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

/// glTextureImage2DEXT
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

/// glTextureImage2DMultisampleCoverageNV
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTextureImage2DMultisampleCoverageNV_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

/// glTextureImage2DMultisampleNV
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTextureImage2DMultisampleNV_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, samples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

/// glTextureImage3DEXT
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

/// glTextureImage3DMultisampleCoverageNV
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTextureImage3DMultisampleCoverageNV_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

/// glTextureImage3DMultisampleNV
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `fixedSampleLocations` group: Boolean
pub type glTextureImage3DMultisampleNV_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, samples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

/// glTextureLightEXT
/// * `pname` group: LightTexturePNameEXT
pub type glTextureLightEXT_t = unsafe extern "system" fn(pname: LightTexturePNameEXT);

/// glTextureMaterialEXT
/// * `face` group: MaterialFace
/// * `mode` group: MaterialParameter
pub type glTextureMaterialEXT_t = unsafe extern "system" fn(face: MaterialFace, mode: MaterialParameter);

/// glTextureNormalEXT
/// * `mode` group: TextureNormalModeEXT
pub type glTextureNormalEXT_t = unsafe extern "system" fn(mode: TextureNormalModeEXT);

/// glTexturePageCommitmentEXT
/// * `texture` class: texture
/// * `commit` group: Boolean
pub type glTexturePageCommitmentEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

/// glTexturePageCommitmentMemNV
/// * `texture` class: texture
/// * `commit` group: Boolean
pub type glTexturePageCommitmentMemNV_t = unsafe extern "system" fn(texture: GLuint, layer: GLint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64, commit: GLboolean);

/// glTextureParameterIiv
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameterIiv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, params: *const GLint);

/// glTextureParameterIivEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTextureParameterIivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// glTextureParameterIuiv
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameterIuiv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, params: *const GLuint);

/// glTextureParameterIuivEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
pub type glTextureParameterIuivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

/// glTextureParameterf
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameterf_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: GLfloat);

/// glTextureParameterfEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedFloat32
pub type glTextureParameterfEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, param: GLfloat);

/// glTextureParameterfv
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameterfv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: *const GLfloat);

/// glTextureParameterfvEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
pub type glTextureParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);

/// glTextureParameteri
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameteri_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: GLint);

/// glTextureParameteriEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedInt32
pub type glTextureParameteriEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, param: GLint);

/// glTextureParameteriv
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
pub type glTextureParameteriv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: *const GLint);

/// glTextureParameterivEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
pub type glTextureParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, pname: TextureParameterName, params: *const GLint);

/// glTextureRangeAPPLE
/// * `pointer` len: length
pub type glTextureRangeAPPLE_t = unsafe extern "system" fn(target: GLenum, length: GLsizei, pointer: *const void);

/// glTextureRenderbufferEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `renderbuffer` class: renderbuffer
pub type glTextureRenderbufferEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, renderbuffer: GLuint);

/// glTextureStorage1D
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage1D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

/// glTextureStorage1DEXT
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

/// glTextureStorage2D
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage2D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glTextureStorage2DEXT
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

/// glTextureStorage2DMultisample
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTextureStorage2DMultisample_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

/// glTextureStorage2DMultisampleEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTextureStorage2DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

/// glTextureStorage3D
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage3D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

/// glTextureStorage3DEXT
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureStorage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

/// glTextureStorage3DMultisample
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTextureStorage3DMultisample_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// glTextureStorage3DMultisampleEXT
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
pub type glTextureStorage3DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

/// glTextureStorageMem1DEXT
/// * `texture` class: texture
pub type glTextureStorageMem1DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, memory: GLuint, offset: GLuint64);

/// glTextureStorageMem2DEXT
/// * `texture` class: texture
pub type glTextureStorageMem2DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, memory: GLuint, offset: GLuint64);

/// glTextureStorageMem2DMultisampleEXT
/// * `texture` class: texture
/// * `fixedSampleLocations` group: Boolean
pub type glTextureStorageMem2DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

/// glTextureStorageMem3DEXT
/// * `texture` class: texture
pub type glTextureStorageMem3DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64);

/// glTextureStorageMem3DMultisampleEXT
/// * `texture` class: texture
/// * `fixedSampleLocations` group: Boolean
pub type glTextureStorageMem3DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

/// glTextureStorageSparseAMD
/// * `texture` class: texture
/// * `internalFormat` group: InternalFormat
/// * `flags` group: TextureStorageMaskAMD
pub type glTextureStorageSparseAMD_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, internalFormat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, layers: GLsizei, flags: GLbitfield);

/// glTextureSubImage1D
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTextureSubImage1DEXT
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
pub type glTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTextureSubImage2D
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTextureSubImage2DEXT
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

/// glTextureSubImage3D
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
pub type glTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

/// glTextureSubImage3DEXT
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

/// glTextureView
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `origtexture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureView_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

/// glTextureViewEXT
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `origtexture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureViewEXT_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

/// glTextureViewOES
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `origtexture` class: texture
/// * `internalformat` group: InternalFormat
pub type glTextureViewOES_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

/// glTrackMatrixNV
/// * `target` group: VertexAttribEnumNV
/// * `matrix` group: VertexAttribEnumNV
/// * `transform` group: VertexAttribEnumNV
pub type glTrackMatrixNV_t = unsafe extern "system" fn(target: VertexAttribEnumNV, address: GLuint, matrix: VertexAttribEnumNV, transform: VertexAttribEnumNV);

/// glTransformFeedbackAttribsNV
/// * `attribs` len: COMPSIZE(count)
pub type glTransformFeedbackAttribsNV_t = unsafe extern "system" fn(count: GLsizei, attribs: *const GLint, bufferMode: GLenum);

/// glTransformFeedbackBufferBase
/// * `xfb` class: transform feedback
/// * `buffer` class: buffer
pub type glTransformFeedbackBufferBase_t = unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint);

/// glTransformFeedbackBufferRange
/// * `xfb` class: transform feedback
/// * `buffer` class: buffer
/// * `size` group: BufferSize
pub type glTransformFeedbackBufferRange_t = unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

/// glTransformFeedbackStreamAttribsNV
/// * `attribs` len: count
/// * `bufstreams` len: nbuffers
pub type glTransformFeedbackStreamAttribsNV_t = unsafe extern "system" fn(count: GLsizei, attribs: *const GLint, nbuffers: GLsizei, bufstreams: *const GLint, bufferMode: GLenum);

/// glTransformFeedbackVaryings
/// * `program` class: program
/// * `varyings` len: count
/// * `bufferMode` group: TransformFeedbackBufferMode
pub type glTransformFeedbackVaryings_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode);

/// glTransformFeedbackVaryingsEXT
/// * `program` class: program
/// * `varyings` len: count
pub type glTransformFeedbackVaryingsEXT_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);

/// glTransformFeedbackVaryingsNV
/// * `program` class: program
/// * `locations` len: count
pub type glTransformFeedbackVaryingsNV_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, locations: *const GLint, bufferMode: GLenum);

/// glTransformPathNV
/// * `resultPath` group: Path
/// * `srcPath` group: Path
/// * `transformType` group: PathTransformType
/// * `transformValues` len: COMPSIZE(transformType)
pub type glTransformPathNV_t = unsafe extern "system" fn(resultPath: GLuint, srcPath: GLuint, transformType: PathTransformType, transformValues: *const GLfloat);

/// glTranslated
pub type glTranslated_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// glTranslatef
pub type glTranslatef_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// glTranslatex
pub type glTranslatex_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// glTranslatexOES
pub type glTranslatexOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// glUniform1d
pub type glUniform1d_t = unsafe extern "system" fn(location: GLint, x: GLdouble);

/// glUniform1dv
/// * `value` len: count*1
pub type glUniform1dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

/// glUniform1f
pub type glUniform1f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);

/// glUniform1fARB
pub type glUniform1fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);

/// glUniform1fv
/// * `value` len: count*1
pub type glUniform1fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// glUniform1fvARB
/// * `value` len: count*1
pub type glUniform1fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// glUniform1i
pub type glUniform1i_t = unsafe extern "system" fn(location: GLint, v0: GLint);

/// glUniform1i64ARB
pub type glUniform1i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64);

/// glUniform1i64NV
pub type glUniform1i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT);

/// glUniform1i64vARB
/// * `value` len: count*1
pub type glUniform1i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

/// glUniform1i64vNV
/// * `value` len: count*1
pub type glUniform1i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

/// glUniform1iARB
pub type glUniform1iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint);

/// glUniform1iv
/// * `value` len: count*1
pub type glUniform1iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// glUniform1ivARB
/// * `value` len: count*1
pub type glUniform1ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// glUniform1ui
pub type glUniform1ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint);

/// glUniform1ui64ARB
pub type glUniform1ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64);

/// glUniform1ui64NV
pub type glUniform1ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT);

/// glUniform1ui64vARB
/// * `value` len: count*1
pub type glUniform1ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// glUniform1ui64vNV
/// * `value` len: count*1
pub type glUniform1ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glUniform1uiEXT
pub type glUniform1uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint);

/// glUniform1uiv
/// * `value` len: count*1
pub type glUniform1uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// glUniform1uivEXT
/// * `value` len: count*1
pub type glUniform1uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// glUniform2d
pub type glUniform2d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble);

/// glUniform2dv
/// * `value` len: count*2
pub type glUniform2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

/// glUniform2f
pub type glUniform2f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);

/// glUniform2fARB
pub type glUniform2fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);

/// glUniform2fv
/// * `value` len: count*2
pub type glUniform2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// glUniform2fvARB
/// * `value` len: count*2
pub type glUniform2fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// glUniform2i
pub type glUniform2i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);

/// glUniform2i64ARB
pub type glUniform2i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64);

/// glUniform2i64NV
pub type glUniform2i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT);

/// glUniform2i64vARB
/// * `value` len: count*2
pub type glUniform2i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

/// glUniform2i64vNV
/// * `value` len: count*2
pub type glUniform2i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

/// glUniform2iARB
pub type glUniform2iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);

/// glUniform2iv
/// * `value` len: count*2
pub type glUniform2iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// glUniform2ivARB
/// * `value` len: count*2
pub type glUniform2ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// glUniform2ui
pub type glUniform2ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);

/// glUniform2ui64ARB
pub type glUniform2ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64);

/// glUniform2ui64NV
pub type glUniform2ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT);

/// glUniform2ui64vARB
/// * `value` len: count*2
pub type glUniform2ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// glUniform2ui64vNV
/// * `value` len: count*2
pub type glUniform2ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glUniform2uiEXT
pub type glUniform2uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);

/// glUniform2uiv
/// * `value` len: count*2
pub type glUniform2uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// glUniform2uivEXT
/// * `value` len: count*2
pub type glUniform2uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// glUniform3d
pub type glUniform3d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);

/// glUniform3dv
/// * `value` len: count*3
pub type glUniform3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

/// glUniform3f
pub type glUniform3f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

/// glUniform3fARB
pub type glUniform3fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

/// glUniform3fv
/// * `value` len: count*3
pub type glUniform3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// glUniform3fvARB
/// * `value` len: count*3
pub type glUniform3fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// glUniform3i
pub type glUniform3i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);

/// glUniform3i64ARB
pub type glUniform3i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64, z: GLint64);

/// glUniform3i64NV
pub type glUniform3i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

/// glUniform3i64vARB
/// * `value` len: count*3
pub type glUniform3i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

/// glUniform3i64vNV
/// * `value` len: count*3
pub type glUniform3i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

/// glUniform3iARB
pub type glUniform3iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);

/// glUniform3iv
/// * `value` len: count*3
pub type glUniform3iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// glUniform3ivARB
/// * `value` len: count*3
pub type glUniform3ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// glUniform3ui
pub type glUniform3ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

/// glUniform3ui64ARB
pub type glUniform3ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);

/// glUniform3ui64NV
pub type glUniform3ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

/// glUniform3ui64vARB
/// * `value` len: count*3
pub type glUniform3ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// glUniform3ui64vNV
/// * `value` len: count*3
pub type glUniform3ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glUniform3uiEXT
pub type glUniform3uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

/// glUniform3uiv
/// * `value` len: count*3
pub type glUniform3uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// glUniform3uivEXT
/// * `value` len: count*3
pub type glUniform3uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// glUniform4d
pub type glUniform4d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glUniform4dv
/// * `value` len: count*4
pub type glUniform4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

/// glUniform4f
pub type glUniform4f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

/// glUniform4fARB
pub type glUniform4fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

/// glUniform4fv
/// * `value` len: count*4
pub type glUniform4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// glUniform4fvARB
/// * `value` len: count*4
pub type glUniform4fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

/// glUniform4i
pub type glUniform4i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

/// glUniform4i64ARB
pub type glUniform4i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);

/// glUniform4i64NV
pub type glUniform4i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

/// glUniform4i64vARB
/// * `value` len: count*4
pub type glUniform4i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

/// glUniform4i64vNV
/// * `value` len: count*4
pub type glUniform4i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

/// glUniform4iARB
pub type glUniform4iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

/// glUniform4iv
/// * `value` len: count*4
pub type glUniform4iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// glUniform4ivARB
/// * `value` len: count*4
pub type glUniform4ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

/// glUniform4ui
pub type glUniform4ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

/// glUniform4ui64ARB
pub type glUniform4ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);

/// glUniform4ui64NV
pub type glUniform4ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

/// glUniform4ui64vARB
/// * `value` len: count*4
pub type glUniform4ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// glUniform4ui64vNV
/// * `value` len: count*4
pub type glUniform4ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glUniform4uiEXT
pub type glUniform4uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

/// glUniform4uiv
/// * `value` len: count*4
pub type glUniform4uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// glUniform4uivEXT
/// * `value` len: count*4
pub type glUniform4uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

/// glUniformBlockBinding
/// * `program` class: program
pub type glUniformBlockBinding_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);

/// glUniformBufferEXT
/// * `program` class: program
/// * `buffer` class: buffer
pub type glUniformBufferEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, buffer: GLuint);

/// glUniformHandleui64ARB
pub type glUniformHandleui64ARB_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

/// glUniformHandleui64IMG
pub type glUniformHandleui64IMG_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

/// glUniformHandleui64NV
pub type glUniformHandleui64NV_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

/// glUniformHandleui64vARB
/// * `value` len: count
pub type glUniformHandleui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// glUniformHandleui64vIMG
/// * `value` len: count
pub type glUniformHandleui64vIMG_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// glUniformHandleui64vNV
/// * `value` len: count
pub type glUniformHandleui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

/// glUniformMatrix2dv
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glUniformMatrix2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix2fv
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glUniformMatrix2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix2fvARB
/// * `transpose` group: Boolean
/// * `value` len: count*4
pub type glUniformMatrix2fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix2x3dv
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix2x3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix2x3fv
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix2x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix2x3fvNV
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix2x3fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix2x4dv
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix2x4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix2x4fv
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix2x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix2x4fvNV
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix2x4fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix3dv
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glUniformMatrix3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix3fv
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glUniformMatrix3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix3fvARB
/// * `transpose` group: Boolean
/// * `value` len: count*9
pub type glUniformMatrix3fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix3x2dv
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix3x2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix3x2fv
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix3x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix3x2fvNV
/// * `transpose` group: Boolean
/// * `value` len: count*6
pub type glUniformMatrix3x2fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix3x4dv
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix3x4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix3x4fv
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix3x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix3x4fvNV
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix3x4fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix4dv
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glUniformMatrix4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix4fv
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glUniformMatrix4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix4fvARB
/// * `transpose` group: Boolean
/// * `value` len: count*16
pub type glUniformMatrix4fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix4x2dv
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix4x2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix4x2fv
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix4x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix4x2fvNV
/// * `transpose` group: Boolean
/// * `value` len: count*8
pub type glUniformMatrix4x2fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix4x3dv
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix4x3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

/// glUniformMatrix4x3fv
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix4x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformMatrix4x3fvNV
/// * `transpose` group: Boolean
/// * `value` len: count*12
pub type glUniformMatrix4x3fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

/// glUniformSubroutinesuiv
/// * `shadertype` group: ShaderType
/// * `indices` len: count
pub type glUniformSubroutinesuiv_t = unsafe extern "system" fn(shadertype: ShaderType, count: GLsizei, indices: *const GLuint);

/// glUniformui64NV
pub type glUniformui64NV_t = unsafe extern "system" fn(location: GLint, value: GLuint64EXT);

/// glUniformui64vNV
/// * `value` len: count*1
pub type glUniformui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

/// glUnlockArraysEXT
pub type glUnlockArraysEXT_t = unsafe extern "system" fn();

/// glUnmapBuffer
/// * `target` group: BufferTargetARB
pub type glUnmapBuffer_t = unsafe extern "system" fn(target: BufferTargetARB) -> GLboolean;

/// glUnmapBufferARB
/// * `target` group: BufferTargetARB
pub type glUnmapBufferARB_t = unsafe extern "system" fn(target: BufferTargetARB) -> GLboolean;

/// glUnmapBufferOES
pub type glUnmapBufferOES_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;

/// glUnmapNamedBuffer
/// * `buffer` class: buffer
pub type glUnmapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// glUnmapNamedBufferEXT
/// * `buffer` class: buffer
pub type glUnmapNamedBufferEXT_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

/// glUnmapObjectBufferATI
/// * `buffer` class: buffer
pub type glUnmapObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint);

/// glUnmapTexture2DINTEL
/// * `texture` class: texture
pub type glUnmapTexture2DINTEL_t = unsafe extern "system" fn(texture: GLuint, level: GLint);

/// glUpdateObjectBufferATI
/// * `buffer` class: buffer
/// * `pointer` len: size
/// * `preserve` group: PreserveModeATI
pub type glUpdateObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint, offset: GLuint, size: GLsizei, pointer: *const void, preserve: PreserveModeATI);

/// glUploadGpuMaskNVX
pub type glUploadGpuMaskNVX_t = unsafe extern "system" fn(mask: GLbitfield);

/// glUseProgram
/// * `program` class: program
pub type glUseProgram_t = unsafe extern "system" fn(program: GLuint);

/// glUseProgramObjectARB
/// * `programObj` group: handleARB
pub type glUseProgramObjectARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

/// glUseProgramStages
/// * `pipeline` class: program pipeline
/// * `stages` group: UseProgramStageMask
/// * `program` class: program
pub type glUseProgramStages_t = unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);

/// glUseProgramStagesEXT
/// * `pipeline` class: program pipeline
/// * `stages` group: UseProgramStageMask
/// * `program` class: program
pub type glUseProgramStagesEXT_t = unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);

/// glUseShaderProgramEXT
/// * `program` class: program
pub type glUseShaderProgramEXT_t = unsafe extern "system" fn(type_: GLenum, program: GLuint);

/// glVDPAUFiniNV
pub type glVDPAUFiniNV_t = unsafe extern "system" fn();

/// glVDPAUGetSurfaceivNV
/// * `surface` group: vdpauSurfaceNV
/// * `values` len: count
pub type glVDPAUGetSurfaceivNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

/// glVDPAUInitNV
pub type glVDPAUInitNV_t = unsafe extern "system" fn(vdpDevice: *const void, getProcAddress: *const void);

/// glVDPAUIsSurfaceNV
/// * `surface` group: vdpauSurfaceNV
pub type glVDPAUIsSurfaceNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV) -> GLboolean;

/// glVDPAUMapSurfacesNV
/// * `surfaces` group: vdpauSurfaceNV
/// * `surfaces` len: numSurfaces
pub type glVDPAUMapSurfacesNV_t = unsafe extern "system" fn(numSurfaces: GLsizei, surfaces: *const GLvdpauSurfaceNV);

/// glVDPAURegisterOutputSurfaceNV
/// * `textureNames` len: numTextureNames
pub type glVDPAURegisterOutputSurfaceNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint) -> GLvdpauSurfaceNV;

/// glVDPAURegisterVideoSurfaceNV
/// * `textureNames` len: numTextureNames
pub type glVDPAURegisterVideoSurfaceNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint) -> GLvdpauSurfaceNV;

/// glVDPAURegisterVideoSurfaceWithPictureStructureNV
/// * `textureNames` len: numTextureNames
/// * `isFrameStructure` group: Boolean
pub type glVDPAURegisterVideoSurfaceWithPictureStructureNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint, isFrameStructure: GLboolean) -> GLvdpauSurfaceNV;

/// glVDPAUSurfaceAccessNV
/// * `surface` group: vdpauSurfaceNV
pub type glVDPAUSurfaceAccessNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV, access: GLenum);

/// glVDPAUUnmapSurfacesNV
/// * `surfaces` group: vdpauSurfaceNV
/// * `surfaces` len: numSurface
pub type glVDPAUUnmapSurfacesNV_t = unsafe extern "system" fn(numSurface: GLsizei, surfaces: *const GLvdpauSurfaceNV);

/// glVDPAUUnregisterSurfaceNV
/// * `surface` group: vdpauSurfaceNV
pub type glVDPAUUnregisterSurfaceNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV);

/// glValidateProgram
/// * `program` class: program
pub type glValidateProgram_t = unsafe extern "system" fn(program: GLuint);

/// glValidateProgramARB
/// * `programObj` group: handleARB
pub type glValidateProgramARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

/// glValidateProgramPipeline
/// * `pipeline` class: program pipeline
pub type glValidateProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);

/// glValidateProgramPipelineEXT
/// * `pipeline` class: program pipeline
pub type glValidateProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint);

/// glVariantArrayObjectATI
/// * `type` group: ScalarType
/// * `buffer` class: buffer
pub type glVariantArrayObjectATI_t = unsafe extern "system" fn(id: GLuint, type_: ScalarType, stride: GLsizei, buffer: GLuint, offset: GLuint);

/// glVariantPointerEXT
/// * `type` group: ScalarType
/// * `addr` len: COMPSIZE(id,type,stride)
pub type glVariantPointerEXT_t = unsafe extern "system" fn(id: GLuint, type_: ScalarType, stride: GLuint, addr: *const void);

/// glVariantbvEXT
/// * `addr` len: COMPSIZE(id)
pub type glVariantbvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLbyte);

/// glVariantdvEXT
/// * `addr` len: COMPSIZE(id)
pub type glVariantdvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLdouble);

/// glVariantfvEXT
/// * `addr` len: COMPSIZE(id)
pub type glVariantfvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLfloat);

/// glVariantivEXT
/// * `addr` len: COMPSIZE(id)
pub type glVariantivEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLint);

/// glVariantsvEXT
/// * `addr` len: COMPSIZE(id)
pub type glVariantsvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLshort);

/// glVariantubvEXT
/// * `addr` len: COMPSIZE(id)
pub type glVariantubvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLubyte);

/// glVariantuivEXT
/// * `addr` len: COMPSIZE(id)
pub type glVariantuivEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLuint);

/// glVariantusvEXT
/// * `addr` len: COMPSIZE(id)
pub type glVariantusvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLushort);

/// glVertex2bOES
pub type glVertex2bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte);

/// glVertex2bvOES
pub type glVertex2bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 2]);

/// glVertex2d
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glVertex2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// glVertex2dv
/// * `v` group: CoordD
pub type glVertex2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// glVertex2f
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glVertex2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// glVertex2fv
/// * `v` group: CoordF
pub type glVertex2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// glVertex2hNV
/// * `x` group: Half16NV
/// * `y` group: Half16NV
pub type glVertex2hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV);

/// glVertex2hvNV
/// * `v` group: Half16NV
pub type glVertex2hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 2]);

/// glVertex2i
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glVertex2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// glVertex2iv
/// * `v` group: CoordI
pub type glVertex2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// glVertex2s
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glVertex2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// glVertex2sv
/// * `v` group: CoordS
pub type glVertex2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// glVertex2xOES
pub type glVertex2xOES_t = unsafe extern "system" fn(x: GLfixed);

/// glVertex2xvOES
pub type glVertex2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

/// glVertex3bOES
pub type glVertex3bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte, z: GLbyte);

/// glVertex3bvOES
pub type glVertex3bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 3]);

/// glVertex3d
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glVertex3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// glVertex3dv
/// * `v` group: CoordD
pub type glVertex3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glVertex3f
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glVertex3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// glVertex3fv
/// * `v` group: CoordF
pub type glVertex3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glVertex3hNV
/// * `x` group: Half16NV
/// * `y` group: Half16NV
/// * `z` group: Half16NV
pub type glVertex3hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV, z: GLhalfNV);

/// glVertex3hvNV
/// * `v` group: Half16NV
pub type glVertex3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

/// glVertex3i
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glVertex3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// glVertex3iv
/// * `v` group: CoordI
pub type glVertex3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glVertex3s
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glVertex3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// glVertex3sv
/// * `v` group: CoordS
pub type glVertex3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glVertex3xOES
pub type glVertex3xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed);

/// glVertex3xvOES
pub type glVertex3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

/// glVertex4bOES
pub type glVertex4bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte, z: GLbyte, w: GLbyte);

/// glVertex4bvOES
pub type glVertex4bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 4]);

/// glVertex4d
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
/// * `w` group: CoordD
pub type glVertex4d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glVertex4dv
/// * `v` group: CoordD
pub type glVertex4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// glVertex4f
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
/// * `w` group: CoordF
pub type glVertex4f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glVertex4fv
/// * `v` group: CoordF
pub type glVertex4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// glVertex4hNV
/// * `x` group: Half16NV
/// * `y` group: Half16NV
/// * `z` group: Half16NV
/// * `w` group: Half16NV
pub type glVertex4hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV, z: GLhalfNV, w: GLhalfNV);

/// glVertex4hvNV
/// * `v` group: Half16NV
pub type glVertex4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

/// glVertex4i
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
/// * `w` group: CoordI
pub type glVertex4i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

/// glVertex4iv
/// * `v` group: CoordI
pub type glVertex4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// glVertex4s
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
/// * `w` group: CoordS
pub type glVertex4s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// glVertex4sv
/// * `v` group: CoordS
pub type glVertex4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// glVertex4xOES
pub type glVertex4xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

/// glVertex4xvOES
pub type glVertex4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

/// glVertexArrayAttribBinding
/// * `vaobj` class: vertex array
pub type glVertexArrayAttribBinding_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);

/// glVertexArrayAttribFormat
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
pub type glVertexArrayAttribFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);

/// glVertexArrayAttribIFormat
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribIType
pub type glVertexArrayAttribIFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);

/// glVertexArrayAttribLFormat
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribLType
pub type glVertexArrayAttribLFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);

/// glVertexArrayBindVertexBufferEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
pub type glVertexArrayBindVertexBufferEXT_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

/// glVertexArrayBindingDivisor
/// * `vaobj` class: vertex array
pub type glVertexArrayBindingDivisor_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);

/// glVertexArrayColorOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: ColorPointerType
pub type glVertexArrayColorOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: ColorPointerType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayEdgeFlagOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
pub type glVertexArrayEdgeFlagOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, stride: GLsizei, offset: GLintptr);

/// glVertexArrayElementBuffer
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
pub type glVertexArrayElementBuffer_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint);

/// glVertexArrayFogCoordOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: FogCoordinatePointerType
pub type glVertexArrayFogCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: FogCoordinatePointerType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayIndexOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: IndexPointerType
pub type glVertexArrayIndexOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: IndexPointerType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayMultiTexCoordOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: TexCoordPointerType
pub type glVertexArrayMultiTexCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, texunit: GLenum, size: GLint, type_: TexCoordPointerType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayNormalOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: NormalPointerType
pub type glVertexArrayNormalOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: NormalPointerType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayParameteriAPPLE
/// * `pname` group: VertexArrayPNameAPPLE
pub type glVertexArrayParameteriAPPLE_t = unsafe extern "system" fn(pname: VertexArrayPNameAPPLE, param: GLint);

/// glVertexArrayRangeAPPLE
/// * `pointer` len: length
pub type glVertexArrayRangeAPPLE_t = unsafe extern "system" fn(length: GLsizei, pointer: *mut void);

/// glVertexArrayRangeNV
/// * `pointer` len: COMPSIZE(length)
pub type glVertexArrayRangeNV_t = unsafe extern "system" fn(length: GLsizei, pointer: *const void);

/// glVertexArraySecondaryColorOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: ColorPointerType
pub type glVertexArraySecondaryColorOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: ColorPointerType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayTexCoordOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: TexCoordPointerType
pub type glVertexArrayTexCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: TexCoordPointerType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayVertexAttribBindingEXT
/// * `vaobj` class: vertex array
pub type glVertexArrayVertexAttribBindingEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);

/// glVertexArrayVertexAttribDivisorEXT
/// * `vaobj` class: vertex array
pub type glVertexArrayVertexAttribDivisorEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, divisor: GLuint);

/// glVertexArrayVertexAttribFormatEXT
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
pub type glVertexArrayVertexAttribFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);

/// glVertexArrayVertexAttribIFormatEXT
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribIType
pub type glVertexArrayVertexAttribIFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);

/// glVertexArrayVertexAttribIOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: VertexAttribType
pub type glVertexArrayVertexAttribIOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: VertexAttribType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayVertexAttribLFormatEXT
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribLType
pub type glVertexArrayVertexAttribLFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);

/// glVertexArrayVertexAttribLOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: VertexAttribLType
/// * `offset` group: BufferOffset
pub type glVertexArrayVertexAttribLOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, offset: GLintptr);

/// glVertexArrayVertexAttribOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexArrayVertexAttribOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, offset: GLintptr);

/// glVertexArrayVertexBindingDivisorEXT
/// * `vaobj` class: vertex array
pub type glVertexArrayVertexBindingDivisorEXT_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);

/// glVertexArrayVertexBuffer
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
pub type glVertexArrayVertexBuffer_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

/// glVertexArrayVertexBuffers
/// * `vaobj` class: vertex array
/// * `buffers` class: buffer
pub type glVertexArrayVertexBuffers_t = unsafe extern "system" fn(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);

/// glVertexArrayVertexOffsetEXT
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
/// * `type` group: VertexPointerType
pub type glVertexArrayVertexOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: VertexPointerType, stride: GLsizei, offset: GLintptr);

/// glVertexAttrib1d
pub type glVertexAttrib1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// glVertexAttrib1dARB
pub type glVertexAttrib1dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// glVertexAttrib1dNV
pub type glVertexAttrib1dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// glVertexAttrib1dv
pub type glVertexAttrib1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// glVertexAttrib1dvARB
pub type glVertexAttrib1dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// glVertexAttrib1dvNV
pub type glVertexAttrib1dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// glVertexAttrib1f
pub type glVertexAttrib1f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

/// glVertexAttrib1fARB
pub type glVertexAttrib1fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

/// glVertexAttrib1fNV
pub type glVertexAttrib1fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

/// glVertexAttrib1fv
pub type glVertexAttrib1fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

/// glVertexAttrib1fvARB
pub type glVertexAttrib1fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

/// glVertexAttrib1fvNV
pub type glVertexAttrib1fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

/// glVertexAttrib1hNV
/// * `x` group: Half16NV
pub type glVertexAttrib1hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV);

/// glVertexAttrib1hvNV
/// * `v` group: Half16NV
pub type glVertexAttrib1hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLhalfNV);

/// glVertexAttrib1s
pub type glVertexAttrib1s_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

/// glVertexAttrib1sARB
pub type glVertexAttrib1sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

/// glVertexAttrib1sNV
pub type glVertexAttrib1sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

/// glVertexAttrib1sv
pub type glVertexAttrib1sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

/// glVertexAttrib1svARB
pub type glVertexAttrib1svARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

/// glVertexAttrib1svNV
pub type glVertexAttrib1svNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

/// glVertexAttrib2d
pub type glVertexAttrib2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// glVertexAttrib2dARB
pub type glVertexAttrib2dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// glVertexAttrib2dNV
pub type glVertexAttrib2dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// glVertexAttrib2dv
pub type glVertexAttrib2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// glVertexAttrib2dvARB
pub type glVertexAttrib2dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// glVertexAttrib2dvNV
pub type glVertexAttrib2dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// glVertexAttrib2f
pub type glVertexAttrib2f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

/// glVertexAttrib2fARB
pub type glVertexAttrib2fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

/// glVertexAttrib2fNV
pub type glVertexAttrib2fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

/// glVertexAttrib2fv
pub type glVertexAttrib2fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

/// glVertexAttrib2fvARB
pub type glVertexAttrib2fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

/// glVertexAttrib2fvNV
pub type glVertexAttrib2fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

/// glVertexAttrib2hNV
/// * `x` group: Half16NV
/// * `y` group: Half16NV
pub type glVertexAttrib2hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV);

/// glVertexAttrib2hvNV
/// * `v` group: Half16NV
pub type glVertexAttrib2hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 2]);

/// glVertexAttrib2s
pub type glVertexAttrib2s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

/// glVertexAttrib2sARB
pub type glVertexAttrib2sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

/// glVertexAttrib2sNV
pub type glVertexAttrib2sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

/// glVertexAttrib2sv
pub type glVertexAttrib2sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

/// glVertexAttrib2svARB
pub type glVertexAttrib2svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

/// glVertexAttrib2svNV
pub type glVertexAttrib2svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

/// glVertexAttrib3d
pub type glVertexAttrib3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// glVertexAttrib3dARB
pub type glVertexAttrib3dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// glVertexAttrib3dNV
pub type glVertexAttrib3dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// glVertexAttrib3dv
pub type glVertexAttrib3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// glVertexAttrib3dvARB
pub type glVertexAttrib3dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// glVertexAttrib3dvNV
pub type glVertexAttrib3dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// glVertexAttrib3f
pub type glVertexAttrib3f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

/// glVertexAttrib3fARB
pub type glVertexAttrib3fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

/// glVertexAttrib3fNV
pub type glVertexAttrib3fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

/// glVertexAttrib3fv
pub type glVertexAttrib3fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

/// glVertexAttrib3fvARB
pub type glVertexAttrib3fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

/// glVertexAttrib3fvNV
pub type glVertexAttrib3fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

/// glVertexAttrib3hNV
/// * `x` group: Half16NV
/// * `y` group: Half16NV
/// * `z` group: Half16NV
pub type glVertexAttrib3hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV, z: GLhalfNV);

/// glVertexAttrib3hvNV
/// * `v` group: Half16NV
pub type glVertexAttrib3hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 3]);

/// glVertexAttrib3s
pub type glVertexAttrib3s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

/// glVertexAttrib3sARB
pub type glVertexAttrib3sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

/// glVertexAttrib3sNV
pub type glVertexAttrib3sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

/// glVertexAttrib3sv
pub type glVertexAttrib3sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

/// glVertexAttrib3svARB
pub type glVertexAttrib3svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

/// glVertexAttrib3svNV
pub type glVertexAttrib3svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

/// glVertexAttrib4Nbv
pub type glVertexAttrib4Nbv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// glVertexAttrib4NbvARB
pub type glVertexAttrib4NbvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// glVertexAttrib4Niv
pub type glVertexAttrib4Niv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glVertexAttrib4NivARB
pub type glVertexAttrib4NivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glVertexAttrib4Nsv
pub type glVertexAttrib4Nsv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// glVertexAttrib4NsvARB
pub type glVertexAttrib4NsvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// glVertexAttrib4Nub
pub type glVertexAttrib4Nub_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

/// glVertexAttrib4NubARB
pub type glVertexAttrib4NubARB_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

/// glVertexAttrib4Nubv
pub type glVertexAttrib4Nubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// glVertexAttrib4NubvARB
pub type glVertexAttrib4NubvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// glVertexAttrib4Nuiv
pub type glVertexAttrib4Nuiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// glVertexAttrib4NuivARB
pub type glVertexAttrib4NuivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// glVertexAttrib4Nusv
pub type glVertexAttrib4Nusv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// glVertexAttrib4NusvARB
pub type glVertexAttrib4NusvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// glVertexAttrib4bv
pub type glVertexAttrib4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// glVertexAttrib4bvARB
pub type glVertexAttrib4bvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// glVertexAttrib4d
pub type glVertexAttrib4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glVertexAttrib4dARB
pub type glVertexAttrib4dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glVertexAttrib4dNV
pub type glVertexAttrib4dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glVertexAttrib4dv
pub type glVertexAttrib4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// glVertexAttrib4dvARB
pub type glVertexAttrib4dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// glVertexAttrib4dvNV
pub type glVertexAttrib4dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// glVertexAttrib4f
pub type glVertexAttrib4f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glVertexAttrib4fARB
pub type glVertexAttrib4fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glVertexAttrib4fNV
pub type glVertexAttrib4fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glVertexAttrib4fv
pub type glVertexAttrib4fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// glVertexAttrib4fvARB
pub type glVertexAttrib4fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// glVertexAttrib4fvNV
pub type glVertexAttrib4fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// glVertexAttrib4hNV
/// * `x` group: Half16NV
/// * `y` group: Half16NV
/// * `z` group: Half16NV
/// * `w` group: Half16NV
pub type glVertexAttrib4hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV, z: GLhalfNV, w: GLhalfNV);

/// glVertexAttrib4hvNV
/// * `v` group: Half16NV
pub type glVertexAttrib4hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 4]);

/// glVertexAttrib4iv
pub type glVertexAttrib4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glVertexAttrib4ivARB
pub type glVertexAttrib4ivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glVertexAttrib4s
pub type glVertexAttrib4s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// glVertexAttrib4sARB
pub type glVertexAttrib4sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// glVertexAttrib4sNV
pub type glVertexAttrib4sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// glVertexAttrib4sv
pub type glVertexAttrib4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// glVertexAttrib4svARB
pub type glVertexAttrib4svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// glVertexAttrib4svNV
pub type glVertexAttrib4svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// glVertexAttrib4ubNV
/// * `x` group: ColorUB
/// * `y` group: ColorUB
/// * `z` group: ColorUB
/// * `w` group: ColorUB
pub type glVertexAttrib4ubNV_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

/// glVertexAttrib4ubv
pub type glVertexAttrib4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// glVertexAttrib4ubvARB
pub type glVertexAttrib4ubvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// glVertexAttrib4ubvNV
/// * `v` group: ColorUB
pub type glVertexAttrib4ubvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// glVertexAttrib4uiv
pub type glVertexAttrib4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// glVertexAttrib4uivARB
pub type glVertexAttrib4uivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// glVertexAttrib4usv
pub type glVertexAttrib4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// glVertexAttrib4usvARB
pub type glVertexAttrib4usvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// glVertexAttribArrayObjectATI
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
/// * `buffer` class: buffer
pub type glVertexAttribArrayObjectATI_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, buffer: GLuint, offset: GLuint);

/// glVertexAttribBinding
pub type glVertexAttribBinding_t = unsafe extern "system" fn(attribindex: GLuint, bindingindex: GLuint);

/// glVertexAttribDivisor
pub type glVertexAttribDivisor_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// glVertexAttribDivisorANGLE
pub type glVertexAttribDivisorANGLE_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// glVertexAttribDivisorARB
pub type glVertexAttribDivisorARB_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// glVertexAttribDivisorEXT
pub type glVertexAttribDivisorEXT_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// glVertexAttribDivisorNV
pub type glVertexAttribDivisorNV_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

/// glVertexAttribFormat
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
pub type glVertexAttribFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);

/// glVertexAttribFormatNV
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
pub type glVertexAttribFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, stride: GLsizei);

/// glVertexAttribI1i
pub type glVertexAttribI1i_t = unsafe extern "system" fn(index: GLuint, x: GLint);

/// glVertexAttribI1iEXT
pub type glVertexAttribI1iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint);

/// glVertexAttribI1iv
pub type glVertexAttribI1iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);

/// glVertexAttribI1ivEXT
pub type glVertexAttribI1ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);

/// glVertexAttribI1ui
pub type glVertexAttribI1ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint);

/// glVertexAttribI1uiEXT
pub type glVertexAttribI1uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint);

/// glVertexAttribI1uiv
pub type glVertexAttribI1uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);

/// glVertexAttribI1uivEXT
pub type glVertexAttribI1uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);

/// glVertexAttribI2i
pub type glVertexAttribI2i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);

/// glVertexAttribI2iEXT
pub type glVertexAttribI2iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);

/// glVertexAttribI2iv
pub type glVertexAttribI2iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 2]);

/// glVertexAttribI2ivEXT
pub type glVertexAttribI2ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 2]);

/// glVertexAttribI2ui
pub type glVertexAttribI2ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);

/// glVertexAttribI2uiEXT
pub type glVertexAttribI2uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);

/// glVertexAttribI2uiv
pub type glVertexAttribI2uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 2]);

/// glVertexAttribI2uivEXT
pub type glVertexAttribI2uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 2]);

/// glVertexAttribI3i
pub type glVertexAttribI3i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);

/// glVertexAttribI3iEXT
pub type glVertexAttribI3iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);

/// glVertexAttribI3iv
pub type glVertexAttribI3iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 3]);

/// glVertexAttribI3ivEXT
pub type glVertexAttribI3ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 3]);

/// glVertexAttribI3ui
pub type glVertexAttribI3ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);

/// glVertexAttribI3uiEXT
pub type glVertexAttribI3uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);

/// glVertexAttribI3uiv
pub type glVertexAttribI3uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 3]);

/// glVertexAttribI3uivEXT
pub type glVertexAttribI3uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 3]);

/// glVertexAttribI4bv
pub type glVertexAttribI4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// glVertexAttribI4bvEXT
pub type glVertexAttribI4bvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

/// glVertexAttribI4i
pub type glVertexAttribI4i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// glVertexAttribI4iEXT
pub type glVertexAttribI4iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

/// glVertexAttribI4iv
pub type glVertexAttribI4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glVertexAttribI4ivEXT
pub type glVertexAttribI4ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

/// glVertexAttribI4sv
pub type glVertexAttribI4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// glVertexAttribI4svEXT
pub type glVertexAttribI4svEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

/// glVertexAttribI4ubv
pub type glVertexAttribI4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// glVertexAttribI4ubvEXT
pub type glVertexAttribI4ubvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

/// glVertexAttribI4ui
pub type glVertexAttribI4ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// glVertexAttribI4uiEXT
pub type glVertexAttribI4uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

/// glVertexAttribI4uiv
pub type glVertexAttribI4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// glVertexAttribI4uivEXT
pub type glVertexAttribI4uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

/// glVertexAttribI4usv
pub type glVertexAttribI4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// glVertexAttribI4usvEXT
pub type glVertexAttribI4usvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

/// glVertexAttribIFormat
/// * `type` group: VertexAttribIType
pub type glVertexAttribIFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);

/// glVertexAttribIFormatNV
/// * `type` group: VertexAttribIType
pub type glVertexAttribIFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei);

/// glVertexAttribIPointer
/// * `type` group: VertexAttribIType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexAttribIPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void);

/// glVertexAttribIPointerEXT
/// * `type` group: VertexAttribIType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexAttribIPointerEXT_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void);

/// glVertexAttribL1d
pub type glVertexAttribL1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// glVertexAttribL1dEXT
pub type glVertexAttribL1dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

/// glVertexAttribL1dv
pub type glVertexAttribL1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// glVertexAttribL1dvEXT
pub type glVertexAttribL1dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

/// glVertexAttribL1i64NV
pub type glVertexAttribL1i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT);

/// glVertexAttribL1i64vNV
pub type glVertexAttribL1i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLint64EXT);

/// glVertexAttribL1ui64ARB
pub type glVertexAttribL1ui64ARB_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT);

/// glVertexAttribL1ui64NV
pub type glVertexAttribL1ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT);

/// glVertexAttribL1ui64vARB
pub type glVertexAttribL1ui64vARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint64EXT);

/// glVertexAttribL1ui64vNV
pub type glVertexAttribL1ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint64EXT);

/// glVertexAttribL2d
pub type glVertexAttribL2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// glVertexAttribL2dEXT
pub type glVertexAttribL2dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

/// glVertexAttribL2dv
pub type glVertexAttribL2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// glVertexAttribL2dvEXT
pub type glVertexAttribL2dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

/// glVertexAttribL2i64NV
pub type glVertexAttribL2i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT);

/// glVertexAttribL2i64vNV
pub type glVertexAttribL2i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 2]);

/// glVertexAttribL2ui64NV
pub type glVertexAttribL2ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT);

/// glVertexAttribL2ui64vNV
pub type glVertexAttribL2ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 2]);

/// glVertexAttribL3d
pub type glVertexAttribL3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// glVertexAttribL3dEXT
pub type glVertexAttribL3dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

/// glVertexAttribL3dv
pub type glVertexAttribL3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// glVertexAttribL3dvEXT
pub type glVertexAttribL3dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

/// glVertexAttribL3i64NV
pub type glVertexAttribL3i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

/// glVertexAttribL3i64vNV
pub type glVertexAttribL3i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 3]);

/// glVertexAttribL3ui64NV
pub type glVertexAttribL3ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

/// glVertexAttribL3ui64vNV
pub type glVertexAttribL3ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 3]);

/// glVertexAttribL4d
pub type glVertexAttribL4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glVertexAttribL4dEXT
pub type glVertexAttribL4dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glVertexAttribL4dv
pub type glVertexAttribL4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// glVertexAttribL4dvEXT
pub type glVertexAttribL4dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

/// glVertexAttribL4i64NV
pub type glVertexAttribL4i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

/// glVertexAttribL4i64vNV
pub type glVertexAttribL4i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 4]);

/// glVertexAttribL4ui64NV
pub type glVertexAttribL4ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

/// glVertexAttribL4ui64vNV
pub type glVertexAttribL4ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 4]);

/// glVertexAttribLFormat
/// * `type` group: VertexAttribLType
pub type glVertexAttribLFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);

/// glVertexAttribLFormatNV
/// * `type` group: VertexAttribLType
pub type glVertexAttribLFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei);

/// glVertexAttribLPointer
/// * `type` group: VertexAttribLType
/// * `pointer` len: size
pub type glVertexAttribLPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, pointer: *const void);

/// glVertexAttribLPointerEXT
/// * `type` group: VertexAttribLType
/// * `pointer` len: size
pub type glVertexAttribLPointerEXT_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, pointer: *const void);

/// glVertexAttribP1ui
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP1ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

/// glVertexAttribP1uiv
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP1uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

/// glVertexAttribP2ui
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP2ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

/// glVertexAttribP2uiv
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP2uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

/// glVertexAttribP3ui
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP3ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

/// glVertexAttribP3uiv
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP3uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

/// glVertexAttribP4ui
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP4ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

/// glVertexAttribP4uiv
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
pub type glVertexAttribP4uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

/// glVertexAttribParameteriAMD
pub type glVertexAttribParameteriAMD_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, param: GLint);

/// glVertexAttribPointer
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexAttribPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void);

/// glVertexAttribPointerARB
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexAttribPointerARB_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void);

/// glVertexAttribPointerNV
/// * `type` group: VertexAttribEnumNV
/// * `pointer` len: COMPSIZE(fsize,type,stride)
pub type glVertexAttribPointerNV_t = unsafe extern "system" fn(index: GLuint, fsize: GLint, type_: VertexAttribEnumNV, stride: GLsizei, pointer: *const void);

/// glVertexAttribs1dvNV
/// * `v` len: count
pub type glVertexAttribs1dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

/// glVertexAttribs1fvNV
/// * `v` len: count
pub type glVertexAttribs1fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

/// glVertexAttribs1hvNV
/// * `v` group: Half16NV
/// * `v` len: n
pub type glVertexAttribs1hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

/// glVertexAttribs1svNV
/// * `v` len: count
pub type glVertexAttribs1svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

/// glVertexAttribs2dvNV
/// * `v` len: count*2
pub type glVertexAttribs2dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

/// glVertexAttribs2fvNV
/// * `v` len: count*2
pub type glVertexAttribs2fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

/// glVertexAttribs2hvNV
/// * `v` group: Half16NV
/// * `v` len: n
pub type glVertexAttribs2hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

/// glVertexAttribs2svNV
/// * `v` len: count*2
pub type glVertexAttribs2svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

/// glVertexAttribs3dvNV
/// * `v` len: count*3
pub type glVertexAttribs3dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

/// glVertexAttribs3fvNV
/// * `v` len: count*3
pub type glVertexAttribs3fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

/// glVertexAttribs3hvNV
/// * `v` group: Half16NV
/// * `v` len: n
pub type glVertexAttribs3hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

/// glVertexAttribs3svNV
/// * `v` len: count*3
pub type glVertexAttribs3svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

/// glVertexAttribs4dvNV
/// * `v` len: count*4
pub type glVertexAttribs4dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

/// glVertexAttribs4fvNV
/// * `v` len: count*4
pub type glVertexAttribs4fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

/// glVertexAttribs4hvNV
/// * `v` group: Half16NV
/// * `v` len: n
pub type glVertexAttribs4hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

/// glVertexAttribs4svNV
/// * `v` len: count*4
pub type glVertexAttribs4svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

/// glVertexAttribs4ubvNV
/// * `v` group: ColorUB
/// * `v` len: count*4
pub type glVertexAttribs4ubvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLubyte);

/// glVertexBindingDivisor
pub type glVertexBindingDivisor_t = unsafe extern "system" fn(bindingindex: GLuint, divisor: GLuint);

/// glVertexBlendARB
pub type glVertexBlendARB_t = unsafe extern "system" fn(count: GLint);

/// glVertexBlendEnvfATI
/// * `pname` group: VertexStreamATI
pub type glVertexBlendEnvfATI_t = unsafe extern "system" fn(pname: VertexStreamATI, param: GLfloat);

/// glVertexBlendEnviATI
/// * `pname` group: VertexStreamATI
pub type glVertexBlendEnviATI_t = unsafe extern "system" fn(pname: VertexStreamATI, param: GLint);

/// glVertexFormatNV
/// * `type` group: VertexPointerType
pub type glVertexFormatNV_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, stride: GLsizei);

/// glVertexP2ui
/// * `type` group: VertexPointerType
pub type glVertexP2ui_t = unsafe extern "system" fn(type_: VertexPointerType, value: GLuint);

/// glVertexP2uiv
/// * `type` group: VertexPointerType
pub type glVertexP2uiv_t = unsafe extern "system" fn(type_: VertexPointerType, value: *const GLuint);

/// glVertexP3ui
/// * `type` group: VertexPointerType
pub type glVertexP3ui_t = unsafe extern "system" fn(type_: VertexPointerType, value: GLuint);

/// glVertexP3uiv
/// * `type` group: VertexPointerType
pub type glVertexP3uiv_t = unsafe extern "system" fn(type_: VertexPointerType, value: *const GLuint);

/// glVertexP4ui
/// * `type` group: VertexPointerType
pub type glVertexP4ui_t = unsafe extern "system" fn(type_: VertexPointerType, value: GLuint);

/// glVertexP4uiv
/// * `type` group: VertexPointerType
pub type glVertexP4uiv_t = unsafe extern "system" fn(type_: VertexPointerType, value: *const GLuint);

/// glVertexPointer
/// * `type` group: VertexPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexPointer_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, stride: GLsizei, pointer: *const void);

/// glVertexPointerEXT
/// * `type` group: VertexPointerType
/// * `pointer` len: COMPSIZE(size,type,stride,count)
pub type glVertexPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, stride: GLsizei, count: GLsizei, pointer: *const void);

/// glVertexPointerListIBM
/// * `type` group: VertexPointerType
/// * `pointer` len: COMPSIZE(size,type,stride)
pub type glVertexPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

/// glVertexPointervINTEL
/// * `type` group: VertexPointerType
pub type glVertexPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: VertexPointerType, pointer: *const [*mut void; 4]);

/// glVertexStream1dATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream1dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLdouble);

/// glVertexStream1dvATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream1dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const GLdouble);

/// glVertexStream1fATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream1fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLfloat);

/// glVertexStream1fvATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream1fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const GLfloat);

/// glVertexStream1iATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream1iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLint);

/// glVertexStream1ivATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream1ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const GLint);

/// glVertexStream1sATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream1sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLshort);

/// glVertexStream1svATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream1svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const GLshort);

/// glVertexStream2dATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream2dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLdouble, y: GLdouble);

/// glVertexStream2dvATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream2dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLdouble; 2]);

/// glVertexStream2fATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream2fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLfloat, y: GLfloat);

/// glVertexStream2fvATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream2fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLfloat; 2]);

/// glVertexStream2iATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream2iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLint, y: GLint);

/// glVertexStream2ivATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream2ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLint; 2]);

/// glVertexStream2sATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream2sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLshort, y: GLshort);

/// glVertexStream2svATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream2svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLshort; 2]);

/// glVertexStream3dATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream3dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLdouble, y: GLdouble, z: GLdouble);

/// glVertexStream3dvATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream3dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLdouble; 3]);

/// glVertexStream3fATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream3fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLfloat, y: GLfloat, z: GLfloat);

/// glVertexStream3fvATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream3fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLfloat; 3]);

/// glVertexStream3iATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream3iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLint, y: GLint, z: GLint);

/// glVertexStream3ivATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream3ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLint; 3]);

/// glVertexStream3sATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream3sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLshort, y: GLshort, z: GLshort);

/// glVertexStream3svATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream3svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLshort; 3]);

/// glVertexStream4dATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream4dATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glVertexStream4dvATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream4dvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLdouble; 4]);

/// glVertexStream4fATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream4fATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glVertexStream4fvATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream4fvATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLfloat; 4]);

/// glVertexStream4iATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream4iATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLint, y: GLint, z: GLint, w: GLint);

/// glVertexStream4ivATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream4ivATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLint; 4]);

/// glVertexStream4sATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream4sATI_t = unsafe extern "system" fn(stream: VertexStreamATI, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// glVertexStream4svATI
/// * `stream` group: VertexStreamATI
pub type glVertexStream4svATI_t = unsafe extern "system" fn(stream: VertexStreamATI, coords: *const [GLshort; 4]);

/// glVertexWeightPointerEXT
/// * `type` group: VertexWeightPointerTypeEXT
/// * `pointer` len: COMPSIZE(type,stride)
pub type glVertexWeightPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: VertexWeightPointerTypeEXT, stride: GLsizei, pointer: *const void);

/// glVertexWeightfEXT
pub type glVertexWeightfEXT_t = unsafe extern "system" fn(weight: GLfloat);

/// glVertexWeightfvEXT
pub type glVertexWeightfvEXT_t = unsafe extern "system" fn(weight: *const GLfloat);

/// glVertexWeighthNV
/// * `weight` group: Half16NV
pub type glVertexWeighthNV_t = unsafe extern "system" fn(weight: GLhalfNV);

/// glVertexWeighthvNV
/// * `weight` group: Half16NV
pub type glVertexWeighthvNV_t = unsafe extern "system" fn(weight: *const GLhalfNV);

/// glVideoCaptureNV
pub type glVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, sequence_num: *mut GLuint, capture_time: *mut GLuint64EXT) -> GLenum;

/// glVideoCaptureStreamParameterdvNV
/// * `params` len: COMPSIZE(pname)
pub type glVideoCaptureStreamParameterdvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLdouble);

/// glVideoCaptureStreamParameterfvNV
/// * `params` len: COMPSIZE(pname)
pub type glVideoCaptureStreamParameterfvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLfloat);

/// glVideoCaptureStreamParameterivNV
/// * `params` len: COMPSIZE(pname)
pub type glVideoCaptureStreamParameterivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLint);

/// glViewport
/// * `x` group: WinCoord
/// * `y` group: WinCoord
pub type glViewport_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

/// glViewportArrayv
/// * `v` len: COMPSIZE(count)
pub type glViewportArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// glViewportArrayvNV
/// * `v` len: COMPSIZE(count)
pub type glViewportArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// glViewportArrayvOES
/// * `v` len: COMPSIZE(count)
pub type glViewportArrayvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

/// glViewportIndexedf
pub type glViewportIndexedf_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

/// glViewportIndexedfNV
pub type glViewportIndexedfNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

/// glViewportIndexedfOES
pub type glViewportIndexedfOES_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

/// glViewportIndexedfv
pub type glViewportIndexedfv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// glViewportIndexedfvNV
pub type glViewportIndexedfvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// glViewportIndexedfvOES
pub type glViewportIndexedfvOES_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

/// glViewportPositionWScaleNV
pub type glViewportPositionWScaleNV_t = unsafe extern "system" fn(index: GLuint, xcoeff: GLfloat, ycoeff: GLfloat);

/// glViewportSwizzleNV
pub type glViewportSwizzleNV_t = unsafe extern "system" fn(index: GLuint, swizzlex: GLenum, swizzley: GLenum, swizzlez: GLenum, swizzlew: GLenum);

/// glWaitSemaphoreEXT
/// * `buffers` class: buffer
/// * `buffers` len: COMPSIZE(numBufferBarriers)
/// * `textures` class: texture
/// * `textures` len: COMPSIZE(numTextureBarriers)
/// * `srcLayouts` group: TextureLayout
/// * `srcLayouts` len: COMPSIZE(numTextureBarriers)
pub type glWaitSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint, numBufferBarriers: GLuint, buffers: *const GLuint, numTextureBarriers: GLuint, textures: *const GLuint, srcLayouts: *const TextureLayout);

/// glWaitSemaphoreui64NVX
/// * `semaphoreArray` len: fenceObjectCount
/// * `fenceValueArray` len: fenceObjectCount
pub type glWaitSemaphoreui64NVX_t = unsafe extern "system" fn(waitGpu: GLuint, fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

/// glWaitSync
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncBehaviorFlags
pub type glWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

/// glWaitSyncAPPLE
/// * `sync` class: sync
/// * `flags` group: SyncBehaviorFlags
pub type glWaitSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

/// glWaitVkSemaphoreNV
pub type glWaitVkSemaphoreNV_t = unsafe extern "system" fn(vkSemaphore: GLuint64);

/// glWeightPathsNV
/// * `resultPath` group: Path
/// * `paths` group: Path
/// * `paths` len: numPaths
/// * `weights` len: numPaths
pub type glWeightPathsNV_t = unsafe extern "system" fn(resultPath: GLuint, numPaths: GLsizei, paths: *const GLuint, weights: *const GLfloat);

/// glWeightPointerARB
/// * `type` group: WeightPointerTypeARB
/// * `pointer` len: COMPSIZE(type,stride)
pub type glWeightPointerARB_t = unsafe extern "system" fn(size: GLint, type_: WeightPointerTypeARB, stride: GLsizei, pointer: *const void);

/// glWeightPointerOES
/// * `pointer` len: COMPSIZE(type,stride)
pub type glWeightPointerOES_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

/// glWeightbvARB
/// * `weights` len: size
pub type glWeightbvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLbyte);

/// glWeightdvARB
/// * `weights` len: size
pub type glWeightdvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLdouble);

/// glWeightfvARB
/// * `weights` len: size
pub type glWeightfvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLfloat);

/// glWeightivARB
/// * `weights` len: size
pub type glWeightivARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLint);

/// glWeightsvARB
/// * `weights` len: size
pub type glWeightsvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLshort);

/// glWeightubvARB
/// * `weights` len: size
pub type glWeightubvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLubyte);

/// glWeightuivARB
/// * `weights` len: size
pub type glWeightuivARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLuint);

/// glWeightusvARB
/// * `weights` len: size
pub type glWeightusvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLushort);

/// glWindowPos2d
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glWindowPos2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// glWindowPos2dARB
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glWindowPos2dARB_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// glWindowPos2dMESA
/// * `x` group: CoordD
/// * `y` group: CoordD
pub type glWindowPos2dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

/// glWindowPos2dv
/// * `v` group: CoordD
pub type glWindowPos2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// glWindowPos2dvARB
/// * `v` group: CoordD
pub type glWindowPos2dvARB_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// glWindowPos2dvMESA
/// * `v` group: CoordD
pub type glWindowPos2dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

/// glWindowPos2f
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glWindowPos2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// glWindowPos2fARB
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glWindowPos2fARB_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// glWindowPos2fMESA
/// * `x` group: CoordF
/// * `y` group: CoordF
pub type glWindowPos2fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

/// glWindowPos2fv
/// * `v` group: CoordF
pub type glWindowPos2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// glWindowPos2fvARB
/// * `v` group: CoordF
pub type glWindowPos2fvARB_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// glWindowPos2fvMESA
/// * `v` group: CoordF
pub type glWindowPos2fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

/// glWindowPos2i
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glWindowPos2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// glWindowPos2iARB
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glWindowPos2iARB_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// glWindowPos2iMESA
/// * `x` group: CoordI
/// * `y` group: CoordI
pub type glWindowPos2iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint);

/// glWindowPos2iv
/// * `v` group: CoordI
pub type glWindowPos2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// glWindowPos2ivARB
/// * `v` group: CoordI
pub type glWindowPos2ivARB_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// glWindowPos2ivMESA
/// * `v` group: CoordI
pub type glWindowPos2ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 2]);

/// glWindowPos2s
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glWindowPos2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// glWindowPos2sARB
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glWindowPos2sARB_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// glWindowPos2sMESA
/// * `x` group: CoordS
/// * `y` group: CoordS
pub type glWindowPos2sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

/// glWindowPos2sv
/// * `v` group: CoordS
pub type glWindowPos2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// glWindowPos2svARB
/// * `v` group: CoordS
pub type glWindowPos2svARB_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// glWindowPos2svMESA
/// * `v` group: CoordS
pub type glWindowPos2svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

/// glWindowPos3d
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glWindowPos3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// glWindowPos3dARB
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glWindowPos3dARB_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// glWindowPos3dMESA
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
pub type glWindowPos3dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

/// glWindowPos3dv
/// * `v` group: CoordD
pub type glWindowPos3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glWindowPos3dvARB
/// * `v` group: CoordD
pub type glWindowPos3dvARB_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glWindowPos3dvMESA
/// * `v` group: CoordD
pub type glWindowPos3dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

/// glWindowPos3f
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glWindowPos3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// glWindowPos3fARB
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glWindowPos3fARB_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// glWindowPos3fMESA
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
pub type glWindowPos3fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

/// glWindowPos3fv
/// * `v` group: CoordF
pub type glWindowPos3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glWindowPos3fvARB
/// * `v` group: CoordF
pub type glWindowPos3fvARB_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glWindowPos3fvMESA
/// * `v` group: CoordF
pub type glWindowPos3fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

/// glWindowPos3i
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glWindowPos3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// glWindowPos3iARB
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glWindowPos3iARB_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// glWindowPos3iMESA
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
pub type glWindowPos3iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

/// glWindowPos3iv
/// * `v` group: CoordI
pub type glWindowPos3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glWindowPos3ivARB
/// * `v` group: CoordI
pub type glWindowPos3ivARB_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glWindowPos3ivMESA
/// * `v` group: CoordI
pub type glWindowPos3ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 3]);

/// glWindowPos3s
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glWindowPos3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// glWindowPos3sARB
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glWindowPos3sARB_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// glWindowPos3sMESA
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
pub type glWindowPos3sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

/// glWindowPos3sv
/// * `v` group: CoordS
pub type glWindowPos3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glWindowPos3svARB
/// * `v` group: CoordS
pub type glWindowPos3svARB_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glWindowPos3svMESA
/// * `v` group: CoordS
pub type glWindowPos3svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

/// glWindowPos4dMESA
/// * `x` group: CoordD
/// * `y` group: CoordD
/// * `z` group: CoordD
/// * `w` group: CoordD
pub type glWindowPos4dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

/// glWindowPos4dvMESA
/// * `v` group: CoordD
pub type glWindowPos4dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

/// glWindowPos4fMESA
/// * `x` group: CoordF
/// * `y` group: CoordF
/// * `z` group: CoordF
/// * `w` group: CoordF
pub type glWindowPos4fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

/// glWindowPos4fvMESA
/// * `v` group: CoordF
pub type glWindowPos4fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

/// glWindowPos4iMESA
/// * `x` group: CoordI
/// * `y` group: CoordI
/// * `z` group: CoordI
/// * `w` group: CoordI
pub type glWindowPos4iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

/// glWindowPos4ivMESA
/// * `v` group: CoordI
pub type glWindowPos4ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 4]);

/// glWindowPos4sMESA
/// * `x` group: CoordS
/// * `y` group: CoordS
/// * `z` group: CoordS
/// * `w` group: CoordS
pub type glWindowPos4sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

/// glWindowPos4svMESA
/// * `v` group: CoordS
pub type glWindowPos4svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

/// glWindowRectanglesEXT
/// * `box` len: COMPSIZE(count)
pub type glWindowRectanglesEXT_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, box_: *const GLint);

/// glWriteMaskEXT
/// * `outX` group: VertexShaderWriteMaskEXT
/// * `outY` group: VertexShaderWriteMaskEXT
/// * `outZ` group: VertexShaderWriteMaskEXT
/// * `outW` group: VertexShaderWriteMaskEXT
pub type glWriteMaskEXT_t = unsafe extern "system" fn(res: GLuint, in_: GLuint, outX: VertexShaderWriteMaskEXT, outY: VertexShaderWriteMaskEXT, outZ: VertexShaderWriteMaskEXT, outW: VertexShaderWriteMaskEXT);
