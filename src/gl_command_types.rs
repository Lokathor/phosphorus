use super::*;

pub type glAccum_t = unsafe extern "system" fn(op: GLenum, value: GLfloat);

pub type glAccumxOES_t = unsafe extern "system" fn(op: GLenum, value: GLfixed);

pub type glAcquireKeyedMutexWin32EXT_t = unsafe extern "system" fn(memory: GLuint, key: GLuint64, timeout: GLuint) -> GLboolean;

pub type glActiveProgramEXT_t = unsafe extern "system" fn(program: GLuint);

pub type glActiveShaderProgram_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);

pub type glActiveShaderProgramEXT_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);

pub type glActiveStencilFaceEXT_t = unsafe extern "system" fn(face: GLenum);

pub type glActiveTexture_t = unsafe extern "system" fn(texture: GLenum);

pub type glActiveTextureARB_t = unsafe extern "system" fn(texture: GLenum);

pub type glActiveVaryingNV_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar);

pub type glAlphaFragmentOp1ATI_t = unsafe extern "system" fn(op: GLenum, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint);

pub type glAlphaFragmentOp2ATI_t = unsafe extern "system" fn(op: GLenum, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint);

pub type glAlphaFragmentOp3ATI_t = unsafe extern "system" fn(op: GLenum, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint, arg3: GLuint, arg3Rep: GLuint, arg3Mod: GLuint);

pub type glAlphaFunc_t = unsafe extern "system" fn(func: GLenum, ref_: GLfloat);

pub type glAlphaFuncQCOM_t = unsafe extern "system" fn(func: GLenum, ref_: GLclampf);

pub type glAlphaFuncx_t = unsafe extern "system" fn(func: GLenum, ref_: GLfixed);

pub type glAlphaFuncxOES_t = unsafe extern "system" fn(func: GLenum, ref_: GLfixed);

pub type glAlphaToCoverageDitherControlNV_t = unsafe extern "system" fn(mode: GLenum);

pub type glApplyFramebufferAttachmentCMAAINTEL_t = unsafe extern "system" fn();

pub type glApplyTextureEXT_t = unsafe extern "system" fn(mode: GLenum);

pub type glAreProgramsResidentNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint, residences: *mut GLboolean) -> GLboolean;

pub type glAreTexturesResident_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean;

pub type glAreTexturesResidentEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean;

pub type glArrayElement_t = unsafe extern "system" fn(i: GLint);

pub type glArrayElementEXT_t = unsafe extern "system" fn(i: GLint);

pub type glArrayObjectATI_t = unsafe extern "system" fn(array: GLenum, size: GLint, type_: GLenum, stride: GLsizei, buffer: GLuint, offset: GLuint);

pub type glAsyncCopyBufferSubDataNVX_t = unsafe extern "system" fn(waitSemaphoreCount: GLsizei, waitSemaphoreArray: *const GLuint, fenceValueArray: *const GLuint64, readGpu: GLuint, writeGpuMask: GLbitfield, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr, signalSemaphoreCount: GLsizei, signalSemaphoreArray: *const GLuint, signalValueArray: *const GLuint64) -> GLuint;

pub type glAsyncCopyImageSubDataNVX_t = unsafe extern "system" fn(waitSemaphoreCount: GLsizei, waitSemaphoreArray: *const GLuint, waitValueArray: *const GLuint64, srcGpu: GLuint, dstGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei, signalSemaphoreCount: GLsizei, signalSemaphoreArray: *const GLuint, signalValueArray: *const GLuint64) -> GLuint;

pub type glAsyncMarkerSGIX_t = unsafe extern "system" fn(marker: GLuint);

pub type glAttachObjectARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, obj: GLhandleARB);

pub type glAttachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);

pub type glBegin_t = unsafe extern "system" fn(mode: GLenum);

pub type glBeginConditionalRender_t = unsafe extern "system" fn(id: GLuint, mode: GLenum);

pub type glBeginConditionalRenderNV_t = unsafe extern "system" fn(id: GLuint, mode: GLenum);

pub type glBeginConditionalRenderNVX_t = unsafe extern "system" fn(id: GLuint);

pub type glBeginFragmentShaderATI_t = unsafe extern "system" fn();

pub type glBeginOcclusionQueryNV_t = unsafe extern "system" fn(id: GLuint);

pub type glBeginPerfMonitorAMD_t = unsafe extern "system" fn(monitor: GLuint);

pub type glBeginPerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

pub type glBeginQuery_t = unsafe extern "system" fn(target: GLenum, id: GLuint);

pub type glBeginQueryARB_t = unsafe extern "system" fn(target: GLenum, id: GLuint);

pub type glBeginQueryEXT_t = unsafe extern "system" fn(target: GLenum, id: GLuint);

pub type glBeginQueryIndexed_t = unsafe extern "system" fn(target: GLenum, index: GLuint, id: GLuint);

pub type glBeginTransformFeedback_t = unsafe extern "system" fn(primitiveMode: GLenum);

pub type glBeginTransformFeedbackEXT_t = unsafe extern "system" fn(primitiveMode: GLenum);

pub type glBeginTransformFeedbackNV_t = unsafe extern "system" fn(primitiveMode: GLenum);

pub type glBeginVertexShaderEXT_t = unsafe extern "system" fn();

pub type glBeginVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint);

pub type glBindAttribLocation_t = unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);

pub type glBindAttribLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, name: *const GLcharARB);

pub type glBindBuffer_t = unsafe extern "system" fn(target: GLenum, buffer: GLuint);

pub type glBindBufferARB_t = unsafe extern "system" fn(target: GLenum, buffer: GLuint);

pub type glBindBufferBase_t = unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint);

pub type glBindBufferBaseEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint);

pub type glBindBufferBaseNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint);

pub type glBindBufferOffsetEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr);

pub type glBindBufferOffsetNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr);

pub type glBindBufferRange_t = unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glBindBufferRangeEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glBindBufferRangeNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glBindBuffersBase_t = unsafe extern "system" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint);

pub type glBindBuffersRange_t = unsafe extern "system" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);

pub type glBindFragDataLocation_t = unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);

pub type glBindFragDataLocationEXT_t = unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);

pub type glBindFragDataLocationIndexed_t = unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);

pub type glBindFragDataLocationIndexedEXT_t = unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);

pub type glBindFragmentShaderATI_t = unsafe extern "system" fn(id: GLuint);

pub type glBindFramebuffer_t = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);

pub type glBindFramebufferEXT_t = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);

pub type glBindFramebufferOES_t = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);

pub type glBindImageTexture_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum);

pub type glBindImageTextureEXT_t = unsafe extern "system" fn(index: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLint);

pub type glBindImageTextures_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);

pub type glBindLightParameterEXT_t = unsafe extern "system" fn(light: GLenum, value: GLenum) -> GLuint;

pub type glBindMaterialParameterEXT_t = unsafe extern "system" fn(face: GLenum, value: GLenum) -> GLuint;

pub type glBindMultiTextureEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, texture: GLuint);

pub type glBindParameterEXT_t = unsafe extern "system" fn(value: GLenum) -> GLuint;

pub type glBindProgramARB_t = unsafe extern "system" fn(target: GLenum, program: GLuint);

pub type glBindProgramNV_t = unsafe extern "system" fn(target: GLenum, id: GLuint);

pub type glBindProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);

pub type glBindProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint);

pub type glBindRenderbuffer_t = unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);

pub type glBindRenderbufferEXT_t = unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);

pub type glBindRenderbufferOES_t = unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);

pub type glBindSampler_t = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);

pub type glBindSamplers_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint);

pub type glBindShadingRateImageNV_t = unsafe extern "system" fn(texture: GLuint);

pub type glBindTexGenParameterEXT_t = unsafe extern "system" fn(unit: GLenum, coord: GLenum, value: GLenum) -> GLuint;

pub type glBindTexture_t = unsafe extern "system" fn(target: GLenum, texture: GLuint);

pub type glBindTextureEXT_t = unsafe extern "system" fn(target: GLenum, texture: GLuint);

pub type glBindTextureUnit_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint);

pub type glBindTextureUnitParameterEXT_t = unsafe extern "system" fn(unit: GLenum, value: GLenum) -> GLuint;

pub type glBindTextures_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);

pub type glBindTransformFeedback_t = unsafe extern "system" fn(target: GLenum, id: GLuint);

pub type glBindTransformFeedbackNV_t = unsafe extern "system" fn(target: GLenum, id: GLuint);

pub type glBindVertexArray_t = unsafe extern "system" fn(array: GLuint);

pub type glBindVertexArrayAPPLE_t = unsafe extern "system" fn(array: GLuint);

pub type glBindVertexArrayOES_t = unsafe extern "system" fn(array: GLuint);

pub type glBindVertexBuffer_t = unsafe extern "system" fn(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

pub type glBindVertexBuffers_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);

pub type glBindVertexShaderEXT_t = unsafe extern "system" fn(id: GLuint);

pub type glBindVideoCaptureStreamBufferNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, frame_region: GLenum, offset: GLintptrARB);

pub type glBindVideoCaptureStreamTextureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, frame_region: GLenum, target: GLenum, texture: GLuint);

pub type glBinormal3bEXT_t = unsafe extern "system" fn(bx: GLbyte, by: GLbyte, bz: GLbyte);

pub type glBinormal3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

pub type glBinormal3dEXT_t = unsafe extern "system" fn(bx: GLdouble, by: GLdouble, bz: GLdouble);

pub type glBinormal3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glBinormal3fEXT_t = unsafe extern "system" fn(bx: GLfloat, by: GLfloat, bz: GLfloat);

pub type glBinormal3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glBinormal3iEXT_t = unsafe extern "system" fn(bx: GLint, by: GLint, bz: GLint);

pub type glBinormal3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glBinormal3sEXT_t = unsafe extern "system" fn(bx: GLshort, by: GLshort, bz: GLshort);

pub type glBinormal3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glBinormalPointerEXT_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glBitmap_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *const GLubyte);

pub type glBitmapxOES_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, xorig: GLfixed, yorig: GLfixed, xmove: GLfixed, ymove: GLfixed, bitmap: *const GLubyte);

pub type glBlendBarrier_t = unsafe extern "system" fn();

pub type glBlendBarrierKHR_t = unsafe extern "system" fn();

pub type glBlendBarrierNV_t = unsafe extern "system" fn();

pub type glBlendColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub type glBlendColorEXT_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub type glBlendColorxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

pub type glBlendEquation_t = unsafe extern "system" fn(mode: GLenum);

pub type glBlendEquationEXT_t = unsafe extern "system" fn(mode: GLenum);

pub type glBlendEquationIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, mode: GLenum);

pub type glBlendEquationOES_t = unsafe extern "system" fn(mode: GLenum);

pub type glBlendEquationSeparate_t = unsafe extern "system" fn(modeRGB: GLenum, modeAlpha: GLenum);

pub type glBlendEquationSeparateEXT_t = unsafe extern "system" fn(modeRGB: GLenum, modeAlpha: GLenum);

pub type glBlendEquationSeparateIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);

pub type glBlendEquationSeparateOES_t = unsafe extern "system" fn(modeRGB: GLenum, modeAlpha: GLenum);

pub type glBlendEquationSeparatei_t = unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);

pub type glBlendEquationSeparateiARB_t = unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);

pub type glBlendEquationSeparateiEXT_t = unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);

pub type glBlendEquationSeparateiOES_t = unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);

pub type glBlendEquationi_t = unsafe extern "system" fn(buf: GLuint, mode: GLenum);

pub type glBlendEquationiARB_t = unsafe extern "system" fn(buf: GLuint, mode: GLenum);

pub type glBlendEquationiEXT_t = unsafe extern "system" fn(buf: GLuint, mode: GLenum);

pub type glBlendEquationiOES_t = unsafe extern "system" fn(buf: GLuint, mode: GLenum);

pub type glBlendFunc_t = unsafe extern "system" fn(sfactor: GLenum, dfactor: GLenum);

pub type glBlendFuncIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);

pub type glBlendFuncSeparate_t = unsafe extern "system" fn(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);

pub type glBlendFuncSeparateEXT_t = unsafe extern "system" fn(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);

pub type glBlendFuncSeparateINGR_t = unsafe extern "system" fn(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);

pub type glBlendFuncSeparateIndexedAMD_t = unsafe extern "system" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);

pub type glBlendFuncSeparateOES_t = unsafe extern "system" fn(srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);

pub type glBlendFuncSeparatei_t = unsafe extern "system" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);

pub type glBlendFuncSeparateiARB_t = unsafe extern "system" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);

pub type glBlendFuncSeparateiEXT_t = unsafe extern "system" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);

pub type glBlendFuncSeparateiOES_t = unsafe extern "system" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);

pub type glBlendFunci_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);

pub type glBlendFunciARB_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);

pub type glBlendFunciEXT_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);

pub type glBlendFunciOES_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);

pub type glBlendParameteriNV_t = unsafe extern "system" fn(pname: GLenum, value: GLint);

pub type glBlitFramebuffer_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);

pub type glBlitFramebufferANGLE_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);

pub type glBlitFramebufferEXT_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);

pub type glBlitFramebufferNV_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);

pub type glBlitNamedFramebuffer_t = unsafe extern "system" fn(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);

pub type glBufferAddressRangeNV_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, address: GLuint64EXT, length: GLsizeiptr);

pub type glBufferAttachMemoryNV_t = unsafe extern "system" fn(target: GLenum, memory: GLuint, offset: GLuint64);

pub type glBufferData_t = unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const void, usage: GLenum);

pub type glBufferDataARB_t = unsafe extern "system" fn(target: GLenum, size: GLsizeiptrARB, data: *const void, usage: GLenum);

pub type glBufferPageCommitmentARB_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

pub type glBufferPageCommitmentMemNV_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, memory: GLuint, memOffset: GLuint64, commit: GLboolean);

pub type glBufferParameteriAPPLE_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

pub type glBufferStorage_t = unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const void, flags: GLbitfield);

pub type glBufferStorageEXT_t = unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const void, flags: GLbitfield);

pub type glBufferStorageExternalEXT_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, clientBuffer: GLeglClientBufferEXT, flags: GLbitfield);

pub type glBufferStorageMemEXT_t = unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, memory: GLuint, offset: GLuint64);

pub type glBufferSubData_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const void);

pub type glBufferSubDataARB_t = unsafe extern "system" fn(target: GLenum, offset: GLintptrARB, size: GLsizeiptrARB, data: *const void);

pub type glCallCommandListNV_t = unsafe extern "system" fn(list: GLuint);

pub type glCallList_t = unsafe extern "system" fn(list: GLuint);

pub type glCallLists_t = unsafe extern "system" fn(n: GLsizei, type_: GLenum, lists: *const void);

pub type glCheckFramebufferStatus_t = unsafe extern "system" fn(target: GLenum) -> GLenum;

pub type glCheckFramebufferStatusEXT_t = unsafe extern "system" fn(target: GLenum) -> GLenum;

pub type glCheckFramebufferStatusOES_t = unsafe extern "system" fn(target: GLenum) -> GLenum;

pub type glCheckNamedFramebufferStatus_t = unsafe extern "system" fn(framebuffer: GLuint, target: GLenum) -> GLenum;

pub type glCheckNamedFramebufferStatusEXT_t = unsafe extern "system" fn(framebuffer: GLuint, target: GLenum) -> GLenum;

pub type glClampColor_t = unsafe extern "system" fn(target: GLenum, clamp: GLenum);

pub type glClampColorARB_t = unsafe extern "system" fn(target: GLenum, clamp: GLenum);

pub type glClear_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glClearAccum_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub type glClearAccumxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

pub type glClearBufferData_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const void);

pub type glClearBufferSubData_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const void);

pub type glClearBufferfi_t = unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

pub type glClearBufferfv_t = unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);

pub type glClearBufferiv_t = unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint);

pub type glClearBufferuiv_t = unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);

pub type glClearColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub type glClearColorIiEXT_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint, alpha: GLint);

pub type glClearColorIuiEXT_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);

pub type glClearColorx_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

pub type glClearColorxOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

pub type glClearDepth_t = unsafe extern "system" fn(depth: GLdouble);

pub type glClearDepthdNV_t = unsafe extern "system" fn(depth: GLdouble);

pub type glClearDepthf_t = unsafe extern "system" fn(d: GLfloat);

pub type glClearDepthfOES_t = unsafe extern "system" fn(depth: GLclampf);

pub type glClearDepthx_t = unsafe extern "system" fn(depth: GLfixed);

pub type glClearDepthxOES_t = unsafe extern "system" fn(depth: GLfixed);

pub type glClearIndex_t = unsafe extern "system" fn(c: GLfloat);

pub type glClearNamedBufferData_t = unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const void);

pub type glClearNamedBufferDataEXT_t = unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const void);

pub type glClearNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const void);

pub type glClearNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, offset: GLsizeiptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const void);

pub type glClearNamedFramebufferfi_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

pub type glClearNamedFramebufferfv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);

pub type glClearNamedFramebufferiv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint);

pub type glClearNamedFramebufferuiv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint);

pub type glClearPixelLocalStorageuiEXT_t = unsafe extern "system" fn(offset: GLsizei, n: GLsizei, values: *const GLuint);

pub type glClearStencil_t = unsafe extern "system" fn(s: GLint);

pub type glClearTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const void);

pub type glClearTexImageEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const void);

pub type glClearTexSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const void);

pub type glClearTexSubImageEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const void);

pub type glClientActiveTexture_t = unsafe extern "system" fn(texture: GLenum);

pub type glClientActiveTextureARB_t = unsafe extern "system" fn(texture: GLenum);

pub type glClientActiveVertexStreamATI_t = unsafe extern "system" fn(stream: GLenum);

pub type glClientAttribDefaultEXT_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glClientWaitSemaphoreui64NVX_t = unsafe extern "system" fn(fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

pub type glClientWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;

pub type glClientWaitSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;

pub type glClipControl_t = unsafe extern "system" fn(origin: GLenum, depth: GLenum);

pub type glClipControlEXT_t = unsafe extern "system" fn(origin: GLenum, depth: GLenum);

pub type glClipPlane_t = unsafe extern "system" fn(plane: GLenum, equation: *const [GLdouble; 4]);

pub type glClipPlanef_t = unsafe extern "system" fn(p: GLenum, eqn: *const [GLfloat; 4]);

pub type glClipPlanefIMG_t = unsafe extern "system" fn(p: GLenum, eqn: *const [GLfloat; 4]);

pub type glClipPlanefOES_t = unsafe extern "system" fn(plane: GLenum, equation: *const [GLfloat; 4]);

pub type glClipPlanex_t = unsafe extern "system" fn(plane: GLenum, equation: *const [GLfixed; 4]);

pub type glClipPlanexIMG_t = unsafe extern "system" fn(p: GLenum, eqn: *const [GLfixed; 4]);

pub type glClipPlanexOES_t = unsafe extern "system" fn(plane: GLenum, equation: *const [GLfixed; 4]);

pub type glColor3b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

pub type glColor3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

pub type glColor3d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

pub type glColor3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glColor3f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

pub type glColor3fVertex3fSUN_t = unsafe extern "system" fn(r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glColor3fVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glColor3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glColor3hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV);

pub type glColor3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

pub type glColor3i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

pub type glColor3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glColor3s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

pub type glColor3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glColor3ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

pub type glColor3ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

pub type glColor3ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

pub type glColor3uiv_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

pub type glColor3us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

pub type glColor3usv_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

pub type glColor3xOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed);

pub type glColor3xvOES_t = unsafe extern "system" fn(components: *const [GLfixed; 3]);

pub type glColor4b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte);

pub type glColor4bv_t = unsafe extern "system" fn(v: *const [GLbyte; 4]);

pub type glColor4d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble);

pub type glColor4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

pub type glColor4f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub type glColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glColor4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

pub type glColor4hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV, alpha: GLhalfNV);

pub type glColor4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

pub type glColor4i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint, alpha: GLint);

pub type glColor4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

pub type glColor4s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort);

pub type glColor4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

pub type glColor4ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte);

pub type glColor4ubVertex2fSUN_t = unsafe extern "system" fn(r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat);

pub type glColor4ubVertex2fvSUN_t = unsafe extern "system" fn(c: *const [GLubyte; 4], v: *const [GLfloat; 2]);

pub type glColor4ubVertex3fSUN_t = unsafe extern "system" fn(r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glColor4ubVertex3fvSUN_t = unsafe extern "system" fn(c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

pub type glColor4ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 4]);

pub type glColor4ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);

pub type glColor4uiv_t = unsafe extern "system" fn(v: *const [GLuint; 4]);

pub type glColor4us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort);

pub type glColor4usv_t = unsafe extern "system" fn(v: *const [GLushort; 4]);

pub type glColor4x_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

pub type glColor4xOES_t = unsafe extern "system" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);

pub type glColor4xvOES_t = unsafe extern "system" fn(components: *const [GLfixed; 4]);

pub type glColorFormatNV_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei);

pub type glColorFragmentOp1ATI_t = unsafe extern "system" fn(op: GLenum, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint);

pub type glColorFragmentOp2ATI_t = unsafe extern "system" fn(op: GLenum, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint);

pub type glColorFragmentOp3ATI_t = unsafe extern "system" fn(op: GLenum, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint, arg3: GLuint, arg3Rep: GLuint, arg3Mod: GLuint);

pub type glColorMask_t = unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

pub type glColorMaskIndexedEXT_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

pub type glColorMaski_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

pub type glColorMaskiEXT_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

pub type glColorMaskiOES_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

pub type glColorMaterial_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);

pub type glColorP3ui_t = unsafe extern "system" fn(type_: GLenum, color: GLuint);

pub type glColorP3uiv_t = unsafe extern "system" fn(type_: GLenum, color: *const GLuint);

pub type glColorP4ui_t = unsafe extern "system" fn(type_: GLenum, color: GLuint);

pub type glColorP4uiv_t = unsafe extern "system" fn(type_: GLenum, color: *const GLuint);

pub type glColorPointer_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glColorPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const void);

pub type glColorPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

pub type glColorPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: GLenum, pointer: *const [*mut void; 4]);

pub type glColorSubTable_t = unsafe extern "system" fn(target: GLenum, start: GLsizei, count: GLsizei, format: GLenum, type_: GLenum, data: *const void);

pub type glColorSubTableEXT_t = unsafe extern "system" fn(target: GLenum, start: GLsizei, count: GLsizei, format: GLenum, type_: GLenum, data: *const void);

pub type glColorTable_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, type_: GLenum, table: *const void);

pub type glColorTableEXT_t = unsafe extern "system" fn(target: GLenum, internalFormat: GLenum, width: GLsizei, format: GLenum, type_: GLenum, table: *const void);

pub type glColorTableParameterfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glColorTableParameterfvSGI_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glColorTableParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glColorTableParameterivSGI_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glColorTableSGI_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, type_: GLenum, table: *const void);

pub type glCombinerInputNV_t = unsafe extern "system" fn(stage: GLenum, portion: GLenum, variable: GLenum, input: GLenum, mapping: GLenum, componentUsage: GLenum);

pub type glCombinerOutputNV_t = unsafe extern "system" fn(stage: GLenum, portion: GLenum, abOutput: GLenum, cdOutput: GLenum, sumOutput: GLenum, scale: GLenum, bias: GLenum, abDotProduct: GLboolean, cdDotProduct: GLboolean, muxSum: GLboolean);

pub type glCombinerParameterfNV_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glCombinerParameterfvNV_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glCombinerParameteriNV_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glCombinerParameterivNV_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);

pub type glCombinerStageParameterfvNV_t = unsafe extern "system" fn(stage: GLenum, pname: GLenum, params: *const GLfloat);

pub type glCommandListSegmentsNV_t = unsafe extern "system" fn(list: GLuint, segments: GLuint);

pub type glCompileCommandListNV_t = unsafe extern "system" fn(list: GLuint);

pub type glCompileShader_t = unsafe extern "system" fn(shader: GLuint);

pub type glCompileShaderARB_t = unsafe extern "system" fn(shaderObj: GLhandleARB);

pub type glCompileShaderIncludeARB_t = unsafe extern "system" fn(shader: GLuint, count: GLsizei, path: *const *const GLchar, length: *const GLint);

pub type glCompressedMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

pub type glCompressedMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

pub type glCompressedMultiTexImage3DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

pub type glCompressedMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const void);

pub type glCompressedMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const void);

pub type glCompressedMultiTexSubImage3DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const void);

pub type glCompressedTexImage1D_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub type glCompressedTexImage1DARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub type glCompressedTexImage2D_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub type glCompressedTexImage2DARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub type glCompressedTexImage3D_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub type glCompressedTexImage3DARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub type glCompressedTexImage3DOES_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub type glCompressedTexSubImage1D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTexSubImage1DARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTexSubImage2D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTexSubImage2DARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTexSubImage3D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTexSubImage3DARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTexSubImage3DOES_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTextureImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

pub type glCompressedTextureImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

pub type glCompressedTextureImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const void);

pub type glCompressedTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const void);

pub type glCompressedTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTextureSubImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const void);

pub type glCompressedTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void);

pub type glCompressedTextureSubImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const void);

pub type glConservativeRasterParameterfNV_t = unsafe extern "system" fn(pname: GLenum, value: GLfloat);

pub type glConservativeRasterParameteriNV_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glConvolutionFilter1D_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, type_: GLenum, image: *const void);

pub type glConvolutionFilter1DEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, type_: GLenum, image: *const void);

pub type glConvolutionFilter2D_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, image: *const void);

pub type glConvolutionFilter2DEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, image: *const void);

pub type glConvolutionParameterf_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: GLfloat);

pub type glConvolutionParameterfEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: GLfloat);

pub type glConvolutionParameterfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glConvolutionParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glConvolutionParameteri_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: GLint);

pub type glConvolutionParameteriEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: GLint);

pub type glConvolutionParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glConvolutionParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glConvolutionParameterxOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfixed);

pub type glConvolutionParameterxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfixed);

pub type glCopyBufferSubData_t = unsafe extern "system" fn(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

pub type glCopyBufferSubDataNV_t = unsafe extern "system" fn(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

pub type glCopyColorSubTable_t = unsafe extern "system" fn(target: GLenum, start: GLsizei, x: GLint, y: GLint, width: GLsizei);

pub type glCopyColorSubTableEXT_t = unsafe extern "system" fn(target: GLenum, start: GLsizei, x: GLint, y: GLint, width: GLsizei);

pub type glCopyColorTable_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei);

pub type glCopyColorTableSGI_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei);

pub type glCopyConvolutionFilter1D_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei);

pub type glCopyConvolutionFilter1DEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei);

pub type glCopyConvolutionFilter2D_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyConvolutionFilter2DEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyImageSubData_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

pub type glCopyImageSubDataEXT_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

pub type glCopyImageSubDataNV_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

pub type glCopyImageSubDataOES_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

pub type glCopyMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);

pub type glCopyMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

pub type glCopyMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

pub type glCopyMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyMultiTexSubImage3DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyNamedBufferSubData_t = unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

pub type glCopyPathNV_t = unsafe extern "system" fn(resultPath: GLuint, srcPath: GLuint);

pub type glCopyPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_: GLenum);

pub type glCopyTexImage1D_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);

pub type glCopyTexImage1DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);

pub type glCopyTexImage2D_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

pub type glCopyTexImage2DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

pub type glCopyTexSubImage1D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

pub type glCopyTexSubImage1DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

pub type glCopyTexSubImage2D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyTexSubImage2DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyTexSubImage3D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyTexSubImage3DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyTexSubImage3DOES_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyTextureImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);

pub type glCopyTextureImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

pub type glCopyTextureLevelsAPPLE_t = unsafe extern "system" fn(destinationTexture: GLuint, sourceTexture: GLuint, sourceBaseLevel: GLint, sourceLevelCount: GLsizei);

pub type glCopyTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

pub type glCopyTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

pub type glCopyTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyTextureSubImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCopyTextureSubImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glCoverFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);

pub type glCoverFillPathNV_t = unsafe extern "system" fn(path: GLuint, coverMode: GLenum);

pub type glCoverStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);

pub type glCoverStrokePathNV_t = unsafe extern "system" fn(path: GLuint, coverMode: GLenum);

pub type glCoverageMaskNV_t = unsafe extern "system" fn(mask: GLboolean);

pub type glCoverageModulationNV_t = unsafe extern "system" fn(components: GLenum);

pub type glCoverageModulationTableNV_t = unsafe extern "system" fn(n: GLsizei, v: *const GLfloat);

pub type glCoverageOperationNV_t = unsafe extern "system" fn(operation: GLenum);

pub type glCreateBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

pub type glCreateCommandListsNV_t = unsafe extern "system" fn(n: GLsizei, lists: *mut GLuint);

pub type glCreateFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

pub type glCreateMemoryObjectsEXT_t = unsafe extern "system" fn(n: GLsizei, memoryObjects: *mut GLuint);

pub type glCreatePerfQueryINTEL_t = unsafe extern "system" fn(queryId: GLuint, queryHandle: *mut GLuint);

pub type glCreateProgram_t = unsafe extern "system" fn() -> GLuint;

pub type glCreateProgramObjectARB_t = unsafe extern "system" fn() -> GLhandleARB;

pub type glCreateProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

pub type glCreateProgressFenceNVX_t = unsafe extern "system" fn() -> GLuint;

pub type glCreateQueries_t = unsafe extern "system" fn(target: GLenum, n: GLsizei, ids: *mut GLuint);

pub type glCreateRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

pub type glCreateSamplers_t = unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint);

pub type glCreateSemaphoresNV_t = unsafe extern "system" fn(n: GLsizei, semaphores: *mut GLuint);

pub type glCreateShader_t = unsafe extern "system" fn(type_: GLenum) -> GLuint;

pub type glCreateShaderObjectARB_t = unsafe extern "system" fn(shaderType: GLenum) -> GLhandleARB;

pub type glCreateShaderProgramEXT_t = unsafe extern "system" fn(type_: GLenum, string: *const GLchar) -> GLuint;

pub type glCreateShaderProgramv_t = unsafe extern "system" fn(type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint;

pub type glCreateShaderProgramvEXT_t = unsafe extern "system" fn(type_: GLenum, count: GLsizei, strings: *const *mut GLchar) -> GLuint;

pub type glCreateStatesNV_t = unsafe extern "system" fn(n: GLsizei, states: *mut GLuint);

pub type glCreateSyncFromCLeventARB_t = unsafe extern "system" fn(context: *mut _cl_context, event: *mut _cl_event, flags: GLbitfield) -> GLsync;

pub type glCreateTextures_t = unsafe extern "system" fn(target: GLenum, n: GLsizei, textures: *mut GLuint);

pub type glCreateTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub type glCreateVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

pub type glCullFace_t = unsafe extern "system" fn(mode: GLenum);

pub type glCullParameterdvEXT_t = unsafe extern "system" fn(pname: GLenum, params: *mut [GLdouble; 4]);

pub type glCullParameterfvEXT_t = unsafe extern "system" fn(pname: GLenum, params: *mut [GLfloat; 4]);

pub type glCurrentPaletteMatrixARB_t = unsafe extern "system" fn(index: GLint);

pub type glCurrentPaletteMatrixOES_t = unsafe extern "system" fn(matrixpaletteindex: GLuint);

pub type glDebugMessageCallback_t = unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const void);

pub type glDebugMessageCallbackAMD_t = unsafe extern "system" fn(callback: GLDEBUGPROCAMD, userParam: *mut void);

pub type glDebugMessageCallbackARB_t = unsafe extern "system" fn(callback: GLDEBUGPROCARB, userParam: *const void);

pub type glDebugMessageCallbackKHR_t = unsafe extern "system" fn(callback: GLDEBUGPROCKHR, userParam: *const void);

pub type glDebugMessageControl_t = unsafe extern "system" fn(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

pub type glDebugMessageControlARB_t = unsafe extern "system" fn(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

pub type glDebugMessageControlKHR_t = unsafe extern "system" fn(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

pub type glDebugMessageEnableAMD_t = unsafe extern "system" fn(category: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

pub type glDebugMessageInsert_t = unsafe extern "system" fn(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);

pub type glDebugMessageInsertAMD_t = unsafe extern "system" fn(category: GLenum, severity: GLenum, id: GLuint, length: GLsizei, buf: *const GLchar);

pub type glDebugMessageInsertARB_t = unsafe extern "system" fn(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);

pub type glDebugMessageInsertKHR_t = unsafe extern "system" fn(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);

pub type glDeformSGIX_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glDeformationMap3dSGIX_t = unsafe extern "system" fn(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, w1: GLdouble, w2: GLdouble, wstride: GLint, worder: GLint, points: *const GLdouble);

pub type glDeformationMap3fSGIX_t = unsafe extern "system" fn(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, w1: GLfloat, w2: GLfloat, wstride: GLint, worder: GLint, points: *const GLfloat);

pub type glDeleteAsyncMarkersSGIX_t = unsafe extern "system" fn(marker: GLuint, range: GLsizei);

pub type glDeleteBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);

pub type glDeleteBuffersARB_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);

pub type glDeleteCommandListsNV_t = unsafe extern "system" fn(n: GLsizei, lists: *const GLuint);

pub type glDeleteFencesAPPLE_t = unsafe extern "system" fn(n: GLsizei, fences: *const GLuint);

pub type glDeleteFencesNV_t = unsafe extern "system" fn(n: GLsizei, fences: *const GLuint);

pub type glDeleteFragmentShaderATI_t = unsafe extern "system" fn(id: GLuint);

pub type glDeleteFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

pub type glDeleteFramebuffersEXT_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

pub type glDeleteFramebuffersOES_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

pub type glDeleteLists_t = unsafe extern "system" fn(list: GLuint, range: GLsizei);

pub type glDeleteMemoryObjectsEXT_t = unsafe extern "system" fn(n: GLsizei, memoryObjects: *const GLuint);

pub type glDeleteNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar);

pub type glDeleteNamesAMD_t = unsafe extern "system" fn(identifier: GLenum, num: GLuint, names: *const GLuint);

pub type glDeleteObjectARB_t = unsafe extern "system" fn(obj: GLhandleARB);

pub type glDeleteOcclusionQueriesNV_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub type glDeletePathsNV_t = unsafe extern "system" fn(path: GLuint, range: GLsizei);

pub type glDeletePerfMonitorsAMD_t = unsafe extern "system" fn(n: GLsizei, monitors: *mut GLuint);

pub type glDeletePerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

pub type glDeleteProgram_t = unsafe extern "system" fn(program: GLuint);

pub type glDeleteProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);

pub type glDeleteProgramPipelinesEXT_t = unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);

pub type glDeleteProgramsARB_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

pub type glDeleteProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

pub type glDeleteQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub type glDeleteQueriesARB_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub type glDeleteQueriesEXT_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub type glDeleteQueryResourceTagNV_t = unsafe extern "system" fn(n: GLsizei, tagIds: *const GLint);

pub type glDeleteRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

pub type glDeleteRenderbuffersEXT_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

pub type glDeleteRenderbuffersOES_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

pub type glDeleteSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);

pub type glDeleteSemaphoresEXT_t = unsafe extern "system" fn(n: GLsizei, semaphores: *const GLuint);

pub type glDeleteShader_t = unsafe extern "system" fn(shader: GLuint);

pub type glDeleteStatesNV_t = unsafe extern "system" fn(n: GLsizei, states: *const GLuint);

pub type glDeleteSync_t = unsafe extern "system" fn(sync: GLsync);

pub type glDeleteSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync);

pub type glDeleteTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);

pub type glDeleteTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);

pub type glDeleteTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub type glDeleteTransformFeedbacksNV_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub type glDeleteVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

pub type glDeleteVertexArraysAPPLE_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

pub type glDeleteVertexArraysOES_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

pub type glDeleteVertexShaderEXT_t = unsafe extern "system" fn(id: GLuint);

pub type glDepthBoundsEXT_t = unsafe extern "system" fn(zmin: GLclampd, zmax: GLclampd);

pub type glDepthBoundsdNV_t = unsafe extern "system" fn(zmin: GLdouble, zmax: GLdouble);

pub type glDepthFunc_t = unsafe extern "system" fn(func: GLenum);

pub type glDepthMask_t = unsafe extern "system" fn(flag: GLboolean);

pub type glDepthRange_t = unsafe extern "system" fn(n: GLdouble, f: GLdouble);

pub type glDepthRangeArraydvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);

pub type glDepthRangeArrayfvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

pub type glDepthRangeArrayfvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

pub type glDepthRangeArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);

pub type glDepthRangeIndexed_t = unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);

pub type glDepthRangeIndexeddNV_t = unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);

pub type glDepthRangeIndexedfNV_t = unsafe extern "system" fn(index: GLuint, n: GLfloat, f: GLfloat);

pub type glDepthRangeIndexedfOES_t = unsafe extern "system" fn(index: GLuint, n: GLfloat, f: GLfloat);

pub type glDepthRangedNV_t = unsafe extern "system" fn(zNear: GLdouble, zFar: GLdouble);

pub type glDepthRangef_t = unsafe extern "system" fn(n: GLfloat, f: GLfloat);

pub type glDepthRangefOES_t = unsafe extern "system" fn(n: GLclampf, f: GLclampf);

pub type glDepthRangex_t = unsafe extern "system" fn(n: GLfixed, f: GLfixed);

pub type glDepthRangexOES_t = unsafe extern "system" fn(n: GLfixed, f: GLfixed);

pub type glDetachObjectARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, attachedObj: GLhandleARB);

pub type glDetachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);

pub type glDetailTexFuncSGIS_t = unsafe extern "system" fn(target: GLenum, n: GLsizei, points: *const GLfloat);

pub type glDisable_t = unsafe extern "system" fn(cap: GLenum);

pub type glDisableClientState_t = unsafe extern "system" fn(array: GLenum);

pub type glDisableClientStateIndexedEXT_t = unsafe extern "system" fn(array: GLenum, index: GLuint);

pub type glDisableClientStateiEXT_t = unsafe extern "system" fn(array: GLenum, index: GLuint);

pub type glDisableDriverControlQCOM_t = unsafe extern "system" fn(driverControl: GLuint);

pub type glDisableIndexedEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glDisableVariantClientStateEXT_t = unsafe extern "system" fn(id: GLuint);

pub type glDisableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

pub type glDisableVertexArrayAttribEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

pub type glDisableVertexArrayEXT_t = unsafe extern "system" fn(vaobj: GLuint, array: GLenum);

pub type glDisableVertexAttribAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum);

pub type glDisableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

pub type glDisableVertexAttribArrayARB_t = unsafe extern "system" fn(index: GLuint);

pub type glDisablei_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glDisableiEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glDisableiNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glDisableiOES_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glDiscardFramebufferEXT_t = unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);

pub type glDispatchCompute_t = unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);

pub type glDispatchComputeGroupSizeARB_t = unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint, group_size_x: GLuint, group_size_y: GLuint, group_size_z: GLuint);

pub type glDispatchComputeIndirect_t = unsafe extern "system" fn(indirect: GLintptr);

pub type glDrawArrays_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);

pub type glDrawArraysEXT_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);

pub type glDrawArraysIndirect_t = unsafe extern "system" fn(mode: GLenum, indirect: *const void);

pub type glDrawArraysInstanced_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);

pub type glDrawArraysInstancedANGLE_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei);

pub type glDrawArraysInstancedARB_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei);

pub type glDrawArraysInstancedBaseInstance_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);

pub type glDrawArraysInstancedBaseInstanceEXT_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);

pub type glDrawArraysInstancedEXT_t = unsafe extern "system" fn(mode: GLenum, start: GLint, count: GLsizei, primcount: GLsizei);

pub type glDrawArraysInstancedNV_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei);

pub type glDrawBuffer_t = unsafe extern "system" fn(buf: GLenum);

pub type glDrawBuffers_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

pub type glDrawBuffersARB_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

pub type glDrawBuffersATI_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

pub type glDrawBuffersEXT_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

pub type glDrawBuffersIndexedEXT_t = unsafe extern "system" fn(n: GLint, location: *const GLenum, indices: *const GLint);

pub type glDrawBuffersNV_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);

pub type glDrawCommandsAddressNV_t = unsafe extern "system" fn(primitiveMode: GLenum, indirects: *const GLuint64, sizes: *const GLsizei, count: GLuint);

pub type glDrawCommandsNV_t = unsafe extern "system" fn(primitiveMode: GLenum, buffer: GLuint, indirects: *const GLintptr, sizes: *const GLsizei, count: GLuint);

pub type glDrawCommandsStatesAddressNV_t = unsafe extern "system" fn(indirects: *const GLuint64, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

pub type glDrawCommandsStatesNV_t = unsafe extern "system" fn(buffer: GLuint, indirects: *const GLintptr, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

pub type glDrawElementArrayAPPLE_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);

pub type glDrawElementArrayATI_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei);

pub type glDrawElements_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void);

pub type glDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, basevertex: GLint);

pub type glDrawElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, basevertex: GLint);

pub type glDrawElementsBaseVertexOES_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, basevertex: GLint);

pub type glDrawElementsIndirect_t = unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const void);

pub type glDrawElementsInstanced_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, instancecount: GLsizei);

pub type glDrawElementsInstancedANGLE_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, primcount: GLsizei);

pub type glDrawElementsInstancedARB_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, primcount: GLsizei);

pub type glDrawElementsInstancedBaseInstance_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, instancecount: GLsizei, baseinstance: GLuint);

pub type glDrawElementsInstancedBaseInstanceEXT_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, instancecount: GLsizei, baseinstance: GLuint);

pub type glDrawElementsInstancedBaseVertex_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, instancecount: GLsizei, basevertex: GLint);

pub type glDrawElementsInstancedBaseVertexBaseInstance_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);

pub type glDrawElementsInstancedBaseVertexBaseInstanceEXT_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);

pub type glDrawElementsInstancedBaseVertexEXT_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, instancecount: GLsizei, basevertex: GLint);

pub type glDrawElementsInstancedBaseVertexOES_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, instancecount: GLsizei, basevertex: GLint);

pub type glDrawElementsInstancedEXT_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, primcount: GLsizei);

pub type glDrawElementsInstancedNV_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void, primcount: GLsizei);

pub type glDrawMeshArraysSUN_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, width: GLsizei);

pub type glDrawMeshTasksIndirectNV_t = unsafe extern "system" fn(indirect: GLintptr);

pub type glDrawMeshTasksNV_t = unsafe extern "system" fn(first: GLuint, count: GLuint);

pub type glDrawPixels_t = unsafe extern "system" fn(width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glDrawRangeElementArrayAPPLE_t = unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, first: GLint, count: GLsizei);

pub type glDrawRangeElementArrayATI_t = unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei);

pub type glDrawRangeElements_t = unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const void);

pub type glDrawRangeElementsBaseVertex_t = unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const void, basevertex: GLint);

pub type glDrawRangeElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const void, basevertex: GLint);

pub type glDrawRangeElementsBaseVertexOES_t = unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const void, basevertex: GLint);

pub type glDrawRangeElementsEXT_t = unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const void);

pub type glDrawTexfOES_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, width: GLfloat, height: GLfloat);

pub type glDrawTexfvOES_t = unsafe extern "system" fn(coords: *const [GLfloat; 5]);

pub type glDrawTexiOES_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, width: GLint, height: GLint);

pub type glDrawTexivOES_t = unsafe extern "system" fn(coords: *const [GLint; 5]);

pub type glDrawTexsOES_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, width: GLshort, height: GLshort);

pub type glDrawTexsvOES_t = unsafe extern "system" fn(coords: *const [GLshort; 5]);

pub type glDrawTextureNV_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint, x0: GLfloat, y0: GLfloat, x1: GLfloat, y1: GLfloat, z: GLfloat, s0: GLfloat, t0: GLfloat, s1: GLfloat, t1: GLfloat);

pub type glDrawTexxOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed, width: GLfixed, height: GLfixed);

pub type glDrawTexxvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 5]);

pub type glDrawTransformFeedback_t = unsafe extern "system" fn(mode: GLenum, id: GLuint);

pub type glDrawTransformFeedbackEXT_t = unsafe extern "system" fn(mode: GLenum, id: GLuint);

pub type glDrawTransformFeedbackInstanced_t = unsafe extern "system" fn(mode: GLenum, id: GLuint, instancecount: GLsizei);

pub type glDrawTransformFeedbackInstancedEXT_t = unsafe extern "system" fn(mode: GLenum, id: GLuint, instancecount: GLsizei);

pub type glDrawTransformFeedbackNV_t = unsafe extern "system" fn(mode: GLenum, id: GLuint);

pub type glDrawTransformFeedbackStream_t = unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint);

pub type glDrawTransformFeedbackStreamInstanced_t = unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei);

pub type glDrawVkImageNV_t = unsafe extern "system" fn(vkImage: GLuint64, sampler: GLuint, x0: GLfloat, y0: GLfloat, x1: GLfloat, y1: GLfloat, z: GLfloat, s0: GLfloat, t0: GLfloat, s1: GLfloat, t1: GLfloat);

pub type glEGLImageTargetRenderbufferStorageOES_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES);

pub type glEGLImageTargetTexStorageEXT_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES, attrib_list: *const GLint);

pub type glEGLImageTargetTexture2DOES_t = unsafe extern "system" fn(target: GLenum, image: GLeglImageOES);

pub type glEGLImageTargetTextureStorageEXT_t = unsafe extern "system" fn(texture: GLuint, image: GLeglImageOES, attrib_list: *const GLint);

pub type glEdgeFlag_t = unsafe extern "system" fn(flag: GLboolean);

pub type glEdgeFlagFormatNV_t = unsafe extern "system" fn(stride: GLsizei);

pub type glEdgeFlagPointer_t = unsafe extern "system" fn(stride: GLsizei, pointer: *const void);

pub type glEdgeFlagPointerEXT_t = unsafe extern "system" fn(stride: GLsizei, count: GLsizei, pointer: *const GLboolean);

pub type glEdgeFlagPointerListIBM_t = unsafe extern "system" fn(stride: GLint, pointer: *const *mut GLboolean, ptrstride: GLint);

pub type glEdgeFlagv_t = unsafe extern "system" fn(flag: *const GLboolean);

pub type glElementPointerAPPLE_t = unsafe extern "system" fn(type_: GLenum, pointer: *const void);

pub type glElementPointerATI_t = unsafe extern "system" fn(type_: GLenum, pointer: *const void);

pub type glEnable_t = unsafe extern "system" fn(cap: GLenum);

pub type glEnableClientState_t = unsafe extern "system" fn(array: GLenum);

pub type glEnableClientStateIndexedEXT_t = unsafe extern "system" fn(array: GLenum, index: GLuint);

pub type glEnableClientStateiEXT_t = unsafe extern "system" fn(array: GLenum, index: GLuint);

pub type glEnableDriverControlQCOM_t = unsafe extern "system" fn(driverControl: GLuint);

pub type glEnableIndexedEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glEnableVariantClientStateEXT_t = unsafe extern "system" fn(id: GLuint);

pub type glEnableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

pub type glEnableVertexArrayAttribEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

pub type glEnableVertexArrayEXT_t = unsafe extern "system" fn(vaobj: GLuint, array: GLenum);

pub type glEnableVertexAttribAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum);

pub type glEnableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

pub type glEnableVertexAttribArrayARB_t = unsafe extern "system" fn(index: GLuint);

pub type glEnablei_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glEnableiEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glEnableiNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glEnableiOES_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glEnd_t = unsafe extern "system" fn();

pub type glEndConditionalRender_t = unsafe extern "system" fn();

pub type glEndConditionalRenderNV_t = unsafe extern "system" fn();

pub type glEndConditionalRenderNVX_t = unsafe extern "system" fn();

pub type glEndFragmentShaderATI_t = unsafe extern "system" fn();

pub type glEndList_t = unsafe extern "system" fn();

pub type glEndOcclusionQueryNV_t = unsafe extern "system" fn();

pub type glEndPerfMonitorAMD_t = unsafe extern "system" fn(monitor: GLuint);

pub type glEndPerfQueryINTEL_t = unsafe extern "system" fn(queryHandle: GLuint);

pub type glEndQuery_t = unsafe extern "system" fn(target: GLenum);

pub type glEndQueryARB_t = unsafe extern "system" fn(target: GLenum);

pub type glEndQueryEXT_t = unsafe extern "system" fn(target: GLenum);

pub type glEndQueryIndexed_t = unsafe extern "system" fn(target: GLenum, index: GLuint);

pub type glEndTilingQCOM_t = unsafe extern "system" fn(preserveMask: GLbitfield);

pub type glEndTransformFeedback_t = unsafe extern "system" fn();

pub type glEndTransformFeedbackEXT_t = unsafe extern "system" fn();

pub type glEndTransformFeedbackNV_t = unsafe extern "system" fn();

pub type glEndVertexShaderEXT_t = unsafe extern "system" fn();

pub type glEndVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint);

pub type glEvalCoord1d_t = unsafe extern "system" fn(u: GLdouble);

pub type glEvalCoord1dv_t = unsafe extern "system" fn(u: *const GLdouble);

pub type glEvalCoord1f_t = unsafe extern "system" fn(u: GLfloat);

pub type glEvalCoord1fv_t = unsafe extern "system" fn(u: *const GLfloat);

pub type glEvalCoord1xOES_t = unsafe extern "system" fn(u: GLfixed);

pub type glEvalCoord1xvOES_t = unsafe extern "system" fn(coords: *const GLfixed);

pub type glEvalCoord2d_t = unsafe extern "system" fn(u: GLdouble, v: GLdouble);

pub type glEvalCoord2dv_t = unsafe extern "system" fn(u: *const [GLdouble; 2]);

pub type glEvalCoord2f_t = unsafe extern "system" fn(u: GLfloat, v: GLfloat);

pub type glEvalCoord2fv_t = unsafe extern "system" fn(u: *const [GLfloat; 2]);

pub type glEvalCoord2xOES_t = unsafe extern "system" fn(u: GLfixed, v: GLfixed);

pub type glEvalCoord2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

pub type glEvalMapsNV_t = unsafe extern "system" fn(target: GLenum, mode: GLenum);

pub type glEvalMesh1_t = unsafe extern "system" fn(mode: GLenum, i1: GLint, i2: GLint);

pub type glEvalMesh2_t = unsafe extern "system" fn(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint);

pub type glEvalPoint1_t = unsafe extern "system" fn(i: GLint);

pub type glEvalPoint2_t = unsafe extern "system" fn(i: GLint, j: GLint);

pub type glEvaluateDepthValuesARB_t = unsafe extern "system" fn();

pub type glExecuteProgramNV_t = unsafe extern "system" fn(target: GLenum, id: GLuint, params: *const [GLfloat; 4]);

pub type glExtGetBufferPointervQCOM_t = unsafe extern "system" fn(target: GLenum, params: *mut *mut void);

pub type glExtGetBuffersQCOM_t = unsafe extern "system" fn(buffers: *mut GLuint, maxBuffers: GLint, numBuffers: *mut GLint);

pub type glExtGetFramebuffersQCOM_t = unsafe extern "system" fn(framebuffers: *mut GLuint, maxFramebuffers: GLint, numFramebuffers: *mut GLint);

pub type glExtGetProgramBinarySourceQCOM_t = unsafe extern "system" fn(program: GLuint, shadertype: GLenum, source: *mut GLchar, length: *mut GLint);

pub type glExtGetProgramsQCOM_t = unsafe extern "system" fn(programs: *mut GLuint, maxPrograms: GLint, numPrograms: *mut GLint);

pub type glExtGetRenderbuffersQCOM_t = unsafe extern "system" fn(renderbuffers: *mut GLuint, maxRenderbuffers: GLint, numRenderbuffers: *mut GLint);

pub type glExtGetShadersQCOM_t = unsafe extern "system" fn(shaders: *mut GLuint, maxShaders: GLint, numShaders: *mut GLint);

pub type glExtGetTexLevelParameterivQCOM_t = unsafe extern "system" fn(texture: GLuint, face: GLenum, level: GLint, pname: GLenum, params: *mut GLint);

pub type glExtGetTexSubImageQCOM_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, texels: *mut void);

pub type glExtGetTexturesQCOM_t = unsafe extern "system" fn(textures: *mut GLuint, maxTextures: GLint, numTextures: *mut GLint);

pub type glExtIsProgramBinaryQCOM_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

pub type glExtTexObjectStateOverrideiQCOM_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

pub type glExtractComponentEXT_t = unsafe extern "system" fn(res: GLuint, src: GLuint, num: GLuint);

pub type glExtrapolateTex2DQCOM_t = unsafe extern "system" fn(src1: GLuint, src2: GLuint, output: GLuint, scaleFactor: GLfloat);

pub type glFeedbackBuffer_t = unsafe extern "system" fn(size: GLsizei, type_: GLenum, buffer: *mut GLfloat);

pub type glFeedbackBufferxOES_t = unsafe extern "system" fn(n: GLsizei, type_: GLenum, buffer: *const GLfixed);

pub type glFenceSync_t = unsafe extern "system" fn(condition: GLenum, flags: GLbitfield) -> GLsync;

pub type glFenceSyncAPPLE_t = unsafe extern "system" fn(condition: GLenum, flags: GLbitfield) -> GLsync;

pub type glFinalCombinerInputNV_t = unsafe extern "system" fn(variable: GLenum, input: GLenum, mapping: GLenum, componentUsage: GLenum);

pub type glFinish_t = unsafe extern "system" fn();

pub type glFinishAsyncSGIX_t = unsafe extern "system" fn(markerp: *mut GLuint) -> GLint;

pub type glFinishFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint);

pub type glFinishFenceNV_t = unsafe extern "system" fn(fence: GLuint);

pub type glFinishObjectAPPLE_t = unsafe extern "system" fn(object: GLenum, name: GLint);

pub type glFinishTextureSUNX_t = unsafe extern "system" fn();

pub type glFlush_t = unsafe extern "system" fn();

pub type glFlushMappedBufferRange_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr);

pub type glFlushMappedBufferRangeAPPLE_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr);

pub type glFlushMappedBufferRangeEXT_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr);

pub type glFlushMappedNamedBufferRange_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

pub type glFlushMappedNamedBufferRangeEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

pub type glFlushPixelDataRangeNV_t = unsafe extern "system" fn(target: GLenum);

pub type glFlushRasterSGIX_t = unsafe extern "system" fn();

pub type glFlushStaticDataIBM_t = unsafe extern "system" fn(target: GLenum);

pub type glFlushVertexArrayRangeAPPLE_t = unsafe extern "system" fn(length: GLsizei, pointer: *mut void);

pub type glFlushVertexArrayRangeNV_t = unsafe extern "system" fn();

pub type glFogCoordFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

pub type glFogCoordPointer_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glFogCoordPointerEXT_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glFogCoordPointerListIBM_t = unsafe extern "system" fn(type_: GLenum, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

pub type glFogCoordd_t = unsafe extern "system" fn(coord: GLdouble);

pub type glFogCoorddEXT_t = unsafe extern "system" fn(coord: GLdouble);

pub type glFogCoorddv_t = unsafe extern "system" fn(coord: *const GLdouble);

pub type glFogCoorddvEXT_t = unsafe extern "system" fn(coord: *const GLdouble);

pub type glFogCoordf_t = unsafe extern "system" fn(coord: GLfloat);

pub type glFogCoordfEXT_t = unsafe extern "system" fn(coord: GLfloat);

pub type glFogCoordfv_t = unsafe extern "system" fn(coord: *const GLfloat);

pub type glFogCoordfvEXT_t = unsafe extern "system" fn(coord: *const GLfloat);

pub type glFogCoordhNV_t = unsafe extern "system" fn(fog: GLhalfNV);

pub type glFogCoordhvNV_t = unsafe extern "system" fn(fog: *const GLhalfNV);

pub type glFogFuncSGIS_t = unsafe extern "system" fn(n: GLsizei, points: *const GLfloat);

pub type glFogf_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glFogfv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glFogi_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glFogiv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);

pub type glFogx_t = unsafe extern "system" fn(pname: GLenum, param: GLfixed);

pub type glFogxOES_t = unsafe extern "system" fn(pname: GLenum, param: GLfixed);

pub type glFogxv_t = unsafe extern "system" fn(pname: GLenum, param: *const GLfixed);

pub type glFogxvOES_t = unsafe extern "system" fn(pname: GLenum, param: *const GLfixed);

pub type glFragmentColorMaterialSGIX_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);

pub type glFragmentCoverageColorNV_t = unsafe extern "system" fn(color: GLuint);

pub type glFragmentLightModelfSGIX_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glFragmentLightModelfvSGIX_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glFragmentLightModeliSGIX_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glFragmentLightModelivSGIX_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);

pub type glFragmentLightfSGIX_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, param: GLfloat);

pub type glFragmentLightfvSGIX_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *const GLfloat);

pub type glFragmentLightiSGIX_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, param: GLint);

pub type glFragmentLightivSGIX_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *const GLint);

pub type glFragmentMaterialfSGIX_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: GLfloat);

pub type glFragmentMaterialfvSGIX_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *const GLfloat);

pub type glFragmentMaterialiSGIX_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: GLint);

pub type glFragmentMaterialivSGIX_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *const GLint);

pub type glFrameTerminatorGREMEDY_t = unsafe extern "system" fn();

pub type glFrameZoomSGIX_t = unsafe extern "system" fn(factor: GLint);

pub type glFramebufferDrawBufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, mode: GLenum);

pub type glFramebufferDrawBuffersEXT_t = unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);

pub type glFramebufferFetchBarrierEXT_t = unsafe extern "system" fn();

pub type glFramebufferFetchBarrierQCOM_t = unsafe extern "system" fn();

pub type glFramebufferFoveationConfigQCOM_t = unsafe extern "system" fn(framebuffer: GLuint, numLayers: GLuint, focalPointsPerLayer: GLuint, requestedFeatures: GLuint, providedFeatures: *mut GLuint);

pub type glFramebufferFoveationParametersQCOM_t = unsafe extern "system" fn(framebuffer: GLuint, layer: GLuint, focalPoint: GLuint, focalX: GLfloat, focalY: GLfloat, gainX: GLfloat, gainY: GLfloat, foveaArea: GLfloat);

pub type glFramebufferParameteri_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

pub type glFramebufferParameteriMESA_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

pub type glFramebufferPixelLocalStorageSizeEXT_t = unsafe extern "system" fn(target: GLuint, size: GLsizei);

pub type glFramebufferReadBufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, mode: GLenum);

pub type glFramebufferRenderbuffer_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);

pub type glFramebufferRenderbufferEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);

pub type glFramebufferRenderbufferOES_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);

pub type glFramebufferSampleLocationsfvARB_t = unsafe extern "system" fn(target: GLenum, start: GLuint, count: GLsizei, v: *const GLfloat);

pub type glFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(target: GLenum, start: GLuint, count: GLsizei, v: *const GLfloat);

pub type glFramebufferSamplePositionsfvAMD_t = unsafe extern "system" fn(target: GLenum, numsamples: GLuint, pixelindex: GLuint, values: *const GLfloat);

pub type glFramebufferTexture_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);

pub type glFramebufferTexture1D_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);

pub type glFramebufferTexture1DEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);

pub type glFramebufferTexture2D_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);

pub type glFramebufferTexture2DDownsampleIMG_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, xscale: GLint, yscale: GLint);

pub type glFramebufferTexture2DEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);

pub type glFramebufferTexture2DMultisampleEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, samples: GLsizei);

pub type glFramebufferTexture2DMultisampleIMG_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, samples: GLsizei);

pub type glFramebufferTexture2DOES_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);

pub type glFramebufferTexture3D_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);

pub type glFramebufferTexture3DEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);

pub type glFramebufferTexture3DOES_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);

pub type glFramebufferTextureARB_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);

pub type glFramebufferTextureEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);

pub type glFramebufferTextureFaceARB_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);

pub type glFramebufferTextureFaceEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);

pub type glFramebufferTextureLayer_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);

pub type glFramebufferTextureLayerARB_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);

pub type glFramebufferTextureLayerDownsampleIMG_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint, xscale: GLint, yscale: GLint);

pub type glFramebufferTextureLayerEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);

pub type glFramebufferTextureMultisampleMultiviewOVR_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, samples: GLsizei, baseViewIndex: GLint, numViews: GLsizei);

pub type glFramebufferTextureMultiviewOVR_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, baseViewIndex: GLint, numViews: GLsizei);

pub type glFramebufferTextureOES_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);

pub type glFreeObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint);

pub type glFrontFace_t = unsafe extern "system" fn(mode: GLenum);

pub type glFrustum_t = unsafe extern "system" fn(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

pub type glFrustumf_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

pub type glFrustumfOES_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

pub type glFrustumx_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

pub type glFrustumxOES_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

pub type glGenAsyncMarkersSGIX_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

pub type glGenBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

pub type glGenBuffersARB_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

pub type glGenFencesAPPLE_t = unsafe extern "system" fn(n: GLsizei, fences: *mut GLuint);

pub type glGenFencesNV_t = unsafe extern "system" fn(n: GLsizei, fences: *mut GLuint);

pub type glGenFragmentShadersATI_t = unsafe extern "system" fn(range: GLuint) -> GLuint;

pub type glGenFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

pub type glGenFramebuffersEXT_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

pub type glGenFramebuffersOES_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

pub type glGenLists_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

pub type glGenNamesAMD_t = unsafe extern "system" fn(identifier: GLenum, num: GLuint, names: *mut GLuint);

pub type glGenOcclusionQueriesNV_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub type glGenPathsNV_t = unsafe extern "system" fn(range: GLsizei) -> GLuint;

pub type glGenPerfMonitorsAMD_t = unsafe extern "system" fn(n: GLsizei, monitors: *mut GLuint);

pub type glGenProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

pub type glGenProgramPipelinesEXT_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

pub type glGenProgramsARB_t = unsafe extern "system" fn(n: GLsizei, programs: *mut GLuint);

pub type glGenProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *mut GLuint);

pub type glGenQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub type glGenQueriesARB_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub type glGenQueriesEXT_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub type glGenQueryResourceTagNV_t = unsafe extern "system" fn(n: GLsizei, tagIds: *mut GLint);

pub type glGenRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

pub type glGenRenderbuffersEXT_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

pub type glGenRenderbuffersOES_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

pub type glGenSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);

pub type glGenSemaphoresEXT_t = unsafe extern "system" fn(n: GLsizei, semaphores: *mut GLuint);

pub type glGenSymbolsEXT_t = unsafe extern "system" fn(datatype: GLenum, storagetype: GLenum, range: GLenum, components: GLuint) -> GLuint;

pub type glGenTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);

pub type glGenTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);

pub type glGenTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub type glGenTransformFeedbacksNV_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub type glGenVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

pub type glGenVertexArraysAPPLE_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

pub type glGenVertexArraysOES_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

pub type glGenVertexShadersEXT_t = unsafe extern "system" fn(range: GLuint) -> GLuint;

pub type glGenerateMipmap_t = unsafe extern "system" fn(target: GLenum);

pub type glGenerateMipmapEXT_t = unsafe extern "system" fn(target: GLenum);

pub type glGenerateMipmapOES_t = unsafe extern "system" fn(target: GLenum);

pub type glGenerateMultiTexMipmapEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum);

pub type glGenerateTextureMipmap_t = unsafe extern "system" fn(texture: GLuint);

pub type glGenerateTextureMipmapEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum);

pub type glGetActiveAtomicCounterBufferiv_t = unsafe extern "system" fn(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetActiveAttrib_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);

pub type glGetActiveAttribARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLcharARB);

pub type glGetActiveSubroutineName_t = unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

pub type glGetActiveSubroutineUniformName_t = unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

pub type glGetActiveSubroutineUniformiv_t = unsafe extern "system" fn(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint);

pub type glGetActiveUniform_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);

pub type glGetActiveUniformARB_t = unsafe extern "system" fn(programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLcharARB);

pub type glGetActiveUniformBlockName_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);

pub type glGetActiveUniformBlockiv_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetActiveUniformName_t = unsafe extern "system" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);

pub type glGetActiveUniformsiv_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint);

pub type glGetActiveVaryingNV_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);

pub type glGetArrayObjectfvATI_t = unsafe extern "system" fn(array: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetArrayObjectivATI_t = unsafe extern "system" fn(array: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetAttachedObjectsARB_t = unsafe extern "system" fn(containerObj: GLhandleARB, maxCount: GLsizei, count: *mut GLsizei, obj: *mut GLhandleARB);

pub type glGetAttachedShaders_t = unsafe extern "system" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);

pub type glGetAttribLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub type glGetAttribLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, name: *const GLcharARB) -> GLint;

pub type glGetBooleanIndexedvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean);

pub type glGetBooleani_v_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean);

pub type glGetBooleanv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLboolean);

pub type glGetBufferParameteri64v_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint64);

pub type glGetBufferParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetBufferParameterivARB_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetBufferParameterui64vNV_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint64EXT);

pub type glGetBufferPointerv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut void);

pub type glGetBufferPointervARB_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut void);

pub type glGetBufferPointervOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut void);

pub type glGetBufferSubData_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut void);

pub type glGetBufferSubDataARB_t = unsafe extern "system" fn(target: GLenum, offset: GLintptrARB, size: GLsizeiptrARB, data: *mut void);

pub type glGetClipPlane_t = unsafe extern "system" fn(plane: GLenum, equation: *mut [GLdouble; 4]);

pub type glGetClipPlanef_t = unsafe extern "system" fn(plane: GLenum, equation: *mut [GLfloat; 4]);

pub type glGetClipPlanefOES_t = unsafe extern "system" fn(plane: GLenum, equation: *mut [GLfloat; 4]);

pub type glGetClipPlanex_t = unsafe extern "system" fn(plane: GLenum, equation: *mut [GLfixed; 4]);

pub type glGetClipPlanexOES_t = unsafe extern "system" fn(plane: GLenum, equation: *mut [GLfixed; 4]);

pub type glGetColorTable_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, table: *mut void);

pub type glGetColorTableEXT_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, data: *mut void);

pub type glGetColorTableParameterfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetColorTableParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetColorTableParameterfvSGI_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetColorTableParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetColorTableParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetColorTableParameterivSGI_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetColorTableSGI_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, table: *mut void);

pub type glGetCombinerInputParameterfvNV_t = unsafe extern "system" fn(stage: GLenum, portion: GLenum, variable: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetCombinerInputParameterivNV_t = unsafe extern "system" fn(stage: GLenum, portion: GLenum, variable: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetCombinerOutputParameterfvNV_t = unsafe extern "system" fn(stage: GLenum, portion: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetCombinerOutputParameterivNV_t = unsafe extern "system" fn(stage: GLenum, portion: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetCombinerStageParameterfvNV_t = unsafe extern "system" fn(stage: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetCommandHeaderNV_t = unsafe extern "system" fn(tokenID: GLenum, size: GLuint) -> GLuint;

pub type glGetCompressedMultiTexImageEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, lod: GLint, img: *mut void);

pub type glGetCompressedTexImage_t = unsafe extern "system" fn(target: GLenum, level: GLint, img: *mut void);

pub type glGetCompressedTexImageARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, img: *mut void);

pub type glGetCompressedTextureImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut void);

pub type glGetCompressedTextureImageEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, lod: GLint, img: *mut void);

pub type glGetCompressedTextureSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut void);

pub type glGetConvolutionFilter_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, image: *mut void);

pub type glGetConvolutionFilterEXT_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, image: *mut void);

pub type glGetConvolutionParameterfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetConvolutionParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetConvolutionParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetConvolutionParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetConvolutionParameterxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetCoverageModulationTableNV_t = unsafe extern "system" fn(bufSize: GLsizei, v: *mut GLfloat);

pub type glGetDebugMessageLog_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;

pub type glGetDebugMessageLogAMD_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, categories: *mut GLenum, severities: *mut GLuint, ids: *mut GLuint, lengths: *mut GLsizei, message: *mut GLchar) -> GLuint;

pub type glGetDebugMessageLogARB_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;

pub type glGetDebugMessageLogKHR_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;

pub type glGetDetailTexFuncSGIS_t = unsafe extern "system" fn(target: GLenum, points: *mut GLfloat);

pub type glGetDoubleIndexedvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLdouble);

pub type glGetDoublei_v_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLdouble);

pub type glGetDoublei_vEXT_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, params: *mut GLdouble);

pub type glGetDoublev_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLdouble);

pub type glGetDriverControlStringQCOM_t = unsafe extern "system" fn(driverControl: GLuint, bufSize: GLsizei, length: *mut GLsizei, driverControlString: *mut GLchar);

pub type glGetDriverControlsQCOM_t = unsafe extern "system" fn(num: *mut GLint, size: GLsizei, driverControls: *mut GLuint);

pub type glGetError_t = unsafe extern "system" fn() -> GLenum;

pub type glGetFenceivNV_t = unsafe extern "system" fn(fence: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetFinalCombinerInputParameterfvNV_t = unsafe extern "system" fn(variable: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetFinalCombinerInputParameterivNV_t = unsafe extern "system" fn(variable: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetFirstPerfQueryIdINTEL_t = unsafe extern "system" fn(queryId: *mut GLuint);

pub type glGetFixedv_t = unsafe extern "system" fn(pname: GLenum, params: *mut GLfixed);

pub type glGetFixedvOES_t = unsafe extern "system" fn(pname: GLenum, params: *mut GLfixed);

pub type glGetFloatIndexedvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLfloat);

pub type glGetFloati_v_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLfloat);

pub type glGetFloati_vEXT_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, params: *mut GLfloat);

pub type glGetFloati_vNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLfloat);

pub type glGetFloati_vOES_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLfloat);

pub type glGetFloatv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLfloat);

pub type glGetFogFuncSGIS_t = unsafe extern "system" fn(points: *mut GLfloat);

pub type glGetFragDataIndex_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub type glGetFragDataIndexEXT_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub type glGetFragDataLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub type glGetFragDataLocationEXT_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub type glGetFragmentLightfvSGIX_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetFragmentLightivSGIX_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetFragmentMaterialfvSGIX_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetFragmentMaterialivSGIX_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetFramebufferAttachmentParameterivEXT_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetFramebufferAttachmentParameterivOES_t = unsafe extern "system" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetFramebufferParameterfvAMD_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, numsamples: GLuint, pixelindex: GLuint, size: GLsizei, values: *mut GLfloat);

pub type glGetFramebufferParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetFramebufferParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetFramebufferParameterivMESA_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetFramebufferPixelLocalStorageSizeEXT_t = unsafe extern "system" fn(target: GLuint) -> GLsizei;

pub type glGetGraphicsResetStatus_t = unsafe extern "system" fn() -> GLenum;

pub type glGetGraphicsResetStatusARB_t = unsafe extern "system" fn() -> GLenum;

pub type glGetGraphicsResetStatusEXT_t = unsafe extern "system" fn() -> GLenum;

pub type glGetGraphicsResetStatusKHR_t = unsafe extern "system" fn() -> GLenum;

pub type glGetHandleARB_t = unsafe extern "system" fn(pname: GLenum) -> GLhandleARB;

pub type glGetHistogram_t = unsafe extern "system" fn(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, values: *mut void);

pub type glGetHistogramEXT_t = unsafe extern "system" fn(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, values: *mut void);

pub type glGetHistogramParameterfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetHistogramParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetHistogramParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetHistogramParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetHistogramParameterxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetImageHandleARB_t = unsafe extern "system" fn(texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: GLenum) -> GLuint64;

pub type glGetImageHandleNV_t = unsafe extern "system" fn(texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: GLenum) -> GLuint64;

pub type glGetImageTransformParameterfvHP_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetImageTransformParameterivHP_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetInfoLogARB_t = unsafe extern "system" fn(obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLcharARB);

pub type glGetInstrumentsSGIX_t = unsafe extern "system" fn() -> GLint;

pub type glGetInteger64i_v_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint64);

pub type glGetInteger64v_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint64);

pub type glGetInteger64vAPPLE_t = unsafe extern "system" fn(pname: GLenum, params: *mut GLint64);

pub type glGetInteger64vEXT_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint64);

pub type glGetIntegerIndexedvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint);

pub type glGetIntegeri_v_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint);

pub type glGetIntegeri_vEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint);

pub type glGetIntegerui64i_vNV_t = unsafe extern "system" fn(value: GLenum, index: GLuint, result: *mut GLuint64EXT);

pub type glGetIntegerui64vNV_t = unsafe extern "system" fn(value: GLenum, result: *mut GLuint64EXT);

pub type glGetIntegerv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint);

pub type glGetInternalformatSampleivNV_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, samples: GLsizei, pname: GLenum, count: GLsizei, params: *mut GLint);

pub type glGetInternalformati64v_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64);

pub type glGetInternalformativ_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint);

pub type glGetInvariantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLboolean);

pub type glGetInvariantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLfloat);

pub type glGetInvariantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLint);

pub type glGetLightfv_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetLightiv_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetLightxOES_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetLightxv_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetLightxvOES_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetListParameterfvSGIX_t = unsafe extern "system" fn(list: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetListParameterivSGIX_t = unsafe extern "system" fn(list: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetLocalConstantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLboolean);

pub type glGetLocalConstantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLfloat);

pub type glGetLocalConstantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLint);

pub type glGetMapAttribParameterfvNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetMapAttribParameterivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetMapControlPointsNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, type_: GLenum, ustride: GLsizei, vstride: GLsizei, packed: GLboolean, points: *mut void);

pub type glGetMapParameterfvNV_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetMapParameterivNV_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetMapdv_t = unsafe extern "system" fn(target: GLenum, query: GLenum, v: *mut GLdouble);

pub type glGetMapfv_t = unsafe extern "system" fn(target: GLenum, query: GLenum, v: *mut GLfloat);

pub type glGetMapiv_t = unsafe extern "system" fn(target: GLenum, query: GLenum, v: *mut GLint);

pub type glGetMapxvOES_t = unsafe extern "system" fn(target: GLenum, query: GLenum, v: *mut GLfixed);

pub type glGetMaterialfv_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetMaterialiv_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetMaterialxOES_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: GLfixed);

pub type glGetMaterialxv_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetMaterialxvOES_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetMemoryObjectDetachedResourcesuivNV_t = unsafe extern "system" fn(memory: GLuint, pname: GLenum, first: GLint, count: GLsizei, params: *mut GLuint);

pub type glGetMemoryObjectParameterivEXT_t = unsafe extern "system" fn(memoryObject: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetMinmax_t = unsafe extern "system" fn(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, values: *mut void);

pub type glGetMinmaxEXT_t = unsafe extern "system" fn(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, values: *mut void);

pub type glGetMinmaxParameterfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetMinmaxParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetMinmaxParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetMinmaxParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetMultiTexEnvfvEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetMultiTexEnvivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetMultiTexGendvEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLdouble);

pub type glGetMultiTexGenfvEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetMultiTexGenivEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetMultiTexImageEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut void);

pub type glGetMultiTexLevelParameterfvEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);

pub type glGetMultiTexLevelParameterivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);

pub type glGetMultiTexParameterIivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetMultiTexParameterIuivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLuint);

pub type glGetMultiTexParameterfvEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetMultiTexParameterivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetMultisamplefv_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *mut GLfloat);

pub type glGetMultisamplefvNV_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *mut [GLfloat; 2]);

pub type glGetNamedBufferParameteri64v_t = unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint64);

pub type glGetNamedBufferParameteriv_t = unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetNamedBufferParameterivEXT_t = unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetNamedBufferParameterui64vNV_t = unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLuint64EXT);

pub type glGetNamedBufferPointerv_t = unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut *mut void);

pub type glGetNamedBufferPointervEXT_t = unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut *mut void);

pub type glGetNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void);

pub type glGetNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void);

pub type glGetNamedFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetNamedFramebufferAttachmentParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetNamedFramebufferParameterfvAMD_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, numsamples: GLuint, pixelindex: GLuint, size: GLsizei, values: *mut GLfloat);

pub type glGetNamedFramebufferParameteriv_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: *mut GLint);

pub type glGetNamedFramebufferParameterivEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetNamedProgramLocalParameterIivEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, params: *mut [GLint; 4]);

pub type glGetNamedProgramLocalParameterIuivEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, params: *mut [GLuint; 4]);

pub type glGetNamedProgramLocalParameterdvEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, params: *mut [GLdouble; 4]);

pub type glGetNamedProgramLocalParameterfvEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, params: *mut [GLfloat; 4]);

pub type glGetNamedProgramStringEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, pname: GLenum, string: *mut void);

pub type glGetNamedProgramivEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetNamedRenderbufferParameteriv_t = unsafe extern "system" fn(renderbuffer: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetNamedRenderbufferParameterivEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar, bufSize: GLsizei, stringlen: *mut GLint, string: *mut GLchar);

pub type glGetNamedStringivARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar, pname: GLenum, params: *mut GLint);

pub type glGetNextPerfQueryIdINTEL_t = unsafe extern "system" fn(queryId: GLuint, nextQueryId: *mut GLuint);

pub type glGetObjectBufferfvATI_t = unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetObjectBufferivATI_t = unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetObjectLabel_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub type glGetObjectLabelEXT_t = unsafe extern "system" fn(type_: GLenum, object: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub type glGetObjectLabelKHR_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub type glGetObjectParameterfvARB_t = unsafe extern "system" fn(obj: GLhandleARB, pname: GLenum, params: *mut GLfloat);

pub type glGetObjectParameterivAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetObjectParameterivARB_t = unsafe extern "system" fn(obj: GLhandleARB, pname: GLenum, params: *mut GLint);

pub type glGetObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub type glGetObjectPtrLabelKHR_t = unsafe extern "system" fn(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub type glGetOcclusionQueryivNV_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetOcclusionQueryuivNV_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetPathColorGenfvNV_t = unsafe extern "system" fn(color: GLenum, pname: GLenum, value: *mut GLfloat);

pub type glGetPathColorGenivNV_t = unsafe extern "system" fn(color: GLenum, pname: GLenum, value: *mut GLint);

pub type glGetPathCommandsNV_t = unsafe extern "system" fn(path: GLuint, commands: *mut GLubyte);

pub type glGetPathCoordsNV_t = unsafe extern "system" fn(path: GLuint, coords: *mut GLfloat);

pub type glGetPathDashArrayNV_t = unsafe extern "system" fn(path: GLuint, dashArray: *mut GLfloat);

pub type glGetPathLengthNV_t = unsafe extern "system" fn(path: GLuint, startSegment: GLsizei, numSegments: GLsizei) -> GLfloat;

pub type glGetPathMetricRangeNV_t = unsafe extern "system" fn(metricQueryMask: GLbitfield, firstPathName: GLuint, numPaths: GLsizei, stride: GLsizei, metrics: *mut GLfloat);

pub type glGetPathMetricsNV_t = unsafe extern "system" fn(metricQueryMask: GLbitfield, numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, stride: GLsizei, metrics: *mut GLfloat);

pub type glGetPathParameterfvNV_t = unsafe extern "system" fn(path: GLuint, pname: GLenum, value: *mut [GLfloat; 4]);

pub type glGetPathParameterivNV_t = unsafe extern "system" fn(path: GLuint, pname: GLenum, value: *mut [GLint; 4]);

pub type glGetPathSpacingNV_t = unsafe extern "system" fn(pathListMode: GLenum, numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, advanceScale: GLfloat, kerningScale: GLfloat, transformType: GLenum, returnedSpacing: *mut GLfloat);

pub type glGetPathTexGenfvNV_t = unsafe extern "system" fn(texCoordSet: GLenum, pname: GLenum, value: *mut GLfloat);

pub type glGetPathTexGenivNV_t = unsafe extern "system" fn(texCoordSet: GLenum, pname: GLenum, value: *mut GLint);

pub type glGetPerfCounterInfoINTEL_t = unsafe extern "system" fn(queryId: GLuint, counterId: GLuint, counterNameLength: GLuint, counterName: *mut GLchar, counterDescLength: GLuint, counterDesc: *mut GLchar, counterOffset: *mut GLuint, counterDataSize: *mut GLuint, counterTypeEnum: *mut GLuint, counterDataTypeEnum: *mut GLuint, rawCounterMaxValue: *mut GLuint64);

pub type glGetPerfMonitorCounterDataAMD_t = unsafe extern "system" fn(monitor: GLuint, pname: GLenum, dataSize: GLsizei, data: *mut GLuint, bytesWritten: *mut GLint);

pub type glGetPerfMonitorCounterInfoAMD_t = unsafe extern "system" fn(group: GLuint, counter: GLuint, pname: GLenum, data: *mut void);

pub type glGetPerfMonitorCounterStringAMD_t = unsafe extern "system" fn(group: GLuint, counter: GLuint, bufSize: GLsizei, length: *mut GLsizei, counterString: *mut GLchar);

pub type glGetPerfMonitorCountersAMD_t = unsafe extern "system" fn(group: GLuint, numCounters: *mut GLint, maxActiveCounters: *mut GLint, counterSize: GLsizei, counters: *mut GLuint);

pub type glGetPerfMonitorGroupStringAMD_t = unsafe extern "system" fn(group: GLuint, bufSize: GLsizei, length: *mut GLsizei, groupString: *mut GLchar);

pub type glGetPerfMonitorGroupsAMD_t = unsafe extern "system" fn(numGroups: *mut GLint, groupsSize: GLsizei, groups: *mut GLuint);

pub type glGetPerfQueryDataINTEL_t = unsafe extern "system" fn(queryHandle: GLuint, flags: GLuint, dataSize: GLsizei, data: *mut void, bytesWritten: *mut GLuint);

pub type glGetPerfQueryIdByNameINTEL_t = unsafe extern "system" fn(queryName: *mut GLchar, queryId: *mut GLuint);

pub type glGetPerfQueryInfoINTEL_t = unsafe extern "system" fn(queryId: GLuint, queryNameLength: GLuint, queryName: *mut GLchar, dataSize: *mut GLuint, noCounters: *mut GLuint, noInstances: *mut GLuint, capsMask: *mut GLuint);

pub type glGetPixelMapfv_t = unsafe extern "system" fn(map: GLenum, values: *mut GLfloat);

pub type glGetPixelMapuiv_t = unsafe extern "system" fn(map: GLenum, values: *mut GLuint);

pub type glGetPixelMapusv_t = unsafe extern "system" fn(map: GLenum, values: *mut GLushort);

pub type glGetPixelMapxv_t = unsafe extern "system" fn(map: GLenum, size: GLint, values: *mut GLfixed);

pub type glGetPixelTexGenParameterfvSGIS_t = unsafe extern "system" fn(pname: GLenum, params: *mut GLfloat);

pub type glGetPixelTexGenParameterivSGIS_t = unsafe extern "system" fn(pname: GLenum, params: *mut GLint);

pub type glGetPixelTransformParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetPixelTransformParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetPointerIndexedvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut *mut void);

pub type glGetPointeri_vEXT_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, params: *mut *mut void);

pub type glGetPointerv_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut void);

pub type glGetPointervEXT_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut void);

pub type glGetPointervKHR_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut void);

pub type glGetPolygonStipple_t = unsafe extern "system" fn(mask: *mut GLubyte);

pub type glGetProgramBinary_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void);

pub type glGetProgramBinaryOES_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void);

pub type glGetProgramEnvParameterIivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *mut [GLint; 4]);

pub type glGetProgramEnvParameterIuivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *mut [GLuint; 4]);

pub type glGetProgramEnvParameterdvARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *mut [GLdouble; 4]);

pub type glGetProgramEnvParameterfvARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *mut [GLfloat; 4]);

pub type glGetProgramInfoLog_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub type glGetProgramInterfaceiv_t = unsafe extern "system" fn(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetProgramLocalParameterIivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *mut [GLint; 4]);

pub type glGetProgramLocalParameterIuivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *mut [GLuint; 4]);

pub type glGetProgramLocalParameterdvARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *mut [GLdouble; 4]);

pub type glGetProgramLocalParameterfvARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *mut [GLfloat; 4]);

pub type glGetProgramNamedParameterdvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, params: *mut [GLdouble; 4]);

pub type glGetProgramNamedParameterfvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, params: *mut [GLfloat; 4]);

pub type glGetProgramParameterdvNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut [GLdouble; 4]);

pub type glGetProgramParameterfvNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut [GLfloat; 4]);

pub type glGetProgramPipelineInfoLog_t = unsafe extern "system" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub type glGetProgramPipelineInfoLogEXT_t = unsafe extern "system" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub type glGetProgramPipelineiv_t = unsafe extern "system" fn(pipeline: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetProgramPipelineivEXT_t = unsafe extern "system" fn(pipeline: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetProgramResourceIndex_t = unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint;

pub type glGetProgramResourceLocation_t = unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;

pub type glGetProgramResourceLocationIndex_t = unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;

pub type glGetProgramResourceLocationIndexEXT_t = unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;

pub type glGetProgramResourceName_t = unsafe extern "system" fn(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

pub type glGetProgramResourcefvNV_t = unsafe extern "system" fn(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLfloat);

pub type glGetProgramResourceiv_t = unsafe extern "system" fn(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint);

pub type glGetProgramStageiv_t = unsafe extern "system" fn(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint);

pub type glGetProgramStringARB_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, string: *mut void);

pub type glGetProgramStringNV_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, program: *mut GLubyte);

pub type glGetProgramSubroutineParameteruivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, param: *mut GLuint);

pub type glGetProgramiv_t = unsafe extern "system" fn(program: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetProgramivARB_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetProgramivNV_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut [GLint; 4]);

pub type glGetQueryBufferObjecti64v_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);

pub type glGetQueryBufferObjectiv_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);

pub type glGetQueryBufferObjectui64v_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);

pub type glGetQueryBufferObjectuiv_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);

pub type glGetQueryIndexediv_t = unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetQueryObjecti64v_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint64);

pub type glGetQueryObjecti64vEXT_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint64);

pub type glGetQueryObjectiv_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetQueryObjectivARB_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetQueryObjectivEXT_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetQueryObjectui64v_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint64);

pub type glGetQueryObjectui64vEXT_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint64);

pub type glGetQueryObjectuiv_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetQueryObjectuivARB_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetQueryObjectuivEXT_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetQueryiv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetQueryivARB_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetQueryivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetRenderbufferParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetRenderbufferParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetRenderbufferParameterivOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetSamplerParameterIivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetSamplerParameterIivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetSamplerParameterIuivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetSamplerParameterIuivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetSemaphoreParameterivNV_t = unsafe extern "system" fn(semaphore: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetSemaphoreParameterui64vEXT_t = unsafe extern "system" fn(semaphore: GLuint, pname: GLenum, params: *mut GLuint64);

pub type glGetSeparableFilter_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, row: *mut void, column: *mut void, span: *mut void);

pub type glGetSeparableFilterEXT_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, row: *mut void, column: *mut void, span: *mut void);

pub type glGetShaderInfoLog_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub type glGetShaderPrecisionFormat_t = unsafe extern "system" fn(shadertype: GLenum, precisiontype: GLenum, range: *mut [GLint; 2], precision: *mut GLint);

pub type glGetShaderSource_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);

pub type glGetShaderSourceARB_t = unsafe extern "system" fn(obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, source: *mut GLcharARB);

pub type glGetShaderiv_t = unsafe extern "system" fn(shader: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetShadingRateImagePaletteNV_t = unsafe extern "system" fn(viewport: GLuint, entry: GLuint, rate: *mut GLenum);

pub type glGetShadingRateSampleLocationivNV_t = unsafe extern "system" fn(rate: GLenum, samples: GLuint, index: GLuint, location: *mut [GLint; 3]);

pub type glGetSharpenTexFuncSGIS_t = unsafe extern "system" fn(target: GLenum, points: *mut GLfloat);

pub type glGetStageIndexNV_t = unsafe extern "system" fn(shadertype: GLenum) -> GLushort;

pub type glGetString_t = unsafe extern "system" fn(name: GLenum) -> GLubyte;

pub type glGetStringi_t = unsafe extern "system" fn(name: GLenum, index: GLuint) -> GLubyte;

pub type glGetSubroutineIndex_t = unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint;

pub type glGetSubroutineUniformLocation_t = unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint;

pub type glGetSynciv_t = unsafe extern "system" fn(sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

pub type glGetSyncivAPPLE_t = unsafe extern "system" fn(sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

pub type glGetTexBumpParameterfvATI_t = unsafe extern "system" fn(pname: GLenum, param: *mut GLfloat);

pub type glGetTexBumpParameterivATI_t = unsafe extern "system" fn(pname: GLenum, param: *mut GLint);

pub type glGetTexEnvfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetTexEnviv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTexEnvxv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetTexEnvxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetTexFilterFuncSGIS_t = unsafe extern "system" fn(target: GLenum, filter: GLenum, weights: *mut GLfloat);

pub type glGetTexGendv_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *mut GLdouble);

pub type glGetTexGenfv_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetTexGenfvOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetTexGeniv_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTexGenivOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTexGenxvOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetTexImage_t = unsafe extern "system" fn(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut void);

pub type glGetTexLevelParameterfv_t = unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);

pub type glGetTexLevelParameteriv_t = unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);

pub type glGetTexLevelParameterxvOES_t = unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfixed);

pub type glGetTexParameterIiv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTexParameterIivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTexParameterIivOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTexParameterIuiv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint);

pub type glGetTexParameterIuivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint);

pub type glGetTexParameterIuivOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint);

pub type glGetTexParameterPointervAPPLE_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut void);

pub type glGetTexParameterfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetTexParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTexParameterxv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetTexParameterxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);

pub type glGetTextureHandleARB_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

pub type glGetTextureHandleIMG_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

pub type glGetTextureHandleNV_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

pub type glGetTextureImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut void);

pub type glGetTextureImageEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut void);

pub type glGetTextureLevelParameterfv_t = unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat);

pub type glGetTextureLevelParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);

pub type glGetTextureLevelParameteriv_t = unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint);

pub type glGetTextureLevelParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);

pub type glGetTextureParameterIiv_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetTextureParameterIivEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTextureParameterIuiv_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetTextureParameterIuivEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLuint);

pub type glGetTextureParameterfv_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetTextureParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLfloat);

pub type glGetTextureParameteriv_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetTextureParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);

pub type glGetTextureSamplerHandleARB_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

pub type glGetTextureSamplerHandleIMG_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

pub type glGetTextureSamplerHandleNV_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

pub type glGetTextureSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut void);

pub type glGetTrackMatrixivNV_t = unsafe extern "system" fn(target: GLenum, address: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetTransformFeedbackVarying_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);

pub type glGetTransformFeedbackVaryingEXT_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);

pub type glGetTransformFeedbackVaryingNV_t = unsafe extern "system" fn(program: GLuint, index: GLuint, location: *mut GLint);

pub type glGetTransformFeedbacki64_v_t = unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64);

pub type glGetTransformFeedbacki_v_t = unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint);

pub type glGetTransformFeedbackiv_t = unsafe extern "system" fn(xfb: GLuint, pname: GLenum, param: *mut GLint);

pub type glGetTranslatedShaderSourceANGLE_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);

pub type glGetUniformBlockIndex_t = unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;

pub type glGetUniformBufferSizeEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint) -> GLint;

pub type glGetUniformIndices_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);

pub type glGetUniformLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub type glGetUniformLocationARB_t = unsafe extern "system" fn(programObj: GLhandleARB, name: *const GLcharARB) -> GLint;

pub type glGetUniformOffsetEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint) -> GLintptr;

pub type glGetUniformSubroutineuiv_t = unsafe extern "system" fn(shadertype: GLenum, location: GLint, params: *mut GLuint);

pub type glGetUniformdv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLdouble);

pub type glGetUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);

pub type glGetUniformfvARB_t = unsafe extern "system" fn(programObj: GLhandleARB, location: GLint, params: *mut GLfloat);

pub type glGetUniformi64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint64);

pub type glGetUniformi64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint64EXT);

pub type glGetUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);

pub type glGetUniformivARB_t = unsafe extern "system" fn(programObj: GLhandleARB, location: GLint, params: *mut GLint);

pub type glGetUniformui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint64);

pub type glGetUniformui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint64EXT);

pub type glGetUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);

pub type glGetUniformuivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);

pub type glGetUnsignedBytei_vEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLubyte);

pub type glGetUnsignedBytevEXT_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLubyte);

pub type glGetVariantArrayObjectfvATI_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetVariantArrayObjectivATI_t = unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetVariantBooleanvEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLboolean);

pub type glGetVariantFloatvEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLfloat);

pub type glGetVariantIntegervEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut GLint);

pub type glGetVariantPointervEXT_t = unsafe extern "system" fn(id: GLuint, value: GLenum, data: *mut *mut void);

pub type glGetVaryingLocationNV_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub type glGetVertexArrayIndexed64iv_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64);

pub type glGetVertexArrayIndexediv_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);

pub type glGetVertexArrayIntegeri_vEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);

pub type glGetVertexArrayIntegervEXT_t = unsafe extern "system" fn(vaobj: GLuint, pname: GLenum, param: *mut GLint);

pub type glGetVertexArrayPointeri_vEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut *mut void);

pub type glGetVertexArrayPointervEXT_t = unsafe extern "system" fn(vaobj: GLuint, pname: GLenum, param: *mut *mut void);

pub type glGetVertexArrayiv_t = unsafe extern "system" fn(vaobj: GLuint, pname: GLenum, param: *mut GLint);

pub type glGetVertexAttribArrayObjectfvATI_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetVertexAttribArrayObjectivATI_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetVertexAttribIiv_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetVertexAttribIivEXT_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetVertexAttribIuiv_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetVertexAttribIuivEXT_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetVertexAttribLdv_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);

pub type glGetVertexAttribLdvEXT_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);

pub type glGetVertexAttribLi64vNV_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint64EXT);

pub type glGetVertexAttribLui64vARB_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint64EXT);

pub type glGetVertexAttribLui64vNV_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint64EXT);

pub type glGetVertexAttribPointerv_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, pointer: *mut *mut void);

pub type glGetVertexAttribPointervARB_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, pointer: *mut *mut void);

pub type glGetVertexAttribPointervNV_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, pointer: *mut *mut void);

pub type glGetVertexAttribdv_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut [GLdouble; 4]);

pub type glGetVertexAttribdvARB_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut [GLdouble; 4]);

pub type glGetVertexAttribdvNV_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);

pub type glGetVertexAttribfv_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut [GLfloat; 4]);

pub type glGetVertexAttribfvARB_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut [GLfloat; 4]);

pub type glGetVertexAttribfvNV_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetVertexAttribiv_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut [GLint; 4]);

pub type glGetVertexAttribivARB_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut [GLint; 4]);

pub type glGetVertexAttribivNV_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetVideoCaptureStreamdvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLdouble);

pub type glGetVideoCaptureStreamfvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLfloat);

pub type glGetVideoCaptureStreamivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetVideoCaptureivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetVideoi64vNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLint64EXT);

pub type glGetVideoivNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLint);

pub type glGetVideoui64vNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLuint64EXT);

pub type glGetVideouivNV_t = unsafe extern "system" fn(video_slot: GLuint, pname: GLenum, params: *mut GLuint);

pub type glGetVkProcAddrNV_t = unsafe extern "system" fn(name: *const GLchar) -> GLVULKANPROCNV;

pub type glGetnColorTable_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut void);

pub type glGetnColorTableARB_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut void);

pub type glGetnCompressedTexImage_t = unsafe extern "system" fn(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut void);

pub type glGetnCompressedTexImageARB_t = unsafe extern "system" fn(target: GLenum, lod: GLint, bufSize: GLsizei, img: *mut void);

pub type glGetnConvolutionFilter_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut void);

pub type glGetnConvolutionFilterARB_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut void);

pub type glGetnHistogram_t = unsafe extern "system" fn(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut void);

pub type glGetnHistogramARB_t = unsafe extern "system" fn(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut void);

pub type glGetnMapdv_t = unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble);

pub type glGetnMapdvARB_t = unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble);

pub type glGetnMapfv_t = unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat);

pub type glGetnMapfvARB_t = unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat);

pub type glGetnMapiv_t = unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint);

pub type glGetnMapivARB_t = unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint);

pub type glGetnMinmax_t = unsafe extern "system" fn(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut void);

pub type glGetnMinmaxARB_t = unsafe extern "system" fn(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut void);

pub type glGetnPixelMapfv_t = unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLfloat);

pub type glGetnPixelMapfvARB_t = unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLfloat);

pub type glGetnPixelMapuiv_t = unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLuint);

pub type glGetnPixelMapuivARB_t = unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLuint);

pub type glGetnPixelMapusv_t = unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLushort);

pub type glGetnPixelMapusvARB_t = unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLushort);

pub type glGetnPolygonStipple_t = unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);

pub type glGetnPolygonStippleARB_t = unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);

pub type glGetnSeparableFilter_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut void, columnBufSize: GLsizei, column: *mut void, span: *mut void);

pub type glGetnSeparableFilterARB_t = unsafe extern "system" fn(target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut void, columnBufSize: GLsizei, column: *mut void, span: *mut void);

pub type glGetnTexImage_t = unsafe extern "system" fn(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut void);

pub type glGetnTexImageARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, img: *mut void);

pub type glGetnUniformdv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);

pub type glGetnUniformdvARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);

pub type glGetnUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

pub type glGetnUniformfvARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

pub type glGetnUniformfvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

pub type glGetnUniformfvKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

pub type glGetnUniformi64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint64);

pub type glGetnUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

pub type glGetnUniformivARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

pub type glGetnUniformivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

pub type glGetnUniformivKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

pub type glGetnUniformui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint64);

pub type glGetnUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

pub type glGetnUniformuivARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

pub type glGetnUniformuivKHR_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

pub type glGlobalAlphaFactorbSUN_t = unsafe extern "system" fn(factor: GLbyte);

pub type glGlobalAlphaFactordSUN_t = unsafe extern "system" fn(factor: GLdouble);

pub type glGlobalAlphaFactorfSUN_t = unsafe extern "system" fn(factor: GLfloat);

pub type glGlobalAlphaFactoriSUN_t = unsafe extern "system" fn(factor: GLint);

pub type glGlobalAlphaFactorsSUN_t = unsafe extern "system" fn(factor: GLshort);

pub type glGlobalAlphaFactorubSUN_t = unsafe extern "system" fn(factor: GLubyte);

pub type glGlobalAlphaFactoruiSUN_t = unsafe extern "system" fn(factor: GLuint);

pub type glGlobalAlphaFactorusSUN_t = unsafe extern "system" fn(factor: GLushort);

pub type glHint_t = unsafe extern "system" fn(target: GLenum, mode: GLenum);

pub type glHintPGI_t = unsafe extern "system" fn(target: GLenum, mode: GLint);

pub type glHistogram_t = unsafe extern "system" fn(target: GLenum, width: GLsizei, internalformat: GLenum, sink: GLboolean);

pub type glHistogramEXT_t = unsafe extern "system" fn(target: GLenum, width: GLsizei, internalformat: GLenum, sink: GLboolean);

pub type glIglooInterfaceSGIX_t = unsafe extern "system" fn(pname: GLenum, params: *const void);

pub type glImageTransformParameterfHP_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat);

pub type glImageTransformParameterfvHP_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glImageTransformParameteriHP_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

pub type glImageTransformParameterivHP_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glImportMemoryFdEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: GLenum, fd: GLint);

pub type glImportMemoryWin32HandleEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: GLenum, handle: *mut void);

pub type glImportMemoryWin32NameEXT_t = unsafe extern "system" fn(memory: GLuint, size: GLuint64, handleType: GLenum, name: *const void);

pub type glImportSemaphoreFdEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: GLenum, fd: GLint);

pub type glImportSemaphoreWin32HandleEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: GLenum, handle: *mut void);

pub type glImportSemaphoreWin32NameEXT_t = unsafe extern "system" fn(semaphore: GLuint, handleType: GLenum, name: *const void);

pub type glImportSyncEXT_t = unsafe extern "system" fn(external_sync_type: GLenum, external_sync: GLintptr, flags: GLbitfield) -> GLsync;

pub type glIndexFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

pub type glIndexFuncEXT_t = unsafe extern "system" fn(func: GLenum, ref_: GLclampf);

pub type glIndexMask_t = unsafe extern "system" fn(mask: GLuint);

pub type glIndexMaterialEXT_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);

pub type glIndexPointer_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glIndexPointerEXT_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const void);

pub type glIndexPointerListIBM_t = unsafe extern "system" fn(type_: GLenum, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

pub type glIndexd_t = unsafe extern "system" fn(c: GLdouble);

pub type glIndexdv_t = unsafe extern "system" fn(c: *const GLdouble);

pub type glIndexf_t = unsafe extern "system" fn(c: GLfloat);

pub type glIndexfv_t = unsafe extern "system" fn(c: *const GLfloat);

pub type glIndexi_t = unsafe extern "system" fn(c: GLint);

pub type glIndexiv_t = unsafe extern "system" fn(c: *const GLint);

pub type glIndexs_t = unsafe extern "system" fn(c: GLshort);

pub type glIndexsv_t = unsafe extern "system" fn(c: *const GLshort);

pub type glIndexub_t = unsafe extern "system" fn(c: GLubyte);

pub type glIndexubv_t = unsafe extern "system" fn(c: *const GLubyte);

pub type glIndexxOES_t = unsafe extern "system" fn(component: GLfixed);

pub type glIndexxvOES_t = unsafe extern "system" fn(component: *const GLfixed);

pub type glInitNames_t = unsafe extern "system" fn();

pub type glInsertComponentEXT_t = unsafe extern "system" fn(res: GLuint, src: GLuint, num: GLuint);

pub type glInsertEventMarkerEXT_t = unsafe extern "system" fn(length: GLsizei, marker: *const GLchar);

pub type glInstrumentsBufferSGIX_t = unsafe extern "system" fn(size: GLsizei, buffer: *mut GLint);

pub type glInterleavedArrays_t = unsafe extern "system" fn(format: GLenum, stride: GLsizei, pointer: *const void);

pub type glInterpolatePathsNV_t = unsafe extern "system" fn(resultPath: GLuint, pathA: GLuint, pathB: GLuint, weight: GLfloat);

pub type glInvalidateBufferData_t = unsafe extern "system" fn(buffer: GLuint);

pub type glInvalidateBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

pub type glInvalidateFramebuffer_t = unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);

pub type glInvalidateNamedFramebufferData_t = unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum);

pub type glInvalidateNamedFramebufferSubData_t = unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glInvalidateSubFramebuffer_t = unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glInvalidateTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint);

pub type glInvalidateTexSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

pub type glIsAsyncMarkerSGIX_t = unsafe extern "system" fn(marker: GLuint) -> GLboolean;

pub type glIsBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub type glIsBufferARB_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub type glIsBufferResidentNV_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;

pub type glIsCommandListNV_t = unsafe extern "system" fn(list: GLuint) -> GLboolean;

pub type glIsEnabled_t = unsafe extern "system" fn(cap: GLenum) -> GLboolean;

pub type glIsEnabledIndexedEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean;

pub type glIsEnabledi_t = unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean;

pub type glIsEnablediEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean;

pub type glIsEnablediNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean;

pub type glIsEnablediOES_t = unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean;

pub type glIsFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

pub type glIsFenceNV_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

pub type glIsFramebuffer_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

pub type glIsFramebufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

pub type glIsFramebufferOES_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

pub type glIsImageHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

pub type glIsImageHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

pub type glIsList_t = unsafe extern "system" fn(list: GLuint) -> GLboolean;

pub type glIsMemoryObjectEXT_t = unsafe extern "system" fn(memoryObject: GLuint) -> GLboolean;

pub type glIsNameAMD_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint) -> GLboolean;

pub type glIsNamedBufferResidentNV_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub type glIsNamedStringARB_t = unsafe extern "system" fn(namelen: GLint, name: *const GLchar) -> GLboolean;

pub type glIsObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub type glIsOcclusionQueryNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub type glIsPathNV_t = unsafe extern "system" fn(path: GLuint) -> GLboolean;

pub type glIsPointInFillPathNV_t = unsafe extern "system" fn(path: GLuint, mask: GLuint, x: GLfloat, y: GLfloat) -> GLboolean;

pub type glIsPointInStrokePathNV_t = unsafe extern "system" fn(path: GLuint, x: GLfloat, y: GLfloat) -> GLboolean;

pub type glIsProgram_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

pub type glIsProgramARB_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

pub type glIsProgramNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub type glIsProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;

pub type glIsProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;

pub type glIsQuery_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub type glIsQueryARB_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub type glIsQueryEXT_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub type glIsRenderbuffer_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

pub type glIsRenderbufferEXT_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

pub type glIsRenderbufferOES_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

pub type glIsSampler_t = unsafe extern "system" fn(sampler: GLuint) -> GLboolean;

pub type glIsSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint) -> GLboolean;

pub type glIsShader_t = unsafe extern "system" fn(shader: GLuint) -> GLboolean;

pub type glIsStateNV_t = unsafe extern "system" fn(state: GLuint) -> GLboolean;

pub type glIsSync_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;

pub type glIsSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;

pub type glIsTexture_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

pub type glIsTextureEXT_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

pub type glIsTextureHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

pub type glIsTextureHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

pub type glIsTransformFeedback_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub type glIsTransformFeedbackNV_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub type glIsVariantEnabledEXT_t = unsafe extern "system" fn(id: GLuint, cap: GLenum) -> GLboolean;

pub type glIsVertexArray_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

pub type glIsVertexArrayAPPLE_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

pub type glIsVertexArrayOES_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

pub type glIsVertexAttribEnabledAPPLE_t = unsafe extern "system" fn(index: GLuint, pname: GLenum) -> GLboolean;

pub type glLGPUCopyImageSubDataNVX_t = unsafe extern "system" fn(sourceGpu: GLuint, destinationGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srxY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

pub type glLGPUInterlockNVX_t = unsafe extern "system" fn();

pub type glLGPUNamedBufferSubDataNVX_t = unsafe extern "system" fn(gpuMask: GLbitfield, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

pub type glLabelObjectEXT_t = unsafe extern "system" fn(type_: GLenum, object: GLuint, length: GLsizei, label: *const GLchar);

pub type glLightEnviSGIX_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glLightModelf_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glLightModelfv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glLightModeli_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glLightModeliv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);

pub type glLightModelx_t = unsafe extern "system" fn(pname: GLenum, param: GLfixed);

pub type glLightModelxOES_t = unsafe extern "system" fn(pname: GLenum, param: GLfixed);

pub type glLightModelxv_t = unsafe extern "system" fn(pname: GLenum, param: *const GLfixed);

pub type glLightModelxvOES_t = unsafe extern "system" fn(pname: GLenum, param: *const GLfixed);

pub type glLightf_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, param: GLfloat);

pub type glLightfv_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *const GLfloat);

pub type glLighti_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, param: GLint);

pub type glLightiv_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *const GLint);

pub type glLightx_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, param: GLfixed);

pub type glLightxOES_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, param: GLfixed);

pub type glLightxv_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *const GLfixed);

pub type glLightxvOES_t = unsafe extern "system" fn(light: GLenum, pname: GLenum, params: *const GLfixed);

pub type glLineStipple_t = unsafe extern "system" fn(factor: GLint, pattern: GLushort);

pub type glLineWidth_t = unsafe extern "system" fn(width: GLfloat);

pub type glLineWidthx_t = unsafe extern "system" fn(width: GLfixed);

pub type glLineWidthxOES_t = unsafe extern "system" fn(width: GLfixed);

pub type glLinkProgram_t = unsafe extern "system" fn(program: GLuint);

pub type glLinkProgramARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

pub type glListBase_t = unsafe extern "system" fn(base: GLuint);

pub type glListDrawCommandsStatesClientNV_t = unsafe extern "system" fn(list: GLuint, segment: GLuint, indirects: *const *mut void, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);

pub type glListParameterfSGIX_t = unsafe extern "system" fn(list: GLuint, pname: GLenum, param: GLfloat);

pub type glListParameterfvSGIX_t = unsafe extern "system" fn(list: GLuint, pname: GLenum, params: *const GLfloat);

pub type glListParameteriSGIX_t = unsafe extern "system" fn(list: GLuint, pname: GLenum, param: GLint);

pub type glListParameterivSGIX_t = unsafe extern "system" fn(list: GLuint, pname: GLenum, params: *const GLint);

pub type glLoadIdentity_t = unsafe extern "system" fn();

pub type glLoadIdentityDeformationMapSGIX_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glLoadMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

pub type glLoadMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

pub type glLoadMatrixx_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

pub type glLoadMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

pub type glLoadName_t = unsafe extern "system" fn(name: GLuint);

pub type glLoadPaletteFromModelViewMatrixOES_t = unsafe extern "system" fn();

pub type glLoadProgramNV_t = unsafe extern "system" fn(target: GLenum, id: GLuint, len: GLsizei, program: *const GLubyte);

pub type glLoadTransposeMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

pub type glLoadTransposeMatrixdARB_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

pub type glLoadTransposeMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

pub type glLoadTransposeMatrixfARB_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

pub type glLoadTransposeMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

pub type glLockArraysEXT_t = unsafe extern "system" fn(first: GLint, count: GLsizei);

pub type glLogicOp_t = unsafe extern "system" fn(opcode: GLenum);

pub type glMakeBufferNonResidentNV_t = unsafe extern "system" fn(target: GLenum);

pub type glMakeBufferResidentNV_t = unsafe extern "system" fn(target: GLenum, access: GLenum);

pub type glMakeImageHandleNonResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

pub type glMakeImageHandleNonResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

pub type glMakeImageHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64, access: GLenum);

pub type glMakeImageHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64, access: GLenum);

pub type glMakeNamedBufferNonResidentNV_t = unsafe extern "system" fn(buffer: GLuint);

pub type glMakeNamedBufferResidentNV_t = unsafe extern "system" fn(buffer: GLuint, access: GLenum);

pub type glMakeTextureHandleNonResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

pub type glMakeTextureHandleNonResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

pub type glMakeTextureHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

pub type glMakeTextureHandleResidentNV_t = unsafe extern "system" fn(handle: GLuint64);

pub type glMap1d_t = unsafe extern "system" fn(target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble);

pub type glMap1f_t = unsafe extern "system" fn(target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat);

pub type glMap1xOES_t = unsafe extern "system" fn(target: GLenum, u1: GLfixed, u2: GLfixed, stride: GLint, order: GLint, points: GLfixed);

pub type glMap2d_t = unsafe extern "system" fn(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble);

pub type glMap2f_t = unsafe extern "system" fn(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat);

pub type glMap2xOES_t = unsafe extern "system" fn(target: GLenum, u1: GLfixed, u2: GLfixed, ustride: GLint, uorder: GLint, v1: GLfixed, v2: GLfixed, vstride: GLint, vorder: GLint, points: GLfixed);

pub type glMapBuffer_t = unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut void;

pub type glMapBufferARB_t = unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut void;

pub type glMapBufferOES_t = unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut void;

pub type glMapBufferRange_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

pub type glMapBufferRangeEXT_t = unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

pub type glMapControlPointsNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, type_: GLenum, ustride: GLsizei, vstride: GLsizei, uorder: GLint, vorder: GLint, packed: GLboolean, points: *const void);

pub type glMapGrid1d_t = unsafe extern "system" fn(un: GLint, u1: GLdouble, u2: GLdouble);

pub type glMapGrid1f_t = unsafe extern "system" fn(un: GLint, u1: GLfloat, u2: GLfloat);

pub type glMapGrid1xOES_t = unsafe extern "system" fn(n: GLint, u1: GLfixed, u2: GLfixed);

pub type glMapGrid2d_t = unsafe extern "system" fn(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble);

pub type glMapGrid2f_t = unsafe extern "system" fn(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat);

pub type glMapGrid2xOES_t = unsafe extern "system" fn(n: GLint, u1: GLfixed, u2: GLfixed, v1: GLfixed, v2: GLfixed);

pub type glMapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint, access: GLenum) -> *mut void;

pub type glMapNamedBufferEXT_t = unsafe extern "system" fn(buffer: GLuint, access: GLenum) -> *mut void;

pub type glMapNamedBufferRange_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

pub type glMapNamedBufferRangeEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

pub type glMapObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint) -> *mut void;

pub type glMapParameterfvNV_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glMapParameterivNV_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glMapTexture2DINTEL_t = unsafe extern "system" fn(texture: GLuint, level: GLint, access: GLbitfield, stride: *mut GLint, layout: *mut GLenum) -> *mut void;

pub type glMapVertexAttrib1dAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble);

pub type glMapVertexAttrib1fAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat);

pub type glMapVertexAttrib2dAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble);

pub type glMapVertexAttrib2fAPPLE_t = unsafe extern "system" fn(index: GLuint, size: GLuint, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat);

pub type glMaterialf_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: GLfloat);

pub type glMaterialfv_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *const GLfloat);

pub type glMateriali_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: GLint);

pub type glMaterialiv_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, params: *const GLint);

pub type glMaterialx_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: GLfixed);

pub type glMaterialxOES_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: GLfixed);

pub type glMaterialxv_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: *const GLfixed);

pub type glMaterialxvOES_t = unsafe extern "system" fn(face: GLenum, pname: GLenum, param: *const GLfixed);

pub type glMatrixFrustumEXT_t = unsafe extern "system" fn(mode: GLenum, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

pub type glMatrixIndexPointerARB_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glMatrixIndexPointerOES_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glMatrixIndexubvARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLubyte);

pub type glMatrixIndexuivARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLuint);

pub type glMatrixIndexusvARB_t = unsafe extern "system" fn(size: GLint, indices: *const GLushort);

pub type glMatrixLoad3x2fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

pub type glMatrixLoad3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

pub type glMatrixLoadIdentityEXT_t = unsafe extern "system" fn(mode: GLenum);

pub type glMatrixLoadTranspose3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

pub type glMatrixLoadTransposedEXT_t = unsafe extern "system" fn(mode: GLenum, m: *const [GLdouble; 16]);

pub type glMatrixLoadTransposefEXT_t = unsafe extern "system" fn(mode: GLenum, m: *const [GLfloat; 16]);

pub type glMatrixLoaddEXT_t = unsafe extern "system" fn(mode: GLenum, m: *const [GLdouble; 16]);

pub type glMatrixLoadfEXT_t = unsafe extern "system" fn(mode: GLenum, m: *const [GLfloat; 16]);

pub type glMatrixMode_t = unsafe extern "system" fn(mode: GLenum);

pub type glMatrixMult3x2fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

pub type glMatrixMult3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

pub type glMatrixMultTranspose3x3fNV_t = unsafe extern "system" fn(matrixMode: GLenum, m: *const GLfloat);

pub type glMatrixMultTransposedEXT_t = unsafe extern "system" fn(mode: GLenum, m: *const [GLdouble; 16]);

pub type glMatrixMultTransposefEXT_t = unsafe extern "system" fn(mode: GLenum, m: *const [GLfloat; 16]);

pub type glMatrixMultdEXT_t = unsafe extern "system" fn(mode: GLenum, m: *const [GLdouble; 16]);

pub type glMatrixMultfEXT_t = unsafe extern "system" fn(mode: GLenum, m: *const [GLfloat; 16]);

pub type glMatrixOrthoEXT_t = unsafe extern "system" fn(mode: GLenum, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

pub type glMatrixPopEXT_t = unsafe extern "system" fn(mode: GLenum);

pub type glMatrixPushEXT_t = unsafe extern "system" fn(mode: GLenum);

pub type glMatrixRotatedEXT_t = unsafe extern "system" fn(mode: GLenum, angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glMatrixRotatefEXT_t = unsafe extern "system" fn(mode: GLenum, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glMatrixScaledEXT_t = unsafe extern "system" fn(mode: GLenum, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glMatrixScalefEXT_t = unsafe extern "system" fn(mode: GLenum, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glMatrixTranslatedEXT_t = unsafe extern "system" fn(mode: GLenum, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glMatrixTranslatefEXT_t = unsafe extern "system" fn(mode: GLenum, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glMaxShaderCompilerThreadsARB_t = unsafe extern "system" fn(count: GLuint);

pub type glMaxShaderCompilerThreadsKHR_t = unsafe extern "system" fn(count: GLuint);

pub type glMemoryBarrier_t = unsafe extern "system" fn(barriers: GLbitfield);

pub type glMemoryBarrierByRegion_t = unsafe extern "system" fn(barriers: GLbitfield);

pub type glMemoryBarrierEXT_t = unsafe extern "system" fn(barriers: GLbitfield);

pub type glMemoryObjectParameterivEXT_t = unsafe extern "system" fn(memoryObject: GLuint, pname: GLenum, params: *const GLint);

pub type glMinSampleShading_t = unsafe extern "system" fn(value: GLfloat);

pub type glMinSampleShadingARB_t = unsafe extern "system" fn(value: GLfloat);

pub type glMinSampleShadingOES_t = unsafe extern "system" fn(value: GLfloat);

pub type glMinmax_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, sink: GLboolean);

pub type glMinmaxEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, sink: GLboolean);

pub type glMultMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

pub type glMultMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

pub type glMultMatrixx_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

pub type glMultMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

pub type glMultTransposeMatrixd_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

pub type glMultTransposeMatrixdARB_t = unsafe extern "system" fn(m: *const [GLdouble; 16]);

pub type glMultTransposeMatrixf_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

pub type glMultTransposeMatrixfARB_t = unsafe extern "system" fn(m: *const [GLfloat; 16]);

pub type glMultTransposeMatrixxOES_t = unsafe extern "system" fn(m: *const [GLfixed; 16]);

pub type glMultiDrawArrays_t = unsafe extern "system" fn(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);

pub type glMultiDrawArraysEXT_t = unsafe extern "system" fn(mode: GLenum, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

pub type glMultiDrawArraysIndirect_t = unsafe extern "system" fn(mode: GLenum, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawArraysIndirectAMD_t = unsafe extern "system" fn(mode: GLenum, indirect: *const void, primcount: GLsizei, stride: GLsizei);

pub type glMultiDrawArraysIndirectBindlessCountNV_t = unsafe extern "system" fn(mode: GLenum, indirect: *const void, drawCount: GLsizei, maxDrawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

pub type glMultiDrawArraysIndirectBindlessNV_t = unsafe extern "system" fn(mode: GLenum, indirect: *const void, drawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

pub type glMultiDrawArraysIndirectCount_t = unsafe extern "system" fn(mode: GLenum, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawArraysIndirectCountARB_t = unsafe extern "system" fn(mode: GLenum, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawArraysIndirectEXT_t = unsafe extern "system" fn(mode: GLenum, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawElementArrayAPPLE_t = unsafe extern "system" fn(mode: GLenum, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

pub type glMultiDrawElements_t = unsafe extern "system" fn(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const void, drawcount: GLsizei);

pub type glMultiDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);

pub type glMultiDrawElementsBaseVertexEXT_t = unsafe extern "system" fn(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);

pub type glMultiDrawElementsEXT_t = unsafe extern "system" fn(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const void, primcount: GLsizei);

pub type glMultiDrawElementsIndirect_t = unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawElementsIndirectAMD_t = unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const void, primcount: GLsizei, stride: GLsizei);

pub type glMultiDrawElementsIndirectBindlessCountNV_t = unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const void, drawCount: GLsizei, maxDrawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

pub type glMultiDrawElementsIndirectBindlessNV_t = unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const void, drawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);

pub type glMultiDrawElementsIndirectCount_t = unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawElementsIndirectCountARB_t = unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawElementsIndirectEXT_t = unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawMeshTasksIndirectCountNV_t = unsafe extern "system" fn(indirect: GLintptr, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawMeshTasksIndirectNV_t = unsafe extern "system" fn(indirect: GLintptr, drawcount: GLsizei, stride: GLsizei);

pub type glMultiDrawRangeElementArrayAPPLE_t = unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, first: *const GLint, count: *const GLsizei, primcount: GLsizei);

pub type glMultiModeDrawArraysIBM_t = unsafe extern "system" fn(mode: *const GLenum, first: *const GLint, count: *const GLsizei, primcount: GLsizei, modestride: GLint);

pub type glMultiModeDrawElementsIBM_t = unsafe extern "system" fn(mode: *const GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const void, primcount: GLsizei, modestride: GLint);

pub type glMultiTexBufferEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, internalformat: GLenum, buffer: GLuint);

pub type glMultiTexCoord1bOES_t = unsafe extern "system" fn(texture: GLenum, s: GLbyte);

pub type glMultiTexCoord1bvOES_t = unsafe extern "system" fn(texture: GLenum, coords: *const GLbyte);

pub type glMultiTexCoord1d_t = unsafe extern "system" fn(target: GLenum, s: GLdouble);

pub type glMultiTexCoord1dARB_t = unsafe extern "system" fn(target: GLenum, s: GLdouble);

pub type glMultiTexCoord1dv_t = unsafe extern "system" fn(target: GLenum, v: *const GLdouble);

pub type glMultiTexCoord1dvARB_t = unsafe extern "system" fn(target: GLenum, v: *const GLdouble);

pub type glMultiTexCoord1f_t = unsafe extern "system" fn(target: GLenum, s: GLfloat);

pub type glMultiTexCoord1fARB_t = unsafe extern "system" fn(target: GLenum, s: GLfloat);

pub type glMultiTexCoord1fv_t = unsafe extern "system" fn(target: GLenum, v: *const GLfloat);

pub type glMultiTexCoord1fvARB_t = unsafe extern "system" fn(target: GLenum, v: *const GLfloat);

pub type glMultiTexCoord1hNV_t = unsafe extern "system" fn(target: GLenum, s: GLhalfNV);

pub type glMultiTexCoord1hvNV_t = unsafe extern "system" fn(target: GLenum, v: *const GLhalfNV);

pub type glMultiTexCoord1i_t = unsafe extern "system" fn(target: GLenum, s: GLint);

pub type glMultiTexCoord1iARB_t = unsafe extern "system" fn(target: GLenum, s: GLint);

pub type glMultiTexCoord1iv_t = unsafe extern "system" fn(target: GLenum, v: *const GLint);

pub type glMultiTexCoord1ivARB_t = unsafe extern "system" fn(target: GLenum, v: *const GLint);

pub type glMultiTexCoord1s_t = unsafe extern "system" fn(target: GLenum, s: GLshort);

pub type glMultiTexCoord1sARB_t = unsafe extern "system" fn(target: GLenum, s: GLshort);

pub type glMultiTexCoord1sv_t = unsafe extern "system" fn(target: GLenum, v: *const GLshort);

pub type glMultiTexCoord1svARB_t = unsafe extern "system" fn(target: GLenum, v: *const GLshort);

pub type glMultiTexCoord1xOES_t = unsafe extern "system" fn(texture: GLenum, s: GLfixed);

pub type glMultiTexCoord1xvOES_t = unsafe extern "system" fn(texture: GLenum, coords: *const GLfixed);

pub type glMultiTexCoord2bOES_t = unsafe extern "system" fn(texture: GLenum, s: GLbyte, t: GLbyte);

pub type glMultiTexCoord2bvOES_t = unsafe extern "system" fn(texture: GLenum, coords: *const [GLbyte; 2]);

pub type glMultiTexCoord2d_t = unsafe extern "system" fn(target: GLenum, s: GLdouble, t: GLdouble);

pub type glMultiTexCoord2dARB_t = unsafe extern "system" fn(target: GLenum, s: GLdouble, t: GLdouble);

pub type glMultiTexCoord2dv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLdouble; 2]);

pub type glMultiTexCoord2dvARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLdouble; 2]);

pub type glMultiTexCoord2f_t = unsafe extern "system" fn(target: GLenum, s: GLfloat, t: GLfloat);

pub type glMultiTexCoord2fARB_t = unsafe extern "system" fn(target: GLenum, s: GLfloat, t: GLfloat);

pub type glMultiTexCoord2fv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLfloat; 2]);

pub type glMultiTexCoord2fvARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLfloat; 2]);

pub type glMultiTexCoord2hNV_t = unsafe extern "system" fn(target: GLenum, s: GLhalfNV, t: GLhalfNV);

pub type glMultiTexCoord2hvNV_t = unsafe extern "system" fn(target: GLenum, v: *const [GLhalfNV; 2]);

pub type glMultiTexCoord2i_t = unsafe extern "system" fn(target: GLenum, s: GLint, t: GLint);

pub type glMultiTexCoord2iARB_t = unsafe extern "system" fn(target: GLenum, s: GLint, t: GLint);

pub type glMultiTexCoord2iv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLint; 2]);

pub type glMultiTexCoord2ivARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLint; 2]);

pub type glMultiTexCoord2s_t = unsafe extern "system" fn(target: GLenum, s: GLshort, t: GLshort);

pub type glMultiTexCoord2sARB_t = unsafe extern "system" fn(target: GLenum, s: GLshort, t: GLshort);

pub type glMultiTexCoord2sv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLshort; 2]);

pub type glMultiTexCoord2svARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLshort; 2]);

pub type glMultiTexCoord2xOES_t = unsafe extern "system" fn(texture: GLenum, s: GLfixed, t: GLfixed);

pub type glMultiTexCoord2xvOES_t = unsafe extern "system" fn(texture: GLenum, coords: *const [GLfixed; 2]);

pub type glMultiTexCoord3bOES_t = unsafe extern "system" fn(texture: GLenum, s: GLbyte, t: GLbyte, r: GLbyte);

pub type glMultiTexCoord3bvOES_t = unsafe extern "system" fn(texture: GLenum, coords: *const [GLbyte; 3]);

pub type glMultiTexCoord3d_t = unsafe extern "system" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble);

pub type glMultiTexCoord3dARB_t = unsafe extern "system" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble);

pub type glMultiTexCoord3dv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLdouble; 3]);

pub type glMultiTexCoord3dvARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLdouble; 3]);

pub type glMultiTexCoord3f_t = unsafe extern "system" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat);

pub type glMultiTexCoord3fARB_t = unsafe extern "system" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat);

pub type glMultiTexCoord3fv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLfloat; 3]);

pub type glMultiTexCoord3fvARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLfloat; 3]);

pub type glMultiTexCoord3hNV_t = unsafe extern "system" fn(target: GLenum, s: GLhalfNV, t: GLhalfNV, r: GLhalfNV);

pub type glMultiTexCoord3hvNV_t = unsafe extern "system" fn(target: GLenum, v: *const [GLhalfNV; 3]);

pub type glMultiTexCoord3i_t = unsafe extern "system" fn(target: GLenum, s: GLint, t: GLint, r: GLint);

pub type glMultiTexCoord3iARB_t = unsafe extern "system" fn(target: GLenum, s: GLint, t: GLint, r: GLint);

pub type glMultiTexCoord3iv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLint; 3]);

pub type glMultiTexCoord3ivARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLint; 3]);

pub type glMultiTexCoord3s_t = unsafe extern "system" fn(target: GLenum, s: GLshort, t: GLshort, r: GLshort);

pub type glMultiTexCoord3sARB_t = unsafe extern "system" fn(target: GLenum, s: GLshort, t: GLshort, r: GLshort);

pub type glMultiTexCoord3sv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLshort; 3]);

pub type glMultiTexCoord3svARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLshort; 3]);

pub type glMultiTexCoord3xOES_t = unsafe extern "system" fn(texture: GLenum, s: GLfixed, t: GLfixed, r: GLfixed);

pub type glMultiTexCoord3xvOES_t = unsafe extern "system" fn(texture: GLenum, coords: *const [GLfixed; 3]);

pub type glMultiTexCoord4bOES_t = unsafe extern "system" fn(texture: GLenum, s: GLbyte, t: GLbyte, r: GLbyte, q: GLbyte);

pub type glMultiTexCoord4bvOES_t = unsafe extern "system" fn(texture: GLenum, coords: *const [GLbyte; 4]);

pub type glMultiTexCoord4d_t = unsafe extern "system" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

pub type glMultiTexCoord4dARB_t = unsafe extern "system" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

pub type glMultiTexCoord4dv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLdouble; 4]);

pub type glMultiTexCoord4dvARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLdouble; 4]);

pub type glMultiTexCoord4f_t = unsafe extern "system" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

pub type glMultiTexCoord4fARB_t = unsafe extern "system" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

pub type glMultiTexCoord4fv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLfloat; 4]);

pub type glMultiTexCoord4fvARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLfloat; 4]);

pub type glMultiTexCoord4hNV_t = unsafe extern "system" fn(target: GLenum, s: GLhalfNV, t: GLhalfNV, r: GLhalfNV, q: GLhalfNV);

pub type glMultiTexCoord4hvNV_t = unsafe extern "system" fn(target: GLenum, v: *const [GLhalfNV; 4]);

pub type glMultiTexCoord4i_t = unsafe extern "system" fn(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint);

pub type glMultiTexCoord4iARB_t = unsafe extern "system" fn(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint);

pub type glMultiTexCoord4iv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLint; 4]);

pub type glMultiTexCoord4ivARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLint; 4]);

pub type glMultiTexCoord4s_t = unsafe extern "system" fn(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort);

pub type glMultiTexCoord4sARB_t = unsafe extern "system" fn(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort);

pub type glMultiTexCoord4sv_t = unsafe extern "system" fn(target: GLenum, v: *const [GLshort; 4]);

pub type glMultiTexCoord4svARB_t = unsafe extern "system" fn(target: GLenum, v: *const [GLshort; 4]);

pub type glMultiTexCoord4x_t = unsafe extern "system" fn(texture: GLenum, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

pub type glMultiTexCoord4xOES_t = unsafe extern "system" fn(texture: GLenum, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

pub type glMultiTexCoord4xvOES_t = unsafe extern "system" fn(texture: GLenum, coords: *const [GLfixed; 4]);

pub type glMultiTexCoordP1ui_t = unsafe extern "system" fn(texture: GLenum, type_: GLenum, coords: GLuint);

pub type glMultiTexCoordP1uiv_t = unsafe extern "system" fn(texture: GLenum, type_: GLenum, coords: *const GLuint);

pub type glMultiTexCoordP2ui_t = unsafe extern "system" fn(texture: GLenum, type_: GLenum, coords: GLuint);

pub type glMultiTexCoordP2uiv_t = unsafe extern "system" fn(texture: GLenum, type_: GLenum, coords: *const GLuint);

pub type glMultiTexCoordP3ui_t = unsafe extern "system" fn(texture: GLenum, type_: GLenum, coords: GLuint);

pub type glMultiTexCoordP3uiv_t = unsafe extern "system" fn(texture: GLenum, type_: GLenum, coords: *const GLuint);

pub type glMultiTexCoordP4ui_t = unsafe extern "system" fn(texture: GLenum, type_: GLenum, coords: GLuint);

pub type glMultiTexCoordP4uiv_t = unsafe extern "system" fn(texture: GLenum, type_: GLenum, coords: *const GLuint);

pub type glMultiTexCoordPointerEXT_t = unsafe extern "system" fn(texunit: GLenum, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glMultiTexEnvfEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, param: GLfloat);

pub type glMultiTexEnvfvEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glMultiTexEnviEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, param: GLint);

pub type glMultiTexEnvivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);

pub type glMultiTexGendEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, param: GLdouble);

pub type glMultiTexGendvEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLdouble);

pub type glMultiTexGenfEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, param: GLfloat);

pub type glMultiTexGenfvEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLfloat);

pub type glMultiTexGeniEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, param: GLint);

pub type glMultiTexGenivEXT_t = unsafe extern "system" fn(texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLint);

pub type glMultiTexImage1DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glMultiTexImage2DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glMultiTexImage3DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glMultiTexParameterIivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);

pub type glMultiTexParameterIuivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLuint);

pub type glMultiTexParameterfEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, param: GLfloat);

pub type glMultiTexParameterfvEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glMultiTexParameteriEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, param: GLint);

pub type glMultiTexParameterivEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);

pub type glMultiTexRenderbufferEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, renderbuffer: GLuint);

pub type glMultiTexSubImage1DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glMultiTexSubImage2DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glMultiTexSubImage3DEXT_t = unsafe extern "system" fn(texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glMulticastBarrierNV_t = unsafe extern "system" fn();

pub type glMulticastBlitFramebufferNV_t = unsafe extern "system" fn(srcGpu: GLuint, dstGpu: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);

pub type glMulticastBufferSubDataNV_t = unsafe extern "system" fn(gpuMask: GLbitfield, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

pub type glMulticastCopyBufferSubDataNV_t = unsafe extern "system" fn(readGpu: GLuint, writeGpuMask: GLbitfield, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

pub type glMulticastCopyImageSubDataNV_t = unsafe extern "system" fn(srcGpu: GLuint, dstGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

pub type glMulticastFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(gpu: GLuint, framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

pub type glMulticastGetQueryObjecti64vNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLint64);

pub type glMulticastGetQueryObjectivNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLint);

pub type glMulticastGetQueryObjectui64vNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLuint64);

pub type glMulticastGetQueryObjectuivNV_t = unsafe extern "system" fn(gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLuint);

pub type glMulticastScissorArrayvNVX_t = unsafe extern "system" fn(gpu: GLuint, first: GLuint, count: GLsizei, v: *const GLint);

pub type glMulticastViewportArrayvNVX_t = unsafe extern "system" fn(gpu: GLuint, first: GLuint, count: GLsizei, v: *const GLfloat);

pub type glMulticastViewportPositionWScaleNVX_t = unsafe extern "system" fn(gpu: GLuint, index: GLuint, xcoeff: GLfloat, ycoeff: GLfloat);

pub type glMulticastWaitSyncNV_t = unsafe extern "system" fn(signalGpu: GLuint, waitGpuMask: GLbitfield);

pub type glNamedBufferAttachMemoryNV_t = unsafe extern "system" fn(buffer: GLuint, memory: GLuint, offset: GLuint64);

pub type glNamedBufferData_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: GLenum);

pub type glNamedBufferDataEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: GLenum);

pub type glNamedBufferPageCommitmentARB_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

pub type glNamedBufferPageCommitmentEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);

pub type glNamedBufferPageCommitmentMemNV_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, memory: GLuint, memOffset: GLuint64, commit: GLboolean);

pub type glNamedBufferStorage_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield);

pub type glNamedBufferStorageEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield);

pub type glNamedBufferStorageExternalEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, clientBuffer: GLeglClientBufferEXT, flags: GLbitfield);

pub type glNamedBufferStorageMemEXT_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, memory: GLuint, offset: GLuint64);

pub type glNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

pub type glNamedBufferSubDataEXT_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

pub type glNamedCopyBufferSubDataEXT_t = unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

pub type glNamedFramebufferDrawBuffer_t = unsafe extern "system" fn(framebuffer: GLuint, buf: GLenum);

pub type glNamedFramebufferDrawBuffers_t = unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);

pub type glNamedFramebufferParameteri_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: GLint);

pub type glNamedFramebufferParameteriEXT_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: GLint);

pub type glNamedFramebufferReadBuffer_t = unsafe extern "system" fn(framebuffer: GLuint, src: GLenum);

pub type glNamedFramebufferRenderbuffer_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);

pub type glNamedFramebufferRenderbufferEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);

pub type glNamedFramebufferSampleLocationsfvARB_t = unsafe extern "system" fn(framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

pub type glNamedFramebufferSampleLocationsfvNV_t = unsafe extern "system" fn(framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);

pub type glNamedFramebufferSamplePositionsfvAMD_t = unsafe extern "system" fn(framebuffer: GLuint, numsamples: GLuint, pixelindex: GLuint, values: *const GLfloat);

pub type glNamedFramebufferTexture_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);

pub type glNamedFramebufferTexture1DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);

pub type glNamedFramebufferTexture2DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);

pub type glNamedFramebufferTexture3DEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);

pub type glNamedFramebufferTextureEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);

pub type glNamedFramebufferTextureFaceEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);

pub type glNamedFramebufferTextureLayer_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);

pub type glNamedFramebufferTextureLayerEXT_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);

pub type glNamedProgramLocalParameter4dEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glNamedProgramLocalParameter4dvEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, params: *const [GLdouble; 4]);

pub type glNamedProgramLocalParameter4fEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glNamedProgramLocalParameter4fvEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, params: *const [GLfloat; 4]);

pub type glNamedProgramLocalParameterI4iEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

pub type glNamedProgramLocalParameterI4ivEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, params: *const [GLint; 4]);

pub type glNamedProgramLocalParameterI4uiEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

pub type glNamedProgramLocalParameterI4uivEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, params: *const [GLuint; 4]);

pub type glNamedProgramLocalParameters4fvEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLfloat);

pub type glNamedProgramLocalParametersI4ivEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLint);

pub type glNamedProgramLocalParametersI4uivEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLuint);

pub type glNamedProgramStringEXT_t = unsafe extern "system" fn(program: GLuint, target: GLenum, format: GLenum, len: GLsizei, string: *const void);

pub type glNamedRenderbufferStorage_t = unsafe extern "system" fn(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glNamedRenderbufferStorageEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glNamedRenderbufferStorageMultisample_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glNamedRenderbufferStorageMultisampleAdvancedAMD_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, storageSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glNamedRenderbufferStorageMultisampleCoverageEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glNamedRenderbufferStorageMultisampleEXT_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glNamedStringARB_t = unsafe extern "system" fn(type_: GLenum, namelen: GLint, name: *const GLchar, stringlen: GLint, string: *const GLchar);

pub type glNewList_t = unsafe extern "system" fn(list: GLuint, mode: GLenum);

pub type glNewObjectBufferATI_t = unsafe extern "system" fn(size: GLsizei, pointer: *const void, usage: GLenum) -> GLuint;

pub type glNormal3b_t = unsafe extern "system" fn(nx: GLbyte, ny: GLbyte, nz: GLbyte);

pub type glNormal3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

pub type glNormal3d_t = unsafe extern "system" fn(nx: GLdouble, ny: GLdouble, nz: GLdouble);

pub type glNormal3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glNormal3f_t = unsafe extern "system" fn(nx: GLfloat, ny: GLfloat, nz: GLfloat);

pub type glNormal3fVertex3fSUN_t = unsafe extern "system" fn(nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glNormal3fVertex3fvSUN_t = unsafe extern "system" fn(n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glNormal3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glNormal3hNV_t = unsafe extern "system" fn(nx: GLhalfNV, ny: GLhalfNV, nz: GLhalfNV);

pub type glNormal3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

pub type glNormal3i_t = unsafe extern "system" fn(nx: GLint, ny: GLint, nz: GLint);

pub type glNormal3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glNormal3s_t = unsafe extern "system" fn(nx: GLshort, ny: GLshort, nz: GLshort);

pub type glNormal3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glNormal3x_t = unsafe extern "system" fn(nx: GLfixed, ny: GLfixed, nz: GLfixed);

pub type glNormal3xOES_t = unsafe extern "system" fn(nx: GLfixed, ny: GLfixed, nz: GLfixed);

pub type glNormal3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

pub type glNormalFormatNV_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei);

pub type glNormalP3ui_t = unsafe extern "system" fn(type_: GLenum, coords: GLuint);

pub type glNormalP3uiv_t = unsafe extern "system" fn(type_: GLenum, coords: *const GLuint);

pub type glNormalPointer_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glNormalPointerEXT_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const void);

pub type glNormalPointerListIBM_t = unsafe extern "system" fn(type_: GLenum, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

pub type glNormalPointervINTEL_t = unsafe extern "system" fn(type_: GLenum, pointer: *const [*mut void; 4]);

pub type glNormalStream3bATI_t = unsafe extern "system" fn(stream: GLenum, nx: GLbyte, ny: GLbyte, nz: GLbyte);

pub type glNormalStream3bvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLbyte; 3]);

pub type glNormalStream3dATI_t = unsafe extern "system" fn(stream: GLenum, nx: GLdouble, ny: GLdouble, nz: GLdouble);

pub type glNormalStream3dvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLdouble; 3]);

pub type glNormalStream3fATI_t = unsafe extern "system" fn(stream: GLenum, nx: GLfloat, ny: GLfloat, nz: GLfloat);

pub type glNormalStream3fvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLfloat; 3]);

pub type glNormalStream3iATI_t = unsafe extern "system" fn(stream: GLenum, nx: GLint, ny: GLint, nz: GLint);

pub type glNormalStream3ivATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLint; 3]);

pub type glNormalStream3sATI_t = unsafe extern "system" fn(stream: GLenum, nx: GLshort, ny: GLshort, nz: GLshort);

pub type glNormalStream3svATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLshort; 3]);

pub type glObjectLabel_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar);

pub type glObjectLabelKHR_t = unsafe extern "system" fn(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar);

pub type glObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, length: GLsizei, label: *const GLchar);

pub type glObjectPtrLabelKHR_t = unsafe extern "system" fn(ptr: *const void, length: GLsizei, label: *const GLchar);

pub type glObjectPurgeableAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, option: GLenum) -> GLenum;

pub type glObjectUnpurgeableAPPLE_t = unsafe extern "system" fn(objectType: GLenum, name: GLuint, option: GLenum) -> GLenum;

pub type glOrtho_t = unsafe extern "system" fn(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);

pub type glOrthof_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

pub type glOrthofOES_t = unsafe extern "system" fn(l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);

pub type glOrthox_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

pub type glOrthoxOES_t = unsafe extern "system" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);

pub type glPNTrianglesfATI_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glPNTrianglesiATI_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glPassTexCoordATI_t = unsafe extern "system" fn(dst: GLuint, coord: GLuint, swizzle: GLenum);

pub type glPassThrough_t = unsafe extern "system" fn(token: GLfloat);

pub type glPassThroughxOES_t = unsafe extern "system" fn(token: GLfixed);

pub type glPatchParameterfv_t = unsafe extern "system" fn(pname: GLenum, values: *const GLfloat);

pub type glPatchParameteri_t = unsafe extern "system" fn(pname: GLenum, value: GLint);

pub type glPatchParameteriEXT_t = unsafe extern "system" fn(pname: GLenum, value: GLint);

pub type glPatchParameteriOES_t = unsafe extern "system" fn(pname: GLenum, value: GLint);

pub type glPathColorGenNV_t = unsafe extern "system" fn(color: GLenum, genMode: GLenum, colorFormat: GLenum, coeffs: *const GLfloat);

pub type glPathCommandsNV_t = unsafe extern "system" fn(path: GLuint, numCommands: GLsizei, commands: *const GLubyte, numCoords: GLsizei, coordType: GLenum, coords: *const void);

pub type glPathCoordsNV_t = unsafe extern "system" fn(path: GLuint, numCoords: GLsizei, coordType: GLenum, coords: *const void);

pub type glPathCoverDepthFuncNV_t = unsafe extern "system" fn(func: GLenum);

pub type glPathDashArrayNV_t = unsafe extern "system" fn(path: GLuint, dashCount: GLsizei, dashArray: *const GLfloat);

pub type glPathFogGenNV_t = unsafe extern "system" fn(genMode: GLenum);

pub type glPathGlyphIndexArrayNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: GLenum, fontName: *const void, fontStyle: GLbitfield, firstGlyphIndex: GLuint, numGlyphs: GLsizei, pathParameterTemplate: GLuint, emScale: GLfloat) -> GLenum;

pub type glPathGlyphIndexRangeNV_t = unsafe extern "system" fn(fontTarget: GLenum, fontName: *const void, fontStyle: GLbitfield, pathParameterTemplate: GLuint, emScale: GLfloat, baseAndCount: GLuint) -> GLenum;

pub type glPathGlyphRangeNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: GLenum, fontName: *const void, fontStyle: GLbitfield, firstGlyph: GLuint, numGlyphs: GLsizei, handleMissingGlyphs: GLenum, pathParameterTemplate: GLuint, emScale: GLfloat);

pub type glPathGlyphsNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: GLenum, fontName: *const void, fontStyle: GLbitfield, numGlyphs: GLsizei, type_: GLenum, charcodes: *const void, handleMissingGlyphs: GLenum, pathParameterTemplate: GLuint, emScale: GLfloat);

pub type glPathMemoryGlyphIndexArrayNV_t = unsafe extern "system" fn(firstPathName: GLuint, fontTarget: GLenum, fontSize: GLsizeiptr, fontData: *const void, faceIndex: GLsizei, firstGlyphIndex: GLuint, numGlyphs: GLsizei, pathParameterTemplate: GLuint, emScale: GLfloat) -> GLenum;

pub type glPathParameterfNV_t = unsafe extern "system" fn(path: GLuint, pname: GLenum, value: GLfloat);

pub type glPathParameterfvNV_t = unsafe extern "system" fn(path: GLuint, pname: GLenum, value: *const GLfloat);

pub type glPathParameteriNV_t = unsafe extern "system" fn(path: GLuint, pname: GLenum, value: GLint);

pub type glPathParameterivNV_t = unsafe extern "system" fn(path: GLuint, pname: GLenum, value: *const GLint);

pub type glPathStencilDepthOffsetNV_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);

pub type glPathStencilFuncNV_t = unsafe extern "system" fn(func: GLenum, ref_: GLint, mask: GLuint);

pub type glPathStringNV_t = unsafe extern "system" fn(path: GLuint, format: GLenum, length: GLsizei, pathString: *const void);

pub type glPathSubCommandsNV_t = unsafe extern "system" fn(path: GLuint, commandStart: GLsizei, commandsToDelete: GLsizei, numCommands: GLsizei, commands: *const GLubyte, numCoords: GLsizei, coordType: GLenum, coords: *const void);

pub type glPathSubCoordsNV_t = unsafe extern "system" fn(path: GLuint, coordStart: GLsizei, numCoords: GLsizei, coordType: GLenum, coords: *const void);

pub type glPathTexGenNV_t = unsafe extern "system" fn(texCoordSet: GLenum, genMode: GLenum, components: GLint, coeffs: *const GLfloat);

pub type glPauseTransformFeedback_t = unsafe extern "system" fn();

pub type glPauseTransformFeedbackNV_t = unsafe extern "system" fn();

pub type glPixelDataRangeNV_t = unsafe extern "system" fn(target: GLenum, length: GLsizei, pointer: *const void);

pub type glPixelMapfv_t = unsafe extern "system" fn(map: GLenum, mapsize: GLsizei, values: *const GLfloat);

pub type glPixelMapuiv_t = unsafe extern "system" fn(map: GLenum, mapsize: GLsizei, values: *const GLuint);

pub type glPixelMapusv_t = unsafe extern "system" fn(map: GLenum, mapsize: GLsizei, values: *const GLushort);

pub type glPixelMapx_t = unsafe extern "system" fn(map: GLenum, size: GLint, values: *const GLfixed);

pub type glPixelStoref_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glPixelStorei_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glPixelStorex_t = unsafe extern "system" fn(pname: GLenum, param: GLfixed);

pub type glPixelTexGenParameterfSGIS_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glPixelTexGenParameterfvSGIS_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glPixelTexGenParameteriSGIS_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glPixelTexGenParameterivSGIS_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);

pub type glPixelTexGenSGIX_t = unsafe extern "system" fn(mode: GLenum);

pub type glPixelTransferf_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glPixelTransferi_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glPixelTransferxOES_t = unsafe extern "system" fn(pname: GLenum, param: GLfixed);

pub type glPixelTransformParameterfEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat);

pub type glPixelTransformParameterfvEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glPixelTransformParameteriEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

pub type glPixelTransformParameterivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glPixelZoom_t = unsafe extern "system" fn(xfactor: GLfloat, yfactor: GLfloat);

pub type glPixelZoomxOES_t = unsafe extern "system" fn(xfactor: GLfixed, yfactor: GLfixed);

pub type glPointAlongPathNV_t = unsafe extern "system" fn(path: GLuint, startSegment: GLsizei, numSegments: GLsizei, distance: GLfloat, x: *mut GLfloat, y: *mut GLfloat, tangentX: *mut GLfloat, tangentY: *mut GLfloat) -> GLboolean;

pub type glPointParameterf_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glPointParameterfARB_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glPointParameterfEXT_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glPointParameterfSGIS_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glPointParameterfv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glPointParameterfvARB_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glPointParameterfvEXT_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glPointParameterfvSGIS_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glPointParameteri_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glPointParameteriNV_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glPointParameteriv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);

pub type glPointParameterivNV_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);

pub type glPointParameterx_t = unsafe extern "system" fn(pname: GLenum, param: GLfixed);

pub type glPointParameterxOES_t = unsafe extern "system" fn(pname: GLenum, param: GLfixed);

pub type glPointParameterxv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfixed);

pub type glPointParameterxvOES_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfixed);

pub type glPointSize_t = unsafe extern "system" fn(size: GLfloat);

pub type glPointSizePointerOES_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glPointSizex_t = unsafe extern "system" fn(size: GLfixed);

pub type glPointSizexOES_t = unsafe extern "system" fn(size: GLfixed);

pub type glPollAsyncSGIX_t = unsafe extern "system" fn(markerp: *mut GLuint) -> GLint;

pub type glPollInstrumentsSGIX_t = unsafe extern "system" fn(marker_p: *mut GLint) -> GLint;

pub type glPolygonMode_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);

pub type glPolygonModeNV_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);

pub type glPolygonOffset_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);

pub type glPolygonOffsetClamp_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);

pub type glPolygonOffsetClampEXT_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);

pub type glPolygonOffsetEXT_t = unsafe extern "system" fn(factor: GLfloat, bias: GLfloat);

pub type glPolygonOffsetx_t = unsafe extern "system" fn(factor: GLfixed, units: GLfixed);

pub type glPolygonOffsetxOES_t = unsafe extern "system" fn(factor: GLfixed, units: GLfixed);

pub type glPolygonStipple_t = unsafe extern "system" fn(mask: *const GLubyte);

pub type glPopAttrib_t = unsafe extern "system" fn();

pub type glPopClientAttrib_t = unsafe extern "system" fn();

pub type glPopDebugGroup_t = unsafe extern "system" fn();

pub type glPopDebugGroupKHR_t = unsafe extern "system" fn();

pub type glPopGroupMarkerEXT_t = unsafe extern "system" fn();

pub type glPopMatrix_t = unsafe extern "system" fn();

pub type glPopName_t = unsafe extern "system" fn();

pub type glPresentFrameDualFillNV_t = unsafe extern "system" fn(video_slot: GLuint, minPresentTime: GLuint64EXT, beginPresentTimeId: GLuint, presentDurationId: GLuint, type_: GLenum, target0: GLenum, fill0: GLuint, target1: GLenum, fill1: GLuint, target2: GLenum, fill2: GLuint, target3: GLenum, fill3: GLuint);

pub type glPresentFrameKeyedNV_t = unsafe extern "system" fn(video_slot: GLuint, minPresentTime: GLuint64EXT, beginPresentTimeId: GLuint, presentDurationId: GLuint, type_: GLenum, target0: GLenum, fill0: GLuint, key0: GLuint, target1: GLenum, fill1: GLuint, key1: GLuint);

pub type glPrimitiveBoundingBox_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

pub type glPrimitiveBoundingBoxARB_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

pub type glPrimitiveBoundingBoxEXT_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

pub type glPrimitiveBoundingBoxOES_t = unsafe extern "system" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);

pub type glPrimitiveRestartIndex_t = unsafe extern "system" fn(index: GLuint);

pub type glPrimitiveRestartIndexNV_t = unsafe extern "system" fn(index: GLuint);

pub type glPrimitiveRestartNV_t = unsafe extern "system" fn();

pub type glPrioritizeTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfloat);

pub type glPrioritizeTexturesEXT_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLclampf);

pub type glPrioritizeTexturesxOES_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfixed);

pub type glProgramBinary_t = unsafe extern "system" fn(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLsizei);

pub type glProgramBinaryOES_t = unsafe extern "system" fn(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLint);

pub type glProgramBufferParametersIivNV_t = unsafe extern "system" fn(target: GLenum, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLint);

pub type glProgramBufferParametersIuivNV_t = unsafe extern "system" fn(target: GLenum, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLuint);

pub type glProgramBufferParametersfvNV_t = unsafe extern "system" fn(target: GLenum, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLfloat);

pub type glProgramEnvParameter4dARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glProgramEnvParameter4dvARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *const [GLdouble; 4]);

pub type glProgramEnvParameter4fARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glProgramEnvParameter4fvARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *const [GLfloat; 4]);

pub type glProgramEnvParameterI4iNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

pub type glProgramEnvParameterI4ivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *const [GLint; 4]);

pub type glProgramEnvParameterI4uiNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

pub type glProgramEnvParameterI4uivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *const [GLuint; 4]);

pub type glProgramEnvParameters4fvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, count: GLsizei, params: *const GLfloat);

pub type glProgramEnvParametersI4ivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, count: GLsizei, params: *const GLint);

pub type glProgramEnvParametersI4uivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, count: GLsizei, params: *const GLuint);

pub type glProgramLocalParameter4dARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glProgramLocalParameter4dvARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *const [GLdouble; 4]);

pub type glProgramLocalParameter4fARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glProgramLocalParameter4fvARB_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *const [GLfloat; 4]);

pub type glProgramLocalParameterI4iNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

pub type glProgramLocalParameterI4ivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *const [GLint; 4]);

pub type glProgramLocalParameterI4uiNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

pub type glProgramLocalParameterI4uivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, params: *const [GLuint; 4]);

pub type glProgramLocalParameters4fvEXT_t = unsafe extern "system" fn(target: GLenum, index: GLuint, count: GLsizei, params: *const GLfloat);

pub type glProgramLocalParametersI4ivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, count: GLsizei, params: *const GLint);

pub type glProgramLocalParametersI4uivNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, count: GLsizei, params: *const GLuint);

pub type glProgramNamedParameter4dNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glProgramNamedParameter4dvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, v: *const [GLdouble; 4]);

pub type glProgramNamedParameter4fNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glProgramNamedParameter4fvNV_t = unsafe extern "system" fn(id: GLuint, len: GLsizei, name: *const GLubyte, v: *const [GLfloat; 4]);

pub type glProgramParameter4dNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glProgramParameter4dvNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, v: *const [GLdouble; 4]);

pub type glProgramParameter4fNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glProgramParameter4fvNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, v: *const [GLfloat; 4]);

pub type glProgramParameteri_t = unsafe extern "system" fn(program: GLuint, pname: GLenum, value: GLint);

pub type glProgramParameteriARB_t = unsafe extern "system" fn(program: GLuint, pname: GLenum, value: GLint);

pub type glProgramParameteriEXT_t = unsafe extern "system" fn(program: GLuint, pname: GLenum, value: GLint);

pub type glProgramParameters4dvNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, count: GLsizei, v: *const GLdouble);

pub type glProgramParameters4fvNV_t = unsafe extern "system" fn(target: GLenum, index: GLuint, count: GLsizei, v: *const GLfloat);

pub type glProgramPathFragmentInputGenNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, genMode: GLenum, components: GLint, coeffs: *const GLfloat);

pub type glProgramStringARB_t = unsafe extern "system" fn(target: GLenum, format: GLenum, len: GLsizei, string: *const void);

pub type glProgramSubroutineParametersuivNV_t = unsafe extern "system" fn(target: GLenum, count: GLsizei, params: *const GLuint);

pub type glProgramUniform1d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble);

pub type glProgramUniform1dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble);

pub type glProgramUniform1dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub type glProgramUniform1dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub type glProgramUniform1f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);

pub type glProgramUniform1fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);

pub type glProgramUniform1fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub type glProgramUniform1fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub type glProgramUniform1i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);

pub type glProgramUniform1i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64);

pub type glProgramUniform1i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT);

pub type glProgramUniform1i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

pub type glProgramUniform1i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

pub type glProgramUniform1iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);

pub type glProgramUniform1iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub type glProgramUniform1ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub type glProgramUniform1ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);

pub type glProgramUniform1ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64);

pub type glProgramUniform1ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT);

pub type glProgramUniform1ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

pub type glProgramUniform1ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glProgramUniform1uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);

pub type glProgramUniform1uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub type glProgramUniform1uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub type glProgramUniform2d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);

pub type glProgramUniform2dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble);

pub type glProgramUniform2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub type glProgramUniform2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub type glProgramUniform2f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);

pub type glProgramUniform2fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);

pub type glProgramUniform2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub type glProgramUniform2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub type glProgramUniform2i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);

pub type glProgramUniform2i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64);

pub type glProgramUniform2i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT);

pub type glProgramUniform2i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

pub type glProgramUniform2i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

pub type glProgramUniform2iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);

pub type glProgramUniform2iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub type glProgramUniform2ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub type glProgramUniform2ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);

pub type glProgramUniform2ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64);

pub type glProgramUniform2ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT);

pub type glProgramUniform2ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

pub type glProgramUniform2ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glProgramUniform2uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);

pub type glProgramUniform2uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub type glProgramUniform2uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub type glProgramUniform3d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);

pub type glProgramUniform3dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glProgramUniform3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub type glProgramUniform3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub type glProgramUniform3f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

pub type glProgramUniform3fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

pub type glProgramUniform3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub type glProgramUniform3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub type glProgramUniform3i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);

pub type glProgramUniform3i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64);

pub type glProgramUniform3i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

pub type glProgramUniform3i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

pub type glProgramUniform3i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

pub type glProgramUniform3iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);

pub type glProgramUniform3iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub type glProgramUniform3ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub type glProgramUniform3ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

pub type glProgramUniform3ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);

pub type glProgramUniform3ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

pub type glProgramUniform3ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

pub type glProgramUniform3ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glProgramUniform3uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

pub type glProgramUniform3uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub type glProgramUniform3uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub type glProgramUniform4d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);

pub type glProgramUniform4dEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glProgramUniform4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub type glProgramUniform4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub type glProgramUniform4f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

pub type glProgramUniform4fEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

pub type glProgramUniform4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub type glProgramUniform4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub type glProgramUniform4i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

pub type glProgramUniform4i64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);

pub type glProgramUniform4i64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

pub type glProgramUniform4i64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);

pub type glProgramUniform4i64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);

pub type glProgramUniform4iEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

pub type glProgramUniform4iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub type glProgramUniform4ivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub type glProgramUniform4ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

pub type glProgramUniform4ui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);

pub type glProgramUniform4ui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

pub type glProgramUniform4ui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);

pub type glProgramUniform4ui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glProgramUniform4uiEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

pub type glProgramUniform4uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub type glProgramUniform4uivEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub type glProgramUniformHandleui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

pub type glProgramUniformHandleui64IMG_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

pub type glProgramUniformHandleui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

pub type glProgramUniformHandleui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

pub type glProgramUniformHandleui64vIMG_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

pub type glProgramUniformHandleui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

pub type glProgramUniformMatrix2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix2x3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix2x3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix2x3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix2x3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix2x4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix2x4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix2x4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix2x4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix3x2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix3x2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix3x2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix3x2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix3x4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix3x4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix3x4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix3x4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix4dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix4fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix4x2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix4x2dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix4x2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix4x2fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix4x3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix4x3dvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glProgramUniformMatrix4x3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformMatrix4x3fvEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glProgramUniformui64NV_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64EXT);

pub type glProgramUniformui64vNV_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glProgramVertexLimitNV_t = unsafe extern "system" fn(target: GLenum, limit: GLint);

pub type glProvokingVertex_t = unsafe extern "system" fn(mode: GLenum);

pub type glProvokingVertexEXT_t = unsafe extern "system" fn(mode: GLenum);

pub type glPushAttrib_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glPushClientAttrib_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glPushClientAttribDefaultEXT_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glPushDebugGroup_t = unsafe extern "system" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);

pub type glPushDebugGroupKHR_t = unsafe extern "system" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);

pub type glPushGroupMarkerEXT_t = unsafe extern "system" fn(length: GLsizei, marker: *const GLchar);

pub type glPushMatrix_t = unsafe extern "system" fn();

pub type glPushName_t = unsafe extern "system" fn(name: GLuint);

pub type glQueryCounter_t = unsafe extern "system" fn(id: GLuint, target: GLenum);

pub type glQueryCounterEXT_t = unsafe extern "system" fn(id: GLuint, target: GLenum);

pub type glQueryMatrixxOES_t = unsafe extern "system" fn(mantissa: *mut [GLfixed; 16], exponent: *mut [GLint; 16]) -> GLbitfield;

pub type glQueryObjectParameteruiAMD_t = unsafe extern "system" fn(target: GLenum, id: GLuint, pname: GLenum, param: GLuint);

pub type glQueryResourceNV_t = unsafe extern "system" fn(queryType: GLenum, tagId: GLint, count: GLuint, buffer: *mut GLint) -> GLint;

pub type glQueryResourceTagNV_t = unsafe extern "system" fn(tagId: GLint, tagString: *const GLchar);

pub type glRasterPos2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

pub type glRasterPos2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

pub type glRasterPos2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

pub type glRasterPos2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

pub type glRasterPos2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

pub type glRasterPos2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

pub type glRasterPos2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

pub type glRasterPos2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

pub type glRasterPos2xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed);

pub type glRasterPos2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

pub type glRasterPos3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

pub type glRasterPos3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glRasterPos3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

pub type glRasterPos3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glRasterPos3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

pub type glRasterPos3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glRasterPos3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

pub type glRasterPos3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glRasterPos3xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

pub type glRasterPos3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

pub type glRasterPos4d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glRasterPos4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

pub type glRasterPos4f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glRasterPos4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

pub type glRasterPos4i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

pub type glRasterPos4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

pub type glRasterPos4s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub type glRasterPos4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

pub type glRasterPos4xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed, w: GLfixed);

pub type glRasterPos4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

pub type glRasterSamplesEXT_t = unsafe extern "system" fn(samples: GLuint, fixedsamplelocations: GLboolean);

pub type glReadBuffer_t = unsafe extern "system" fn(src: GLenum);

pub type glReadBufferIndexedEXT_t = unsafe extern "system" fn(src: GLenum, index: GLint);

pub type glReadBufferNV_t = unsafe extern "system" fn(mode: GLenum);

pub type glReadInstrumentsSGIX_t = unsafe extern "system" fn(marker: GLint);

pub type glReadPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut void);

pub type glReadnPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut void);

pub type glReadnPixelsARB_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut void);

pub type glReadnPixelsEXT_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut void);

pub type glReadnPixelsKHR_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut void);

pub type glRectd_t = unsafe extern "system" fn(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble);

pub type glRectdv_t = unsafe extern "system" fn(v1: *const [GLdouble; 2], v2: *const [GLdouble; 2]);

pub type glRectf_t = unsafe extern "system" fn(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat);

pub type glRectfv_t = unsafe extern "system" fn(v1: *const [GLfloat; 2], v2: *const [GLfloat; 2]);

pub type glRecti_t = unsafe extern "system" fn(x1: GLint, y1: GLint, x2: GLint, y2: GLint);

pub type glRectiv_t = unsafe extern "system" fn(v1: *const [GLint; 2], v2: *const [GLint; 2]);

pub type glRects_t = unsafe extern "system" fn(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort);

pub type glRectsv_t = unsafe extern "system" fn(v1: *const [GLshort; 2], v2: *const [GLshort; 2]);

pub type glRectxOES_t = unsafe extern "system" fn(x1: GLfixed, y1: GLfixed, x2: GLfixed, y2: GLfixed);

pub type glRectxvOES_t = unsafe extern "system" fn(v1: *const [GLfixed; 2], v2: *const [GLfixed; 2]);

pub type glReferencePlaneSGIX_t = unsafe extern "system" fn(equation: *const [GLdouble; 4]);

pub type glReleaseKeyedMutexWin32EXT_t = unsafe extern "system" fn(memory: GLuint, key: GLuint64) -> GLboolean;

pub type glReleaseShaderCompiler_t = unsafe extern "system" fn();

pub type glRenderGpuMaskNV_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glRenderMode_t = unsafe extern "system" fn(mode: GLenum) -> GLint;

pub type glRenderbufferStorage_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageMultisample_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageMultisampleANGLE_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageMultisampleAPPLE_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageMultisampleAdvancedAMD_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, storageSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageMultisampleCoverageNV_t = unsafe extern "system" fn(target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageMultisampleEXT_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageMultisampleIMG_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageMultisampleNV_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glRenderbufferStorageOES_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glReplacementCodePointerSUN_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const *mut void);

pub type glReplacementCodeubSUN_t = unsafe extern "system" fn(code: GLubyte);

pub type glReplacementCodeubvSUN_t = unsafe extern "system" fn(code: *const GLubyte);

pub type glReplacementCodeuiColor3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glReplacementCodeuiColor3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glReplacementCodeuiColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glReplacementCodeuiColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glReplacementCodeuiColor4ubVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glReplacementCodeuiColor4ubVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

pub type glReplacementCodeuiNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glReplacementCodeuiNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glReplacementCodeuiSUN_t = unsafe extern "system" fn(code: GLuint);

pub type glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glReplacementCodeuiTexCoord2fVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, s: GLfloat, t: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glReplacementCodeuiTexCoord2fVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, tc: *const [GLfloat; 2], v: *const [GLfloat; 3]);

pub type glReplacementCodeuiVertex3fSUN_t = unsafe extern "system" fn(rc: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glReplacementCodeuiVertex3fvSUN_t = unsafe extern "system" fn(rc: *const GLuint, v: *const [GLfloat; 3]);

pub type glReplacementCodeuivSUN_t = unsafe extern "system" fn(code: *const GLuint);

pub type glReplacementCodeusSUN_t = unsafe extern "system" fn(code: GLushort);

pub type glReplacementCodeusvSUN_t = unsafe extern "system" fn(code: *const GLushort);

pub type glRequestResidentProgramsNV_t = unsafe extern "system" fn(n: GLsizei, programs: *const GLuint);

pub type glResetHistogram_t = unsafe extern "system" fn(target: GLenum);

pub type glResetHistogramEXT_t = unsafe extern "system" fn(target: GLenum);

pub type glResetMemoryObjectParameterNV_t = unsafe extern "system" fn(memory: GLuint, pname: GLenum);

pub type glResetMinmax_t = unsafe extern "system" fn(target: GLenum);

pub type glResetMinmaxEXT_t = unsafe extern "system" fn(target: GLenum);

pub type glResizeBuffersMESA_t = unsafe extern "system" fn();

pub type glResolveDepthValuesNV_t = unsafe extern "system" fn();

pub type glResolveMultisampleFramebufferAPPLE_t = unsafe extern "system" fn();

pub type glResumeTransformFeedback_t = unsafe extern "system" fn();

pub type glResumeTransformFeedbackNV_t = unsafe extern "system" fn();

pub type glRotated_t = unsafe extern "system" fn(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glRotatef_t = unsafe extern "system" fn(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glRotatex_t = unsafe extern "system" fn(angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);

pub type glRotatexOES_t = unsafe extern "system" fn(angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);

pub type glSampleCoverage_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);

pub type glSampleCoverageARB_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);

pub type glSampleCoveragex_t = unsafe extern "system" fn(value: GLclampx, invert: GLboolean);

pub type glSampleCoveragexOES_t = unsafe extern "system" fn(value: GLclampx, invert: GLboolean);

pub type glSampleMapATI_t = unsafe extern "system" fn(dst: GLuint, interp: GLuint, swizzle: GLenum);

pub type glSampleMaskEXT_t = unsafe extern "system" fn(value: GLclampf, invert: GLboolean);

pub type glSampleMaskIndexedNV_t = unsafe extern "system" fn(index: GLuint, mask: GLbitfield);

pub type glSampleMaskSGIS_t = unsafe extern "system" fn(value: GLclampf, invert: GLboolean);

pub type glSampleMaski_t = unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield);

pub type glSamplePatternEXT_t = unsafe extern "system" fn(pattern: GLenum);

pub type glSamplePatternSGIS_t = unsafe extern "system" fn(pattern: GLenum);

pub type glSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);

pub type glSamplerParameterIivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);

pub type glSamplerParameterIivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);

pub type glSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint);

pub type glSamplerParameterIuivEXT_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint);

pub type glSamplerParameterIuivOES_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint);

pub type glSamplerParameterf_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLfloat);

pub type glSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLfloat);

pub type glSamplerParameteri_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLint);

pub type glSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);

pub type glScaled_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

pub type glScalef_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

pub type glScalex_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

pub type glScalexOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

pub type glScissor_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glScissorArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

pub type glScissorArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

pub type glScissorArrayvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

pub type glScissorExclusiveArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

pub type glScissorExclusiveNV_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glScissorIndexed_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

pub type glScissorIndexedNV_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

pub type glScissorIndexedOES_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

pub type glScissorIndexedv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glScissorIndexedvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glScissorIndexedvOES_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glSecondaryColor3b_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

pub type glSecondaryColor3bEXT_t = unsafe extern "system" fn(red: GLbyte, green: GLbyte, blue: GLbyte);

pub type glSecondaryColor3bv_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

pub type glSecondaryColor3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

pub type glSecondaryColor3d_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

pub type glSecondaryColor3dEXT_t = unsafe extern "system" fn(red: GLdouble, green: GLdouble, blue: GLdouble);

pub type glSecondaryColor3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glSecondaryColor3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glSecondaryColor3f_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

pub type glSecondaryColor3fEXT_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat);

pub type glSecondaryColor3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glSecondaryColor3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glSecondaryColor3hNV_t = unsafe extern "system" fn(red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV);

pub type glSecondaryColor3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

pub type glSecondaryColor3i_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

pub type glSecondaryColor3iEXT_t = unsafe extern "system" fn(red: GLint, green: GLint, blue: GLint);

pub type glSecondaryColor3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glSecondaryColor3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glSecondaryColor3s_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

pub type glSecondaryColor3sEXT_t = unsafe extern "system" fn(red: GLshort, green: GLshort, blue: GLshort);

pub type glSecondaryColor3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glSecondaryColor3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glSecondaryColor3ub_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

pub type glSecondaryColor3ubEXT_t = unsafe extern "system" fn(red: GLubyte, green: GLubyte, blue: GLubyte);

pub type glSecondaryColor3ubv_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

pub type glSecondaryColor3ubvEXT_t = unsafe extern "system" fn(v: *const [GLubyte; 3]);

pub type glSecondaryColor3ui_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

pub type glSecondaryColor3uiEXT_t = unsafe extern "system" fn(red: GLuint, green: GLuint, blue: GLuint);

pub type glSecondaryColor3uiv_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

pub type glSecondaryColor3uivEXT_t = unsafe extern "system" fn(v: *const [GLuint; 3]);

pub type glSecondaryColor3us_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

pub type glSecondaryColor3usEXT_t = unsafe extern "system" fn(red: GLushort, green: GLushort, blue: GLushort);

pub type glSecondaryColor3usv_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

pub type glSecondaryColor3usvEXT_t = unsafe extern "system" fn(v: *const [GLushort; 3]);

pub type glSecondaryColorFormatNV_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei);

pub type glSecondaryColorP3ui_t = unsafe extern "system" fn(type_: GLenum, color: GLuint);

pub type glSecondaryColorP3uiv_t = unsafe extern "system" fn(type_: GLenum, color: *const GLuint);

pub type glSecondaryColorPointer_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glSecondaryColorPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glSecondaryColorPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

pub type glSelectBuffer_t = unsafe extern "system" fn(size: GLsizei, buffer: *mut GLuint);

pub type glSelectPerfMonitorCountersAMD_t = unsafe extern "system" fn(monitor: GLuint, enable: GLboolean, group: GLuint, numCounters: GLint, counterList: *mut GLuint);

pub type glSemaphoreParameterivNV_t = unsafe extern "system" fn(semaphore: GLuint, pname: GLenum, params: *const GLint);

pub type glSemaphoreParameterui64vEXT_t = unsafe extern "system" fn(semaphore: GLuint, pname: GLenum, params: *const GLuint64);

pub type glSeparableFilter2D_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, row: *const void, column: *const void);

pub type glSeparableFilter2DEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, row: *const void, column: *const void);

pub type glSetFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint);

pub type glSetFenceNV_t = unsafe extern "system" fn(fence: GLuint, condition: GLenum);

pub type glSetFragmentShaderConstantATI_t = unsafe extern "system" fn(dst: GLuint, value: *const [GLfloat; 4]);

pub type glSetInvariantEXT_t = unsafe extern "system" fn(id: GLuint, type_: GLenum, addr: *const void);

pub type glSetLocalConstantEXT_t = unsafe extern "system" fn(id: GLuint, type_: GLenum, addr: *const void);

pub type glSetMultisamplefvAMD_t = unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *const [GLfloat; 2]);

pub type glShadeModel_t = unsafe extern "system" fn(mode: GLenum);

pub type glShaderBinary_t = unsafe extern "system" fn(count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const void, length: GLsizei);

pub type glShaderOp1EXT_t = unsafe extern "system" fn(op: GLenum, res: GLuint, arg1: GLuint);

pub type glShaderOp2EXT_t = unsafe extern "system" fn(op: GLenum, res: GLuint, arg1: GLuint, arg2: GLuint);

pub type glShaderOp3EXT_t = unsafe extern "system" fn(op: GLenum, res: GLuint, arg1: GLuint, arg2: GLuint, arg3: GLuint);

pub type glShaderSource_t = unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);

pub type glShaderSourceARB_t = unsafe extern "system" fn(shaderObj: GLhandleARB, count: GLsizei, string: *const *mut GLcharARB, length: *const GLint);

pub type glShaderStorageBlockBinding_t = unsafe extern "system" fn(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);

pub type glShadingRateImageBarrierNV_t = unsafe extern "system" fn(synchronize: GLboolean);

pub type glShadingRateImagePaletteNV_t = unsafe extern "system" fn(viewport: GLuint, first: GLuint, count: GLsizei, rates: *const GLenum);

pub type glShadingRateQCOM_t = unsafe extern "system" fn(rate: GLenum);

pub type glShadingRateSampleOrderCustomNV_t = unsafe extern "system" fn(rate: GLenum, samples: GLuint, locations: *const GLint);

pub type glShadingRateSampleOrderNV_t = unsafe extern "system" fn(order: GLenum);

pub type glSharpenTexFuncSGIS_t = unsafe extern "system" fn(target: GLenum, n: GLsizei, points: *const GLfloat);

pub type glSignalSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint, numBufferBarriers: GLuint, buffers: *const GLuint, numTextureBarriers: GLuint, textures: *const GLuint, dstLayouts: *const GLenum);

pub type glSignalSemaphoreui64NVX_t = unsafe extern "system" fn(signalGpu: GLuint, fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

pub type glSignalVkFenceNV_t = unsafe extern "system" fn(vkFence: GLuint64);

pub type glSignalVkSemaphoreNV_t = unsafe extern "system" fn(vkSemaphore: GLuint64);

pub type glSpecializeShader_t = unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);

pub type glSpecializeShaderARB_t = unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);

pub type glSpriteParameterfSGIX_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glSpriteParameterfvSGIX_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);

pub type glSpriteParameteriSGIX_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glSpriteParameterivSGIX_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);

pub type glStartInstrumentsSGIX_t = unsafe extern "system" fn();

pub type glStartTilingQCOM_t = unsafe extern "system" fn(x: GLuint, y: GLuint, width: GLuint, height: GLuint, preserveMask: GLbitfield);

pub type glStateCaptureNV_t = unsafe extern "system" fn(state: GLuint, mode: GLenum);

pub type glStencilClearTagEXT_t = unsafe extern "system" fn(stencilTagBits: GLsizei, stencilClearTag: GLuint);

pub type glStencilFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, fillMode: GLenum, mask: GLuint, transformType: GLenum, transformValues: *const GLfloat);

pub type glStencilFillPathNV_t = unsafe extern "system" fn(path: GLuint, fillMode: GLenum, mask: GLuint);

pub type glStencilFunc_t = unsafe extern "system" fn(func: GLenum, ref_: GLint, mask: GLuint);

pub type glStencilFuncSeparate_t = unsafe extern "system" fn(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);

pub type glStencilFuncSeparateATI_t = unsafe extern "system" fn(frontfunc: GLenum, backfunc: GLenum, ref_: GLint, mask: GLuint);

pub type glStencilMask_t = unsafe extern "system" fn(mask: GLuint);

pub type glStencilMaskSeparate_t = unsafe extern "system" fn(face: GLenum, mask: GLuint);

pub type glStencilOp_t = unsafe extern "system" fn(fail: GLenum, zfail: GLenum, zpass: GLenum);

pub type glStencilOpSeparate_t = unsafe extern "system" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);

pub type glStencilOpSeparateATI_t = unsafe extern "system" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);

pub type glStencilOpValueAMD_t = unsafe extern "system" fn(face: GLenum, value: GLuint);

pub type glStencilStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, reference: GLint, mask: GLuint, transformType: GLenum, transformValues: *const GLfloat);

pub type glStencilStrokePathNV_t = unsafe extern "system" fn(path: GLuint, reference: GLint, mask: GLuint);

pub type glStencilThenCoverFillPathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, fillMode: GLenum, mask: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);

pub type glStencilThenCoverFillPathNV_t = unsafe extern "system" fn(path: GLuint, fillMode: GLenum, mask: GLuint, coverMode: GLenum);

pub type glStencilThenCoverStrokePathInstancedNV_t = unsafe extern "system" fn(numPaths: GLsizei, pathNameType: GLenum, paths: *const void, pathBase: GLuint, reference: GLint, mask: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);

pub type glStencilThenCoverStrokePathNV_t = unsafe extern "system" fn(path: GLuint, reference: GLint, mask: GLuint, coverMode: GLenum);

pub type glStopInstrumentsSGIX_t = unsafe extern "system" fn(marker: GLint);

pub type glStringMarkerGREMEDY_t = unsafe extern "system" fn(len: GLsizei, string: *const void);

pub type glSubpixelPrecisionBiasNV_t = unsafe extern "system" fn(xbits: GLuint, ybits: GLuint);

pub type glSwizzleEXT_t = unsafe extern "system" fn(res: GLuint, in_: GLuint, outX: GLenum, outY: GLenum, outZ: GLenum, outW: GLenum);

pub type glSyncTextureINTEL_t = unsafe extern "system" fn(texture: GLuint);

pub type glTagSampleBufferSGIX_t = unsafe extern "system" fn();

pub type glTangent3bEXT_t = unsafe extern "system" fn(tx: GLbyte, ty: GLbyte, tz: GLbyte);

pub type glTangent3bvEXT_t = unsafe extern "system" fn(v: *const [GLbyte; 3]);

pub type glTangent3dEXT_t = unsafe extern "system" fn(tx: GLdouble, ty: GLdouble, tz: GLdouble);

pub type glTangent3dvEXT_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glTangent3fEXT_t = unsafe extern "system" fn(tx: GLfloat, ty: GLfloat, tz: GLfloat);

pub type glTangent3fvEXT_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glTangent3iEXT_t = unsafe extern "system" fn(tx: GLint, ty: GLint, tz: GLint);

pub type glTangent3ivEXT_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glTangent3sEXT_t = unsafe extern "system" fn(tx: GLshort, ty: GLshort, tz: GLshort);

pub type glTangent3svEXT_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glTangentPointerEXT_t = unsafe extern "system" fn(type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glTbufferMask3DFX_t = unsafe extern "system" fn(mask: GLuint);

pub type glTessellationFactorAMD_t = unsafe extern "system" fn(factor: GLfloat);

pub type glTessellationModeAMD_t = unsafe extern "system" fn(mode: GLenum);

pub type glTestFenceAPPLE_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

pub type glTestFenceNV_t = unsafe extern "system" fn(fence: GLuint) -> GLboolean;

pub type glTestObjectAPPLE_t = unsafe extern "system" fn(object: GLenum, name: GLuint) -> GLboolean;

pub type glTexAttachMemoryNV_t = unsafe extern "system" fn(target: GLenum, memory: GLuint, offset: GLuint64);

pub type glTexBuffer_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);

pub type glTexBufferARB_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);

pub type glTexBufferEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);

pub type glTexBufferOES_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);

pub type glTexBufferRange_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glTexBufferRangeEXT_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glTexBufferRangeOES_t = unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glTexBumpParameterfvATI_t = unsafe extern "system" fn(pname: GLenum, param: *const GLfloat);

pub type glTexBumpParameterivATI_t = unsafe extern "system" fn(pname: GLenum, param: *const GLint);

pub type glTexCoord1bOES_t = unsafe extern "system" fn(s: GLbyte);

pub type glTexCoord1bvOES_t = unsafe extern "system" fn(coords: *const GLbyte);

pub type glTexCoord1d_t = unsafe extern "system" fn(s: GLdouble);

pub type glTexCoord1dv_t = unsafe extern "system" fn(v: *const GLdouble);

pub type glTexCoord1f_t = unsafe extern "system" fn(s: GLfloat);

pub type glTexCoord1fv_t = unsafe extern "system" fn(v: *const GLfloat);

pub type glTexCoord1hNV_t = unsafe extern "system" fn(s: GLhalfNV);

pub type glTexCoord1hvNV_t = unsafe extern "system" fn(v: *const GLhalfNV);

pub type glTexCoord1i_t = unsafe extern "system" fn(s: GLint);

pub type glTexCoord1iv_t = unsafe extern "system" fn(v: *const GLint);

pub type glTexCoord1s_t = unsafe extern "system" fn(s: GLshort);

pub type glTexCoord1sv_t = unsafe extern "system" fn(v: *const GLshort);

pub type glTexCoord1xOES_t = unsafe extern "system" fn(s: GLfixed);

pub type glTexCoord1xvOES_t = unsafe extern "system" fn(coords: *const GLfixed);

pub type glTexCoord2bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte);

pub type glTexCoord2bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 2]);

pub type glTexCoord2d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble);

pub type glTexCoord2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

pub type glTexCoord2f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat);

pub type glTexCoord2fColor3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glTexCoord2fColor3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glTexCoord2fColor4fNormal3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glTexCoord2fColor4fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glTexCoord2fColor4ubVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glTexCoord2fColor4ubVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], c: *const [GLubyte; 4], v: *const [GLfloat; 3]);

pub type glTexCoord2fNormal3fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glTexCoord2fNormal3fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], n: *const [GLfloat; 3], v: *const [GLfloat; 3]);

pub type glTexCoord2fVertex3fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glTexCoord2fVertex3fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 2], v: *const [GLfloat; 3]);

pub type glTexCoord2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

pub type glTexCoord2hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV);

pub type glTexCoord2hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 2]);

pub type glTexCoord2i_t = unsafe extern "system" fn(s: GLint, t: GLint);

pub type glTexCoord2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

pub type glTexCoord2s_t = unsafe extern "system" fn(s: GLshort, t: GLshort);

pub type glTexCoord2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

pub type glTexCoord2xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed);

pub type glTexCoord2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

pub type glTexCoord3bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte, r: GLbyte);

pub type glTexCoord3bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 3]);

pub type glTexCoord3d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble, r: GLdouble);

pub type glTexCoord3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glTexCoord3f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat);

pub type glTexCoord3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glTexCoord3hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV, r: GLhalfNV);

pub type glTexCoord3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

pub type glTexCoord3i_t = unsafe extern "system" fn(s: GLint, t: GLint, r: GLint);

pub type glTexCoord3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glTexCoord3s_t = unsafe extern "system" fn(s: GLshort, t: GLshort, r: GLshort);

pub type glTexCoord3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glTexCoord3xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed, r: GLfixed);

pub type glTexCoord3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

pub type glTexCoord4bOES_t = unsafe extern "system" fn(s: GLbyte, t: GLbyte, r: GLbyte, q: GLbyte);

pub type glTexCoord4bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 4]);

pub type glTexCoord4d_t = unsafe extern "system" fn(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);

pub type glTexCoord4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

pub type glTexCoord4f_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);

pub type glTexCoord4fColor4fNormal3fVertex4fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, p: GLfloat, q: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glTexCoord4fColor4fNormal3fVertex4fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 4], c: *const [GLfloat; 4], n: *const [GLfloat; 3], v: *const [GLfloat; 4]);

pub type glTexCoord4fVertex4fSUN_t = unsafe extern "system" fn(s: GLfloat, t: GLfloat, p: GLfloat, q: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glTexCoord4fVertex4fvSUN_t = unsafe extern "system" fn(tc: *const [GLfloat; 4], v: *const [GLfloat; 4]);

pub type glTexCoord4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

pub type glTexCoord4hNV_t = unsafe extern "system" fn(s: GLhalfNV, t: GLhalfNV, r: GLhalfNV, q: GLhalfNV);

pub type glTexCoord4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

pub type glTexCoord4i_t = unsafe extern "system" fn(s: GLint, t: GLint, r: GLint, q: GLint);

pub type glTexCoord4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

pub type glTexCoord4s_t = unsafe extern "system" fn(s: GLshort, t: GLshort, r: GLshort, q: GLshort);

pub type glTexCoord4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

pub type glTexCoord4xOES_t = unsafe extern "system" fn(s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);

pub type glTexCoord4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

pub type glTexCoordFormatNV_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei);

pub type glTexCoordP1ui_t = unsafe extern "system" fn(type_: GLenum, coords: GLuint);

pub type glTexCoordP1uiv_t = unsafe extern "system" fn(type_: GLenum, coords: *const GLuint);

pub type glTexCoordP2ui_t = unsafe extern "system" fn(type_: GLenum, coords: GLuint);

pub type glTexCoordP2uiv_t = unsafe extern "system" fn(type_: GLenum, coords: *const GLuint);

pub type glTexCoordP3ui_t = unsafe extern "system" fn(type_: GLenum, coords: GLuint);

pub type glTexCoordP3uiv_t = unsafe extern "system" fn(type_: GLenum, coords: *const GLuint);

pub type glTexCoordP4ui_t = unsafe extern "system" fn(type_: GLenum, coords: GLuint);

pub type glTexCoordP4uiv_t = unsafe extern "system" fn(type_: GLenum, coords: *const GLuint);

pub type glTexCoordPointer_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glTexCoordPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const void);

pub type glTexCoordPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

pub type glTexCoordPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: GLenum, pointer: *const [*mut void; 4]);

pub type glTexEnvf_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat);

pub type glTexEnvfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glTexEnvi_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

pub type glTexEnviv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glTexEnvx_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfixed);

pub type glTexEnvxOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfixed);

pub type glTexEnvxv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfixed);

pub type glTexEnvxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfixed);

pub type glTexEstimateMotionQCOM_t = unsafe extern "system" fn(ref_: GLuint, target: GLuint, output: GLuint);

pub type glTexEstimateMotionRegionsQCOM_t = unsafe extern "system" fn(ref_: GLuint, target: GLuint, output: GLuint, mask: GLuint);

pub type glTexFilterFuncSGIS_t = unsafe extern "system" fn(target: GLenum, filter: GLenum, n: GLsizei, weights: *const GLfloat);

pub type glTexGend_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, param: GLdouble);

pub type glTexGendv_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *const GLdouble);

pub type glTexGenf_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, param: GLfloat);

pub type glTexGenfOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, param: GLfloat);

pub type glTexGenfv_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *const GLfloat);

pub type glTexGenfvOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *const GLfloat);

pub type glTexGeni_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, param: GLint);

pub type glTexGeniOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, param: GLint);

pub type glTexGeniv_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *const GLint);

pub type glTexGenivOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *const GLint);

pub type glTexGenxOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, param: GLfixed);

pub type glTexGenxvOES_t = unsafe extern "system" fn(coord: GLenum, pname: GLenum, params: *const GLfixed);

pub type glTexImage1D_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexImage2D_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexImage2DMultisample_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

pub type glTexImage2DMultisampleCoverageNV_t = unsafe extern "system" fn(target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

pub type glTexImage3D_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexImage3DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexImage3DMultisample_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub type glTexImage3DMultisampleCoverageNV_t = unsafe extern "system" fn(target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

pub type glTexImage3DOES_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexImage4DSGIS_t = unsafe extern "system" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, size4d: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexPageCommitmentARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

pub type glTexPageCommitmentEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

pub type glTexPageCommitmentMemNV_t = unsafe extern "system" fn(target: GLenum, layer: GLint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64, commit: GLboolean);

pub type glTexParameterIiv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glTexParameterIivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glTexParameterIivOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glTexParameterIuiv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLuint);

pub type glTexParameterIuivEXT_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLuint);

pub type glTexParameterIuivOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLuint);

pub type glTexParameterf_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat);

pub type glTexParameterfv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glTexParameteri_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);

pub type glTexParameteriv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);

pub type glTexParameterx_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfixed);

pub type glTexParameterxOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfixed);

pub type glTexParameterxv_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfixed);

pub type glTexParameterxvOES_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfixed);

pub type glTexRenderbufferNV_t = unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);

pub type glTexStorage1D_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);

pub type glTexStorage1DEXT_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);

pub type glTexStorage2D_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glTexStorage2DEXT_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glTexStorage2DMultisample_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

pub type glTexStorage3D_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);

pub type glTexStorage3DEXT_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);

pub type glTexStorage3DMultisample_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub type glTexStorage3DMultisampleOES_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub type glTexStorageMem1DEXT_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalFormat: GLenum, width: GLsizei, memory: GLuint, offset: GLuint64);

pub type glTexStorageMem2DEXT_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, memory: GLuint, offset: GLuint64);

pub type glTexStorageMem2DMultisampleEXT_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

pub type glTexStorageMem3DEXT_t = unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64);

pub type glTexStorageMem3DMultisampleEXT_t = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

pub type glTexStorageSparseAMD_t = unsafe extern "system" fn(target: GLenum, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, layers: GLsizei, flags: GLbitfield);

pub type glTexSubImage1D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexSubImage1DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexSubImage2D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexSubImage2DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexSubImage3D_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexSubImage3DEXT_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexSubImage3DOES_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTexSubImage4DSGIS_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, woffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, size4d: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureAttachMemoryNV_t = unsafe extern "system" fn(texture: GLuint, memory: GLuint, offset: GLuint64);

pub type glTextureBarrier_t = unsafe extern "system" fn();

pub type glTextureBarrierNV_t = unsafe extern "system" fn();

pub type glTextureBuffer_t = unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint);

pub type glTextureBufferEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, internalformat: GLenum, buffer: GLuint);

pub type glTextureBufferRange_t = unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glTextureBufferRangeEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glTextureColorMaskSGIS_t = unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

pub type glTextureFoveationParametersQCOM_t = unsafe extern "system" fn(texture: GLuint, layer: GLuint, focalPoint: GLuint, focalX: GLfloat, focalY: GLfloat, gainX: GLfloat, gainY: GLfloat, foveaArea: GLfloat);

pub type glTextureImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureImage2DMultisampleCoverageNV_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

pub type glTextureImage2DMultisampleNV_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, samples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);

pub type glTextureImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureImage3DMultisampleCoverageNV_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

pub type glTextureImage3DMultisampleNV_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, samples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);

pub type glTextureLightEXT_t = unsafe extern "system" fn(pname: GLenum);

pub type glTextureMaterialEXT_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);

pub type glTextureNormalEXT_t = unsafe extern "system" fn(mode: GLenum);

pub type glTexturePageCommitmentEXT_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

pub type glTexturePageCommitmentMemNV_t = unsafe extern "system" fn(texture: GLuint, layer: GLint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64, commit: GLboolean);

pub type glTextureParameterIiv_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLint);

pub type glTextureParameterIivEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, params: *const GLint);

pub type glTextureParameterIuiv_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLuint);

pub type glTextureParameterIuivEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, params: *const GLuint);

pub type glTextureParameterf_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLfloat);

pub type glTextureParameterfEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, param: GLfloat);

pub type glTextureParameterfv_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLfloat);

pub type glTextureParameterfvEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, params: *const GLfloat);

pub type glTextureParameteri_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLint);

pub type glTextureParameteriEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, param: GLint);

pub type glTextureParameteriv_t = unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLint);

pub type glTextureParameterivEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, pname: GLenum, params: *const GLint);

pub type glTextureRangeAPPLE_t = unsafe extern "system" fn(target: GLenum, length: GLsizei, pointer: *const void);

pub type glTextureRenderbufferEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, renderbuffer: GLuint);

pub type glTextureStorage1D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei);

pub type glTextureStorage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);

pub type glTextureStorage2D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glTextureStorage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub type glTextureStorage2DMultisample_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

pub type glTextureStorage2DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

pub type glTextureStorage3D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);

pub type glTextureStorage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);

pub type glTextureStorage3DMultisample_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub type glTextureStorage3DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub type glTextureStorageMem1DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, memory: GLuint, offset: GLuint64);

pub type glTextureStorageMem2DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, memory: GLuint, offset: GLuint64);

pub type glTextureStorageMem2DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

pub type glTextureStorageMem3DEXT_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64);

pub type glTextureStorageMem3DMultisampleEXT_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);

pub type glTextureStorageSparseAMD_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, layers: GLsizei, flags: GLbitfield);

pub type glTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureSubImage1DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureSubImage2DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureSubImage3DEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const void);

pub type glTextureView_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

pub type glTextureViewEXT_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

pub type glTextureViewOES_t = unsafe extern "system" fn(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

pub type glTrackMatrixNV_t = unsafe extern "system" fn(target: GLenum, address: GLuint, matrix: GLenum, transform: GLenum);

pub type glTransformFeedbackAttribsNV_t = unsafe extern "system" fn(count: GLsizei, attribs: *const GLint, bufferMode: GLenum);

pub type glTransformFeedbackBufferBase_t = unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint);

pub type glTransformFeedbackBufferRange_t = unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub type glTransformFeedbackStreamAttribsNV_t = unsafe extern "system" fn(count: GLsizei, attribs: *const GLint, nbuffers: GLsizei, bufstreams: *const GLint, bufferMode: GLenum);

pub type glTransformFeedbackVaryings_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);

pub type glTransformFeedbackVaryingsEXT_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);

pub type glTransformFeedbackVaryingsNV_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, locations: *const GLint, bufferMode: GLenum);

pub type glTransformPathNV_t = unsafe extern "system" fn(resultPath: GLuint, srcPath: GLuint, transformType: GLenum, transformValues: *const GLfloat);

pub type glTranslated_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

pub type glTranslatef_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

pub type glTranslatex_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

pub type glTranslatexOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

pub type glUniform1d_t = unsafe extern "system" fn(location: GLint, x: GLdouble);

pub type glUniform1dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

pub type glUniform1f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);

pub type glUniform1fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);

pub type glUniform1fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub type glUniform1fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub type glUniform1i_t = unsafe extern "system" fn(location: GLint, v0: GLint);

pub type glUniform1i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64);

pub type glUniform1i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT);

pub type glUniform1i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

pub type glUniform1i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

pub type glUniform1iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint);

pub type glUniform1iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub type glUniform1ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub type glUniform1ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint);

pub type glUniform1ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64);

pub type glUniform1ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT);

pub type glUniform1ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

pub type glUniform1ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glUniform1uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint);

pub type glUniform1uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub type glUniform1uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub type glUniform2d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble);

pub type glUniform2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

pub type glUniform2f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);

pub type glUniform2fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);

pub type glUniform2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub type glUniform2fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub type glUniform2i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);

pub type glUniform2i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64);

pub type glUniform2i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT);

pub type glUniform2i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

pub type glUniform2i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

pub type glUniform2iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);

pub type glUniform2iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub type glUniform2ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub type glUniform2ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);

pub type glUniform2ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64);

pub type glUniform2ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT);

pub type glUniform2ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

pub type glUniform2ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glUniform2uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);

pub type glUniform2uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub type glUniform2uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub type glUniform3d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glUniform3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

pub type glUniform3f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

pub type glUniform3fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

pub type glUniform3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub type glUniform3fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub type glUniform3i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);

pub type glUniform3i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64, z: GLint64);

pub type glUniform3i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

pub type glUniform3i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

pub type glUniform3i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

pub type glUniform3iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);

pub type glUniform3iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub type glUniform3ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub type glUniform3ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

pub type glUniform3ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);

pub type glUniform3ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

pub type glUniform3ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

pub type glUniform3ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glUniform3uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

pub type glUniform3uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub type glUniform3uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub type glUniform4d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glUniform4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

pub type glUniform4f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

pub type glUniform4fARB_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

pub type glUniform4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub type glUniform4fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub type glUniform4i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

pub type glUniform4i64ARB_t = unsafe extern "system" fn(location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);

pub type glUniform4i64NV_t = unsafe extern "system" fn(location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

pub type glUniform4i64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64);

pub type glUniform4i64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint64EXT);

pub type glUniform4iARB_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

pub type glUniform4iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub type glUniform4ivARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub type glUniform4ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

pub type glUniform4ui64ARB_t = unsafe extern "system" fn(location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);

pub type glUniform4ui64NV_t = unsafe extern "system" fn(location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

pub type glUniform4ui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

pub type glUniform4ui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glUniform4uiEXT_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

pub type glUniform4uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub type glUniform4uivEXT_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub type glUniformBlockBinding_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);

pub type glUniformBufferEXT_t = unsafe extern "system" fn(program: GLuint, location: GLint, buffer: GLuint);

pub type glUniformHandleui64ARB_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

pub type glUniformHandleui64IMG_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

pub type glUniformHandleui64NV_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

pub type glUniformHandleui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

pub type glUniformHandleui64vIMG_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

pub type glUniformHandleui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

pub type glUniformMatrix2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix2fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix2x3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix2x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix2x3fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix2x4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix2x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix2x4fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix3fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix3x2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix3x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix3x2fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix3x4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix3x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix3x4fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix4fvARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix4x2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix4x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix4x2fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix4x3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub type glUniformMatrix4x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformMatrix4x3fvNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub type glUniformSubroutinesuiv_t = unsafe extern "system" fn(shadertype: GLenum, count: GLsizei, indices: *const GLuint);

pub type glUniformui64NV_t = unsafe extern "system" fn(location: GLint, value: GLuint64EXT);

pub type glUniformui64vNV_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64EXT);

pub type glUnlockArraysEXT_t = unsafe extern "system" fn();

pub type glUnmapBuffer_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;

pub type glUnmapBufferARB_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;

pub type glUnmapBufferOES_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;

pub type glUnmapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub type glUnmapNamedBufferEXT_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub type glUnmapObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint);

pub type glUnmapTexture2DINTEL_t = unsafe extern "system" fn(texture: GLuint, level: GLint);

pub type glUpdateObjectBufferATI_t = unsafe extern "system" fn(buffer: GLuint, offset: GLuint, size: GLsizei, pointer: *const void, preserve: GLenum);

pub type glUploadGpuMaskNVX_t = unsafe extern "system" fn(mask: GLbitfield);

pub type glUseProgram_t = unsafe extern "system" fn(program: GLuint);

pub type glUseProgramObjectARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

pub type glUseProgramStages_t = unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);

pub type glUseProgramStagesEXT_t = unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);

pub type glUseShaderProgramEXT_t = unsafe extern "system" fn(type_: GLenum, program: GLuint);

pub type glVDPAUFiniNV_t = unsafe extern "system" fn();

pub type glVDPAUGetSurfaceivNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

pub type glVDPAUInitNV_t = unsafe extern "system" fn(vdpDevice: *const void, getProcAddress: *const void);

pub type glVDPAUIsSurfaceNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV) -> GLboolean;

pub type glVDPAUMapSurfacesNV_t = unsafe extern "system" fn(numSurfaces: GLsizei, surfaces: *const GLvdpauSurfaceNV);

pub type glVDPAURegisterOutputSurfaceNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint) -> GLvdpauSurfaceNV;

pub type glVDPAURegisterVideoSurfaceNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint) -> GLvdpauSurfaceNV;

pub type glVDPAURegisterVideoSurfaceWithPictureStructureNV_t = unsafe extern "system" fn(vdpSurface: *const void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint, isFrameStructure: GLboolean) -> GLvdpauSurfaceNV;

pub type glVDPAUSurfaceAccessNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV, access: GLenum);

pub type glVDPAUUnmapSurfacesNV_t = unsafe extern "system" fn(numSurface: GLsizei, surfaces: *const GLvdpauSurfaceNV);

pub type glVDPAUUnregisterSurfaceNV_t = unsafe extern "system" fn(surface: GLvdpauSurfaceNV);

pub type glValidateProgram_t = unsafe extern "system" fn(program: GLuint);

pub type glValidateProgramARB_t = unsafe extern "system" fn(programObj: GLhandleARB);

pub type glValidateProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);

pub type glValidateProgramPipelineEXT_t = unsafe extern "system" fn(pipeline: GLuint);

pub type glVariantArrayObjectATI_t = unsafe extern "system" fn(id: GLuint, type_: GLenum, stride: GLsizei, buffer: GLuint, offset: GLuint);

pub type glVariantPointerEXT_t = unsafe extern "system" fn(id: GLuint, type_: GLenum, stride: GLuint, addr: *const void);

pub type glVariantbvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLbyte);

pub type glVariantdvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLdouble);

pub type glVariantfvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLfloat);

pub type glVariantivEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLint);

pub type glVariantsvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLshort);

pub type glVariantubvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLubyte);

pub type glVariantuivEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLuint);

pub type glVariantusvEXT_t = unsafe extern "system" fn(id: GLuint, addr: *const GLushort);

pub type glVertex2bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte);

pub type glVertex2bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 2]);

pub type glVertex2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

pub type glVertex2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

pub type glVertex2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

pub type glVertex2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

pub type glVertex2hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV);

pub type glVertex2hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 2]);

pub type glVertex2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

pub type glVertex2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

pub type glVertex2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

pub type glVertex2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

pub type glVertex2xOES_t = unsafe extern "system" fn(x: GLfixed);

pub type glVertex2xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 2]);

pub type glVertex3bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte, z: GLbyte);

pub type glVertex3bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 3]);

pub type glVertex3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

pub type glVertex3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glVertex3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

pub type glVertex3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glVertex3hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV, z: GLhalfNV);

pub type glVertex3hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 3]);

pub type glVertex3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

pub type glVertex3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glVertex3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

pub type glVertex3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glVertex3xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed);

pub type glVertex3xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 3]);

pub type glVertex4bOES_t = unsafe extern "system" fn(x: GLbyte, y: GLbyte, z: GLbyte, w: GLbyte);

pub type glVertex4bvOES_t = unsafe extern "system" fn(coords: *const [GLbyte; 4]);

pub type glVertex4d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glVertex4dv_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

pub type glVertex4f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glVertex4fv_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

pub type glVertex4hNV_t = unsafe extern "system" fn(x: GLhalfNV, y: GLhalfNV, z: GLhalfNV, w: GLhalfNV);

pub type glVertex4hvNV_t = unsafe extern "system" fn(v: *const [GLhalfNV; 4]);

pub type glVertex4i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

pub type glVertex4iv_t = unsafe extern "system" fn(v: *const [GLint; 4]);

pub type glVertex4s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub type glVertex4sv_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

pub type glVertex4xOES_t = unsafe extern "system" fn(x: GLfixed, y: GLfixed, z: GLfixed);

pub type glVertex4xvOES_t = unsafe extern "system" fn(coords: *const [GLfixed; 4]);

pub type glVertexArrayAttribBinding_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);

pub type glVertexArrayAttribFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);

pub type glVertexArrayAttribIFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);

pub type glVertexArrayAttribLFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);

pub type glVertexArrayBindVertexBufferEXT_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

pub type glVertexArrayBindingDivisor_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);

pub type glVertexArrayColorOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayEdgeFlagOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayElementBuffer_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint);

pub type glVertexArrayFogCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayIndexOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayMultiTexCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, texunit: GLenum, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayNormalOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayParameteriAPPLE_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glVertexArrayRangeAPPLE_t = unsafe extern "system" fn(length: GLsizei, pointer: *mut void);

pub type glVertexArrayRangeNV_t = unsafe extern "system" fn(length: GLsizei, pointer: *const void);

pub type glVertexArraySecondaryColorOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayTexCoordOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayVertexAttribBindingEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);

pub type glVertexArrayVertexAttribDivisorEXT_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, divisor: GLuint);

pub type glVertexArrayVertexAttribFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);

pub type glVertexArrayVertexAttribIFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);

pub type glVertexArrayVertexAttribIOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayVertexAttribLFormatEXT_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);

pub type glVertexArrayVertexAttribLOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayVertexAttribOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, offset: GLintptr);

pub type glVertexArrayVertexBindingDivisorEXT_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);

pub type glVertexArrayVertexBuffer_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

pub type glVertexArrayVertexBuffers_t = unsafe extern "system" fn(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);

pub type glVertexArrayVertexOffsetEXT_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);

pub type glVertexAttrib1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

pub type glVertexAttrib1dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

pub type glVertexAttrib1dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

pub type glVertexAttrib1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

pub type glVertexAttrib1dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

pub type glVertexAttrib1dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

pub type glVertexAttrib1f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

pub type glVertexAttrib1fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

pub type glVertexAttrib1fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

pub type glVertexAttrib1fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

pub type glVertexAttrib1fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

pub type glVertexAttrib1fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

pub type glVertexAttrib1hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV);

pub type glVertexAttrib1hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLhalfNV);

pub type glVertexAttrib1s_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

pub type glVertexAttrib1sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

pub type glVertexAttrib1sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

pub type glVertexAttrib1sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

pub type glVertexAttrib1svARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

pub type glVertexAttrib1svNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

pub type glVertexAttrib2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

pub type glVertexAttrib2dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

pub type glVertexAttrib2dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

pub type glVertexAttrib2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

pub type glVertexAttrib2dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

pub type glVertexAttrib2dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

pub type glVertexAttrib2f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

pub type glVertexAttrib2fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

pub type glVertexAttrib2fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

pub type glVertexAttrib2fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

pub type glVertexAttrib2fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

pub type glVertexAttrib2fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

pub type glVertexAttrib2hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV);

pub type glVertexAttrib2hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 2]);

pub type glVertexAttrib2s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

pub type glVertexAttrib2sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

pub type glVertexAttrib2sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

pub type glVertexAttrib2sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

pub type glVertexAttrib2svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

pub type glVertexAttrib2svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

pub type glVertexAttrib3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glVertexAttrib3dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glVertexAttrib3dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glVertexAttrib3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

pub type glVertexAttrib3dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

pub type glVertexAttrib3dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

pub type glVertexAttrib3f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glVertexAttrib3fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glVertexAttrib3fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glVertexAttrib3fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

pub type glVertexAttrib3fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

pub type glVertexAttrib3fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

pub type glVertexAttrib3hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV, z: GLhalfNV);

pub type glVertexAttrib3hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 3]);

pub type glVertexAttrib3s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

pub type glVertexAttrib3sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

pub type glVertexAttrib3sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

pub type glVertexAttrib3sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

pub type glVertexAttrib3svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

pub type glVertexAttrib3svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

pub type glVertexAttrib4Nbv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub type glVertexAttrib4NbvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub type glVertexAttrib4Niv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glVertexAttrib4NivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glVertexAttrib4Nsv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub type glVertexAttrib4NsvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub type glVertexAttrib4Nub_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

pub type glVertexAttrib4NubARB_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

pub type glVertexAttrib4Nubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub type glVertexAttrib4NubvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub type glVertexAttrib4Nuiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub type glVertexAttrib4NuivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub type glVertexAttrib4Nusv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub type glVertexAttrib4NusvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub type glVertexAttrib4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub type glVertexAttrib4bvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub type glVertexAttrib4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glVertexAttrib4dARB_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glVertexAttrib4dNV_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glVertexAttrib4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

pub type glVertexAttrib4dvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

pub type glVertexAttrib4dvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

pub type glVertexAttrib4f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glVertexAttrib4fARB_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glVertexAttrib4fNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glVertexAttrib4fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub type glVertexAttrib4fvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub type glVertexAttrib4fvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub type glVertexAttrib4hNV_t = unsafe extern "system" fn(index: GLuint, x: GLhalfNV, y: GLhalfNV, z: GLhalfNV, w: GLhalfNV);

pub type glVertexAttrib4hvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLhalfNV; 4]);

pub type glVertexAttrib4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glVertexAttrib4ivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glVertexAttrib4s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub type glVertexAttrib4sARB_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub type glVertexAttrib4sNV_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub type glVertexAttrib4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub type glVertexAttrib4svARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub type glVertexAttrib4svNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub type glVertexAttrib4ubNV_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

pub type glVertexAttrib4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub type glVertexAttrib4ubvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub type glVertexAttrib4ubvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub type glVertexAttrib4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub type glVertexAttrib4uivARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub type glVertexAttrib4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub type glVertexAttrib4usvARB_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub type glVertexAttribArrayObjectATI_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, buffer: GLuint, offset: GLuint);

pub type glVertexAttribBinding_t = unsafe extern "system" fn(attribindex: GLuint, bindingindex: GLuint);

pub type glVertexAttribDivisor_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

pub type glVertexAttribDivisorANGLE_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

pub type glVertexAttribDivisorARB_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

pub type glVertexAttribDivisorEXT_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

pub type glVertexAttribDivisorNV_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

pub type glVertexAttribFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);

pub type glVertexAttribFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei);

pub type glVertexAttribI1i_t = unsafe extern "system" fn(index: GLuint, x: GLint);

pub type glVertexAttribI1iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint);

pub type glVertexAttribI1iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);

pub type glVertexAttribI1ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);

pub type glVertexAttribI1ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint);

pub type glVertexAttribI1uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint);

pub type glVertexAttribI1uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);

pub type glVertexAttribI1uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);

pub type glVertexAttribI2i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);

pub type glVertexAttribI2iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);

pub type glVertexAttribI2iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 2]);

pub type glVertexAttribI2ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 2]);

pub type glVertexAttribI2ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);

pub type glVertexAttribI2uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);

pub type glVertexAttribI2uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 2]);

pub type glVertexAttribI2uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 2]);

pub type glVertexAttribI3i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);

pub type glVertexAttribI3iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);

pub type glVertexAttribI3iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 3]);

pub type glVertexAttribI3ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 3]);

pub type glVertexAttribI3ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);

pub type glVertexAttribI3uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);

pub type glVertexAttribI3uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 3]);

pub type glVertexAttribI3uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 3]);

pub type glVertexAttribI4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub type glVertexAttribI4bvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub type glVertexAttribI4i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

pub type glVertexAttribI4iEXT_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

pub type glVertexAttribI4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glVertexAttribI4ivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub type glVertexAttribI4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub type glVertexAttribI4svEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub type glVertexAttribI4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub type glVertexAttribI4ubvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub type glVertexAttribI4ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

pub type glVertexAttribI4uiEXT_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

pub type glVertexAttribI4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub type glVertexAttribI4uivEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub type glVertexAttribI4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub type glVertexAttribI4usvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub type glVertexAttribIFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);

pub type glVertexAttribIFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei);

pub type glVertexAttribIPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glVertexAttribIPointerEXT_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glVertexAttribL1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

pub type glVertexAttribL1dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

pub type glVertexAttribL1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

pub type glVertexAttribL1dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

pub type glVertexAttribL1i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT);

pub type glVertexAttribL1i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLint64EXT);

pub type glVertexAttribL1ui64ARB_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT);

pub type glVertexAttribL1ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT);

pub type glVertexAttribL1ui64vARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint64EXT);

pub type glVertexAttribL1ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint64EXT);

pub type glVertexAttribL2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

pub type glVertexAttribL2dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

pub type glVertexAttribL2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

pub type glVertexAttribL2dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

pub type glVertexAttribL2i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT);

pub type glVertexAttribL2i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 2]);

pub type glVertexAttribL2ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT);

pub type glVertexAttribL2ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 2]);

pub type glVertexAttribL3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glVertexAttribL3dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glVertexAttribL3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

pub type glVertexAttribL3dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

pub type glVertexAttribL3i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);

pub type glVertexAttribL3i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 3]);

pub type glVertexAttribL3ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);

pub type glVertexAttribL3ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 3]);

pub type glVertexAttribL4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glVertexAttribL4dEXT_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glVertexAttribL4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

pub type glVertexAttribL4dvEXT_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

pub type glVertexAttribL4i64NV_t = unsafe extern "system" fn(index: GLuint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);

pub type glVertexAttribL4i64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint64EXT; 4]);

pub type glVertexAttribL4ui64NV_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);

pub type glVertexAttribL4ui64vNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint64EXT; 4]);

pub type glVertexAttribLFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);

pub type glVertexAttribLFormatNV_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei);

pub type glVertexAttribLPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glVertexAttribLPointerEXT_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glVertexAttribP1ui_t = unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);

pub type glVertexAttribP1uiv_t = unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);

pub type glVertexAttribP2ui_t = unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);

pub type glVertexAttribP2uiv_t = unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);

pub type glVertexAttribP3ui_t = unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);

pub type glVertexAttribP3uiv_t = unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);

pub type glVertexAttribP4ui_t = unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);

pub type glVertexAttribP4uiv_t = unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);

pub type glVertexAttribParameteriAMD_t = unsafe extern "system" fn(index: GLuint, pname: GLenum, param: GLint);

pub type glVertexAttribPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const void);

pub type glVertexAttribPointerARB_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const void);

pub type glVertexAttribPointerNV_t = unsafe extern "system" fn(index: GLuint, fsize: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glVertexAttribs1dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

pub type glVertexAttribs1fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

pub type glVertexAttribs1hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

pub type glVertexAttribs1svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

pub type glVertexAttribs2dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

pub type glVertexAttribs2fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

pub type glVertexAttribs2hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

pub type glVertexAttribs2svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

pub type glVertexAttribs3dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

pub type glVertexAttribs3fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

pub type glVertexAttribs3hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

pub type glVertexAttribs3svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

pub type glVertexAttribs4dvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLdouble);

pub type glVertexAttribs4fvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLfloat);

pub type glVertexAttribs4hvNV_t = unsafe extern "system" fn(index: GLuint, n: GLsizei, v: *const GLhalfNV);

pub type glVertexAttribs4svNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLshort);

pub type glVertexAttribs4ubvNV_t = unsafe extern "system" fn(index: GLuint, count: GLsizei, v: *const GLubyte);

pub type glVertexBindingDivisor_t = unsafe extern "system" fn(bindingindex: GLuint, divisor: GLuint);

pub type glVertexBlendARB_t = unsafe extern "system" fn(count: GLint);

pub type glVertexBlendEnvfATI_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);

pub type glVertexBlendEnviATI_t = unsafe extern "system" fn(pname: GLenum, param: GLint);

pub type glVertexFormatNV_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei);

pub type glVertexP2ui_t = unsafe extern "system" fn(type_: GLenum, value: GLuint);

pub type glVertexP2uiv_t = unsafe extern "system" fn(type_: GLenum, value: *const GLuint);

pub type glVertexP3ui_t = unsafe extern "system" fn(type_: GLenum, value: GLuint);

pub type glVertexP3uiv_t = unsafe extern "system" fn(type_: GLenum, value: *const GLuint);

pub type glVertexP4ui_t = unsafe extern "system" fn(type_: GLenum, value: GLuint);

pub type glVertexP4uiv_t = unsafe extern "system" fn(type_: GLenum, value: *const GLuint);

pub type glVertexPointer_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glVertexPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const void);

pub type glVertexPointerListIBM_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLint, pointer: *const *mut void, ptrstride: GLint);

pub type glVertexPointervINTEL_t = unsafe extern "system" fn(size: GLint, type_: GLenum, pointer: *const [*mut void; 4]);

pub type glVertexStream1dATI_t = unsafe extern "system" fn(stream: GLenum, x: GLdouble);

pub type glVertexStream1dvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const GLdouble);

pub type glVertexStream1fATI_t = unsafe extern "system" fn(stream: GLenum, x: GLfloat);

pub type glVertexStream1fvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const GLfloat);

pub type glVertexStream1iATI_t = unsafe extern "system" fn(stream: GLenum, x: GLint);

pub type glVertexStream1ivATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const GLint);

pub type glVertexStream1sATI_t = unsafe extern "system" fn(stream: GLenum, x: GLshort);

pub type glVertexStream1svATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const GLshort);

pub type glVertexStream2dATI_t = unsafe extern "system" fn(stream: GLenum, x: GLdouble, y: GLdouble);

pub type glVertexStream2dvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLdouble; 2]);

pub type glVertexStream2fATI_t = unsafe extern "system" fn(stream: GLenum, x: GLfloat, y: GLfloat);

pub type glVertexStream2fvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLfloat; 2]);

pub type glVertexStream2iATI_t = unsafe extern "system" fn(stream: GLenum, x: GLint, y: GLint);

pub type glVertexStream2ivATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLint; 2]);

pub type glVertexStream2sATI_t = unsafe extern "system" fn(stream: GLenum, x: GLshort, y: GLshort);

pub type glVertexStream2svATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLshort; 2]);

pub type glVertexStream3dATI_t = unsafe extern "system" fn(stream: GLenum, x: GLdouble, y: GLdouble, z: GLdouble);

pub type glVertexStream3dvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLdouble; 3]);

pub type glVertexStream3fATI_t = unsafe extern "system" fn(stream: GLenum, x: GLfloat, y: GLfloat, z: GLfloat);

pub type glVertexStream3fvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLfloat; 3]);

pub type glVertexStream3iATI_t = unsafe extern "system" fn(stream: GLenum, x: GLint, y: GLint, z: GLint);

pub type glVertexStream3ivATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLint; 3]);

pub type glVertexStream3sATI_t = unsafe extern "system" fn(stream: GLenum, x: GLshort, y: GLshort, z: GLshort);

pub type glVertexStream3svATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLshort; 3]);

pub type glVertexStream4dATI_t = unsafe extern "system" fn(stream: GLenum, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glVertexStream4dvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLdouble; 4]);

pub type glVertexStream4fATI_t = unsafe extern "system" fn(stream: GLenum, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glVertexStream4fvATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLfloat; 4]);

pub type glVertexStream4iATI_t = unsafe extern "system" fn(stream: GLenum, x: GLint, y: GLint, z: GLint, w: GLint);

pub type glVertexStream4ivATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLint; 4]);

pub type glVertexStream4sATI_t = unsafe extern "system" fn(stream: GLenum, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub type glVertexStream4svATI_t = unsafe extern "system" fn(stream: GLenum, coords: *const [GLshort; 4]);

pub type glVertexWeightPointerEXT_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glVertexWeightfEXT_t = unsafe extern "system" fn(weight: GLfloat);

pub type glVertexWeightfvEXT_t = unsafe extern "system" fn(weight: *const GLfloat);

pub type glVertexWeighthNV_t = unsafe extern "system" fn(weight: GLhalfNV);

pub type glVertexWeighthvNV_t = unsafe extern "system" fn(weight: *const GLhalfNV);

pub type glVideoCaptureNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, sequence_num: *mut GLuint, capture_time: *mut GLuint64EXT) -> GLenum;

pub type glVideoCaptureStreamParameterdvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLdouble);

pub type glVideoCaptureStreamParameterfvNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLfloat);

pub type glVideoCaptureStreamParameterivNV_t = unsafe extern "system" fn(video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLint);

pub type glViewport_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type glViewportArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

pub type glViewportArrayvNV_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

pub type glViewportArrayvOES_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

pub type glViewportIndexedf_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

pub type glViewportIndexedfNV_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

pub type glViewportIndexedfOES_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

pub type glViewportIndexedfv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub type glViewportIndexedfvNV_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub type glViewportIndexedfvOES_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub type glViewportPositionWScaleNV_t = unsafe extern "system" fn(index: GLuint, xcoeff: GLfloat, ycoeff: GLfloat);

pub type glViewportSwizzleNV_t = unsafe extern "system" fn(index: GLuint, swizzlex: GLenum, swizzley: GLenum, swizzlez: GLenum, swizzlew: GLenum);

pub type glWaitSemaphoreEXT_t = unsafe extern "system" fn(semaphore: GLuint, numBufferBarriers: GLuint, buffers: *const GLuint, numTextureBarriers: GLuint, textures: *const GLuint, srcLayouts: *const GLenum);

pub type glWaitSemaphoreui64NVX_t = unsafe extern "system" fn(waitGpu: GLuint, fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);

pub type glWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

pub type glWaitSyncAPPLE_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

pub type glWaitVkSemaphoreNV_t = unsafe extern "system" fn(vkSemaphore: GLuint64);

pub type glWeightPathsNV_t = unsafe extern "system" fn(resultPath: GLuint, numPaths: GLsizei, paths: *const GLuint, weights: *const GLfloat);

pub type glWeightPointerARB_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glWeightPointerOES_t = unsafe extern "system" fn(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const void);

pub type glWeightbvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLbyte);

pub type glWeightdvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLdouble);

pub type glWeightfvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLfloat);

pub type glWeightivARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLint);

pub type glWeightsvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLshort);

pub type glWeightubvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLubyte);

pub type glWeightuivARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLuint);

pub type glWeightusvARB_t = unsafe extern "system" fn(size: GLint, weights: *const GLushort);

pub type glWindowPos2d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

pub type glWindowPos2dARB_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

pub type glWindowPos2dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble);

pub type glWindowPos2dv_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

pub type glWindowPos2dvARB_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

pub type glWindowPos2dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 2]);

pub type glWindowPos2f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

pub type glWindowPos2fARB_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

pub type glWindowPos2fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat);

pub type glWindowPos2fv_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

pub type glWindowPos2fvARB_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

pub type glWindowPos2fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 2]);

pub type glWindowPos2i_t = unsafe extern "system" fn(x: GLint, y: GLint);

pub type glWindowPos2iARB_t = unsafe extern "system" fn(x: GLint, y: GLint);

pub type glWindowPos2iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint);

pub type glWindowPos2iv_t = unsafe extern "system" fn(v: *const [GLint; 2]);

pub type glWindowPos2ivARB_t = unsafe extern "system" fn(v: *const [GLint; 2]);

pub type glWindowPos2ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 2]);

pub type glWindowPos2s_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

pub type glWindowPos2sARB_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

pub type glWindowPos2sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort);

pub type glWindowPos2sv_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

pub type glWindowPos2svARB_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

pub type glWindowPos2svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 2]);

pub type glWindowPos3d_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

pub type glWindowPos3dARB_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

pub type glWindowPos3dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble);

pub type glWindowPos3dv_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glWindowPos3dvARB_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glWindowPos3dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 3]);

pub type glWindowPos3f_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

pub type glWindowPos3fARB_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

pub type glWindowPos3fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat);

pub type glWindowPos3fv_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glWindowPos3fvARB_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glWindowPos3fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 3]);

pub type glWindowPos3i_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

pub type glWindowPos3iARB_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

pub type glWindowPos3iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint);

pub type glWindowPos3iv_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glWindowPos3ivARB_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glWindowPos3ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 3]);

pub type glWindowPos3s_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

pub type glWindowPos3sARB_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

pub type glWindowPos3sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort);

pub type glWindowPos3sv_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glWindowPos3svARB_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glWindowPos3svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 3]);

pub type glWindowPos4dMESA_t = unsafe extern "system" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub type glWindowPos4dvMESA_t = unsafe extern "system" fn(v: *const [GLdouble; 4]);

pub type glWindowPos4fMESA_t = unsafe extern "system" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub type glWindowPos4fvMESA_t = unsafe extern "system" fn(v: *const [GLfloat; 4]);

pub type glWindowPos4iMESA_t = unsafe extern "system" fn(x: GLint, y: GLint, z: GLint, w: GLint);

pub type glWindowPos4ivMESA_t = unsafe extern "system" fn(v: *const [GLint; 4]);

pub type glWindowPos4sMESA_t = unsafe extern "system" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub type glWindowPos4svMESA_t = unsafe extern "system" fn(v: *const [GLshort; 4]);

pub type glWindowRectanglesEXT_t = unsafe extern "system" fn(mode: GLenum, count: GLsizei, box_: *const GLint);

pub type glWriteMaskEXT_t = unsafe extern "system" fn(res: GLuint, in_: GLuint, outX: GLenum, outY: GLenum, outZ: GLenum, outW: GLenum);
