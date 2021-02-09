use super::*;

#[repr(transparent)]
struct GlFnCell<T>(core::cell::UnsafeCell<Option<T>>);

// Note(Lokathor): _p for ptr, _t for type

// Note(Lokathor): This is a **lie**, and `GlFnCell` MUST remain private to this
// module for safety to be upheld.
unsafe impl<T> Sync for GlFnCell<T> {}

fn gl_ptr_filter(p: *const c_void) -> Option<core::ptr::NonNull<c_void>> {
  match p as usize {
    // Note(Lokathor): wgl is known to sometimes give phony non-null pointer values.
    0 | 1 | 2 | 3 | usize::MAX => None,
    _ => unsafe { core::mem::transmute(p) },
  }
}

#[cold]
#[inline(never)]
#[cfg_attr(feature = "track_caller", track_caller)]
fn gl_not_loaded(name: &str) -> ! {
  panic!("GL function not loaded: {name}", name = name);
}

/// Loads function pointer for global-style GL usage.
///
/// Note: This function is effectively just a shortcut for calling each
/// individual function pointer loader function individually. The individual
/// function pointer loaders are named `{cmd}_load_with`, though they are all
/// `doc(hidden)` to avoid polluting the module docs.
///
/// ## Safety
/// * This action alters non-atomic global memory (`static` UnsafeCell values),
///   and it is not synchronized. If your program is using GL in a
///   multi-threaded way you **must** provide external synchronization of your
///   own. The best practice is to initialize global GL *before* spawning any
///   other threads.
/// * This function is only safe to use if all GL contexts in your program will
///   be able to share the same set of function pointers. Here's some guidance,
///   depending on platform:
///   * Windows: In practice, if two GL contexts come from the same vendor, and refer to the same GPU, and are for the same pixel format, then they will use identical functions.
///     See Also [the OpenGL wiki](https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions#Windows).
///   * Mac: While GL is increasingly less supported with each passing release
///     of Mac, starting in 10.2 all GL symbols are weak-linked. This means,
///     implicitly, that a single set of symbols is valid for your whole
///     program, regardless of a particular context. You must still only call
///     functions that are valid for your current context's API version and
///     available extensions.
///   * Linux: On Linux the GL functions are not context specific. You must
///     still only call functions that are valid for your current context's API
///     version and available extensions.
pub unsafe fn load_global_gl(f: &dyn Fn(*const u8) -> *const c_void) {
  glActiveShaderProgram_load_with(f);
  glActiveTexture_load_with(f);
  glAttachShader_load_with(f);
  glBeginConditionalRender_load_with(f);
  glBeginQuery_load_with(f);
  glBeginQueryIndexed_load_with(f);
  glBeginTransformFeedback_load_with(f);
  glBindAttribLocation_load_with(f);
  glBindBuffer_load_with(f);
  glBindBufferBase_load_with(f);
  glBindBufferRange_load_with(f);
  glBindBuffersBase_load_with(f);
  glBindBuffersRange_load_with(f);
  glBindFragDataLocation_load_with(f);
  glBindFragDataLocationIndexed_load_with(f);
  glBindFramebuffer_load_with(f);
  glBindImageTexture_load_with(f);
  glBindImageTextures_load_with(f);
  glBindProgramPipeline_load_with(f);
  glBindRenderbuffer_load_with(f);
  glBindSampler_load_with(f);
  glBindSamplers_load_with(f);
  glBindTexture_load_with(f);
  glBindTextureUnit_load_with(f);
  glBindTextures_load_with(f);
  glBindTransformFeedback_load_with(f);
  glBindVertexArray_load_with(f);
  glBindVertexBuffer_load_with(f);
  glBindVertexBuffers_load_with(f);
  glBlendColor_load_with(f);
  glBlendEquation_load_with(f);
  glBlendEquationSeparate_load_with(f);
  glBlendEquationSeparatei_load_with(f);
  glBlendEquationi_load_with(f);
  glBlendFunc_load_with(f);
  glBlendFuncSeparate_load_with(f);
  glBlendFuncSeparatei_load_with(f);
  glBlendFunci_load_with(f);
  glBlitFramebuffer_load_with(f);
  glBlitNamedFramebuffer_load_with(f);
  glBufferData_load_with(f);
  glBufferStorage_load_with(f);
  glBufferSubData_load_with(f);
  glCheckFramebufferStatus_load_with(f);
  glCheckNamedFramebufferStatus_load_with(f);
  glClampColor_load_with(f);
  glClear_load_with(f);
  glClearBufferData_load_with(f);
  glClearBufferSubData_load_with(f);
  glClearBufferfi_load_with(f);
  glClearBufferfv_load_with(f);
  glClearBufferiv_load_with(f);
  glClearBufferuiv_load_with(f);
  glClearColor_load_with(f);
  glClearDepth_load_with(f);
  glClearDepthf_load_with(f);
  glClearNamedBufferData_load_with(f);
  glClearNamedBufferSubData_load_with(f);
  glClearNamedFramebufferfi_load_with(f);
  glClearNamedFramebufferfv_load_with(f);
  glClearNamedFramebufferiv_load_with(f);
  glClearNamedFramebufferuiv_load_with(f);
  glClearStencil_load_with(f);
  glClearTexImage_load_with(f);
  glClearTexSubImage_load_with(f);
  glClientWaitSync_load_with(f);
  glClipControl_load_with(f);
  glColorMask_load_with(f);
  glColorMaski_load_with(f);
  glColorP3ui_load_with(f);
  glColorP3uiv_load_with(f);
  glColorP4ui_load_with(f);
  glColorP4uiv_load_with(f);
  glCompileShader_load_with(f);
  glCompressedTexImage1D_load_with(f);
  glCompressedTexImage2D_load_with(f);
  glCompressedTexImage3D_load_with(f);
  glCompressedTexSubImage1D_load_with(f);
  glCompressedTexSubImage2D_load_with(f);
  glCompressedTexSubImage3D_load_with(f);
  glCompressedTextureSubImage1D_load_with(f);
  glCompressedTextureSubImage2D_load_with(f);
  glCompressedTextureSubImage3D_load_with(f);
  glCopyBufferSubData_load_with(f);
  glCopyImageSubData_load_with(f);
  glCopyNamedBufferSubData_load_with(f);
  glCopyTexImage1D_load_with(f);
  glCopyTexImage2D_load_with(f);
  glCopyTexSubImage1D_load_with(f);
  glCopyTexSubImage2D_load_with(f);
  glCopyTexSubImage3D_load_with(f);
  glCopyTextureSubImage1D_load_with(f);
  glCopyTextureSubImage2D_load_with(f);
  glCopyTextureSubImage3D_load_with(f);
  glCreateBuffers_load_with(f);
  glCreateFramebuffers_load_with(f);
  glCreateProgram_load_with(f);
  glCreateProgramPipelines_load_with(f);
  glCreateQueries_load_with(f);
  glCreateRenderbuffers_load_with(f);
  glCreateSamplers_load_with(f);
  glCreateShader_load_with(f);
  glCreateShaderProgramv_load_with(f);
  glCreateTextures_load_with(f);
  glCreateTransformFeedbacks_load_with(f);
  glCreateVertexArrays_load_with(f);
  glCullFace_load_with(f);
  glDebugMessageCallback_load_with(f);
  glDebugMessageCallbackARB_load_with(f);
  glDebugMessageCallbackKHR_load_with(f);
  glDebugMessageControl_load_with(f);
  glDebugMessageControlARB_load_with(f);
  glDebugMessageControlKHR_load_with(f);
  glDebugMessageInsert_load_with(f);
  glDebugMessageInsertARB_load_with(f);
  glDebugMessageInsertKHR_load_with(f);
  glDeleteBuffers_load_with(f);
  glDeleteFramebuffers_load_with(f);
  glDeleteProgram_load_with(f);
  glDeleteProgramPipelines_load_with(f);
  glDeleteQueries_load_with(f);
  glDeleteRenderbuffers_load_with(f);
  glDeleteSamplers_load_with(f);
  glDeleteShader_load_with(f);
  glDeleteSync_load_with(f);
  glDeleteTextures_load_with(f);
  glDeleteTransformFeedbacks_load_with(f);
  glDeleteVertexArrays_load_with(f);
  glDepthFunc_load_with(f);
  glDepthMask_load_with(f);
  glDepthRange_load_with(f);
  glDepthRangeArrayv_load_with(f);
  glDepthRangeIndexed_load_with(f);
  glDepthRangef_load_with(f);
  glDetachShader_load_with(f);
  glDisable_load_with(f);
  glDisableVertexArrayAttrib_load_with(f);
  glDisableVertexAttribArray_load_with(f);
  glDisablei_load_with(f);
  glDispatchCompute_load_with(f);
  glDispatchComputeIndirect_load_with(f);
  glDrawArrays_load_with(f);
  glDrawArraysIndirect_load_with(f);
  glDrawArraysInstanced_load_with(f);
  glDrawArraysInstancedBaseInstance_load_with(f);
  glDrawBuffer_load_with(f);
  glDrawBuffers_load_with(f);
  glDrawElements_load_with(f);
  glDrawElementsBaseVertex_load_with(f);
  glDrawElementsIndirect_load_with(f);
  glDrawElementsInstanced_load_with(f);
  glDrawElementsInstancedBaseInstance_load_with(f);
  glDrawElementsInstancedBaseVertex_load_with(f);
  glDrawElementsInstancedBaseVertexBaseInstance_load_with(f);
  glDrawRangeElements_load_with(f);
  glDrawRangeElementsBaseVertex_load_with(f);
  glDrawTransformFeedback_load_with(f);
  glDrawTransformFeedbackInstanced_load_with(f);
  glDrawTransformFeedbackStream_load_with(f);
  glDrawTransformFeedbackStreamInstanced_load_with(f);
  glEnable_load_with(f);
  glEnableVertexArrayAttrib_load_with(f);
  glEnableVertexAttribArray_load_with(f);
  glEnablei_load_with(f);
  glEndConditionalRender_load_with(f);
  glEndQuery_load_with(f);
  glEndQueryIndexed_load_with(f);
  glEndTransformFeedback_load_with(f);
  glFenceSync_load_with(f);
  glFinish_load_with(f);
  glFlush_load_with(f);
  glFlushMappedBufferRange_load_with(f);
  glFlushMappedNamedBufferRange_load_with(f);
  glFramebufferParameteri_load_with(f);
  glFramebufferRenderbuffer_load_with(f);
  glFramebufferTexture_load_with(f);
  glFramebufferTexture1D_load_with(f);
  glFramebufferTexture2D_load_with(f);
  glFramebufferTexture3D_load_with(f);
  glFramebufferTextureLayer_load_with(f);
  glFrontFace_load_with(f);
  glGenBuffers_load_with(f);
  glGenFramebuffers_load_with(f);
  glGenProgramPipelines_load_with(f);
  glGenQueries_load_with(f);
  glGenRenderbuffers_load_with(f);
  glGenSamplers_load_with(f);
  glGenTextures_load_with(f);
  glGenTransformFeedbacks_load_with(f);
  glGenVertexArrays_load_with(f);
  glGenerateMipmap_load_with(f);
  glGenerateTextureMipmap_load_with(f);
  glGetActiveAtomicCounterBufferiv_load_with(f);
  glGetActiveAttrib_load_with(f);
  glGetActiveSubroutineName_load_with(f);
  glGetActiveSubroutineUniformName_load_with(f);
  glGetActiveSubroutineUniformiv_load_with(f);
  glGetActiveUniform_load_with(f);
  glGetActiveUniformBlockName_load_with(f);
  glGetActiveUniformBlockiv_load_with(f);
  glGetActiveUniformName_load_with(f);
  glGetActiveUniformsiv_load_with(f);
  glGetAttachedShaders_load_with(f);
  glGetAttribLocation_load_with(f);
  glGetBooleani_v_load_with(f);
  glGetBooleanv_load_with(f);
  glGetBufferParameteri64v_load_with(f);
  glGetBufferParameteriv_load_with(f);
  glGetBufferPointerv_load_with(f);
  glGetBufferSubData_load_with(f);
  glGetCompressedTexImage_load_with(f);
  glGetCompressedTextureImage_load_with(f);
  glGetCompressedTextureSubImage_load_with(f);
  glGetDebugMessageLog_load_with(f);
  glGetDebugMessageLogARB_load_with(f);
  glGetDebugMessageLogKHR_load_with(f);
  glGetDoublei_v_load_with(f);
  glGetDoublev_load_with(f);
  glGetError_load_with(f);
  glGetFloati_v_load_with(f);
  glGetFloatv_load_with(f);
  glGetFragDataIndex_load_with(f);
  glGetFragDataLocation_load_with(f);
  glGetFramebufferAttachmentParameteriv_load_with(f);
  glGetFramebufferParameteriv_load_with(f);
  glGetGraphicsResetStatus_load_with(f);
  glGetInteger64i_v_load_with(f);
  glGetInteger64v_load_with(f);
  glGetIntegeri_v_load_with(f);
  glGetIntegerv_load_with(f);
  glGetInternalformati64v_load_with(f);
  glGetInternalformativ_load_with(f);
  glGetMultisamplefv_load_with(f);
  glGetNamedBufferParameteri64v_load_with(f);
  glGetNamedBufferParameteriv_load_with(f);
  glGetNamedBufferPointerv_load_with(f);
  glGetNamedBufferSubData_load_with(f);
  glGetNamedFramebufferAttachmentParameteriv_load_with(f);
  glGetNamedFramebufferParameteriv_load_with(f);
  glGetNamedRenderbufferParameteriv_load_with(f);
  glGetObjectLabel_load_with(f);
  glGetObjectLabelKHR_load_with(f);
  glGetObjectPtrLabel_load_with(f);
  glGetObjectPtrLabelKHR_load_with(f);
  glGetPointerv_load_with(f);
  glGetPointervKHR_load_with(f);
  glGetProgramBinary_load_with(f);
  glGetProgramInfoLog_load_with(f);
  glGetProgramInterfaceiv_load_with(f);
  glGetProgramPipelineInfoLog_load_with(f);
  glGetProgramPipelineiv_load_with(f);
  glGetProgramResourceIndex_load_with(f);
  glGetProgramResourceLocation_load_with(f);
  glGetProgramResourceLocationIndex_load_with(f);
  glGetProgramResourceName_load_with(f);
  glGetProgramResourceiv_load_with(f);
  glGetProgramStageiv_load_with(f);
  glGetProgramiv_load_with(f);
  glGetQueryBufferObjecti64v_load_with(f);
  glGetQueryBufferObjectiv_load_with(f);
  glGetQueryBufferObjectui64v_load_with(f);
  glGetQueryBufferObjectuiv_load_with(f);
  glGetQueryIndexediv_load_with(f);
  glGetQueryObjecti64v_load_with(f);
  glGetQueryObjectiv_load_with(f);
  glGetQueryObjectui64v_load_with(f);
  glGetQueryObjectuiv_load_with(f);
  glGetQueryiv_load_with(f);
  glGetRenderbufferParameteriv_load_with(f);
  glGetSamplerParameterIiv_load_with(f);
  glGetSamplerParameterIuiv_load_with(f);
  glGetSamplerParameterfv_load_with(f);
  glGetSamplerParameteriv_load_with(f);
  glGetShaderInfoLog_load_with(f);
  glGetShaderPrecisionFormat_load_with(f);
  glGetShaderSource_load_with(f);
  glGetShaderiv_load_with(f);
  glGetString_load_with(f);
  glGetStringi_load_with(f);
  glGetSubroutineIndex_load_with(f);
  glGetSubroutineUniformLocation_load_with(f);
  glGetSynciv_load_with(f);
  glGetTexImage_load_with(f);
  glGetTexLevelParameterfv_load_with(f);
  glGetTexLevelParameteriv_load_with(f);
  glGetTexParameterIiv_load_with(f);
  glGetTexParameterIuiv_load_with(f);
  glGetTexParameterfv_load_with(f);
  glGetTexParameteriv_load_with(f);
  glGetTextureImage_load_with(f);
  glGetTextureLevelParameterfv_load_with(f);
  glGetTextureLevelParameteriv_load_with(f);
  glGetTextureParameterIiv_load_with(f);
  glGetTextureParameterIuiv_load_with(f);
  glGetTextureParameterfv_load_with(f);
  glGetTextureParameteriv_load_with(f);
  glGetTextureSubImage_load_with(f);
  glGetTransformFeedbackVarying_load_with(f);
  glGetTransformFeedbacki64_v_load_with(f);
  glGetTransformFeedbacki_v_load_with(f);
  glGetTransformFeedbackiv_load_with(f);
  glGetUniformBlockIndex_load_with(f);
  glGetUniformIndices_load_with(f);
  glGetUniformLocation_load_with(f);
  glGetUniformSubroutineuiv_load_with(f);
  glGetUniformdv_load_with(f);
  glGetUniformfv_load_with(f);
  glGetUniformiv_load_with(f);
  glGetUniformuiv_load_with(f);
  glGetVertexArrayIndexed64iv_load_with(f);
  glGetVertexArrayIndexediv_load_with(f);
  glGetVertexArrayiv_load_with(f);
  glGetVertexAttribIiv_load_with(f);
  glGetVertexAttribIuiv_load_with(f);
  glGetVertexAttribLdv_load_with(f);
  glGetVertexAttribPointerv_load_with(f);
  glGetVertexAttribdv_load_with(f);
  glGetVertexAttribfv_load_with(f);
  glGetVertexAttribiv_load_with(f);
  glGetnColorTable_load_with(f);
  glGetnCompressedTexImage_load_with(f);
  glGetnConvolutionFilter_load_with(f);
  glGetnHistogram_load_with(f);
  glGetnMapdv_load_with(f);
  glGetnMapfv_load_with(f);
  glGetnMapiv_load_with(f);
  glGetnMinmax_load_with(f);
  glGetnPixelMapfv_load_with(f);
  glGetnPixelMapuiv_load_with(f);
  glGetnPixelMapusv_load_with(f);
  glGetnPolygonStipple_load_with(f);
  glGetnSeparableFilter_load_with(f);
  glGetnTexImage_load_with(f);
  glGetnUniformdv_load_with(f);
  glGetnUniformfv_load_with(f);
  glGetnUniformiv_load_with(f);
  glGetnUniformuiv_load_with(f);
  glHint_load_with(f);
  glInvalidateBufferData_load_with(f);
  glInvalidateBufferSubData_load_with(f);
  glInvalidateFramebuffer_load_with(f);
  glInvalidateNamedFramebufferData_load_with(f);
  glInvalidateNamedFramebufferSubData_load_with(f);
  glInvalidateSubFramebuffer_load_with(f);
  glInvalidateTexImage_load_with(f);
  glInvalidateTexSubImage_load_with(f);
  glIsBuffer_load_with(f);
  glIsEnabled_load_with(f);
  glIsEnabledi_load_with(f);
  glIsFramebuffer_load_with(f);
  glIsProgram_load_with(f);
  glIsProgramPipeline_load_with(f);
  glIsQuery_load_with(f);
  glIsRenderbuffer_load_with(f);
  glIsSampler_load_with(f);
  glIsShader_load_with(f);
  glIsSync_load_with(f);
  glIsTexture_load_with(f);
  glIsTransformFeedback_load_with(f);
  glIsVertexArray_load_with(f);
  glLineWidth_load_with(f);
  glLinkProgram_load_with(f);
  glLogicOp_load_with(f);
  glMapBuffer_load_with(f);
  glMapBufferRange_load_with(f);
  glMapNamedBuffer_load_with(f);
  glMapNamedBufferRange_load_with(f);
  glMemoryBarrier_load_with(f);
  glMemoryBarrierByRegion_load_with(f);
  glMinSampleShading_load_with(f);
  glMultiDrawArrays_load_with(f);
  glMultiDrawArraysIndirect_load_with(f);
  glMultiDrawArraysIndirectCount_load_with(f);
  glMultiDrawElements_load_with(f);
  glMultiDrawElementsBaseVertex_load_with(f);
  glMultiDrawElementsIndirect_load_with(f);
  glMultiDrawElementsIndirectCount_load_with(f);
  glMultiTexCoordP1ui_load_with(f);
  glMultiTexCoordP1uiv_load_with(f);
  glMultiTexCoordP2ui_load_with(f);
  glMultiTexCoordP2uiv_load_with(f);
  glMultiTexCoordP3ui_load_with(f);
  glMultiTexCoordP3uiv_load_with(f);
  glMultiTexCoordP4ui_load_with(f);
  glMultiTexCoordP4uiv_load_with(f);
  glNamedBufferData_load_with(f);
  glNamedBufferStorage_load_with(f);
  glNamedBufferSubData_load_with(f);
  glNamedFramebufferDrawBuffer_load_with(f);
  glNamedFramebufferDrawBuffers_load_with(f);
  glNamedFramebufferParameteri_load_with(f);
  glNamedFramebufferReadBuffer_load_with(f);
  glNamedFramebufferRenderbuffer_load_with(f);
  glNamedFramebufferTexture_load_with(f);
  glNamedFramebufferTextureLayer_load_with(f);
  glNamedRenderbufferStorage_load_with(f);
  glNamedRenderbufferStorageMultisample_load_with(f);
  glNormalP3ui_load_with(f);
  glNormalP3uiv_load_with(f);
  glObjectLabel_load_with(f);
  glObjectLabelKHR_load_with(f);
  glObjectPtrLabel_load_with(f);
  glObjectPtrLabelKHR_load_with(f);
  glPatchParameterfv_load_with(f);
  glPatchParameteri_load_with(f);
  glPauseTransformFeedback_load_with(f);
  glPixelStoref_load_with(f);
  glPixelStorei_load_with(f);
  glPointParameterf_load_with(f);
  glPointParameterfv_load_with(f);
  glPointParameteri_load_with(f);
  glPointParameteriv_load_with(f);
  glPointSize_load_with(f);
  glPolygonMode_load_with(f);
  glPolygonOffset_load_with(f);
  glPolygonOffsetClamp_load_with(f);
  glPopDebugGroup_load_with(f);
  glPopDebugGroupKHR_load_with(f);
  glPrimitiveRestartIndex_load_with(f);
  glProgramBinary_load_with(f);
  glProgramParameteri_load_with(f);
  glProgramUniform1d_load_with(f);
  glProgramUniform1dv_load_with(f);
  glProgramUniform1f_load_with(f);
  glProgramUniform1fv_load_with(f);
  glProgramUniform1i_load_with(f);
  glProgramUniform1iv_load_with(f);
  glProgramUniform1ui_load_with(f);
  glProgramUniform1uiv_load_with(f);
  glProgramUniform2d_load_with(f);
  glProgramUniform2dv_load_with(f);
  glProgramUniform2f_load_with(f);
  glProgramUniform2fv_load_with(f);
  glProgramUniform2i_load_with(f);
  glProgramUniform2iv_load_with(f);
  glProgramUniform2ui_load_with(f);
  glProgramUniform2uiv_load_with(f);
  glProgramUniform3d_load_with(f);
  glProgramUniform3dv_load_with(f);
  glProgramUniform3f_load_with(f);
  glProgramUniform3fv_load_with(f);
  glProgramUniform3i_load_with(f);
  glProgramUniform3iv_load_with(f);
  glProgramUniform3ui_load_with(f);
  glProgramUniform3uiv_load_with(f);
  glProgramUniform4d_load_with(f);
  glProgramUniform4dv_load_with(f);
  glProgramUniform4f_load_with(f);
  glProgramUniform4fv_load_with(f);
  glProgramUniform4i_load_with(f);
  glProgramUniform4iv_load_with(f);
  glProgramUniform4ui_load_with(f);
  glProgramUniform4uiv_load_with(f);
  glProgramUniformMatrix2dv_load_with(f);
  glProgramUniformMatrix2fv_load_with(f);
  glProgramUniformMatrix2x3dv_load_with(f);
  glProgramUniformMatrix2x3fv_load_with(f);
  glProgramUniformMatrix2x4dv_load_with(f);
  glProgramUniformMatrix2x4fv_load_with(f);
  glProgramUniformMatrix3dv_load_with(f);
  glProgramUniformMatrix3fv_load_with(f);
  glProgramUniformMatrix3x2dv_load_with(f);
  glProgramUniformMatrix3x2fv_load_with(f);
  glProgramUniformMatrix3x4dv_load_with(f);
  glProgramUniformMatrix3x4fv_load_with(f);
  glProgramUniformMatrix4dv_load_with(f);
  glProgramUniformMatrix4fv_load_with(f);
  glProgramUniformMatrix4x2dv_load_with(f);
  glProgramUniformMatrix4x2fv_load_with(f);
  glProgramUniformMatrix4x3dv_load_with(f);
  glProgramUniformMatrix4x3fv_load_with(f);
  glProvokingVertex_load_with(f);
  glPushDebugGroup_load_with(f);
  glPushDebugGroupKHR_load_with(f);
  glQueryCounter_load_with(f);
  glReadBuffer_load_with(f);
  glReadPixels_load_with(f);
  glReadnPixels_load_with(f);
  glReleaseShaderCompiler_load_with(f);
  glRenderbufferStorage_load_with(f);
  glRenderbufferStorageMultisample_load_with(f);
  glResumeTransformFeedback_load_with(f);
  glSampleCoverage_load_with(f);
  glSampleMaski_load_with(f);
  glSamplerParameterIiv_load_with(f);
  glSamplerParameterIuiv_load_with(f);
  glSamplerParameterf_load_with(f);
  glSamplerParameterfv_load_with(f);
  glSamplerParameteri_load_with(f);
  glSamplerParameteriv_load_with(f);
  glScissor_load_with(f);
  glScissorArrayv_load_with(f);
  glScissorIndexed_load_with(f);
  glScissorIndexedv_load_with(f);
  glSecondaryColorP3ui_load_with(f);
  glSecondaryColorP3uiv_load_with(f);
  glShaderBinary_load_with(f);
  glShaderSource_load_with(f);
  glShaderStorageBlockBinding_load_with(f);
  glSpecializeShader_load_with(f);
  glStencilFunc_load_with(f);
  glStencilFuncSeparate_load_with(f);
  glStencilMask_load_with(f);
  glStencilMaskSeparate_load_with(f);
  glStencilOp_load_with(f);
  glStencilOpSeparate_load_with(f);
  glTexBuffer_load_with(f);
  glTexBufferRange_load_with(f);
  glTexCoordP1ui_load_with(f);
  glTexCoordP1uiv_load_with(f);
  glTexCoordP2ui_load_with(f);
  glTexCoordP2uiv_load_with(f);
  glTexCoordP3ui_load_with(f);
  glTexCoordP3uiv_load_with(f);
  glTexCoordP4ui_load_with(f);
  glTexCoordP4uiv_load_with(f);
  glTexImage1D_load_with(f);
  glTexImage2D_load_with(f);
  glTexImage2DMultisample_load_with(f);
  glTexImage3D_load_with(f);
  glTexImage3DMultisample_load_with(f);
  glTexParameterIiv_load_with(f);
  glTexParameterIuiv_load_with(f);
  glTexParameterf_load_with(f);
  glTexParameterfv_load_with(f);
  glTexParameteri_load_with(f);
  glTexParameteriv_load_with(f);
  glTexStorage1D_load_with(f);
  glTexStorage2D_load_with(f);
  glTexStorage2DMultisample_load_with(f);
  glTexStorage3D_load_with(f);
  glTexStorage3DMultisample_load_with(f);
  glTexSubImage1D_load_with(f);
  glTexSubImage2D_load_with(f);
  glTexSubImage3D_load_with(f);
  glTextureBarrier_load_with(f);
  glTextureBuffer_load_with(f);
  glTextureBufferRange_load_with(f);
  glTextureParameterIiv_load_with(f);
  glTextureParameterIuiv_load_with(f);
  glTextureParameterf_load_with(f);
  glTextureParameterfv_load_with(f);
  glTextureParameteri_load_with(f);
  glTextureParameteriv_load_with(f);
  glTextureStorage1D_load_with(f);
  glTextureStorage2D_load_with(f);
  glTextureStorage2DMultisample_load_with(f);
  glTextureStorage3D_load_with(f);
  glTextureStorage3DMultisample_load_with(f);
  glTextureSubImage1D_load_with(f);
  glTextureSubImage2D_load_with(f);
  glTextureSubImage3D_load_with(f);
  glTextureView_load_with(f);
  glTransformFeedbackBufferBase_load_with(f);
  glTransformFeedbackBufferRange_load_with(f);
  glTransformFeedbackVaryings_load_with(f);
  glUniform1d_load_with(f);
  glUniform1dv_load_with(f);
  glUniform1f_load_with(f);
  glUniform1fv_load_with(f);
  glUniform1i_load_with(f);
  glUniform1iv_load_with(f);
  glUniform1ui_load_with(f);
  glUniform1uiv_load_with(f);
  glUniform2d_load_with(f);
  glUniform2dv_load_with(f);
  glUniform2f_load_with(f);
  glUniform2fv_load_with(f);
  glUniform2i_load_with(f);
  glUniform2iv_load_with(f);
  glUniform2ui_load_with(f);
  glUniform2uiv_load_with(f);
  glUniform3d_load_with(f);
  glUniform3dv_load_with(f);
  glUniform3f_load_with(f);
  glUniform3fv_load_with(f);
  glUniform3i_load_with(f);
  glUniform3iv_load_with(f);
  glUniform3ui_load_with(f);
  glUniform3uiv_load_with(f);
  glUniform4d_load_with(f);
  glUniform4dv_load_with(f);
  glUniform4f_load_with(f);
  glUniform4fv_load_with(f);
  glUniform4i_load_with(f);
  glUniform4iv_load_with(f);
  glUniform4ui_load_with(f);
  glUniform4uiv_load_with(f);
  glUniformBlockBinding_load_with(f);
  glUniformMatrix2dv_load_with(f);
  glUniformMatrix2fv_load_with(f);
  glUniformMatrix2x3dv_load_with(f);
  glUniformMatrix2x3fv_load_with(f);
  glUniformMatrix2x4dv_load_with(f);
  glUniformMatrix2x4fv_load_with(f);
  glUniformMatrix3dv_load_with(f);
  glUniformMatrix3fv_load_with(f);
  glUniformMatrix3x2dv_load_with(f);
  glUniformMatrix3x2fv_load_with(f);
  glUniformMatrix3x4dv_load_with(f);
  glUniformMatrix3x4fv_load_with(f);
  glUniformMatrix4dv_load_with(f);
  glUniformMatrix4fv_load_with(f);
  glUniformMatrix4x2dv_load_with(f);
  glUniformMatrix4x2fv_load_with(f);
  glUniformMatrix4x3dv_load_with(f);
  glUniformMatrix4x3fv_load_with(f);
  glUniformSubroutinesuiv_load_with(f);
  glUnmapBuffer_load_with(f);
  glUnmapNamedBuffer_load_with(f);
  glUseProgram_load_with(f);
  glUseProgramStages_load_with(f);
  glValidateProgram_load_with(f);
  glValidateProgramPipeline_load_with(f);
  glVertexArrayAttribBinding_load_with(f);
  glVertexArrayAttribFormat_load_with(f);
  glVertexArrayAttribIFormat_load_with(f);
  glVertexArrayAttribLFormat_load_with(f);
  glVertexArrayBindingDivisor_load_with(f);
  glVertexArrayElementBuffer_load_with(f);
  glVertexArrayVertexBuffer_load_with(f);
  glVertexArrayVertexBuffers_load_with(f);
  glVertexAttrib1d_load_with(f);
  glVertexAttrib1dv_load_with(f);
  glVertexAttrib1f_load_with(f);
  glVertexAttrib1fv_load_with(f);
  glVertexAttrib1s_load_with(f);
  glVertexAttrib1sv_load_with(f);
  glVertexAttrib2d_load_with(f);
  glVertexAttrib2dv_load_with(f);
  glVertexAttrib2f_load_with(f);
  glVertexAttrib2fv_load_with(f);
  glVertexAttrib2s_load_with(f);
  glVertexAttrib2sv_load_with(f);
  glVertexAttrib3d_load_with(f);
  glVertexAttrib3dv_load_with(f);
  glVertexAttrib3f_load_with(f);
  glVertexAttrib3fv_load_with(f);
  glVertexAttrib3s_load_with(f);
  glVertexAttrib3sv_load_with(f);
  glVertexAttrib4Nbv_load_with(f);
  glVertexAttrib4Niv_load_with(f);
  glVertexAttrib4Nsv_load_with(f);
  glVertexAttrib4Nub_load_with(f);
  glVertexAttrib4Nubv_load_with(f);
  glVertexAttrib4Nuiv_load_with(f);
  glVertexAttrib4Nusv_load_with(f);
  glVertexAttrib4bv_load_with(f);
  glVertexAttrib4d_load_with(f);
  glVertexAttrib4dv_load_with(f);
  glVertexAttrib4f_load_with(f);
  glVertexAttrib4fv_load_with(f);
  glVertexAttrib4iv_load_with(f);
  glVertexAttrib4s_load_with(f);
  glVertexAttrib4sv_load_with(f);
  glVertexAttrib4ubv_load_with(f);
  glVertexAttrib4uiv_load_with(f);
  glVertexAttrib4usv_load_with(f);
  glVertexAttribBinding_load_with(f);
  glVertexAttribDivisor_load_with(f);
  glVertexAttribFormat_load_with(f);
  glVertexAttribI1i_load_with(f);
  glVertexAttribI1iv_load_with(f);
  glVertexAttribI1ui_load_with(f);
  glVertexAttribI1uiv_load_with(f);
  glVertexAttribI2i_load_with(f);
  glVertexAttribI2iv_load_with(f);
  glVertexAttribI2ui_load_with(f);
  glVertexAttribI2uiv_load_with(f);
  glVertexAttribI3i_load_with(f);
  glVertexAttribI3iv_load_with(f);
  glVertexAttribI3ui_load_with(f);
  glVertexAttribI3uiv_load_with(f);
  glVertexAttribI4bv_load_with(f);
  glVertexAttribI4i_load_with(f);
  glVertexAttribI4iv_load_with(f);
  glVertexAttribI4sv_load_with(f);
  glVertexAttribI4ubv_load_with(f);
  glVertexAttribI4ui_load_with(f);
  glVertexAttribI4uiv_load_with(f);
  glVertexAttribI4usv_load_with(f);
  glVertexAttribIFormat_load_with(f);
  glVertexAttribIPointer_load_with(f);
  glVertexAttribL1d_load_with(f);
  glVertexAttribL1dv_load_with(f);
  glVertexAttribL2d_load_with(f);
  glVertexAttribL2dv_load_with(f);
  glVertexAttribL3d_load_with(f);
  glVertexAttribL3dv_load_with(f);
  glVertexAttribL4d_load_with(f);
  glVertexAttribL4dv_load_with(f);
  glVertexAttribLFormat_load_with(f);
  glVertexAttribLPointer_load_with(f);
  glVertexAttribP1ui_load_with(f);
  glVertexAttribP1uiv_load_with(f);
  glVertexAttribP2ui_load_with(f);
  glVertexAttribP2uiv_load_with(f);
  glVertexAttribP3ui_load_with(f);
  glVertexAttribP3uiv_load_with(f);
  glVertexAttribP4ui_load_with(f);
  glVertexAttribP4uiv_load_with(f);
  glVertexAttribPointer_load_with(f);
  glVertexBindingDivisor_load_with(f);
  glVertexP2ui_load_with(f);
  glVertexP2uiv_load_with(f);
  glVertexP3ui_load_with(f);
  glVertexP3uiv_load_with(f);
  glVertexP4ui_load_with(f);
  glVertexP4uiv_load_with(f);
  glViewport_load_with(f);
  glViewportArrayv_load_with(f);
  glViewportIndexedf_load_with(f);
  glViewportIndexedfv_load_with(f);
  glWaitSync_load_with(f);
}

/// Clears the global GL function settings.
///
/// ## Safety
/// * Similar to [`load_global_gl`]. Specifically, while it's safe to clear a
///   function pointer's value (calling the function will just panic), this is
///   altering non-atomic global memory, so you **must** provide external
///   synchronization of your own.
pub unsafe fn reset_global_gl() {
  glActiveShaderProgram_reset_ptr();
  glActiveTexture_reset_ptr();
  glAttachShader_reset_ptr();
  glBeginConditionalRender_reset_ptr();
  glBeginQuery_reset_ptr();
  glBeginQueryIndexed_reset_ptr();
  glBeginTransformFeedback_reset_ptr();
  glBindAttribLocation_reset_ptr();
  glBindBuffer_reset_ptr();
  glBindBufferBase_reset_ptr();
  glBindBufferRange_reset_ptr();
  glBindBuffersBase_reset_ptr();
  glBindBuffersRange_reset_ptr();
  glBindFragDataLocation_reset_ptr();
  glBindFragDataLocationIndexed_reset_ptr();
  glBindFramebuffer_reset_ptr();
  glBindImageTexture_reset_ptr();
  glBindImageTextures_reset_ptr();
  glBindProgramPipeline_reset_ptr();
  glBindRenderbuffer_reset_ptr();
  glBindSampler_reset_ptr();
  glBindSamplers_reset_ptr();
  glBindTexture_reset_ptr();
  glBindTextureUnit_reset_ptr();
  glBindTextures_reset_ptr();
  glBindTransformFeedback_reset_ptr();
  glBindVertexArray_reset_ptr();
  glBindVertexBuffer_reset_ptr();
  glBindVertexBuffers_reset_ptr();
  glBlendColor_reset_ptr();
  glBlendEquation_reset_ptr();
  glBlendEquationSeparate_reset_ptr();
  glBlendEquationSeparatei_reset_ptr();
  glBlendEquationi_reset_ptr();
  glBlendFunc_reset_ptr();
  glBlendFuncSeparate_reset_ptr();
  glBlendFuncSeparatei_reset_ptr();
  glBlendFunci_reset_ptr();
  glBlitFramebuffer_reset_ptr();
  glBlitNamedFramebuffer_reset_ptr();
  glBufferData_reset_ptr();
  glBufferStorage_reset_ptr();
  glBufferSubData_reset_ptr();
  glCheckFramebufferStatus_reset_ptr();
  glCheckNamedFramebufferStatus_reset_ptr();
  glClampColor_reset_ptr();
  glClear_reset_ptr();
  glClearBufferData_reset_ptr();
  glClearBufferSubData_reset_ptr();
  glClearBufferfi_reset_ptr();
  glClearBufferfv_reset_ptr();
  glClearBufferiv_reset_ptr();
  glClearBufferuiv_reset_ptr();
  glClearColor_reset_ptr();
  glClearDepth_reset_ptr();
  glClearDepthf_reset_ptr();
  glClearNamedBufferData_reset_ptr();
  glClearNamedBufferSubData_reset_ptr();
  glClearNamedFramebufferfi_reset_ptr();
  glClearNamedFramebufferfv_reset_ptr();
  glClearNamedFramebufferiv_reset_ptr();
  glClearNamedFramebufferuiv_reset_ptr();
  glClearStencil_reset_ptr();
  glClearTexImage_reset_ptr();
  glClearTexSubImage_reset_ptr();
  glClientWaitSync_reset_ptr();
  glClipControl_reset_ptr();
  glColorMask_reset_ptr();
  glColorMaski_reset_ptr();
  glColorP3ui_reset_ptr();
  glColorP3uiv_reset_ptr();
  glColorP4ui_reset_ptr();
  glColorP4uiv_reset_ptr();
  glCompileShader_reset_ptr();
  glCompressedTexImage1D_reset_ptr();
  glCompressedTexImage2D_reset_ptr();
  glCompressedTexImage3D_reset_ptr();
  glCompressedTexSubImage1D_reset_ptr();
  glCompressedTexSubImage2D_reset_ptr();
  glCompressedTexSubImage3D_reset_ptr();
  glCompressedTextureSubImage1D_reset_ptr();
  glCompressedTextureSubImage2D_reset_ptr();
  glCompressedTextureSubImage3D_reset_ptr();
  glCopyBufferSubData_reset_ptr();
  glCopyImageSubData_reset_ptr();
  glCopyNamedBufferSubData_reset_ptr();
  glCopyTexImage1D_reset_ptr();
  glCopyTexImage2D_reset_ptr();
  glCopyTexSubImage1D_reset_ptr();
  glCopyTexSubImage2D_reset_ptr();
  glCopyTexSubImage3D_reset_ptr();
  glCopyTextureSubImage1D_reset_ptr();
  glCopyTextureSubImage2D_reset_ptr();
  glCopyTextureSubImage3D_reset_ptr();
  glCreateBuffers_reset_ptr();
  glCreateFramebuffers_reset_ptr();
  glCreateProgram_reset_ptr();
  glCreateProgramPipelines_reset_ptr();
  glCreateQueries_reset_ptr();
  glCreateRenderbuffers_reset_ptr();
  glCreateSamplers_reset_ptr();
  glCreateShader_reset_ptr();
  glCreateShaderProgramv_reset_ptr();
  glCreateTextures_reset_ptr();
  glCreateTransformFeedbacks_reset_ptr();
  glCreateVertexArrays_reset_ptr();
  glCullFace_reset_ptr();
  glDebugMessageCallback_reset_ptr();
  glDebugMessageCallbackARB_reset_ptr();
  glDebugMessageCallbackKHR_reset_ptr();
  glDebugMessageControl_reset_ptr();
  glDebugMessageControlARB_reset_ptr();
  glDebugMessageControlKHR_reset_ptr();
  glDebugMessageInsert_reset_ptr();
  glDebugMessageInsertARB_reset_ptr();
  glDebugMessageInsertKHR_reset_ptr();
  glDeleteBuffers_reset_ptr();
  glDeleteFramebuffers_reset_ptr();
  glDeleteProgram_reset_ptr();
  glDeleteProgramPipelines_reset_ptr();
  glDeleteQueries_reset_ptr();
  glDeleteRenderbuffers_reset_ptr();
  glDeleteSamplers_reset_ptr();
  glDeleteShader_reset_ptr();
  glDeleteSync_reset_ptr();
  glDeleteTextures_reset_ptr();
  glDeleteTransformFeedbacks_reset_ptr();
  glDeleteVertexArrays_reset_ptr();
  glDepthFunc_reset_ptr();
  glDepthMask_reset_ptr();
  glDepthRange_reset_ptr();
  glDepthRangeArrayv_reset_ptr();
  glDepthRangeIndexed_reset_ptr();
  glDepthRangef_reset_ptr();
  glDetachShader_reset_ptr();
  glDisable_reset_ptr();
  glDisableVertexArrayAttrib_reset_ptr();
  glDisableVertexAttribArray_reset_ptr();
  glDisablei_reset_ptr();
  glDispatchCompute_reset_ptr();
  glDispatchComputeIndirect_reset_ptr();
  glDrawArrays_reset_ptr();
  glDrawArraysIndirect_reset_ptr();
  glDrawArraysInstanced_reset_ptr();
  glDrawArraysInstancedBaseInstance_reset_ptr();
  glDrawBuffer_reset_ptr();
  glDrawBuffers_reset_ptr();
  glDrawElements_reset_ptr();
  glDrawElementsBaseVertex_reset_ptr();
  glDrawElementsIndirect_reset_ptr();
  glDrawElementsInstanced_reset_ptr();
  glDrawElementsInstancedBaseInstance_reset_ptr();
  glDrawElementsInstancedBaseVertex_reset_ptr();
  glDrawElementsInstancedBaseVertexBaseInstance_reset_ptr();
  glDrawRangeElements_reset_ptr();
  glDrawRangeElementsBaseVertex_reset_ptr();
  glDrawTransformFeedback_reset_ptr();
  glDrawTransformFeedbackInstanced_reset_ptr();
  glDrawTransformFeedbackStream_reset_ptr();
  glDrawTransformFeedbackStreamInstanced_reset_ptr();
  glEnable_reset_ptr();
  glEnableVertexArrayAttrib_reset_ptr();
  glEnableVertexAttribArray_reset_ptr();
  glEnablei_reset_ptr();
  glEndConditionalRender_reset_ptr();
  glEndQuery_reset_ptr();
  glEndQueryIndexed_reset_ptr();
  glEndTransformFeedback_reset_ptr();
  glFenceSync_reset_ptr();
  glFinish_reset_ptr();
  glFlush_reset_ptr();
  glFlushMappedBufferRange_reset_ptr();
  glFlushMappedNamedBufferRange_reset_ptr();
  glFramebufferParameteri_reset_ptr();
  glFramebufferRenderbuffer_reset_ptr();
  glFramebufferTexture_reset_ptr();
  glFramebufferTexture1D_reset_ptr();
  glFramebufferTexture2D_reset_ptr();
  glFramebufferTexture3D_reset_ptr();
  glFramebufferTextureLayer_reset_ptr();
  glFrontFace_reset_ptr();
  glGenBuffers_reset_ptr();
  glGenFramebuffers_reset_ptr();
  glGenProgramPipelines_reset_ptr();
  glGenQueries_reset_ptr();
  glGenRenderbuffers_reset_ptr();
  glGenSamplers_reset_ptr();
  glGenTextures_reset_ptr();
  glGenTransformFeedbacks_reset_ptr();
  glGenVertexArrays_reset_ptr();
  glGenerateMipmap_reset_ptr();
  glGenerateTextureMipmap_reset_ptr();
  glGetActiveAtomicCounterBufferiv_reset_ptr();
  glGetActiveAttrib_reset_ptr();
  glGetActiveSubroutineName_reset_ptr();
  glGetActiveSubroutineUniformName_reset_ptr();
  glGetActiveSubroutineUniformiv_reset_ptr();
  glGetActiveUniform_reset_ptr();
  glGetActiveUniformBlockName_reset_ptr();
  glGetActiveUniformBlockiv_reset_ptr();
  glGetActiveUniformName_reset_ptr();
  glGetActiveUniformsiv_reset_ptr();
  glGetAttachedShaders_reset_ptr();
  glGetAttribLocation_reset_ptr();
  glGetBooleani_v_reset_ptr();
  glGetBooleanv_reset_ptr();
  glGetBufferParameteri64v_reset_ptr();
  glGetBufferParameteriv_reset_ptr();
  glGetBufferPointerv_reset_ptr();
  glGetBufferSubData_reset_ptr();
  glGetCompressedTexImage_reset_ptr();
  glGetCompressedTextureImage_reset_ptr();
  glGetCompressedTextureSubImage_reset_ptr();
  glGetDebugMessageLog_reset_ptr();
  glGetDebugMessageLogARB_reset_ptr();
  glGetDebugMessageLogKHR_reset_ptr();
  glGetDoublei_v_reset_ptr();
  glGetDoublev_reset_ptr();
  glGetError_reset_ptr();
  glGetFloati_v_reset_ptr();
  glGetFloatv_reset_ptr();
  glGetFragDataIndex_reset_ptr();
  glGetFragDataLocation_reset_ptr();
  glGetFramebufferAttachmentParameteriv_reset_ptr();
  glGetFramebufferParameteriv_reset_ptr();
  glGetGraphicsResetStatus_reset_ptr();
  glGetInteger64i_v_reset_ptr();
  glGetInteger64v_reset_ptr();
  glGetIntegeri_v_reset_ptr();
  glGetIntegerv_reset_ptr();
  glGetInternalformati64v_reset_ptr();
  glGetInternalformativ_reset_ptr();
  glGetMultisamplefv_reset_ptr();
  glGetNamedBufferParameteri64v_reset_ptr();
  glGetNamedBufferParameteriv_reset_ptr();
  glGetNamedBufferPointerv_reset_ptr();
  glGetNamedBufferSubData_reset_ptr();
  glGetNamedFramebufferAttachmentParameteriv_reset_ptr();
  glGetNamedFramebufferParameteriv_reset_ptr();
  glGetNamedRenderbufferParameteriv_reset_ptr();
  glGetObjectLabel_reset_ptr();
  glGetObjectLabelKHR_reset_ptr();
  glGetObjectPtrLabel_reset_ptr();
  glGetObjectPtrLabelKHR_reset_ptr();
  glGetPointerv_reset_ptr();
  glGetPointervKHR_reset_ptr();
  glGetProgramBinary_reset_ptr();
  glGetProgramInfoLog_reset_ptr();
  glGetProgramInterfaceiv_reset_ptr();
  glGetProgramPipelineInfoLog_reset_ptr();
  glGetProgramPipelineiv_reset_ptr();
  glGetProgramResourceIndex_reset_ptr();
  glGetProgramResourceLocation_reset_ptr();
  glGetProgramResourceLocationIndex_reset_ptr();
  glGetProgramResourceName_reset_ptr();
  glGetProgramResourceiv_reset_ptr();
  glGetProgramStageiv_reset_ptr();
  glGetProgramiv_reset_ptr();
  glGetQueryBufferObjecti64v_reset_ptr();
  glGetQueryBufferObjectiv_reset_ptr();
  glGetQueryBufferObjectui64v_reset_ptr();
  glGetQueryBufferObjectuiv_reset_ptr();
  glGetQueryIndexediv_reset_ptr();
  glGetQueryObjecti64v_reset_ptr();
  glGetQueryObjectiv_reset_ptr();
  glGetQueryObjectui64v_reset_ptr();
  glGetQueryObjectuiv_reset_ptr();
  glGetQueryiv_reset_ptr();
  glGetRenderbufferParameteriv_reset_ptr();
  glGetSamplerParameterIiv_reset_ptr();
  glGetSamplerParameterIuiv_reset_ptr();
  glGetSamplerParameterfv_reset_ptr();
  glGetSamplerParameteriv_reset_ptr();
  glGetShaderInfoLog_reset_ptr();
  glGetShaderPrecisionFormat_reset_ptr();
  glGetShaderSource_reset_ptr();
  glGetShaderiv_reset_ptr();
  glGetString_reset_ptr();
  glGetStringi_reset_ptr();
  glGetSubroutineIndex_reset_ptr();
  glGetSubroutineUniformLocation_reset_ptr();
  glGetSynciv_reset_ptr();
  glGetTexImage_reset_ptr();
  glGetTexLevelParameterfv_reset_ptr();
  glGetTexLevelParameteriv_reset_ptr();
  glGetTexParameterIiv_reset_ptr();
  glGetTexParameterIuiv_reset_ptr();
  glGetTexParameterfv_reset_ptr();
  glGetTexParameteriv_reset_ptr();
  glGetTextureImage_reset_ptr();
  glGetTextureLevelParameterfv_reset_ptr();
  glGetTextureLevelParameteriv_reset_ptr();
  glGetTextureParameterIiv_reset_ptr();
  glGetTextureParameterIuiv_reset_ptr();
  glGetTextureParameterfv_reset_ptr();
  glGetTextureParameteriv_reset_ptr();
  glGetTextureSubImage_reset_ptr();
  glGetTransformFeedbackVarying_reset_ptr();
  glGetTransformFeedbacki64_v_reset_ptr();
  glGetTransformFeedbacki_v_reset_ptr();
  glGetTransformFeedbackiv_reset_ptr();
  glGetUniformBlockIndex_reset_ptr();
  glGetUniformIndices_reset_ptr();
  glGetUniformLocation_reset_ptr();
  glGetUniformSubroutineuiv_reset_ptr();
  glGetUniformdv_reset_ptr();
  glGetUniformfv_reset_ptr();
  glGetUniformiv_reset_ptr();
  glGetUniformuiv_reset_ptr();
  glGetVertexArrayIndexed64iv_reset_ptr();
  glGetVertexArrayIndexediv_reset_ptr();
  glGetVertexArrayiv_reset_ptr();
  glGetVertexAttribIiv_reset_ptr();
  glGetVertexAttribIuiv_reset_ptr();
  glGetVertexAttribLdv_reset_ptr();
  glGetVertexAttribPointerv_reset_ptr();
  glGetVertexAttribdv_reset_ptr();
  glGetVertexAttribfv_reset_ptr();
  glGetVertexAttribiv_reset_ptr();
  glGetnColorTable_reset_ptr();
  glGetnCompressedTexImage_reset_ptr();
  glGetnConvolutionFilter_reset_ptr();
  glGetnHistogram_reset_ptr();
  glGetnMapdv_reset_ptr();
  glGetnMapfv_reset_ptr();
  glGetnMapiv_reset_ptr();
  glGetnMinmax_reset_ptr();
  glGetnPixelMapfv_reset_ptr();
  glGetnPixelMapuiv_reset_ptr();
  glGetnPixelMapusv_reset_ptr();
  glGetnPolygonStipple_reset_ptr();
  glGetnSeparableFilter_reset_ptr();
  glGetnTexImage_reset_ptr();
  glGetnUniformdv_reset_ptr();
  glGetnUniformfv_reset_ptr();
  glGetnUniformiv_reset_ptr();
  glGetnUniformuiv_reset_ptr();
  glHint_reset_ptr();
  glInvalidateBufferData_reset_ptr();
  glInvalidateBufferSubData_reset_ptr();
  glInvalidateFramebuffer_reset_ptr();
  glInvalidateNamedFramebufferData_reset_ptr();
  glInvalidateNamedFramebufferSubData_reset_ptr();
  glInvalidateSubFramebuffer_reset_ptr();
  glInvalidateTexImage_reset_ptr();
  glInvalidateTexSubImage_reset_ptr();
  glIsBuffer_reset_ptr();
  glIsEnabled_reset_ptr();
  glIsEnabledi_reset_ptr();
  glIsFramebuffer_reset_ptr();
  glIsProgram_reset_ptr();
  glIsProgramPipeline_reset_ptr();
  glIsQuery_reset_ptr();
  glIsRenderbuffer_reset_ptr();
  glIsSampler_reset_ptr();
  glIsShader_reset_ptr();
  glIsSync_reset_ptr();
  glIsTexture_reset_ptr();
  glIsTransformFeedback_reset_ptr();
  glIsVertexArray_reset_ptr();
  glLineWidth_reset_ptr();
  glLinkProgram_reset_ptr();
  glLogicOp_reset_ptr();
  glMapBuffer_reset_ptr();
  glMapBufferRange_reset_ptr();
  glMapNamedBuffer_reset_ptr();
  glMapNamedBufferRange_reset_ptr();
  glMemoryBarrier_reset_ptr();
  glMemoryBarrierByRegion_reset_ptr();
  glMinSampleShading_reset_ptr();
  glMultiDrawArrays_reset_ptr();
  glMultiDrawArraysIndirect_reset_ptr();
  glMultiDrawArraysIndirectCount_reset_ptr();
  glMultiDrawElements_reset_ptr();
  glMultiDrawElementsBaseVertex_reset_ptr();
  glMultiDrawElementsIndirect_reset_ptr();
  glMultiDrawElementsIndirectCount_reset_ptr();
  glMultiTexCoordP1ui_reset_ptr();
  glMultiTexCoordP1uiv_reset_ptr();
  glMultiTexCoordP2ui_reset_ptr();
  glMultiTexCoordP2uiv_reset_ptr();
  glMultiTexCoordP3ui_reset_ptr();
  glMultiTexCoordP3uiv_reset_ptr();
  glMultiTexCoordP4ui_reset_ptr();
  glMultiTexCoordP4uiv_reset_ptr();
  glNamedBufferData_reset_ptr();
  glNamedBufferStorage_reset_ptr();
  glNamedBufferSubData_reset_ptr();
  glNamedFramebufferDrawBuffer_reset_ptr();
  glNamedFramebufferDrawBuffers_reset_ptr();
  glNamedFramebufferParameteri_reset_ptr();
  glNamedFramebufferReadBuffer_reset_ptr();
  glNamedFramebufferRenderbuffer_reset_ptr();
  glNamedFramebufferTexture_reset_ptr();
  glNamedFramebufferTextureLayer_reset_ptr();
  glNamedRenderbufferStorage_reset_ptr();
  glNamedRenderbufferStorageMultisample_reset_ptr();
  glNormalP3ui_reset_ptr();
  glNormalP3uiv_reset_ptr();
  glObjectLabel_reset_ptr();
  glObjectLabelKHR_reset_ptr();
  glObjectPtrLabel_reset_ptr();
  glObjectPtrLabelKHR_reset_ptr();
  glPatchParameterfv_reset_ptr();
  glPatchParameteri_reset_ptr();
  glPauseTransformFeedback_reset_ptr();
  glPixelStoref_reset_ptr();
  glPixelStorei_reset_ptr();
  glPointParameterf_reset_ptr();
  glPointParameterfv_reset_ptr();
  glPointParameteri_reset_ptr();
  glPointParameteriv_reset_ptr();
  glPointSize_reset_ptr();
  glPolygonMode_reset_ptr();
  glPolygonOffset_reset_ptr();
  glPolygonOffsetClamp_reset_ptr();
  glPopDebugGroup_reset_ptr();
  glPopDebugGroupKHR_reset_ptr();
  glPrimitiveRestartIndex_reset_ptr();
  glProgramBinary_reset_ptr();
  glProgramParameteri_reset_ptr();
  glProgramUniform1d_reset_ptr();
  glProgramUniform1dv_reset_ptr();
  glProgramUniform1f_reset_ptr();
  glProgramUniform1fv_reset_ptr();
  glProgramUniform1i_reset_ptr();
  glProgramUniform1iv_reset_ptr();
  glProgramUniform1ui_reset_ptr();
  glProgramUniform1uiv_reset_ptr();
  glProgramUniform2d_reset_ptr();
  glProgramUniform2dv_reset_ptr();
  glProgramUniform2f_reset_ptr();
  glProgramUniform2fv_reset_ptr();
  glProgramUniform2i_reset_ptr();
  glProgramUniform2iv_reset_ptr();
  glProgramUniform2ui_reset_ptr();
  glProgramUniform2uiv_reset_ptr();
  glProgramUniform3d_reset_ptr();
  glProgramUniform3dv_reset_ptr();
  glProgramUniform3f_reset_ptr();
  glProgramUniform3fv_reset_ptr();
  glProgramUniform3i_reset_ptr();
  glProgramUniform3iv_reset_ptr();
  glProgramUniform3ui_reset_ptr();
  glProgramUniform3uiv_reset_ptr();
  glProgramUniform4d_reset_ptr();
  glProgramUniform4dv_reset_ptr();
  glProgramUniform4f_reset_ptr();
  glProgramUniform4fv_reset_ptr();
  glProgramUniform4i_reset_ptr();
  glProgramUniform4iv_reset_ptr();
  glProgramUniform4ui_reset_ptr();
  glProgramUniform4uiv_reset_ptr();
  glProgramUniformMatrix2dv_reset_ptr();
  glProgramUniformMatrix2fv_reset_ptr();
  glProgramUniformMatrix2x3dv_reset_ptr();
  glProgramUniformMatrix2x3fv_reset_ptr();
  glProgramUniformMatrix2x4dv_reset_ptr();
  glProgramUniformMatrix2x4fv_reset_ptr();
  glProgramUniformMatrix3dv_reset_ptr();
  glProgramUniformMatrix3fv_reset_ptr();
  glProgramUniformMatrix3x2dv_reset_ptr();
  glProgramUniformMatrix3x2fv_reset_ptr();
  glProgramUniformMatrix3x4dv_reset_ptr();
  glProgramUniformMatrix3x4fv_reset_ptr();
  glProgramUniformMatrix4dv_reset_ptr();
  glProgramUniformMatrix4fv_reset_ptr();
  glProgramUniformMatrix4x2dv_reset_ptr();
  glProgramUniformMatrix4x2fv_reset_ptr();
  glProgramUniformMatrix4x3dv_reset_ptr();
  glProgramUniformMatrix4x3fv_reset_ptr();
  glProvokingVertex_reset_ptr();
  glPushDebugGroup_reset_ptr();
  glPushDebugGroupKHR_reset_ptr();
  glQueryCounter_reset_ptr();
  glReadBuffer_reset_ptr();
  glReadPixels_reset_ptr();
  glReadnPixels_reset_ptr();
  glReleaseShaderCompiler_reset_ptr();
  glRenderbufferStorage_reset_ptr();
  glRenderbufferStorageMultisample_reset_ptr();
  glResumeTransformFeedback_reset_ptr();
  glSampleCoverage_reset_ptr();
  glSampleMaski_reset_ptr();
  glSamplerParameterIiv_reset_ptr();
  glSamplerParameterIuiv_reset_ptr();
  glSamplerParameterf_reset_ptr();
  glSamplerParameterfv_reset_ptr();
  glSamplerParameteri_reset_ptr();
  glSamplerParameteriv_reset_ptr();
  glScissor_reset_ptr();
  glScissorArrayv_reset_ptr();
  glScissorIndexed_reset_ptr();
  glScissorIndexedv_reset_ptr();
  glSecondaryColorP3ui_reset_ptr();
  glSecondaryColorP3uiv_reset_ptr();
  glShaderBinary_reset_ptr();
  glShaderSource_reset_ptr();
  glShaderStorageBlockBinding_reset_ptr();
  glSpecializeShader_reset_ptr();
  glStencilFunc_reset_ptr();
  glStencilFuncSeparate_reset_ptr();
  glStencilMask_reset_ptr();
  glStencilMaskSeparate_reset_ptr();
  glStencilOp_reset_ptr();
  glStencilOpSeparate_reset_ptr();
  glTexBuffer_reset_ptr();
  glTexBufferRange_reset_ptr();
  glTexCoordP1ui_reset_ptr();
  glTexCoordP1uiv_reset_ptr();
  glTexCoordP2ui_reset_ptr();
  glTexCoordP2uiv_reset_ptr();
  glTexCoordP3ui_reset_ptr();
  glTexCoordP3uiv_reset_ptr();
  glTexCoordP4ui_reset_ptr();
  glTexCoordP4uiv_reset_ptr();
  glTexImage1D_reset_ptr();
  glTexImage2D_reset_ptr();
  glTexImage2DMultisample_reset_ptr();
  glTexImage3D_reset_ptr();
  glTexImage3DMultisample_reset_ptr();
  glTexParameterIiv_reset_ptr();
  glTexParameterIuiv_reset_ptr();
  glTexParameterf_reset_ptr();
  glTexParameterfv_reset_ptr();
  glTexParameteri_reset_ptr();
  glTexParameteriv_reset_ptr();
  glTexStorage1D_reset_ptr();
  glTexStorage2D_reset_ptr();
  glTexStorage2DMultisample_reset_ptr();
  glTexStorage3D_reset_ptr();
  glTexStorage3DMultisample_reset_ptr();
  glTexSubImage1D_reset_ptr();
  glTexSubImage2D_reset_ptr();
  glTexSubImage3D_reset_ptr();
  glTextureBarrier_reset_ptr();
  glTextureBuffer_reset_ptr();
  glTextureBufferRange_reset_ptr();
  glTextureParameterIiv_reset_ptr();
  glTextureParameterIuiv_reset_ptr();
  glTextureParameterf_reset_ptr();
  glTextureParameterfv_reset_ptr();
  glTextureParameteri_reset_ptr();
  glTextureParameteriv_reset_ptr();
  glTextureStorage1D_reset_ptr();
  glTextureStorage2D_reset_ptr();
  glTextureStorage2DMultisample_reset_ptr();
  glTextureStorage3D_reset_ptr();
  glTextureStorage3DMultisample_reset_ptr();
  glTextureSubImage1D_reset_ptr();
  glTextureSubImage2D_reset_ptr();
  glTextureSubImage3D_reset_ptr();
  glTextureView_reset_ptr();
  glTransformFeedbackBufferBase_reset_ptr();
  glTransformFeedbackBufferRange_reset_ptr();
  glTransformFeedbackVaryings_reset_ptr();
  glUniform1d_reset_ptr();
  glUniform1dv_reset_ptr();
  glUniform1f_reset_ptr();
  glUniform1fv_reset_ptr();
  glUniform1i_reset_ptr();
  glUniform1iv_reset_ptr();
  glUniform1ui_reset_ptr();
  glUniform1uiv_reset_ptr();
  glUniform2d_reset_ptr();
  glUniform2dv_reset_ptr();
  glUniform2f_reset_ptr();
  glUniform2fv_reset_ptr();
  glUniform2i_reset_ptr();
  glUniform2iv_reset_ptr();
  glUniform2ui_reset_ptr();
  glUniform2uiv_reset_ptr();
  glUniform3d_reset_ptr();
  glUniform3dv_reset_ptr();
  glUniform3f_reset_ptr();
  glUniform3fv_reset_ptr();
  glUniform3i_reset_ptr();
  glUniform3iv_reset_ptr();
  glUniform3ui_reset_ptr();
  glUniform3uiv_reset_ptr();
  glUniform4d_reset_ptr();
  glUniform4dv_reset_ptr();
  glUniform4f_reset_ptr();
  glUniform4fv_reset_ptr();
  glUniform4i_reset_ptr();
  glUniform4iv_reset_ptr();
  glUniform4ui_reset_ptr();
  glUniform4uiv_reset_ptr();
  glUniformBlockBinding_reset_ptr();
  glUniformMatrix2dv_reset_ptr();
  glUniformMatrix2fv_reset_ptr();
  glUniformMatrix2x3dv_reset_ptr();
  glUniformMatrix2x3fv_reset_ptr();
  glUniformMatrix2x4dv_reset_ptr();
  glUniformMatrix2x4fv_reset_ptr();
  glUniformMatrix3dv_reset_ptr();
  glUniformMatrix3fv_reset_ptr();
  glUniformMatrix3x2dv_reset_ptr();
  glUniformMatrix3x2fv_reset_ptr();
  glUniformMatrix3x4dv_reset_ptr();
  glUniformMatrix3x4fv_reset_ptr();
  glUniformMatrix4dv_reset_ptr();
  glUniformMatrix4fv_reset_ptr();
  glUniformMatrix4x2dv_reset_ptr();
  glUniformMatrix4x2fv_reset_ptr();
  glUniformMatrix4x3dv_reset_ptr();
  glUniformMatrix4x3fv_reset_ptr();
  glUniformSubroutinesuiv_reset_ptr();
  glUnmapBuffer_reset_ptr();
  glUnmapNamedBuffer_reset_ptr();
  glUseProgram_reset_ptr();
  glUseProgramStages_reset_ptr();
  glValidateProgram_reset_ptr();
  glValidateProgramPipeline_reset_ptr();
  glVertexArrayAttribBinding_reset_ptr();
  glVertexArrayAttribFormat_reset_ptr();
  glVertexArrayAttribIFormat_reset_ptr();
  glVertexArrayAttribLFormat_reset_ptr();
  glVertexArrayBindingDivisor_reset_ptr();
  glVertexArrayElementBuffer_reset_ptr();
  glVertexArrayVertexBuffer_reset_ptr();
  glVertexArrayVertexBuffers_reset_ptr();
  glVertexAttrib1d_reset_ptr();
  glVertexAttrib1dv_reset_ptr();
  glVertexAttrib1f_reset_ptr();
  glVertexAttrib1fv_reset_ptr();
  glVertexAttrib1s_reset_ptr();
  glVertexAttrib1sv_reset_ptr();
  glVertexAttrib2d_reset_ptr();
  glVertexAttrib2dv_reset_ptr();
  glVertexAttrib2f_reset_ptr();
  glVertexAttrib2fv_reset_ptr();
  glVertexAttrib2s_reset_ptr();
  glVertexAttrib2sv_reset_ptr();
  glVertexAttrib3d_reset_ptr();
  glVertexAttrib3dv_reset_ptr();
  glVertexAttrib3f_reset_ptr();
  glVertexAttrib3fv_reset_ptr();
  glVertexAttrib3s_reset_ptr();
  glVertexAttrib3sv_reset_ptr();
  glVertexAttrib4Nbv_reset_ptr();
  glVertexAttrib4Niv_reset_ptr();
  glVertexAttrib4Nsv_reset_ptr();
  glVertexAttrib4Nub_reset_ptr();
  glVertexAttrib4Nubv_reset_ptr();
  glVertexAttrib4Nuiv_reset_ptr();
  glVertexAttrib4Nusv_reset_ptr();
  glVertexAttrib4bv_reset_ptr();
  glVertexAttrib4d_reset_ptr();
  glVertexAttrib4dv_reset_ptr();
  glVertexAttrib4f_reset_ptr();
  glVertexAttrib4fv_reset_ptr();
  glVertexAttrib4iv_reset_ptr();
  glVertexAttrib4s_reset_ptr();
  glVertexAttrib4sv_reset_ptr();
  glVertexAttrib4ubv_reset_ptr();
  glVertexAttrib4uiv_reset_ptr();
  glVertexAttrib4usv_reset_ptr();
  glVertexAttribBinding_reset_ptr();
  glVertexAttribDivisor_reset_ptr();
  glVertexAttribFormat_reset_ptr();
  glVertexAttribI1i_reset_ptr();
  glVertexAttribI1iv_reset_ptr();
  glVertexAttribI1ui_reset_ptr();
  glVertexAttribI1uiv_reset_ptr();
  glVertexAttribI2i_reset_ptr();
  glVertexAttribI2iv_reset_ptr();
  glVertexAttribI2ui_reset_ptr();
  glVertexAttribI2uiv_reset_ptr();
  glVertexAttribI3i_reset_ptr();
  glVertexAttribI3iv_reset_ptr();
  glVertexAttribI3ui_reset_ptr();
  glVertexAttribI3uiv_reset_ptr();
  glVertexAttribI4bv_reset_ptr();
  glVertexAttribI4i_reset_ptr();
  glVertexAttribI4iv_reset_ptr();
  glVertexAttribI4sv_reset_ptr();
  glVertexAttribI4ubv_reset_ptr();
  glVertexAttribI4ui_reset_ptr();
  glVertexAttribI4uiv_reset_ptr();
  glVertexAttribI4usv_reset_ptr();
  glVertexAttribIFormat_reset_ptr();
  glVertexAttribIPointer_reset_ptr();
  glVertexAttribL1d_reset_ptr();
  glVertexAttribL1dv_reset_ptr();
  glVertexAttribL2d_reset_ptr();
  glVertexAttribL2dv_reset_ptr();
  glVertexAttribL3d_reset_ptr();
  glVertexAttribL3dv_reset_ptr();
  glVertexAttribL4d_reset_ptr();
  glVertexAttribL4dv_reset_ptr();
  glVertexAttribLFormat_reset_ptr();
  glVertexAttribLPointer_reset_ptr();
  glVertexAttribP1ui_reset_ptr();
  glVertexAttribP1uiv_reset_ptr();
  glVertexAttribP2ui_reset_ptr();
  glVertexAttribP2uiv_reset_ptr();
  glVertexAttribP3ui_reset_ptr();
  glVertexAttribP3uiv_reset_ptr();
  glVertexAttribP4ui_reset_ptr();
  glVertexAttribP4uiv_reset_ptr();
  glVertexAttribPointer_reset_ptr();
  glVertexBindingDivisor_reset_ptr();
  glVertexP2ui_reset_ptr();
  glVertexP2uiv_reset_ptr();
  glVertexP3ui_reset_ptr();
  glVertexP3uiv_reset_ptr();
  glVertexP4ui_reset_ptr();
  glVertexP4uiv_reset_ptr();
  glViewport_reset_ptr();
  glViewportArrayv_reset_ptr();
  glViewportIndexedf_reset_ptr();
  glViewportIndexedfv_reset_ptr();
  glWaitSync_reset_ptr();
}

/// [glActiveShaderProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveShaderProgram.xhtml)
/// * `pipeline` class: program pipeline
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glActiveShaderProgram(pipeline: GLuint, program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glActiveShaderProgram_p.0.get() } {
    Some(glActiveShaderProgram_inner) => glActiveShaderProgram_inner(pipeline, program),
    None => gl_not_loaded("glActiveShaderProgram"),
  }
}
static glActiveShaderProgram_p: GlFnCell<glActiveShaderProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glActiveShaderProgram_is_loaded() -> bool {
  unsafe { *glActiveShaderProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glActiveShaderProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glActiveShaderProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glActiveShaderProgram_t>>(gl_ptr_filter(f(b"glActiveShaderProgram\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glActiveShaderProgram_reset_ptr() {
  *glActiveShaderProgram_p.0.get() = None;
}
/// [glActiveTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveTexture.xhtml)
/// * `texture` group: TextureUnit
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glActiveTexture(texture: TextureUnit) {
  #[allow(unused_unsafe)]
  match unsafe { *glActiveTexture_p.0.get() } {
    Some(glActiveTexture_inner) => glActiveTexture_inner(texture),
    None => gl_not_loaded("glActiveTexture"),
  }
}
static glActiveTexture_p: GlFnCell<glActiveTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glActiveTexture_is_loaded() -> bool {
  unsafe { *glActiveTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glActiveTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glActiveTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glActiveTexture_t>>(gl_ptr_filter(f(b"glActiveTexture\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glActiveTexture_reset_ptr() {
  *glActiveTexture_p.0.get() = None;
}
/// [glAttachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml)
/// * `program` class: program
/// * `shader` class: shader
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glAttachShader(program: GLuint, shader: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glAttachShader_p.0.get() } {
    Some(glAttachShader_inner) => glAttachShader_inner(program, shader),
    None => gl_not_loaded("glAttachShader"),
  }
}
static glAttachShader_p: GlFnCell<glAttachShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glAttachShader_is_loaded() -> bool {
  unsafe { *glAttachShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glAttachShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glAttachShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glAttachShader_t>>(gl_ptr_filter(f(b"glAttachShader\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glAttachShader_reset_ptr() {
  *glAttachShader_p.0.get() = None;
}
/// [glBeginConditionalRender](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginConditionalRender.xhtml)
/// * `mode` group: ConditionalRenderMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBeginConditionalRender(id: GLuint, mode: ConditionalRenderMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glBeginConditionalRender_p.0.get() } {
    Some(glBeginConditionalRender_inner) => glBeginConditionalRender_inner(id, mode),
    None => gl_not_loaded("glBeginConditionalRender"),
  }
}
static glBeginConditionalRender_p: GlFnCell<glBeginConditionalRender_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBeginConditionalRender_is_loaded() -> bool {
  unsafe { *glBeginConditionalRender_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBeginConditionalRender_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBeginConditionalRender_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBeginConditionalRender_t>>(gl_ptr_filter(f(b"glBeginConditionalRender\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBeginConditionalRender_reset_ptr() {
  *glBeginConditionalRender_p.0.get() = None;
}
/// [glBeginQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginQuery.xhtml)
/// * `target` group: QueryTarget
/// * `id` class: query
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBeginQuery(target: QueryTarget, id: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBeginQuery_p.0.get() } {
    Some(glBeginQuery_inner) => glBeginQuery_inner(target, id),
    None => gl_not_loaded("glBeginQuery"),
  }
}
static glBeginQuery_p: GlFnCell<glBeginQuery_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBeginQuery_is_loaded() -> bool {
  unsafe { *glBeginQuery_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBeginQuery_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBeginQuery_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBeginQuery_t>>(gl_ptr_filter(f(b"glBeginQuery\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBeginQuery_reset_ptr() {
  *glBeginQuery_p.0.get() = None;
}
/// [glBeginQueryIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginQueryIndexed.xhtml)
/// * `target` group: QueryTarget
/// * `id` class: query
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBeginQueryIndexed(target: QueryTarget, index: GLuint, id: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBeginQueryIndexed_p.0.get() } {
    Some(glBeginQueryIndexed_inner) => glBeginQueryIndexed_inner(target, index, id),
    None => gl_not_loaded("glBeginQueryIndexed"),
  }
}
static glBeginQueryIndexed_p: GlFnCell<glBeginQueryIndexed_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBeginQueryIndexed_is_loaded() -> bool {
  unsafe { *glBeginQueryIndexed_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBeginQueryIndexed_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBeginQueryIndexed_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBeginQueryIndexed_t>>(gl_ptr_filter(f(b"glBeginQueryIndexed\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBeginQueryIndexed_reset_ptr() {
  *glBeginQueryIndexed_p.0.get() = None;
}
/// [glBeginTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginTransformFeedback.xhtml)
/// * `primitiveMode` group: PrimitiveType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBeginTransformFeedback(primitiveMode: PrimitiveType) {
  #[allow(unused_unsafe)]
  match unsafe { *glBeginTransformFeedback_p.0.get() } {
    Some(glBeginTransformFeedback_inner) => glBeginTransformFeedback_inner(primitiveMode),
    None => gl_not_loaded("glBeginTransformFeedback"),
  }
}
static glBeginTransformFeedback_p: GlFnCell<glBeginTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBeginTransformFeedback_is_loaded() -> bool {
  unsafe { *glBeginTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBeginTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBeginTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBeginTransformFeedback_t>>(gl_ptr_filter(f(b"glBeginTransformFeedback\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBeginTransformFeedback_reset_ptr() {
  *glBeginTransformFeedback_p.0.get() = None;
}
/// [glBindAttribLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindAttribLocation.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindAttribLocation_p.0.get() } {
    Some(glBindAttribLocation_inner) => glBindAttribLocation_inner(program, index, name),
    None => gl_not_loaded("glBindAttribLocation"),
  }
}
static glBindAttribLocation_p: GlFnCell<glBindAttribLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindAttribLocation_is_loaded() -> bool {
  unsafe { *glBindAttribLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindAttribLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindAttribLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindAttribLocation_t>>(gl_ptr_filter(f(b"glBindAttribLocation\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindAttribLocation_reset_ptr() {
  *glBindAttribLocation_p.0.get() = None;
}
/// [glBindBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindBuffer(target: BufferTargetARB, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindBuffer_p.0.get() } {
    Some(glBindBuffer_inner) => glBindBuffer_inner(target, buffer),
    None => gl_not_loaded("glBindBuffer"),
  }
}
static glBindBuffer_p: GlFnCell<glBindBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindBuffer_is_loaded() -> bool {
  unsafe { *glBindBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindBuffer_t>>(gl_ptr_filter(f(b"glBindBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindBuffer_reset_ptr() {
  *glBindBuffer_p.0.get() = None;
}
/// [glBindBufferBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferBase.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindBufferBase(target: BufferTargetARB, index: GLuint, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindBufferBase_p.0.get() } {
    Some(glBindBufferBase_inner) => glBindBufferBase_inner(target, index, buffer),
    None => gl_not_loaded("glBindBufferBase"),
  }
}
static glBindBufferBase_p: GlFnCell<glBindBufferBase_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindBufferBase_is_loaded() -> bool {
  unsafe { *glBindBufferBase_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindBufferBase_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindBufferBase_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindBufferBase_t>>(gl_ptr_filter(f(b"glBindBufferBase\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindBufferBase_reset_ptr() {
  *glBindBufferBase_p.0.get() = None;
}
/// [glBindBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferRange.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindBufferRange(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindBufferRange_p.0.get() } {
    Some(glBindBufferRange_inner) => glBindBufferRange_inner(target, index, buffer, offset, size),
    None => gl_not_loaded("glBindBufferRange"),
  }
}
static glBindBufferRange_p: GlFnCell<glBindBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindBufferRange_is_loaded() -> bool {
  unsafe { *glBindBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindBufferRange_t>>(gl_ptr_filter(f(b"glBindBufferRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindBufferRange_reset_ptr() {
  *glBindBufferRange_p.0.get() = None;
}
/// [glBindBuffersBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffersBase.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffers` class: buffer
/// * `buffers` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindBuffersBase(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindBuffersBase_p.0.get() } {
    Some(glBindBuffersBase_inner) => glBindBuffersBase_inner(target, first, count, buffers),
    None => gl_not_loaded("glBindBuffersBase"),
  }
}
static glBindBuffersBase_p: GlFnCell<glBindBuffersBase_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindBuffersBase_is_loaded() -> bool {
  unsafe { *glBindBuffersBase_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindBuffersBase_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindBuffersBase_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindBuffersBase_t>>(gl_ptr_filter(f(b"glBindBuffersBase\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindBuffersBase_reset_ptr() {
  *glBindBuffersBase_p.0.get() = None;
}
/// [glBindBuffersRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffersRange.xhtml)
/// * `target` group: BufferTargetARB
/// * `buffers` class: buffer
/// * `buffers` len: count
/// * `offsets` len: count
/// * `sizes` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindBuffersRange(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindBuffersRange_p.0.get() } {
    Some(glBindBuffersRange_inner) => glBindBuffersRange_inner(target, first, count, buffers, offsets, sizes),
    None => gl_not_loaded("glBindBuffersRange"),
  }
}
static glBindBuffersRange_p: GlFnCell<glBindBuffersRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindBuffersRange_is_loaded() -> bool {
  unsafe { *glBindBuffersRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindBuffersRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindBuffersRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindBuffersRange_t>>(gl_ptr_filter(f(b"glBindBuffersRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindBuffersRange_reset_ptr() {
  *glBindBuffersRange_p.0.get() = None;
}
/// [glBindFragDataLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocation.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindFragDataLocation_p.0.get() } {
    Some(glBindFragDataLocation_inner) => glBindFragDataLocation_inner(program, color, name),
    None => gl_not_loaded("glBindFragDataLocation"),
  }
}
static glBindFragDataLocation_p: GlFnCell<glBindFragDataLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindFragDataLocation_is_loaded() -> bool {
  unsafe { *glBindFragDataLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindFragDataLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindFragDataLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindFragDataLocation_t>>(gl_ptr_filter(f(b"glBindFragDataLocation\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindFragDataLocation_reset_ptr() {
  *glBindFragDataLocation_p.0.get() = None;
}
/// [glBindFragDataLocationIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocationIndexed.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindFragDataLocationIndexed_p.0.get() } {
    Some(glBindFragDataLocationIndexed_inner) => glBindFragDataLocationIndexed_inner(program, colorNumber, index, name),
    None => gl_not_loaded("glBindFragDataLocationIndexed"),
  }
}
static glBindFragDataLocationIndexed_p: GlFnCell<glBindFragDataLocationIndexed_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindFragDataLocationIndexed_is_loaded() -> bool {
  unsafe { *glBindFragDataLocationIndexed_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindFragDataLocationIndexed_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindFragDataLocationIndexed_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindFragDataLocationIndexed_t>>(gl_ptr_filter(f(b"glBindFragDataLocationIndexed\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindFragDataLocationIndexed_reset_ptr() {
  *glBindFragDataLocationIndexed_p.0.get() = None;
}
/// [glBindFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFramebuffer.xhtml)
/// * `target` group: FramebufferTarget
/// * `framebuffer` class: framebuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindFramebuffer(target: FramebufferTarget, framebuffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindFramebuffer_p.0.get() } {
    Some(glBindFramebuffer_inner) => glBindFramebuffer_inner(target, framebuffer),
    None => gl_not_loaded("glBindFramebuffer"),
  }
}
static glBindFramebuffer_p: GlFnCell<glBindFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindFramebuffer_is_loaded() -> bool {
  unsafe { *glBindFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindFramebuffer_t>>(gl_ptr_filter(f(b"glBindFramebuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindFramebuffer_reset_ptr() {
  *glBindFramebuffer_p.0.get() = None;
}
/// [glBindImageTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindImageTexture.xhtml)
/// * `texture` class: texture
/// * `layered` group: Boolean
/// * `access` group: BufferAccessARB
/// * `format` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: BufferAccessARB, format: InternalFormat) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindImageTexture_p.0.get() } {
    Some(glBindImageTexture_inner) => glBindImageTexture_inner(unit, texture, level, layered, layer, access, format),
    None => gl_not_loaded("glBindImageTexture"),
  }
}
static glBindImageTexture_p: GlFnCell<glBindImageTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindImageTexture_is_loaded() -> bool {
  unsafe { *glBindImageTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindImageTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindImageTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindImageTexture_t>>(gl_ptr_filter(f(b"glBindImageTexture\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindImageTexture_reset_ptr() {
  *glBindImageTexture_p.0.get() = None;
}
/// [glBindImageTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindImageTextures.xhtml)
/// * `textures` class: texture
/// * `textures` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindImageTextures_p.0.get() } {
    Some(glBindImageTextures_inner) => glBindImageTextures_inner(first, count, textures),
    None => gl_not_loaded("glBindImageTextures"),
  }
}
static glBindImageTextures_p: GlFnCell<glBindImageTextures_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindImageTextures_is_loaded() -> bool {
  unsafe { *glBindImageTextures_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindImageTextures_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindImageTextures_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindImageTextures_t>>(gl_ptr_filter(f(b"glBindImageTextures\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindImageTextures_reset_ptr() {
  *glBindImageTextures_p.0.get() = None;
}
/// [glBindProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindProgramPipeline.xhtml)
/// * `pipeline` class: program pipeline
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindProgramPipeline(pipeline: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindProgramPipeline_p.0.get() } {
    Some(glBindProgramPipeline_inner) => glBindProgramPipeline_inner(pipeline),
    None => gl_not_loaded("glBindProgramPipeline"),
  }
}
static glBindProgramPipeline_p: GlFnCell<glBindProgramPipeline_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindProgramPipeline_is_loaded() -> bool {
  unsafe { *glBindProgramPipeline_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindProgramPipeline_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindProgramPipeline_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindProgramPipeline_t>>(gl_ptr_filter(f(b"glBindProgramPipeline\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindProgramPipeline_reset_ptr() {
  *glBindProgramPipeline_p.0.get() = None;
}
/// [glBindRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindRenderbuffer.xhtml)
/// * `target` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindRenderbuffer(target: RenderbufferTarget, renderbuffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindRenderbuffer_p.0.get() } {
    Some(glBindRenderbuffer_inner) => glBindRenderbuffer_inner(target, renderbuffer),
    None => gl_not_loaded("glBindRenderbuffer"),
  }
}
static glBindRenderbuffer_p: GlFnCell<glBindRenderbuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindRenderbuffer_is_loaded() -> bool {
  unsafe { *glBindRenderbuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindRenderbuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindRenderbuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindRenderbuffer_t>>(gl_ptr_filter(f(b"glBindRenderbuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindRenderbuffer_reset_ptr() {
  *glBindRenderbuffer_p.0.get() = None;
}
/// [glBindSampler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindSampler.xhtml)
/// * `sampler` class: sampler
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindSampler(unit: GLuint, sampler: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindSampler_p.0.get() } {
    Some(glBindSampler_inner) => glBindSampler_inner(unit, sampler),
    None => gl_not_loaded("glBindSampler"),
  }
}
static glBindSampler_p: GlFnCell<glBindSampler_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindSampler_is_loaded() -> bool {
  unsafe { *glBindSampler_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindSampler_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindSampler_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindSampler_t>>(gl_ptr_filter(f(b"glBindSampler\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindSampler_reset_ptr() {
  *glBindSampler_p.0.get() = None;
}
/// [glBindSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindSamplers.xhtml)
/// * `samplers` class: sampler
/// * `samplers` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindSamplers_p.0.get() } {
    Some(glBindSamplers_inner) => glBindSamplers_inner(first, count, samplers),
    None => gl_not_loaded("glBindSamplers"),
  }
}
static glBindSamplers_p: GlFnCell<glBindSamplers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindSamplers_is_loaded() -> bool {
  unsafe { *glBindSamplers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindSamplers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindSamplers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindSamplers_t>>(gl_ptr_filter(f(b"glBindSamplers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindSamplers_reset_ptr() {
  *glBindSamplers_p.0.get() = None;
}
/// [glBindTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTexture.xhtml)
/// * `target` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindTexture(target: TextureTarget, texture: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindTexture_p.0.get() } {
    Some(glBindTexture_inner) => glBindTexture_inner(target, texture),
    None => gl_not_loaded("glBindTexture"),
  }
}
static glBindTexture_p: GlFnCell<glBindTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindTexture_is_loaded() -> bool {
  unsafe { *glBindTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindTexture_t>>(gl_ptr_filter(f(b"glBindTexture\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindTexture_reset_ptr() {
  *glBindTexture_p.0.get() = None;
}
/// [glBindTextureUnit](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTextureUnit.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindTextureUnit(unit: GLuint, texture: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindTextureUnit_p.0.get() } {
    Some(glBindTextureUnit_inner) => glBindTextureUnit_inner(unit, texture),
    None => gl_not_loaded("glBindTextureUnit"),
  }
}
static glBindTextureUnit_p: GlFnCell<glBindTextureUnit_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindTextureUnit_is_loaded() -> bool {
  unsafe { *glBindTextureUnit_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindTextureUnit_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindTextureUnit_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindTextureUnit_t>>(gl_ptr_filter(f(b"glBindTextureUnit\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindTextureUnit_reset_ptr() {
  *glBindTextureUnit_p.0.get() = None;
}
/// [glBindTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTextures.xhtml)
/// * `textures` class: texture
/// * `textures` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindTextures_p.0.get() } {
    Some(glBindTextures_inner) => glBindTextures_inner(first, count, textures),
    None => gl_not_loaded("glBindTextures"),
  }
}
static glBindTextures_p: GlFnCell<glBindTextures_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindTextures_is_loaded() -> bool {
  unsafe { *glBindTextures_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindTextures_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindTextures_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindTextures_t>>(gl_ptr_filter(f(b"glBindTextures\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindTextures_reset_ptr() {
  *glBindTextures_p.0.get() = None;
}
/// [glBindTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTransformFeedback.xhtml)
/// * `target` group: BindTransformFeedbackTarget
/// * `id` class: transform feedback
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindTransformFeedback(target: BindTransformFeedbackTarget, id: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindTransformFeedback_p.0.get() } {
    Some(glBindTransformFeedback_inner) => glBindTransformFeedback_inner(target, id),
    None => gl_not_loaded("glBindTransformFeedback"),
  }
}
static glBindTransformFeedback_p: GlFnCell<glBindTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindTransformFeedback_is_loaded() -> bool {
  unsafe { *glBindTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindTransformFeedback_t>>(gl_ptr_filter(f(b"glBindTransformFeedback\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindTransformFeedback_reset_ptr() {
  *glBindTransformFeedback_p.0.get() = None;
}
/// [glBindVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml)
/// * `array` class: vertex array
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindVertexArray(array: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindVertexArray_p.0.get() } {
    Some(glBindVertexArray_inner) => glBindVertexArray_inner(array),
    None => gl_not_loaded("glBindVertexArray"),
  }
}
static glBindVertexArray_p: GlFnCell<glBindVertexArray_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindVertexArray_is_loaded() -> bool {
  unsafe { *glBindVertexArray_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindVertexArray_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindVertexArray_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindVertexArray_t>>(gl_ptr_filter(f(b"glBindVertexArray\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindVertexArray_reset_ptr() {
  *glBindVertexArray_p.0.get() = None;
}
/// [glBindVertexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffer.xhtml)
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindVertexBuffer_p.0.get() } {
    Some(glBindVertexBuffer_inner) => glBindVertexBuffer_inner(bindingindex, buffer, offset, stride),
    None => gl_not_loaded("glBindVertexBuffer"),
  }
}
static glBindVertexBuffer_p: GlFnCell<glBindVertexBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindVertexBuffer_is_loaded() -> bool {
  unsafe { *glBindVertexBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindVertexBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindVertexBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindVertexBuffer_t>>(gl_ptr_filter(f(b"glBindVertexBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindVertexBuffer_reset_ptr() {
  *glBindVertexBuffer_p.0.get() = None;
}
/// [glBindVertexBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffers.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: count
/// * `offsets` len: count
/// * `strides` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindVertexBuffers(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindVertexBuffers_p.0.get() } {
    Some(glBindVertexBuffers_inner) => glBindVertexBuffers_inner(first, count, buffers, offsets, strides),
    None => gl_not_loaded("glBindVertexBuffers"),
  }
}
static glBindVertexBuffers_p: GlFnCell<glBindVertexBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindVertexBuffers_is_loaded() -> bool {
  unsafe { *glBindVertexBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindVertexBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindVertexBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindVertexBuffers_t>>(gl_ptr_filter(f(b"glBindVertexBuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBindVertexBuffers_reset_ptr() {
  *glBindVertexBuffers_p.0.get() = None;
}
/// [glBlendColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendColor.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendColor_p.0.get() } {
    Some(glBlendColor_inner) => glBlendColor_inner(red, green, blue, alpha),
    None => gl_not_loaded("glBlendColor"),
  }
}
static glBlendColor_p: GlFnCell<glBlendColor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendColor_is_loaded() -> bool {
  unsafe { *glBlendColor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendColor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendColor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendColor_t>>(gl_ptr_filter(f(b"glBlendColor\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendColor_reset_ptr() {
  *glBlendColor_p.0.get() = None;
}
/// [glBlendEquation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquation.xhtml)
/// * `mode` group: BlendEquationModeEXT
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendEquation(mode: BlendEquationModeEXT) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendEquation_p.0.get() } {
    Some(glBlendEquation_inner) => glBlendEquation_inner(mode),
    None => gl_not_loaded("glBlendEquation"),
  }
}
static glBlendEquation_p: GlFnCell<glBlendEquation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendEquation_is_loaded() -> bool {
  unsafe { *glBlendEquation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendEquation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendEquation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendEquation_t>>(gl_ptr_filter(f(b"glBlendEquation\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendEquation_reset_ptr() {
  *glBlendEquation_p.0.get() = None;
}
/// [glBlendEquationSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparate.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendEquationSeparate(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendEquationSeparate_p.0.get() } {
    Some(glBlendEquationSeparate_inner) => glBlendEquationSeparate_inner(modeRGB, modeAlpha),
    None => gl_not_loaded("glBlendEquationSeparate"),
  }
}
static glBlendEquationSeparate_p: GlFnCell<glBlendEquationSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendEquationSeparate_is_loaded() -> bool {
  unsafe { *glBlendEquationSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendEquationSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendEquationSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendEquationSeparate_t>>(gl_ptr_filter(f(b"glBlendEquationSeparate\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendEquationSeparate_reset_ptr() {
  *glBlendEquationSeparate_p.0.get() = None;
}
/// [glBlendEquationSeparatei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparatei.xhtml)
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendEquationSeparatei(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendEquationSeparatei_p.0.get() } {
    Some(glBlendEquationSeparatei_inner) => glBlendEquationSeparatei_inner(buf, modeRGB, modeAlpha),
    None => gl_not_loaded("glBlendEquationSeparatei"),
  }
}
static glBlendEquationSeparatei_p: GlFnCell<glBlendEquationSeparatei_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendEquationSeparatei_is_loaded() -> bool {
  unsafe { *glBlendEquationSeparatei_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendEquationSeparatei_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendEquationSeparatei_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendEquationSeparatei_t>>(gl_ptr_filter(f(b"glBlendEquationSeparatei\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendEquationSeparatei_reset_ptr() {
  *glBlendEquationSeparatei_p.0.get() = None;
}
/// [glBlendEquationi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationi.xhtml)
/// * `mode` group: BlendEquationModeEXT
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendEquationi(buf: GLuint, mode: BlendEquationModeEXT) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendEquationi_p.0.get() } {
    Some(glBlendEquationi_inner) => glBlendEquationi_inner(buf, mode),
    None => gl_not_loaded("glBlendEquationi"),
  }
}
static glBlendEquationi_p: GlFnCell<glBlendEquationi_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendEquationi_is_loaded() -> bool {
  unsafe { *glBlendEquationi_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendEquationi_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendEquationi_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendEquationi_t>>(gl_ptr_filter(f(b"glBlendEquationi\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendEquationi_reset_ptr() {
  *glBlendEquationi_p.0.get() = None;
}
/// [glBlendFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunc.xhtml)
/// * `sfactor` group: BlendingFactor
/// * `dfactor` group: BlendingFactor
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendFunc(sfactor: BlendingFactor, dfactor: BlendingFactor) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendFunc_p.0.get() } {
    Some(glBlendFunc_inner) => glBlendFunc_inner(sfactor, dfactor),
    None => gl_not_loaded("glBlendFunc"),
  }
}
static glBlendFunc_p: GlFnCell<glBlendFunc_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendFunc_is_loaded() -> bool {
  unsafe { *glBlendFunc_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendFunc_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendFunc_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendFunc_t>>(gl_ptr_filter(f(b"glBlendFunc\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendFunc_reset_ptr() {
  *glBlendFunc_p.0.get() = None;
}
/// [glBlendFuncSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparate.xhtml)
/// * `sfactorRGB` group: BlendingFactor
/// * `dfactorRGB` group: BlendingFactor
/// * `sfactorAlpha` group: BlendingFactor
/// * `dfactorAlpha` group: BlendingFactor
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendFuncSeparate(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendFuncSeparate_p.0.get() } {
    Some(glBlendFuncSeparate_inner) => glBlendFuncSeparate_inner(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha),
    None => gl_not_loaded("glBlendFuncSeparate"),
  }
}
static glBlendFuncSeparate_p: GlFnCell<glBlendFuncSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendFuncSeparate_is_loaded() -> bool {
  unsafe { *glBlendFuncSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendFuncSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendFuncSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendFuncSeparate_t>>(gl_ptr_filter(f(b"glBlendFuncSeparate\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendFuncSeparate_reset_ptr() {
  *glBlendFuncSeparate_p.0.get() = None;
}
/// [glBlendFuncSeparatei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparatei.xhtml)
/// * `srcRGB` group: BlendingFactor
/// * `dstRGB` group: BlendingFactor
/// * `srcAlpha` group: BlendingFactor
/// * `dstAlpha` group: BlendingFactor
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendFuncSeparatei(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendFuncSeparatei_p.0.get() } {
    Some(glBlendFuncSeparatei_inner) => glBlendFuncSeparatei_inner(buf, srcRGB, dstRGB, srcAlpha, dstAlpha),
    None => gl_not_loaded("glBlendFuncSeparatei"),
  }
}
static glBlendFuncSeparatei_p: GlFnCell<glBlendFuncSeparatei_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendFuncSeparatei_is_loaded() -> bool {
  unsafe { *glBlendFuncSeparatei_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendFuncSeparatei_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendFuncSeparatei_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendFuncSeparatei_t>>(gl_ptr_filter(f(b"glBlendFuncSeparatei\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendFuncSeparatei_reset_ptr() {
  *glBlendFuncSeparatei_p.0.get() = None;
}
/// [glBlendFunci](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunci.xhtml)
/// * `src` group: BlendingFactor
/// * `dst` group: BlendingFactor
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendFunci(buf: GLuint, src: BlendingFactor, dst: BlendingFactor) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendFunci_p.0.get() } {
    Some(glBlendFunci_inner) => glBlendFunci_inner(buf, src, dst),
    None => gl_not_loaded("glBlendFunci"),
  }
}
static glBlendFunci_p: GlFnCell<glBlendFunci_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendFunci_is_loaded() -> bool {
  unsafe { *glBlendFunci_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendFunci_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendFunci_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendFunci_t>>(gl_ptr_filter(f(b"glBlendFunci\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlendFunci_reset_ptr() {
  *glBlendFunci_p.0.get() = None;
}
/// [glBlitFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitFramebuffer.xhtml)
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlitFramebuffer_p.0.get() } {
    Some(glBlitFramebuffer_inner) => glBlitFramebuffer_inner(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter),
    None => gl_not_loaded("glBlitFramebuffer"),
  }
}
static glBlitFramebuffer_p: GlFnCell<glBlitFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlitFramebuffer_is_loaded() -> bool {
  unsafe { *glBlitFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlitFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlitFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlitFramebuffer_t>>(gl_ptr_filter(f(b"glBlitFramebuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlitFramebuffer_reset_ptr() {
  *glBlitFramebuffer_p.0.get() = None;
}
/// [glBlitNamedFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitNamedFramebuffer.xhtml)
/// * `readFramebuffer` class: framebuffer
/// * `drawFramebuffer` class: framebuffer
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlitNamedFramebuffer(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlitNamedFramebuffer_p.0.get() } {
    Some(glBlitNamedFramebuffer_inner) => glBlitNamedFramebuffer_inner(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter),
    None => gl_not_loaded("glBlitNamedFramebuffer"),
  }
}
static glBlitNamedFramebuffer_p: GlFnCell<glBlitNamedFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlitNamedFramebuffer_is_loaded() -> bool {
  unsafe { *glBlitNamedFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlitNamedFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlitNamedFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlitNamedFramebuffer_t>>(gl_ptr_filter(f(b"glBlitNamedFramebuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBlitNamedFramebuffer_reset_ptr() {
  *glBlitNamedFramebuffer_p.0.get() = None;
}
/// [glBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml)
/// * `target` group: BufferTargetARB
/// * `size` group: BufferSize
/// * `data` len: size
/// * `usage` group: BufferUsageARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBufferData(target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB) {
  #[allow(unused_unsafe)]
  match unsafe { *glBufferData_p.0.get() } {
    Some(glBufferData_inner) => glBufferData_inner(target, size, data, usage),
    None => gl_not_loaded("glBufferData"),
  }
}
static glBufferData_p: GlFnCell<glBufferData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBufferData_is_loaded() -> bool {
  unsafe { *glBufferData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBufferData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBufferData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBufferData_t>>(gl_ptr_filter(f(b"glBufferData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBufferData_reset_ptr() {
  *glBufferData_p.0.get() = None;
}
/// [glBufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferStorage.xhtml)
/// * `target` group: BufferStorageTarget
/// * `data` len: size
/// * `flags` group: BufferStorageMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBufferStorage(target: BufferStorageTarget, size: GLsizeiptr, data: *const void, flags: GLbitfield) {
  #[allow(unused_unsafe)]
  match unsafe { *glBufferStorage_p.0.get() } {
    Some(glBufferStorage_inner) => glBufferStorage_inner(target, size, data, flags),
    None => gl_not_loaded("glBufferStorage"),
  }
}
static glBufferStorage_p: GlFnCell<glBufferStorage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBufferStorage_is_loaded() -> bool {
  unsafe { *glBufferStorage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBufferStorage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBufferStorage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBufferStorage_t>>(gl_ptr_filter(f(b"glBufferStorage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBufferStorage_reset_ptr() {
  *glBufferStorage_p.0.get() = None;
}
/// [glBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferSubData.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `data` len: size
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBufferSubData(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glBufferSubData_p.0.get() } {
    Some(glBufferSubData_inner) => glBufferSubData_inner(target, offset, size, data),
    None => gl_not_loaded("glBufferSubData"),
  }
}
static glBufferSubData_p: GlFnCell<glBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBufferSubData_is_loaded() -> bool {
  unsafe { *glBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBufferSubData_t>>(gl_ptr_filter(f(b"glBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glBufferSubData_reset_ptr() {
  *glBufferSubData_p.0.get() = None;
}
/// [glCheckFramebufferStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckFramebufferStatus.xhtml)
/// * `target` group: FramebufferTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCheckFramebufferStatus(target: FramebufferTarget) -> FramebufferStatus {
  #[allow(unused_unsafe)]
  match unsafe { *glCheckFramebufferStatus_p.0.get() } {
    Some(glCheckFramebufferStatus_inner) => glCheckFramebufferStatus_inner(target),
    None => gl_not_loaded("glCheckFramebufferStatus"),
  }
}
static glCheckFramebufferStatus_p: GlFnCell<glCheckFramebufferStatus_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCheckFramebufferStatus_is_loaded() -> bool {
  unsafe { *glCheckFramebufferStatus_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCheckFramebufferStatus_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCheckFramebufferStatus_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCheckFramebufferStatus_t>>(gl_ptr_filter(f(b"glCheckFramebufferStatus\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCheckFramebufferStatus_reset_ptr() {
  *glCheckFramebufferStatus_p.0.get() = None;
}
/// [glCheckNamedFramebufferStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckNamedFramebufferStatus.xhtml)
/// * `framebuffer` class: framebuffer
/// * `target` group: FramebufferTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCheckNamedFramebufferStatus(framebuffer: GLuint, target: FramebufferTarget) -> FramebufferStatus {
  #[allow(unused_unsafe)]
  match unsafe { *glCheckNamedFramebufferStatus_p.0.get() } {
    Some(glCheckNamedFramebufferStatus_inner) => glCheckNamedFramebufferStatus_inner(framebuffer, target),
    None => gl_not_loaded("glCheckNamedFramebufferStatus"),
  }
}
static glCheckNamedFramebufferStatus_p: GlFnCell<glCheckNamedFramebufferStatus_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCheckNamedFramebufferStatus_is_loaded() -> bool {
  unsafe { *glCheckNamedFramebufferStatus_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCheckNamedFramebufferStatus_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCheckNamedFramebufferStatus_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCheckNamedFramebufferStatus_t>>(gl_ptr_filter(f(b"glCheckNamedFramebufferStatus\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCheckNamedFramebufferStatus_reset_ptr() {
  *glCheckNamedFramebufferStatus_p.0.get() = None;
}
/// [glClampColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClampColor.xhtml)
/// * `target` group: ClampColorTargetARB
/// * `clamp` group: ClampColorModeARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClampColor(target: ClampColorTargetARB, clamp: ClampColorModeARB) {
  #[allow(unused_unsafe)]
  match unsafe { *glClampColor_p.0.get() } {
    Some(glClampColor_inner) => glClampColor_inner(target, clamp),
    None => gl_not_loaded("glClampColor"),
  }
}
static glClampColor_p: GlFnCell<glClampColor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClampColor_is_loaded() -> bool {
  unsafe { *glClampColor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClampColor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClampColor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClampColor_t>>(gl_ptr_filter(f(b"glClampColor\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClampColor_reset_ptr() {
  *glClampColor_p.0.get() = None;
}
/// [glClear](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClear.xhtml)
/// * `mask` group: ClearBufferMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClear(mask: GLbitfield) {
  #[allow(unused_unsafe)]
  match unsafe { *glClear_p.0.get() } {
    Some(glClear_inner) => glClear_inner(mask),
    None => gl_not_loaded("glClear"),
  }
}
static glClear_p: GlFnCell<glClear_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClear_is_loaded() -> bool {
  unsafe { *glClear_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClear_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClear_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClear_t>>(gl_ptr_filter(f(b"glClear\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClear_reset_ptr() {
  *glClear_p.0.get() = None;
}
/// [glClearBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferData.xhtml)
/// * `target` group: BufferStorageTarget
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferData(target: BufferStorageTarget, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferData_p.0.get() } {
    Some(glClearBufferData_inner) => glClearBufferData_inner(target, internalformat, format, type_, data),
    None => gl_not_loaded("glClearBufferData"),
  }
}
static glClearBufferData_p: GlFnCell<glClearBufferData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferData_is_loaded() -> bool {
  unsafe { *glClearBufferData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferData_t>>(gl_ptr_filter(f(b"glClearBufferData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearBufferData_reset_ptr() {
  *glClearBufferData_p.0.get() = None;
}
/// [glClearBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferSubData.xhtml)
/// * `target` group: BufferTargetARB
/// * `internalformat` group: InternalFormat
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferSubData(target: BufferTargetARB, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferSubData_p.0.get() } {
    Some(glClearBufferSubData_inner) => glClearBufferSubData_inner(target, internalformat, offset, size, format, type_, data),
    None => gl_not_loaded("glClearBufferSubData"),
  }
}
static glClearBufferSubData_p: GlFnCell<glClearBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferSubData_is_loaded() -> bool {
  unsafe { *glClearBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferSubData_t>>(gl_ptr_filter(f(b"glClearBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearBufferSubData_reset_ptr() {
  *glClearBufferSubData_p.0.get() = None;
}
/// [glClearBufferfi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferfi.xhtml)
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferfi(buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferfi_p.0.get() } {
    Some(glClearBufferfi_inner) => glClearBufferfi_inner(buffer, drawbuffer, depth, stencil),
    None => gl_not_loaded("glClearBufferfi"),
  }
}
static glClearBufferfi_p: GlFnCell<glClearBufferfi_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferfi_is_loaded() -> bool {
  unsafe { *glClearBufferfi_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferfi_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferfi_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferfi_t>>(gl_ptr_filter(f(b"glClearBufferfi\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearBufferfi_reset_ptr() {
  *glClearBufferfi_p.0.get() = None;
}
/// [glClearBufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferfv.xhtml)
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferfv(buffer: Buffer, drawbuffer: GLint, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferfv_p.0.get() } {
    Some(glClearBufferfv_inner) => glClearBufferfv_inner(buffer, drawbuffer, value),
    None => gl_not_loaded("glClearBufferfv"),
  }
}
static glClearBufferfv_p: GlFnCell<glClearBufferfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferfv_is_loaded() -> bool {
  unsafe { *glClearBufferfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferfv_t>>(gl_ptr_filter(f(b"glClearBufferfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearBufferfv_reset_ptr() {
  *glClearBufferfv_p.0.get() = None;
}
/// [glClearBufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferiv.xhtml)
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferiv(buffer: Buffer, drawbuffer: GLint, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferiv_p.0.get() } {
    Some(glClearBufferiv_inner) => glClearBufferiv_inner(buffer, drawbuffer, value),
    None => gl_not_loaded("glClearBufferiv"),
  }
}
static glClearBufferiv_p: GlFnCell<glClearBufferiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferiv_is_loaded() -> bool {
  unsafe { *glClearBufferiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferiv_t>>(gl_ptr_filter(f(b"glClearBufferiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearBufferiv_reset_ptr() {
  *glClearBufferiv_p.0.get() = None;
}
/// [glClearBufferuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferuiv.xhtml)
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferuiv(buffer: Buffer, drawbuffer: GLint, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferuiv_p.0.get() } {
    Some(glClearBufferuiv_inner) => glClearBufferuiv_inner(buffer, drawbuffer, value),
    None => gl_not_loaded("glClearBufferuiv"),
  }
}
static glClearBufferuiv_p: GlFnCell<glClearBufferuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferuiv_is_loaded() -> bool {
  unsafe { *glClearBufferuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferuiv_t>>(gl_ptr_filter(f(b"glClearBufferuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearBufferuiv_reset_ptr() {
  *glClearBufferuiv_p.0.get() = None;
}
/// [glClearColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml)
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearColor_p.0.get() } {
    Some(glClearColor_inner) => glClearColor_inner(red, green, blue, alpha),
    None => gl_not_loaded("glClearColor"),
  }
}
static glClearColor_p: GlFnCell<glClearColor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearColor_is_loaded() -> bool {
  unsafe { *glClearColor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearColor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearColor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearColor_t>>(gl_ptr_filter(f(b"glClearColor\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearColor_reset_ptr() {
  *glClearColor_p.0.get() = None;
}
/// [glClearDepth](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepth.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearDepth(depth: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearDepth_p.0.get() } {
    Some(glClearDepth_inner) => glClearDepth_inner(depth),
    None => gl_not_loaded("glClearDepth"),
  }
}
static glClearDepth_p: GlFnCell<glClearDepth_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearDepth_is_loaded() -> bool {
  unsafe { *glClearDepth_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearDepth_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearDepth_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearDepth_t>>(gl_ptr_filter(f(b"glClearDepth\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearDepth_reset_ptr() {
  *glClearDepth_p.0.get() = None;
}
/// [glClearDepthf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepthf.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearDepthf(d: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearDepthf_p.0.get() } {
    Some(glClearDepthf_inner) => glClearDepthf_inner(d),
    None => gl_not_loaded("glClearDepthf"),
  }
}
static glClearDepthf_p: GlFnCell<glClearDepthf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearDepthf_is_loaded() -> bool {
  unsafe { *glClearDepthf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearDepthf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearDepthf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearDepthf_t>>(gl_ptr_filter(f(b"glClearDepthf\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearDepthf_reset_ptr() {
  *glClearDepthf_p.0.get() = None;
}
/// [glClearNamedBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedBufferData.xhtml)
/// * `buffer` class: buffer
/// * `internalformat` group: InternalFormat
/// * `format` group: PixelFormat
/// * `type` group: PixelType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearNamedBufferData(buffer: GLuint, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearNamedBufferData_p.0.get() } {
    Some(glClearNamedBufferData_inner) => glClearNamedBufferData_inner(buffer, internalformat, format, type_, data),
    None => gl_not_loaded("glClearNamedBufferData"),
  }
}
static glClearNamedBufferData_p: GlFnCell<glClearNamedBufferData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearNamedBufferData_is_loaded() -> bool {
  unsafe { *glClearNamedBufferData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearNamedBufferData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearNamedBufferData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearNamedBufferData_t>>(gl_ptr_filter(f(b"glClearNamedBufferData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearNamedBufferData_reset_ptr() {
  *glClearNamedBufferData_p.0.get() = None;
}
/// [glClearNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedBufferSubData.xhtml)
/// * `buffer` class: buffer
/// * `internalformat` group: InternalFormat
/// * `size` group: BufferSize
/// * `format` group: PixelFormat
/// * `type` group: PixelType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearNamedBufferSubData(buffer: GLuint, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearNamedBufferSubData_p.0.get() } {
    Some(glClearNamedBufferSubData_inner) => glClearNamedBufferSubData_inner(buffer, internalformat, offset, size, format, type_, data),
    None => gl_not_loaded("glClearNamedBufferSubData"),
  }
}
static glClearNamedBufferSubData_p: GlFnCell<glClearNamedBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearNamedBufferSubData_is_loaded() -> bool {
  unsafe { *glClearNamedBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearNamedBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearNamedBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearNamedBufferSubData_t>>(gl_ptr_filter(f(b"glClearNamedBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearNamedBufferSubData_reset_ptr() {
  *glClearNamedBufferSubData_p.0.get() = None;
}
/// [glClearNamedFramebufferfi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferfi.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearNamedFramebufferfi(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearNamedFramebufferfi_p.0.get() } {
    Some(glClearNamedFramebufferfi_inner) => glClearNamedFramebufferfi_inner(framebuffer, buffer, drawbuffer, depth, stencil),
    None => gl_not_loaded("glClearNamedFramebufferfi"),
  }
}
static glClearNamedFramebufferfi_p: GlFnCell<glClearNamedFramebufferfi_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearNamedFramebufferfi_is_loaded() -> bool {
  unsafe { *glClearNamedFramebufferfi_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearNamedFramebufferfi_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearNamedFramebufferfi_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearNamedFramebufferfi_t>>(gl_ptr_filter(f(b"glClearNamedFramebufferfi\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearNamedFramebufferfi_reset_ptr() {
  *glClearNamedFramebufferfi_p.0.get() = None;
}
/// [glClearNamedFramebufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferfv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearNamedFramebufferfv(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearNamedFramebufferfv_p.0.get() } {
    Some(glClearNamedFramebufferfv_inner) => glClearNamedFramebufferfv_inner(framebuffer, buffer, drawbuffer, value),
    None => gl_not_loaded("glClearNamedFramebufferfv"),
  }
}
static glClearNamedFramebufferfv_p: GlFnCell<glClearNamedFramebufferfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearNamedFramebufferfv_is_loaded() -> bool {
  unsafe { *glClearNamedFramebufferfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearNamedFramebufferfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearNamedFramebufferfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearNamedFramebufferfv_t>>(gl_ptr_filter(f(b"glClearNamedFramebufferfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearNamedFramebufferfv_reset_ptr() {
  *glClearNamedFramebufferfv_p.0.get() = None;
}
/// [glClearNamedFramebufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferiv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearNamedFramebufferiv(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearNamedFramebufferiv_p.0.get() } {
    Some(glClearNamedFramebufferiv_inner) => glClearNamedFramebufferiv_inner(framebuffer, buffer, drawbuffer, value),
    None => gl_not_loaded("glClearNamedFramebufferiv"),
  }
}
static glClearNamedFramebufferiv_p: GlFnCell<glClearNamedFramebufferiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearNamedFramebufferiv_is_loaded() -> bool {
  unsafe { *glClearNamedFramebufferiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearNamedFramebufferiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearNamedFramebufferiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearNamedFramebufferiv_t>>(gl_ptr_filter(f(b"glClearNamedFramebufferiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearNamedFramebufferiv_reset_ptr() {
  *glClearNamedFramebufferiv_p.0.get() = None;
}
/// [glClearNamedFramebufferuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferuiv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buffer` group: Buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearNamedFramebufferuiv(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearNamedFramebufferuiv_p.0.get() } {
    Some(glClearNamedFramebufferuiv_inner) => glClearNamedFramebufferuiv_inner(framebuffer, buffer, drawbuffer, value),
    None => gl_not_loaded("glClearNamedFramebufferuiv"),
  }
}
static glClearNamedFramebufferuiv_p: GlFnCell<glClearNamedFramebufferuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearNamedFramebufferuiv_is_loaded() -> bool {
  unsafe { *glClearNamedFramebufferuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearNamedFramebufferuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearNamedFramebufferuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearNamedFramebufferuiv_t>>(gl_ptr_filter(f(b"glClearNamedFramebufferuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearNamedFramebufferuiv_reset_ptr() {
  *glClearNamedFramebufferuiv_p.0.get() = None;
}
/// [glClearStencil](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearStencil.xhtml)
/// * `s` group: StencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearStencil(s: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearStencil_p.0.get() } {
    Some(glClearStencil_inner) => glClearStencil_inner(s),
    None => gl_not_loaded("glClearStencil"),
  }
}
static glClearStencil_p: GlFnCell<glClearStencil_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearStencil_is_loaded() -> bool {
  unsafe { *glClearStencil_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearStencil_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearStencil_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearStencil_t>>(gl_ptr_filter(f(b"glClearStencil\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearStencil_reset_ptr() {
  *glClearStencil_p.0.get() = None;
}
/// [glClearTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearTexImage.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearTexImage(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearTexImage_p.0.get() } {
    Some(glClearTexImage_inner) => glClearTexImage_inner(texture, level, format, type_, data),
    None => gl_not_loaded("glClearTexImage"),
  }
}
static glClearTexImage_p: GlFnCell<glClearTexImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearTexImage_is_loaded() -> bool {
  unsafe { *glClearTexImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearTexImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearTexImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearTexImage_t>>(gl_ptr_filter(f(b"glClearTexImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearTexImage_reset_ptr() {
  *glClearTexImage_p.0.get() = None;
}
/// [glClearTexSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearTexSubImage.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: COMPSIZE(format,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearTexSubImage_p.0.get() } {
    Some(glClearTexSubImage_inner) => glClearTexSubImage_inner(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data),
    None => gl_not_loaded("glClearTexSubImage"),
  }
}
static glClearTexSubImage_p: GlFnCell<glClearTexSubImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearTexSubImage_is_loaded() -> bool {
  unsafe { *glClearTexSubImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearTexSubImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearTexSubImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearTexSubImage_t>>(gl_ptr_filter(f(b"glClearTexSubImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClearTexSubImage_reset_ptr() {
  *glClearTexSubImage_p.0.get() = None;
}
/// [glClientWaitSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientWaitSync.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncObjectMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus {
  #[allow(unused_unsafe)]
  match unsafe { *glClientWaitSync_p.0.get() } {
    Some(glClientWaitSync_inner) => glClientWaitSync_inner(sync, flags, timeout),
    None => gl_not_loaded("glClientWaitSync"),
  }
}
static glClientWaitSync_p: GlFnCell<glClientWaitSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClientWaitSync_is_loaded() -> bool {
  unsafe { *glClientWaitSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClientWaitSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClientWaitSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClientWaitSync_t>>(gl_ptr_filter(f(b"glClientWaitSync\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClientWaitSync_reset_ptr() {
  *glClientWaitSync_p.0.get() = None;
}
/// [glClipControl](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipControl.xhtml)
/// * `origin` group: ClipControlOrigin
/// * `depth` group: ClipControlDepth
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClipControl(origin: ClipControlOrigin, depth: ClipControlDepth) {
  #[allow(unused_unsafe)]
  match unsafe { *glClipControl_p.0.get() } {
    Some(glClipControl_inner) => glClipControl_inner(origin, depth),
    None => gl_not_loaded("glClipControl"),
  }
}
static glClipControl_p: GlFnCell<glClipControl_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClipControl_is_loaded() -> bool {
  unsafe { *glClipControl_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClipControl_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClipControl_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClipControl_t>>(gl_ptr_filter(f(b"glClipControl\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glClipControl_reset_ptr() {
  *glClipControl_p.0.get() = None;
}
/// [glColorMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMask.xhtml)
/// * `red` group: Boolean
/// * `green` group: Boolean
/// * `blue` group: Boolean
/// * `alpha` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glColorMask_p.0.get() } {
    Some(glColorMask_inner) => glColorMask_inner(red, green, blue, alpha),
    None => gl_not_loaded("glColorMask"),
  }
}
static glColorMask_p: GlFnCell<glColorMask_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glColorMask_is_loaded() -> bool {
  unsafe { *glColorMask_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glColorMask_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glColorMask_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glColorMask_t>>(gl_ptr_filter(f(b"glColorMask\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glColorMask_reset_ptr() {
  *glColorMask_p.0.get() = None;
}
/// [glColorMaski](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMaski.xhtml)
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glColorMaski_p.0.get() } {
    Some(glColorMaski_inner) => glColorMaski_inner(index, r, g, b, a),
    None => gl_not_loaded("glColorMaski"),
  }
}
static glColorMaski_p: GlFnCell<glColorMaski_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glColorMaski_is_loaded() -> bool {
  unsafe { *glColorMaski_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glColorMaski_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glColorMaski_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glColorMaski_t>>(gl_ptr_filter(f(b"glColorMaski\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glColorMaski_reset_ptr() {
  *glColorMaski_p.0.get() = None;
}
/// [glColorP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP3ui.xhtml)
/// * `type` group: ColorPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glColorP3ui(type_: ColorPointerType, color: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glColorP3ui_p.0.get() } {
    Some(glColorP3ui_inner) => glColorP3ui_inner(type_, color),
    None => gl_not_loaded("glColorP3ui"),
  }
}
static glColorP3ui_p: GlFnCell<glColorP3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glColorP3ui_is_loaded() -> bool {
  unsafe { *glColorP3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glColorP3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glColorP3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glColorP3ui_t>>(gl_ptr_filter(f(b"glColorP3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glColorP3ui_reset_ptr() {
  *glColorP3ui_p.0.get() = None;
}
/// [glColorP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP3uiv.xhtml)
/// * `type` group: ColorPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glColorP3uiv(type_: ColorPointerType, color: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glColorP3uiv_p.0.get() } {
    Some(glColorP3uiv_inner) => glColorP3uiv_inner(type_, color),
    None => gl_not_loaded("glColorP3uiv"),
  }
}
static glColorP3uiv_p: GlFnCell<glColorP3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glColorP3uiv_is_loaded() -> bool {
  unsafe { *glColorP3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glColorP3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glColorP3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glColorP3uiv_t>>(gl_ptr_filter(f(b"glColorP3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glColorP3uiv_reset_ptr() {
  *glColorP3uiv_p.0.get() = None;
}
/// [glColorP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP4ui.xhtml)
/// * `type` group: ColorPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glColorP4ui(type_: ColorPointerType, color: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glColorP4ui_p.0.get() } {
    Some(glColorP4ui_inner) => glColorP4ui_inner(type_, color),
    None => gl_not_loaded("glColorP4ui"),
  }
}
static glColorP4ui_p: GlFnCell<glColorP4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glColorP4ui_is_loaded() -> bool {
  unsafe { *glColorP4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glColorP4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glColorP4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glColorP4ui_t>>(gl_ptr_filter(f(b"glColorP4ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glColorP4ui_reset_ptr() {
  *glColorP4ui_p.0.get() = None;
}
/// [glColorP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP4uiv.xhtml)
/// * `type` group: ColorPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glColorP4uiv(type_: ColorPointerType, color: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glColorP4uiv_p.0.get() } {
    Some(glColorP4uiv_inner) => glColorP4uiv_inner(type_, color),
    None => gl_not_loaded("glColorP4uiv"),
  }
}
static glColorP4uiv_p: GlFnCell<glColorP4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glColorP4uiv_is_loaded() -> bool {
  unsafe { *glColorP4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glColorP4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glColorP4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glColorP4uiv_t>>(gl_ptr_filter(f(b"glColorP4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glColorP4uiv_reset_ptr() {
  *glColorP4uiv_p.0.get() = None;
}
/// [glCompileShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml)
/// * `shader` class: shader
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompileShader(shader: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompileShader_p.0.get() } {
    Some(glCompileShader_inner) => glCompileShader_inner(shader),
    None => gl_not_loaded("glCompileShader"),
  }
}
static glCompileShader_p: GlFnCell<glCompileShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompileShader_is_loaded() -> bool {
  unsafe { *glCompileShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompileShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompileShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompileShader_t>>(gl_ptr_filter(f(b"glCompileShader\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompileShader_reset_ptr() {
  *glCompileShader_p.0.get() = None;
}
/// [glCompressedTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexImage1D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexImage1D_p.0.get() } {
    Some(glCompressedTexImage1D_inner) => glCompressedTexImage1D_inner(target, level, internalformat, width, border, imageSize, data),
    None => gl_not_loaded("glCompressedTexImage1D"),
  }
}
static glCompressedTexImage1D_p: GlFnCell<glCompressedTexImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexImage1D_is_loaded() -> bool {
  unsafe { *glCompressedTexImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexImage1D_t>>(gl_ptr_filter(f(b"glCompressedTexImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage1D_reset_ptr() {
  *glCompressedTexImage1D_p.0.get() = None;
}
/// [glCompressedTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexImage2D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexImage2D_p.0.get() } {
    Some(glCompressedTexImage2D_inner) => glCompressedTexImage2D_inner(target, level, internalformat, width, height, border, imageSize, data),
    None => gl_not_loaded("glCompressedTexImage2D"),
  }
}
static glCompressedTexImage2D_p: GlFnCell<glCompressedTexImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexImage2D_is_loaded() -> bool {
  unsafe { *glCompressedTexImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexImage2D_t>>(gl_ptr_filter(f(b"glCompressedTexImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage2D_reset_ptr() {
  *glCompressedTexImage2D_p.0.get() = None;
}
/// [glCompressedTexImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexImage3D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexImage3D_p.0.get() } {
    Some(glCompressedTexImage3D_inner) => glCompressedTexImage3D_inner(target, level, internalformat, width, height, depth, border, imageSize, data),
    None => gl_not_loaded("glCompressedTexImage3D"),
  }
}
static glCompressedTexImage3D_p: GlFnCell<glCompressedTexImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexImage3D_is_loaded() -> bool {
  unsafe { *glCompressedTexImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexImage3D_t>>(gl_ptr_filter(f(b"glCompressedTexImage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage3D_reset_ptr() {
  *glCompressedTexImage3D_p.0.get() = None;
}
/// [glCompressedTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexSubImage1D_p.0.get() } {
    Some(glCompressedTexSubImage1D_inner) => glCompressedTexSubImage1D_inner(target, level, xoffset, width, format, imageSize, data),
    None => gl_not_loaded("glCompressedTexSubImage1D"),
  }
}
static glCompressedTexSubImage1D_p: GlFnCell<glCompressedTexSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexSubImage1D_is_loaded() -> bool {
  unsafe { *glCompressedTexSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexSubImage1D_t>>(gl_ptr_filter(f(b"glCompressedTexSubImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage1D_reset_ptr() {
  *glCompressedTexSubImage1D_p.0.get() = None;
}
/// [glCompressedTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexSubImage2D_p.0.get() } {
    Some(glCompressedTexSubImage2D_inner) => glCompressedTexSubImage2D_inner(target, level, xoffset, yoffset, width, height, format, imageSize, data),
    None => gl_not_loaded("glCompressedTexSubImage2D"),
  }
}
static glCompressedTexSubImage2D_p: GlFnCell<glCompressedTexSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexSubImage2D_is_loaded() -> bool {
  unsafe { *glCompressedTexSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexSubImage2D_t>>(gl_ptr_filter(f(b"glCompressedTexSubImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage2D_reset_ptr() {
  *glCompressedTexSubImage2D_p.0.get() = None;
}
/// [glCompressedTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexSubImage3D_p.0.get() } {
    Some(glCompressedTexSubImage3D_inner) => glCompressedTexSubImage3D_inner(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data),
    None => gl_not_loaded("glCompressedTexSubImage3D"),
  }
}
static glCompressedTexSubImage3D_p: GlFnCell<glCompressedTexSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexSubImage3D_is_loaded() -> bool {
  unsafe { *glCompressedTexSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexSubImage3D_t>>(gl_ptr_filter(f(b"glCompressedTexSubImage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage3D_reset_ptr() {
  *glCompressedTexSubImage3D_p.0.get() = None;
}
/// [glCompressedTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage1D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTextureSubImage1D_p.0.get() } {
    Some(glCompressedTextureSubImage1D_inner) => glCompressedTextureSubImage1D_inner(texture, level, xoffset, width, format, imageSize, data),
    None => gl_not_loaded("glCompressedTextureSubImage1D"),
  }
}
static glCompressedTextureSubImage1D_p: GlFnCell<glCompressedTextureSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTextureSubImage1D_is_loaded() -> bool {
  unsafe { *glCompressedTextureSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTextureSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTextureSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTextureSubImage1D_t>>(gl_ptr_filter(f(b"glCompressedTextureSubImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTextureSubImage1D_reset_ptr() {
  *glCompressedTextureSubImage1D_p.0.get() = None;
}
/// [glCompressedTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage2D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTextureSubImage2D_p.0.get() } {
    Some(glCompressedTextureSubImage2D_inner) => glCompressedTextureSubImage2D_inner(texture, level, xoffset, yoffset, width, height, format, imageSize, data),
    None => gl_not_loaded("glCompressedTextureSubImage2D"),
  }
}
static glCompressedTextureSubImage2D_p: GlFnCell<glCompressedTextureSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTextureSubImage2D_is_loaded() -> bool {
  unsafe { *glCompressedTextureSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTextureSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTextureSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTextureSubImage2D_t>>(gl_ptr_filter(f(b"glCompressedTextureSubImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTextureSubImage2D_reset_ptr() {
  *glCompressedTextureSubImage2D_p.0.get() = None;
}
/// [glCompressedTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage3D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTextureSubImage3D_p.0.get() } {
    Some(glCompressedTextureSubImage3D_inner) => glCompressedTextureSubImage3D_inner(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data),
    None => gl_not_loaded("glCompressedTextureSubImage3D"),
  }
}
static glCompressedTextureSubImage3D_p: GlFnCell<glCompressedTextureSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTextureSubImage3D_is_loaded() -> bool {
  unsafe { *glCompressedTextureSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTextureSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTextureSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTextureSubImage3D_t>>(gl_ptr_filter(f(b"glCompressedTextureSubImage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCompressedTextureSubImage3D_reset_ptr() {
  *glCompressedTextureSubImage3D_p.0.get() = None;
}
/// [glCopyBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyBufferSubData.xhtml)
/// * `readTarget` group: CopyBufferSubDataTarget
/// * `writeTarget` group: CopyBufferSubDataTarget
/// * `readOffset` group: BufferOffset
/// * `writeOffset` group: BufferOffset
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyBufferSubData(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyBufferSubData_p.0.get() } {
    Some(glCopyBufferSubData_inner) => glCopyBufferSubData_inner(readTarget, writeTarget, readOffset, writeOffset, size),
    None => gl_not_loaded("glCopyBufferSubData"),
  }
}
static glCopyBufferSubData_p: GlFnCell<glCopyBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyBufferSubData_is_loaded() -> bool {
  unsafe { *glCopyBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyBufferSubData_t>>(gl_ptr_filter(f(b"glCopyBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyBufferSubData_reset_ptr() {
  *glCopyBufferSubData_p.0.get() = None;
}
/// [glCopyImageSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyImageSubData.xhtml)
/// * `srcTarget` group: CopyImageSubDataTarget
/// * `dstTarget` group: CopyImageSubDataTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyImageSubData(srcName: GLuint, srcTarget: CopyImageSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyImageSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyImageSubData_p.0.get() } {
    Some(glCopyImageSubData_inner) => glCopyImageSubData_inner(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth),
    None => gl_not_loaded("glCopyImageSubData"),
  }
}
static glCopyImageSubData_p: GlFnCell<glCopyImageSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyImageSubData_is_loaded() -> bool {
  unsafe { *glCopyImageSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyImageSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyImageSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyImageSubData_t>>(gl_ptr_filter(f(b"glCopyImageSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyImageSubData_reset_ptr() {
  *glCopyImageSubData_p.0.get() = None;
}
/// [glCopyNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyNamedBufferSubData.xhtml)
/// * `readBuffer` class: buffer
/// * `writeBuffer` class: buffer
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyNamedBufferSubData(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyNamedBufferSubData_p.0.get() } {
    Some(glCopyNamedBufferSubData_inner) => glCopyNamedBufferSubData_inner(readBuffer, writeBuffer, readOffset, writeOffset, size),
    None => gl_not_loaded("glCopyNamedBufferSubData"),
  }
}
static glCopyNamedBufferSubData_p: GlFnCell<glCopyNamedBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyNamedBufferSubData_is_loaded() -> bool {
  unsafe { *glCopyNamedBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyNamedBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyNamedBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyNamedBufferSubData_t>>(gl_ptr_filter(f(b"glCopyNamedBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyNamedBufferSubData_reset_ptr() {
  *glCopyNamedBufferSubData_p.0.get() = None;
}
/// [glCopyTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexImage1D(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexImage1D_p.0.get() } {
    Some(glCopyTexImage1D_inner) => glCopyTexImage1D_inner(target, level, internalformat, x, y, width, border),
    None => gl_not_loaded("glCopyTexImage1D"),
  }
}
static glCopyTexImage1D_p: GlFnCell<glCopyTexImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexImage1D_is_loaded() -> bool {
  unsafe { *glCopyTexImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexImage1D_t>>(gl_ptr_filter(f(b"glCopyTexImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyTexImage1D_reset_ptr() {
  *glCopyTexImage1D_p.0.get() = None;
}
/// [glCopyTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexImage2D(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexImage2D_p.0.get() } {
    Some(glCopyTexImage2D_inner) => glCopyTexImage2D_inner(target, level, internalformat, x, y, width, height, border),
    None => gl_not_loaded("glCopyTexImage2D"),
  }
}
static glCopyTexImage2D_p: GlFnCell<glCopyTexImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexImage2D_is_loaded() -> bool {
  unsafe { *glCopyTexImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexImage2D_t>>(gl_ptr_filter(f(b"glCopyTexImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyTexImage2D_reset_ptr() {
  *glCopyTexImage2D_p.0.get() = None;
}
/// [glCopyTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexSubImage1D_p.0.get() } {
    Some(glCopyTexSubImage1D_inner) => glCopyTexSubImage1D_inner(target, level, xoffset, x, y, width),
    None => gl_not_loaded("glCopyTexSubImage1D"),
  }
}
static glCopyTexSubImage1D_p: GlFnCell<glCopyTexSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexSubImage1D_is_loaded() -> bool {
  unsafe { *glCopyTexSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexSubImage1D_t>>(gl_ptr_filter(f(b"glCopyTexSubImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage1D_reset_ptr() {
  *glCopyTexSubImage1D_p.0.get() = None;
}
/// [glCopyTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexSubImage2D_p.0.get() } {
    Some(glCopyTexSubImage2D_inner) => glCopyTexSubImage2D_inner(target, level, xoffset, yoffset, x, y, width, height),
    None => gl_not_loaded("glCopyTexSubImage2D"),
  }
}
static glCopyTexSubImage2D_p: GlFnCell<glCopyTexSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexSubImage2D_is_loaded() -> bool {
  unsafe { *glCopyTexSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexSubImage2D_t>>(gl_ptr_filter(f(b"glCopyTexSubImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage2D_reset_ptr() {
  *glCopyTexSubImage2D_p.0.get() = None;
}
/// [glCopyTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexSubImage3D_p.0.get() } {
    Some(glCopyTexSubImage3D_inner) => glCopyTexSubImage3D_inner(target, level, xoffset, yoffset, zoffset, x, y, width, height),
    None => gl_not_loaded("glCopyTexSubImage3D"),
  }
}
static glCopyTexSubImage3D_p: GlFnCell<glCopyTexSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexSubImage3D_is_loaded() -> bool {
  unsafe { *glCopyTexSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexSubImage3D_t>>(gl_ptr_filter(f(b"glCopyTexSubImage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage3D_reset_ptr() {
  *glCopyTexSubImage3D_p.0.get() = None;
}
/// [glCopyTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage1D.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTextureSubImage1D_p.0.get() } {
    Some(glCopyTextureSubImage1D_inner) => glCopyTextureSubImage1D_inner(texture, level, xoffset, x, y, width),
    None => gl_not_loaded("glCopyTextureSubImage1D"),
  }
}
static glCopyTextureSubImage1D_p: GlFnCell<glCopyTextureSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTextureSubImage1D_is_loaded() -> bool {
  unsafe { *glCopyTextureSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTextureSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTextureSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTextureSubImage1D_t>>(gl_ptr_filter(f(b"glCopyTextureSubImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyTextureSubImage1D_reset_ptr() {
  *glCopyTextureSubImage1D_p.0.get() = None;
}
/// [glCopyTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage2D.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTextureSubImage2D_p.0.get() } {
    Some(glCopyTextureSubImage2D_inner) => glCopyTextureSubImage2D_inner(texture, level, xoffset, yoffset, x, y, width, height),
    None => gl_not_loaded("glCopyTextureSubImage2D"),
  }
}
static glCopyTextureSubImage2D_p: GlFnCell<glCopyTextureSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTextureSubImage2D_is_loaded() -> bool {
  unsafe { *glCopyTextureSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTextureSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTextureSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTextureSubImage2D_t>>(gl_ptr_filter(f(b"glCopyTextureSubImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyTextureSubImage2D_reset_ptr() {
  *glCopyTextureSubImage2D_p.0.get() = None;
}
/// [glCopyTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage3D.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTextureSubImage3D_p.0.get() } {
    Some(glCopyTextureSubImage3D_inner) => glCopyTextureSubImage3D_inner(texture, level, xoffset, yoffset, zoffset, x, y, width, height),
    None => gl_not_loaded("glCopyTextureSubImage3D"),
  }
}
static glCopyTextureSubImage3D_p: GlFnCell<glCopyTextureSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTextureSubImage3D_is_loaded() -> bool {
  unsafe { *glCopyTextureSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTextureSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTextureSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTextureSubImage3D_t>>(gl_ptr_filter(f(b"glCopyTextureSubImage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCopyTextureSubImage3D_reset_ptr() {
  *glCopyTextureSubImage3D_p.0.get() = None;
}
/// [glCreateBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateBuffers.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateBuffers(n: GLsizei, buffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateBuffers_p.0.get() } {
    Some(glCreateBuffers_inner) => glCreateBuffers_inner(n, buffers),
    None => gl_not_loaded("glCreateBuffers"),
  }
}
static glCreateBuffers_p: GlFnCell<glCreateBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateBuffers_is_loaded() -> bool {
  unsafe { *glCreateBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateBuffers_t>>(gl_ptr_filter(f(b"glCreateBuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateBuffers_reset_ptr() {
  *glCreateBuffers_p.0.get() = None;
}
/// [glCreateFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateFramebuffers.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateFramebuffers_p.0.get() } {
    Some(glCreateFramebuffers_inner) => glCreateFramebuffers_inner(n, framebuffers),
    None => gl_not_loaded("glCreateFramebuffers"),
  }
}
static glCreateFramebuffers_p: GlFnCell<glCreateFramebuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateFramebuffers_is_loaded() -> bool {
  unsafe { *glCreateFramebuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateFramebuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateFramebuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateFramebuffers_t>>(gl_ptr_filter(f(b"glCreateFramebuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateFramebuffers_reset_ptr() {
  *glCreateFramebuffers_p.0.get() = None;
}
/// [glCreateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateProgram() -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateProgram_p.0.get() } {
    Some(glCreateProgram_inner) => glCreateProgram_inner(),
    None => gl_not_loaded("glCreateProgram"),
  }
}
static glCreateProgram_p: GlFnCell<glCreateProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateProgram_is_loaded() -> bool {
  unsafe { *glCreateProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateProgram_t>>(gl_ptr_filter(f(b"glCreateProgram\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateProgram_reset_ptr() {
  *glCreateProgram_p.0.get() = None;
}
/// [glCreateProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgramPipelines.xhtml)
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateProgramPipelines_p.0.get() } {
    Some(glCreateProgramPipelines_inner) => glCreateProgramPipelines_inner(n, pipelines),
    None => gl_not_loaded("glCreateProgramPipelines"),
  }
}
static glCreateProgramPipelines_p: GlFnCell<glCreateProgramPipelines_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateProgramPipelines_is_loaded() -> bool {
  unsafe { *glCreateProgramPipelines_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateProgramPipelines_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateProgramPipelines_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateProgramPipelines_t>>(gl_ptr_filter(f(b"glCreateProgramPipelines\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateProgramPipelines_reset_ptr() {
  *glCreateProgramPipelines_p.0.get() = None;
}
/// [glCreateQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateQueries.xhtml)
/// * `target` group: QueryTarget
/// * `ids` class: query
/// * `ids` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateQueries(target: QueryTarget, n: GLsizei, ids: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateQueries_p.0.get() } {
    Some(glCreateQueries_inner) => glCreateQueries_inner(target, n, ids),
    None => gl_not_loaded("glCreateQueries"),
  }
}
static glCreateQueries_p: GlFnCell<glCreateQueries_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateQueries_is_loaded() -> bool {
  unsafe { *glCreateQueries_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateQueries_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateQueries_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateQueries_t>>(gl_ptr_filter(f(b"glCreateQueries\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateQueries_reset_ptr() {
  *glCreateQueries_p.0.get() = None;
}
/// [glCreateRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateRenderbuffers.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateRenderbuffers_p.0.get() } {
    Some(glCreateRenderbuffers_inner) => glCreateRenderbuffers_inner(n, renderbuffers),
    None => gl_not_loaded("glCreateRenderbuffers"),
  }
}
static glCreateRenderbuffers_p: GlFnCell<glCreateRenderbuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateRenderbuffers_is_loaded() -> bool {
  unsafe { *glCreateRenderbuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateRenderbuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateRenderbuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateRenderbuffers_t>>(gl_ptr_filter(f(b"glCreateRenderbuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateRenderbuffers_reset_ptr() {
  *glCreateRenderbuffers_p.0.get() = None;
}
/// [glCreateSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateSamplers.xhtml)
/// * `samplers` class: sampler
/// * `samplers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateSamplers(n: GLsizei, samplers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateSamplers_p.0.get() } {
    Some(glCreateSamplers_inner) => glCreateSamplers_inner(n, samplers),
    None => gl_not_loaded("glCreateSamplers"),
  }
}
static glCreateSamplers_p: GlFnCell<glCreateSamplers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateSamplers_is_loaded() -> bool {
  unsafe { *glCreateSamplers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateSamplers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateSamplers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateSamplers_t>>(gl_ptr_filter(f(b"glCreateSamplers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateSamplers_reset_ptr() {
  *glCreateSamplers_p.0.get() = None;
}
/// [glCreateShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml)
/// * `type` group: ShaderType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateShader(type_: ShaderType) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateShader_p.0.get() } {
    Some(glCreateShader_inner) => glCreateShader_inner(type_),
    None => gl_not_loaded("glCreateShader"),
  }
}
static glCreateShader_p: GlFnCell<glCreateShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateShader_is_loaded() -> bool {
  unsafe { *glCreateShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateShader_t>>(gl_ptr_filter(f(b"glCreateShader\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateShader_reset_ptr() {
  *glCreateShader_p.0.get() = None;
}
/// [glCreateShaderProgramv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShaderProgramv.xhtml)
/// * `type` group: ShaderType
/// * `strings` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateShaderProgramv(type_: ShaderType, count: GLsizei, strings: *const *const GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateShaderProgramv_p.0.get() } {
    Some(glCreateShaderProgramv_inner) => glCreateShaderProgramv_inner(type_, count, strings),
    None => gl_not_loaded("glCreateShaderProgramv"),
  }
}
static glCreateShaderProgramv_p: GlFnCell<glCreateShaderProgramv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateShaderProgramv_is_loaded() -> bool {
  unsafe { *glCreateShaderProgramv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateShaderProgramv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateShaderProgramv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateShaderProgramv_t>>(gl_ptr_filter(f(b"glCreateShaderProgramv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateShaderProgramv_reset_ptr() {
  *glCreateShaderProgramv_p.0.get() = None;
}
/// [glCreateTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateTextures.xhtml)
/// * `target` group: TextureTarget
/// * `textures` class: texture
/// * `textures` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateTextures(target: TextureTarget, n: GLsizei, textures: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateTextures_p.0.get() } {
    Some(glCreateTextures_inner) => glCreateTextures_inner(target, n, textures),
    None => gl_not_loaded("glCreateTextures"),
  }
}
static glCreateTextures_p: GlFnCell<glCreateTextures_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateTextures_is_loaded() -> bool {
  unsafe { *glCreateTextures_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateTextures_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateTextures_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateTextures_t>>(gl_ptr_filter(f(b"glCreateTextures\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateTextures_reset_ptr() {
  *glCreateTextures_p.0.get() = None;
}
/// [glCreateTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateTransformFeedbacks.xhtml)
/// * `ids` class: transform feedback
/// * `ids` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateTransformFeedbacks_p.0.get() } {
    Some(glCreateTransformFeedbacks_inner) => glCreateTransformFeedbacks_inner(n, ids),
    None => gl_not_loaded("glCreateTransformFeedbacks"),
  }
}
static glCreateTransformFeedbacks_p: GlFnCell<glCreateTransformFeedbacks_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateTransformFeedbacks_is_loaded() -> bool {
  unsafe { *glCreateTransformFeedbacks_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateTransformFeedbacks_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateTransformFeedbacks_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateTransformFeedbacks_t>>(gl_ptr_filter(f(b"glCreateTransformFeedbacks\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateTransformFeedbacks_reset_ptr() {
  *glCreateTransformFeedbacks_p.0.get() = None;
}
/// [glCreateVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateVertexArrays.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCreateVertexArrays(n: GLsizei, arrays: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateVertexArrays_p.0.get() } {
    Some(glCreateVertexArrays_inner) => glCreateVertexArrays_inner(n, arrays),
    None => gl_not_loaded("glCreateVertexArrays"),
  }
}
static glCreateVertexArrays_p: GlFnCell<glCreateVertexArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateVertexArrays_is_loaded() -> bool {
  unsafe { *glCreateVertexArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateVertexArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateVertexArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateVertexArrays_t>>(gl_ptr_filter(f(b"glCreateVertexArrays\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCreateVertexArrays_reset_ptr() {
  *glCreateVertexArrays_p.0.get() = None;
}
/// [glCullFace](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCullFace.xhtml)
/// * `mode` group: CullFaceMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCullFace(mode: CullFaceMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glCullFace_p.0.get() } {
    Some(glCullFace_inner) => glCullFace_inner(mode),
    None => gl_not_loaded("glCullFace"),
  }
}
static glCullFace_p: GlFnCell<glCullFace_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCullFace_is_loaded() -> bool {
  unsafe { *glCullFace_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCullFace_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCullFace_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCullFace_t>>(gl_ptr_filter(f(b"glCullFace\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glCullFace_reset_ptr() {
  *glCullFace_p.0.get() = None;
}
/// [glDebugMessageCallback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallback.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageCallback(callback: GLDEBUGPROC, userParam: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageCallback_p.0.get() } {
    Some(glDebugMessageCallback_inner) => glDebugMessageCallback_inner(callback, userParam),
    None => gl_not_loaded("glDebugMessageCallback"),
  }
}
static glDebugMessageCallback_p: GlFnCell<glDebugMessageCallback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageCallback_is_loaded() -> bool {
  unsafe { *glDebugMessageCallback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageCallback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageCallback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageCallback_t>>(gl_ptr_filter(f(b"glDebugMessageCallback\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageCallback_reset_ptr() {
  *glDebugMessageCallback_p.0.get() = None;
}
/// [glDebugMessageCallbackARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallbackARB.xhtml)
/// * `userParam` len: COMPSIZE(callback)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageCallbackARB(callback: GLDEBUGPROCARB, userParam: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageCallbackARB_p.0.get() } {
    Some(glDebugMessageCallbackARB_inner) => glDebugMessageCallbackARB_inner(callback, userParam),
    None => gl_not_loaded("glDebugMessageCallbackARB"),
  }
}
static glDebugMessageCallbackARB_p: GlFnCell<glDebugMessageCallbackARB_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageCallbackARB_is_loaded() -> bool {
  unsafe { *glDebugMessageCallbackARB_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageCallbackARB_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageCallbackARB_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageCallbackARB_t>>(gl_ptr_filter(f(b"glDebugMessageCallbackARB\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageCallbackARB_reset_ptr() {
  *glDebugMessageCallbackARB_p.0.get() = None;
}
/// [glDebugMessageCallbackKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallbackKHR.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageCallbackKHR(callback: GLDEBUGPROCKHR, userParam: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageCallbackKHR_p.0.get() } {
    Some(glDebugMessageCallbackKHR_inner) => glDebugMessageCallbackKHR_inner(callback, userParam),
    None => gl_not_loaded("glDebugMessageCallbackKHR"),
  }
}
static glDebugMessageCallbackKHR_p: GlFnCell<glDebugMessageCallbackKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageCallbackKHR_is_loaded() -> bool {
  unsafe { *glDebugMessageCallbackKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageCallbackKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageCallbackKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageCallbackKHR_t>>(gl_ptr_filter(f(b"glDebugMessageCallbackKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageCallbackKHR_reset_ptr() {
  *glDebugMessageCallbackKHR_p.0.get() = None;
}
/// [glDebugMessageControl](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControl.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageControl(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageControl_p.0.get() } {
    Some(glDebugMessageControl_inner) => glDebugMessageControl_inner(source, type_, severity, count, ids, enabled),
    None => gl_not_loaded("glDebugMessageControl"),
  }
}
static glDebugMessageControl_p: GlFnCell<glDebugMessageControl_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageControl_is_loaded() -> bool {
  unsafe { *glDebugMessageControl_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageControl_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageControl_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageControl_t>>(gl_ptr_filter(f(b"glDebugMessageControl\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageControl_reset_ptr() {
  *glDebugMessageControl_p.0.get() = None;
}
/// [glDebugMessageControlARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControlARB.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageControlARB(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageControlARB_p.0.get() } {
    Some(glDebugMessageControlARB_inner) => glDebugMessageControlARB_inner(source, type_, severity, count, ids, enabled),
    None => gl_not_loaded("glDebugMessageControlARB"),
  }
}
static glDebugMessageControlARB_p: GlFnCell<glDebugMessageControlARB_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageControlARB_is_loaded() -> bool {
  unsafe { *glDebugMessageControlARB_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageControlARB_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageControlARB_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageControlARB_t>>(gl_ptr_filter(f(b"glDebugMessageControlARB\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageControlARB_reset_ptr() {
  *glDebugMessageControlARB_p.0.get() = None;
}
/// [glDebugMessageControlKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControlKHR.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `enabled` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageControlKHR(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageControlKHR_p.0.get() } {
    Some(glDebugMessageControlKHR_inner) => glDebugMessageControlKHR_inner(source, type_, severity, count, ids, enabled),
    None => gl_not_loaded("glDebugMessageControlKHR"),
  }
}
static glDebugMessageControlKHR_p: GlFnCell<glDebugMessageControlKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageControlKHR_is_loaded() -> bool {
  unsafe { *glDebugMessageControlKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageControlKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageControlKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageControlKHR_t>>(gl_ptr_filter(f(b"glDebugMessageControlKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageControlKHR_reset_ptr() {
  *glDebugMessageControlKHR_p.0.get() = None;
}
/// [glDebugMessageInsert](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsert.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `buf` len: COMPSIZE(buf,length)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageInsert(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageInsert_p.0.get() } {
    Some(glDebugMessageInsert_inner) => glDebugMessageInsert_inner(source, type_, id, severity, length, buf),
    None => gl_not_loaded("glDebugMessageInsert"),
  }
}
static glDebugMessageInsert_p: GlFnCell<glDebugMessageInsert_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageInsert_is_loaded() -> bool {
  unsafe { *glDebugMessageInsert_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageInsert_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageInsert_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageInsert_t>>(gl_ptr_filter(f(b"glDebugMessageInsert\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageInsert_reset_ptr() {
  *glDebugMessageInsert_p.0.get() = None;
}
/// [glDebugMessageInsertARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsertARB.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `buf` len: length
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageInsertARB(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageInsertARB_p.0.get() } {
    Some(glDebugMessageInsertARB_inner) => glDebugMessageInsertARB_inner(source, type_, id, severity, length, buf),
    None => gl_not_loaded("glDebugMessageInsertARB"),
  }
}
static glDebugMessageInsertARB_p: GlFnCell<glDebugMessageInsertARB_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageInsertARB_is_loaded() -> bool {
  unsafe { *glDebugMessageInsertARB_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageInsertARB_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageInsertARB_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageInsertARB_t>>(gl_ptr_filter(f(b"glDebugMessageInsertARB\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageInsertARB_reset_ptr() {
  *glDebugMessageInsertARB_p.0.get() = None;
}
/// [glDebugMessageInsertKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsertKHR.xhtml)
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageInsertKHR(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageInsertKHR_p.0.get() } {
    Some(glDebugMessageInsertKHR_inner) => glDebugMessageInsertKHR_inner(source, type_, id, severity, length, buf),
    None => gl_not_loaded("glDebugMessageInsertKHR"),
  }
}
static glDebugMessageInsertKHR_p: GlFnCell<glDebugMessageInsertKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageInsertKHR_is_loaded() -> bool {
  unsafe { *glDebugMessageInsertKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageInsertKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageInsertKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageInsertKHR_t>>(gl_ptr_filter(f(b"glDebugMessageInsertKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDebugMessageInsertKHR_reset_ptr() {
  *glDebugMessageInsertKHR_p.0.get() = None;
}
/// [glDeleteBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteBuffers.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteBuffers_p.0.get() } {
    Some(glDeleteBuffers_inner) => glDeleteBuffers_inner(n, buffers),
    None => gl_not_loaded("glDeleteBuffers"),
  }
}
static glDeleteBuffers_p: GlFnCell<glDeleteBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteBuffers_is_loaded() -> bool {
  unsafe { *glDeleteBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteBuffers_t>>(gl_ptr_filter(f(b"glDeleteBuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteBuffers_reset_ptr() {
  *glDeleteBuffers_p.0.get() = None;
}
/// [glDeleteFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteFramebuffers.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteFramebuffers_p.0.get() } {
    Some(glDeleteFramebuffers_inner) => glDeleteFramebuffers_inner(n, framebuffers),
    None => gl_not_loaded("glDeleteFramebuffers"),
  }
}
static glDeleteFramebuffers_p: GlFnCell<glDeleteFramebuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteFramebuffers_is_loaded() -> bool {
  unsafe { *glDeleteFramebuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteFramebuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteFramebuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteFramebuffers_t>>(gl_ptr_filter(f(b"glDeleteFramebuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteFramebuffers_reset_ptr() {
  *glDeleteFramebuffers_p.0.get() = None;
}
/// [glDeleteProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgram.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteProgram(program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteProgram_p.0.get() } {
    Some(glDeleteProgram_inner) => glDeleteProgram_inner(program),
    None => gl_not_loaded("glDeleteProgram"),
  }
}
static glDeleteProgram_p: GlFnCell<glDeleteProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteProgram_is_loaded() -> bool {
  unsafe { *glDeleteProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteProgram_t>>(gl_ptr_filter(f(b"glDeleteProgram\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteProgram_reset_ptr() {
  *glDeleteProgram_p.0.get() = None;
}
/// [glDeleteProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgramPipelines.xhtml)
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteProgramPipelines_p.0.get() } {
    Some(glDeleteProgramPipelines_inner) => glDeleteProgramPipelines_inner(n, pipelines),
    None => gl_not_loaded("glDeleteProgramPipelines"),
  }
}
static glDeleteProgramPipelines_p: GlFnCell<glDeleteProgramPipelines_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteProgramPipelines_is_loaded() -> bool {
  unsafe { *glDeleteProgramPipelines_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteProgramPipelines_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteProgramPipelines_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteProgramPipelines_t>>(gl_ptr_filter(f(b"glDeleteProgramPipelines\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteProgramPipelines_reset_ptr() {
  *glDeleteProgramPipelines_p.0.get() = None;
}
/// [glDeleteQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteQueries.xhtml)
/// * `ids` class: query
/// * `ids` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteQueries(n: GLsizei, ids: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteQueries_p.0.get() } {
    Some(glDeleteQueries_inner) => glDeleteQueries_inner(n, ids),
    None => gl_not_loaded("glDeleteQueries"),
  }
}
static glDeleteQueries_p: GlFnCell<glDeleteQueries_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteQueries_is_loaded() -> bool {
  unsafe { *glDeleteQueries_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteQueries_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteQueries_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteQueries_t>>(gl_ptr_filter(f(b"glDeleteQueries\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteQueries_reset_ptr() {
  *glDeleteQueries_p.0.get() = None;
}
/// [glDeleteRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteRenderbuffers.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteRenderbuffers_p.0.get() } {
    Some(glDeleteRenderbuffers_inner) => glDeleteRenderbuffers_inner(n, renderbuffers),
    None => gl_not_loaded("glDeleteRenderbuffers"),
  }
}
static glDeleteRenderbuffers_p: GlFnCell<glDeleteRenderbuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteRenderbuffers_is_loaded() -> bool {
  unsafe { *glDeleteRenderbuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteRenderbuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteRenderbuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteRenderbuffers_t>>(gl_ptr_filter(f(b"glDeleteRenderbuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteRenderbuffers_reset_ptr() {
  *glDeleteRenderbuffers_p.0.get() = None;
}
/// [glDeleteSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteSamplers.xhtml)
/// * `samplers` class: sampler
/// * `samplers` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteSamplers_p.0.get() } {
    Some(glDeleteSamplers_inner) => glDeleteSamplers_inner(count, samplers),
    None => gl_not_loaded("glDeleteSamplers"),
  }
}
static glDeleteSamplers_p: GlFnCell<glDeleteSamplers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteSamplers_is_loaded() -> bool {
  unsafe { *glDeleteSamplers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteSamplers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteSamplers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteSamplers_t>>(gl_ptr_filter(f(b"glDeleteSamplers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteSamplers_reset_ptr() {
  *glDeleteSamplers_p.0.get() = None;
}
/// [glDeleteShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml)
/// * `shader` class: shader
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteShader(shader: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteShader_p.0.get() } {
    Some(glDeleteShader_inner) => glDeleteShader_inner(shader),
    None => gl_not_loaded("glDeleteShader"),
  }
}
static glDeleteShader_p: GlFnCell<glDeleteShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteShader_is_loaded() -> bool {
  unsafe { *glDeleteShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteShader_t>>(gl_ptr_filter(f(b"glDeleteShader\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteShader_reset_ptr() {
  *glDeleteShader_p.0.get() = None;
}
/// [glDeleteSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteSync.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteSync(sync: GLsync) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteSync_p.0.get() } {
    Some(glDeleteSync_inner) => glDeleteSync_inner(sync),
    None => gl_not_loaded("glDeleteSync"),
  }
}
static glDeleteSync_p: GlFnCell<glDeleteSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteSync_is_loaded() -> bool {
  unsafe { *glDeleteSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteSync_t>>(gl_ptr_filter(f(b"glDeleteSync\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteSync_reset_ptr() {
  *glDeleteSync_p.0.get() = None;
}
/// [glDeleteTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteTextures.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteTextures_p.0.get() } {
    Some(glDeleteTextures_inner) => glDeleteTextures_inner(n, textures),
    None => gl_not_loaded("glDeleteTextures"),
  }
}
static glDeleteTextures_p: GlFnCell<glDeleteTextures_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteTextures_is_loaded() -> bool {
  unsafe { *glDeleteTextures_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteTextures_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteTextures_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteTextures_t>>(gl_ptr_filter(f(b"glDeleteTextures\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteTextures_reset_ptr() {
  *glDeleteTextures_p.0.get() = None;
}
/// [glDeleteTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteTransformFeedbacks.xhtml)
/// * `ids` class: transform feedback
/// * `ids` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteTransformFeedbacks_p.0.get() } {
    Some(glDeleteTransformFeedbacks_inner) => glDeleteTransformFeedbacks_inner(n, ids),
    None => gl_not_loaded("glDeleteTransformFeedbacks"),
  }
}
static glDeleteTransformFeedbacks_p: GlFnCell<glDeleteTransformFeedbacks_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteTransformFeedbacks_is_loaded() -> bool {
  unsafe { *glDeleteTransformFeedbacks_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteTransformFeedbacks_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteTransformFeedbacks_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteTransformFeedbacks_t>>(gl_ptr_filter(f(b"glDeleteTransformFeedbacks\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteTransformFeedbacks_reset_ptr() {
  *glDeleteTransformFeedbacks_p.0.get() = None;
}
/// [glDeleteVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteVertexArrays_p.0.get() } {
    Some(glDeleteVertexArrays_inner) => glDeleteVertexArrays_inner(n, arrays),
    None => gl_not_loaded("glDeleteVertexArrays"),
  }
}
static glDeleteVertexArrays_p: GlFnCell<glDeleteVertexArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteVertexArrays_is_loaded() -> bool {
  unsafe { *glDeleteVertexArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteVertexArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteVertexArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteVertexArrays_t>>(gl_ptr_filter(f(b"glDeleteVertexArrays\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDeleteVertexArrays_reset_ptr() {
  *glDeleteVertexArrays_p.0.get() = None;
}
/// [glDepthFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthFunc.xhtml)
/// * `func` group: DepthFunction
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthFunc(func: DepthFunction) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthFunc_p.0.get() } {
    Some(glDepthFunc_inner) => glDepthFunc_inner(func),
    None => gl_not_loaded("glDepthFunc"),
  }
}
static glDepthFunc_p: GlFnCell<glDepthFunc_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthFunc_is_loaded() -> bool {
  unsafe { *glDepthFunc_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthFunc_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthFunc_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthFunc_t>>(gl_ptr_filter(f(b"glDepthFunc\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDepthFunc_reset_ptr() {
  *glDepthFunc_p.0.get() = None;
}
/// [glDepthMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthMask.xhtml)
/// * `flag` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthMask(flag: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthMask_p.0.get() } {
    Some(glDepthMask_inner) => glDepthMask_inner(flag),
    None => gl_not_loaded("glDepthMask"),
  }
}
static glDepthMask_p: GlFnCell<glDepthMask_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthMask_is_loaded() -> bool {
  unsafe { *glDepthMask_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthMask_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthMask_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthMask_t>>(gl_ptr_filter(f(b"glDepthMask\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDepthMask_reset_ptr() {
  *glDepthMask_p.0.get() = None;
}
/// [glDepthRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRange.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthRange(n: GLdouble, f: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthRange_p.0.get() } {
    Some(glDepthRange_inner) => glDepthRange_inner(n, f),
    None => gl_not_loaded("glDepthRange"),
  }
}
static glDepthRange_p: GlFnCell<glDepthRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthRange_is_loaded() -> bool {
  unsafe { *glDepthRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthRange_t>>(gl_ptr_filter(f(b"glDepthRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDepthRange_reset_ptr() {
  *glDepthRange_p.0.get() = None;
}
/// [glDepthRangeArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeArrayv.xhtml)
/// * `v` len: COMPSIZE(count)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthRangeArrayv_p.0.get() } {
    Some(glDepthRangeArrayv_inner) => glDepthRangeArrayv_inner(first, count, v),
    None => gl_not_loaded("glDepthRangeArrayv"),
  }
}
static glDepthRangeArrayv_p: GlFnCell<glDepthRangeArrayv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthRangeArrayv_is_loaded() -> bool {
  unsafe { *glDepthRangeArrayv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthRangeArrayv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthRangeArrayv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthRangeArrayv_t>>(gl_ptr_filter(f(b"glDepthRangeArrayv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDepthRangeArrayv_reset_ptr() {
  *glDepthRangeArrayv_p.0.get() = None;
}
/// [glDepthRangeIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeIndexed.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthRangeIndexed_p.0.get() } {
    Some(glDepthRangeIndexed_inner) => glDepthRangeIndexed_inner(index, n, f),
    None => gl_not_loaded("glDepthRangeIndexed"),
  }
}
static glDepthRangeIndexed_p: GlFnCell<glDepthRangeIndexed_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthRangeIndexed_is_loaded() -> bool {
  unsafe { *glDepthRangeIndexed_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthRangeIndexed_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthRangeIndexed_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthRangeIndexed_t>>(gl_ptr_filter(f(b"glDepthRangeIndexed\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDepthRangeIndexed_reset_ptr() {
  *glDepthRangeIndexed_p.0.get() = None;
}
/// [glDepthRangef](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangef.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthRangef(n: GLfloat, f: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthRangef_p.0.get() } {
    Some(glDepthRangef_inner) => glDepthRangef_inner(n, f),
    None => gl_not_loaded("glDepthRangef"),
  }
}
static glDepthRangef_p: GlFnCell<glDepthRangef_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthRangef_is_loaded() -> bool {
  unsafe { *glDepthRangef_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthRangef_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthRangef_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthRangef_t>>(gl_ptr_filter(f(b"glDepthRangef\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDepthRangef_reset_ptr() {
  *glDepthRangef_p.0.get() = None;
}
/// [glDetachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDetachShader.xhtml)
/// * `program` class: program
/// * `shader` class: shader
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDetachShader(program: GLuint, shader: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDetachShader_p.0.get() } {
    Some(glDetachShader_inner) => glDetachShader_inner(program, shader),
    None => gl_not_loaded("glDetachShader"),
  }
}
static glDetachShader_p: GlFnCell<glDetachShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDetachShader_is_loaded() -> bool {
  unsafe { *glDetachShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDetachShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDetachShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDetachShader_t>>(gl_ptr_filter(f(b"glDetachShader\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDetachShader_reset_ptr() {
  *glDetachShader_p.0.get() = None;
}
/// [glDisable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisable.xhtml)
/// * `cap` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDisable(cap: EnableCap) {
  #[allow(unused_unsafe)]
  match unsafe { *glDisable_p.0.get() } {
    Some(glDisable_inner) => glDisable_inner(cap),
    None => gl_not_loaded("glDisable"),
  }
}
static glDisable_p: GlFnCell<glDisable_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDisable_is_loaded() -> bool {
  unsafe { *glDisable_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDisable_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDisable_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDisable_t>>(gl_ptr_filter(f(b"glDisable\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDisable_reset_ptr() {
  *glDisable_p.0.get() = None;
}
/// [glDisableVertexArrayAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexArrayAttrib.xhtml)
/// * `vaobj` class: vertex array
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDisableVertexArrayAttrib_p.0.get() } {
    Some(glDisableVertexArrayAttrib_inner) => glDisableVertexArrayAttrib_inner(vaobj, index),
    None => gl_not_loaded("glDisableVertexArrayAttrib"),
  }
}
static glDisableVertexArrayAttrib_p: GlFnCell<glDisableVertexArrayAttrib_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDisableVertexArrayAttrib_is_loaded() -> bool {
  unsafe { *glDisableVertexArrayAttrib_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDisableVertexArrayAttrib_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDisableVertexArrayAttrib_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDisableVertexArrayAttrib_t>>(gl_ptr_filter(f(b"glDisableVertexArrayAttrib\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDisableVertexArrayAttrib_reset_ptr() {
  *glDisableVertexArrayAttrib_p.0.get() = None;
}
/// [glDisableVertexAttribArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexAttribArray.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDisableVertexAttribArray(index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDisableVertexAttribArray_p.0.get() } {
    Some(glDisableVertexAttribArray_inner) => glDisableVertexAttribArray_inner(index),
    None => gl_not_loaded("glDisableVertexAttribArray"),
  }
}
static glDisableVertexAttribArray_p: GlFnCell<glDisableVertexAttribArray_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDisableVertexAttribArray_is_loaded() -> bool {
  unsafe { *glDisableVertexAttribArray_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDisableVertexAttribArray_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDisableVertexAttribArray_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDisableVertexAttribArray_t>>(gl_ptr_filter(f(b"glDisableVertexAttribArray\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDisableVertexAttribArray_reset_ptr() {
  *glDisableVertexAttribArray_p.0.get() = None;
}
/// [glDisablei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisablei.xhtml)
/// * `target` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDisablei(target: EnableCap, index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDisablei_p.0.get() } {
    Some(glDisablei_inner) => glDisablei_inner(target, index),
    None => gl_not_loaded("glDisablei"),
  }
}
static glDisablei_p: GlFnCell<glDisablei_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDisablei_is_loaded() -> bool {
  unsafe { *glDisablei_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDisablei_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDisablei_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDisablei_t>>(gl_ptr_filter(f(b"glDisablei\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDisablei_reset_ptr() {
  *glDisablei_p.0.get() = None;
}
/// [glDispatchCompute](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchCompute.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDispatchCompute_p.0.get() } {
    Some(glDispatchCompute_inner) => glDispatchCompute_inner(num_groups_x, num_groups_y, num_groups_z),
    None => gl_not_loaded("glDispatchCompute"),
  }
}
static glDispatchCompute_p: GlFnCell<glDispatchCompute_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDispatchCompute_is_loaded() -> bool {
  unsafe { *glDispatchCompute_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDispatchCompute_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDispatchCompute_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDispatchCompute_t>>(gl_ptr_filter(f(b"glDispatchCompute\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDispatchCompute_reset_ptr() {
  *glDispatchCompute_p.0.get() = None;
}
/// [glDispatchComputeIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchComputeIndirect.xhtml)
/// * `indirect` group: BufferOffset
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDispatchComputeIndirect(indirect: GLintptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glDispatchComputeIndirect_p.0.get() } {
    Some(glDispatchComputeIndirect_inner) => glDispatchComputeIndirect_inner(indirect),
    None => gl_not_loaded("glDispatchComputeIndirect"),
  }
}
static glDispatchComputeIndirect_p: GlFnCell<glDispatchComputeIndirect_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDispatchComputeIndirect_is_loaded() -> bool {
  unsafe { *glDispatchComputeIndirect_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDispatchComputeIndirect_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDispatchComputeIndirect_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDispatchComputeIndirect_t>>(gl_ptr_filter(f(b"glDispatchComputeIndirect\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDispatchComputeIndirect_reset_ptr() {
  *glDispatchComputeIndirect_p.0.get() = None;
}
/// [glDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml)
/// * `mode` group: PrimitiveType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawArrays(mode: PrimitiveType, first: GLint, count: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawArrays_p.0.get() } {
    Some(glDrawArrays_inner) => glDrawArrays_inner(mode, first, count),
    None => gl_not_loaded("glDrawArrays"),
  }
}
static glDrawArrays_p: GlFnCell<glDrawArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawArrays_is_loaded() -> bool {
  unsafe { *glDrawArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawArrays_t>>(gl_ptr_filter(f(b"glDrawArrays\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawArrays_reset_ptr() {
  *glDrawArrays_p.0.get() = None;
}
/// [glDrawArraysIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysIndirect.xhtml)
/// * `mode` group: PrimitiveType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawArraysIndirect(mode: PrimitiveType, indirect: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawArraysIndirect_p.0.get() } {
    Some(glDrawArraysIndirect_inner) => glDrawArraysIndirect_inner(mode, indirect),
    None => gl_not_loaded("glDrawArraysIndirect"),
  }
}
static glDrawArraysIndirect_p: GlFnCell<glDrawArraysIndirect_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawArraysIndirect_is_loaded() -> bool {
  unsafe { *glDrawArraysIndirect_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawArraysIndirect_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawArraysIndirect_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawArraysIndirect_t>>(gl_ptr_filter(f(b"glDrawArraysIndirect\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawArraysIndirect_reset_ptr() {
  *glDrawArraysIndirect_p.0.get() = None;
}
/// [glDrawArraysInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstanced.xhtml)
/// * `mode` group: PrimitiveType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawArraysInstanced(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawArraysInstanced_p.0.get() } {
    Some(glDrawArraysInstanced_inner) => glDrawArraysInstanced_inner(mode, first, count, instancecount),
    None => gl_not_loaded("glDrawArraysInstanced"),
  }
}
static glDrawArraysInstanced_p: GlFnCell<glDrawArraysInstanced_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawArraysInstanced_is_loaded() -> bool {
  unsafe { *glDrawArraysInstanced_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawArraysInstanced_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawArraysInstanced_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawArraysInstanced_t>>(gl_ptr_filter(f(b"glDrawArraysInstanced\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawArraysInstanced_reset_ptr() {
  *glDrawArraysInstanced_p.0.get() = None;
}
/// [glDrawArraysInstancedBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstancedBaseInstance.xhtml)
/// * `mode` group: PrimitiveType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawArraysInstancedBaseInstance(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawArraysInstancedBaseInstance_p.0.get() } {
    Some(glDrawArraysInstancedBaseInstance_inner) => glDrawArraysInstancedBaseInstance_inner(mode, first, count, instancecount, baseinstance),
    None => gl_not_loaded("glDrawArraysInstancedBaseInstance"),
  }
}
static glDrawArraysInstancedBaseInstance_p: GlFnCell<glDrawArraysInstancedBaseInstance_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawArraysInstancedBaseInstance_is_loaded() -> bool {
  unsafe { *glDrawArraysInstancedBaseInstance_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawArraysInstancedBaseInstance_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawArraysInstancedBaseInstance_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawArraysInstancedBaseInstance_t>>(gl_ptr_filter(f(b"glDrawArraysInstancedBaseInstance\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawArraysInstancedBaseInstance_reset_ptr() {
  *glDrawArraysInstancedBaseInstance_p.0.get() = None;
}
/// [glDrawBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffer.xhtml)
/// * `buf` group: DrawBufferMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawBuffer(buf: DrawBufferMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawBuffer_p.0.get() } {
    Some(glDrawBuffer_inner) => glDrawBuffer_inner(buf),
    None => gl_not_loaded("glDrawBuffer"),
  }
}
static glDrawBuffer_p: GlFnCell<glDrawBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawBuffer_is_loaded() -> bool {
  unsafe { *glDrawBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawBuffer_t>>(gl_ptr_filter(f(b"glDrawBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawBuffer_reset_ptr() {
  *glDrawBuffer_p.0.get() = None;
}
/// [glDrawBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffers.xhtml)
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawBuffers(n: GLsizei, bufs: *const DrawBufferMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawBuffers_p.0.get() } {
    Some(glDrawBuffers_inner) => glDrawBuffers_inner(n, bufs),
    None => gl_not_loaded("glDrawBuffers"),
  }
}
static glDrawBuffers_p: GlFnCell<glDrawBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawBuffers_is_loaded() -> bool {
  unsafe { *glDrawBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawBuffers_t>>(gl_ptr_filter(f(b"glDrawBuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawBuffers_reset_ptr() {
  *glDrawBuffers_p.0.get() = None;
}
/// [glDrawElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElements.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElements(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElements_p.0.get() } {
    Some(glDrawElements_inner) => glDrawElements_inner(mode, count, type_, indices),
    None => gl_not_loaded("glDrawElements"),
  }
}
static glDrawElements_p: GlFnCell<glDrawElements_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElements_is_loaded() -> bool {
  unsafe { *glDrawElements_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElements_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElements_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElements_t>>(gl_ptr_filter(f(b"glDrawElements\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawElements_reset_ptr() {
  *glDrawElements_p.0.get() = None;
}
/// [glDrawElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsBaseVertex.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsBaseVertex(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsBaseVertex_p.0.get() } {
    Some(glDrawElementsBaseVertex_inner) => glDrawElementsBaseVertex_inner(mode, count, type_, indices, basevertex),
    None => gl_not_loaded("glDrawElementsBaseVertex"),
  }
}
static glDrawElementsBaseVertex_p: GlFnCell<glDrawElementsBaseVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsBaseVertex_is_loaded() -> bool {
  unsafe { *glDrawElementsBaseVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsBaseVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsBaseVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsBaseVertex_t>>(gl_ptr_filter(f(b"glDrawElementsBaseVertex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawElementsBaseVertex_reset_ptr() {
  *glDrawElementsBaseVertex_p.0.get() = None;
}
/// [glDrawElementsIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsIndirect.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsIndirect(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsIndirect_p.0.get() } {
    Some(glDrawElementsIndirect_inner) => glDrawElementsIndirect_inner(mode, type_, indirect),
    None => gl_not_loaded("glDrawElementsIndirect"),
  }
}
static glDrawElementsIndirect_p: GlFnCell<glDrawElementsIndirect_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsIndirect_is_loaded() -> bool {
  unsafe { *glDrawElementsIndirect_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsIndirect_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsIndirect_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsIndirect_t>>(gl_ptr_filter(f(b"glDrawElementsIndirect\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawElementsIndirect_reset_ptr() {
  *glDrawElementsIndirect_p.0.get() = None;
}
/// [glDrawElementsInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstanced.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsInstanced(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsInstanced_p.0.get() } {
    Some(glDrawElementsInstanced_inner) => glDrawElementsInstanced_inner(mode, count, type_, indices, instancecount),
    None => gl_not_loaded("glDrawElementsInstanced"),
  }
}
static glDrawElementsInstanced_p: GlFnCell<glDrawElementsInstanced_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsInstanced_is_loaded() -> bool {
  unsafe { *glDrawElementsInstanced_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstanced_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsInstanced_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsInstanced_t>>(gl_ptr_filter(f(b"glDrawElementsInstanced\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstanced_reset_ptr() {
  *glDrawElementsInstanced_p.0.get() = None;
}
/// [glDrawElementsInstancedBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseInstance.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: PrimitiveType
/// * `indices` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsInstancedBaseInstance(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, instancecount: GLsizei, baseinstance: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsInstancedBaseInstance_p.0.get() } {
    Some(glDrawElementsInstancedBaseInstance_inner) => glDrawElementsInstancedBaseInstance_inner(mode, count, type_, indices, instancecount, baseinstance),
    None => gl_not_loaded("glDrawElementsInstancedBaseInstance"),
  }
}
static glDrawElementsInstancedBaseInstance_p: GlFnCell<glDrawElementsInstancedBaseInstance_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsInstancedBaseInstance_is_loaded() -> bool {
  unsafe { *glDrawElementsInstancedBaseInstance_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstancedBaseInstance_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsInstancedBaseInstance_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsInstancedBaseInstance_t>>(gl_ptr_filter(f(b"glDrawElementsInstancedBaseInstance\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstancedBaseInstance_reset_ptr() {
  *glDrawElementsInstancedBaseInstance_p.0.get() = None;
}
/// [glDrawElementsInstancedBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertex.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsInstancedBaseVertex(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsInstancedBaseVertex_p.0.get() } {
    Some(glDrawElementsInstancedBaseVertex_inner) => glDrawElementsInstancedBaseVertex_inner(mode, count, type_, indices, instancecount, basevertex),
    None => gl_not_loaded("glDrawElementsInstancedBaseVertex"),
  }
}
static glDrawElementsInstancedBaseVertex_p: GlFnCell<glDrawElementsInstancedBaseVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsInstancedBaseVertex_is_loaded() -> bool {
  unsafe { *glDrawElementsInstancedBaseVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstancedBaseVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsInstancedBaseVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsInstancedBaseVertex_t>>(gl_ptr_filter(f(b"glDrawElementsInstancedBaseVertex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstancedBaseVertex_reset_ptr() {
  *glDrawElementsInstancedBaseVertex_p.0.get() = None;
}
/// [glDrawElementsInstancedBaseVertexBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertexBaseInstance.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsInstancedBaseVertexBaseInstance(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsInstancedBaseVertexBaseInstance_p.0.get() } {
    Some(glDrawElementsInstancedBaseVertexBaseInstance_inner) => glDrawElementsInstancedBaseVertexBaseInstance_inner(mode, count, type_, indices, instancecount, basevertex, baseinstance),
    None => gl_not_loaded("glDrawElementsInstancedBaseVertexBaseInstance"),
  }
}
static glDrawElementsInstancedBaseVertexBaseInstance_p: GlFnCell<glDrawElementsInstancedBaseVertexBaseInstance_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsInstancedBaseVertexBaseInstance_is_loaded() -> bool {
  unsafe { *glDrawElementsInstancedBaseVertexBaseInstance_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstancedBaseVertexBaseInstance_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsInstancedBaseVertexBaseInstance_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsInstancedBaseVertexBaseInstance_t>>(gl_ptr_filter(f(b"glDrawElementsInstancedBaseVertexBaseInstance\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstancedBaseVertexBaseInstance_reset_ptr() {
  *glDrawElementsInstancedBaseVertexBaseInstance_p.0.get() = None;
}
/// [glDrawRangeElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElements.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawRangeElements(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawRangeElements_p.0.get() } {
    Some(glDrawRangeElements_inner) => glDrawRangeElements_inner(mode, start, end, count, type_, indices),
    None => gl_not_loaded("glDrawRangeElements"),
  }
}
static glDrawRangeElements_p: GlFnCell<glDrawRangeElements_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawRangeElements_is_loaded() -> bool {
  unsafe { *glDrawRangeElements_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawRangeElements_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawRangeElements_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawRangeElements_t>>(gl_ptr_filter(f(b"glDrawRangeElements\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawRangeElements_reset_ptr() {
  *glDrawRangeElements_p.0.get() = None;
}
/// [glDrawRangeElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElementsBaseVertex.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawRangeElementsBaseVertex(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawRangeElementsBaseVertex_p.0.get() } {
    Some(glDrawRangeElementsBaseVertex_inner) => glDrawRangeElementsBaseVertex_inner(mode, start, end, count, type_, indices, basevertex),
    None => gl_not_loaded("glDrawRangeElementsBaseVertex"),
  }
}
static glDrawRangeElementsBaseVertex_p: GlFnCell<glDrawRangeElementsBaseVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawRangeElementsBaseVertex_is_loaded() -> bool {
  unsafe { *glDrawRangeElementsBaseVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawRangeElementsBaseVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawRangeElementsBaseVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawRangeElementsBaseVertex_t>>(gl_ptr_filter(f(b"glDrawRangeElementsBaseVertex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawRangeElementsBaseVertex_reset_ptr() {
  *glDrawRangeElementsBaseVertex_p.0.get() = None;
}
/// [glDrawTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedback.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawTransformFeedback(mode: PrimitiveType, id: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawTransformFeedback_p.0.get() } {
    Some(glDrawTransformFeedback_inner) => glDrawTransformFeedback_inner(mode, id),
    None => gl_not_loaded("glDrawTransformFeedback"),
  }
}
static glDrawTransformFeedback_p: GlFnCell<glDrawTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawTransformFeedback_is_loaded() -> bool {
  unsafe { *glDrawTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawTransformFeedback_t>>(gl_ptr_filter(f(b"glDrawTransformFeedback\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawTransformFeedback_reset_ptr() {
  *glDrawTransformFeedback_p.0.get() = None;
}
/// [glDrawTransformFeedbackInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackInstanced.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawTransformFeedbackInstanced(mode: PrimitiveType, id: GLuint, instancecount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawTransformFeedbackInstanced_p.0.get() } {
    Some(glDrawTransformFeedbackInstanced_inner) => glDrawTransformFeedbackInstanced_inner(mode, id, instancecount),
    None => gl_not_loaded("glDrawTransformFeedbackInstanced"),
  }
}
static glDrawTransformFeedbackInstanced_p: GlFnCell<glDrawTransformFeedbackInstanced_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawTransformFeedbackInstanced_is_loaded() -> bool {
  unsafe { *glDrawTransformFeedbackInstanced_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawTransformFeedbackInstanced_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawTransformFeedbackInstanced_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawTransformFeedbackInstanced_t>>(gl_ptr_filter(f(b"glDrawTransformFeedbackInstanced\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawTransformFeedbackInstanced_reset_ptr() {
  *glDrawTransformFeedbackInstanced_p.0.get() = None;
}
/// [glDrawTransformFeedbackStream](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackStream.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawTransformFeedbackStream(mode: PrimitiveType, id: GLuint, stream: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawTransformFeedbackStream_p.0.get() } {
    Some(glDrawTransformFeedbackStream_inner) => glDrawTransformFeedbackStream_inner(mode, id, stream),
    None => gl_not_loaded("glDrawTransformFeedbackStream"),
  }
}
static glDrawTransformFeedbackStream_p: GlFnCell<glDrawTransformFeedbackStream_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawTransformFeedbackStream_is_loaded() -> bool {
  unsafe { *glDrawTransformFeedbackStream_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawTransformFeedbackStream_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawTransformFeedbackStream_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawTransformFeedbackStream_t>>(gl_ptr_filter(f(b"glDrawTransformFeedbackStream\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawTransformFeedbackStream_reset_ptr() {
  *glDrawTransformFeedbackStream_p.0.get() = None;
}
/// [glDrawTransformFeedbackStreamInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackStreamInstanced.xhtml)
/// * `mode` group: PrimitiveType
/// * `id` class: transform feedback
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawTransformFeedbackStreamInstanced(mode: PrimitiveType, id: GLuint, stream: GLuint, instancecount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawTransformFeedbackStreamInstanced_p.0.get() } {
    Some(glDrawTransformFeedbackStreamInstanced_inner) => glDrawTransformFeedbackStreamInstanced_inner(mode, id, stream, instancecount),
    None => gl_not_loaded("glDrawTransformFeedbackStreamInstanced"),
  }
}
static glDrawTransformFeedbackStreamInstanced_p: GlFnCell<glDrawTransformFeedbackStreamInstanced_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawTransformFeedbackStreamInstanced_is_loaded() -> bool {
  unsafe { *glDrawTransformFeedbackStreamInstanced_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawTransformFeedbackStreamInstanced_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawTransformFeedbackStreamInstanced_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawTransformFeedbackStreamInstanced_t>>(gl_ptr_filter(f(b"glDrawTransformFeedbackStreamInstanced\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glDrawTransformFeedbackStreamInstanced_reset_ptr() {
  *glDrawTransformFeedbackStreamInstanced_p.0.get() = None;
}
/// [glEnable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml)
/// * `cap` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEnable(cap: EnableCap) {
  #[allow(unused_unsafe)]
  match unsafe { *glEnable_p.0.get() } {
    Some(glEnable_inner) => glEnable_inner(cap),
    None => gl_not_loaded("glEnable"),
  }
}
static glEnable_p: GlFnCell<glEnable_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEnable_is_loaded() -> bool {
  unsafe { *glEnable_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEnable_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEnable_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEnable_t>>(gl_ptr_filter(f(b"glEnable\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glEnable_reset_ptr() {
  *glEnable_p.0.get() = None;
}
/// [glEnableVertexArrayAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexArrayAttrib.xhtml)
/// * `vaobj` class: vertex array
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glEnableVertexArrayAttrib_p.0.get() } {
    Some(glEnableVertexArrayAttrib_inner) => glEnableVertexArrayAttrib_inner(vaobj, index),
    None => gl_not_loaded("glEnableVertexArrayAttrib"),
  }
}
static glEnableVertexArrayAttrib_p: GlFnCell<glEnableVertexArrayAttrib_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEnableVertexArrayAttrib_is_loaded() -> bool {
  unsafe { *glEnableVertexArrayAttrib_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEnableVertexArrayAttrib_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEnableVertexArrayAttrib_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEnableVertexArrayAttrib_t>>(gl_ptr_filter(f(b"glEnableVertexArrayAttrib\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glEnableVertexArrayAttrib_reset_ptr() {
  *glEnableVertexArrayAttrib_p.0.get() = None;
}
/// [glEnableVertexAttribArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEnableVertexAttribArray(index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glEnableVertexAttribArray_p.0.get() } {
    Some(glEnableVertexAttribArray_inner) => glEnableVertexAttribArray_inner(index),
    None => gl_not_loaded("glEnableVertexAttribArray"),
  }
}
static glEnableVertexAttribArray_p: GlFnCell<glEnableVertexAttribArray_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEnableVertexAttribArray_is_loaded() -> bool {
  unsafe { *glEnableVertexAttribArray_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEnableVertexAttribArray_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEnableVertexAttribArray_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEnableVertexAttribArray_t>>(gl_ptr_filter(f(b"glEnableVertexAttribArray\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glEnableVertexAttribArray_reset_ptr() {
  *glEnableVertexAttribArray_p.0.get() = None;
}
/// [glEnablei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnablei.xhtml)
/// * `target` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEnablei(target: EnableCap, index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glEnablei_p.0.get() } {
    Some(glEnablei_inner) => glEnablei_inner(target, index),
    None => gl_not_loaded("glEnablei"),
  }
}
static glEnablei_p: GlFnCell<glEnablei_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEnablei_is_loaded() -> bool {
  unsafe { *glEnablei_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEnablei_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEnablei_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEnablei_t>>(gl_ptr_filter(f(b"glEnablei\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glEnablei_reset_ptr() {
  *glEnablei_p.0.get() = None;
}
/// [glEndConditionalRender](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndConditionalRender.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEndConditionalRender() {
  #[allow(unused_unsafe)]
  match unsafe { *glEndConditionalRender_p.0.get() } {
    Some(glEndConditionalRender_inner) => glEndConditionalRender_inner(),
    None => gl_not_loaded("glEndConditionalRender"),
  }
}
static glEndConditionalRender_p: GlFnCell<glEndConditionalRender_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEndConditionalRender_is_loaded() -> bool {
  unsafe { *glEndConditionalRender_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEndConditionalRender_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEndConditionalRender_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEndConditionalRender_t>>(gl_ptr_filter(f(b"glEndConditionalRender\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glEndConditionalRender_reset_ptr() {
  *glEndConditionalRender_p.0.get() = None;
}
/// [glEndQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndQuery.xhtml)
/// * `target` group: QueryTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEndQuery(target: QueryTarget) {
  #[allow(unused_unsafe)]
  match unsafe { *glEndQuery_p.0.get() } {
    Some(glEndQuery_inner) => glEndQuery_inner(target),
    None => gl_not_loaded("glEndQuery"),
  }
}
static glEndQuery_p: GlFnCell<glEndQuery_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEndQuery_is_loaded() -> bool {
  unsafe { *glEndQuery_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEndQuery_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEndQuery_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEndQuery_t>>(gl_ptr_filter(f(b"glEndQuery\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glEndQuery_reset_ptr() {
  *glEndQuery_p.0.get() = None;
}
/// [glEndQueryIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndQueryIndexed.xhtml)
/// * `target` group: QueryTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEndQueryIndexed(target: QueryTarget, index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glEndQueryIndexed_p.0.get() } {
    Some(glEndQueryIndexed_inner) => glEndQueryIndexed_inner(target, index),
    None => gl_not_loaded("glEndQueryIndexed"),
  }
}
static glEndQueryIndexed_p: GlFnCell<glEndQueryIndexed_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEndQueryIndexed_is_loaded() -> bool {
  unsafe { *glEndQueryIndexed_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEndQueryIndexed_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEndQueryIndexed_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEndQueryIndexed_t>>(gl_ptr_filter(f(b"glEndQueryIndexed\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glEndQueryIndexed_reset_ptr() {
  *glEndQueryIndexed_p.0.get() = None;
}
/// [glEndTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndTransformFeedback.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEndTransformFeedback() {
  #[allow(unused_unsafe)]
  match unsafe { *glEndTransformFeedback_p.0.get() } {
    Some(glEndTransformFeedback_inner) => glEndTransformFeedback_inner(),
    None => gl_not_loaded("glEndTransformFeedback"),
  }
}
static glEndTransformFeedback_p: GlFnCell<glEndTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEndTransformFeedback_is_loaded() -> bool {
  unsafe { *glEndTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEndTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEndTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEndTransformFeedback_t>>(gl_ptr_filter(f(b"glEndTransformFeedback\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glEndTransformFeedback_reset_ptr() {
  *glEndTransformFeedback_p.0.get() = None;
}
/// [glFenceSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFenceSync.xhtml)
/// * `condition` group: SyncCondition
/// * `flags` group: SyncBehaviorFlags
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFenceSync(condition: SyncCondition, flags: GLbitfield) -> GLsync {
  #[allow(unused_unsafe)]
  match unsafe { *glFenceSync_p.0.get() } {
    Some(glFenceSync_inner) => glFenceSync_inner(condition, flags),
    None => gl_not_loaded("glFenceSync"),
  }
}
static glFenceSync_p: GlFnCell<glFenceSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFenceSync_is_loaded() -> bool {
  unsafe { *glFenceSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFenceSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFenceSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFenceSync_t>>(gl_ptr_filter(f(b"glFenceSync\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFenceSync_reset_ptr() {
  *glFenceSync_p.0.get() = None;
}
/// [glFinish](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinish.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFinish() {
  #[allow(unused_unsafe)]
  match unsafe { *glFinish_p.0.get() } {
    Some(glFinish_inner) => glFinish_inner(),
    None => gl_not_loaded("glFinish"),
  }
}
static glFinish_p: GlFnCell<glFinish_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFinish_is_loaded() -> bool {
  unsafe { *glFinish_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFinish_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFinish_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFinish_t>>(gl_ptr_filter(f(b"glFinish\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFinish_reset_ptr() {
  *glFinish_p.0.get() = None;
}
/// [glFlush](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlush.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFlush() {
  #[allow(unused_unsafe)]
  match unsafe { *glFlush_p.0.get() } {
    Some(glFlush_inner) => glFlush_inner(),
    None => gl_not_loaded("glFlush"),
  }
}
static glFlush_p: GlFnCell<glFlush_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFlush_is_loaded() -> bool {
  unsafe { *glFlush_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFlush_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFlush_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFlush_t>>(gl_ptr_filter(f(b"glFlush\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFlush_reset_ptr() {
  *glFlush_p.0.get() = None;
}
/// [glFlushMappedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedBufferRange.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFlushMappedBufferRange(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glFlushMappedBufferRange_p.0.get() } {
    Some(glFlushMappedBufferRange_inner) => glFlushMappedBufferRange_inner(target, offset, length),
    None => gl_not_loaded("glFlushMappedBufferRange"),
  }
}
static glFlushMappedBufferRange_p: GlFnCell<glFlushMappedBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFlushMappedBufferRange_is_loaded() -> bool {
  unsafe { *glFlushMappedBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFlushMappedBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFlushMappedBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFlushMappedBufferRange_t>>(gl_ptr_filter(f(b"glFlushMappedBufferRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFlushMappedBufferRange_reset_ptr() {
  *glFlushMappedBufferRange_p.0.get() = None;
}
/// [glFlushMappedNamedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedNamedBufferRange.xhtml)
/// * `buffer` class: buffer
/// * `length` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFlushMappedNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glFlushMappedNamedBufferRange_p.0.get() } {
    Some(glFlushMappedNamedBufferRange_inner) => glFlushMappedNamedBufferRange_inner(buffer, offset, length),
    None => gl_not_loaded("glFlushMappedNamedBufferRange"),
  }
}
static glFlushMappedNamedBufferRange_p: GlFnCell<glFlushMappedNamedBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFlushMappedNamedBufferRange_is_loaded() -> bool {
  unsafe { *glFlushMappedNamedBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFlushMappedNamedBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFlushMappedNamedBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFlushMappedNamedBufferRange_t>>(gl_ptr_filter(f(b"glFlushMappedNamedBufferRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFlushMappedNamedBufferRange_reset_ptr() {
  *glFlushMappedNamedBufferRange_p.0.get() = None;
}
/// [glFramebufferParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferParameteri.xhtml)
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferParameteri(target: FramebufferTarget, pname: FramebufferParameterName, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferParameteri_p.0.get() } {
    Some(glFramebufferParameteri_inner) => glFramebufferParameteri_inner(target, pname, param),
    None => gl_not_loaded("glFramebufferParameteri"),
  }
}
static glFramebufferParameteri_p: GlFnCell<glFramebufferParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferParameteri_is_loaded() -> bool {
  unsafe { *glFramebufferParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferParameteri_t>>(gl_ptr_filter(f(b"glFramebufferParameteri\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFramebufferParameteri_reset_ptr() {
  *glFramebufferParameteri_p.0.get() = None;
}
/// [glFramebufferRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferRenderbuffer.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferRenderbuffer(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferRenderbuffer_p.0.get() } {
    Some(glFramebufferRenderbuffer_inner) => glFramebufferRenderbuffer_inner(target, attachment, renderbuffertarget, renderbuffer),
    None => gl_not_loaded("glFramebufferRenderbuffer"),
  }
}
static glFramebufferRenderbuffer_p: GlFnCell<glFramebufferRenderbuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferRenderbuffer_is_loaded() -> bool {
  unsafe { *glFramebufferRenderbuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferRenderbuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferRenderbuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferRenderbuffer_t>>(gl_ptr_filter(f(b"glFramebufferRenderbuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFramebufferRenderbuffer_reset_ptr() {
  *glFramebufferRenderbuffer_p.0.get() = None;
}
/// [glFramebufferTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTexture(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTexture_p.0.get() } {
    Some(glFramebufferTexture_inner) => glFramebufferTexture_inner(target, attachment, texture, level),
    None => gl_not_loaded("glFramebufferTexture"),
  }
}
static glFramebufferTexture_p: GlFnCell<glFramebufferTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTexture_is_loaded() -> bool {
  unsafe { *glFramebufferTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTexture_t>>(gl_ptr_filter(f(b"glFramebufferTexture\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture_reset_ptr() {
  *glFramebufferTexture_p.0.get() = None;
}
/// [glFramebufferTexture1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture1D.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTexture1D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTexture1D_p.0.get() } {
    Some(glFramebufferTexture1D_inner) => glFramebufferTexture1D_inner(target, attachment, textarget, texture, level),
    None => gl_not_loaded("glFramebufferTexture1D"),
  }
}
static glFramebufferTexture1D_p: GlFnCell<glFramebufferTexture1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTexture1D_is_loaded() -> bool {
  unsafe { *glFramebufferTexture1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTexture1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTexture1D_t>>(gl_ptr_filter(f(b"glFramebufferTexture1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture1D_reset_ptr() {
  *glFramebufferTexture1D_p.0.get() = None;
}
/// [glFramebufferTexture2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture2D.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTexture2D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTexture2D_p.0.get() } {
    Some(glFramebufferTexture2D_inner) => glFramebufferTexture2D_inner(target, attachment, textarget, texture, level),
    None => gl_not_loaded("glFramebufferTexture2D"),
  }
}
static glFramebufferTexture2D_p: GlFnCell<glFramebufferTexture2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTexture2D_is_loaded() -> bool {
  unsafe { *glFramebufferTexture2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTexture2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTexture2D_t>>(gl_ptr_filter(f(b"glFramebufferTexture2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture2D_reset_ptr() {
  *glFramebufferTexture2D_p.0.get() = None;
}
/// [glFramebufferTexture3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture3D.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTexture3D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTexture3D_p.0.get() } {
    Some(glFramebufferTexture3D_inner) => glFramebufferTexture3D_inner(target, attachment, textarget, texture, level, zoffset),
    None => gl_not_loaded("glFramebufferTexture3D"),
  }
}
static glFramebufferTexture3D_p: GlFnCell<glFramebufferTexture3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTexture3D_is_loaded() -> bool {
  unsafe { *glFramebufferTexture3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTexture3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTexture3D_t>>(gl_ptr_filter(f(b"glFramebufferTexture3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture3D_reset_ptr() {
  *glFramebufferTexture3D_p.0.get() = None;
}
/// [glFramebufferTextureLayer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureLayer.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTextureLayer(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTextureLayer_p.0.get() } {
    Some(glFramebufferTextureLayer_inner) => glFramebufferTextureLayer_inner(target, attachment, texture, level, layer),
    None => gl_not_loaded("glFramebufferTextureLayer"),
  }
}
static glFramebufferTextureLayer_p: GlFnCell<glFramebufferTextureLayer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTextureLayer_is_loaded() -> bool {
  unsafe { *glFramebufferTextureLayer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTextureLayer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTextureLayer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTextureLayer_t>>(gl_ptr_filter(f(b"glFramebufferTextureLayer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFramebufferTextureLayer_reset_ptr() {
  *glFramebufferTextureLayer_p.0.get() = None;
}
/// [glFrontFace](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrontFace.xhtml)
/// * `mode` group: FrontFaceDirection
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFrontFace(mode: FrontFaceDirection) {
  #[allow(unused_unsafe)]
  match unsafe { *glFrontFace_p.0.get() } {
    Some(glFrontFace_inner) => glFrontFace_inner(mode),
    None => gl_not_loaded("glFrontFace"),
  }
}
static glFrontFace_p: GlFnCell<glFrontFace_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFrontFace_is_loaded() -> bool {
  unsafe { *glFrontFace_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFrontFace_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFrontFace_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFrontFace_t>>(gl_ptr_filter(f(b"glFrontFace\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glFrontFace_reset_ptr() {
  *glFrontFace_p.0.get() = None;
}
/// [glGenBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml)
/// * `buffers` class: buffer
/// * `buffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenBuffers_p.0.get() } {
    Some(glGenBuffers_inner) => glGenBuffers_inner(n, buffers),
    None => gl_not_loaded("glGenBuffers"),
  }
}
static glGenBuffers_p: GlFnCell<glGenBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenBuffers_is_loaded() -> bool {
  unsafe { *glGenBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenBuffers_t>>(gl_ptr_filter(f(b"glGenBuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenBuffers_reset_ptr() {
  *glGenBuffers_p.0.get() = None;
}
/// [glGenFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenFramebuffers.xhtml)
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenFramebuffers_p.0.get() } {
    Some(glGenFramebuffers_inner) => glGenFramebuffers_inner(n, framebuffers),
    None => gl_not_loaded("glGenFramebuffers"),
  }
}
static glGenFramebuffers_p: GlFnCell<glGenFramebuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenFramebuffers_is_loaded() -> bool {
  unsafe { *glGenFramebuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenFramebuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenFramebuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenFramebuffers_t>>(gl_ptr_filter(f(b"glGenFramebuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenFramebuffers_reset_ptr() {
  *glGenFramebuffers_p.0.get() = None;
}
/// [glGenProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenProgramPipelines.xhtml)
/// * `pipelines` class: program pipeline
/// * `pipelines` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenProgramPipelines_p.0.get() } {
    Some(glGenProgramPipelines_inner) => glGenProgramPipelines_inner(n, pipelines),
    None => gl_not_loaded("glGenProgramPipelines"),
  }
}
static glGenProgramPipelines_p: GlFnCell<glGenProgramPipelines_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenProgramPipelines_is_loaded() -> bool {
  unsafe { *glGenProgramPipelines_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenProgramPipelines_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenProgramPipelines_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenProgramPipelines_t>>(gl_ptr_filter(f(b"glGenProgramPipelines\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenProgramPipelines_reset_ptr() {
  *glGenProgramPipelines_p.0.get() = None;
}
/// [glGenQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenQueries.xhtml)
/// * `ids` class: query
/// * `ids` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenQueries(n: GLsizei, ids: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenQueries_p.0.get() } {
    Some(glGenQueries_inner) => glGenQueries_inner(n, ids),
    None => gl_not_loaded("glGenQueries"),
  }
}
static glGenQueries_p: GlFnCell<glGenQueries_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenQueries_is_loaded() -> bool {
  unsafe { *glGenQueries_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenQueries_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenQueries_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenQueries_t>>(gl_ptr_filter(f(b"glGenQueries\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenQueries_reset_ptr() {
  *glGenQueries_p.0.get() = None;
}
/// [glGenRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenRenderbuffers.xhtml)
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenRenderbuffers_p.0.get() } {
    Some(glGenRenderbuffers_inner) => glGenRenderbuffers_inner(n, renderbuffers),
    None => gl_not_loaded("glGenRenderbuffers"),
  }
}
static glGenRenderbuffers_p: GlFnCell<glGenRenderbuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenRenderbuffers_is_loaded() -> bool {
  unsafe { *glGenRenderbuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenRenderbuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenRenderbuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenRenderbuffers_t>>(gl_ptr_filter(f(b"glGenRenderbuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenRenderbuffers_reset_ptr() {
  *glGenRenderbuffers_p.0.get() = None;
}
/// [glGenSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenSamplers.xhtml)
/// * `samplers` class: sampler
/// * `samplers` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenSamplers(count: GLsizei, samplers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenSamplers_p.0.get() } {
    Some(glGenSamplers_inner) => glGenSamplers_inner(count, samplers),
    None => gl_not_loaded("glGenSamplers"),
  }
}
static glGenSamplers_p: GlFnCell<glGenSamplers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenSamplers_is_loaded() -> bool {
  unsafe { *glGenSamplers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenSamplers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenSamplers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenSamplers_t>>(gl_ptr_filter(f(b"glGenSamplers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenSamplers_reset_ptr() {
  *glGenSamplers_p.0.get() = None;
}
/// [glGenTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTextures.xhtml)
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenTextures_p.0.get() } {
    Some(glGenTextures_inner) => glGenTextures_inner(n, textures),
    None => gl_not_loaded("glGenTextures"),
  }
}
static glGenTextures_p: GlFnCell<glGenTextures_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenTextures_is_loaded() -> bool {
  unsafe { *glGenTextures_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenTextures_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenTextures_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenTextures_t>>(gl_ptr_filter(f(b"glGenTextures\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenTextures_reset_ptr() {
  *glGenTextures_p.0.get() = None;
}
/// [glGenTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTransformFeedbacks.xhtml)
/// * `ids` class: transform feedback
/// * `ids` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenTransformFeedbacks_p.0.get() } {
    Some(glGenTransformFeedbacks_inner) => glGenTransformFeedbacks_inner(n, ids),
    None => gl_not_loaded("glGenTransformFeedbacks"),
  }
}
static glGenTransformFeedbacks_p: GlFnCell<glGenTransformFeedbacks_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenTransformFeedbacks_is_loaded() -> bool {
  unsafe { *glGenTransformFeedbacks_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenTransformFeedbacks_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenTransformFeedbacks_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenTransformFeedbacks_t>>(gl_ptr_filter(f(b"glGenTransformFeedbacks\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenTransformFeedbacks_reset_ptr() {
  *glGenTransformFeedbacks_p.0.get() = None;
}
/// [glGenVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml)
/// * `arrays` class: vertex array
/// * `arrays` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenVertexArrays_p.0.get() } {
    Some(glGenVertexArrays_inner) => glGenVertexArrays_inner(n, arrays),
    None => gl_not_loaded("glGenVertexArrays"),
  }
}
static glGenVertexArrays_p: GlFnCell<glGenVertexArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenVertexArrays_is_loaded() -> bool {
  unsafe { *glGenVertexArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenVertexArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenVertexArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenVertexArrays_t>>(gl_ptr_filter(f(b"glGenVertexArrays\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenVertexArrays_reset_ptr() {
  *glGenVertexArrays_p.0.get() = None;
}
/// [glGenerateMipmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateMipmap.xhtml)
/// * `target` group: TextureTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenerateMipmap(target: TextureTarget) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenerateMipmap_p.0.get() } {
    Some(glGenerateMipmap_inner) => glGenerateMipmap_inner(target),
    None => gl_not_loaded("glGenerateMipmap"),
  }
}
static glGenerateMipmap_p: GlFnCell<glGenerateMipmap_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenerateMipmap_is_loaded() -> bool {
  unsafe { *glGenerateMipmap_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenerateMipmap_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenerateMipmap_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenerateMipmap_t>>(gl_ptr_filter(f(b"glGenerateMipmap\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenerateMipmap_reset_ptr() {
  *glGenerateMipmap_p.0.get() = None;
}
/// [glGenerateTextureMipmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateTextureMipmap.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenerateTextureMipmap(texture: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenerateTextureMipmap_p.0.get() } {
    Some(glGenerateTextureMipmap_inner) => glGenerateTextureMipmap_inner(texture),
    None => gl_not_loaded("glGenerateTextureMipmap"),
  }
}
static glGenerateTextureMipmap_p: GlFnCell<glGenerateTextureMipmap_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenerateTextureMipmap_is_loaded() -> bool {
  unsafe { *glGenerateTextureMipmap_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenerateTextureMipmap_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenerateTextureMipmap_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenerateTextureMipmap_t>>(gl_ptr_filter(f(b"glGenerateTextureMipmap\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGenerateTextureMipmap_reset_ptr() {
  *glGenerateTextureMipmap_p.0.get() = None;
}
/// [glGetActiveAtomicCounterBufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveAtomicCounterBufferiv.xhtml)
/// * `program` class: program
/// * `pname` group: AtomicCounterBufferPName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: AtomicCounterBufferPName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveAtomicCounterBufferiv_p.0.get() } {
    Some(glGetActiveAtomicCounterBufferiv_inner) => glGetActiveAtomicCounterBufferiv_inner(program, bufferIndex, pname, params),
    None => gl_not_loaded("glGetActiveAtomicCounterBufferiv"),
  }
}
static glGetActiveAtomicCounterBufferiv_p: GlFnCell<glGetActiveAtomicCounterBufferiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveAtomicCounterBufferiv_is_loaded() -> bool {
  unsafe { *glGetActiveAtomicCounterBufferiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveAtomicCounterBufferiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveAtomicCounterBufferiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveAtomicCounterBufferiv_t>>(gl_ptr_filter(f(b"glGetActiveAtomicCounterBufferiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveAtomicCounterBufferiv_reset_ptr() {
  *glGetActiveAtomicCounterBufferiv_p.0.get() = None;
}
/// [glGetActiveAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveAttrib.xhtml)
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveAttrib_p.0.get() } {
    Some(glGetActiveAttrib_inner) => glGetActiveAttrib_inner(program, index, bufSize, length, size, type_, name),
    None => gl_not_loaded("glGetActiveAttrib"),
  }
}
static glGetActiveAttrib_p: GlFnCell<glGetActiveAttrib_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveAttrib_is_loaded() -> bool {
  unsafe { *glGetActiveAttrib_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveAttrib_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveAttrib_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveAttrib_t>>(gl_ptr_filter(f(b"glGetActiveAttrib\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveAttrib_reset_ptr() {
  *glGetActiveAttrib_p.0.get() = None;
}
/// [glGetActiveSubroutineName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineName.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveSubroutineName(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveSubroutineName_p.0.get() } {
    Some(glGetActiveSubroutineName_inner) => glGetActiveSubroutineName_inner(program, shadertype, index, bufSize, length, name),
    None => gl_not_loaded("glGetActiveSubroutineName"),
  }
}
static glGetActiveSubroutineName_p: GlFnCell<glGetActiveSubroutineName_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveSubroutineName_is_loaded() -> bool {
  unsafe { *glGetActiveSubroutineName_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveSubroutineName_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveSubroutineName_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveSubroutineName_t>>(gl_ptr_filter(f(b"glGetActiveSubroutineName\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveSubroutineName_reset_ptr() {
  *glGetActiveSubroutineName_p.0.get() = None;
}
/// [glGetActiveSubroutineUniformName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineUniformName.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveSubroutineUniformName(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveSubroutineUniformName_p.0.get() } {
    Some(glGetActiveSubroutineUniformName_inner) => glGetActiveSubroutineUniformName_inner(program, shadertype, index, bufSize, length, name),
    None => gl_not_loaded("glGetActiveSubroutineUniformName"),
  }
}
static glGetActiveSubroutineUniformName_p: GlFnCell<glGetActiveSubroutineUniformName_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveSubroutineUniformName_is_loaded() -> bool {
  unsafe { *glGetActiveSubroutineUniformName_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveSubroutineUniformName_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveSubroutineUniformName_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveSubroutineUniformName_t>>(gl_ptr_filter(f(b"glGetActiveSubroutineUniformName\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveSubroutineUniformName_reset_ptr() {
  *glGetActiveSubroutineUniformName_p.0.get() = None;
}
/// [glGetActiveSubroutineUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineUniformiv.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `pname` group: SubroutineParameterName
/// * `values` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveSubroutineUniformiv(program: GLuint, shadertype: ShaderType, index: GLuint, pname: SubroutineParameterName, values: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveSubroutineUniformiv_p.0.get() } {
    Some(glGetActiveSubroutineUniformiv_inner) => glGetActiveSubroutineUniformiv_inner(program, shadertype, index, pname, values),
    None => gl_not_loaded("glGetActiveSubroutineUniformiv"),
  }
}
static glGetActiveSubroutineUniformiv_p: GlFnCell<glGetActiveSubroutineUniformiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveSubroutineUniformiv_is_loaded() -> bool {
  unsafe { *glGetActiveSubroutineUniformiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveSubroutineUniformiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveSubroutineUniformiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveSubroutineUniformiv_t>>(gl_ptr_filter(f(b"glGetActiveSubroutineUniformiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveSubroutineUniformiv_reset_ptr() {
  *glGetActiveSubroutineUniformiv_p.0.get() = None;
}
/// [glGetActiveUniform](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniform.xhtml)
/// * `program` class: program
/// * `type` group: UniformType
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniform_p.0.get() } {
    Some(glGetActiveUniform_inner) => glGetActiveUniform_inner(program, index, bufSize, length, size, type_, name),
    None => gl_not_loaded("glGetActiveUniform"),
  }
}
static glGetActiveUniform_p: GlFnCell<glGetActiveUniform_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniform_is_loaded() -> bool {
  unsafe { *glGetActiveUniform_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniform_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniform_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniform_t>>(gl_ptr_filter(f(b"glGetActiveUniform\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniform_reset_ptr() {
  *glGetActiveUniform_p.0.get() = None;
}
/// [glGetActiveUniformBlockName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformBlockName.xhtml)
/// * `program` class: program
/// * `uniformBlockName` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniformBlockName_p.0.get() } {
    Some(glGetActiveUniformBlockName_inner) => glGetActiveUniformBlockName_inner(program, uniformBlockIndex, bufSize, length, uniformBlockName),
    None => gl_not_loaded("glGetActiveUniformBlockName"),
  }
}
static glGetActiveUniformBlockName_p: GlFnCell<glGetActiveUniformBlockName_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniformBlockName_is_loaded() -> bool {
  unsafe { *glGetActiveUniformBlockName_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformBlockName_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniformBlockName_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniformBlockName_t>>(gl_ptr_filter(f(b"glGetActiveUniformBlockName\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformBlockName_reset_ptr() {
  *glGetActiveUniformBlockName_p.0.get() = None;
}
/// [glGetActiveUniformBlockiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformBlockiv.xhtml)
/// * `program` class: program
/// * `pname` group: UniformBlockPName
/// * `params` len: COMPSIZE(program,uniformBlockIndex,pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniformBlockiv_p.0.get() } {
    Some(glGetActiveUniformBlockiv_inner) => glGetActiveUniformBlockiv_inner(program, uniformBlockIndex, pname, params),
    None => gl_not_loaded("glGetActiveUniformBlockiv"),
  }
}
static glGetActiveUniformBlockiv_p: GlFnCell<glGetActiveUniformBlockiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniformBlockiv_is_loaded() -> bool {
  unsafe { *glGetActiveUniformBlockiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformBlockiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniformBlockiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniformBlockiv_t>>(gl_ptr_filter(f(b"glGetActiveUniformBlockiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformBlockiv_reset_ptr() {
  *glGetActiveUniformBlockiv_p.0.get() = None;
}
/// [glGetActiveUniformName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformName.xhtml)
/// * `program` class: program
/// * `uniformName` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniformName_p.0.get() } {
    Some(glGetActiveUniformName_inner) => glGetActiveUniformName_inner(program, uniformIndex, bufSize, length, uniformName),
    None => gl_not_loaded("glGetActiveUniformName"),
  }
}
static glGetActiveUniformName_p: GlFnCell<glGetActiveUniformName_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniformName_is_loaded() -> bool {
  unsafe { *glGetActiveUniformName_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformName_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniformName_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniformName_t>>(gl_ptr_filter(f(b"glGetActiveUniformName\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformName_reset_ptr() {
  *glGetActiveUniformName_p.0.get() = None;
}
/// [glGetActiveUniformsiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformsiv.xhtml)
/// * `program` class: program
/// * `uniformIndices` len: uniformCount
/// * `pname` group: UniformPName
/// * `params` len: COMPSIZE(uniformCount,pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniformsiv_p.0.get() } {
    Some(glGetActiveUniformsiv_inner) => glGetActiveUniformsiv_inner(program, uniformCount, uniformIndices, pname, params),
    None => gl_not_loaded("glGetActiveUniformsiv"),
  }
}
static glGetActiveUniformsiv_p: GlFnCell<glGetActiveUniformsiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniformsiv_is_loaded() -> bool {
  unsafe { *glGetActiveUniformsiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformsiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniformsiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniformsiv_t>>(gl_ptr_filter(f(b"glGetActiveUniformsiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformsiv_reset_ptr() {
  *glGetActiveUniformsiv_p.0.get() = None;
}
/// [glGetAttachedShaders](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttachedShaders.xhtml)
/// * `program` class: program
/// * `shaders` class: shader
/// * `shaders` len: maxCount
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetAttachedShaders_p.0.get() } {
    Some(glGetAttachedShaders_inner) => glGetAttachedShaders_inner(program, maxCount, count, shaders),
    None => gl_not_loaded("glGetAttachedShaders"),
  }
}
static glGetAttachedShaders_p: GlFnCell<glGetAttachedShaders_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetAttachedShaders_is_loaded() -> bool {
  unsafe { *glGetAttachedShaders_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetAttachedShaders_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetAttachedShaders_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetAttachedShaders_t>>(gl_ptr_filter(f(b"glGetAttachedShaders\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetAttachedShaders_reset_ptr() {
  *glGetAttachedShaders_p.0.get() = None;
}
/// [glGetAttribLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttribLocation.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetAttribLocation_p.0.get() } {
    Some(glGetAttribLocation_inner) => glGetAttribLocation_inner(program, name),
    None => gl_not_loaded("glGetAttribLocation"),
  }
}
static glGetAttribLocation_p: GlFnCell<glGetAttribLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetAttribLocation_is_loaded() -> bool {
  unsafe { *glGetAttribLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetAttribLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetAttribLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetAttribLocation_t>>(gl_ptr_filter(f(b"glGetAttribLocation\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetAttribLocation_reset_ptr() {
  *glGetAttribLocation_p.0.get() = None;
}
/// [glGetBooleani_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBooleani_v.xhtml)
/// * `target` group: BufferTargetARB
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(target)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBooleani_v(target: BufferTargetARB, index: GLuint, data: *mut GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBooleani_v_p.0.get() } {
    Some(glGetBooleani_v_inner) => glGetBooleani_v_inner(target, index, data),
    None => gl_not_loaded("glGetBooleani_v"),
  }
}
static glGetBooleani_v_p: GlFnCell<glGetBooleani_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBooleani_v_is_loaded() -> bool {
  unsafe { *glGetBooleani_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBooleani_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBooleani_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBooleani_v_t>>(gl_ptr_filter(f(b"glGetBooleani_v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetBooleani_v_reset_ptr() {
  *glGetBooleani_v_p.0.get() = None;
}
/// [glGetBooleanv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBooleanv.xhtml)
/// * `pname` group: GetPName
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBooleanv(pname: GetPName, data: *mut GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBooleanv_p.0.get() } {
    Some(glGetBooleanv_inner) => glGetBooleanv_inner(pname, data),
    None => gl_not_loaded("glGetBooleanv"),
  }
}
static glGetBooleanv_p: GlFnCell<glGetBooleanv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBooleanv_is_loaded() -> bool {
  unsafe { *glGetBooleanv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBooleanv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBooleanv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBooleanv_t>>(gl_ptr_filter(f(b"glGetBooleanv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetBooleanv_reset_ptr() {
  *glGetBooleanv_p.0.get() = None;
}
/// [glGetBufferParameteri64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferParameteri64v.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBufferParameteri64v(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBufferParameteri64v_p.0.get() } {
    Some(glGetBufferParameteri64v_inner) => glGetBufferParameteri64v_inner(target, pname, params),
    None => gl_not_loaded("glGetBufferParameteri64v"),
  }
}
static glGetBufferParameteri64v_p: GlFnCell<glGetBufferParameteri64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBufferParameteri64v_is_loaded() -> bool {
  unsafe { *glGetBufferParameteri64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBufferParameteri64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBufferParameteri64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBufferParameteri64v_t>>(gl_ptr_filter(f(b"glGetBufferParameteri64v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetBufferParameteri64v_reset_ptr() {
  *glGetBufferParameteri64v_p.0.get() = None;
}
/// [glGetBufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferParameteriv.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBufferParameteriv(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBufferParameteriv_p.0.get() } {
    Some(glGetBufferParameteriv_inner) => glGetBufferParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glGetBufferParameteriv"),
  }
}
static glGetBufferParameteriv_p: GlFnCell<glGetBufferParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBufferParameteriv_is_loaded() -> bool {
  unsafe { *glGetBufferParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBufferParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBufferParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBufferParameteriv_t>>(gl_ptr_filter(f(b"glGetBufferParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetBufferParameteriv_reset_ptr() {
  *glGetBufferParameteriv_p.0.get() = None;
}
/// [glGetBufferPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferPointerv.xhtml)
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPointerNameARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBufferPointerv(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBufferPointerv_p.0.get() } {
    Some(glGetBufferPointerv_inner) => glGetBufferPointerv_inner(target, pname, params),
    None => gl_not_loaded("glGetBufferPointerv"),
  }
}
static glGetBufferPointerv_p: GlFnCell<glGetBufferPointerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBufferPointerv_is_loaded() -> bool {
  unsafe { *glGetBufferPointerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBufferPointerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBufferPointerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBufferPointerv_t>>(gl_ptr_filter(f(b"glGetBufferPointerv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetBufferPointerv_reset_ptr() {
  *glGetBufferPointerv_p.0.get() = None;
}
/// [glGetBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferSubData.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `data` len: size
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBufferSubData(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBufferSubData_p.0.get() } {
    Some(glGetBufferSubData_inner) => glGetBufferSubData_inner(target, offset, size, data),
    None => gl_not_loaded("glGetBufferSubData"),
  }
}
static glGetBufferSubData_p: GlFnCell<glGetBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBufferSubData_is_loaded() -> bool {
  unsafe { *glGetBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBufferSubData_t>>(gl_ptr_filter(f(b"glGetBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetBufferSubData_reset_ptr() {
  *glGetBufferSubData_p.0.get() = None;
}
/// [glGetCompressedTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTexImage.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `img` group: CompressedTextureARB
/// * `img` len: COMPSIZE(target,level)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetCompressedTexImage(target: TextureTarget, level: GLint, img: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetCompressedTexImage_p.0.get() } {
    Some(glGetCompressedTexImage_inner) => glGetCompressedTexImage_inner(target, level, img),
    None => gl_not_loaded("glGetCompressedTexImage"),
  }
}
static glGetCompressedTexImage_p: GlFnCell<glGetCompressedTexImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetCompressedTexImage_is_loaded() -> bool {
  unsafe { *glGetCompressedTexImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetCompressedTexImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetCompressedTexImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetCompressedTexImage_t>>(gl_ptr_filter(f(b"glGetCompressedTexImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetCompressedTexImage_reset_ptr() {
  *glGetCompressedTexImage_p.0.get() = None;
}
/// [glGetCompressedTextureImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTextureImage.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetCompressedTextureImage(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetCompressedTextureImage_p.0.get() } {
    Some(glGetCompressedTextureImage_inner) => glGetCompressedTextureImage_inner(texture, level, bufSize, pixels),
    None => gl_not_loaded("glGetCompressedTextureImage"),
  }
}
static glGetCompressedTextureImage_p: GlFnCell<glGetCompressedTextureImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetCompressedTextureImage_is_loaded() -> bool {
  unsafe { *glGetCompressedTextureImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetCompressedTextureImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetCompressedTextureImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetCompressedTextureImage_t>>(gl_ptr_filter(f(b"glGetCompressedTextureImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetCompressedTextureImage_reset_ptr() {
  *glGetCompressedTextureImage_p.0.get() = None;
}
/// [glGetCompressedTextureSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTextureSubImage.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetCompressedTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetCompressedTextureSubImage_p.0.get() } {
    Some(glGetCompressedTextureSubImage_inner) => glGetCompressedTextureSubImage_inner(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels),
    None => gl_not_loaded("glGetCompressedTextureSubImage"),
  }
}
static glGetCompressedTextureSubImage_p: GlFnCell<glGetCompressedTextureSubImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetCompressedTextureSubImage_is_loaded() -> bool {
  unsafe { *glGetCompressedTextureSubImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetCompressedTextureSubImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetCompressedTextureSubImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetCompressedTextureSubImage_t>>(gl_ptr_filter(f(b"glGetCompressedTextureSubImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetCompressedTextureSubImage_reset_ptr() {
  *glGetCompressedTextureSubImage_p.0.get() = None;
}
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
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetDebugMessageLog_p.0.get() } {
    Some(glGetDebugMessageLog_inner) => glGetDebugMessageLog_inner(count, bufSize, sources, types, ids, severities, lengths, messageLog),
    None => gl_not_loaded("glGetDebugMessageLog"),
  }
}
static glGetDebugMessageLog_p: GlFnCell<glGetDebugMessageLog_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetDebugMessageLog_is_loaded() -> bool {
  unsafe { *glGetDebugMessageLog_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetDebugMessageLog_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetDebugMessageLog_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetDebugMessageLog_t>>(gl_ptr_filter(f(b"glGetDebugMessageLog\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetDebugMessageLog_reset_ptr() {
  *glGetDebugMessageLog_p.0.get() = None;
}
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
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetDebugMessageLogARB(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetDebugMessageLogARB_p.0.get() } {
    Some(glGetDebugMessageLogARB_inner) => glGetDebugMessageLogARB_inner(count, bufSize, sources, types, ids, severities, lengths, messageLog),
    None => gl_not_loaded("glGetDebugMessageLogARB"),
  }
}
static glGetDebugMessageLogARB_p: GlFnCell<glGetDebugMessageLogARB_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetDebugMessageLogARB_is_loaded() -> bool {
  unsafe { *glGetDebugMessageLogARB_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetDebugMessageLogARB_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetDebugMessageLogARB_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetDebugMessageLogARB_t>>(gl_ptr_filter(f(b"glGetDebugMessageLogARB\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetDebugMessageLogARB_reset_ptr() {
  *glGetDebugMessageLogARB_p.0.get() = None;
}
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
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetDebugMessageLogKHR(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetDebugMessageLogKHR_p.0.get() } {
    Some(glGetDebugMessageLogKHR_inner) => glGetDebugMessageLogKHR_inner(count, bufSize, sources, types, ids, severities, lengths, messageLog),
    None => gl_not_loaded("glGetDebugMessageLogKHR"),
  }
}
static glGetDebugMessageLogKHR_p: GlFnCell<glGetDebugMessageLogKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetDebugMessageLogKHR_is_loaded() -> bool {
  unsafe { *glGetDebugMessageLogKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetDebugMessageLogKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetDebugMessageLogKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetDebugMessageLogKHR_t>>(gl_ptr_filter(f(b"glGetDebugMessageLogKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetDebugMessageLogKHR_reset_ptr() {
  *glGetDebugMessageLogKHR_p.0.get() = None;
}
/// [glGetDoublei_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDoublei_v.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetDoublei_v(target: GetPName, index: GLuint, data: *mut GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetDoublei_v_p.0.get() } {
    Some(glGetDoublei_v_inner) => glGetDoublei_v_inner(target, index, data),
    None => gl_not_loaded("glGetDoublei_v"),
  }
}
static glGetDoublei_v_p: GlFnCell<glGetDoublei_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetDoublei_v_is_loaded() -> bool {
  unsafe { *glGetDoublei_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetDoublei_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetDoublei_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetDoublei_v_t>>(gl_ptr_filter(f(b"glGetDoublei_v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetDoublei_v_reset_ptr() {
  *glGetDoublei_v_p.0.get() = None;
}
/// [glGetDoublev](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDoublev.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetDoublev(pname: GetPName, data: *mut GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetDoublev_p.0.get() } {
    Some(glGetDoublev_inner) => glGetDoublev_inner(pname, data),
    None => gl_not_loaded("glGetDoublev"),
  }
}
static glGetDoublev_p: GlFnCell<glGetDoublev_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetDoublev_is_loaded() -> bool {
  unsafe { *glGetDoublev_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetDoublev_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetDoublev_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetDoublev_t>>(gl_ptr_filter(f(b"glGetDoublev\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetDoublev_reset_ptr() {
  *glGetDoublev_p.0.get() = None;
}
/// [glGetError](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetError() -> ErrorCode {
  #[allow(unused_unsafe)]
  match unsafe { *glGetError_p.0.get() } {
    Some(glGetError_inner) => glGetError_inner(),
    None => gl_not_loaded("glGetError"),
  }
}
static glGetError_p: GlFnCell<glGetError_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetError_is_loaded() -> bool {
  unsafe { *glGetError_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetError_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetError_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetError_t>>(gl_ptr_filter(f(b"glGetError\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetError_reset_ptr() {
  *glGetError_p.0.get() = None;
}
/// [glGetFloati_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloati_v.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFloati_v(target: GetPName, index: GLuint, data: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFloati_v_p.0.get() } {
    Some(glGetFloati_v_inner) => glGetFloati_v_inner(target, index, data),
    None => gl_not_loaded("glGetFloati_v"),
  }
}
static glGetFloati_v_p: GlFnCell<glGetFloati_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFloati_v_is_loaded() -> bool {
  unsafe { *glGetFloati_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFloati_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFloati_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFloati_v_t>>(gl_ptr_filter(f(b"glGetFloati_v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetFloati_v_reset_ptr() {
  *glGetFloati_v_p.0.get() = None;
}
/// [glGetFloatv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloatv.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFloatv(pname: GetPName, data: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFloatv_p.0.get() } {
    Some(glGetFloatv_inner) => glGetFloatv_inner(pname, data),
    None => gl_not_loaded("glGetFloatv"),
  }
}
static glGetFloatv_p: GlFnCell<glGetFloatv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFloatv_is_loaded() -> bool {
  unsafe { *glGetFloatv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFloatv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFloatv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFloatv_t>>(gl_ptr_filter(f(b"glGetFloatv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetFloatv_reset_ptr() {
  *glGetFloatv_p.0.get() = None;
}
/// [glGetFragDataIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragDataIndex.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFragDataIndex_p.0.get() } {
    Some(glGetFragDataIndex_inner) => glGetFragDataIndex_inner(program, name),
    None => gl_not_loaded("glGetFragDataIndex"),
  }
}
static glGetFragDataIndex_p: GlFnCell<glGetFragDataIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFragDataIndex_is_loaded() -> bool {
  unsafe { *glGetFragDataIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFragDataIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFragDataIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFragDataIndex_t>>(gl_ptr_filter(f(b"glGetFragDataIndex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetFragDataIndex_reset_ptr() {
  *glGetFragDataIndex_p.0.get() = None;
}
/// [glGetFragDataLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragDataLocation.xhtml)
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFragDataLocation_p.0.get() } {
    Some(glGetFragDataLocation_inner) => glGetFragDataLocation_inner(program, name),
    None => gl_not_loaded("glGetFragDataLocation"),
  }
}
static glGetFragDataLocation_p: GlFnCell<glGetFragDataLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFragDataLocation_is_loaded() -> bool {
  unsafe { *glGetFragDataLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFragDataLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFragDataLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFragDataLocation_t>>(gl_ptr_filter(f(b"glGetFragDataLocation\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetFragDataLocation_reset_ptr() {
  *glGetFragDataLocation_p.0.get() = None;
}
/// [glGetFramebufferAttachmentParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferAttachmentParameteriv.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFramebufferAttachmentParameteriv(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFramebufferAttachmentParameteriv_p.0.get() } {
    Some(glGetFramebufferAttachmentParameteriv_inner) => glGetFramebufferAttachmentParameteriv_inner(target, attachment, pname, params),
    None => gl_not_loaded("glGetFramebufferAttachmentParameteriv"),
  }
}
static glGetFramebufferAttachmentParameteriv_p: GlFnCell<glGetFramebufferAttachmentParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFramebufferAttachmentParameteriv_is_loaded() -> bool {
  unsafe { *glGetFramebufferAttachmentParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFramebufferAttachmentParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFramebufferAttachmentParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFramebufferAttachmentParameteriv_t>>(gl_ptr_filter(f(b"glGetFramebufferAttachmentParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetFramebufferAttachmentParameteriv_reset_ptr() {
  *glGetFramebufferAttachmentParameteriv_p.0.get() = None;
}
/// [glGetFramebufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferParameteriv.xhtml)
/// * `target` group: FramebufferTarget
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFramebufferParameteriv(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFramebufferParameteriv_p.0.get() } {
    Some(glGetFramebufferParameteriv_inner) => glGetFramebufferParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glGetFramebufferParameteriv"),
  }
}
static glGetFramebufferParameteriv_p: GlFnCell<glGetFramebufferParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFramebufferParameteriv_is_loaded() -> bool {
  unsafe { *glGetFramebufferParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFramebufferParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFramebufferParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFramebufferParameteriv_t>>(gl_ptr_filter(f(b"glGetFramebufferParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetFramebufferParameteriv_reset_ptr() {
  *glGetFramebufferParameteriv_p.0.get() = None;
}
/// [glGetGraphicsResetStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetGraphicsResetStatus.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetGraphicsResetStatus() -> GraphicsResetStatus {
  #[allow(unused_unsafe)]
  match unsafe { *glGetGraphicsResetStatus_p.0.get() } {
    Some(glGetGraphicsResetStatus_inner) => glGetGraphicsResetStatus_inner(),
    None => gl_not_loaded("glGetGraphicsResetStatus"),
  }
}
static glGetGraphicsResetStatus_p: GlFnCell<glGetGraphicsResetStatus_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetGraphicsResetStatus_is_loaded() -> bool {
  unsafe { *glGetGraphicsResetStatus_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetGraphicsResetStatus_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetGraphicsResetStatus_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetGraphicsResetStatus_t>>(gl_ptr_filter(f(b"glGetGraphicsResetStatus\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetGraphicsResetStatus_reset_ptr() {
  *glGetGraphicsResetStatus_p.0.get() = None;
}
/// [glGetInteger64i_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInteger64i_v.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetInteger64i_v(target: GetPName, index: GLuint, data: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetInteger64i_v_p.0.get() } {
    Some(glGetInteger64i_v_inner) => glGetInteger64i_v_inner(target, index, data),
    None => gl_not_loaded("glGetInteger64i_v"),
  }
}
static glGetInteger64i_v_p: GlFnCell<glGetInteger64i_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetInteger64i_v_is_loaded() -> bool {
  unsafe { *glGetInteger64i_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetInteger64i_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetInteger64i_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetInteger64i_v_t>>(gl_ptr_filter(f(b"glGetInteger64i_v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetInteger64i_v_reset_ptr() {
  *glGetInteger64i_v_p.0.get() = None;
}
/// [glGetInteger64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInteger64v.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetInteger64v(pname: GetPName, data: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetInteger64v_p.0.get() } {
    Some(glGetInteger64v_inner) => glGetInteger64v_inner(pname, data),
    None => gl_not_loaded("glGetInteger64v"),
  }
}
static glGetInteger64v_p: GlFnCell<glGetInteger64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetInteger64v_is_loaded() -> bool {
  unsafe { *glGetInteger64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetInteger64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetInteger64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetInteger64v_t>>(gl_ptr_filter(f(b"glGetInteger64v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetInteger64v_reset_ptr() {
  *glGetInteger64v_p.0.get() = None;
}
/// [glGetIntegeri_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegeri_v.xhtml)
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetIntegeri_v(target: GetPName, index: GLuint, data: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetIntegeri_v_p.0.get() } {
    Some(glGetIntegeri_v_inner) => glGetIntegeri_v_inner(target, index, data),
    None => gl_not_loaded("glGetIntegeri_v"),
  }
}
static glGetIntegeri_v_p: GlFnCell<glGetIntegeri_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetIntegeri_v_is_loaded() -> bool {
  unsafe { *glGetIntegeri_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetIntegeri_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetIntegeri_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetIntegeri_v_t>>(gl_ptr_filter(f(b"glGetIntegeri_v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetIntegeri_v_reset_ptr() {
  *glGetIntegeri_v_p.0.get() = None;
}
/// [glGetIntegerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegerv.xhtml)
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetIntegerv(pname: GetPName, data: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetIntegerv_p.0.get() } {
    Some(glGetIntegerv_inner) => glGetIntegerv_inner(pname, data),
    None => gl_not_loaded("glGetIntegerv"),
  }
}
static glGetIntegerv_p: GlFnCell<glGetIntegerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetIntegerv_is_loaded() -> bool {
  unsafe { *glGetIntegerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetIntegerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetIntegerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetIntegerv_t>>(gl_ptr_filter(f(b"glGetIntegerv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetIntegerv_reset_ptr() {
  *glGetIntegerv_p.0.get() = None;
}
/// [glGetInternalformati64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInternalformati64v.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `pname` group: InternalFormatPName
/// * `params` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetInternalformati64v(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetInternalformati64v_p.0.get() } {
    Some(glGetInternalformati64v_inner) => glGetInternalformati64v_inner(target, internalformat, pname, count, params),
    None => gl_not_loaded("glGetInternalformati64v"),
  }
}
static glGetInternalformati64v_p: GlFnCell<glGetInternalformati64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetInternalformati64v_is_loaded() -> bool {
  unsafe { *glGetInternalformati64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetInternalformati64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetInternalformati64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetInternalformati64v_t>>(gl_ptr_filter(f(b"glGetInternalformati64v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetInternalformati64v_reset_ptr() {
  *glGetInternalformati64v_p.0.get() = None;
}
/// [glGetInternalformativ](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInternalformativ.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `pname` group: InternalFormatPName
/// * `params` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetInternalformativ(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetInternalformativ_p.0.get() } {
    Some(glGetInternalformativ_inner) => glGetInternalformativ_inner(target, internalformat, pname, count, params),
    None => gl_not_loaded("glGetInternalformativ"),
  }
}
static glGetInternalformativ_p: GlFnCell<glGetInternalformativ_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetInternalformativ_is_loaded() -> bool {
  unsafe { *glGetInternalformativ_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetInternalformativ_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetInternalformativ_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetInternalformativ_t>>(gl_ptr_filter(f(b"glGetInternalformativ\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetInternalformativ_reset_ptr() {
  *glGetInternalformativ_p.0.get() = None;
}
/// [glGetMultisamplefv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultisamplefv.xhtml)
/// * `pname` group: GetMultisamplePNameNV
/// * `val` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetMultisamplefv(pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetMultisamplefv_p.0.get() } {
    Some(glGetMultisamplefv_inner) => glGetMultisamplefv_inner(pname, index, val),
    None => gl_not_loaded("glGetMultisamplefv"),
  }
}
static glGetMultisamplefv_p: GlFnCell<glGetMultisamplefv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetMultisamplefv_is_loaded() -> bool {
  unsafe { *glGetMultisamplefv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetMultisamplefv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetMultisamplefv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetMultisamplefv_t>>(gl_ptr_filter(f(b"glGetMultisamplefv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetMultisamplefv_reset_ptr() {
  *glGetMultisamplefv_p.0.get() = None;
}
/// [glGetNamedBufferParameteri64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferParameteri64v.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetNamedBufferParameteri64v(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetNamedBufferParameteri64v_p.0.get() } {
    Some(glGetNamedBufferParameteri64v_inner) => glGetNamedBufferParameteri64v_inner(buffer, pname, params),
    None => gl_not_loaded("glGetNamedBufferParameteri64v"),
  }
}
static glGetNamedBufferParameteri64v_p: GlFnCell<glGetNamedBufferParameteri64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetNamedBufferParameteri64v_is_loaded() -> bool {
  unsafe { *glGetNamedBufferParameteri64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetNamedBufferParameteri64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetNamedBufferParameteri64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetNamedBufferParameteri64v_t>>(gl_ptr_filter(f(b"glGetNamedBufferParameteri64v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetNamedBufferParameteri64v_reset_ptr() {
  *glGetNamedBufferParameteri64v_p.0.get() = None;
}
/// [glGetNamedBufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferParameteriv.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPNameARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetNamedBufferParameteriv(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetNamedBufferParameteriv_p.0.get() } {
    Some(glGetNamedBufferParameteriv_inner) => glGetNamedBufferParameteriv_inner(buffer, pname, params),
    None => gl_not_loaded("glGetNamedBufferParameteriv"),
  }
}
static glGetNamedBufferParameteriv_p: GlFnCell<glGetNamedBufferParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetNamedBufferParameteriv_is_loaded() -> bool {
  unsafe { *glGetNamedBufferParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetNamedBufferParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetNamedBufferParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetNamedBufferParameteriv_t>>(gl_ptr_filter(f(b"glGetNamedBufferParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetNamedBufferParameteriv_reset_ptr() {
  *glGetNamedBufferParameteriv_p.0.get() = None;
}
/// [glGetNamedBufferPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferPointerv.xhtml)
/// * `buffer` class: buffer
/// * `pname` group: BufferPointerNameARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetNamedBufferPointerv(buffer: GLuint, pname: BufferPointerNameARB, params: *mut *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetNamedBufferPointerv_p.0.get() } {
    Some(glGetNamedBufferPointerv_inner) => glGetNamedBufferPointerv_inner(buffer, pname, params),
    None => gl_not_loaded("glGetNamedBufferPointerv"),
  }
}
static glGetNamedBufferPointerv_p: GlFnCell<glGetNamedBufferPointerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetNamedBufferPointerv_is_loaded() -> bool {
  unsafe { *glGetNamedBufferPointerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetNamedBufferPointerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetNamedBufferPointerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetNamedBufferPointerv_t>>(gl_ptr_filter(f(b"glGetNamedBufferPointerv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetNamedBufferPointerv_reset_ptr() {
  *glGetNamedBufferPointerv_p.0.get() = None;
}
/// [glGetNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferSubData.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetNamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetNamedBufferSubData_p.0.get() } {
    Some(glGetNamedBufferSubData_inner) => glGetNamedBufferSubData_inner(buffer, offset, size, data),
    None => gl_not_loaded("glGetNamedBufferSubData"),
  }
}
static glGetNamedBufferSubData_p: GlFnCell<glGetNamedBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetNamedBufferSubData_is_loaded() -> bool {
  unsafe { *glGetNamedBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetNamedBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetNamedBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetNamedBufferSubData_t>>(gl_ptr_filter(f(b"glGetNamedBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetNamedBufferSubData_reset_ptr() {
  *glGetNamedBufferSubData_p.0.get() = None;
}
/// [glGetNamedFramebufferAttachmentParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferAttachmentParameteriv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetNamedFramebufferAttachmentParameteriv(framebuffer: GLuint, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetNamedFramebufferAttachmentParameteriv_p.0.get() } {
    Some(glGetNamedFramebufferAttachmentParameteriv_inner) => glGetNamedFramebufferAttachmentParameteriv_inner(framebuffer, attachment, pname, params),
    None => gl_not_loaded("glGetNamedFramebufferAttachmentParameteriv"),
  }
}
static glGetNamedFramebufferAttachmentParameteriv_p: GlFnCell<glGetNamedFramebufferAttachmentParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetNamedFramebufferAttachmentParameteriv_is_loaded() -> bool {
  unsafe { *glGetNamedFramebufferAttachmentParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetNamedFramebufferAttachmentParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetNamedFramebufferAttachmentParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetNamedFramebufferAttachmentParameteriv_t>>(gl_ptr_filter(f(b"glGetNamedFramebufferAttachmentParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetNamedFramebufferAttachmentParameteriv_reset_ptr() {
  *glGetNamedFramebufferAttachmentParameteriv_p.0.get() = None;
}
/// [glGetNamedFramebufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferParameteriv.xhtml)
/// * `framebuffer` class: framebuffer
/// * `pname` group: GetFramebufferParameter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetNamedFramebufferParameteriv(framebuffer: GLuint, pname: GetFramebufferParameter, param: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetNamedFramebufferParameteriv_p.0.get() } {
    Some(glGetNamedFramebufferParameteriv_inner) => glGetNamedFramebufferParameteriv_inner(framebuffer, pname, param),
    None => gl_not_loaded("glGetNamedFramebufferParameteriv"),
  }
}
static glGetNamedFramebufferParameteriv_p: GlFnCell<glGetNamedFramebufferParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetNamedFramebufferParameteriv_is_loaded() -> bool {
  unsafe { *glGetNamedFramebufferParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetNamedFramebufferParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetNamedFramebufferParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetNamedFramebufferParameteriv_t>>(gl_ptr_filter(f(b"glGetNamedFramebufferParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetNamedFramebufferParameteriv_reset_ptr() {
  *glGetNamedFramebufferParameteriv_p.0.get() = None;
}
/// [glGetNamedRenderbufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedRenderbufferParameteriv.xhtml)
/// * `renderbuffer` class: renderbuffer
/// * `pname` group: RenderbufferParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetNamedRenderbufferParameteriv(renderbuffer: GLuint, pname: RenderbufferParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetNamedRenderbufferParameteriv_p.0.get() } {
    Some(glGetNamedRenderbufferParameteriv_inner) => glGetNamedRenderbufferParameteriv_inner(renderbuffer, pname, params),
    None => gl_not_loaded("glGetNamedRenderbufferParameteriv"),
  }
}
static glGetNamedRenderbufferParameteriv_p: GlFnCell<glGetNamedRenderbufferParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetNamedRenderbufferParameteriv_is_loaded() -> bool {
  unsafe { *glGetNamedRenderbufferParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetNamedRenderbufferParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetNamedRenderbufferParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetNamedRenderbufferParameteriv_t>>(gl_ptr_filter(f(b"glGetNamedRenderbufferParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetNamedRenderbufferParameteriv_reset_ptr() {
  *glGetNamedRenderbufferParameteriv_p.0.get() = None;
}
/// [glGetObjectLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectLabel.xhtml)
/// * `identifier` group: ObjectIdentifier
/// * `label` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetObjectLabel(identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetObjectLabel_p.0.get() } {
    Some(glGetObjectLabel_inner) => glGetObjectLabel_inner(identifier, name, bufSize, length, label),
    None => gl_not_loaded("glGetObjectLabel"),
  }
}
static glGetObjectLabel_p: GlFnCell<glGetObjectLabel_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetObjectLabel_is_loaded() -> bool {
  unsafe { *glGetObjectLabel_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetObjectLabel_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetObjectLabel_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetObjectLabel_t>>(gl_ptr_filter(f(b"glGetObjectLabel\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetObjectLabel_reset_ptr() {
  *glGetObjectLabel_p.0.get() = None;
}
/// [glGetObjectLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectLabelKHR.xhtml)
/// * `label` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetObjectLabelKHR(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetObjectLabelKHR_p.0.get() } {
    Some(glGetObjectLabelKHR_inner) => glGetObjectLabelKHR_inner(identifier, name, bufSize, length, label),
    None => gl_not_loaded("glGetObjectLabelKHR"),
  }
}
static glGetObjectLabelKHR_p: GlFnCell<glGetObjectLabelKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetObjectLabelKHR_is_loaded() -> bool {
  unsafe { *glGetObjectLabelKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetObjectLabelKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetObjectLabelKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetObjectLabelKHR_t>>(gl_ptr_filter(f(b"glGetObjectLabelKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetObjectLabelKHR_reset_ptr() {
  *glGetObjectLabelKHR_p.0.get() = None;
}
/// [glGetObjectPtrLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectPtrLabel.xhtml)
/// * `label` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetObjectPtrLabel(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetObjectPtrLabel_p.0.get() } {
    Some(glGetObjectPtrLabel_inner) => glGetObjectPtrLabel_inner(ptr, bufSize, length, label),
    None => gl_not_loaded("glGetObjectPtrLabel"),
  }
}
static glGetObjectPtrLabel_p: GlFnCell<glGetObjectPtrLabel_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetObjectPtrLabel_is_loaded() -> bool {
  unsafe { *glGetObjectPtrLabel_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetObjectPtrLabel_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetObjectPtrLabel_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetObjectPtrLabel_t>>(gl_ptr_filter(f(b"glGetObjectPtrLabel\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetObjectPtrLabel_reset_ptr() {
  *glGetObjectPtrLabel_p.0.get() = None;
}
/// [glGetObjectPtrLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectPtrLabelKHR.xhtml)
/// * `label` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetObjectPtrLabelKHR(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetObjectPtrLabelKHR_p.0.get() } {
    Some(glGetObjectPtrLabelKHR_inner) => glGetObjectPtrLabelKHR_inner(ptr, bufSize, length, label),
    None => gl_not_loaded("glGetObjectPtrLabelKHR"),
  }
}
static glGetObjectPtrLabelKHR_p: GlFnCell<glGetObjectPtrLabelKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetObjectPtrLabelKHR_is_loaded() -> bool {
  unsafe { *glGetObjectPtrLabelKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetObjectPtrLabelKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetObjectPtrLabelKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetObjectPtrLabelKHR_t>>(gl_ptr_filter(f(b"glGetObjectPtrLabelKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetObjectPtrLabelKHR_reset_ptr() {
  *glGetObjectPtrLabelKHR_p.0.get() = None;
}
/// [glGetPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointerv.xhtml)
/// * `pname` group: GetPointervPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetPointerv(pname: GetPointervPName, params: *mut *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetPointerv_p.0.get() } {
    Some(glGetPointerv_inner) => glGetPointerv_inner(pname, params),
    None => gl_not_loaded("glGetPointerv"),
  }
}
static glGetPointerv_p: GlFnCell<glGetPointerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetPointerv_is_loaded() -> bool {
  unsafe { *glGetPointerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetPointerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetPointerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetPointerv_t>>(gl_ptr_filter(f(b"glGetPointerv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetPointerv_reset_ptr() {
  *glGetPointerv_p.0.get() = None;
}
/// [glGetPointervKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointervKHR.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetPointervKHR(pname: GLenum, params: *mut *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetPointervKHR_p.0.get() } {
    Some(glGetPointervKHR_inner) => glGetPointervKHR_inner(pname, params),
    None => gl_not_loaded("glGetPointervKHR"),
  }
}
static glGetPointervKHR_p: GlFnCell<glGetPointervKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetPointervKHR_is_loaded() -> bool {
  unsafe { *glGetPointervKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetPointervKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetPointervKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetPointervKHR_t>>(gl_ptr_filter(f(b"glGetPointervKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetPointervKHR_reset_ptr() {
  *glGetPointervKHR_p.0.get() = None;
}
/// [glGetProgramBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramBinary.xhtml)
/// * `program` class: program
/// * `binary` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramBinary_p.0.get() } {
    Some(glGetProgramBinary_inner) => glGetProgramBinary_inner(program, bufSize, length, binaryFormat, binary),
    None => gl_not_loaded("glGetProgramBinary"),
  }
}
static glGetProgramBinary_p: GlFnCell<glGetProgramBinary_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramBinary_is_loaded() -> bool {
  unsafe { *glGetProgramBinary_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramBinary_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramBinary_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramBinary_t>>(gl_ptr_filter(f(b"glGetProgramBinary\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramBinary_reset_ptr() {
  *glGetProgramBinary_p.0.get() = None;
}
/// [glGetProgramInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInfoLog.xhtml)
/// * `program` class: program
/// * `infoLog` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramInfoLog_p.0.get() } {
    Some(glGetProgramInfoLog_inner) => glGetProgramInfoLog_inner(program, bufSize, length, infoLog),
    None => gl_not_loaded("glGetProgramInfoLog"),
  }
}
static glGetProgramInfoLog_p: GlFnCell<glGetProgramInfoLog_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramInfoLog_is_loaded() -> bool {
  unsafe { *glGetProgramInfoLog_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramInfoLog_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramInfoLog_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramInfoLog_t>>(gl_ptr_filter(f(b"glGetProgramInfoLog\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramInfoLog_reset_ptr() {
  *glGetProgramInfoLog_p.0.get() = None;
}
/// [glGetProgramInterfaceiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInterfaceiv.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `pname` group: ProgramInterfacePName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramInterfaceiv(program: GLuint, programInterface: ProgramInterface, pname: ProgramInterfacePName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramInterfaceiv_p.0.get() } {
    Some(glGetProgramInterfaceiv_inner) => glGetProgramInterfaceiv_inner(program, programInterface, pname, params),
    None => gl_not_loaded("glGetProgramInterfaceiv"),
  }
}
static glGetProgramInterfaceiv_p: GlFnCell<glGetProgramInterfaceiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramInterfaceiv_is_loaded() -> bool {
  unsafe { *glGetProgramInterfaceiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramInterfaceiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramInterfaceiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramInterfaceiv_t>>(gl_ptr_filter(f(b"glGetProgramInterfaceiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramInterfaceiv_reset_ptr() {
  *glGetProgramInterfaceiv_p.0.get() = None;
}
/// [glGetProgramPipelineInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramPipelineInfoLog.xhtml)
/// * `pipeline` class: program pipeline
/// * `infoLog` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramPipelineInfoLog_p.0.get() } {
    Some(glGetProgramPipelineInfoLog_inner) => glGetProgramPipelineInfoLog_inner(pipeline, bufSize, length, infoLog),
    None => gl_not_loaded("glGetProgramPipelineInfoLog"),
  }
}
static glGetProgramPipelineInfoLog_p: GlFnCell<glGetProgramPipelineInfoLog_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramPipelineInfoLog_is_loaded() -> bool {
  unsafe { *glGetProgramPipelineInfoLog_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramPipelineInfoLog_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramPipelineInfoLog_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramPipelineInfoLog_t>>(gl_ptr_filter(f(b"glGetProgramPipelineInfoLog\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramPipelineInfoLog_reset_ptr() {
  *glGetProgramPipelineInfoLog_p.0.get() = None;
}
/// [glGetProgramPipelineiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramPipelineiv.xhtml)
/// * `pipeline` class: program pipeline
/// * `pname` group: PipelineParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramPipelineiv(pipeline: GLuint, pname: PipelineParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramPipelineiv_p.0.get() } {
    Some(glGetProgramPipelineiv_inner) => glGetProgramPipelineiv_inner(pipeline, pname, params),
    None => gl_not_loaded("glGetProgramPipelineiv"),
  }
}
static glGetProgramPipelineiv_p: GlFnCell<glGetProgramPipelineiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramPipelineiv_is_loaded() -> bool {
  unsafe { *glGetProgramPipelineiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramPipelineiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramPipelineiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramPipelineiv_t>>(gl_ptr_filter(f(b"glGetProgramPipelineiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramPipelineiv_reset_ptr() {
  *glGetProgramPipelineiv_p.0.get() = None;
}
/// [glGetProgramResourceIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceIndex.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramResourceIndex(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramResourceIndex_p.0.get() } {
    Some(glGetProgramResourceIndex_inner) => glGetProgramResourceIndex_inner(program, programInterface, name),
    None => gl_not_loaded("glGetProgramResourceIndex"),
  }
}
static glGetProgramResourceIndex_p: GlFnCell<glGetProgramResourceIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramResourceIndex_is_loaded() -> bool {
  unsafe { *glGetProgramResourceIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramResourceIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramResourceIndex_t>>(gl_ptr_filter(f(b"glGetProgramResourceIndex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceIndex_reset_ptr() {
  *glGetProgramResourceIndex_p.0.get() = None;
}
/// [glGetProgramResourceLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceLocation.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramResourceLocation(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramResourceLocation_p.0.get() } {
    Some(glGetProgramResourceLocation_inner) => glGetProgramResourceLocation_inner(program, programInterface, name),
    None => gl_not_loaded("glGetProgramResourceLocation"),
  }
}
static glGetProgramResourceLocation_p: GlFnCell<glGetProgramResourceLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramResourceLocation_is_loaded() -> bool {
  unsafe { *glGetProgramResourceLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramResourceLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramResourceLocation_t>>(gl_ptr_filter(f(b"glGetProgramResourceLocation\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceLocation_reset_ptr() {
  *glGetProgramResourceLocation_p.0.get() = None;
}
/// [glGetProgramResourceLocationIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceLocationIndex.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: COMPSIZE(name)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramResourceLocationIndex(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramResourceLocationIndex_p.0.get() } {
    Some(glGetProgramResourceLocationIndex_inner) => glGetProgramResourceLocationIndex_inner(program, programInterface, name),
    None => gl_not_loaded("glGetProgramResourceLocationIndex"),
  }
}
static glGetProgramResourceLocationIndex_p: GlFnCell<glGetProgramResourceLocationIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramResourceLocationIndex_is_loaded() -> bool {
  unsafe { *glGetProgramResourceLocationIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceLocationIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramResourceLocationIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramResourceLocationIndex_t>>(gl_ptr_filter(f(b"glGetProgramResourceLocationIndex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceLocationIndex_reset_ptr() {
  *glGetProgramResourceLocationIndex_p.0.get() = None;
}
/// [glGetProgramResourceName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceName.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramResourceName(program: GLuint, programInterface: ProgramInterface, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramResourceName_p.0.get() } {
    Some(glGetProgramResourceName_inner) => glGetProgramResourceName_inner(program, programInterface, index, bufSize, length, name),
    None => gl_not_loaded("glGetProgramResourceName"),
  }
}
static glGetProgramResourceName_p: GlFnCell<glGetProgramResourceName_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramResourceName_is_loaded() -> bool {
  unsafe { *glGetProgramResourceName_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceName_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramResourceName_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramResourceName_t>>(gl_ptr_filter(f(b"glGetProgramResourceName\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceName_reset_ptr() {
  *glGetProgramResourceName_p.0.get() = None;
}
/// [glGetProgramResourceiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceiv.xhtml)
/// * `program` class: program
/// * `programInterface` group: ProgramInterface
/// * `props` group: ProgramResourceProperty
/// * `props` len: propCount
/// * `params` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramResourceiv(program: GLuint, programInterface: ProgramInterface, index: GLuint, propCount: GLsizei, props: *const ProgramResourceProperty, count: GLsizei, length: *mut GLsizei, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramResourceiv_p.0.get() } {
    Some(glGetProgramResourceiv_inner) => glGetProgramResourceiv_inner(program, programInterface, index, propCount, props, count, length, params),
    None => gl_not_loaded("glGetProgramResourceiv"),
  }
}
static glGetProgramResourceiv_p: GlFnCell<glGetProgramResourceiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramResourceiv_is_loaded() -> bool {
  unsafe { *glGetProgramResourceiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramResourceiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramResourceiv_t>>(gl_ptr_filter(f(b"glGetProgramResourceiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramResourceiv_reset_ptr() {
  *glGetProgramResourceiv_p.0.get() = None;
}
/// [glGetProgramStageiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramStageiv.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
/// * `pname` group: ProgramStagePName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramStageiv(program: GLuint, shadertype: ShaderType, pname: ProgramStagePName, values: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramStageiv_p.0.get() } {
    Some(glGetProgramStageiv_inner) => glGetProgramStageiv_inner(program, shadertype, pname, values),
    None => gl_not_loaded("glGetProgramStageiv"),
  }
}
static glGetProgramStageiv_p: GlFnCell<glGetProgramStageiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramStageiv_is_loaded() -> bool {
  unsafe { *glGetProgramStageiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramStageiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramStageiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramStageiv_t>>(gl_ptr_filter(f(b"glGetProgramStageiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramStageiv_reset_ptr() {
  *glGetProgramStageiv_p.0.get() = None;
}
/// [glGetProgramiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramiv.xhtml)
/// * `program` class: program
/// * `pname` group: ProgramPropertyARB
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramiv(program: GLuint, pname: ProgramPropertyARB, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramiv_p.0.get() } {
    Some(glGetProgramiv_inner) => glGetProgramiv_inner(program, pname, params),
    None => gl_not_loaded("glGetProgramiv"),
  }
}
static glGetProgramiv_p: GlFnCell<glGetProgramiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramiv_is_loaded() -> bool {
  unsafe { *glGetProgramiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramiv_t>>(gl_ptr_filter(f(b"glGetProgramiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetProgramiv_reset_ptr() {
  *glGetProgramiv_p.0.get() = None;
}
/// [glGetQueryBufferObjecti64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjecti64v.xhtml)
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryBufferObjecti64v(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryBufferObjecti64v_p.0.get() } {
    Some(glGetQueryBufferObjecti64v_inner) => glGetQueryBufferObjecti64v_inner(id, buffer, pname, offset),
    None => gl_not_loaded("glGetQueryBufferObjecti64v"),
  }
}
static glGetQueryBufferObjecti64v_p: GlFnCell<glGetQueryBufferObjecti64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryBufferObjecti64v_is_loaded() -> bool {
  unsafe { *glGetQueryBufferObjecti64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryBufferObjecti64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryBufferObjecti64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryBufferObjecti64v_t>>(gl_ptr_filter(f(b"glGetQueryBufferObjecti64v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryBufferObjecti64v_reset_ptr() {
  *glGetQueryBufferObjecti64v_p.0.get() = None;
}
/// [glGetQueryBufferObjectiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectiv.xhtml)
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryBufferObjectiv(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryBufferObjectiv_p.0.get() } {
    Some(glGetQueryBufferObjectiv_inner) => glGetQueryBufferObjectiv_inner(id, buffer, pname, offset),
    None => gl_not_loaded("glGetQueryBufferObjectiv"),
  }
}
static glGetQueryBufferObjectiv_p: GlFnCell<glGetQueryBufferObjectiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryBufferObjectiv_is_loaded() -> bool {
  unsafe { *glGetQueryBufferObjectiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryBufferObjectiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryBufferObjectiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryBufferObjectiv_t>>(gl_ptr_filter(f(b"glGetQueryBufferObjectiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryBufferObjectiv_reset_ptr() {
  *glGetQueryBufferObjectiv_p.0.get() = None;
}
/// [glGetQueryBufferObjectui64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectui64v.xhtml)
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryBufferObjectui64v(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryBufferObjectui64v_p.0.get() } {
    Some(glGetQueryBufferObjectui64v_inner) => glGetQueryBufferObjectui64v_inner(id, buffer, pname, offset),
    None => gl_not_loaded("glGetQueryBufferObjectui64v"),
  }
}
static glGetQueryBufferObjectui64v_p: GlFnCell<glGetQueryBufferObjectui64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryBufferObjectui64v_is_loaded() -> bool {
  unsafe { *glGetQueryBufferObjectui64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryBufferObjectui64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryBufferObjectui64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryBufferObjectui64v_t>>(gl_ptr_filter(f(b"glGetQueryBufferObjectui64v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryBufferObjectui64v_reset_ptr() {
  *glGetQueryBufferObjectui64v_p.0.get() = None;
}
/// [glGetQueryBufferObjectuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectuiv.xhtml)
/// * `id` class: query
/// * `buffer` class: buffer
/// * `pname` group: QueryObjectParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryBufferObjectuiv(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryBufferObjectuiv_p.0.get() } {
    Some(glGetQueryBufferObjectuiv_inner) => glGetQueryBufferObjectuiv_inner(id, buffer, pname, offset),
    None => gl_not_loaded("glGetQueryBufferObjectuiv"),
  }
}
static glGetQueryBufferObjectuiv_p: GlFnCell<glGetQueryBufferObjectuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryBufferObjectuiv_is_loaded() -> bool {
  unsafe { *glGetQueryBufferObjectuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryBufferObjectuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryBufferObjectuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryBufferObjectuiv_t>>(gl_ptr_filter(f(b"glGetQueryBufferObjectuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryBufferObjectuiv_reset_ptr() {
  *glGetQueryBufferObjectuiv_p.0.get() = None;
}
/// [glGetQueryIndexediv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryIndexediv.xhtml)
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryIndexediv(target: QueryTarget, index: GLuint, pname: QueryParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryIndexediv_p.0.get() } {
    Some(glGetQueryIndexediv_inner) => glGetQueryIndexediv_inner(target, index, pname, params),
    None => gl_not_loaded("glGetQueryIndexediv"),
  }
}
static glGetQueryIndexediv_p: GlFnCell<glGetQueryIndexediv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryIndexediv_is_loaded() -> bool {
  unsafe { *glGetQueryIndexediv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryIndexediv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryIndexediv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryIndexediv_t>>(gl_ptr_filter(f(b"glGetQueryIndexediv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryIndexediv_reset_ptr() {
  *glGetQueryIndexediv_p.0.get() = None;
}
/// [glGetQueryObjecti64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjecti64v.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryObjecti64v(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryObjecti64v_p.0.get() } {
    Some(glGetQueryObjecti64v_inner) => glGetQueryObjecti64v_inner(id, pname, params),
    None => gl_not_loaded("glGetQueryObjecti64v"),
  }
}
static glGetQueryObjecti64v_p: GlFnCell<glGetQueryObjecti64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryObjecti64v_is_loaded() -> bool {
  unsafe { *glGetQueryObjecti64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjecti64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryObjecti64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryObjecti64v_t>>(gl_ptr_filter(f(b"glGetQueryObjecti64v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjecti64v_reset_ptr() {
  *glGetQueryObjecti64v_p.0.get() = None;
}
/// [glGetQueryObjectiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectiv.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryObjectiv(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryObjectiv_p.0.get() } {
    Some(glGetQueryObjectiv_inner) => glGetQueryObjectiv_inner(id, pname, params),
    None => gl_not_loaded("glGetQueryObjectiv"),
  }
}
static glGetQueryObjectiv_p: GlFnCell<glGetQueryObjectiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryObjectiv_is_loaded() -> bool {
  unsafe { *glGetQueryObjectiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryObjectiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryObjectiv_t>>(gl_ptr_filter(f(b"glGetQueryObjectiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectiv_reset_ptr() {
  *glGetQueryObjectiv_p.0.get() = None;
}
/// [glGetQueryObjectui64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectui64v.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryObjectui64v(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryObjectui64v_p.0.get() } {
    Some(glGetQueryObjectui64v_inner) => glGetQueryObjectui64v_inner(id, pname, params),
    None => gl_not_loaded("glGetQueryObjectui64v"),
  }
}
static glGetQueryObjectui64v_p: GlFnCell<glGetQueryObjectui64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryObjectui64v_is_loaded() -> bool {
  unsafe { *glGetQueryObjectui64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectui64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryObjectui64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryObjectui64v_t>>(gl_ptr_filter(f(b"glGetQueryObjectui64v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectui64v_reset_ptr() {
  *glGetQueryObjectui64v_p.0.get() = None;
}
/// [glGetQueryObjectuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectuiv.xhtml)
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryObjectuiv(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryObjectuiv_p.0.get() } {
    Some(glGetQueryObjectuiv_inner) => glGetQueryObjectuiv_inner(id, pname, params),
    None => gl_not_loaded("glGetQueryObjectuiv"),
  }
}
static glGetQueryObjectuiv_p: GlFnCell<glGetQueryObjectuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryObjectuiv_is_loaded() -> bool {
  unsafe { *glGetQueryObjectuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryObjectuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryObjectuiv_t>>(gl_ptr_filter(f(b"glGetQueryObjectuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectuiv_reset_ptr() {
  *glGetQueryObjectuiv_p.0.get() = None;
}
/// [glGetQueryiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryiv.xhtml)
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryiv(target: QueryTarget, pname: QueryParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryiv_p.0.get() } {
    Some(glGetQueryiv_inner) => glGetQueryiv_inner(target, pname, params),
    None => gl_not_loaded("glGetQueryiv"),
  }
}
static glGetQueryiv_p: GlFnCell<glGetQueryiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryiv_is_loaded() -> bool {
  unsafe { *glGetQueryiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryiv_t>>(gl_ptr_filter(f(b"glGetQueryiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetQueryiv_reset_ptr() {
  *glGetQueryiv_p.0.get() = None;
}
/// [glGetRenderbufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetRenderbufferParameteriv.xhtml)
/// * `target` group: RenderbufferTarget
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetRenderbufferParameteriv(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetRenderbufferParameteriv_p.0.get() } {
    Some(glGetRenderbufferParameteriv_inner) => glGetRenderbufferParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glGetRenderbufferParameteriv"),
  }
}
static glGetRenderbufferParameteriv_p: GlFnCell<glGetRenderbufferParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetRenderbufferParameteriv_is_loaded() -> bool {
  unsafe { *glGetRenderbufferParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetRenderbufferParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetRenderbufferParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetRenderbufferParameteriv_t>>(gl_ptr_filter(f(b"glGetRenderbufferParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetRenderbufferParameteriv_reset_ptr() {
  *glGetRenderbufferParameteriv_p.0.get() = None;
}
/// [glGetSamplerParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIiv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSamplerParameterIiv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSamplerParameterIiv_p.0.get() } {
    Some(glGetSamplerParameterIiv_inner) => glGetSamplerParameterIiv_inner(sampler, pname, params),
    None => gl_not_loaded("glGetSamplerParameterIiv"),
  }
}
static glGetSamplerParameterIiv_p: GlFnCell<glGetSamplerParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSamplerParameterIiv_is_loaded() -> bool {
  unsafe { *glGetSamplerParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSamplerParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSamplerParameterIiv_t>>(gl_ptr_filter(f(b"glGetSamplerParameterIiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterIiv_reset_ptr() {
  *glGetSamplerParameterIiv_p.0.get() = None;
}
/// [glGetSamplerParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIuiv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSamplerParameterIuiv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSamplerParameterIuiv_p.0.get() } {
    Some(glGetSamplerParameterIuiv_inner) => glGetSamplerParameterIuiv_inner(sampler, pname, params),
    None => gl_not_loaded("glGetSamplerParameterIuiv"),
  }
}
static glGetSamplerParameterIuiv_p: GlFnCell<glGetSamplerParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSamplerParameterIuiv_is_loaded() -> bool {
  unsafe { *glGetSamplerParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSamplerParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSamplerParameterIuiv_t>>(gl_ptr_filter(f(b"glGetSamplerParameterIuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterIuiv_reset_ptr() {
  *glGetSamplerParameterIuiv_p.0.get() = None;
}
/// [glGetSamplerParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterfv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSamplerParameterfv(sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSamplerParameterfv_p.0.get() } {
    Some(glGetSamplerParameterfv_inner) => glGetSamplerParameterfv_inner(sampler, pname, params),
    None => gl_not_loaded("glGetSamplerParameterfv"),
  }
}
static glGetSamplerParameterfv_p: GlFnCell<glGetSamplerParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSamplerParameterfv_is_loaded() -> bool {
  unsafe { *glGetSamplerParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSamplerParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSamplerParameterfv_t>>(gl_ptr_filter(f(b"glGetSamplerParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterfv_reset_ptr() {
  *glGetSamplerParameterfv_p.0.get() = None;
}
/// [glGetSamplerParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameteriv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSamplerParameteriv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSamplerParameteriv_p.0.get() } {
    Some(glGetSamplerParameteriv_inner) => glGetSamplerParameteriv_inner(sampler, pname, params),
    None => gl_not_loaded("glGetSamplerParameteriv"),
  }
}
static glGetSamplerParameteriv_p: GlFnCell<glGetSamplerParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSamplerParameteriv_is_loaded() -> bool {
  unsafe { *glGetSamplerParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSamplerParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSamplerParameteriv_t>>(gl_ptr_filter(f(b"glGetSamplerParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameteriv_reset_ptr() {
  *glGetSamplerParameteriv_p.0.get() = None;
}
/// [glGetShaderInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderInfoLog.xhtml)
/// * `shader` class: shader
/// * `infoLog` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetShaderInfoLog_p.0.get() } {
    Some(glGetShaderInfoLog_inner) => glGetShaderInfoLog_inner(shader, bufSize, length, infoLog),
    None => gl_not_loaded("glGetShaderInfoLog"),
  }
}
static glGetShaderInfoLog_p: GlFnCell<glGetShaderInfoLog_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetShaderInfoLog_is_loaded() -> bool {
  unsafe { *glGetShaderInfoLog_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetShaderInfoLog_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetShaderInfoLog_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetShaderInfoLog_t>>(gl_ptr_filter(f(b"glGetShaderInfoLog\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetShaderInfoLog_reset_ptr() {
  *glGetShaderInfoLog_p.0.get() = None;
}
/// [glGetShaderPrecisionFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderPrecisionFormat.xhtml)
/// * `shadertype` group: ShaderType
/// * `precisiontype` group: PrecisionType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetShaderPrecisionFormat(shadertype: ShaderType, precisiontype: PrecisionType, range: *mut [GLint; 2], precision: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetShaderPrecisionFormat_p.0.get() } {
    Some(glGetShaderPrecisionFormat_inner) => glGetShaderPrecisionFormat_inner(shadertype, precisiontype, range, precision),
    None => gl_not_loaded("glGetShaderPrecisionFormat"),
  }
}
static glGetShaderPrecisionFormat_p: GlFnCell<glGetShaderPrecisionFormat_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetShaderPrecisionFormat_is_loaded() -> bool {
  unsafe { *glGetShaderPrecisionFormat_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetShaderPrecisionFormat_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetShaderPrecisionFormat_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetShaderPrecisionFormat_t>>(gl_ptr_filter(f(b"glGetShaderPrecisionFormat\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetShaderPrecisionFormat_reset_ptr() {
  *glGetShaderPrecisionFormat_p.0.get() = None;
}
/// [glGetShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderSource.xhtml)
/// * `shader` class: shader
/// * `source` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetShaderSource_p.0.get() } {
    Some(glGetShaderSource_inner) => glGetShaderSource_inner(shader, bufSize, length, source),
    None => gl_not_loaded("glGetShaderSource"),
  }
}
static glGetShaderSource_p: GlFnCell<glGetShaderSource_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetShaderSource_is_loaded() -> bool {
  unsafe { *glGetShaderSource_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetShaderSource_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetShaderSource_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetShaderSource_t>>(gl_ptr_filter(f(b"glGetShaderSource\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetShaderSource_reset_ptr() {
  *glGetShaderSource_p.0.get() = None;
}
/// [glGetShaderiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderiv.xhtml)
/// * `shader` class: shader
/// * `pname` group: ShaderParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetShaderiv(shader: GLuint, pname: ShaderParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetShaderiv_p.0.get() } {
    Some(glGetShaderiv_inner) => glGetShaderiv_inner(shader, pname, params),
    None => gl_not_loaded("glGetShaderiv"),
  }
}
static glGetShaderiv_p: GlFnCell<glGetShaderiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetShaderiv_is_loaded() -> bool {
  unsafe { *glGetShaderiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetShaderiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetShaderiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetShaderiv_t>>(gl_ptr_filter(f(b"glGetShaderiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetShaderiv_reset_ptr() {
  *glGetShaderiv_p.0.get() = None;
}
/// [glGetString](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetString.xhtml)
/// * `name` group: StringName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetString(name: StringName) -> GLubyte {
  #[allow(unused_unsafe)]
  match unsafe { *glGetString_p.0.get() } {
    Some(glGetString_inner) => glGetString_inner(name),
    None => gl_not_loaded("glGetString"),
  }
}
static glGetString_p: GlFnCell<glGetString_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetString_is_loaded() -> bool {
  unsafe { *glGetString_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetString_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetString_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetString_t>>(gl_ptr_filter(f(b"glGetString\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetString_reset_ptr() {
  *glGetString_p.0.get() = None;
}
/// [glGetStringi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetStringi.xhtml)
/// * `name` group: StringName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetStringi(name: StringName, index: GLuint) -> GLubyte {
  #[allow(unused_unsafe)]
  match unsafe { *glGetStringi_p.0.get() } {
    Some(glGetStringi_inner) => glGetStringi_inner(name, index),
    None => gl_not_loaded("glGetStringi"),
  }
}
static glGetStringi_p: GlFnCell<glGetStringi_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetStringi_is_loaded() -> bool {
  unsafe { *glGetStringi_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetStringi_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetStringi_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetStringi_t>>(gl_ptr_filter(f(b"glGetStringi\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetStringi_reset_ptr() {
  *glGetStringi_p.0.get() = None;
}
/// [glGetSubroutineIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSubroutineIndex.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSubroutineIndex(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSubroutineIndex_p.0.get() } {
    Some(glGetSubroutineIndex_inner) => glGetSubroutineIndex_inner(program, shadertype, name),
    None => gl_not_loaded("glGetSubroutineIndex"),
  }
}
static glGetSubroutineIndex_p: GlFnCell<glGetSubroutineIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSubroutineIndex_is_loaded() -> bool {
  unsafe { *glGetSubroutineIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSubroutineIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSubroutineIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSubroutineIndex_t>>(gl_ptr_filter(f(b"glGetSubroutineIndex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetSubroutineIndex_reset_ptr() {
  *glGetSubroutineIndex_p.0.get() = None;
}
/// [glGetSubroutineUniformLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSubroutineUniformLocation.xhtml)
/// * `program` class: program
/// * `shadertype` group: ShaderType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSubroutineUniformLocation(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSubroutineUniformLocation_p.0.get() } {
    Some(glGetSubroutineUniformLocation_inner) => glGetSubroutineUniformLocation_inner(program, shadertype, name),
    None => gl_not_loaded("glGetSubroutineUniformLocation"),
  }
}
static glGetSubroutineUniformLocation_p: GlFnCell<glGetSubroutineUniformLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSubroutineUniformLocation_is_loaded() -> bool {
  unsafe { *glGetSubroutineUniformLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSubroutineUniformLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSubroutineUniformLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSubroutineUniformLocation_t>>(gl_ptr_filter(f(b"glGetSubroutineUniformLocation\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetSubroutineUniformLocation_reset_ptr() {
  *glGetSubroutineUniformLocation_p.0.get() = None;
}
/// [glGetSynciv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSynciv.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
/// * `pname` group: SyncParameterName
/// * `values` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSynciv(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSynciv_p.0.get() } {
    Some(glGetSynciv_inner) => glGetSynciv_inner(sync, pname, count, length, values),
    None => gl_not_loaded("glGetSynciv"),
  }
}
static glGetSynciv_p: GlFnCell<glGetSynciv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSynciv_is_loaded() -> bool {
  unsafe { *glGetSynciv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSynciv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSynciv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSynciv_t>>(gl_ptr_filter(f(b"glGetSynciv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetSynciv_reset_ptr() {
  *glGetSynciv_p.0.get() = None;
}
/// [glGetTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexImage.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(target,level,format,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexImage(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexImage_p.0.get() } {
    Some(glGetTexImage_inner) => glGetTexImage_inner(target, level, format, type_, pixels),
    None => gl_not_loaded("glGetTexImage"),
  }
}
static glGetTexImage_p: GlFnCell<glGetTexImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexImage_is_loaded() -> bool {
  unsafe { *glGetTexImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexImage_t>>(gl_ptr_filter(f(b"glGetTexImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTexImage_reset_ptr() {
  *glGetTexImage_p.0.get() = None;
}
/// [glGetTexLevelParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexLevelParameterfv.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexLevelParameterfv(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexLevelParameterfv_p.0.get() } {
    Some(glGetTexLevelParameterfv_inner) => glGetTexLevelParameterfv_inner(target, level, pname, params),
    None => gl_not_loaded("glGetTexLevelParameterfv"),
  }
}
static glGetTexLevelParameterfv_p: GlFnCell<glGetTexLevelParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexLevelParameterfv_is_loaded() -> bool {
  unsafe { *glGetTexLevelParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexLevelParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexLevelParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexLevelParameterfv_t>>(gl_ptr_filter(f(b"glGetTexLevelParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTexLevelParameterfv_reset_ptr() {
  *glGetTexLevelParameterfv_p.0.get() = None;
}
/// [glGetTexLevelParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexLevelParameteriv.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexLevelParameteriv(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexLevelParameteriv_p.0.get() } {
    Some(glGetTexLevelParameteriv_inner) => glGetTexLevelParameteriv_inner(target, level, pname, params),
    None => gl_not_loaded("glGetTexLevelParameteriv"),
  }
}
static glGetTexLevelParameteriv_p: GlFnCell<glGetTexLevelParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexLevelParameteriv_is_loaded() -> bool {
  unsafe { *glGetTexLevelParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexLevelParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexLevelParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexLevelParameteriv_t>>(gl_ptr_filter(f(b"glGetTexLevelParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTexLevelParameteriv_reset_ptr() {
  *glGetTexLevelParameteriv_p.0.get() = None;
}
/// [glGetTexParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIiv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexParameterIiv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexParameterIiv_p.0.get() } {
    Some(glGetTexParameterIiv_inner) => glGetTexParameterIiv_inner(target, pname, params),
    None => gl_not_loaded("glGetTexParameterIiv"),
  }
}
static glGetTexParameterIiv_p: GlFnCell<glGetTexParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexParameterIiv_is_loaded() -> bool {
  unsafe { *glGetTexParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexParameterIiv_t>>(gl_ptr_filter(f(b"glGetTexParameterIiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterIiv_reset_ptr() {
  *glGetTexParameterIiv_p.0.get() = None;
}
/// [glGetTexParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIuiv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexParameterIuiv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexParameterIuiv_p.0.get() } {
    Some(glGetTexParameterIuiv_inner) => glGetTexParameterIuiv_inner(target, pname, params),
    None => gl_not_loaded("glGetTexParameterIuiv"),
  }
}
static glGetTexParameterIuiv_p: GlFnCell<glGetTexParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexParameterIuiv_is_loaded() -> bool {
  unsafe { *glGetTexParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexParameterIuiv_t>>(gl_ptr_filter(f(b"glGetTexParameterIuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterIuiv_reset_ptr() {
  *glGetTexParameterIuiv_p.0.get() = None;
}
/// [glGetTexParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterfv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexParameterfv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexParameterfv_p.0.get() } {
    Some(glGetTexParameterfv_inner) => glGetTexParameterfv_inner(target, pname, params),
    None => gl_not_loaded("glGetTexParameterfv"),
  }
}
static glGetTexParameterfv_p: GlFnCell<glGetTexParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexParameterfv_is_loaded() -> bool {
  unsafe { *glGetTexParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexParameterfv_t>>(gl_ptr_filter(f(b"glGetTexParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterfv_reset_ptr() {
  *glGetTexParameterfv_p.0.get() = None;
}
/// [glGetTexParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameteriv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexParameteriv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexParameteriv_p.0.get() } {
    Some(glGetTexParameteriv_inner) => glGetTexParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glGetTexParameteriv"),
  }
}
static glGetTexParameteriv_p: GlFnCell<glGetTexParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexParameteriv_is_loaded() -> bool {
  unsafe { *glGetTexParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexParameteriv_t>>(gl_ptr_filter(f(b"glGetTexParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTexParameteriv_reset_ptr() {
  *glGetTexParameteriv_p.0.get() = None;
}
/// [glGetTextureImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureImage.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTextureImage(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTextureImage_p.0.get() } {
    Some(glGetTextureImage_inner) => glGetTextureImage_inner(texture, level, format, type_, bufSize, pixels),
    None => gl_not_loaded("glGetTextureImage"),
  }
}
static glGetTextureImage_p: GlFnCell<glGetTextureImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTextureImage_is_loaded() -> bool {
  unsafe { *glGetTextureImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTextureImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTextureImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTextureImage_t>>(gl_ptr_filter(f(b"glGetTextureImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTextureImage_reset_ptr() {
  *glGetTextureImage_p.0.get() = None;
}
/// [glGetTextureLevelParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureLevelParameterfv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTextureLevelParameterfv(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTextureLevelParameterfv_p.0.get() } {
    Some(glGetTextureLevelParameterfv_inner) => glGetTextureLevelParameterfv_inner(texture, level, pname, params),
    None => gl_not_loaded("glGetTextureLevelParameterfv"),
  }
}
static glGetTextureLevelParameterfv_p: GlFnCell<glGetTextureLevelParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTextureLevelParameterfv_is_loaded() -> bool {
  unsafe { *glGetTextureLevelParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTextureLevelParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTextureLevelParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTextureLevelParameterfv_t>>(gl_ptr_filter(f(b"glGetTextureLevelParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTextureLevelParameterfv_reset_ptr() {
  *glGetTextureLevelParameterfv_p.0.get() = None;
}
/// [glGetTextureLevelParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureLevelParameteriv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTextureLevelParameteriv(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTextureLevelParameteriv_p.0.get() } {
    Some(glGetTextureLevelParameteriv_inner) => glGetTextureLevelParameteriv_inner(texture, level, pname, params),
    None => gl_not_loaded("glGetTextureLevelParameteriv"),
  }
}
static glGetTextureLevelParameteriv_p: GlFnCell<glGetTextureLevelParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTextureLevelParameteriv_is_loaded() -> bool {
  unsafe { *glGetTextureLevelParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTextureLevelParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTextureLevelParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTextureLevelParameteriv_t>>(gl_ptr_filter(f(b"glGetTextureLevelParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTextureLevelParameteriv_reset_ptr() {
  *glGetTextureLevelParameteriv_p.0.get() = None;
}
/// [glGetTextureParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterIiv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTextureParameterIiv(texture: GLuint, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTextureParameterIiv_p.0.get() } {
    Some(glGetTextureParameterIiv_inner) => glGetTextureParameterIiv_inner(texture, pname, params),
    None => gl_not_loaded("glGetTextureParameterIiv"),
  }
}
static glGetTextureParameterIiv_p: GlFnCell<glGetTextureParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTextureParameterIiv_is_loaded() -> bool {
  unsafe { *glGetTextureParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTextureParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTextureParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTextureParameterIiv_t>>(gl_ptr_filter(f(b"glGetTextureParameterIiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTextureParameterIiv_reset_ptr() {
  *glGetTextureParameterIiv_p.0.get() = None;
}
/// [glGetTextureParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterIuiv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTextureParameterIuiv(texture: GLuint, pname: GetTextureParameter, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTextureParameterIuiv_p.0.get() } {
    Some(glGetTextureParameterIuiv_inner) => glGetTextureParameterIuiv_inner(texture, pname, params),
    None => gl_not_loaded("glGetTextureParameterIuiv"),
  }
}
static glGetTextureParameterIuiv_p: GlFnCell<glGetTextureParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTextureParameterIuiv_is_loaded() -> bool {
  unsafe { *glGetTextureParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTextureParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTextureParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTextureParameterIuiv_t>>(gl_ptr_filter(f(b"glGetTextureParameterIuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTextureParameterIuiv_reset_ptr() {
  *glGetTextureParameterIuiv_p.0.get() = None;
}
/// [glGetTextureParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterfv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTextureParameterfv(texture: GLuint, pname: GetTextureParameter, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTextureParameterfv_p.0.get() } {
    Some(glGetTextureParameterfv_inner) => glGetTextureParameterfv_inner(texture, pname, params),
    None => gl_not_loaded("glGetTextureParameterfv"),
  }
}
static glGetTextureParameterfv_p: GlFnCell<glGetTextureParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTextureParameterfv_is_loaded() -> bool {
  unsafe { *glGetTextureParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTextureParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTextureParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTextureParameterfv_t>>(gl_ptr_filter(f(b"glGetTextureParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTextureParameterfv_reset_ptr() {
  *glGetTextureParameterfv_p.0.get() = None;
}
/// [glGetTextureParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameteriv.xhtml)
/// * `texture` class: texture
/// * `pname` group: GetTextureParameter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTextureParameteriv(texture: GLuint, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTextureParameteriv_p.0.get() } {
    Some(glGetTextureParameteriv_inner) => glGetTextureParameteriv_inner(texture, pname, params),
    None => gl_not_loaded("glGetTextureParameteriv"),
  }
}
static glGetTextureParameteriv_p: GlFnCell<glGetTextureParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTextureParameteriv_is_loaded() -> bool {
  unsafe { *glGetTextureParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTextureParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTextureParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTextureParameteriv_t>>(gl_ptr_filter(f(b"glGetTextureParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTextureParameteriv_reset_ptr() {
  *glGetTextureParameteriv_p.0.get() = None;
}
/// [glGetTextureSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureSubImage.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTextureSubImage_p.0.get() } {
    Some(glGetTextureSubImage_inner) => glGetTextureSubImage_inner(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels),
    None => gl_not_loaded("glGetTextureSubImage"),
  }
}
static glGetTextureSubImage_p: GlFnCell<glGetTextureSubImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTextureSubImage_is_loaded() -> bool {
  unsafe { *glGetTextureSubImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTextureSubImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTextureSubImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTextureSubImage_t>>(gl_ptr_filter(f(b"glGetTextureSubImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTextureSubImage_reset_ptr() {
  *glGetTextureSubImage_p.0.get() = None;
}
/// [glGetTransformFeedbackVarying](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbackVarying.xhtml)
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTransformFeedbackVarying_p.0.get() } {
    Some(glGetTransformFeedbackVarying_inner) => glGetTransformFeedbackVarying_inner(program, index, bufSize, length, size, type_, name),
    None => gl_not_loaded("glGetTransformFeedbackVarying"),
  }
}
static glGetTransformFeedbackVarying_p: GlFnCell<glGetTransformFeedbackVarying_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTransformFeedbackVarying_is_loaded() -> bool {
  unsafe { *glGetTransformFeedbackVarying_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbackVarying_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTransformFeedbackVarying_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTransformFeedbackVarying_t>>(gl_ptr_filter(f(b"glGetTransformFeedbackVarying\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbackVarying_reset_ptr() {
  *glGetTransformFeedbackVarying_p.0.get() = None;
}
/// [glGetTransformFeedbacki64_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbacki64_v.xhtml)
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTransformFeedbacki64_v(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTransformFeedbacki64_v_p.0.get() } {
    Some(glGetTransformFeedbacki64_v_inner) => glGetTransformFeedbacki64_v_inner(xfb, pname, index, param),
    None => gl_not_loaded("glGetTransformFeedbacki64_v"),
  }
}
static glGetTransformFeedbacki64_v_p: GlFnCell<glGetTransformFeedbacki64_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTransformFeedbacki64_v_is_loaded() -> bool {
  unsafe { *glGetTransformFeedbacki64_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbacki64_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTransformFeedbacki64_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTransformFeedbacki64_v_t>>(gl_ptr_filter(f(b"glGetTransformFeedbacki64_v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbacki64_v_reset_ptr() {
  *glGetTransformFeedbacki64_v_p.0.get() = None;
}
/// [glGetTransformFeedbacki_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbacki_v.xhtml)
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTransformFeedbacki_v(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTransformFeedbacki_v_p.0.get() } {
    Some(glGetTransformFeedbacki_v_inner) => glGetTransformFeedbacki_v_inner(xfb, pname, index, param),
    None => gl_not_loaded("glGetTransformFeedbacki_v"),
  }
}
static glGetTransformFeedbacki_v_p: GlFnCell<glGetTransformFeedbacki_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTransformFeedbacki_v_is_loaded() -> bool {
  unsafe { *glGetTransformFeedbacki_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbacki_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTransformFeedbacki_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTransformFeedbacki_v_t>>(gl_ptr_filter(f(b"glGetTransformFeedbacki_v\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbacki_v_reset_ptr() {
  *glGetTransformFeedbacki_v_p.0.get() = None;
}
/// [glGetTransformFeedbackiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbackiv.xhtml)
/// * `xfb` class: transform feedback
/// * `pname` group: TransformFeedbackPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTransformFeedbackiv(xfb: GLuint, pname: TransformFeedbackPName, param: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTransformFeedbackiv_p.0.get() } {
    Some(glGetTransformFeedbackiv_inner) => glGetTransformFeedbackiv_inner(xfb, pname, param),
    None => gl_not_loaded("glGetTransformFeedbackiv"),
  }
}
static glGetTransformFeedbackiv_p: GlFnCell<glGetTransformFeedbackiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTransformFeedbackiv_is_loaded() -> bool {
  unsafe { *glGetTransformFeedbackiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbackiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTransformFeedbackiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTransformFeedbackiv_t>>(gl_ptr_filter(f(b"glGetTransformFeedbackiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbackiv_reset_ptr() {
  *glGetTransformFeedbackiv_p.0.get() = None;
}
/// [glGetUniformBlockIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformBlockIndex.xhtml)
/// * `program` class: program
/// * `uniformBlockName` len: COMPSIZE()
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformBlockIndex_p.0.get() } {
    Some(glGetUniformBlockIndex_inner) => glGetUniformBlockIndex_inner(program, uniformBlockName),
    None => gl_not_loaded("glGetUniformBlockIndex"),
  }
}
static glGetUniformBlockIndex_p: GlFnCell<glGetUniformBlockIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformBlockIndex_is_loaded() -> bool {
  unsafe { *glGetUniformBlockIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformBlockIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformBlockIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformBlockIndex_t>>(gl_ptr_filter(f(b"glGetUniformBlockIndex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetUniformBlockIndex_reset_ptr() {
  *glGetUniformBlockIndex_p.0.get() = None;
}
/// [glGetUniformIndices](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformIndices.xhtml)
/// * `program` class: program
/// * `uniformNames` len: COMPSIZE(uniformCount)
/// * `uniformIndices` len: COMPSIZE(uniformCount)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformIndices_p.0.get() } {
    Some(glGetUniformIndices_inner) => glGetUniformIndices_inner(program, uniformCount, uniformNames, uniformIndices),
    None => gl_not_loaded("glGetUniformIndices"),
  }
}
static glGetUniformIndices_p: GlFnCell<glGetUniformIndices_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformIndices_is_loaded() -> bool {
  unsafe { *glGetUniformIndices_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformIndices_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformIndices_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformIndices_t>>(gl_ptr_filter(f(b"glGetUniformIndices\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetUniformIndices_reset_ptr() {
  *glGetUniformIndices_p.0.get() = None;
}
/// [glGetUniformLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformLocation.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformLocation_p.0.get() } {
    Some(glGetUniformLocation_inner) => glGetUniformLocation_inner(program, name),
    None => gl_not_loaded("glGetUniformLocation"),
  }
}
static glGetUniformLocation_p: GlFnCell<glGetUniformLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformLocation_is_loaded() -> bool {
  unsafe { *glGetUniformLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformLocation_t>>(gl_ptr_filter(f(b"glGetUniformLocation\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetUniformLocation_reset_ptr() {
  *glGetUniformLocation_p.0.get() = None;
}
/// [glGetUniformSubroutineuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformSubroutineuiv.xhtml)
/// * `shadertype` group: ShaderType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformSubroutineuiv(shadertype: ShaderType, location: GLint, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformSubroutineuiv_p.0.get() } {
    Some(glGetUniformSubroutineuiv_inner) => glGetUniformSubroutineuiv_inner(shadertype, location, params),
    None => gl_not_loaded("glGetUniformSubroutineuiv"),
  }
}
static glGetUniformSubroutineuiv_p: GlFnCell<glGetUniformSubroutineuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformSubroutineuiv_is_loaded() -> bool {
  unsafe { *glGetUniformSubroutineuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformSubroutineuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformSubroutineuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformSubroutineuiv_t>>(gl_ptr_filter(f(b"glGetUniformSubroutineuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetUniformSubroutineuiv_reset_ptr() {
  *glGetUniformSubroutineuiv_p.0.get() = None;
}
/// [glGetUniformdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformdv.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformdv_p.0.get() } {
    Some(glGetUniformdv_inner) => glGetUniformdv_inner(program, location, params),
    None => gl_not_loaded("glGetUniformdv"),
  }
}
static glGetUniformdv_p: GlFnCell<glGetUniformdv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformdv_is_loaded() -> bool {
  unsafe { *glGetUniformdv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformdv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformdv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformdv_t>>(gl_ptr_filter(f(b"glGetUniformdv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetUniformdv_reset_ptr() {
  *glGetUniformdv_p.0.get() = None;
}
/// [glGetUniformfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformfv.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformfv_p.0.get() } {
    Some(glGetUniformfv_inner) => glGetUniformfv_inner(program, location, params),
    None => gl_not_loaded("glGetUniformfv"),
  }
}
static glGetUniformfv_p: GlFnCell<glGetUniformfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformfv_is_loaded() -> bool {
  unsafe { *glGetUniformfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformfv_t>>(gl_ptr_filter(f(b"glGetUniformfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetUniformfv_reset_ptr() {
  *glGetUniformfv_p.0.get() = None;
}
/// [glGetUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformiv.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformiv_p.0.get() } {
    Some(glGetUniformiv_inner) => glGetUniformiv_inner(program, location, params),
    None => gl_not_loaded("glGetUniformiv"),
  }
}
static glGetUniformiv_p: GlFnCell<glGetUniformiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformiv_is_loaded() -> bool {
  unsafe { *glGetUniformiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformiv_t>>(gl_ptr_filter(f(b"glGetUniformiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetUniformiv_reset_ptr() {
  *glGetUniformiv_p.0.get() = None;
}
/// [glGetUniformuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformuiv.xhtml)
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformuiv_p.0.get() } {
    Some(glGetUniformuiv_inner) => glGetUniformuiv_inner(program, location, params),
    None => gl_not_loaded("glGetUniformuiv"),
  }
}
static glGetUniformuiv_p: GlFnCell<glGetUniformuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformuiv_is_loaded() -> bool {
  unsafe { *glGetUniformuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformuiv_t>>(gl_ptr_filter(f(b"glGetUniformuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetUniformuiv_reset_ptr() {
  *glGetUniformuiv_p.0.get() = None;
}
/// [glGetVertexArrayIndexed64iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayIndexed64iv.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexArrayIndexed64iv(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexArrayIndexed64iv_p.0.get() } {
    Some(glGetVertexArrayIndexed64iv_inner) => glGetVertexArrayIndexed64iv_inner(vaobj, index, pname, param),
    None => gl_not_loaded("glGetVertexArrayIndexed64iv"),
  }
}
static glGetVertexArrayIndexed64iv_p: GlFnCell<glGetVertexArrayIndexed64iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexArrayIndexed64iv_is_loaded() -> bool {
  unsafe { *glGetVertexArrayIndexed64iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexArrayIndexed64iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexArrayIndexed64iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexArrayIndexed64iv_t>>(gl_ptr_filter(f(b"glGetVertexArrayIndexed64iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexArrayIndexed64iv_reset_ptr() {
  *glGetVertexArrayIndexed64iv_p.0.get() = None;
}
/// [glGetVertexArrayIndexediv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayIndexediv.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexArrayIndexediv(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexArrayIndexediv_p.0.get() } {
    Some(glGetVertexArrayIndexediv_inner) => glGetVertexArrayIndexediv_inner(vaobj, index, pname, param),
    None => gl_not_loaded("glGetVertexArrayIndexediv"),
  }
}
static glGetVertexArrayIndexediv_p: GlFnCell<glGetVertexArrayIndexediv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexArrayIndexediv_is_loaded() -> bool {
  unsafe { *glGetVertexArrayIndexediv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexArrayIndexediv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexArrayIndexediv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexArrayIndexediv_t>>(gl_ptr_filter(f(b"glGetVertexArrayIndexediv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexArrayIndexediv_reset_ptr() {
  *glGetVertexArrayIndexediv_p.0.get() = None;
}
/// [glGetVertexArrayiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayiv.xhtml)
/// * `vaobj` class: vertex array
/// * `pname` group: VertexArrayPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexArrayiv(vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexArrayiv_p.0.get() } {
    Some(glGetVertexArrayiv_inner) => glGetVertexArrayiv_inner(vaobj, pname, param),
    None => gl_not_loaded("glGetVertexArrayiv"),
  }
}
static glGetVertexArrayiv_p: GlFnCell<glGetVertexArrayiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexArrayiv_is_loaded() -> bool {
  unsafe { *glGetVertexArrayiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexArrayiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexArrayiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexArrayiv_t>>(gl_ptr_filter(f(b"glGetVertexArrayiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexArrayiv_reset_ptr() {
  *glGetVertexArrayiv_p.0.get() = None;
}
/// [glGetVertexAttribIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribIiv.xhtml)
/// * `pname` group: VertexAttribEnum
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribIiv(index: GLuint, pname: VertexAttribEnum, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribIiv_p.0.get() } {
    Some(glGetVertexAttribIiv_inner) => glGetVertexAttribIiv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribIiv"),
  }
}
static glGetVertexAttribIiv_p: GlFnCell<glGetVertexAttribIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribIiv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribIiv_t>>(gl_ptr_filter(f(b"glGetVertexAttribIiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribIiv_reset_ptr() {
  *glGetVertexAttribIiv_p.0.get() = None;
}
/// [glGetVertexAttribIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribIuiv.xhtml)
/// * `pname` group: VertexAttribEnum
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribIuiv(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribIuiv_p.0.get() } {
    Some(glGetVertexAttribIuiv_inner) => glGetVertexAttribIuiv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribIuiv"),
  }
}
static glGetVertexAttribIuiv_p: GlFnCell<glGetVertexAttribIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribIuiv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribIuiv_t>>(gl_ptr_filter(f(b"glGetVertexAttribIuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribIuiv_reset_ptr() {
  *glGetVertexAttribIuiv_p.0.get() = None;
}
/// [glGetVertexAttribLdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribLdv.xhtml)
/// * `pname` group: VertexAttribEnum
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribLdv(index: GLuint, pname: VertexAttribEnum, params: *mut GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribLdv_p.0.get() } {
    Some(glGetVertexAttribLdv_inner) => glGetVertexAttribLdv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribLdv"),
  }
}
static glGetVertexAttribLdv_p: GlFnCell<glGetVertexAttribLdv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribLdv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribLdv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribLdv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribLdv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribLdv_t>>(gl_ptr_filter(f(b"glGetVertexAttribLdv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribLdv_reset_ptr() {
  *glGetVertexAttribLdv_p.0.get() = None;
}
/// [glGetVertexAttribPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribPointerv.xhtml)
/// * `pname` group: VertexAttribPointerPropertyARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribPointerv(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribPointerv_p.0.get() } {
    Some(glGetVertexAttribPointerv_inner) => glGetVertexAttribPointerv_inner(index, pname, pointer),
    None => gl_not_loaded("glGetVertexAttribPointerv"),
  }
}
static glGetVertexAttribPointerv_p: GlFnCell<glGetVertexAttribPointerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribPointerv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribPointerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribPointerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribPointerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribPointerv_t>>(gl_ptr_filter(f(b"glGetVertexAttribPointerv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribPointerv_reset_ptr() {
  *glGetVertexAttribPointerv_p.0.get() = None;
}
/// [glGetVertexAttribdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribdv.xhtml)
/// * `pname` group: VertexAttribPropertyARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribdv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribdv_p.0.get() } {
    Some(glGetVertexAttribdv_inner) => glGetVertexAttribdv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribdv"),
  }
}
static glGetVertexAttribdv_p: GlFnCell<glGetVertexAttribdv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribdv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribdv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribdv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribdv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribdv_t>>(gl_ptr_filter(f(b"glGetVertexAttribdv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribdv_reset_ptr() {
  *glGetVertexAttribdv_p.0.get() = None;
}
/// [glGetVertexAttribfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribfv.xhtml)
/// * `pname` group: VertexAttribPropertyARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribfv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribfv_p.0.get() } {
    Some(glGetVertexAttribfv_inner) => glGetVertexAttribfv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribfv"),
  }
}
static glGetVertexAttribfv_p: GlFnCell<glGetVertexAttribfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribfv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribfv_t>>(gl_ptr_filter(f(b"glGetVertexAttribfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribfv_reset_ptr() {
  *glGetVertexAttribfv_p.0.get() = None;
}
/// [glGetVertexAttribiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribiv.xhtml)
/// * `pname` group: VertexAttribPropertyARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribiv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribiv_p.0.get() } {
    Some(glGetVertexAttribiv_inner) => glGetVertexAttribiv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribiv"),
  }
}
static glGetVertexAttribiv_p: GlFnCell<glGetVertexAttribiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribiv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribiv_t>>(gl_ptr_filter(f(b"glGetVertexAttribiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribiv_reset_ptr() {
  *glGetVertexAttribiv_p.0.get() = None;
}
/// [glGetnColorTable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnColorTable.xhtml)
/// * `target` group: ColorTableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `table` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnColorTable(target: ColorTableTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, table: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnColorTable_p.0.get() } {
    Some(glGetnColorTable_inner) => glGetnColorTable_inner(target, format, type_, bufSize, table),
    None => gl_not_loaded("glGetnColorTable"),
  }
}
static glGetnColorTable_p: GlFnCell<glGetnColorTable_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnColorTable_is_loaded() -> bool {
  unsafe { *glGetnColorTable_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnColorTable_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnColorTable_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnColorTable_t>>(gl_ptr_filter(f(b"glGetnColorTable\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnColorTable_reset_ptr() {
  *glGetnColorTable_p.0.get() = None;
}
/// [glGetnCompressedTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnCompressedTexImage.xhtml)
/// * `target` group: TextureTarget
/// * `pixels` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnCompressedTexImage(target: TextureTarget, lod: GLint, bufSize: GLsizei, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnCompressedTexImage_p.0.get() } {
    Some(glGetnCompressedTexImage_inner) => glGetnCompressedTexImage_inner(target, lod, bufSize, pixels),
    None => gl_not_loaded("glGetnCompressedTexImage"),
  }
}
static glGetnCompressedTexImage_p: GlFnCell<glGetnCompressedTexImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnCompressedTexImage_is_loaded() -> bool {
  unsafe { *glGetnCompressedTexImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnCompressedTexImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnCompressedTexImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnCompressedTexImage_t>>(gl_ptr_filter(f(b"glGetnCompressedTexImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnCompressedTexImage_reset_ptr() {
  *glGetnCompressedTexImage_p.0.get() = None;
}
/// [glGetnConvolutionFilter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnConvolutionFilter.xhtml)
/// * `target` group: ConvolutionTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `image` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnConvolutionFilter(target: ConvolutionTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, image: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnConvolutionFilter_p.0.get() } {
    Some(glGetnConvolutionFilter_inner) => glGetnConvolutionFilter_inner(target, format, type_, bufSize, image),
    None => gl_not_loaded("glGetnConvolutionFilter"),
  }
}
static glGetnConvolutionFilter_p: GlFnCell<glGetnConvolutionFilter_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnConvolutionFilter_is_loaded() -> bool {
  unsafe { *glGetnConvolutionFilter_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnConvolutionFilter_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnConvolutionFilter_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnConvolutionFilter_t>>(gl_ptr_filter(f(b"glGetnConvolutionFilter\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnConvolutionFilter_reset_ptr() {
  *glGetnConvolutionFilter_p.0.get() = None;
}
/// [glGetnHistogram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnHistogram.xhtml)
/// * `target` group: HistogramTarget
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnHistogram(target: HistogramTarget, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnHistogram_p.0.get() } {
    Some(glGetnHistogram_inner) => glGetnHistogram_inner(target, reset, format, type_, bufSize, values),
    None => gl_not_loaded("glGetnHistogram"),
  }
}
static glGetnHistogram_p: GlFnCell<glGetnHistogram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnHistogram_is_loaded() -> bool {
  unsafe { *glGetnHistogram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnHistogram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnHistogram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnHistogram_t>>(gl_ptr_filter(f(b"glGetnHistogram\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnHistogram_reset_ptr() {
  *glGetnHistogram_p.0.get() = None;
}
/// [glGetnMapdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapdv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
/// * `v` len: COMPSIZE(bufSize)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnMapdv(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnMapdv_p.0.get() } {
    Some(glGetnMapdv_inner) => glGetnMapdv_inner(target, query, bufSize, v),
    None => gl_not_loaded("glGetnMapdv"),
  }
}
static glGetnMapdv_p: GlFnCell<glGetnMapdv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnMapdv_is_loaded() -> bool {
  unsafe { *glGetnMapdv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnMapdv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnMapdv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnMapdv_t>>(gl_ptr_filter(f(b"glGetnMapdv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnMapdv_reset_ptr() {
  *glGetnMapdv_p.0.get() = None;
}
/// [glGetnMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapfv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnMapfv(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnMapfv_p.0.get() } {
    Some(glGetnMapfv_inner) => glGetnMapfv_inner(target, query, bufSize, v),
    None => gl_not_loaded("glGetnMapfv"),
  }
}
static glGetnMapfv_p: GlFnCell<glGetnMapfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnMapfv_is_loaded() -> bool {
  unsafe { *glGetnMapfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnMapfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnMapfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnMapfv_t>>(gl_ptr_filter(f(b"glGetnMapfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnMapfv_reset_ptr() {
  *glGetnMapfv_p.0.get() = None;
}
/// [glGetnMapiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapiv.xhtml)
/// * `target` group: MapTarget
/// * `query` group: MapQuery
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnMapiv(target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnMapiv_p.0.get() } {
    Some(glGetnMapiv_inner) => glGetnMapiv_inner(target, query, bufSize, v),
    None => gl_not_loaded("glGetnMapiv"),
  }
}
static glGetnMapiv_p: GlFnCell<glGetnMapiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnMapiv_is_loaded() -> bool {
  unsafe { *glGetnMapiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnMapiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnMapiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnMapiv_t>>(gl_ptr_filter(f(b"glGetnMapiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnMapiv_reset_ptr() {
  *glGetnMapiv_p.0.get() = None;
}
/// [glGetnMinmax](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMinmax.xhtml)
/// * `target` group: MinmaxTarget
/// * `reset` group: Boolean
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `values` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnMinmax(target: MinmaxTarget, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnMinmax_p.0.get() } {
    Some(glGetnMinmax_inner) => glGetnMinmax_inner(target, reset, format, type_, bufSize, values),
    None => gl_not_loaded("glGetnMinmax"),
  }
}
static glGetnMinmax_p: GlFnCell<glGetnMinmax_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnMinmax_is_loaded() -> bool {
  unsafe { *glGetnMinmax_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnMinmax_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnMinmax_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnMinmax_t>>(gl_ptr_filter(f(b"glGetnMinmax\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnMinmax_reset_ptr() {
  *glGetnMinmax_p.0.get() = None;
}
/// [glGetnPixelMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapfv.xhtml)
/// * `map` group: PixelMap
/// * `values` len: COMPSIZE(bufSize)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnPixelMapfv(map: PixelMap, bufSize: GLsizei, values: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnPixelMapfv_p.0.get() } {
    Some(glGetnPixelMapfv_inner) => glGetnPixelMapfv_inner(map, bufSize, values),
    None => gl_not_loaded("glGetnPixelMapfv"),
  }
}
static glGetnPixelMapfv_p: GlFnCell<glGetnPixelMapfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnPixelMapfv_is_loaded() -> bool {
  unsafe { *glGetnPixelMapfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnPixelMapfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnPixelMapfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnPixelMapfv_t>>(gl_ptr_filter(f(b"glGetnPixelMapfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnPixelMapfv_reset_ptr() {
  *glGetnPixelMapfv_p.0.get() = None;
}
/// [glGetnPixelMapuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapuiv.xhtml)
/// * `map` group: PixelMap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnPixelMapuiv(map: PixelMap, bufSize: GLsizei, values: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnPixelMapuiv_p.0.get() } {
    Some(glGetnPixelMapuiv_inner) => glGetnPixelMapuiv_inner(map, bufSize, values),
    None => gl_not_loaded("glGetnPixelMapuiv"),
  }
}
static glGetnPixelMapuiv_p: GlFnCell<glGetnPixelMapuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnPixelMapuiv_is_loaded() -> bool {
  unsafe { *glGetnPixelMapuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnPixelMapuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnPixelMapuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnPixelMapuiv_t>>(gl_ptr_filter(f(b"glGetnPixelMapuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnPixelMapuiv_reset_ptr() {
  *glGetnPixelMapuiv_p.0.get() = None;
}
/// [glGetnPixelMapusv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapusv.xhtml)
/// * `map` group: PixelMap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnPixelMapusv(map: PixelMap, bufSize: GLsizei, values: *mut GLushort) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnPixelMapusv_p.0.get() } {
    Some(glGetnPixelMapusv_inner) => glGetnPixelMapusv_inner(map, bufSize, values),
    None => gl_not_loaded("glGetnPixelMapusv"),
  }
}
static glGetnPixelMapusv_p: GlFnCell<glGetnPixelMapusv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnPixelMapusv_is_loaded() -> bool {
  unsafe { *glGetnPixelMapusv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnPixelMapusv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnPixelMapusv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnPixelMapusv_t>>(gl_ptr_filter(f(b"glGetnPixelMapusv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnPixelMapusv_reset_ptr() {
  *glGetnPixelMapusv_p.0.get() = None;
}
/// [glGetnPolygonStipple](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPolygonStipple.xhtml)
/// * `pattern` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnPolygonStipple(bufSize: GLsizei, pattern: *mut GLubyte) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnPolygonStipple_p.0.get() } {
    Some(glGetnPolygonStipple_inner) => glGetnPolygonStipple_inner(bufSize, pattern),
    None => gl_not_loaded("glGetnPolygonStipple"),
  }
}
static glGetnPolygonStipple_p: GlFnCell<glGetnPolygonStipple_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnPolygonStipple_is_loaded() -> bool {
  unsafe { *glGetnPolygonStipple_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnPolygonStipple_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnPolygonStipple_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnPolygonStipple_t>>(gl_ptr_filter(f(b"glGetnPolygonStipple\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnPolygonStipple_reset_ptr() {
  *glGetnPolygonStipple_p.0.get() = None;
}
/// [glGetnSeparableFilter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnSeparableFilter.xhtml)
/// * `target` group: SeparableTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `row` len: rowBufSize
/// * `column` len: columnBufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnSeparableFilter(target: SeparableTarget, format: PixelFormat, type_: PixelType, rowBufSize: GLsizei, row: *mut void, columnBufSize: GLsizei, column: *mut void, span: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnSeparableFilter_p.0.get() } {
    Some(glGetnSeparableFilter_inner) => glGetnSeparableFilter_inner(target, format, type_, rowBufSize, row, columnBufSize, column, span),
    None => gl_not_loaded("glGetnSeparableFilter"),
  }
}
static glGetnSeparableFilter_p: GlFnCell<glGetnSeparableFilter_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnSeparableFilter_is_loaded() -> bool {
  unsafe { *glGetnSeparableFilter_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnSeparableFilter_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnSeparableFilter_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnSeparableFilter_t>>(gl_ptr_filter(f(b"glGetnSeparableFilter\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnSeparableFilter_reset_ptr() {
  *glGetnSeparableFilter_p.0.get() = None;
}
/// [glGetnTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnTexImage.xhtml)
/// * `target` group: TextureTarget
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnTexImage(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnTexImage_p.0.get() } {
    Some(glGetnTexImage_inner) => glGetnTexImage_inner(target, level, format, type_, bufSize, pixels),
    None => gl_not_loaded("glGetnTexImage"),
  }
}
static glGetnTexImage_p: GlFnCell<glGetnTexImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnTexImage_is_loaded() -> bool {
  unsafe { *glGetnTexImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnTexImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnTexImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnTexImage_t>>(gl_ptr_filter(f(b"glGetnTexImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnTexImage_reset_ptr() {
  *glGetnTexImage_p.0.get() = None;
}
/// [glGetnUniformdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformdv.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnUniformdv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnUniformdv_p.0.get() } {
    Some(glGetnUniformdv_inner) => glGetnUniformdv_inner(program, location, bufSize, params),
    None => gl_not_loaded("glGetnUniformdv"),
  }
}
static glGetnUniformdv_p: GlFnCell<glGetnUniformdv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnUniformdv_is_loaded() -> bool {
  unsafe { *glGetnUniformdv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnUniformdv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnUniformdv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnUniformdv_t>>(gl_ptr_filter(f(b"glGetnUniformdv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnUniformdv_reset_ptr() {
  *glGetnUniformdv_p.0.get() = None;
}
/// [glGetnUniformfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformfv.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnUniformfv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnUniformfv_p.0.get() } {
    Some(glGetnUniformfv_inner) => glGetnUniformfv_inner(program, location, bufSize, params),
    None => gl_not_loaded("glGetnUniformfv"),
  }
}
static glGetnUniformfv_p: GlFnCell<glGetnUniformfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnUniformfv_is_loaded() -> bool {
  unsafe { *glGetnUniformfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnUniformfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnUniformfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnUniformfv_t>>(gl_ptr_filter(f(b"glGetnUniformfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnUniformfv_reset_ptr() {
  *glGetnUniformfv_p.0.get() = None;
}
/// [glGetnUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformiv.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnUniformiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnUniformiv_p.0.get() } {
    Some(glGetnUniformiv_inner) => glGetnUniformiv_inner(program, location, bufSize, params),
    None => gl_not_loaded("glGetnUniformiv"),
  }
}
static glGetnUniformiv_p: GlFnCell<glGetnUniformiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnUniformiv_is_loaded() -> bool {
  unsafe { *glGetnUniformiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnUniformiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnUniformiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnUniformiv_t>>(gl_ptr_filter(f(b"glGetnUniformiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnUniformiv_reset_ptr() {
  *glGetnUniformiv_p.0.get() = None;
}
/// [glGetnUniformuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformuiv.xhtml)
/// * `program` class: program
/// * `params` len: bufSize / 4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetnUniformuiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetnUniformuiv_p.0.get() } {
    Some(glGetnUniformuiv_inner) => glGetnUniformuiv_inner(program, location, bufSize, params),
    None => gl_not_loaded("glGetnUniformuiv"),
  }
}
static glGetnUniformuiv_p: GlFnCell<glGetnUniformuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetnUniformuiv_is_loaded() -> bool {
  unsafe { *glGetnUniformuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetnUniformuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetnUniformuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetnUniformuiv_t>>(gl_ptr_filter(f(b"glGetnUniformuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glGetnUniformuiv_reset_ptr() {
  *glGetnUniformuiv_p.0.get() = None;
}
/// [glHint](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glHint.xhtml)
/// * `target` group: HintTarget
/// * `mode` group: HintMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glHint(target: HintTarget, mode: HintMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glHint_p.0.get() } {
    Some(glHint_inner) => glHint_inner(target, mode),
    None => gl_not_loaded("glHint"),
  }
}
static glHint_p: GlFnCell<glHint_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glHint_is_loaded() -> bool {
  unsafe { *glHint_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glHint_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glHint_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glHint_t>>(gl_ptr_filter(f(b"glHint\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glHint_reset_ptr() {
  *glHint_p.0.get() = None;
}
/// [glInvalidateBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateBufferData.xhtml)
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glInvalidateBufferData(buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glInvalidateBufferData_p.0.get() } {
    Some(glInvalidateBufferData_inner) => glInvalidateBufferData_inner(buffer),
    None => gl_not_loaded("glInvalidateBufferData"),
  }
}
static glInvalidateBufferData_p: GlFnCell<glInvalidateBufferData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glInvalidateBufferData_is_loaded() -> bool {
  unsafe { *glInvalidateBufferData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glInvalidateBufferData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glInvalidateBufferData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glInvalidateBufferData_t>>(gl_ptr_filter(f(b"glInvalidateBufferData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glInvalidateBufferData_reset_ptr() {
  *glInvalidateBufferData_p.0.get() = None;
}
/// [glInvalidateBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateBufferSubData.xhtml)
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glInvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glInvalidateBufferSubData_p.0.get() } {
    Some(glInvalidateBufferSubData_inner) => glInvalidateBufferSubData_inner(buffer, offset, length),
    None => gl_not_loaded("glInvalidateBufferSubData"),
  }
}
static glInvalidateBufferSubData_p: GlFnCell<glInvalidateBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glInvalidateBufferSubData_is_loaded() -> bool {
  unsafe { *glInvalidateBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glInvalidateBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glInvalidateBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glInvalidateBufferSubData_t>>(gl_ptr_filter(f(b"glInvalidateBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glInvalidateBufferSubData_reset_ptr() {
  *glInvalidateBufferSubData_p.0.get() = None;
}
/// [glInvalidateFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateFramebuffer.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachments` group: InvalidateFramebufferAttachment
/// * `attachments` len: numAttachments
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glInvalidateFramebuffer(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment) {
  #[allow(unused_unsafe)]
  match unsafe { *glInvalidateFramebuffer_p.0.get() } {
    Some(glInvalidateFramebuffer_inner) => glInvalidateFramebuffer_inner(target, numAttachments, attachments),
    None => gl_not_loaded("glInvalidateFramebuffer"),
  }
}
static glInvalidateFramebuffer_p: GlFnCell<glInvalidateFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glInvalidateFramebuffer_is_loaded() -> bool {
  unsafe { *glInvalidateFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glInvalidateFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glInvalidateFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glInvalidateFramebuffer_t>>(gl_ptr_filter(f(b"glInvalidateFramebuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glInvalidateFramebuffer_reset_ptr() {
  *glInvalidateFramebuffer_p.0.get() = None;
}
/// [glInvalidateNamedFramebufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateNamedFramebufferData.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachments` group: FramebufferAttachment
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glInvalidateNamedFramebufferData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment) {
  #[allow(unused_unsafe)]
  match unsafe { *glInvalidateNamedFramebufferData_p.0.get() } {
    Some(glInvalidateNamedFramebufferData_inner) => glInvalidateNamedFramebufferData_inner(framebuffer, numAttachments, attachments),
    None => gl_not_loaded("glInvalidateNamedFramebufferData"),
  }
}
static glInvalidateNamedFramebufferData_p: GlFnCell<glInvalidateNamedFramebufferData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glInvalidateNamedFramebufferData_is_loaded() -> bool {
  unsafe { *glInvalidateNamedFramebufferData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glInvalidateNamedFramebufferData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glInvalidateNamedFramebufferData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glInvalidateNamedFramebufferData_t>>(gl_ptr_filter(f(b"glInvalidateNamedFramebufferData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glInvalidateNamedFramebufferData_reset_ptr() {
  *glInvalidateNamedFramebufferData_p.0.get() = None;
}
/// [glInvalidateNamedFramebufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateNamedFramebufferSubData.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachments` group: FramebufferAttachment
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glInvalidateNamedFramebufferSubData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glInvalidateNamedFramebufferSubData_p.0.get() } {
    Some(glInvalidateNamedFramebufferSubData_inner) => glInvalidateNamedFramebufferSubData_inner(framebuffer, numAttachments, attachments, x, y, width, height),
    None => gl_not_loaded("glInvalidateNamedFramebufferSubData"),
  }
}
static glInvalidateNamedFramebufferSubData_p: GlFnCell<glInvalidateNamedFramebufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glInvalidateNamedFramebufferSubData_is_loaded() -> bool {
  unsafe { *glInvalidateNamedFramebufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glInvalidateNamedFramebufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glInvalidateNamedFramebufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glInvalidateNamedFramebufferSubData_t>>(gl_ptr_filter(f(b"glInvalidateNamedFramebufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glInvalidateNamedFramebufferSubData_reset_ptr() {
  *glInvalidateNamedFramebufferSubData_p.0.get() = None;
}
/// [glInvalidateSubFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateSubFramebuffer.xhtml)
/// * `target` group: FramebufferTarget
/// * `attachments` group: InvalidateFramebufferAttachment
/// * `attachments` len: numAttachments
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glInvalidateSubFramebuffer(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glInvalidateSubFramebuffer_p.0.get() } {
    Some(glInvalidateSubFramebuffer_inner) => glInvalidateSubFramebuffer_inner(target, numAttachments, attachments, x, y, width, height),
    None => gl_not_loaded("glInvalidateSubFramebuffer"),
  }
}
static glInvalidateSubFramebuffer_p: GlFnCell<glInvalidateSubFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glInvalidateSubFramebuffer_is_loaded() -> bool {
  unsafe { *glInvalidateSubFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glInvalidateSubFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glInvalidateSubFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glInvalidateSubFramebuffer_t>>(gl_ptr_filter(f(b"glInvalidateSubFramebuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glInvalidateSubFramebuffer_reset_ptr() {
  *glInvalidateSubFramebuffer_p.0.get() = None;
}
/// [glInvalidateTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateTexImage.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glInvalidateTexImage(texture: GLuint, level: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glInvalidateTexImage_p.0.get() } {
    Some(glInvalidateTexImage_inner) => glInvalidateTexImage_inner(texture, level),
    None => gl_not_loaded("glInvalidateTexImage"),
  }
}
static glInvalidateTexImage_p: GlFnCell<glInvalidateTexImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glInvalidateTexImage_is_loaded() -> bool {
  unsafe { *glInvalidateTexImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glInvalidateTexImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glInvalidateTexImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glInvalidateTexImage_t>>(gl_ptr_filter(f(b"glInvalidateTexImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glInvalidateTexImage_reset_ptr() {
  *glInvalidateTexImage_p.0.get() = None;
}
/// [glInvalidateTexSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateTexSubImage.xhtml)
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glInvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glInvalidateTexSubImage_p.0.get() } {
    Some(glInvalidateTexSubImage_inner) => glInvalidateTexSubImage_inner(texture, level, xoffset, yoffset, zoffset, width, height, depth),
    None => gl_not_loaded("glInvalidateTexSubImage"),
  }
}
static glInvalidateTexSubImage_p: GlFnCell<glInvalidateTexSubImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glInvalidateTexSubImage_is_loaded() -> bool {
  unsafe { *glInvalidateTexSubImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glInvalidateTexSubImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glInvalidateTexSubImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glInvalidateTexSubImage_t>>(gl_ptr_filter(f(b"glInvalidateTexSubImage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glInvalidateTexSubImage_reset_ptr() {
  *glInvalidateTexSubImage_p.0.get() = None;
}
/// [glIsBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsBuffer.xhtml)
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsBuffer(buffer: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsBuffer_p.0.get() } {
    Some(glIsBuffer_inner) => glIsBuffer_inner(buffer),
    None => gl_not_loaded("glIsBuffer"),
  }
}
static glIsBuffer_p: GlFnCell<glIsBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsBuffer_is_loaded() -> bool {
  unsafe { *glIsBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsBuffer_t>>(gl_ptr_filter(f(b"glIsBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsBuffer_reset_ptr() {
  *glIsBuffer_p.0.get() = None;
}
/// [glIsEnabled](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnabled.xhtml)
/// * `cap` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsEnabled(cap: EnableCap) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsEnabled_p.0.get() } {
    Some(glIsEnabled_inner) => glIsEnabled_inner(cap),
    None => gl_not_loaded("glIsEnabled"),
  }
}
static glIsEnabled_p: GlFnCell<glIsEnabled_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsEnabled_is_loaded() -> bool {
  unsafe { *glIsEnabled_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsEnabled_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsEnabled_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsEnabled_t>>(gl_ptr_filter(f(b"glIsEnabled\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsEnabled_reset_ptr() {
  *glIsEnabled_p.0.get() = None;
}
/// [glIsEnabledi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnabledi.xhtml)
/// * `target` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsEnabledi(target: EnableCap, index: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsEnabledi_p.0.get() } {
    Some(glIsEnabledi_inner) => glIsEnabledi_inner(target, index),
    None => gl_not_loaded("glIsEnabledi"),
  }
}
static glIsEnabledi_p: GlFnCell<glIsEnabledi_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsEnabledi_is_loaded() -> bool {
  unsafe { *glIsEnabledi_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsEnabledi_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsEnabledi_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsEnabledi_t>>(gl_ptr_filter(f(b"glIsEnabledi\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsEnabledi_reset_ptr() {
  *glIsEnabledi_p.0.get() = None;
}
/// [glIsFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsFramebuffer.xhtml)
/// * `framebuffer` class: framebuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsFramebuffer_p.0.get() } {
    Some(glIsFramebuffer_inner) => glIsFramebuffer_inner(framebuffer),
    None => gl_not_loaded("glIsFramebuffer"),
  }
}
static glIsFramebuffer_p: GlFnCell<glIsFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsFramebuffer_is_loaded() -> bool {
  unsafe { *glIsFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsFramebuffer_t>>(gl_ptr_filter(f(b"glIsFramebuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsFramebuffer_reset_ptr() {
  *glIsFramebuffer_p.0.get() = None;
}
/// [glIsProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgram.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsProgram(program: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsProgram_p.0.get() } {
    Some(glIsProgram_inner) => glIsProgram_inner(program),
    None => gl_not_loaded("glIsProgram"),
  }
}
static glIsProgram_p: GlFnCell<glIsProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsProgram_is_loaded() -> bool {
  unsafe { *glIsProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsProgram_t>>(gl_ptr_filter(f(b"glIsProgram\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsProgram_reset_ptr() {
  *glIsProgram_p.0.get() = None;
}
/// [glIsProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgramPipeline.xhtml)
/// * `pipeline` class: program pipeline
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsProgramPipeline(pipeline: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsProgramPipeline_p.0.get() } {
    Some(glIsProgramPipeline_inner) => glIsProgramPipeline_inner(pipeline),
    None => gl_not_loaded("glIsProgramPipeline"),
  }
}
static glIsProgramPipeline_p: GlFnCell<glIsProgramPipeline_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsProgramPipeline_is_loaded() -> bool {
  unsafe { *glIsProgramPipeline_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsProgramPipeline_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsProgramPipeline_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsProgramPipeline_t>>(gl_ptr_filter(f(b"glIsProgramPipeline\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsProgramPipeline_reset_ptr() {
  *glIsProgramPipeline_p.0.get() = None;
}
/// [glIsQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsQuery.xhtml)
/// * `id` class: query
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsQuery(id: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsQuery_p.0.get() } {
    Some(glIsQuery_inner) => glIsQuery_inner(id),
    None => gl_not_loaded("glIsQuery"),
  }
}
static glIsQuery_p: GlFnCell<glIsQuery_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsQuery_is_loaded() -> bool {
  unsafe { *glIsQuery_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsQuery_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsQuery_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsQuery_t>>(gl_ptr_filter(f(b"glIsQuery\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsQuery_reset_ptr() {
  *glIsQuery_p.0.get() = None;
}
/// [glIsRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsRenderbuffer.xhtml)
/// * `renderbuffer` class: renderbuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsRenderbuffer_p.0.get() } {
    Some(glIsRenderbuffer_inner) => glIsRenderbuffer_inner(renderbuffer),
    None => gl_not_loaded("glIsRenderbuffer"),
  }
}
static glIsRenderbuffer_p: GlFnCell<glIsRenderbuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsRenderbuffer_is_loaded() -> bool {
  unsafe { *glIsRenderbuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsRenderbuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsRenderbuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsRenderbuffer_t>>(gl_ptr_filter(f(b"glIsRenderbuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsRenderbuffer_reset_ptr() {
  *glIsRenderbuffer_p.0.get() = None;
}
/// [glIsSampler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsSampler.xhtml)
/// * `sampler` class: sampler
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsSampler(sampler: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsSampler_p.0.get() } {
    Some(glIsSampler_inner) => glIsSampler_inner(sampler),
    None => gl_not_loaded("glIsSampler"),
  }
}
static glIsSampler_p: GlFnCell<glIsSampler_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsSampler_is_loaded() -> bool {
  unsafe { *glIsSampler_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsSampler_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsSampler_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsSampler_t>>(gl_ptr_filter(f(b"glIsSampler\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsSampler_reset_ptr() {
  *glIsSampler_p.0.get() = None;
}
/// [glIsShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsShader.xhtml)
/// * `shader` class: shader
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsShader(shader: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsShader_p.0.get() } {
    Some(glIsShader_inner) => glIsShader_inner(shader),
    None => gl_not_loaded("glIsShader"),
  }
}
static glIsShader_p: GlFnCell<glIsShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsShader_is_loaded() -> bool {
  unsafe { *glIsShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsShader_t>>(gl_ptr_filter(f(b"glIsShader\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsShader_reset_ptr() {
  *glIsShader_p.0.get() = None;
}
/// [glIsSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsSync.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsSync(sync: GLsync) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsSync_p.0.get() } {
    Some(glIsSync_inner) => glIsSync_inner(sync),
    None => gl_not_loaded("glIsSync"),
  }
}
static glIsSync_p: GlFnCell<glIsSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsSync_is_loaded() -> bool {
  unsafe { *glIsSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsSync_t>>(gl_ptr_filter(f(b"glIsSync\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsSync_reset_ptr() {
  *glIsSync_p.0.get() = None;
}
/// [glIsTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTexture.xhtml)
/// * `texture` group: Texture
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsTexture(texture: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsTexture_p.0.get() } {
    Some(glIsTexture_inner) => glIsTexture_inner(texture),
    None => gl_not_loaded("glIsTexture"),
  }
}
static glIsTexture_p: GlFnCell<glIsTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsTexture_is_loaded() -> bool {
  unsafe { *glIsTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsTexture_t>>(gl_ptr_filter(f(b"glIsTexture\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsTexture_reset_ptr() {
  *glIsTexture_p.0.get() = None;
}
/// [glIsTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTransformFeedback.xhtml)
/// * `id` class: transform feedback
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsTransformFeedback(id: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsTransformFeedback_p.0.get() } {
    Some(glIsTransformFeedback_inner) => glIsTransformFeedback_inner(id),
    None => gl_not_loaded("glIsTransformFeedback"),
  }
}
static glIsTransformFeedback_p: GlFnCell<glIsTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsTransformFeedback_is_loaded() -> bool {
  unsafe { *glIsTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsTransformFeedback_t>>(gl_ptr_filter(f(b"glIsTransformFeedback\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsTransformFeedback_reset_ptr() {
  *glIsTransformFeedback_p.0.get() = None;
}
/// [glIsVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsVertexArray.xhtml)
/// * `array` class: vertex array
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsVertexArray(array: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsVertexArray_p.0.get() } {
    Some(glIsVertexArray_inner) => glIsVertexArray_inner(array),
    None => gl_not_loaded("glIsVertexArray"),
  }
}
static glIsVertexArray_p: GlFnCell<glIsVertexArray_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsVertexArray_is_loaded() -> bool {
  unsafe { *glIsVertexArray_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsVertexArray_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsVertexArray_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsVertexArray_t>>(gl_ptr_filter(f(b"glIsVertexArray\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glIsVertexArray_reset_ptr() {
  *glIsVertexArray_p.0.get() = None;
}
/// [glLineWidth](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLineWidth.xhtml)
/// * `width` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glLineWidth(width: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glLineWidth_p.0.get() } {
    Some(glLineWidth_inner) => glLineWidth_inner(width),
    None => gl_not_loaded("glLineWidth"),
  }
}
static glLineWidth_p: GlFnCell<glLineWidth_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glLineWidth_is_loaded() -> bool {
  unsafe { *glLineWidth_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glLineWidth_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glLineWidth_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glLineWidth_t>>(gl_ptr_filter(f(b"glLineWidth\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glLineWidth_reset_ptr() {
  *glLineWidth_p.0.get() = None;
}
/// [glLinkProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glLinkProgram(program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glLinkProgram_p.0.get() } {
    Some(glLinkProgram_inner) => glLinkProgram_inner(program),
    None => gl_not_loaded("glLinkProgram"),
  }
}
static glLinkProgram_p: GlFnCell<glLinkProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glLinkProgram_is_loaded() -> bool {
  unsafe { *glLinkProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glLinkProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glLinkProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glLinkProgram_t>>(gl_ptr_filter(f(b"glLinkProgram\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glLinkProgram_reset_ptr() {
  *glLinkProgram_p.0.get() = None;
}
/// [glLogicOp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLogicOp.xhtml)
/// * `opcode` group: LogicOp
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glLogicOp(opcode: LogicOp) {
  #[allow(unused_unsafe)]
  match unsafe { *glLogicOp_p.0.get() } {
    Some(glLogicOp_inner) => glLogicOp_inner(opcode),
    None => gl_not_loaded("glLogicOp"),
  }
}
static glLogicOp_p: GlFnCell<glLogicOp_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glLogicOp_is_loaded() -> bool {
  unsafe { *glLogicOp_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glLogicOp_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glLogicOp_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glLogicOp_t>>(gl_ptr_filter(f(b"glLogicOp\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glLogicOp_reset_ptr() {
  *glLogicOp_p.0.get() = None;
}
/// [glMapBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBuffer.xhtml)
/// * `target` group: BufferTargetARB
/// * `access` group: BufferAccessARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMapBuffer(target: BufferTargetARB, access: BufferAccessARB) -> *mut void {
  #[allow(unused_unsafe)]
  match unsafe { *glMapBuffer_p.0.get() } {
    Some(glMapBuffer_inner) => glMapBuffer_inner(target, access),
    None => gl_not_loaded("glMapBuffer"),
  }
}
static glMapBuffer_p: GlFnCell<glMapBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMapBuffer_is_loaded() -> bool {
  unsafe { *glMapBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMapBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMapBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMapBuffer_t>>(gl_ptr_filter(f(b"glMapBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMapBuffer_reset_ptr() {
  *glMapBuffer_p.0.get() = None;
}
/// [glMapBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBufferRange.xhtml)
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
/// * `access` group: MapBufferAccessMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMapBufferRange(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void {
  #[allow(unused_unsafe)]
  match unsafe { *glMapBufferRange_p.0.get() } {
    Some(glMapBufferRange_inner) => glMapBufferRange_inner(target, offset, length, access),
    None => gl_not_loaded("glMapBufferRange"),
  }
}
static glMapBufferRange_p: GlFnCell<glMapBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMapBufferRange_is_loaded() -> bool {
  unsafe { *glMapBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMapBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMapBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMapBufferRange_t>>(gl_ptr_filter(f(b"glMapBufferRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMapBufferRange_reset_ptr() {
  *glMapBufferRange_p.0.get() = None;
}
/// [glMapNamedBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapNamedBuffer.xhtml)
/// * `buffer` class: buffer
/// * `access` group: BufferAccessARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMapNamedBuffer(buffer: GLuint, access: BufferAccessARB) -> *mut void {
  #[allow(unused_unsafe)]
  match unsafe { *glMapNamedBuffer_p.0.get() } {
    Some(glMapNamedBuffer_inner) => glMapNamedBuffer_inner(buffer, access),
    None => gl_not_loaded("glMapNamedBuffer"),
  }
}
static glMapNamedBuffer_p: GlFnCell<glMapNamedBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMapNamedBuffer_is_loaded() -> bool {
  unsafe { *glMapNamedBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMapNamedBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMapNamedBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMapNamedBuffer_t>>(gl_ptr_filter(f(b"glMapNamedBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMapNamedBuffer_reset_ptr() {
  *glMapNamedBuffer_p.0.get() = None;
}
/// [glMapNamedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapNamedBufferRange.xhtml)
/// * `buffer` class: buffer
/// * `length` group: BufferSize
/// * `access` group: MapBufferAccessMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMapNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void {
  #[allow(unused_unsafe)]
  match unsafe { *glMapNamedBufferRange_p.0.get() } {
    Some(glMapNamedBufferRange_inner) => glMapNamedBufferRange_inner(buffer, offset, length, access),
    None => gl_not_loaded("glMapNamedBufferRange"),
  }
}
static glMapNamedBufferRange_p: GlFnCell<glMapNamedBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMapNamedBufferRange_is_loaded() -> bool {
  unsafe { *glMapNamedBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMapNamedBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMapNamedBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMapNamedBufferRange_t>>(gl_ptr_filter(f(b"glMapNamedBufferRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMapNamedBufferRange_reset_ptr() {
  *glMapNamedBufferRange_p.0.get() = None;
}
/// [glMemoryBarrier](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml)
/// * `barriers` group: MemoryBarrierMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMemoryBarrier(barriers: GLbitfield) {
  #[allow(unused_unsafe)]
  match unsafe { *glMemoryBarrier_p.0.get() } {
    Some(glMemoryBarrier_inner) => glMemoryBarrier_inner(barriers),
    None => gl_not_loaded("glMemoryBarrier"),
  }
}
static glMemoryBarrier_p: GlFnCell<glMemoryBarrier_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMemoryBarrier_is_loaded() -> bool {
  unsafe { *glMemoryBarrier_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMemoryBarrier_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMemoryBarrier_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMemoryBarrier_t>>(gl_ptr_filter(f(b"glMemoryBarrier\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMemoryBarrier_reset_ptr() {
  *glMemoryBarrier_p.0.get() = None;
}
/// [glMemoryBarrierByRegion](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrierByRegion.xhtml)
/// * `barriers` group: MemoryBarrierMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMemoryBarrierByRegion(barriers: GLbitfield) {
  #[allow(unused_unsafe)]
  match unsafe { *glMemoryBarrierByRegion_p.0.get() } {
    Some(glMemoryBarrierByRegion_inner) => glMemoryBarrierByRegion_inner(barriers),
    None => gl_not_loaded("glMemoryBarrierByRegion"),
  }
}
static glMemoryBarrierByRegion_p: GlFnCell<glMemoryBarrierByRegion_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMemoryBarrierByRegion_is_loaded() -> bool {
  unsafe { *glMemoryBarrierByRegion_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMemoryBarrierByRegion_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMemoryBarrierByRegion_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMemoryBarrierByRegion_t>>(gl_ptr_filter(f(b"glMemoryBarrierByRegion\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMemoryBarrierByRegion_reset_ptr() {
  *glMemoryBarrierByRegion_p.0.get() = None;
}
/// [glMinSampleShading](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMinSampleShading.xhtml)
/// * `value` group: ColorF
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMinSampleShading(value: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glMinSampleShading_p.0.get() } {
    Some(glMinSampleShading_inner) => glMinSampleShading_inner(value),
    None => gl_not_loaded("glMinSampleShading"),
  }
}
static glMinSampleShading_p: GlFnCell<glMinSampleShading_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMinSampleShading_is_loaded() -> bool {
  unsafe { *glMinSampleShading_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMinSampleShading_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMinSampleShading_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMinSampleShading_t>>(gl_ptr_filter(f(b"glMinSampleShading\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMinSampleShading_reset_ptr() {
  *glMinSampleShading_p.0.get() = None;
}
/// [glMultiDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArrays.xhtml)
/// * `mode` group: PrimitiveType
/// * `first` len: COMPSIZE(drawcount)
/// * `count` len: COMPSIZE(drawcount)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawArrays(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawArrays_p.0.get() } {
    Some(glMultiDrawArrays_inner) => glMultiDrawArrays_inner(mode, first, count, drawcount),
    None => gl_not_loaded("glMultiDrawArrays"),
  }
}
static glMultiDrawArrays_p: GlFnCell<glMultiDrawArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawArrays_is_loaded() -> bool {
  unsafe { *glMultiDrawArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawArrays_t>>(gl_ptr_filter(f(b"glMultiDrawArrays\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiDrawArrays_reset_ptr() {
  *glMultiDrawArrays_p.0.get() = None;
}
/// [glMultiDrawArraysIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirect.xhtml)
/// * `mode` group: PrimitiveType
/// * `indirect` len: COMPSIZE(drawcount,stride)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawArraysIndirect(mode: PrimitiveType, indirect: *const void, drawcount: GLsizei, stride: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawArraysIndirect_p.0.get() } {
    Some(glMultiDrawArraysIndirect_inner) => glMultiDrawArraysIndirect_inner(mode, indirect, drawcount, stride),
    None => gl_not_loaded("glMultiDrawArraysIndirect"),
  }
}
static glMultiDrawArraysIndirect_p: GlFnCell<glMultiDrawArraysIndirect_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawArraysIndirect_is_loaded() -> bool {
  unsafe { *glMultiDrawArraysIndirect_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawArraysIndirect_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawArraysIndirect_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawArraysIndirect_t>>(gl_ptr_filter(f(b"glMultiDrawArraysIndirect\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiDrawArraysIndirect_reset_ptr() {
  *glMultiDrawArraysIndirect_p.0.get() = None;
}
/// [glMultiDrawArraysIndirectCount](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirectCount.xhtml)
/// * `mode` group: PrimitiveType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawArraysIndirectCount(mode: PrimitiveType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawArraysIndirectCount_p.0.get() } {
    Some(glMultiDrawArraysIndirectCount_inner) => glMultiDrawArraysIndirectCount_inner(mode, indirect, drawcount, maxdrawcount, stride),
    None => gl_not_loaded("glMultiDrawArraysIndirectCount"),
  }
}
static glMultiDrawArraysIndirectCount_p: GlFnCell<glMultiDrawArraysIndirectCount_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawArraysIndirectCount_is_loaded() -> bool {
  unsafe { *glMultiDrawArraysIndirectCount_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawArraysIndirectCount_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawArraysIndirectCount_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawArraysIndirectCount_t>>(gl_ptr_filter(f(b"glMultiDrawArraysIndirectCount\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiDrawArraysIndirectCount_reset_ptr() {
  *glMultiDrawArraysIndirectCount_p.0.get() = None;
}
/// [glMultiDrawElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElements.xhtml)
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawElements(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawElements_p.0.get() } {
    Some(glMultiDrawElements_inner) => glMultiDrawElements_inner(mode, count, type_, indices, drawcount),
    None => gl_not_loaded("glMultiDrawElements"),
  }
}
static glMultiDrawElements_p: GlFnCell<glMultiDrawElements_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawElements_is_loaded() -> bool {
  unsafe { *glMultiDrawElements_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElements_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawElements_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawElements_t>>(gl_ptr_filter(f(b"glMultiDrawElements\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElements_reset_ptr() {
  *glMultiDrawElements_p.0.get() = None;
}
/// [glMultiDrawElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsBaseVertex.xhtml)
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
/// * `basevertex` len: COMPSIZE(drawcount)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawElementsBaseVertex(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawElementsBaseVertex_p.0.get() } {
    Some(glMultiDrawElementsBaseVertex_inner) => glMultiDrawElementsBaseVertex_inner(mode, count, type_, indices, drawcount, basevertex),
    None => gl_not_loaded("glMultiDrawElementsBaseVertex"),
  }
}
static glMultiDrawElementsBaseVertex_p: GlFnCell<glMultiDrawElementsBaseVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawElementsBaseVertex_is_loaded() -> bool {
  unsafe { *glMultiDrawElementsBaseVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElementsBaseVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawElementsBaseVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawElementsBaseVertex_t>>(gl_ptr_filter(f(b"glMultiDrawElementsBaseVertex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElementsBaseVertex_reset_ptr() {
  *glMultiDrawElementsBaseVertex_p.0.get() = None;
}
/// [glMultiDrawElementsIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirect.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indirect` len: COMPSIZE(drawcount,stride)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawElementsIndirect(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLsizei, stride: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawElementsIndirect_p.0.get() } {
    Some(glMultiDrawElementsIndirect_inner) => glMultiDrawElementsIndirect_inner(mode, type_, indirect, drawcount, stride),
    None => gl_not_loaded("glMultiDrawElementsIndirect"),
  }
}
static glMultiDrawElementsIndirect_p: GlFnCell<glMultiDrawElementsIndirect_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawElementsIndirect_is_loaded() -> bool {
  unsafe { *glMultiDrawElementsIndirect_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElementsIndirect_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawElementsIndirect_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawElementsIndirect_t>>(gl_ptr_filter(f(b"glMultiDrawElementsIndirect\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElementsIndirect_reset_ptr() {
  *glMultiDrawElementsIndirect_p.0.get() = None;
}
/// [glMultiDrawElementsIndirectCount](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirectCount.xhtml)
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawElementsIndirectCount(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawElementsIndirectCount_p.0.get() } {
    Some(glMultiDrawElementsIndirectCount_inner) => glMultiDrawElementsIndirectCount_inner(mode, type_, indirect, drawcount, maxdrawcount, stride),
    None => gl_not_loaded("glMultiDrawElementsIndirectCount"),
  }
}
static glMultiDrawElementsIndirectCount_p: GlFnCell<glMultiDrawElementsIndirectCount_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawElementsIndirectCount_is_loaded() -> bool {
  unsafe { *glMultiDrawElementsIndirectCount_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElementsIndirectCount_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawElementsIndirectCount_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawElementsIndirectCount_t>>(gl_ptr_filter(f(b"glMultiDrawElementsIndirectCount\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElementsIndirectCount_reset_ptr() {
  *glMultiDrawElementsIndirectCount_p.0.get() = None;
}
/// [glMultiTexCoordP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP1ui.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiTexCoordP1ui(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiTexCoordP1ui_p.0.get() } {
    Some(glMultiTexCoordP1ui_inner) => glMultiTexCoordP1ui_inner(texture, type_, coords),
    None => gl_not_loaded("glMultiTexCoordP1ui"),
  }
}
static glMultiTexCoordP1ui_p: GlFnCell<glMultiTexCoordP1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiTexCoordP1ui_is_loaded() -> bool {
  unsafe { *glMultiTexCoordP1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiTexCoordP1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiTexCoordP1ui_t>>(gl_ptr_filter(f(b"glMultiTexCoordP1ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP1ui_reset_ptr() {
  *glMultiTexCoordP1ui_p.0.get() = None;
}
/// [glMultiTexCoordP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP1uiv.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiTexCoordP1uiv(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiTexCoordP1uiv_p.0.get() } {
    Some(glMultiTexCoordP1uiv_inner) => glMultiTexCoordP1uiv_inner(texture, type_, coords),
    None => gl_not_loaded("glMultiTexCoordP1uiv"),
  }
}
static glMultiTexCoordP1uiv_p: GlFnCell<glMultiTexCoordP1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiTexCoordP1uiv_is_loaded() -> bool {
  unsafe { *glMultiTexCoordP1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiTexCoordP1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiTexCoordP1uiv_t>>(gl_ptr_filter(f(b"glMultiTexCoordP1uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP1uiv_reset_ptr() {
  *glMultiTexCoordP1uiv_p.0.get() = None;
}
/// [glMultiTexCoordP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP2ui.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiTexCoordP2ui(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiTexCoordP2ui_p.0.get() } {
    Some(glMultiTexCoordP2ui_inner) => glMultiTexCoordP2ui_inner(texture, type_, coords),
    None => gl_not_loaded("glMultiTexCoordP2ui"),
  }
}
static glMultiTexCoordP2ui_p: GlFnCell<glMultiTexCoordP2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiTexCoordP2ui_is_loaded() -> bool {
  unsafe { *glMultiTexCoordP2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiTexCoordP2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiTexCoordP2ui_t>>(gl_ptr_filter(f(b"glMultiTexCoordP2ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP2ui_reset_ptr() {
  *glMultiTexCoordP2ui_p.0.get() = None;
}
/// [glMultiTexCoordP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP2uiv.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiTexCoordP2uiv(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiTexCoordP2uiv_p.0.get() } {
    Some(glMultiTexCoordP2uiv_inner) => glMultiTexCoordP2uiv_inner(texture, type_, coords),
    None => gl_not_loaded("glMultiTexCoordP2uiv"),
  }
}
static glMultiTexCoordP2uiv_p: GlFnCell<glMultiTexCoordP2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiTexCoordP2uiv_is_loaded() -> bool {
  unsafe { *glMultiTexCoordP2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiTexCoordP2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiTexCoordP2uiv_t>>(gl_ptr_filter(f(b"glMultiTexCoordP2uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP2uiv_reset_ptr() {
  *glMultiTexCoordP2uiv_p.0.get() = None;
}
/// [glMultiTexCoordP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP3ui.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiTexCoordP3ui(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiTexCoordP3ui_p.0.get() } {
    Some(glMultiTexCoordP3ui_inner) => glMultiTexCoordP3ui_inner(texture, type_, coords),
    None => gl_not_loaded("glMultiTexCoordP3ui"),
  }
}
static glMultiTexCoordP3ui_p: GlFnCell<glMultiTexCoordP3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiTexCoordP3ui_is_loaded() -> bool {
  unsafe { *glMultiTexCoordP3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiTexCoordP3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiTexCoordP3ui_t>>(gl_ptr_filter(f(b"glMultiTexCoordP3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP3ui_reset_ptr() {
  *glMultiTexCoordP3ui_p.0.get() = None;
}
/// [glMultiTexCoordP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP3uiv.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiTexCoordP3uiv(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiTexCoordP3uiv_p.0.get() } {
    Some(glMultiTexCoordP3uiv_inner) => glMultiTexCoordP3uiv_inner(texture, type_, coords),
    None => gl_not_loaded("glMultiTexCoordP3uiv"),
  }
}
static glMultiTexCoordP3uiv_p: GlFnCell<glMultiTexCoordP3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiTexCoordP3uiv_is_loaded() -> bool {
  unsafe { *glMultiTexCoordP3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiTexCoordP3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiTexCoordP3uiv_t>>(gl_ptr_filter(f(b"glMultiTexCoordP3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP3uiv_reset_ptr() {
  *glMultiTexCoordP3uiv_p.0.get() = None;
}
/// [glMultiTexCoordP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP4ui.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiTexCoordP4ui(texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiTexCoordP4ui_p.0.get() } {
    Some(glMultiTexCoordP4ui_inner) => glMultiTexCoordP4ui_inner(texture, type_, coords),
    None => gl_not_loaded("glMultiTexCoordP4ui"),
  }
}
static glMultiTexCoordP4ui_p: GlFnCell<glMultiTexCoordP4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiTexCoordP4ui_is_loaded() -> bool {
  unsafe { *glMultiTexCoordP4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiTexCoordP4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiTexCoordP4ui_t>>(gl_ptr_filter(f(b"glMultiTexCoordP4ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP4ui_reset_ptr() {
  *glMultiTexCoordP4ui_p.0.get() = None;
}
/// [glMultiTexCoordP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP4uiv.xhtml)
/// * `texture` group: TextureUnit
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiTexCoordP4uiv(texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiTexCoordP4uiv_p.0.get() } {
    Some(glMultiTexCoordP4uiv_inner) => glMultiTexCoordP4uiv_inner(texture, type_, coords),
    None => gl_not_loaded("glMultiTexCoordP4uiv"),
  }
}
static glMultiTexCoordP4uiv_p: GlFnCell<glMultiTexCoordP4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiTexCoordP4uiv_is_loaded() -> bool {
  unsafe { *glMultiTexCoordP4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiTexCoordP4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiTexCoordP4uiv_t>>(gl_ptr_filter(f(b"glMultiTexCoordP4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glMultiTexCoordP4uiv_reset_ptr() {
  *glMultiTexCoordP4uiv_p.0.get() = None;
}
/// [glNamedBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferData.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `usage` group: VertexBufferObjectUsage
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedBufferData(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: VertexBufferObjectUsage) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedBufferData_p.0.get() } {
    Some(glNamedBufferData_inner) => glNamedBufferData_inner(buffer, size, data, usage),
    None => gl_not_loaded("glNamedBufferData"),
  }
}
static glNamedBufferData_p: GlFnCell<glNamedBufferData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedBufferData_is_loaded() -> bool {
  unsafe { *glNamedBufferData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedBufferData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedBufferData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedBufferData_t>>(gl_ptr_filter(f(b"glNamedBufferData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedBufferData_reset_ptr() {
  *glNamedBufferData_p.0.get() = None;
}
/// [glNamedBufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferStorage.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: size
/// * `flags` group: BufferStorageMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedBufferStorage(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedBufferStorage_p.0.get() } {
    Some(glNamedBufferStorage_inner) => glNamedBufferStorage_inner(buffer, size, data, flags),
    None => gl_not_loaded("glNamedBufferStorage"),
  }
}
static glNamedBufferStorage_p: GlFnCell<glNamedBufferStorage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedBufferStorage_is_loaded() -> bool {
  unsafe { *glNamedBufferStorage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedBufferStorage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedBufferStorage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedBufferStorage_t>>(gl_ptr_filter(f(b"glNamedBufferStorage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedBufferStorage_reset_ptr() {
  *glNamedBufferStorage_p.0.get() = None;
}
/// [glNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferSubData.xhtml)
/// * `buffer` class: buffer
/// * `size` group: BufferSize
/// * `data` len: COMPSIZE(size)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedBufferSubData_p.0.get() } {
    Some(glNamedBufferSubData_inner) => glNamedBufferSubData_inner(buffer, offset, size, data),
    None => gl_not_loaded("glNamedBufferSubData"),
  }
}
static glNamedBufferSubData_p: GlFnCell<glNamedBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedBufferSubData_is_loaded() -> bool {
  unsafe { *glNamedBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedBufferSubData_t>>(gl_ptr_filter(f(b"glNamedBufferSubData\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedBufferSubData_reset_ptr() {
  *glNamedBufferSubData_p.0.get() = None;
}
/// [glNamedFramebufferDrawBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferDrawBuffer.xhtml)
/// * `framebuffer` class: framebuffer
/// * `buf` group: ColorBuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedFramebufferDrawBuffer(framebuffer: GLuint, buf: ColorBuffer) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedFramebufferDrawBuffer_p.0.get() } {
    Some(glNamedFramebufferDrawBuffer_inner) => glNamedFramebufferDrawBuffer_inner(framebuffer, buf),
    None => gl_not_loaded("glNamedFramebufferDrawBuffer"),
  }
}
static glNamedFramebufferDrawBuffer_p: GlFnCell<glNamedFramebufferDrawBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedFramebufferDrawBuffer_is_loaded() -> bool {
  unsafe { *glNamedFramebufferDrawBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferDrawBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedFramebufferDrawBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedFramebufferDrawBuffer_t>>(gl_ptr_filter(f(b"glNamedFramebufferDrawBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferDrawBuffer_reset_ptr() {
  *glNamedFramebufferDrawBuffer_p.0.get() = None;
}
/// [glNamedFramebufferDrawBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferDrawBuffers.xhtml)
/// * `framebuffer` class: framebuffer
/// * `bufs` group: ColorBuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedFramebufferDrawBuffers(framebuffer: GLuint, n: GLsizei, bufs: *const ColorBuffer) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedFramebufferDrawBuffers_p.0.get() } {
    Some(glNamedFramebufferDrawBuffers_inner) => glNamedFramebufferDrawBuffers_inner(framebuffer, n, bufs),
    None => gl_not_loaded("glNamedFramebufferDrawBuffers"),
  }
}
static glNamedFramebufferDrawBuffers_p: GlFnCell<glNamedFramebufferDrawBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedFramebufferDrawBuffers_is_loaded() -> bool {
  unsafe { *glNamedFramebufferDrawBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferDrawBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedFramebufferDrawBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedFramebufferDrawBuffers_t>>(gl_ptr_filter(f(b"glNamedFramebufferDrawBuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferDrawBuffers_reset_ptr() {
  *glNamedFramebufferDrawBuffers_p.0.get() = None;
}
/// [glNamedFramebufferParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferParameteri.xhtml)
/// * `framebuffer` class: framebuffer
/// * `pname` group: FramebufferParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedFramebufferParameteri(framebuffer: GLuint, pname: FramebufferParameterName, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedFramebufferParameteri_p.0.get() } {
    Some(glNamedFramebufferParameteri_inner) => glNamedFramebufferParameteri_inner(framebuffer, pname, param),
    None => gl_not_loaded("glNamedFramebufferParameteri"),
  }
}
static glNamedFramebufferParameteri_p: GlFnCell<glNamedFramebufferParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedFramebufferParameteri_is_loaded() -> bool {
  unsafe { *glNamedFramebufferParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedFramebufferParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedFramebufferParameteri_t>>(gl_ptr_filter(f(b"glNamedFramebufferParameteri\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferParameteri_reset_ptr() {
  *glNamedFramebufferParameteri_p.0.get() = None;
}
/// [glNamedFramebufferReadBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferReadBuffer.xhtml)
/// * `framebuffer` class: framebuffer
/// * `src` group: ColorBuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedFramebufferReadBuffer(framebuffer: GLuint, src: ColorBuffer) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedFramebufferReadBuffer_p.0.get() } {
    Some(glNamedFramebufferReadBuffer_inner) => glNamedFramebufferReadBuffer_inner(framebuffer, src),
    None => gl_not_loaded("glNamedFramebufferReadBuffer"),
  }
}
static glNamedFramebufferReadBuffer_p: GlFnCell<glNamedFramebufferReadBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedFramebufferReadBuffer_is_loaded() -> bool {
  unsafe { *glNamedFramebufferReadBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferReadBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedFramebufferReadBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedFramebufferReadBuffer_t>>(gl_ptr_filter(f(b"glNamedFramebufferReadBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferReadBuffer_reset_ptr() {
  *glNamedFramebufferReadBuffer_p.0.get() = None;
}
/// [glNamedFramebufferRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferRenderbuffer.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedFramebufferRenderbuffer(framebuffer: GLuint, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedFramebufferRenderbuffer_p.0.get() } {
    Some(glNamedFramebufferRenderbuffer_inner) => glNamedFramebufferRenderbuffer_inner(framebuffer, attachment, renderbuffertarget, renderbuffer),
    None => gl_not_loaded("glNamedFramebufferRenderbuffer"),
  }
}
static glNamedFramebufferRenderbuffer_p: GlFnCell<glNamedFramebufferRenderbuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedFramebufferRenderbuffer_is_loaded() -> bool {
  unsafe { *glNamedFramebufferRenderbuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferRenderbuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedFramebufferRenderbuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedFramebufferRenderbuffer_t>>(gl_ptr_filter(f(b"glNamedFramebufferRenderbuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferRenderbuffer_reset_ptr() {
  *glNamedFramebufferRenderbuffer_p.0.get() = None;
}
/// [glNamedFramebufferTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTexture.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedFramebufferTexture(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedFramebufferTexture_p.0.get() } {
    Some(glNamedFramebufferTexture_inner) => glNamedFramebufferTexture_inner(framebuffer, attachment, texture, level),
    None => gl_not_loaded("glNamedFramebufferTexture"),
  }
}
static glNamedFramebufferTexture_p: GlFnCell<glNamedFramebufferTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedFramebufferTexture_is_loaded() -> bool {
  unsafe { *glNamedFramebufferTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedFramebufferTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedFramebufferTexture_t>>(gl_ptr_filter(f(b"glNamedFramebufferTexture\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferTexture_reset_ptr() {
  *glNamedFramebufferTexture_p.0.get() = None;
}
/// [glNamedFramebufferTextureLayer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTextureLayer.xhtml)
/// * `framebuffer` class: framebuffer
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedFramebufferTextureLayer(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedFramebufferTextureLayer_p.0.get() } {
    Some(glNamedFramebufferTextureLayer_inner) => glNamedFramebufferTextureLayer_inner(framebuffer, attachment, texture, level, layer),
    None => gl_not_loaded("glNamedFramebufferTextureLayer"),
  }
}
static glNamedFramebufferTextureLayer_p: GlFnCell<glNamedFramebufferTextureLayer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedFramebufferTextureLayer_is_loaded() -> bool {
  unsafe { *glNamedFramebufferTextureLayer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferTextureLayer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedFramebufferTextureLayer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedFramebufferTextureLayer_t>>(gl_ptr_filter(f(b"glNamedFramebufferTextureLayer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedFramebufferTextureLayer_reset_ptr() {
  *glNamedFramebufferTextureLayer_p.0.get() = None;
}
/// [glNamedRenderbufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorage.xhtml)
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedRenderbufferStorage(renderbuffer: GLuint, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedRenderbufferStorage_p.0.get() } {
    Some(glNamedRenderbufferStorage_inner) => glNamedRenderbufferStorage_inner(renderbuffer, internalformat, width, height),
    None => gl_not_loaded("glNamedRenderbufferStorage"),
  }
}
static glNamedRenderbufferStorage_p: GlFnCell<glNamedRenderbufferStorage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedRenderbufferStorage_is_loaded() -> bool {
  unsafe { *glNamedRenderbufferStorage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedRenderbufferStorage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedRenderbufferStorage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedRenderbufferStorage_t>>(gl_ptr_filter(f(b"glNamedRenderbufferStorage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedRenderbufferStorage_reset_ptr() {
  *glNamedRenderbufferStorage_p.0.get() = None;
}
/// [glNamedRenderbufferStorageMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorageMultisample.xhtml)
/// * `renderbuffer` class: renderbuffer
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNamedRenderbufferStorageMultisample(renderbuffer: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glNamedRenderbufferStorageMultisample_p.0.get() } {
    Some(glNamedRenderbufferStorageMultisample_inner) => glNamedRenderbufferStorageMultisample_inner(renderbuffer, samples, internalformat, width, height),
    None => gl_not_loaded("glNamedRenderbufferStorageMultisample"),
  }
}
static glNamedRenderbufferStorageMultisample_p: GlFnCell<glNamedRenderbufferStorageMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNamedRenderbufferStorageMultisample_is_loaded() -> bool {
  unsafe { *glNamedRenderbufferStorageMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNamedRenderbufferStorageMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNamedRenderbufferStorageMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNamedRenderbufferStorageMultisample_t>>(gl_ptr_filter(f(b"glNamedRenderbufferStorageMultisample\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNamedRenderbufferStorageMultisample_reset_ptr() {
  *glNamedRenderbufferStorageMultisample_p.0.get() = None;
}
/// [glNormalP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalP3ui.xhtml)
/// * `type` group: NormalPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNormalP3ui(type_: NormalPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glNormalP3ui_p.0.get() } {
    Some(glNormalP3ui_inner) => glNormalP3ui_inner(type_, coords),
    None => gl_not_loaded("glNormalP3ui"),
  }
}
static glNormalP3ui_p: GlFnCell<glNormalP3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNormalP3ui_is_loaded() -> bool {
  unsafe { *glNormalP3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNormalP3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNormalP3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNormalP3ui_t>>(gl_ptr_filter(f(b"glNormalP3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNormalP3ui_reset_ptr() {
  *glNormalP3ui_p.0.get() = None;
}
/// [glNormalP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalP3uiv.xhtml)
/// * `type` group: NormalPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glNormalP3uiv(type_: NormalPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glNormalP3uiv_p.0.get() } {
    Some(glNormalP3uiv_inner) => glNormalP3uiv_inner(type_, coords),
    None => gl_not_loaded("glNormalP3uiv"),
  }
}
static glNormalP3uiv_p: GlFnCell<glNormalP3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glNormalP3uiv_is_loaded() -> bool {
  unsafe { *glNormalP3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glNormalP3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glNormalP3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glNormalP3uiv_t>>(gl_ptr_filter(f(b"glNormalP3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glNormalP3uiv_reset_ptr() {
  *glNormalP3uiv_p.0.get() = None;
}
/// [glObjectLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectLabel.xhtml)
/// * `identifier` group: ObjectIdentifier
/// * `label` len: COMPSIZE(label,length)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glObjectLabel(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glObjectLabel_p.0.get() } {
    Some(glObjectLabel_inner) => glObjectLabel_inner(identifier, name, length, label),
    None => gl_not_loaded("glObjectLabel"),
  }
}
static glObjectLabel_p: GlFnCell<glObjectLabel_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glObjectLabel_is_loaded() -> bool {
  unsafe { *glObjectLabel_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glObjectLabel_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glObjectLabel_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glObjectLabel_t>>(gl_ptr_filter(f(b"glObjectLabel\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glObjectLabel_reset_ptr() {
  *glObjectLabel_p.0.get() = None;
}
/// [glObjectLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectLabelKHR.xhtml)
/// * `identifier` group: ObjectIdentifier
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glObjectLabelKHR(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glObjectLabelKHR_p.0.get() } {
    Some(glObjectLabelKHR_inner) => glObjectLabelKHR_inner(identifier, name, length, label),
    None => gl_not_loaded("glObjectLabelKHR"),
  }
}
static glObjectLabelKHR_p: GlFnCell<glObjectLabelKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glObjectLabelKHR_is_loaded() -> bool {
  unsafe { *glObjectLabelKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glObjectLabelKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glObjectLabelKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glObjectLabelKHR_t>>(gl_ptr_filter(f(b"glObjectLabelKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glObjectLabelKHR_reset_ptr() {
  *glObjectLabelKHR_p.0.get() = None;
}
/// [glObjectPtrLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectPtrLabel.xhtml)
/// * `label` len: COMPSIZE(label,length)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glObjectPtrLabel(ptr: *const void, length: GLsizei, label: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glObjectPtrLabel_p.0.get() } {
    Some(glObjectPtrLabel_inner) => glObjectPtrLabel_inner(ptr, length, label),
    None => gl_not_loaded("glObjectPtrLabel"),
  }
}
static glObjectPtrLabel_p: GlFnCell<glObjectPtrLabel_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glObjectPtrLabel_is_loaded() -> bool {
  unsafe { *glObjectPtrLabel_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glObjectPtrLabel_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glObjectPtrLabel_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glObjectPtrLabel_t>>(gl_ptr_filter(f(b"glObjectPtrLabel\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glObjectPtrLabel_reset_ptr() {
  *glObjectPtrLabel_p.0.get() = None;
}
/// [glObjectPtrLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectPtrLabelKHR.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glObjectPtrLabelKHR(ptr: *const void, length: GLsizei, label: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glObjectPtrLabelKHR_p.0.get() } {
    Some(glObjectPtrLabelKHR_inner) => glObjectPtrLabelKHR_inner(ptr, length, label),
    None => gl_not_loaded("glObjectPtrLabelKHR"),
  }
}
static glObjectPtrLabelKHR_p: GlFnCell<glObjectPtrLabelKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glObjectPtrLabelKHR_is_loaded() -> bool {
  unsafe { *glObjectPtrLabelKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glObjectPtrLabelKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glObjectPtrLabelKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glObjectPtrLabelKHR_t>>(gl_ptr_filter(f(b"glObjectPtrLabelKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glObjectPtrLabelKHR_reset_ptr() {
  *glObjectPtrLabelKHR_p.0.get() = None;
}
/// [glPatchParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPatchParameterfv.xhtml)
/// * `pname` group: PatchParameterName
/// * `values` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPatchParameterfv(pname: PatchParameterName, values: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPatchParameterfv_p.0.get() } {
    Some(glPatchParameterfv_inner) => glPatchParameterfv_inner(pname, values),
    None => gl_not_loaded("glPatchParameterfv"),
  }
}
static glPatchParameterfv_p: GlFnCell<glPatchParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPatchParameterfv_is_loaded() -> bool {
  unsafe { *glPatchParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPatchParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPatchParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPatchParameterfv_t>>(gl_ptr_filter(f(b"glPatchParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPatchParameterfv_reset_ptr() {
  *glPatchParameterfv_p.0.get() = None;
}
/// [glPatchParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPatchParameteri.xhtml)
/// * `pname` group: PatchParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPatchParameteri(pname: PatchParameterName, value: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPatchParameteri_p.0.get() } {
    Some(glPatchParameteri_inner) => glPatchParameteri_inner(pname, value),
    None => gl_not_loaded("glPatchParameteri"),
  }
}
static glPatchParameteri_p: GlFnCell<glPatchParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPatchParameteri_is_loaded() -> bool {
  unsafe { *glPatchParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPatchParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPatchParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPatchParameteri_t>>(gl_ptr_filter(f(b"glPatchParameteri\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPatchParameteri_reset_ptr() {
  *glPatchParameteri_p.0.get() = None;
}
/// [glPauseTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPauseTransformFeedback.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPauseTransformFeedback() {
  #[allow(unused_unsafe)]
  match unsafe { *glPauseTransformFeedback_p.0.get() } {
    Some(glPauseTransformFeedback_inner) => glPauseTransformFeedback_inner(),
    None => gl_not_loaded("glPauseTransformFeedback"),
  }
}
static glPauseTransformFeedback_p: GlFnCell<glPauseTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPauseTransformFeedback_is_loaded() -> bool {
  unsafe { *glPauseTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPauseTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPauseTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPauseTransformFeedback_t>>(gl_ptr_filter(f(b"glPauseTransformFeedback\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPauseTransformFeedback_reset_ptr() {
  *glPauseTransformFeedback_p.0.get() = None;
}
/// [glPixelStoref](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelStoref.xhtml)
/// * `pname` group: PixelStoreParameter
/// * `param` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPixelStoref(pname: PixelStoreParameter, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPixelStoref_p.0.get() } {
    Some(glPixelStoref_inner) => glPixelStoref_inner(pname, param),
    None => gl_not_loaded("glPixelStoref"),
  }
}
static glPixelStoref_p: GlFnCell<glPixelStoref_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPixelStoref_is_loaded() -> bool {
  unsafe { *glPixelStoref_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPixelStoref_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPixelStoref_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPixelStoref_t>>(gl_ptr_filter(f(b"glPixelStoref\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPixelStoref_reset_ptr() {
  *glPixelStoref_p.0.get() = None;
}
/// [glPixelStorei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelStorei.xhtml)
/// * `pname` group: PixelStoreParameter
/// * `param` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPixelStorei(pname: PixelStoreParameter, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPixelStorei_p.0.get() } {
    Some(glPixelStorei_inner) => glPixelStorei_inner(pname, param),
    None => gl_not_loaded("glPixelStorei"),
  }
}
static glPixelStorei_p: GlFnCell<glPixelStorei_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPixelStorei_is_loaded() -> bool {
  unsafe { *glPixelStorei_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPixelStorei_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPixelStorei_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPixelStorei_t>>(gl_ptr_filter(f(b"glPixelStorei\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPixelStorei_reset_ptr() {
  *glPixelStorei_p.0.get() = None;
}
/// [glPointParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterf.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointParameterf(pname: PointParameterNameARB, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointParameterf_p.0.get() } {
    Some(glPointParameterf_inner) => glPointParameterf_inner(pname, param),
    None => gl_not_loaded("glPointParameterf"),
  }
}
static glPointParameterf_p: GlFnCell<glPointParameterf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointParameterf_is_loaded() -> bool {
  unsafe { *glPointParameterf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointParameterf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointParameterf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointParameterf_t>>(gl_ptr_filter(f(b"glPointParameterf\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPointParameterf_reset_ptr() {
  *glPointParameterf_p.0.get() = None;
}
/// [glPointParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfv.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointParameterfv(pname: PointParameterNameARB, params: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointParameterfv_p.0.get() } {
    Some(glPointParameterfv_inner) => glPointParameterfv_inner(pname, params),
    None => gl_not_loaded("glPointParameterfv"),
  }
}
static glPointParameterfv_p: GlFnCell<glPointParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointParameterfv_is_loaded() -> bool {
  unsafe { *glPointParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointParameterfv_t>>(gl_ptr_filter(f(b"glPointParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPointParameterfv_reset_ptr() {
  *glPointParameterfv_p.0.get() = None;
}
/// [glPointParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameteri.xhtml)
/// * `pname` group: PointParameterNameARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointParameteri(pname: PointParameterNameARB, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointParameteri_p.0.get() } {
    Some(glPointParameteri_inner) => glPointParameteri_inner(pname, param),
    None => gl_not_loaded("glPointParameteri"),
  }
}
static glPointParameteri_p: GlFnCell<glPointParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointParameteri_is_loaded() -> bool {
  unsafe { *glPointParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointParameteri_t>>(gl_ptr_filter(f(b"glPointParameteri\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPointParameteri_reset_ptr() {
  *glPointParameteri_p.0.get() = None;
}
/// [glPointParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameteriv.xhtml)
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointParameteriv(pname: PointParameterNameARB, params: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointParameteriv_p.0.get() } {
    Some(glPointParameteriv_inner) => glPointParameteriv_inner(pname, params),
    None => gl_not_loaded("glPointParameteriv"),
  }
}
static glPointParameteriv_p: GlFnCell<glPointParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointParameteriv_is_loaded() -> bool {
  unsafe { *glPointParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointParameteriv_t>>(gl_ptr_filter(f(b"glPointParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPointParameteriv_reset_ptr() {
  *glPointParameteriv_p.0.get() = None;
}
/// [glPointSize](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSize.xhtml)
/// * `size` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointSize(size: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointSize_p.0.get() } {
    Some(glPointSize_inner) => glPointSize_inner(size),
    None => gl_not_loaded("glPointSize"),
  }
}
static glPointSize_p: GlFnCell<glPointSize_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointSize_is_loaded() -> bool {
  unsafe { *glPointSize_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointSize_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointSize_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointSize_t>>(gl_ptr_filter(f(b"glPointSize\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPointSize_reset_ptr() {
  *glPointSize_p.0.get() = None;
}
/// [glPolygonMode](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonMode.xhtml)
/// * `face` group: MaterialFace
/// * `mode` group: PolygonMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPolygonMode(face: MaterialFace, mode: PolygonMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glPolygonMode_p.0.get() } {
    Some(glPolygonMode_inner) => glPolygonMode_inner(face, mode),
    None => gl_not_loaded("glPolygonMode"),
  }
}
static glPolygonMode_p: GlFnCell<glPolygonMode_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPolygonMode_is_loaded() -> bool {
  unsafe { *glPolygonMode_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPolygonMode_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPolygonMode_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPolygonMode_t>>(gl_ptr_filter(f(b"glPolygonMode\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPolygonMode_reset_ptr() {
  *glPolygonMode_p.0.get() = None;
}
/// [glPolygonOffset](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffset.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPolygonOffset(factor: GLfloat, units: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPolygonOffset_p.0.get() } {
    Some(glPolygonOffset_inner) => glPolygonOffset_inner(factor, units),
    None => gl_not_loaded("glPolygonOffset"),
  }
}
static glPolygonOffset_p: GlFnCell<glPolygonOffset_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPolygonOffset_is_loaded() -> bool {
  unsafe { *glPolygonOffset_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPolygonOffset_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPolygonOffset_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPolygonOffset_t>>(gl_ptr_filter(f(b"glPolygonOffset\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPolygonOffset_reset_ptr() {
  *glPolygonOffset_p.0.get() = None;
}
/// [glPolygonOffsetClamp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffsetClamp.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPolygonOffsetClamp(factor: GLfloat, units: GLfloat, clamp: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPolygonOffsetClamp_p.0.get() } {
    Some(glPolygonOffsetClamp_inner) => glPolygonOffsetClamp_inner(factor, units, clamp),
    None => gl_not_loaded("glPolygonOffsetClamp"),
  }
}
static glPolygonOffsetClamp_p: GlFnCell<glPolygonOffsetClamp_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPolygonOffsetClamp_is_loaded() -> bool {
  unsafe { *glPolygonOffsetClamp_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPolygonOffsetClamp_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPolygonOffsetClamp_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPolygonOffsetClamp_t>>(gl_ptr_filter(f(b"glPolygonOffsetClamp\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPolygonOffsetClamp_reset_ptr() {
  *glPolygonOffsetClamp_p.0.get() = None;
}
/// [glPopDebugGroup](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopDebugGroup.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPopDebugGroup() {
  #[allow(unused_unsafe)]
  match unsafe { *glPopDebugGroup_p.0.get() } {
    Some(glPopDebugGroup_inner) => glPopDebugGroup_inner(),
    None => gl_not_loaded("glPopDebugGroup"),
  }
}
static glPopDebugGroup_p: GlFnCell<glPopDebugGroup_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPopDebugGroup_is_loaded() -> bool {
  unsafe { *glPopDebugGroup_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPopDebugGroup_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPopDebugGroup_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPopDebugGroup_t>>(gl_ptr_filter(f(b"glPopDebugGroup\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPopDebugGroup_reset_ptr() {
  *glPopDebugGroup_p.0.get() = None;
}
/// [glPopDebugGroupKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopDebugGroupKHR.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPopDebugGroupKHR() {
  #[allow(unused_unsafe)]
  match unsafe { *glPopDebugGroupKHR_p.0.get() } {
    Some(glPopDebugGroupKHR_inner) => glPopDebugGroupKHR_inner(),
    None => gl_not_loaded("glPopDebugGroupKHR"),
  }
}
static glPopDebugGroupKHR_p: GlFnCell<glPopDebugGroupKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPopDebugGroupKHR_is_loaded() -> bool {
  unsafe { *glPopDebugGroupKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPopDebugGroupKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPopDebugGroupKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPopDebugGroupKHR_t>>(gl_ptr_filter(f(b"glPopDebugGroupKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPopDebugGroupKHR_reset_ptr() {
  *glPopDebugGroupKHR_p.0.get() = None;
}
/// [glPrimitiveRestartIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveRestartIndex.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPrimitiveRestartIndex(index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPrimitiveRestartIndex_p.0.get() } {
    Some(glPrimitiveRestartIndex_inner) => glPrimitiveRestartIndex_inner(index),
    None => gl_not_loaded("glPrimitiveRestartIndex"),
  }
}
static glPrimitiveRestartIndex_p: GlFnCell<glPrimitiveRestartIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPrimitiveRestartIndex_is_loaded() -> bool {
  unsafe { *glPrimitiveRestartIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPrimitiveRestartIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPrimitiveRestartIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPrimitiveRestartIndex_t>>(gl_ptr_filter(f(b"glPrimitiveRestartIndex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPrimitiveRestartIndex_reset_ptr() {
  *glPrimitiveRestartIndex_p.0.get() = None;
}
/// [glProgramBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramBinary.xhtml)
/// * `program` class: program
/// * `binary` len: length
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramBinary_p.0.get() } {
    Some(glProgramBinary_inner) => glProgramBinary_inner(program, binaryFormat, binary, length),
    None => gl_not_loaded("glProgramBinary"),
  }
}
static glProgramBinary_p: GlFnCell<glProgramBinary_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramBinary_is_loaded() -> bool {
  unsafe { *glProgramBinary_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramBinary_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramBinary_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramBinary_t>>(gl_ptr_filter(f(b"glProgramBinary\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramBinary_reset_ptr() {
  *glProgramBinary_p.0.get() = None;
}
/// [glProgramParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameteri.xhtml)
/// * `program` class: program
/// * `pname` group: ProgramParameterPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramParameteri(program: GLuint, pname: ProgramParameterPName, value: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramParameteri_p.0.get() } {
    Some(glProgramParameteri_inner) => glProgramParameteri_inner(program, pname, value),
    None => gl_not_loaded("glProgramParameteri"),
  }
}
static glProgramParameteri_p: GlFnCell<glProgramParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramParameteri_is_loaded() -> bool {
  unsafe { *glProgramParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramParameteri_t>>(gl_ptr_filter(f(b"glProgramParameteri\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramParameteri_reset_ptr() {
  *glProgramParameteri_p.0.get() = None;
}
/// [glProgramUniform1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1d.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform1d_p.0.get() } {
    Some(glProgramUniform1d_inner) => glProgramUniform1d_inner(program, location, v0),
    None => gl_not_loaded("glProgramUniform1d"),
  }
}
static glProgramUniform1d_p: GlFnCell<glProgramUniform1d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform1d_is_loaded() -> bool {
  unsafe { *glProgramUniform1d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform1d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform1d_t>>(gl_ptr_filter(f(b"glProgramUniform1d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1d_reset_ptr() {
  *glProgramUniform1d_p.0.get() = None;
}
/// [glProgramUniform1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1dv.xhtml)
/// * `program` class: program
/// * `value` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform1dv_p.0.get() } {
    Some(glProgramUniform1dv_inner) => glProgramUniform1dv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform1dv"),
  }
}
static glProgramUniform1dv_p: GlFnCell<glProgramUniform1dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform1dv_is_loaded() -> bool {
  unsafe { *glProgramUniform1dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform1dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform1dv_t>>(gl_ptr_filter(f(b"glProgramUniform1dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1dv_reset_ptr() {
  *glProgramUniform1dv_p.0.get() = None;
}
/// [glProgramUniform1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1f.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform1f_p.0.get() } {
    Some(glProgramUniform1f_inner) => glProgramUniform1f_inner(program, location, v0),
    None => gl_not_loaded("glProgramUniform1f"),
  }
}
static glProgramUniform1f_p: GlFnCell<glProgramUniform1f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform1f_is_loaded() -> bool {
  unsafe { *glProgramUniform1f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform1f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform1f_t>>(gl_ptr_filter(f(b"glProgramUniform1f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1f_reset_ptr() {
  *glProgramUniform1f_p.0.get() = None;
}
/// [glProgramUniform1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1fv.xhtml)
/// * `program` class: program
/// * `value` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform1fv_p.0.get() } {
    Some(glProgramUniform1fv_inner) => glProgramUniform1fv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform1fv"),
  }
}
static glProgramUniform1fv_p: GlFnCell<glProgramUniform1fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform1fv_is_loaded() -> bool {
  unsafe { *glProgramUniform1fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform1fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform1fv_t>>(gl_ptr_filter(f(b"glProgramUniform1fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1fv_reset_ptr() {
  *glProgramUniform1fv_p.0.get() = None;
}
/// [glProgramUniform1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1i.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform1i(program: GLuint, location: GLint, v0: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform1i_p.0.get() } {
    Some(glProgramUniform1i_inner) => glProgramUniform1i_inner(program, location, v0),
    None => gl_not_loaded("glProgramUniform1i"),
  }
}
static glProgramUniform1i_p: GlFnCell<glProgramUniform1i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform1i_is_loaded() -> bool {
  unsafe { *glProgramUniform1i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform1i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform1i_t>>(gl_ptr_filter(f(b"glProgramUniform1i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1i_reset_ptr() {
  *glProgramUniform1i_p.0.get() = None;
}
/// [glProgramUniform1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1iv.xhtml)
/// * `program` class: program
/// * `value` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform1iv_p.0.get() } {
    Some(glProgramUniform1iv_inner) => glProgramUniform1iv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform1iv"),
  }
}
static glProgramUniform1iv_p: GlFnCell<glProgramUniform1iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform1iv_is_loaded() -> bool {
  unsafe { *glProgramUniform1iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform1iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform1iv_t>>(gl_ptr_filter(f(b"glProgramUniform1iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1iv_reset_ptr() {
  *glProgramUniform1iv_p.0.get() = None;
}
/// [glProgramUniform1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1ui.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform1ui_p.0.get() } {
    Some(glProgramUniform1ui_inner) => glProgramUniform1ui_inner(program, location, v0),
    None => gl_not_loaded("glProgramUniform1ui"),
  }
}
static glProgramUniform1ui_p: GlFnCell<glProgramUniform1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform1ui_is_loaded() -> bool {
  unsafe { *glProgramUniform1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform1ui_t>>(gl_ptr_filter(f(b"glProgramUniform1ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1ui_reset_ptr() {
  *glProgramUniform1ui_p.0.get() = None;
}
/// [glProgramUniform1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1uiv.xhtml)
/// * `program` class: program
/// * `value` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform1uiv_p.0.get() } {
    Some(glProgramUniform1uiv_inner) => glProgramUniform1uiv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform1uiv"),
  }
}
static glProgramUniform1uiv_p: GlFnCell<glProgramUniform1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform1uiv_is_loaded() -> bool {
  unsafe { *glProgramUniform1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform1uiv_t>>(gl_ptr_filter(f(b"glProgramUniform1uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform1uiv_reset_ptr() {
  *glProgramUniform1uiv_p.0.get() = None;
}
/// [glProgramUniform2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2d.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform2d_p.0.get() } {
    Some(glProgramUniform2d_inner) => glProgramUniform2d_inner(program, location, v0, v1),
    None => gl_not_loaded("glProgramUniform2d"),
  }
}
static glProgramUniform2d_p: GlFnCell<glProgramUniform2d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform2d_is_loaded() -> bool {
  unsafe { *glProgramUniform2d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform2d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform2d_t>>(gl_ptr_filter(f(b"glProgramUniform2d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2d_reset_ptr() {
  *glProgramUniform2d_p.0.get() = None;
}
/// [glProgramUniform2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2dv.xhtml)
/// * `program` class: program
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform2dv_p.0.get() } {
    Some(glProgramUniform2dv_inner) => glProgramUniform2dv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform2dv"),
  }
}
static glProgramUniform2dv_p: GlFnCell<glProgramUniform2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform2dv_is_loaded() -> bool {
  unsafe { *glProgramUniform2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform2dv_t>>(gl_ptr_filter(f(b"glProgramUniform2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2dv_reset_ptr() {
  *glProgramUniform2dv_p.0.get() = None;
}
/// [glProgramUniform2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2f.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform2f_p.0.get() } {
    Some(glProgramUniform2f_inner) => glProgramUniform2f_inner(program, location, v0, v1),
    None => gl_not_loaded("glProgramUniform2f"),
  }
}
static glProgramUniform2f_p: GlFnCell<glProgramUniform2f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform2f_is_loaded() -> bool {
  unsafe { *glProgramUniform2f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform2f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform2f_t>>(gl_ptr_filter(f(b"glProgramUniform2f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2f_reset_ptr() {
  *glProgramUniform2f_p.0.get() = None;
}
/// [glProgramUniform2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2fv.xhtml)
/// * `program` class: program
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform2fv_p.0.get() } {
    Some(glProgramUniform2fv_inner) => glProgramUniform2fv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform2fv"),
  }
}
static glProgramUniform2fv_p: GlFnCell<glProgramUniform2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform2fv_is_loaded() -> bool {
  unsafe { *glProgramUniform2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform2fv_t>>(gl_ptr_filter(f(b"glProgramUniform2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2fv_reset_ptr() {
  *glProgramUniform2fv_p.0.get() = None;
}
/// [glProgramUniform2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2i.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform2i_p.0.get() } {
    Some(glProgramUniform2i_inner) => glProgramUniform2i_inner(program, location, v0, v1),
    None => gl_not_loaded("glProgramUniform2i"),
  }
}
static glProgramUniform2i_p: GlFnCell<glProgramUniform2i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform2i_is_loaded() -> bool {
  unsafe { *glProgramUniform2i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform2i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform2i_t>>(gl_ptr_filter(f(b"glProgramUniform2i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2i_reset_ptr() {
  *glProgramUniform2i_p.0.get() = None;
}
/// [glProgramUniform2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2iv.xhtml)
/// * `program` class: program
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform2iv_p.0.get() } {
    Some(glProgramUniform2iv_inner) => glProgramUniform2iv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform2iv"),
  }
}
static glProgramUniform2iv_p: GlFnCell<glProgramUniform2iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform2iv_is_loaded() -> bool {
  unsafe { *glProgramUniform2iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform2iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform2iv_t>>(gl_ptr_filter(f(b"glProgramUniform2iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2iv_reset_ptr() {
  *glProgramUniform2iv_p.0.get() = None;
}
/// [glProgramUniform2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2ui.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform2ui_p.0.get() } {
    Some(glProgramUniform2ui_inner) => glProgramUniform2ui_inner(program, location, v0, v1),
    None => gl_not_loaded("glProgramUniform2ui"),
  }
}
static glProgramUniform2ui_p: GlFnCell<glProgramUniform2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform2ui_is_loaded() -> bool {
  unsafe { *glProgramUniform2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform2ui_t>>(gl_ptr_filter(f(b"glProgramUniform2ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2ui_reset_ptr() {
  *glProgramUniform2ui_p.0.get() = None;
}
/// [glProgramUniform2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2uiv.xhtml)
/// * `program` class: program
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform2uiv_p.0.get() } {
    Some(glProgramUniform2uiv_inner) => glProgramUniform2uiv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform2uiv"),
  }
}
static glProgramUniform2uiv_p: GlFnCell<glProgramUniform2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform2uiv_is_loaded() -> bool {
  unsafe { *glProgramUniform2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform2uiv_t>>(gl_ptr_filter(f(b"glProgramUniform2uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform2uiv_reset_ptr() {
  *glProgramUniform2uiv_p.0.get() = None;
}
/// [glProgramUniform3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3d.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform3d_p.0.get() } {
    Some(glProgramUniform3d_inner) => glProgramUniform3d_inner(program, location, v0, v1, v2),
    None => gl_not_loaded("glProgramUniform3d"),
  }
}
static glProgramUniform3d_p: GlFnCell<glProgramUniform3d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform3d_is_loaded() -> bool {
  unsafe { *glProgramUniform3d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform3d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform3d_t>>(gl_ptr_filter(f(b"glProgramUniform3d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3d_reset_ptr() {
  *glProgramUniform3d_p.0.get() = None;
}
/// [glProgramUniform3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3dv.xhtml)
/// * `program` class: program
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform3dv_p.0.get() } {
    Some(glProgramUniform3dv_inner) => glProgramUniform3dv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform3dv"),
  }
}
static glProgramUniform3dv_p: GlFnCell<glProgramUniform3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform3dv_is_loaded() -> bool {
  unsafe { *glProgramUniform3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform3dv_t>>(gl_ptr_filter(f(b"glProgramUniform3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3dv_reset_ptr() {
  *glProgramUniform3dv_p.0.get() = None;
}
/// [glProgramUniform3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3f.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform3f_p.0.get() } {
    Some(glProgramUniform3f_inner) => glProgramUniform3f_inner(program, location, v0, v1, v2),
    None => gl_not_loaded("glProgramUniform3f"),
  }
}
static glProgramUniform3f_p: GlFnCell<glProgramUniform3f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform3f_is_loaded() -> bool {
  unsafe { *glProgramUniform3f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform3f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform3f_t>>(gl_ptr_filter(f(b"glProgramUniform3f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3f_reset_ptr() {
  *glProgramUniform3f_p.0.get() = None;
}
/// [glProgramUniform3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3fv.xhtml)
/// * `program` class: program
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform3fv_p.0.get() } {
    Some(glProgramUniform3fv_inner) => glProgramUniform3fv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform3fv"),
  }
}
static glProgramUniform3fv_p: GlFnCell<glProgramUniform3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform3fv_is_loaded() -> bool {
  unsafe { *glProgramUniform3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform3fv_t>>(gl_ptr_filter(f(b"glProgramUniform3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3fv_reset_ptr() {
  *glProgramUniform3fv_p.0.get() = None;
}
/// [glProgramUniform3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3i.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform3i_p.0.get() } {
    Some(glProgramUniform3i_inner) => glProgramUniform3i_inner(program, location, v0, v1, v2),
    None => gl_not_loaded("glProgramUniform3i"),
  }
}
static glProgramUniform3i_p: GlFnCell<glProgramUniform3i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform3i_is_loaded() -> bool {
  unsafe { *glProgramUniform3i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform3i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform3i_t>>(gl_ptr_filter(f(b"glProgramUniform3i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3i_reset_ptr() {
  *glProgramUniform3i_p.0.get() = None;
}
/// [glProgramUniform3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3iv.xhtml)
/// * `program` class: program
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform3iv_p.0.get() } {
    Some(glProgramUniform3iv_inner) => glProgramUniform3iv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform3iv"),
  }
}
static glProgramUniform3iv_p: GlFnCell<glProgramUniform3iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform3iv_is_loaded() -> bool {
  unsafe { *glProgramUniform3iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform3iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform3iv_t>>(gl_ptr_filter(f(b"glProgramUniform3iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3iv_reset_ptr() {
  *glProgramUniform3iv_p.0.get() = None;
}
/// [glProgramUniform3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3ui.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform3ui_p.0.get() } {
    Some(glProgramUniform3ui_inner) => glProgramUniform3ui_inner(program, location, v0, v1, v2),
    None => gl_not_loaded("glProgramUniform3ui"),
  }
}
static glProgramUniform3ui_p: GlFnCell<glProgramUniform3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform3ui_is_loaded() -> bool {
  unsafe { *glProgramUniform3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform3ui_t>>(gl_ptr_filter(f(b"glProgramUniform3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3ui_reset_ptr() {
  *glProgramUniform3ui_p.0.get() = None;
}
/// [glProgramUniform3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3uiv.xhtml)
/// * `program` class: program
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform3uiv_p.0.get() } {
    Some(glProgramUniform3uiv_inner) => glProgramUniform3uiv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform3uiv"),
  }
}
static glProgramUniform3uiv_p: GlFnCell<glProgramUniform3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform3uiv_is_loaded() -> bool {
  unsafe { *glProgramUniform3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform3uiv_t>>(gl_ptr_filter(f(b"glProgramUniform3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform3uiv_reset_ptr() {
  *glProgramUniform3uiv_p.0.get() = None;
}
/// [glProgramUniform4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4d.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform4d_p.0.get() } {
    Some(glProgramUniform4d_inner) => glProgramUniform4d_inner(program, location, v0, v1, v2, v3),
    None => gl_not_loaded("glProgramUniform4d"),
  }
}
static glProgramUniform4d_p: GlFnCell<glProgramUniform4d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform4d_is_loaded() -> bool {
  unsafe { *glProgramUniform4d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform4d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform4d_t>>(gl_ptr_filter(f(b"glProgramUniform4d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4d_reset_ptr() {
  *glProgramUniform4d_p.0.get() = None;
}
/// [glProgramUniform4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4dv.xhtml)
/// * `program` class: program
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform4dv_p.0.get() } {
    Some(glProgramUniform4dv_inner) => glProgramUniform4dv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform4dv"),
  }
}
static glProgramUniform4dv_p: GlFnCell<glProgramUniform4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform4dv_is_loaded() -> bool {
  unsafe { *glProgramUniform4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform4dv_t>>(gl_ptr_filter(f(b"glProgramUniform4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4dv_reset_ptr() {
  *glProgramUniform4dv_p.0.get() = None;
}
/// [glProgramUniform4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4f.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform4f_p.0.get() } {
    Some(glProgramUniform4f_inner) => glProgramUniform4f_inner(program, location, v0, v1, v2, v3),
    None => gl_not_loaded("glProgramUniform4f"),
  }
}
static glProgramUniform4f_p: GlFnCell<glProgramUniform4f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform4f_is_loaded() -> bool {
  unsafe { *glProgramUniform4f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform4f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform4f_t>>(gl_ptr_filter(f(b"glProgramUniform4f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4f_reset_ptr() {
  *glProgramUniform4f_p.0.get() = None;
}
/// [glProgramUniform4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4fv.xhtml)
/// * `program` class: program
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform4fv_p.0.get() } {
    Some(glProgramUniform4fv_inner) => glProgramUniform4fv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform4fv"),
  }
}
static glProgramUniform4fv_p: GlFnCell<glProgramUniform4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform4fv_is_loaded() -> bool {
  unsafe { *glProgramUniform4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform4fv_t>>(gl_ptr_filter(f(b"glProgramUniform4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4fv_reset_ptr() {
  *glProgramUniform4fv_p.0.get() = None;
}
/// [glProgramUniform4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4i.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform4i_p.0.get() } {
    Some(glProgramUniform4i_inner) => glProgramUniform4i_inner(program, location, v0, v1, v2, v3),
    None => gl_not_loaded("glProgramUniform4i"),
  }
}
static glProgramUniform4i_p: GlFnCell<glProgramUniform4i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform4i_is_loaded() -> bool {
  unsafe { *glProgramUniform4i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform4i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform4i_t>>(gl_ptr_filter(f(b"glProgramUniform4i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4i_reset_ptr() {
  *glProgramUniform4i_p.0.get() = None;
}
/// [glProgramUniform4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4iv.xhtml)
/// * `program` class: program
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform4iv_p.0.get() } {
    Some(glProgramUniform4iv_inner) => glProgramUniform4iv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform4iv"),
  }
}
static glProgramUniform4iv_p: GlFnCell<glProgramUniform4iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform4iv_is_loaded() -> bool {
  unsafe { *glProgramUniform4iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform4iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform4iv_t>>(gl_ptr_filter(f(b"glProgramUniform4iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4iv_reset_ptr() {
  *glProgramUniform4iv_p.0.get() = None;
}
/// [glProgramUniform4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4ui.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform4ui_p.0.get() } {
    Some(glProgramUniform4ui_inner) => glProgramUniform4ui_inner(program, location, v0, v1, v2, v3),
    None => gl_not_loaded("glProgramUniform4ui"),
  }
}
static glProgramUniform4ui_p: GlFnCell<glProgramUniform4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform4ui_is_loaded() -> bool {
  unsafe { *glProgramUniform4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform4ui_t>>(gl_ptr_filter(f(b"glProgramUniform4ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4ui_reset_ptr() {
  *glProgramUniform4ui_p.0.get() = None;
}
/// [glProgramUniform4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4uiv.xhtml)
/// * `program` class: program
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniform4uiv_p.0.get() } {
    Some(glProgramUniform4uiv_inner) => glProgramUniform4uiv_inner(program, location, count, value),
    None => gl_not_loaded("glProgramUniform4uiv"),
  }
}
static glProgramUniform4uiv_p: GlFnCell<glProgramUniform4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniform4uiv_is_loaded() -> bool {
  unsafe { *glProgramUniform4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniform4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniform4uiv_t>>(gl_ptr_filter(f(b"glProgramUniform4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniform4uiv_reset_ptr() {
  *glProgramUniform4uiv_p.0.get() = None;
}
/// [glProgramUniformMatrix2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix2dv_p.0.get() } {
    Some(glProgramUniformMatrix2dv_inner) => glProgramUniformMatrix2dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix2dv"),
  }
}
static glProgramUniformMatrix2dv_p: GlFnCell<glProgramUniformMatrix2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix2dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix2dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2dv_reset_ptr() {
  *glProgramUniformMatrix2dv_p.0.get() = None;
}
/// [glProgramUniformMatrix2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix2fv_p.0.get() } {
    Some(glProgramUniformMatrix2fv_inner) => glProgramUniformMatrix2fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix2fv"),
  }
}
static glProgramUniformMatrix2fv_p: GlFnCell<glProgramUniformMatrix2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix2fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix2fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2fv_reset_ptr() {
  *glProgramUniformMatrix2fv_p.0.get() = None;
}
/// [glProgramUniformMatrix2x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x3dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix2x3dv_p.0.get() } {
    Some(glProgramUniformMatrix2x3dv_inner) => glProgramUniformMatrix2x3dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix2x3dv"),
  }
}
static glProgramUniformMatrix2x3dv_p: GlFnCell<glProgramUniformMatrix2x3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix2x3dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix2x3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2x3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix2x3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix2x3dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix2x3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2x3dv_reset_ptr() {
  *glProgramUniformMatrix2x3dv_p.0.get() = None;
}
/// [glProgramUniformMatrix2x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x3fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix2x3fv_p.0.get() } {
    Some(glProgramUniformMatrix2x3fv_inner) => glProgramUniformMatrix2x3fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix2x3fv"),
  }
}
static glProgramUniformMatrix2x3fv_p: GlFnCell<glProgramUniformMatrix2x3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix2x3fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix2x3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2x3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix2x3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix2x3fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix2x3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2x3fv_reset_ptr() {
  *glProgramUniformMatrix2x3fv_p.0.get() = None;
}
/// [glProgramUniformMatrix2x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x4dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix2x4dv_p.0.get() } {
    Some(glProgramUniformMatrix2x4dv_inner) => glProgramUniformMatrix2x4dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix2x4dv"),
  }
}
static glProgramUniformMatrix2x4dv_p: GlFnCell<glProgramUniformMatrix2x4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix2x4dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix2x4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2x4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix2x4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix2x4dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix2x4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2x4dv_reset_ptr() {
  *glProgramUniformMatrix2x4dv_p.0.get() = None;
}
/// [glProgramUniformMatrix2x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x4fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix2x4fv_p.0.get() } {
    Some(glProgramUniformMatrix2x4fv_inner) => glProgramUniformMatrix2x4fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix2x4fv"),
  }
}
static glProgramUniformMatrix2x4fv_p: GlFnCell<glProgramUniformMatrix2x4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix2x4fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix2x4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2x4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix2x4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix2x4fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix2x4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix2x4fv_reset_ptr() {
  *glProgramUniformMatrix2x4fv_p.0.get() = None;
}
/// [glProgramUniformMatrix3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix3dv_p.0.get() } {
    Some(glProgramUniformMatrix3dv_inner) => glProgramUniformMatrix3dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix3dv"),
  }
}
static glProgramUniformMatrix3dv_p: GlFnCell<glProgramUniformMatrix3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix3dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix3dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3dv_reset_ptr() {
  *glProgramUniformMatrix3dv_p.0.get() = None;
}
/// [glProgramUniformMatrix3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*9
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix3fv_p.0.get() } {
    Some(glProgramUniformMatrix3fv_inner) => glProgramUniformMatrix3fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix3fv"),
  }
}
static glProgramUniformMatrix3fv_p: GlFnCell<glProgramUniformMatrix3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix3fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix3fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3fv_reset_ptr() {
  *glProgramUniformMatrix3fv_p.0.get() = None;
}
/// [glProgramUniformMatrix3x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x2dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix3x2dv_p.0.get() } {
    Some(glProgramUniformMatrix3x2dv_inner) => glProgramUniformMatrix3x2dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix3x2dv"),
  }
}
static glProgramUniformMatrix3x2dv_p: GlFnCell<glProgramUniformMatrix3x2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix3x2dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix3x2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3x2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix3x2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix3x2dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix3x2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3x2dv_reset_ptr() {
  *glProgramUniformMatrix3x2dv_p.0.get() = None;
}
/// [glProgramUniformMatrix3x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x2fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix3x2fv_p.0.get() } {
    Some(glProgramUniformMatrix3x2fv_inner) => glProgramUniformMatrix3x2fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix3x2fv"),
  }
}
static glProgramUniformMatrix3x2fv_p: GlFnCell<glProgramUniformMatrix3x2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix3x2fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix3x2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3x2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix3x2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix3x2fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix3x2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3x2fv_reset_ptr() {
  *glProgramUniformMatrix3x2fv_p.0.get() = None;
}
/// [glProgramUniformMatrix3x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x4dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix3x4dv_p.0.get() } {
    Some(glProgramUniformMatrix3x4dv_inner) => glProgramUniformMatrix3x4dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix3x4dv"),
  }
}
static glProgramUniformMatrix3x4dv_p: GlFnCell<glProgramUniformMatrix3x4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix3x4dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix3x4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3x4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix3x4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix3x4dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix3x4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3x4dv_reset_ptr() {
  *glProgramUniformMatrix3x4dv_p.0.get() = None;
}
/// [glProgramUniformMatrix3x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x4fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix3x4fv_p.0.get() } {
    Some(glProgramUniformMatrix3x4fv_inner) => glProgramUniformMatrix3x4fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix3x4fv"),
  }
}
static glProgramUniformMatrix3x4fv_p: GlFnCell<glProgramUniformMatrix3x4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix3x4fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix3x4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3x4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix3x4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix3x4fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix3x4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix3x4fv_reset_ptr() {
  *glProgramUniformMatrix3x4fv_p.0.get() = None;
}
/// [glProgramUniformMatrix4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix4dv_p.0.get() } {
    Some(glProgramUniformMatrix4dv_inner) => glProgramUniformMatrix4dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix4dv"),
  }
}
static glProgramUniformMatrix4dv_p: GlFnCell<glProgramUniformMatrix4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix4dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix4dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4dv_reset_ptr() {
  *glProgramUniformMatrix4dv_p.0.get() = None;
}
/// [glProgramUniformMatrix4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*16
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix4fv_p.0.get() } {
    Some(glProgramUniformMatrix4fv_inner) => glProgramUniformMatrix4fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix4fv"),
  }
}
static glProgramUniformMatrix4fv_p: GlFnCell<glProgramUniformMatrix4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix4fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix4fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4fv_reset_ptr() {
  *glProgramUniformMatrix4fv_p.0.get() = None;
}
/// [glProgramUniformMatrix4x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x2dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix4x2dv_p.0.get() } {
    Some(glProgramUniformMatrix4x2dv_inner) => glProgramUniformMatrix4x2dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix4x2dv"),
  }
}
static glProgramUniformMatrix4x2dv_p: GlFnCell<glProgramUniformMatrix4x2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix4x2dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix4x2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4x2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix4x2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix4x2dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix4x2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4x2dv_reset_ptr() {
  *glProgramUniformMatrix4x2dv_p.0.get() = None;
}
/// [glProgramUniformMatrix4x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x2fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix4x2fv_p.0.get() } {
    Some(glProgramUniformMatrix4x2fv_inner) => glProgramUniformMatrix4x2fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix4x2fv"),
  }
}
static glProgramUniformMatrix4x2fv_p: GlFnCell<glProgramUniformMatrix4x2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix4x2fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix4x2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4x2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix4x2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix4x2fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix4x2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4x2fv_reset_ptr() {
  *glProgramUniformMatrix4x2fv_p.0.get() = None;
}
/// [glProgramUniformMatrix4x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x3dv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix4x3dv_p.0.get() } {
    Some(glProgramUniformMatrix4x3dv_inner) => glProgramUniformMatrix4x3dv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix4x3dv"),
  }
}
static glProgramUniformMatrix4x3dv_p: GlFnCell<glProgramUniformMatrix4x3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix4x3dv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix4x3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4x3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix4x3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix4x3dv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix4x3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4x3dv_reset_ptr() {
  *glProgramUniformMatrix4x3dv_p.0.get() = None;
}
/// [glProgramUniformMatrix4x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x3fv.xhtml)
/// * `program` class: program
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glProgramUniformMatrix4x3fv_p.0.get() } {
    Some(glProgramUniformMatrix4x3fv_inner) => glProgramUniformMatrix4x3fv_inner(program, location, count, transpose, value),
    None => gl_not_loaded("glProgramUniformMatrix4x3fv"),
  }
}
static glProgramUniformMatrix4x3fv_p: GlFnCell<glProgramUniformMatrix4x3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProgramUniformMatrix4x3fv_is_loaded() -> bool {
  unsafe { *glProgramUniformMatrix4x3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4x3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProgramUniformMatrix4x3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProgramUniformMatrix4x3fv_t>>(gl_ptr_filter(f(b"glProgramUniformMatrix4x3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProgramUniformMatrix4x3fv_reset_ptr() {
  *glProgramUniformMatrix4x3fv_p.0.get() = None;
}
/// [glProvokingVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProvokingVertex.xhtml)
/// * `mode` group: VertexProvokingMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProvokingVertex(mode: VertexProvokingMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glProvokingVertex_p.0.get() } {
    Some(glProvokingVertex_inner) => glProvokingVertex_inner(mode),
    None => gl_not_loaded("glProvokingVertex"),
  }
}
static glProvokingVertex_p: GlFnCell<glProvokingVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProvokingVertex_is_loaded() -> bool {
  unsafe { *glProvokingVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProvokingVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProvokingVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProvokingVertex_t>>(gl_ptr_filter(f(b"glProvokingVertex\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glProvokingVertex_reset_ptr() {
  *glProvokingVertex_p.0.get() = None;
}
/// [glPushDebugGroup](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushDebugGroup.xhtml)
/// * `source` group: DebugSource
/// * `message` len: COMPSIZE(message,length)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPushDebugGroup(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glPushDebugGroup_p.0.get() } {
    Some(glPushDebugGroup_inner) => glPushDebugGroup_inner(source, id, length, message),
    None => gl_not_loaded("glPushDebugGroup"),
  }
}
static glPushDebugGroup_p: GlFnCell<glPushDebugGroup_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPushDebugGroup_is_loaded() -> bool {
  unsafe { *glPushDebugGroup_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPushDebugGroup_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPushDebugGroup_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPushDebugGroup_t>>(gl_ptr_filter(f(b"glPushDebugGroup\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPushDebugGroup_reset_ptr() {
  *glPushDebugGroup_p.0.get() = None;
}
/// [glPushDebugGroupKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushDebugGroupKHR.xhtml)
/// * `source` group: DebugSource
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPushDebugGroupKHR(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glPushDebugGroupKHR_p.0.get() } {
    Some(glPushDebugGroupKHR_inner) => glPushDebugGroupKHR_inner(source, id, length, message),
    None => gl_not_loaded("glPushDebugGroupKHR"),
  }
}
static glPushDebugGroupKHR_p: GlFnCell<glPushDebugGroupKHR_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPushDebugGroupKHR_is_loaded() -> bool {
  unsafe { *glPushDebugGroupKHR_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPushDebugGroupKHR_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPushDebugGroupKHR_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPushDebugGroupKHR_t>>(gl_ptr_filter(f(b"glPushDebugGroupKHR\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glPushDebugGroupKHR_reset_ptr() {
  *glPushDebugGroupKHR_p.0.get() = None;
}
/// [glQueryCounter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glQueryCounter.xhtml)
/// * `id` class: query
/// * `target` group: QueryCounterTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glQueryCounter(id: GLuint, target: QueryCounterTarget) {
  #[allow(unused_unsafe)]
  match unsafe { *glQueryCounter_p.0.get() } {
    Some(glQueryCounter_inner) => glQueryCounter_inner(id, target),
    None => gl_not_loaded("glQueryCounter"),
  }
}
static glQueryCounter_p: GlFnCell<glQueryCounter_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glQueryCounter_is_loaded() -> bool {
  unsafe { *glQueryCounter_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glQueryCounter_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glQueryCounter_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glQueryCounter_t>>(gl_ptr_filter(f(b"glQueryCounter\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glQueryCounter_reset_ptr() {
  *glQueryCounter_p.0.get() = None;
}
/// [glReadBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadBuffer.xhtml)
/// * `src` group: ReadBufferMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glReadBuffer(src: ReadBufferMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glReadBuffer_p.0.get() } {
    Some(glReadBuffer_inner) => glReadBuffer_inner(src),
    None => gl_not_loaded("glReadBuffer"),
  }
}
static glReadBuffer_p: GlFnCell<glReadBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glReadBuffer_is_loaded() -> bool {
  unsafe { *glReadBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glReadBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glReadBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glReadBuffer_t>>(gl_ptr_filter(f(b"glReadBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glReadBuffer_reset_ptr() {
  *glReadBuffer_p.0.get() = None;
}
/// [glReadPixels](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadPixels.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glReadPixels_p.0.get() } {
    Some(glReadPixels_inner) => glReadPixels_inner(x, y, width, height, format, type_, pixels),
    None => gl_not_loaded("glReadPixels"),
  }
}
static glReadPixels_p: GlFnCell<glReadPixels_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glReadPixels_is_loaded() -> bool {
  unsafe { *glReadPixels_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glReadPixels_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glReadPixels_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glReadPixels_t>>(gl_ptr_filter(f(b"glReadPixels\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glReadPixels_reset_ptr() {
  *glReadPixels_p.0.get() = None;
}
/// [glReadnPixels](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadnPixels.xhtml)
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `data` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glReadnPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glReadnPixels_p.0.get() } {
    Some(glReadnPixels_inner) => glReadnPixels_inner(x, y, width, height, format, type_, bufSize, data),
    None => gl_not_loaded("glReadnPixels"),
  }
}
static glReadnPixels_p: GlFnCell<glReadnPixels_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glReadnPixels_is_loaded() -> bool {
  unsafe { *glReadnPixels_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glReadnPixels_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glReadnPixels_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glReadnPixels_t>>(gl_ptr_filter(f(b"glReadnPixels\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glReadnPixels_reset_ptr() {
  *glReadnPixels_p.0.get() = None;
}
/// [glReleaseShaderCompiler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReleaseShaderCompiler.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glReleaseShaderCompiler() {
  #[allow(unused_unsafe)]
  match unsafe { *glReleaseShaderCompiler_p.0.get() } {
    Some(glReleaseShaderCompiler_inner) => glReleaseShaderCompiler_inner(),
    None => gl_not_loaded("glReleaseShaderCompiler"),
  }
}
static glReleaseShaderCompiler_p: GlFnCell<glReleaseShaderCompiler_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glReleaseShaderCompiler_is_loaded() -> bool {
  unsafe { *glReleaseShaderCompiler_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glReleaseShaderCompiler_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glReleaseShaderCompiler_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glReleaseShaderCompiler_t>>(gl_ptr_filter(f(b"glReleaseShaderCompiler\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glReleaseShaderCompiler_reset_ptr() {
  *glReleaseShaderCompiler_p.0.get() = None;
}
/// [glRenderbufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorage.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glRenderbufferStorage(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glRenderbufferStorage_p.0.get() } {
    Some(glRenderbufferStorage_inner) => glRenderbufferStorage_inner(target, internalformat, width, height),
    None => gl_not_loaded("glRenderbufferStorage"),
  }
}
static glRenderbufferStorage_p: GlFnCell<glRenderbufferStorage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glRenderbufferStorage_is_loaded() -> bool {
  unsafe { *glRenderbufferStorage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glRenderbufferStorage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glRenderbufferStorage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glRenderbufferStorage_t>>(gl_ptr_filter(f(b"glRenderbufferStorage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glRenderbufferStorage_reset_ptr() {
  *glRenderbufferStorage_p.0.get() = None;
}
/// [glRenderbufferStorageMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisample.xhtml)
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glRenderbufferStorageMultisample(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glRenderbufferStorageMultisample_p.0.get() } {
    Some(glRenderbufferStorageMultisample_inner) => glRenderbufferStorageMultisample_inner(target, samples, internalformat, width, height),
    None => gl_not_loaded("glRenderbufferStorageMultisample"),
  }
}
static glRenderbufferStorageMultisample_p: GlFnCell<glRenderbufferStorageMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glRenderbufferStorageMultisample_is_loaded() -> bool {
  unsafe { *glRenderbufferStorageMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glRenderbufferStorageMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glRenderbufferStorageMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glRenderbufferStorageMultisample_t>>(gl_ptr_filter(f(b"glRenderbufferStorageMultisample\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glRenderbufferStorageMultisample_reset_ptr() {
  *glRenderbufferStorageMultisample_p.0.get() = None;
}
/// [glResumeTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResumeTransformFeedback.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glResumeTransformFeedback() {
  #[allow(unused_unsafe)]
  match unsafe { *glResumeTransformFeedback_p.0.get() } {
    Some(glResumeTransformFeedback_inner) => glResumeTransformFeedback_inner(),
    None => gl_not_loaded("glResumeTransformFeedback"),
  }
}
static glResumeTransformFeedback_p: GlFnCell<glResumeTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glResumeTransformFeedback_is_loaded() -> bool {
  unsafe { *glResumeTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glResumeTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glResumeTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glResumeTransformFeedback_t>>(gl_ptr_filter(f(b"glResumeTransformFeedback\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glResumeTransformFeedback_reset_ptr() {
  *glResumeTransformFeedback_p.0.get() = None;
}
/// [glSampleCoverage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleCoverage.xhtml)
/// * `invert` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSampleCoverage(value: GLfloat, invert: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glSampleCoverage_p.0.get() } {
    Some(glSampleCoverage_inner) => glSampleCoverage_inner(value, invert),
    None => gl_not_loaded("glSampleCoverage"),
  }
}
static glSampleCoverage_p: GlFnCell<glSampleCoverage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSampleCoverage_is_loaded() -> bool {
  unsafe { *glSampleCoverage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSampleCoverage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSampleCoverage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSampleCoverage_t>>(gl_ptr_filter(f(b"glSampleCoverage\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSampleCoverage_reset_ptr() {
  *glSampleCoverage_p.0.get() = None;
}
/// [glSampleMaski](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleMaski.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSampleMaski(maskNumber: GLuint, mask: GLbitfield) {
  #[allow(unused_unsafe)]
  match unsafe { *glSampleMaski_p.0.get() } {
    Some(glSampleMaski_inner) => glSampleMaski_inner(maskNumber, mask),
    None => gl_not_loaded("glSampleMaski"),
  }
}
static glSampleMaski_p: GlFnCell<glSampleMaski_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSampleMaski_is_loaded() -> bool {
  unsafe { *glSampleMaski_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSampleMaski_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSampleMaski_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSampleMaski_t>>(gl_ptr_filter(f(b"glSampleMaski\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSampleMaski_reset_ptr() {
  *glSampleMaski_p.0.get() = None;
}
/// [glSamplerParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIiv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameterIiv(sampler: GLuint, pname: SamplerParameterI, param: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameterIiv_p.0.get() } {
    Some(glSamplerParameterIiv_inner) => glSamplerParameterIiv_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameterIiv"),
  }
}
static glSamplerParameterIiv_p: GlFnCell<glSamplerParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameterIiv_is_loaded() -> bool {
  unsafe { *glSamplerParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameterIiv_t>>(gl_ptr_filter(f(b"glSamplerParameterIiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterIiv_reset_ptr() {
  *glSamplerParameterIiv_p.0.get() = None;
}
/// [glSamplerParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIuiv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameterIuiv(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameterIuiv_p.0.get() } {
    Some(glSamplerParameterIuiv_inner) => glSamplerParameterIuiv_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameterIuiv"),
  }
}
static glSamplerParameterIuiv_p: GlFnCell<glSamplerParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameterIuiv_is_loaded() -> bool {
  unsafe { *glSamplerParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameterIuiv_t>>(gl_ptr_filter(f(b"glSamplerParameterIuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterIuiv_reset_ptr() {
  *glSamplerParameterIuiv_p.0.get() = None;
}
/// [glSamplerParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterf.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameterf(sampler: GLuint, pname: SamplerParameterF, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameterf_p.0.get() } {
    Some(glSamplerParameterf_inner) => glSamplerParameterf_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameterf"),
  }
}
static glSamplerParameterf_p: GlFnCell<glSamplerParameterf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameterf_is_loaded() -> bool {
  unsafe { *glSamplerParameterf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameterf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameterf_t>>(gl_ptr_filter(f(b"glSamplerParameterf\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterf_reset_ptr() {
  *glSamplerParameterf_p.0.get() = None;
}
/// [glSamplerParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterfv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
/// * `param` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameterfv(sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameterfv_p.0.get() } {
    Some(glSamplerParameterfv_inner) => glSamplerParameterfv_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameterfv"),
  }
}
static glSamplerParameterfv_p: GlFnCell<glSamplerParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameterfv_is_loaded() -> bool {
  unsafe { *glSamplerParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameterfv_t>>(gl_ptr_filter(f(b"glSamplerParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterfv_reset_ptr() {
  *glSamplerParameterfv_p.0.get() = None;
}
/// [glSamplerParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameteri.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameteri(sampler: GLuint, pname: SamplerParameterI, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameteri_p.0.get() } {
    Some(glSamplerParameteri_inner) => glSamplerParameteri_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameteri"),
  }
}
static glSamplerParameteri_p: GlFnCell<glSamplerParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameteri_is_loaded() -> bool {
  unsafe { *glSamplerParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameteri_t>>(gl_ptr_filter(f(b"glSamplerParameteri\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSamplerParameteri_reset_ptr() {
  *glSamplerParameteri_p.0.get() = None;
}
/// [glSamplerParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameteriv.xhtml)
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameteriv(sampler: GLuint, pname: SamplerParameterI, param: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameteriv_p.0.get() } {
    Some(glSamplerParameteriv_inner) => glSamplerParameteriv_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameteriv"),
  }
}
static glSamplerParameteriv_p: GlFnCell<glSamplerParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameteriv_is_loaded() -> bool {
  unsafe { *glSamplerParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameteriv_t>>(gl_ptr_filter(f(b"glSamplerParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSamplerParameteriv_reset_ptr() {
  *glSamplerParameteriv_p.0.get() = None;
}
/// [glScissor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissor.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glScissor_p.0.get() } {
    Some(glScissor_inner) => glScissor_inner(x, y, width, height),
    None => gl_not_loaded("glScissor"),
  }
}
static glScissor_p: GlFnCell<glScissor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glScissor_is_loaded() -> bool {
  unsafe { *glScissor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glScissor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glScissor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glScissor_t>>(gl_ptr_filter(f(b"glScissor\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glScissor_reset_ptr() {
  *glScissor_p.0.get() = None;
}
/// [glScissorArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorArrayv.xhtml)
/// * `v` len: COMPSIZE(count)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glScissorArrayv_p.0.get() } {
    Some(glScissorArrayv_inner) => glScissorArrayv_inner(first, count, v),
    None => gl_not_loaded("glScissorArrayv"),
  }
}
static glScissorArrayv_p: GlFnCell<glScissorArrayv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glScissorArrayv_is_loaded() -> bool {
  unsafe { *glScissorArrayv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glScissorArrayv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glScissorArrayv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glScissorArrayv_t>>(gl_ptr_filter(f(b"glScissorArrayv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glScissorArrayv_reset_ptr() {
  *glScissorArrayv_p.0.get() = None;
}
/// [glScissorIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexed.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glScissorIndexed_p.0.get() } {
    Some(glScissorIndexed_inner) => glScissorIndexed_inner(index, left, bottom, width, height),
    None => gl_not_loaded("glScissorIndexed"),
  }
}
static glScissorIndexed_p: GlFnCell<glScissorIndexed_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glScissorIndexed_is_loaded() -> bool {
  unsafe { *glScissorIndexed_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glScissorIndexed_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glScissorIndexed_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glScissorIndexed_t>>(gl_ptr_filter(f(b"glScissorIndexed\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glScissorIndexed_reset_ptr() {
  *glScissorIndexed_p.0.get() = None;
}
/// [glScissorIndexedv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexedv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glScissorIndexedv(index: GLuint, v: *const [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glScissorIndexedv_p.0.get() } {
    Some(glScissorIndexedv_inner) => glScissorIndexedv_inner(index, v),
    None => gl_not_loaded("glScissorIndexedv"),
  }
}
static glScissorIndexedv_p: GlFnCell<glScissorIndexedv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glScissorIndexedv_is_loaded() -> bool {
  unsafe { *glScissorIndexedv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glScissorIndexedv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glScissorIndexedv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glScissorIndexedv_t>>(gl_ptr_filter(f(b"glScissorIndexedv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glScissorIndexedv_reset_ptr() {
  *glScissorIndexedv_p.0.get() = None;
}
/// [glSecondaryColorP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorP3ui.xhtml)
/// * `type` group: ColorPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSecondaryColorP3ui(type_: ColorPointerType, color: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSecondaryColorP3ui_p.0.get() } {
    Some(glSecondaryColorP3ui_inner) => glSecondaryColorP3ui_inner(type_, color),
    None => gl_not_loaded("glSecondaryColorP3ui"),
  }
}
static glSecondaryColorP3ui_p: GlFnCell<glSecondaryColorP3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSecondaryColorP3ui_is_loaded() -> bool {
  unsafe { *glSecondaryColorP3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSecondaryColorP3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSecondaryColorP3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSecondaryColorP3ui_t>>(gl_ptr_filter(f(b"glSecondaryColorP3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSecondaryColorP3ui_reset_ptr() {
  *glSecondaryColorP3ui_p.0.get() = None;
}
/// [glSecondaryColorP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorP3uiv.xhtml)
/// * `type` group: ColorPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSecondaryColorP3uiv(type_: ColorPointerType, color: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSecondaryColorP3uiv_p.0.get() } {
    Some(glSecondaryColorP3uiv_inner) => glSecondaryColorP3uiv_inner(type_, color),
    None => gl_not_loaded("glSecondaryColorP3uiv"),
  }
}
static glSecondaryColorP3uiv_p: GlFnCell<glSecondaryColorP3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSecondaryColorP3uiv_is_loaded() -> bool {
  unsafe { *glSecondaryColorP3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSecondaryColorP3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSecondaryColorP3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSecondaryColorP3uiv_t>>(gl_ptr_filter(f(b"glSecondaryColorP3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSecondaryColorP3uiv_reset_ptr() {
  *glSecondaryColorP3uiv_p.0.get() = None;
}
/// [glShaderBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderBinary.xhtml)
/// * `shaders` class: shader
/// * `shaders` len: count
/// * `binaryFormat` group: ShaderBinaryFormat
/// * `binary` len: length
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glShaderBinary(count: GLsizei, shaders: *const GLuint, binaryFormat: ShaderBinaryFormat, binary: *const void, length: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glShaderBinary_p.0.get() } {
    Some(glShaderBinary_inner) => glShaderBinary_inner(count, shaders, binaryFormat, binary, length),
    None => gl_not_loaded("glShaderBinary"),
  }
}
static glShaderBinary_p: GlFnCell<glShaderBinary_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glShaderBinary_is_loaded() -> bool {
  unsafe { *glShaderBinary_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glShaderBinary_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glShaderBinary_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glShaderBinary_t>>(gl_ptr_filter(f(b"glShaderBinary\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glShaderBinary_reset_ptr() {
  *glShaderBinary_p.0.get() = None;
}
/// [glShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml)
/// * `shader` class: shader
/// * `string` len: count
/// * `length` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glShaderSource_p.0.get() } {
    Some(glShaderSource_inner) => glShaderSource_inner(shader, count, string, length),
    None => gl_not_loaded("glShaderSource"),
  }
}
static glShaderSource_p: GlFnCell<glShaderSource_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glShaderSource_is_loaded() -> bool {
  unsafe { *glShaderSource_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glShaderSource_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glShaderSource_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glShaderSource_t>>(gl_ptr_filter(f(b"glShaderSource\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glShaderSource_reset_ptr() {
  *glShaderSource_p.0.get() = None;
}
/// [glShaderStorageBlockBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderStorageBlockBinding.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glShaderStorageBlockBinding_p.0.get() } {
    Some(glShaderStorageBlockBinding_inner) => glShaderStorageBlockBinding_inner(program, storageBlockIndex, storageBlockBinding),
    None => gl_not_loaded("glShaderStorageBlockBinding"),
  }
}
static glShaderStorageBlockBinding_p: GlFnCell<glShaderStorageBlockBinding_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glShaderStorageBlockBinding_is_loaded() -> bool {
  unsafe { *glShaderStorageBlockBinding_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glShaderStorageBlockBinding_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glShaderStorageBlockBinding_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glShaderStorageBlockBinding_t>>(gl_ptr_filter(f(b"glShaderStorageBlockBinding\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glShaderStorageBlockBinding_reset_ptr() {
  *glShaderStorageBlockBinding_p.0.get() = None;
}
/// [glSpecializeShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSpecializeShader.xhtml)
/// * `shader` class: shader
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSpecializeShader(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSpecializeShader_p.0.get() } {
    Some(glSpecializeShader_inner) => glSpecializeShader_inner(shader, pEntryPoint, numSpecializationConstants, pConstantIndex, pConstantValue),
    None => gl_not_loaded("glSpecializeShader"),
  }
}
static glSpecializeShader_p: GlFnCell<glSpecializeShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSpecializeShader_is_loaded() -> bool {
  unsafe { *glSpecializeShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSpecializeShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSpecializeShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSpecializeShader_t>>(gl_ptr_filter(f(b"glSpecializeShader\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glSpecializeShader_reset_ptr() {
  *glSpecializeShader_p.0.get() = None;
}
/// [glStencilFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFunc.xhtml)
/// * `func` group: StencilFunction
/// * `ref` group: StencilValue
/// * `mask` group: MaskedStencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilFunc(func: StencilFunction, ref_: GLint, mask: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilFunc_p.0.get() } {
    Some(glStencilFunc_inner) => glStencilFunc_inner(func, ref_, mask),
    None => gl_not_loaded("glStencilFunc"),
  }
}
static glStencilFunc_p: GlFnCell<glStencilFunc_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilFunc_is_loaded() -> bool {
  unsafe { *glStencilFunc_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilFunc_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilFunc_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilFunc_t>>(gl_ptr_filter(f(b"glStencilFunc\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glStencilFunc_reset_ptr() {
  *glStencilFunc_p.0.get() = None;
}
/// [glStencilFuncSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFuncSeparate.xhtml)
/// * `face` group: StencilFaceDirection
/// * `func` group: StencilFunction
/// * `ref` group: StencilValue
/// * `mask` group: MaskedStencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilFuncSeparate(face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilFuncSeparate_p.0.get() } {
    Some(glStencilFuncSeparate_inner) => glStencilFuncSeparate_inner(face, func, ref_, mask),
    None => gl_not_loaded("glStencilFuncSeparate"),
  }
}
static glStencilFuncSeparate_p: GlFnCell<glStencilFuncSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilFuncSeparate_is_loaded() -> bool {
  unsafe { *glStencilFuncSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilFuncSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilFuncSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilFuncSeparate_t>>(gl_ptr_filter(f(b"glStencilFuncSeparate\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glStencilFuncSeparate_reset_ptr() {
  *glStencilFuncSeparate_p.0.get() = None;
}
/// [glStencilMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilMask.xhtml)
/// * `mask` group: MaskedStencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilMask(mask: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilMask_p.0.get() } {
    Some(glStencilMask_inner) => glStencilMask_inner(mask),
    None => gl_not_loaded("glStencilMask"),
  }
}
static glStencilMask_p: GlFnCell<glStencilMask_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilMask_is_loaded() -> bool {
  unsafe { *glStencilMask_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilMask_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilMask_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilMask_t>>(gl_ptr_filter(f(b"glStencilMask\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glStencilMask_reset_ptr() {
  *glStencilMask_p.0.get() = None;
}
/// [glStencilMaskSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilMaskSeparate.xhtml)
/// * `face` group: StencilFaceDirection
/// * `mask` group: MaskedStencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilMaskSeparate(face: StencilFaceDirection, mask: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilMaskSeparate_p.0.get() } {
    Some(glStencilMaskSeparate_inner) => glStencilMaskSeparate_inner(face, mask),
    None => gl_not_loaded("glStencilMaskSeparate"),
  }
}
static glStencilMaskSeparate_p: GlFnCell<glStencilMaskSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilMaskSeparate_is_loaded() -> bool {
  unsafe { *glStencilMaskSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilMaskSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilMaskSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilMaskSeparate_t>>(gl_ptr_filter(f(b"glStencilMaskSeparate\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glStencilMaskSeparate_reset_ptr() {
  *glStencilMaskSeparate_p.0.get() = None;
}
/// [glStencilOp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilOp.xhtml)
/// * `fail` group: StencilOp
/// * `zfail` group: StencilOp
/// * `zpass` group: StencilOp
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilOp(fail: StencilOp, zfail: StencilOp, zpass: StencilOp) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilOp_p.0.get() } {
    Some(glStencilOp_inner) => glStencilOp_inner(fail, zfail, zpass),
    None => gl_not_loaded("glStencilOp"),
  }
}
static glStencilOp_p: GlFnCell<glStencilOp_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilOp_is_loaded() -> bool {
  unsafe { *glStencilOp_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilOp_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilOp_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilOp_t>>(gl_ptr_filter(f(b"glStencilOp\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glStencilOp_reset_ptr() {
  *glStencilOp_p.0.get() = None;
}
/// [glStencilOpSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilOpSeparate.xhtml)
/// * `face` group: StencilFaceDirection
/// * `sfail` group: StencilOp
/// * `dpfail` group: StencilOp
/// * `dppass` group: StencilOp
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilOpSeparate(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilOpSeparate_p.0.get() } {
    Some(glStencilOpSeparate_inner) => glStencilOpSeparate_inner(face, sfail, dpfail, dppass),
    None => gl_not_loaded("glStencilOpSeparate"),
  }
}
static glStencilOpSeparate_p: GlFnCell<glStencilOpSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilOpSeparate_is_loaded() -> bool {
  unsafe { *glStencilOpSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilOpSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilOpSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilOpSeparate_t>>(gl_ptr_filter(f(b"glStencilOpSeparate\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glStencilOpSeparate_reset_ptr() {
  *glStencilOpSeparate_p.0.get() = None;
}
/// [glTexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBuffer.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexBuffer(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexBuffer_p.0.get() } {
    Some(glTexBuffer_inner) => glTexBuffer_inner(target, internalformat, buffer),
    None => gl_not_loaded("glTexBuffer"),
  }
}
static glTexBuffer_p: GlFnCell<glTexBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexBuffer_is_loaded() -> bool {
  unsafe { *glTexBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexBuffer_t>>(gl_ptr_filter(f(b"glTexBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexBuffer_reset_ptr() {
  *glTexBuffer_p.0.get() = None;
}
/// [glTexBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBufferRange.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexBufferRange(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexBufferRange_p.0.get() } {
    Some(glTexBufferRange_inner) => glTexBufferRange_inner(target, internalformat, buffer, offset, size),
    None => gl_not_loaded("glTexBufferRange"),
  }
}
static glTexBufferRange_p: GlFnCell<glTexBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexBufferRange_is_loaded() -> bool {
  unsafe { *glTexBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexBufferRange_t>>(gl_ptr_filter(f(b"glTexBufferRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexBufferRange_reset_ptr() {
  *glTexBufferRange_p.0.get() = None;
}
/// [glTexCoordP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP1ui.xhtml)
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexCoordP1ui(type_: TexCoordPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexCoordP1ui_p.0.get() } {
    Some(glTexCoordP1ui_inner) => glTexCoordP1ui_inner(type_, coords),
    None => gl_not_loaded("glTexCoordP1ui"),
  }
}
static glTexCoordP1ui_p: GlFnCell<glTexCoordP1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexCoordP1ui_is_loaded() -> bool {
  unsafe { *glTexCoordP1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexCoordP1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexCoordP1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexCoordP1ui_t>>(gl_ptr_filter(f(b"glTexCoordP1ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexCoordP1ui_reset_ptr() {
  *glTexCoordP1ui_p.0.get() = None;
}
/// [glTexCoordP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP1uiv.xhtml)
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexCoordP1uiv(type_: TexCoordPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexCoordP1uiv_p.0.get() } {
    Some(glTexCoordP1uiv_inner) => glTexCoordP1uiv_inner(type_, coords),
    None => gl_not_loaded("glTexCoordP1uiv"),
  }
}
static glTexCoordP1uiv_p: GlFnCell<glTexCoordP1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexCoordP1uiv_is_loaded() -> bool {
  unsafe { *glTexCoordP1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexCoordP1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexCoordP1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexCoordP1uiv_t>>(gl_ptr_filter(f(b"glTexCoordP1uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexCoordP1uiv_reset_ptr() {
  *glTexCoordP1uiv_p.0.get() = None;
}
/// [glTexCoordP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP2ui.xhtml)
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexCoordP2ui(type_: TexCoordPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexCoordP2ui_p.0.get() } {
    Some(glTexCoordP2ui_inner) => glTexCoordP2ui_inner(type_, coords),
    None => gl_not_loaded("glTexCoordP2ui"),
  }
}
static glTexCoordP2ui_p: GlFnCell<glTexCoordP2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexCoordP2ui_is_loaded() -> bool {
  unsafe { *glTexCoordP2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexCoordP2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexCoordP2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexCoordP2ui_t>>(gl_ptr_filter(f(b"glTexCoordP2ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexCoordP2ui_reset_ptr() {
  *glTexCoordP2ui_p.0.get() = None;
}
/// [glTexCoordP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP2uiv.xhtml)
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexCoordP2uiv(type_: TexCoordPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexCoordP2uiv_p.0.get() } {
    Some(glTexCoordP2uiv_inner) => glTexCoordP2uiv_inner(type_, coords),
    None => gl_not_loaded("glTexCoordP2uiv"),
  }
}
static glTexCoordP2uiv_p: GlFnCell<glTexCoordP2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexCoordP2uiv_is_loaded() -> bool {
  unsafe { *glTexCoordP2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexCoordP2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexCoordP2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexCoordP2uiv_t>>(gl_ptr_filter(f(b"glTexCoordP2uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexCoordP2uiv_reset_ptr() {
  *glTexCoordP2uiv_p.0.get() = None;
}
/// [glTexCoordP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP3ui.xhtml)
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexCoordP3ui(type_: TexCoordPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexCoordP3ui_p.0.get() } {
    Some(glTexCoordP3ui_inner) => glTexCoordP3ui_inner(type_, coords),
    None => gl_not_loaded("glTexCoordP3ui"),
  }
}
static glTexCoordP3ui_p: GlFnCell<glTexCoordP3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexCoordP3ui_is_loaded() -> bool {
  unsafe { *glTexCoordP3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexCoordP3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexCoordP3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexCoordP3ui_t>>(gl_ptr_filter(f(b"glTexCoordP3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexCoordP3ui_reset_ptr() {
  *glTexCoordP3ui_p.0.get() = None;
}
/// [glTexCoordP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP3uiv.xhtml)
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexCoordP3uiv(type_: TexCoordPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexCoordP3uiv_p.0.get() } {
    Some(glTexCoordP3uiv_inner) => glTexCoordP3uiv_inner(type_, coords),
    None => gl_not_loaded("glTexCoordP3uiv"),
  }
}
static glTexCoordP3uiv_p: GlFnCell<glTexCoordP3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexCoordP3uiv_is_loaded() -> bool {
  unsafe { *glTexCoordP3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexCoordP3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexCoordP3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexCoordP3uiv_t>>(gl_ptr_filter(f(b"glTexCoordP3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexCoordP3uiv_reset_ptr() {
  *glTexCoordP3uiv_p.0.get() = None;
}
/// [glTexCoordP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP4ui.xhtml)
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexCoordP4ui(type_: TexCoordPointerType, coords: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexCoordP4ui_p.0.get() } {
    Some(glTexCoordP4ui_inner) => glTexCoordP4ui_inner(type_, coords),
    None => gl_not_loaded("glTexCoordP4ui"),
  }
}
static glTexCoordP4ui_p: GlFnCell<glTexCoordP4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexCoordP4ui_is_loaded() -> bool {
  unsafe { *glTexCoordP4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexCoordP4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexCoordP4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexCoordP4ui_t>>(gl_ptr_filter(f(b"glTexCoordP4ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexCoordP4ui_reset_ptr() {
  *glTexCoordP4ui_p.0.get() = None;
}
/// [glTexCoordP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP4uiv.xhtml)
/// * `type` group: TexCoordPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexCoordP4uiv(type_: TexCoordPointerType, coords: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexCoordP4uiv_p.0.get() } {
    Some(glTexCoordP4uiv_inner) => glTexCoordP4uiv_inner(type_, coords),
    None => gl_not_loaded("glTexCoordP4uiv"),
  }
}
static glTexCoordP4uiv_p: GlFnCell<glTexCoordP4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexCoordP4uiv_is_loaded() -> bool {
  unsafe { *glTexCoordP4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexCoordP4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexCoordP4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexCoordP4uiv_t>>(gl_ptr_filter(f(b"glTexCoordP4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexCoordP4uiv_reset_ptr() {
  *glTexCoordP4uiv_p.0.get() = None;
}
/// [glTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage1D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage1D_p.0.get() } {
    Some(glTexImage1D_inner) => glTexImage1D_inner(target, level, internalformat, width, border, format, type_, pixels),
    None => gl_not_loaded("glTexImage1D"),
  }
}
static glTexImage1D_p: GlFnCell<glTexImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage1D_is_loaded() -> bool {
  unsafe { *glTexImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage1D_t>>(gl_ptr_filter(f(b"glTexImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexImage1D_reset_ptr() {
  *glTexImage1D_p.0.get() = None;
}
/// [glTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage2D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage2D_p.0.get() } {
    Some(glTexImage2D_inner) => glTexImage2D_inner(target, level, internalformat, width, height, border, format, type_, pixels),
    None => gl_not_loaded("glTexImage2D"),
  }
}
static glTexImage2D_p: GlFnCell<glTexImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage2D_is_loaded() -> bool {
  unsafe { *glTexImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage2D_t>>(gl_ptr_filter(f(b"glTexImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexImage2D_reset_ptr() {
  *glTexImage2D_p.0.get() = None;
}
/// [glTexImage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2DMultisample.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage2DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage2DMultisample_p.0.get() } {
    Some(glTexImage2DMultisample_inner) => glTexImage2DMultisample_inner(target, samples, internalformat, width, height, fixedsamplelocations),
    None => gl_not_loaded("glTexImage2DMultisample"),
  }
}
static glTexImage2DMultisample_p: GlFnCell<glTexImage2DMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage2DMultisample_is_loaded() -> bool {
  unsafe { *glTexImage2DMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage2DMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage2DMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage2DMultisample_t>>(gl_ptr_filter(f(b"glTexImage2DMultisample\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexImage2DMultisample_reset_ptr() {
  *glTexImage2DMultisample_p.0.get() = None;
}
/// [glTexImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage3D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage3D_p.0.get() } {
    Some(glTexImage3D_inner) => glTexImage3D_inner(target, level, internalformat, width, height, depth, border, format, type_, pixels),
    None => gl_not_loaded("glTexImage3D"),
  }
}
static glTexImage3D_p: GlFnCell<glTexImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage3D_is_loaded() -> bool {
  unsafe { *glTexImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage3D_t>>(gl_ptr_filter(f(b"glTexImage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexImage3D_reset_ptr() {
  *glTexImage3D_p.0.get() = None;
}
/// [glTexImage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3DMultisample.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage3DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage3DMultisample_p.0.get() } {
    Some(glTexImage3DMultisample_inner) => glTexImage3DMultisample_inner(target, samples, internalformat, width, height, depth, fixedsamplelocations),
    None => gl_not_loaded("glTexImage3DMultisample"),
  }
}
static glTexImage3DMultisample_p: GlFnCell<glTexImage3DMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage3DMultisample_is_loaded() -> bool {
  unsafe { *glTexImage3DMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage3DMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage3DMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage3DMultisample_t>>(gl_ptr_filter(f(b"glTexImage3DMultisample\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexImage3DMultisample_reset_ptr() {
  *glTexImage3DMultisample_p.0.get() = None;
}
/// [glTexParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIiv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameterIiv(target: TextureTarget, pname: TextureParameterName, params: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameterIiv_p.0.get() } {
    Some(glTexParameterIiv_inner) => glTexParameterIiv_inner(target, pname, params),
    None => gl_not_loaded("glTexParameterIiv"),
  }
}
static glTexParameterIiv_p: GlFnCell<glTexParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameterIiv_is_loaded() -> bool {
  unsafe { *glTexParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameterIiv_t>>(gl_ptr_filter(f(b"glTexParameterIiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexParameterIiv_reset_ptr() {
  *glTexParameterIiv_p.0.get() = None;
}
/// [glTexParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIuiv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameterIuiv(target: TextureTarget, pname: TextureParameterName, params: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameterIuiv_p.0.get() } {
    Some(glTexParameterIuiv_inner) => glTexParameterIuiv_inner(target, pname, params),
    None => gl_not_loaded("glTexParameterIuiv"),
  }
}
static glTexParameterIuiv_p: GlFnCell<glTexParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameterIuiv_is_loaded() -> bool {
  unsafe { *glTexParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameterIuiv_t>>(gl_ptr_filter(f(b"glTexParameterIuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexParameterIuiv_reset_ptr() {
  *glTexParameterIuiv_p.0.get() = None;
}
/// [glTexParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterf.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameterf(target: TextureTarget, pname: TextureParameterName, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameterf_p.0.get() } {
    Some(glTexParameterf_inner) => glTexParameterf_inner(target, pname, param),
    None => gl_not_loaded("glTexParameterf"),
  }
}
static glTexParameterf_p: GlFnCell<glTexParameterf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameterf_is_loaded() -> bool {
  unsafe { *glTexParameterf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameterf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameterf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameterf_t>>(gl_ptr_filter(f(b"glTexParameterf\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexParameterf_reset_ptr() {
  *glTexParameterf_p.0.get() = None;
}
/// [glTexParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterfv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameterfv(target: TextureTarget, pname: TextureParameterName, params: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameterfv_p.0.get() } {
    Some(glTexParameterfv_inner) => glTexParameterfv_inner(target, pname, params),
    None => gl_not_loaded("glTexParameterfv"),
  }
}
static glTexParameterfv_p: GlFnCell<glTexParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameterfv_is_loaded() -> bool {
  unsafe { *glTexParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameterfv_t>>(gl_ptr_filter(f(b"glTexParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexParameterfv_reset_ptr() {
  *glTexParameterfv_p.0.get() = None;
}
/// [glTexParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameteri.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameteri(target: TextureTarget, pname: TextureParameterName, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameteri_p.0.get() } {
    Some(glTexParameteri_inner) => glTexParameteri_inner(target, pname, param),
    None => gl_not_loaded("glTexParameteri"),
  }
}
static glTexParameteri_p: GlFnCell<glTexParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameteri_is_loaded() -> bool {
  unsafe { *glTexParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameteri_t>>(gl_ptr_filter(f(b"glTexParameteri\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexParameteri_reset_ptr() {
  *glTexParameteri_p.0.get() = None;
}
/// [glTexParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameteriv.xhtml)
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameteriv(target: TextureTarget, pname: TextureParameterName, params: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameteriv_p.0.get() } {
    Some(glTexParameteriv_inner) => glTexParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glTexParameteriv"),
  }
}
static glTexParameteriv_p: GlFnCell<glTexParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameteriv_is_loaded() -> bool {
  unsafe { *glTexParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameteriv_t>>(gl_ptr_filter(f(b"glTexParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexParameteriv_reset_ptr() {
  *glTexParameteriv_p.0.get() = None;
}
/// [glTexStorage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage1D.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexStorage1D(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexStorage1D_p.0.get() } {
    Some(glTexStorage1D_inner) => glTexStorage1D_inner(target, levels, internalformat, width),
    None => gl_not_loaded("glTexStorage1D"),
  }
}
static glTexStorage1D_p: GlFnCell<glTexStorage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexStorage1D_is_loaded() -> bool {
  unsafe { *glTexStorage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexStorage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexStorage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexStorage1D_t>>(gl_ptr_filter(f(b"glTexStorage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexStorage1D_reset_ptr() {
  *glTexStorage1D_p.0.get() = None;
}
/// [glTexStorage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage2D.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexStorage2D(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexStorage2D_p.0.get() } {
    Some(glTexStorage2D_inner) => glTexStorage2D_inner(target, levels, internalformat, width, height),
    None => gl_not_loaded("glTexStorage2D"),
  }
}
static glTexStorage2D_p: GlFnCell<glTexStorage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexStorage2D_is_loaded() -> bool {
  unsafe { *glTexStorage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexStorage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexStorage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexStorage2D_t>>(gl_ptr_filter(f(b"glTexStorage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexStorage2D_reset_ptr() {
  *glTexStorage2D_p.0.get() = None;
}
/// [glTexStorage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage2DMultisample.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexStorage2DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexStorage2DMultisample_p.0.get() } {
    Some(glTexStorage2DMultisample_inner) => glTexStorage2DMultisample_inner(target, samples, internalformat, width, height, fixedsamplelocations),
    None => gl_not_loaded("glTexStorage2DMultisample"),
  }
}
static glTexStorage2DMultisample_p: GlFnCell<glTexStorage2DMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexStorage2DMultisample_is_loaded() -> bool {
  unsafe { *glTexStorage2DMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexStorage2DMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexStorage2DMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexStorage2DMultisample_t>>(gl_ptr_filter(f(b"glTexStorage2DMultisample\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexStorage2DMultisample_reset_ptr() {
  *glTexStorage2DMultisample_p.0.get() = None;
}
/// [glTexStorage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage3D.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexStorage3D(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexStorage3D_p.0.get() } {
    Some(glTexStorage3D_inner) => glTexStorage3D_inner(target, levels, internalformat, width, height, depth),
    None => gl_not_loaded("glTexStorage3D"),
  }
}
static glTexStorage3D_p: GlFnCell<glTexStorage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexStorage3D_is_loaded() -> bool {
  unsafe { *glTexStorage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexStorage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexStorage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexStorage3D_t>>(gl_ptr_filter(f(b"glTexStorage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexStorage3D_reset_ptr() {
  *glTexStorage3D_p.0.get() = None;
}
/// [glTexStorage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage3DMultisample.xhtml)
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexStorage3DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexStorage3DMultisample_p.0.get() } {
    Some(glTexStorage3DMultisample_inner) => glTexStorage3DMultisample_inner(target, samples, internalformat, width, height, depth, fixedsamplelocations),
    None => gl_not_loaded("glTexStorage3DMultisample"),
  }
}
static glTexStorage3DMultisample_p: GlFnCell<glTexStorage3DMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexStorage3DMultisample_is_loaded() -> bool {
  unsafe { *glTexStorage3DMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexStorage3DMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexStorage3DMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexStorage3DMultisample_t>>(gl_ptr_filter(f(b"glTexStorage3DMultisample\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexStorage3DMultisample_reset_ptr() {
  *glTexStorage3DMultisample_p.0.get() = None;
}
/// [glTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage1D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexSubImage1D_p.0.get() } {
    Some(glTexSubImage1D_inner) => glTexSubImage1D_inner(target, level, xoffset, width, format, type_, pixels),
    None => gl_not_loaded("glTexSubImage1D"),
  }
}
static glTexSubImage1D_p: GlFnCell<glTexSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexSubImage1D_is_loaded() -> bool {
  unsafe { *glTexSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexSubImage1D_t>>(gl_ptr_filter(f(b"glTexSubImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexSubImage1D_reset_ptr() {
  *glTexSubImage1D_p.0.get() = None;
}
/// [glTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage2D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexSubImage2D_p.0.get() } {
    Some(glTexSubImage2D_inner) => glTexSubImage2D_inner(target, level, xoffset, yoffset, width, height, format, type_, pixels),
    None => gl_not_loaded("glTexSubImage2D"),
  }
}
static glTexSubImage2D_p: GlFnCell<glTexSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexSubImage2D_is_loaded() -> bool {
  unsafe { *glTexSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexSubImage2D_t>>(gl_ptr_filter(f(b"glTexSubImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexSubImage2D_reset_ptr() {
  *glTexSubImage2D_p.0.get() = None;
}
/// [glTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage3D.xhtml)
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexSubImage3D_p.0.get() } {
    Some(glTexSubImage3D_inner) => glTexSubImage3D_inner(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels),
    None => gl_not_loaded("glTexSubImage3D"),
  }
}
static glTexSubImage3D_p: GlFnCell<glTexSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexSubImage3D_is_loaded() -> bool {
  unsafe { *glTexSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexSubImage3D_t>>(gl_ptr_filter(f(b"glTexSubImage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTexSubImage3D_reset_ptr() {
  *glTexSubImage3D_p.0.get() = None;
}
/// [glTextureBarrier](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBarrier.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureBarrier() {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureBarrier_p.0.get() } {
    Some(glTextureBarrier_inner) => glTextureBarrier_inner(),
    None => gl_not_loaded("glTextureBarrier"),
  }
}
static glTextureBarrier_p: GlFnCell<glTextureBarrier_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureBarrier_is_loaded() -> bool {
  unsafe { *glTextureBarrier_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureBarrier_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureBarrier_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureBarrier_t>>(gl_ptr_filter(f(b"glTextureBarrier\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureBarrier_reset_ptr() {
  *glTextureBarrier_p.0.get() = None;
}
/// [glTextureBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBuffer.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureBuffer(texture: GLuint, internalformat: InternalFormat, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureBuffer_p.0.get() } {
    Some(glTextureBuffer_inner) => glTextureBuffer_inner(texture, internalformat, buffer),
    None => gl_not_loaded("glTextureBuffer"),
  }
}
static glTextureBuffer_p: GlFnCell<glTextureBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureBuffer_is_loaded() -> bool {
  unsafe { *glTextureBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureBuffer_t>>(gl_ptr_filter(f(b"glTextureBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureBuffer_reset_ptr() {
  *glTextureBuffer_p.0.get() = None;
}
/// [glTextureBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBufferRange.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureBufferRange(texture: GLuint, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureBufferRange_p.0.get() } {
    Some(glTextureBufferRange_inner) => glTextureBufferRange_inner(texture, internalformat, buffer, offset, size),
    None => gl_not_loaded("glTextureBufferRange"),
  }
}
static glTextureBufferRange_p: GlFnCell<glTextureBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureBufferRange_is_loaded() -> bool {
  unsafe { *glTextureBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureBufferRange_t>>(gl_ptr_filter(f(b"glTextureBufferRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureBufferRange_reset_ptr() {
  *glTextureBufferRange_p.0.get() = None;
}
/// [glTextureParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterIiv.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureParameterIiv(texture: GLuint, pname: TextureParameterName, params: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureParameterIiv_p.0.get() } {
    Some(glTextureParameterIiv_inner) => glTextureParameterIiv_inner(texture, pname, params),
    None => gl_not_loaded("glTextureParameterIiv"),
  }
}
static glTextureParameterIiv_p: GlFnCell<glTextureParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureParameterIiv_is_loaded() -> bool {
  unsafe { *glTextureParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureParameterIiv_t>>(gl_ptr_filter(f(b"glTextureParameterIiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureParameterIiv_reset_ptr() {
  *glTextureParameterIiv_p.0.get() = None;
}
/// [glTextureParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterIuiv.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureParameterIuiv(texture: GLuint, pname: TextureParameterName, params: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureParameterIuiv_p.0.get() } {
    Some(glTextureParameterIuiv_inner) => glTextureParameterIuiv_inner(texture, pname, params),
    None => gl_not_loaded("glTextureParameterIuiv"),
  }
}
static glTextureParameterIuiv_p: GlFnCell<glTextureParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureParameterIuiv_is_loaded() -> bool {
  unsafe { *glTextureParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureParameterIuiv_t>>(gl_ptr_filter(f(b"glTextureParameterIuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureParameterIuiv_reset_ptr() {
  *glTextureParameterIuiv_p.0.get() = None;
}
/// [glTextureParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterf.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureParameterf(texture: GLuint, pname: TextureParameterName, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureParameterf_p.0.get() } {
    Some(glTextureParameterf_inner) => glTextureParameterf_inner(texture, pname, param),
    None => gl_not_loaded("glTextureParameterf"),
  }
}
static glTextureParameterf_p: GlFnCell<glTextureParameterf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureParameterf_is_loaded() -> bool {
  unsafe { *glTextureParameterf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureParameterf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureParameterf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureParameterf_t>>(gl_ptr_filter(f(b"glTextureParameterf\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureParameterf_reset_ptr() {
  *glTextureParameterf_p.0.get() = None;
}
/// [glTextureParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterfv.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureParameterfv(texture: GLuint, pname: TextureParameterName, param: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureParameterfv_p.0.get() } {
    Some(glTextureParameterfv_inner) => glTextureParameterfv_inner(texture, pname, param),
    None => gl_not_loaded("glTextureParameterfv"),
  }
}
static glTextureParameterfv_p: GlFnCell<glTextureParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureParameterfv_is_loaded() -> bool {
  unsafe { *glTextureParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureParameterfv_t>>(gl_ptr_filter(f(b"glTextureParameterfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureParameterfv_reset_ptr() {
  *glTextureParameterfv_p.0.get() = None;
}
/// [glTextureParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameteri.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureParameteri(texture: GLuint, pname: TextureParameterName, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureParameteri_p.0.get() } {
    Some(glTextureParameteri_inner) => glTextureParameteri_inner(texture, pname, param),
    None => gl_not_loaded("glTextureParameteri"),
  }
}
static glTextureParameteri_p: GlFnCell<glTextureParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureParameteri_is_loaded() -> bool {
  unsafe { *glTextureParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureParameteri_t>>(gl_ptr_filter(f(b"glTextureParameteri\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureParameteri_reset_ptr() {
  *glTextureParameteri_p.0.get() = None;
}
/// [glTextureParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameteriv.xhtml)
/// * `texture` class: texture
/// * `pname` group: TextureParameterName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureParameteriv(texture: GLuint, pname: TextureParameterName, param: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureParameteriv_p.0.get() } {
    Some(glTextureParameteriv_inner) => glTextureParameteriv_inner(texture, pname, param),
    None => gl_not_loaded("glTextureParameteriv"),
  }
}
static glTextureParameteriv_p: GlFnCell<glTextureParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureParameteriv_is_loaded() -> bool {
  unsafe { *glTextureParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureParameteriv_t>>(gl_ptr_filter(f(b"glTextureParameteriv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureParameteriv_reset_ptr() {
  *glTextureParameteriv_p.0.get() = None;
}
/// [glTextureStorage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage1D.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureStorage1D(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureStorage1D_p.0.get() } {
    Some(glTextureStorage1D_inner) => glTextureStorage1D_inner(texture, levels, internalformat, width),
    None => gl_not_loaded("glTextureStorage1D"),
  }
}
static glTextureStorage1D_p: GlFnCell<glTextureStorage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureStorage1D_is_loaded() -> bool {
  unsafe { *glTextureStorage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureStorage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureStorage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureStorage1D_t>>(gl_ptr_filter(f(b"glTextureStorage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureStorage1D_reset_ptr() {
  *glTextureStorage1D_p.0.get() = None;
}
/// [glTextureStorage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage2D.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureStorage2D(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureStorage2D_p.0.get() } {
    Some(glTextureStorage2D_inner) => glTextureStorage2D_inner(texture, levels, internalformat, width, height),
    None => gl_not_loaded("glTextureStorage2D"),
  }
}
static glTextureStorage2D_p: GlFnCell<glTextureStorage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureStorage2D_is_loaded() -> bool {
  unsafe { *glTextureStorage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureStorage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureStorage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureStorage2D_t>>(gl_ptr_filter(f(b"glTextureStorage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureStorage2D_reset_ptr() {
  *glTextureStorage2D_p.0.get() = None;
}
/// [glTextureStorage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage2DMultisample.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureStorage2DMultisample(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureStorage2DMultisample_p.0.get() } {
    Some(glTextureStorage2DMultisample_inner) => glTextureStorage2DMultisample_inner(texture, samples, internalformat, width, height, fixedsamplelocations),
    None => gl_not_loaded("glTextureStorage2DMultisample"),
  }
}
static glTextureStorage2DMultisample_p: GlFnCell<glTextureStorage2DMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureStorage2DMultisample_is_loaded() -> bool {
  unsafe { *glTextureStorage2DMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureStorage2DMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureStorage2DMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureStorage2DMultisample_t>>(gl_ptr_filter(f(b"glTextureStorage2DMultisample\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureStorage2DMultisample_reset_ptr() {
  *glTextureStorage2DMultisample_p.0.get() = None;
}
/// [glTextureStorage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage3D.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureStorage3D(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureStorage3D_p.0.get() } {
    Some(glTextureStorage3D_inner) => glTextureStorage3D_inner(texture, levels, internalformat, width, height, depth),
    None => gl_not_loaded("glTextureStorage3D"),
  }
}
static glTextureStorage3D_p: GlFnCell<glTextureStorage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureStorage3D_is_loaded() -> bool {
  unsafe { *glTextureStorage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureStorage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureStorage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureStorage3D_t>>(gl_ptr_filter(f(b"glTextureStorage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureStorage3D_reset_ptr() {
  *glTextureStorage3D_p.0.get() = None;
}
/// [glTextureStorage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage3DMultisample.xhtml)
/// * `texture` class: texture
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureStorage3DMultisample(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureStorage3DMultisample_p.0.get() } {
    Some(glTextureStorage3DMultisample_inner) => glTextureStorage3DMultisample_inner(texture, samples, internalformat, width, height, depth, fixedsamplelocations),
    None => gl_not_loaded("glTextureStorage3DMultisample"),
  }
}
static glTextureStorage3DMultisample_p: GlFnCell<glTextureStorage3DMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureStorage3DMultisample_is_loaded() -> bool {
  unsafe { *glTextureStorage3DMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureStorage3DMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureStorage3DMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureStorage3DMultisample_t>>(gl_ptr_filter(f(b"glTextureStorage3DMultisample\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureStorage3DMultisample_reset_ptr() {
  *glTextureStorage3DMultisample_p.0.get() = None;
}
/// [glTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage1D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureSubImage1D_p.0.get() } {
    Some(glTextureSubImage1D_inner) => glTextureSubImage1D_inner(texture, level, xoffset, width, format, type_, pixels),
    None => gl_not_loaded("glTextureSubImage1D"),
  }
}
static glTextureSubImage1D_p: GlFnCell<glTextureSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureSubImage1D_is_loaded() -> bool {
  unsafe { *glTextureSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureSubImage1D_t>>(gl_ptr_filter(f(b"glTextureSubImage1D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureSubImage1D_reset_ptr() {
  *glTextureSubImage1D_p.0.get() = None;
}
/// [glTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage2D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureSubImage2D_p.0.get() } {
    Some(glTextureSubImage2D_inner) => glTextureSubImage2D_inner(texture, level, xoffset, yoffset, width, height, format, type_, pixels),
    None => gl_not_loaded("glTextureSubImage2D"),
  }
}
static glTextureSubImage2D_p: GlFnCell<glTextureSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureSubImage2D_is_loaded() -> bool {
  unsafe { *glTextureSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureSubImage2D_t>>(gl_ptr_filter(f(b"glTextureSubImage2D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureSubImage2D_reset_ptr() {
  *glTextureSubImage2D_p.0.get() = None;
}
/// [glTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage3D.xhtml)
/// * `texture` class: texture
/// * `format` group: PixelFormat
/// * `type` group: PixelType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureSubImage3D_p.0.get() } {
    Some(glTextureSubImage3D_inner) => glTextureSubImage3D_inner(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels),
    None => gl_not_loaded("glTextureSubImage3D"),
  }
}
static glTextureSubImage3D_p: GlFnCell<glTextureSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureSubImage3D_is_loaded() -> bool {
  unsafe { *glTextureSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureSubImage3D_t>>(gl_ptr_filter(f(b"glTextureSubImage3D\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureSubImage3D_reset_ptr() {
  *glTextureSubImage3D_p.0.get() = None;
}
/// [glTextureView](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureView.xhtml)
/// * `texture` class: texture
/// * `target` group: TextureTarget
/// * `origtexture` class: texture
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTextureView(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTextureView_p.0.get() } {
    Some(glTextureView_inner) => glTextureView_inner(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers),
    None => gl_not_loaded("glTextureView"),
  }
}
static glTextureView_p: GlFnCell<glTextureView_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTextureView_is_loaded() -> bool {
  unsafe { *glTextureView_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTextureView_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTextureView_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTextureView_t>>(gl_ptr_filter(f(b"glTextureView\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTextureView_reset_ptr() {
  *glTextureView_p.0.get() = None;
}
/// [glTransformFeedbackBufferBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackBufferBase.xhtml)
/// * `xfb` class: transform feedback
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTransformFeedbackBufferBase_p.0.get() } {
    Some(glTransformFeedbackBufferBase_inner) => glTransformFeedbackBufferBase_inner(xfb, index, buffer),
    None => gl_not_loaded("glTransformFeedbackBufferBase"),
  }
}
static glTransformFeedbackBufferBase_p: GlFnCell<glTransformFeedbackBufferBase_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTransformFeedbackBufferBase_is_loaded() -> bool {
  unsafe { *glTransformFeedbackBufferBase_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTransformFeedbackBufferBase_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTransformFeedbackBufferBase_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTransformFeedbackBufferBase_t>>(gl_ptr_filter(f(b"glTransformFeedbackBufferBase\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTransformFeedbackBufferBase_reset_ptr() {
  *glTransformFeedbackBufferBase_p.0.get() = None;
}
/// [glTransformFeedbackBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackBufferRange.xhtml)
/// * `xfb` class: transform feedback
/// * `buffer` class: buffer
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTransformFeedbackBufferRange(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glTransformFeedbackBufferRange_p.0.get() } {
    Some(glTransformFeedbackBufferRange_inner) => glTransformFeedbackBufferRange_inner(xfb, index, buffer, offset, size),
    None => gl_not_loaded("glTransformFeedbackBufferRange"),
  }
}
static glTransformFeedbackBufferRange_p: GlFnCell<glTransformFeedbackBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTransformFeedbackBufferRange_is_loaded() -> bool {
  unsafe { *glTransformFeedbackBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTransformFeedbackBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTransformFeedbackBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTransformFeedbackBufferRange_t>>(gl_ptr_filter(f(b"glTransformFeedbackBufferRange\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTransformFeedbackBufferRange_reset_ptr() {
  *glTransformFeedbackBufferRange_p.0.get() = None;
}
/// [glTransformFeedbackVaryings](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackVaryings.xhtml)
/// * `program` class: program
/// * `varyings` len: count
/// * `bufferMode` group: TransformFeedbackBufferMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glTransformFeedbackVaryings_p.0.get() } {
    Some(glTransformFeedbackVaryings_inner) => glTransformFeedbackVaryings_inner(program, count, varyings, bufferMode),
    None => gl_not_loaded("glTransformFeedbackVaryings"),
  }
}
static glTransformFeedbackVaryings_p: GlFnCell<glTransformFeedbackVaryings_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTransformFeedbackVaryings_is_loaded() -> bool {
  unsafe { *glTransformFeedbackVaryings_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTransformFeedbackVaryings_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTransformFeedbackVaryings_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTransformFeedbackVaryings_t>>(gl_ptr_filter(f(b"glTransformFeedbackVaryings\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glTransformFeedbackVaryings_reset_ptr() {
  *glTransformFeedbackVaryings_p.0.get() = None;
}
/// [glUniform1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1d(location: GLint, x: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1d_p.0.get() } {
    Some(glUniform1d_inner) => glUniform1d_inner(location, x),
    None => gl_not_loaded("glUniform1d"),
  }
}
static glUniform1d_p: GlFnCell<glUniform1d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1d_is_loaded() -> bool {
  unsafe { *glUniform1d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1d_t>>(gl_ptr_filter(f(b"glUniform1d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform1d_reset_ptr() {
  *glUniform1d_p.0.get() = None;
}
/// [glUniform1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1dv.xhtml)
/// * `value` len: count*1
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1dv_p.0.get() } {
    Some(glUniform1dv_inner) => glUniform1dv_inner(location, count, value),
    None => gl_not_loaded("glUniform1dv"),
  }
}
static glUniform1dv_p: GlFnCell<glUniform1dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1dv_is_loaded() -> bool {
  unsafe { *glUniform1dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1dv_t>>(gl_ptr_filter(f(b"glUniform1dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform1dv_reset_ptr() {
  *glUniform1dv_p.0.get() = None;
}
/// [glUniform1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1f.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1f(location: GLint, v0: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1f_p.0.get() } {
    Some(glUniform1f_inner) => glUniform1f_inner(location, v0),
    None => gl_not_loaded("glUniform1f"),
  }
}
static glUniform1f_p: GlFnCell<glUniform1f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1f_is_loaded() -> bool {
  unsafe { *glUniform1f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1f_t>>(gl_ptr_filter(f(b"glUniform1f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform1f_reset_ptr() {
  *glUniform1f_p.0.get() = None;
}
/// [glUniform1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1fv.xhtml)
/// * `value` len: count*1
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1fv_p.0.get() } {
    Some(glUniform1fv_inner) => glUniform1fv_inner(location, count, value),
    None => gl_not_loaded("glUniform1fv"),
  }
}
static glUniform1fv_p: GlFnCell<glUniform1fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1fv_is_loaded() -> bool {
  unsafe { *glUniform1fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1fv_t>>(gl_ptr_filter(f(b"glUniform1fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform1fv_reset_ptr() {
  *glUniform1fv_p.0.get() = None;
}
/// [glUniform1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1i.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1i(location: GLint, v0: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1i_p.0.get() } {
    Some(glUniform1i_inner) => glUniform1i_inner(location, v0),
    None => gl_not_loaded("glUniform1i"),
  }
}
static glUniform1i_p: GlFnCell<glUniform1i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1i_is_loaded() -> bool {
  unsafe { *glUniform1i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1i_t>>(gl_ptr_filter(f(b"glUniform1i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform1i_reset_ptr() {
  *glUniform1i_p.0.get() = None;
}
/// [glUniform1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1iv.xhtml)
/// * `value` len: count*1
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1iv_p.0.get() } {
    Some(glUniform1iv_inner) => glUniform1iv_inner(location, count, value),
    None => gl_not_loaded("glUniform1iv"),
  }
}
static glUniform1iv_p: GlFnCell<glUniform1iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1iv_is_loaded() -> bool {
  unsafe { *glUniform1iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1iv_t>>(gl_ptr_filter(f(b"glUniform1iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform1iv_reset_ptr() {
  *glUniform1iv_p.0.get() = None;
}
/// [glUniform1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1ui.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1ui(location: GLint, v0: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1ui_p.0.get() } {
    Some(glUniform1ui_inner) => glUniform1ui_inner(location, v0),
    None => gl_not_loaded("glUniform1ui"),
  }
}
static glUniform1ui_p: GlFnCell<glUniform1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1ui_is_loaded() -> bool {
  unsafe { *glUniform1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1ui_t>>(gl_ptr_filter(f(b"glUniform1ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform1ui_reset_ptr() {
  *glUniform1ui_p.0.get() = None;
}
/// [glUniform1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1uiv.xhtml)
/// * `value` len: count*1
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1uiv_p.0.get() } {
    Some(glUniform1uiv_inner) => glUniform1uiv_inner(location, count, value),
    None => gl_not_loaded("glUniform1uiv"),
  }
}
static glUniform1uiv_p: GlFnCell<glUniform1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1uiv_is_loaded() -> bool {
  unsafe { *glUniform1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1uiv_t>>(gl_ptr_filter(f(b"glUniform1uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform1uiv_reset_ptr() {
  *glUniform1uiv_p.0.get() = None;
}
/// [glUniform2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2d(location: GLint, x: GLdouble, y: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2d_p.0.get() } {
    Some(glUniform2d_inner) => glUniform2d_inner(location, x, y),
    None => gl_not_loaded("glUniform2d"),
  }
}
static glUniform2d_p: GlFnCell<glUniform2d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2d_is_loaded() -> bool {
  unsafe { *glUniform2d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2d_t>>(gl_ptr_filter(f(b"glUniform2d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform2d_reset_ptr() {
  *glUniform2d_p.0.get() = None;
}
/// [glUniform2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2dv.xhtml)
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2dv_p.0.get() } {
    Some(glUniform2dv_inner) => glUniform2dv_inner(location, count, value),
    None => gl_not_loaded("glUniform2dv"),
  }
}
static glUniform2dv_p: GlFnCell<glUniform2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2dv_is_loaded() -> bool {
  unsafe { *glUniform2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2dv_t>>(gl_ptr_filter(f(b"glUniform2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform2dv_reset_ptr() {
  *glUniform2dv_p.0.get() = None;
}
/// [glUniform2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2f.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2f_p.0.get() } {
    Some(glUniform2f_inner) => glUniform2f_inner(location, v0, v1),
    None => gl_not_loaded("glUniform2f"),
  }
}
static glUniform2f_p: GlFnCell<glUniform2f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2f_is_loaded() -> bool {
  unsafe { *glUniform2f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2f_t>>(gl_ptr_filter(f(b"glUniform2f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform2f_reset_ptr() {
  *glUniform2f_p.0.get() = None;
}
/// [glUniform2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2fv.xhtml)
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2fv_p.0.get() } {
    Some(glUniform2fv_inner) => glUniform2fv_inner(location, count, value),
    None => gl_not_loaded("glUniform2fv"),
  }
}
static glUniform2fv_p: GlFnCell<glUniform2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2fv_is_loaded() -> bool {
  unsafe { *glUniform2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2fv_t>>(gl_ptr_filter(f(b"glUniform2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform2fv_reset_ptr() {
  *glUniform2fv_p.0.get() = None;
}
/// [glUniform2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2i.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2i(location: GLint, v0: GLint, v1: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2i_p.0.get() } {
    Some(glUniform2i_inner) => glUniform2i_inner(location, v0, v1),
    None => gl_not_loaded("glUniform2i"),
  }
}
static glUniform2i_p: GlFnCell<glUniform2i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2i_is_loaded() -> bool {
  unsafe { *glUniform2i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2i_t>>(gl_ptr_filter(f(b"glUniform2i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform2i_reset_ptr() {
  *glUniform2i_p.0.get() = None;
}
/// [glUniform2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2iv.xhtml)
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2iv_p.0.get() } {
    Some(glUniform2iv_inner) => glUniform2iv_inner(location, count, value),
    None => gl_not_loaded("glUniform2iv"),
  }
}
static glUniform2iv_p: GlFnCell<glUniform2iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2iv_is_loaded() -> bool {
  unsafe { *glUniform2iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2iv_t>>(gl_ptr_filter(f(b"glUniform2iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform2iv_reset_ptr() {
  *glUniform2iv_p.0.get() = None;
}
/// [glUniform2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2ui.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2ui_p.0.get() } {
    Some(glUniform2ui_inner) => glUniform2ui_inner(location, v0, v1),
    None => gl_not_loaded("glUniform2ui"),
  }
}
static glUniform2ui_p: GlFnCell<glUniform2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2ui_is_loaded() -> bool {
  unsafe { *glUniform2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2ui_t>>(gl_ptr_filter(f(b"glUniform2ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform2ui_reset_ptr() {
  *glUniform2ui_p.0.get() = None;
}
/// [glUniform2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2uiv.xhtml)
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2uiv_p.0.get() } {
    Some(glUniform2uiv_inner) => glUniform2uiv_inner(location, count, value),
    None => gl_not_loaded("glUniform2uiv"),
  }
}
static glUniform2uiv_p: GlFnCell<glUniform2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2uiv_is_loaded() -> bool {
  unsafe { *glUniform2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2uiv_t>>(gl_ptr_filter(f(b"glUniform2uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform2uiv_reset_ptr() {
  *glUniform2uiv_p.0.get() = None;
}
/// [glUniform3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3d_p.0.get() } {
    Some(glUniform3d_inner) => glUniform3d_inner(location, x, y, z),
    None => gl_not_loaded("glUniform3d"),
  }
}
static glUniform3d_p: GlFnCell<glUniform3d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3d_is_loaded() -> bool {
  unsafe { *glUniform3d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3d_t>>(gl_ptr_filter(f(b"glUniform3d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform3d_reset_ptr() {
  *glUniform3d_p.0.get() = None;
}
/// [glUniform3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3dv.xhtml)
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3dv_p.0.get() } {
    Some(glUniform3dv_inner) => glUniform3dv_inner(location, count, value),
    None => gl_not_loaded("glUniform3dv"),
  }
}
static glUniform3dv_p: GlFnCell<glUniform3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3dv_is_loaded() -> bool {
  unsafe { *glUniform3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3dv_t>>(gl_ptr_filter(f(b"glUniform3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform3dv_reset_ptr() {
  *glUniform3dv_p.0.get() = None;
}
/// [glUniform3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3f.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3f_p.0.get() } {
    Some(glUniform3f_inner) => glUniform3f_inner(location, v0, v1, v2),
    None => gl_not_loaded("glUniform3f"),
  }
}
static glUniform3f_p: GlFnCell<glUniform3f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3f_is_loaded() -> bool {
  unsafe { *glUniform3f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3f_t>>(gl_ptr_filter(f(b"glUniform3f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform3f_reset_ptr() {
  *glUniform3f_p.0.get() = None;
}
/// [glUniform3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3fv.xhtml)
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3fv_p.0.get() } {
    Some(glUniform3fv_inner) => glUniform3fv_inner(location, count, value),
    None => gl_not_loaded("glUniform3fv"),
  }
}
static glUniform3fv_p: GlFnCell<glUniform3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3fv_is_loaded() -> bool {
  unsafe { *glUniform3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3fv_t>>(gl_ptr_filter(f(b"glUniform3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform3fv_reset_ptr() {
  *glUniform3fv_p.0.get() = None;
}
/// [glUniform3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3i.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3i_p.0.get() } {
    Some(glUniform3i_inner) => glUniform3i_inner(location, v0, v1, v2),
    None => gl_not_loaded("glUniform3i"),
  }
}
static glUniform3i_p: GlFnCell<glUniform3i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3i_is_loaded() -> bool {
  unsafe { *glUniform3i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3i_t>>(gl_ptr_filter(f(b"glUniform3i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform3i_reset_ptr() {
  *glUniform3i_p.0.get() = None;
}
/// [glUniform3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3iv.xhtml)
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3iv_p.0.get() } {
    Some(glUniform3iv_inner) => glUniform3iv_inner(location, count, value),
    None => gl_not_loaded("glUniform3iv"),
  }
}
static glUniform3iv_p: GlFnCell<glUniform3iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3iv_is_loaded() -> bool {
  unsafe { *glUniform3iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3iv_t>>(gl_ptr_filter(f(b"glUniform3iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform3iv_reset_ptr() {
  *glUniform3iv_p.0.get() = None;
}
/// [glUniform3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3ui.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3ui_p.0.get() } {
    Some(glUniform3ui_inner) => glUniform3ui_inner(location, v0, v1, v2),
    None => gl_not_loaded("glUniform3ui"),
  }
}
static glUniform3ui_p: GlFnCell<glUniform3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3ui_is_loaded() -> bool {
  unsafe { *glUniform3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3ui_t>>(gl_ptr_filter(f(b"glUniform3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform3ui_reset_ptr() {
  *glUniform3ui_p.0.get() = None;
}
/// [glUniform3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3uiv.xhtml)
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3uiv_p.0.get() } {
    Some(glUniform3uiv_inner) => glUniform3uiv_inner(location, count, value),
    None => gl_not_loaded("glUniform3uiv"),
  }
}
static glUniform3uiv_p: GlFnCell<glUniform3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3uiv_is_loaded() -> bool {
  unsafe { *glUniform3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3uiv_t>>(gl_ptr_filter(f(b"glUniform3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform3uiv_reset_ptr() {
  *glUniform3uiv_p.0.get() = None;
}
/// [glUniform4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4d_p.0.get() } {
    Some(glUniform4d_inner) => glUniform4d_inner(location, x, y, z, w),
    None => gl_not_loaded("glUniform4d"),
  }
}
static glUniform4d_p: GlFnCell<glUniform4d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4d_is_loaded() -> bool {
  unsafe { *glUniform4d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4d_t>>(gl_ptr_filter(f(b"glUniform4d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform4d_reset_ptr() {
  *glUniform4d_p.0.get() = None;
}
/// [glUniform4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4dv.xhtml)
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4dv_p.0.get() } {
    Some(glUniform4dv_inner) => glUniform4dv_inner(location, count, value),
    None => gl_not_loaded("glUniform4dv"),
  }
}
static glUniform4dv_p: GlFnCell<glUniform4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4dv_is_loaded() -> bool {
  unsafe { *glUniform4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4dv_t>>(gl_ptr_filter(f(b"glUniform4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform4dv_reset_ptr() {
  *glUniform4dv_p.0.get() = None;
}
/// [glUniform4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4f.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4f_p.0.get() } {
    Some(glUniform4f_inner) => glUniform4f_inner(location, v0, v1, v2, v3),
    None => gl_not_loaded("glUniform4f"),
  }
}
static glUniform4f_p: GlFnCell<glUniform4f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4f_is_loaded() -> bool {
  unsafe { *glUniform4f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4f_t>>(gl_ptr_filter(f(b"glUniform4f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform4f_reset_ptr() {
  *glUniform4f_p.0.get() = None;
}
/// [glUniform4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4fv.xhtml)
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4fv_p.0.get() } {
    Some(glUniform4fv_inner) => glUniform4fv_inner(location, count, value),
    None => gl_not_loaded("glUniform4fv"),
  }
}
static glUniform4fv_p: GlFnCell<glUniform4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4fv_is_loaded() -> bool {
  unsafe { *glUniform4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4fv_t>>(gl_ptr_filter(f(b"glUniform4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform4fv_reset_ptr() {
  *glUniform4fv_p.0.get() = None;
}
/// [glUniform4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4i.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4i_p.0.get() } {
    Some(glUniform4i_inner) => glUniform4i_inner(location, v0, v1, v2, v3),
    None => gl_not_loaded("glUniform4i"),
  }
}
static glUniform4i_p: GlFnCell<glUniform4i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4i_is_loaded() -> bool {
  unsafe { *glUniform4i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4i_t>>(gl_ptr_filter(f(b"glUniform4i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform4i_reset_ptr() {
  *glUniform4i_p.0.get() = None;
}
/// [glUniform4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4iv.xhtml)
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4iv_p.0.get() } {
    Some(glUniform4iv_inner) => glUniform4iv_inner(location, count, value),
    None => gl_not_loaded("glUniform4iv"),
  }
}
static glUniform4iv_p: GlFnCell<glUniform4iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4iv_is_loaded() -> bool {
  unsafe { *glUniform4iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4iv_t>>(gl_ptr_filter(f(b"glUniform4iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform4iv_reset_ptr() {
  *glUniform4iv_p.0.get() = None;
}
/// [glUniform4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4ui.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4ui_p.0.get() } {
    Some(glUniform4ui_inner) => glUniform4ui_inner(location, v0, v1, v2, v3),
    None => gl_not_loaded("glUniform4ui"),
  }
}
static glUniform4ui_p: GlFnCell<glUniform4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4ui_is_loaded() -> bool {
  unsafe { *glUniform4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4ui_t>>(gl_ptr_filter(f(b"glUniform4ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform4ui_reset_ptr() {
  *glUniform4ui_p.0.get() = None;
}
/// [glUniform4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4uiv.xhtml)
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4uiv_p.0.get() } {
    Some(glUniform4uiv_inner) => glUniform4uiv_inner(location, count, value),
    None => gl_not_loaded("glUniform4uiv"),
  }
}
static glUniform4uiv_p: GlFnCell<glUniform4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4uiv_is_loaded() -> bool {
  unsafe { *glUniform4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4uiv_t>>(gl_ptr_filter(f(b"glUniform4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniform4uiv_reset_ptr() {
  *glUniform4uiv_p.0.get() = None;
}
/// [glUniformBlockBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformBlockBinding.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformBlockBinding_p.0.get() } {
    Some(glUniformBlockBinding_inner) => glUniformBlockBinding_inner(program, uniformBlockIndex, uniformBlockBinding),
    None => gl_not_loaded("glUniformBlockBinding"),
  }
}
static glUniformBlockBinding_p: GlFnCell<glUniformBlockBinding_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformBlockBinding_is_loaded() -> bool {
  unsafe { *glUniformBlockBinding_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformBlockBinding_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformBlockBinding_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformBlockBinding_t>>(gl_ptr_filter(f(b"glUniformBlockBinding\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformBlockBinding_reset_ptr() {
  *glUniformBlockBinding_p.0.get() = None;
}
/// [glUniformMatrix2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2dv_p.0.get() } {
    Some(glUniformMatrix2dv_inner) => glUniformMatrix2dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2dv"),
  }
}
static glUniformMatrix2dv_p: GlFnCell<glUniformMatrix2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2dv_t>>(gl_ptr_filter(f(b"glUniformMatrix2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2dv_reset_ptr() {
  *glUniformMatrix2dv_p.0.get() = None;
}
/// [glUniformMatrix2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2fv_p.0.get() } {
    Some(glUniformMatrix2fv_inner) => glUniformMatrix2fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2fv"),
  }
}
static glUniformMatrix2fv_p: GlFnCell<glUniformMatrix2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2fv_t>>(gl_ptr_filter(f(b"glUniformMatrix2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2fv_reset_ptr() {
  *glUniformMatrix2fv_p.0.get() = None;
}
/// [glUniformMatrix2x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x3dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2x3dv_p.0.get() } {
    Some(glUniformMatrix2x3dv_inner) => glUniformMatrix2x3dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2x3dv"),
  }
}
static glUniformMatrix2x3dv_p: GlFnCell<glUniformMatrix2x3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2x3dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2x3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2x3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2x3dv_t>>(gl_ptr_filter(f(b"glUniformMatrix2x3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x3dv_reset_ptr() {
  *glUniformMatrix2x3dv_p.0.get() = None;
}
/// [glUniformMatrix2x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x3fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2x3fv_p.0.get() } {
    Some(glUniformMatrix2x3fv_inner) => glUniformMatrix2x3fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2x3fv"),
  }
}
static glUniformMatrix2x3fv_p: GlFnCell<glUniformMatrix2x3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2x3fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2x3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2x3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2x3fv_t>>(gl_ptr_filter(f(b"glUniformMatrix2x3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x3fv_reset_ptr() {
  *glUniformMatrix2x3fv_p.0.get() = None;
}
/// [glUniformMatrix2x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x4dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2x4dv_p.0.get() } {
    Some(glUniformMatrix2x4dv_inner) => glUniformMatrix2x4dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2x4dv"),
  }
}
static glUniformMatrix2x4dv_p: GlFnCell<glUniformMatrix2x4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2x4dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2x4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2x4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2x4dv_t>>(gl_ptr_filter(f(b"glUniformMatrix2x4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x4dv_reset_ptr() {
  *glUniformMatrix2x4dv_p.0.get() = None;
}
/// [glUniformMatrix2x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x4fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2x4fv_p.0.get() } {
    Some(glUniformMatrix2x4fv_inner) => glUniformMatrix2x4fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2x4fv"),
  }
}
static glUniformMatrix2x4fv_p: GlFnCell<glUniformMatrix2x4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2x4fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2x4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2x4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2x4fv_t>>(gl_ptr_filter(f(b"glUniformMatrix2x4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x4fv_reset_ptr() {
  *glUniformMatrix2x4fv_p.0.get() = None;
}
/// [glUniformMatrix3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*9
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3dv_p.0.get() } {
    Some(glUniformMatrix3dv_inner) => glUniformMatrix3dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3dv"),
  }
}
static glUniformMatrix3dv_p: GlFnCell<glUniformMatrix3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3dv_t>>(gl_ptr_filter(f(b"glUniformMatrix3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3dv_reset_ptr() {
  *glUniformMatrix3dv_p.0.get() = None;
}
/// [glUniformMatrix3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*9
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3fv_p.0.get() } {
    Some(glUniformMatrix3fv_inner) => glUniformMatrix3fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3fv"),
  }
}
static glUniformMatrix3fv_p: GlFnCell<glUniformMatrix3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3fv_t>>(gl_ptr_filter(f(b"glUniformMatrix3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3fv_reset_ptr() {
  *glUniformMatrix3fv_p.0.get() = None;
}
/// [glUniformMatrix3x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x2dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3x2dv_p.0.get() } {
    Some(glUniformMatrix3x2dv_inner) => glUniformMatrix3x2dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3x2dv"),
  }
}
static glUniformMatrix3x2dv_p: GlFnCell<glUniformMatrix3x2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3x2dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3x2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3x2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3x2dv_t>>(gl_ptr_filter(f(b"glUniformMatrix3x2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x2dv_reset_ptr() {
  *glUniformMatrix3x2dv_p.0.get() = None;
}
/// [glUniformMatrix3x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x2fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3x2fv_p.0.get() } {
    Some(glUniformMatrix3x2fv_inner) => glUniformMatrix3x2fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3x2fv"),
  }
}
static glUniformMatrix3x2fv_p: GlFnCell<glUniformMatrix3x2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3x2fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3x2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3x2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3x2fv_t>>(gl_ptr_filter(f(b"glUniformMatrix3x2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x2fv_reset_ptr() {
  *glUniformMatrix3x2fv_p.0.get() = None;
}
/// [glUniformMatrix3x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x4dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3x4dv_p.0.get() } {
    Some(glUniformMatrix3x4dv_inner) => glUniformMatrix3x4dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3x4dv"),
  }
}
static glUniformMatrix3x4dv_p: GlFnCell<glUniformMatrix3x4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3x4dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3x4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3x4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3x4dv_t>>(gl_ptr_filter(f(b"glUniformMatrix3x4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x4dv_reset_ptr() {
  *glUniformMatrix3x4dv_p.0.get() = None;
}
/// [glUniformMatrix3x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x4fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3x4fv_p.0.get() } {
    Some(glUniformMatrix3x4fv_inner) => glUniformMatrix3x4fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3x4fv"),
  }
}
static glUniformMatrix3x4fv_p: GlFnCell<glUniformMatrix3x4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3x4fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3x4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3x4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3x4fv_t>>(gl_ptr_filter(f(b"glUniformMatrix3x4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x4fv_reset_ptr() {
  *glUniformMatrix3x4fv_p.0.get() = None;
}
/// [glUniformMatrix4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*16
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4dv_p.0.get() } {
    Some(glUniformMatrix4dv_inner) => glUniformMatrix4dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4dv"),
  }
}
static glUniformMatrix4dv_p: GlFnCell<glUniformMatrix4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4dv_t>>(gl_ptr_filter(f(b"glUniformMatrix4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4dv_reset_ptr() {
  *glUniformMatrix4dv_p.0.get() = None;
}
/// [glUniformMatrix4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*16
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4fv_p.0.get() } {
    Some(glUniformMatrix4fv_inner) => glUniformMatrix4fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4fv"),
  }
}
static glUniformMatrix4fv_p: GlFnCell<glUniformMatrix4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4fv_t>>(gl_ptr_filter(f(b"glUniformMatrix4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4fv_reset_ptr() {
  *glUniformMatrix4fv_p.0.get() = None;
}
/// [glUniformMatrix4x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x2dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4x2dv_p.0.get() } {
    Some(glUniformMatrix4x2dv_inner) => glUniformMatrix4x2dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4x2dv"),
  }
}
static glUniformMatrix4x2dv_p: GlFnCell<glUniformMatrix4x2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4x2dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4x2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4x2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4x2dv_t>>(gl_ptr_filter(f(b"glUniformMatrix4x2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x2dv_reset_ptr() {
  *glUniformMatrix4x2dv_p.0.get() = None;
}
/// [glUniformMatrix4x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x2fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4x2fv_p.0.get() } {
    Some(glUniformMatrix4x2fv_inner) => glUniformMatrix4x2fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4x2fv"),
  }
}
static glUniformMatrix4x2fv_p: GlFnCell<glUniformMatrix4x2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4x2fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4x2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4x2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4x2fv_t>>(gl_ptr_filter(f(b"glUniformMatrix4x2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x2fv_reset_ptr() {
  *glUniformMatrix4x2fv_p.0.get() = None;
}
/// [glUniformMatrix4x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x3dv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4x3dv_p.0.get() } {
    Some(glUniformMatrix4x3dv_inner) => glUniformMatrix4x3dv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4x3dv"),
  }
}
static glUniformMatrix4x3dv_p: GlFnCell<glUniformMatrix4x3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4x3dv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4x3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4x3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4x3dv_t>>(gl_ptr_filter(f(b"glUniformMatrix4x3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x3dv_reset_ptr() {
  *glUniformMatrix4x3dv_p.0.get() = None;
}
/// [glUniformMatrix4x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x3fv.xhtml)
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4x3fv_p.0.get() } {
    Some(glUniformMatrix4x3fv_inner) => glUniformMatrix4x3fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4x3fv"),
  }
}
static glUniformMatrix4x3fv_p: GlFnCell<glUniformMatrix4x3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4x3fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4x3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4x3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4x3fv_t>>(gl_ptr_filter(f(b"glUniformMatrix4x3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x3fv_reset_ptr() {
  *glUniformMatrix4x3fv_p.0.get() = None;
}
/// [glUniformSubroutinesuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformSubroutinesuiv.xhtml)
/// * `shadertype` group: ShaderType
/// * `indices` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformSubroutinesuiv(shadertype: ShaderType, count: GLsizei, indices: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformSubroutinesuiv_p.0.get() } {
    Some(glUniformSubroutinesuiv_inner) => glUniformSubroutinesuiv_inner(shadertype, count, indices),
    None => gl_not_loaded("glUniformSubroutinesuiv"),
  }
}
static glUniformSubroutinesuiv_p: GlFnCell<glUniformSubroutinesuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformSubroutinesuiv_is_loaded() -> bool {
  unsafe { *glUniformSubroutinesuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformSubroutinesuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformSubroutinesuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformSubroutinesuiv_t>>(gl_ptr_filter(f(b"glUniformSubroutinesuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUniformSubroutinesuiv_reset_ptr() {
  *glUniformSubroutinesuiv_p.0.get() = None;
}
/// [glUnmapBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapBuffer.xhtml)
/// * `target` group: BufferTargetARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUnmapBuffer(target: BufferTargetARB) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glUnmapBuffer_p.0.get() } {
    Some(glUnmapBuffer_inner) => glUnmapBuffer_inner(target),
    None => gl_not_loaded("glUnmapBuffer"),
  }
}
static glUnmapBuffer_p: GlFnCell<glUnmapBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUnmapBuffer_is_loaded() -> bool {
  unsafe { *glUnmapBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUnmapBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUnmapBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUnmapBuffer_t>>(gl_ptr_filter(f(b"glUnmapBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUnmapBuffer_reset_ptr() {
  *glUnmapBuffer_p.0.get() = None;
}
/// [glUnmapNamedBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapNamedBuffer.xhtml)
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUnmapNamedBuffer(buffer: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glUnmapNamedBuffer_p.0.get() } {
    Some(glUnmapNamedBuffer_inner) => glUnmapNamedBuffer_inner(buffer),
    None => gl_not_loaded("glUnmapNamedBuffer"),
  }
}
static glUnmapNamedBuffer_p: GlFnCell<glUnmapNamedBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUnmapNamedBuffer_is_loaded() -> bool {
  unsafe { *glUnmapNamedBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUnmapNamedBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUnmapNamedBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUnmapNamedBuffer_t>>(gl_ptr_filter(f(b"glUnmapNamedBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUnmapNamedBuffer_reset_ptr() {
  *glUnmapNamedBuffer_p.0.get() = None;
}
/// [glUseProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUseProgram(program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUseProgram_p.0.get() } {
    Some(glUseProgram_inner) => glUseProgram_inner(program),
    None => gl_not_loaded("glUseProgram"),
  }
}
static glUseProgram_p: GlFnCell<glUseProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUseProgram_is_loaded() -> bool {
  unsafe { *glUseProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUseProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUseProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUseProgram_t>>(gl_ptr_filter(f(b"glUseProgram\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUseProgram_reset_ptr() {
  *glUseProgram_p.0.get() = None;
}
/// [glUseProgramStages](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgramStages.xhtml)
/// * `pipeline` class: program pipeline
/// * `stages` group: UseProgramStageMask
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUseProgramStages_p.0.get() } {
    Some(glUseProgramStages_inner) => glUseProgramStages_inner(pipeline, stages, program),
    None => gl_not_loaded("glUseProgramStages"),
  }
}
static glUseProgramStages_p: GlFnCell<glUseProgramStages_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUseProgramStages_is_loaded() -> bool {
  unsafe { *glUseProgramStages_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUseProgramStages_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUseProgramStages_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUseProgramStages_t>>(gl_ptr_filter(f(b"glUseProgramStages\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glUseProgramStages_reset_ptr() {
  *glUseProgramStages_p.0.get() = None;
}
/// [glValidateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glValidateProgram.xhtml)
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glValidateProgram(program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glValidateProgram_p.0.get() } {
    Some(glValidateProgram_inner) => glValidateProgram_inner(program),
    None => gl_not_loaded("glValidateProgram"),
  }
}
static glValidateProgram_p: GlFnCell<glValidateProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glValidateProgram_is_loaded() -> bool {
  unsafe { *glValidateProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glValidateProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glValidateProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glValidateProgram_t>>(gl_ptr_filter(f(b"glValidateProgram\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glValidateProgram_reset_ptr() {
  *glValidateProgram_p.0.get() = None;
}
/// [glValidateProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glValidateProgramPipeline.xhtml)
/// * `pipeline` class: program pipeline
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glValidateProgramPipeline(pipeline: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glValidateProgramPipeline_p.0.get() } {
    Some(glValidateProgramPipeline_inner) => glValidateProgramPipeline_inner(pipeline),
    None => gl_not_loaded("glValidateProgramPipeline"),
  }
}
static glValidateProgramPipeline_p: GlFnCell<glValidateProgramPipeline_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glValidateProgramPipeline_is_loaded() -> bool {
  unsafe { *glValidateProgramPipeline_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glValidateProgramPipeline_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glValidateProgramPipeline_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glValidateProgramPipeline_t>>(gl_ptr_filter(f(b"glValidateProgramPipeline\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glValidateProgramPipeline_reset_ptr() {
  *glValidateProgramPipeline_p.0.get() = None;
}
/// [glVertexArrayAttribBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribBinding.xhtml)
/// * `vaobj` class: vertex array
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexArrayAttribBinding_p.0.get() } {
    Some(glVertexArrayAttribBinding_inner) => glVertexArrayAttribBinding_inner(vaobj, attribindex, bindingindex),
    None => gl_not_loaded("glVertexArrayAttribBinding"),
  }
}
static glVertexArrayAttribBinding_p: GlFnCell<glVertexArrayAttribBinding_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexArrayAttribBinding_is_loaded() -> bool {
  unsafe { *glVertexArrayAttribBinding_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexArrayAttribBinding_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexArrayAttribBinding_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexArrayAttribBinding_t>>(gl_ptr_filter(f(b"glVertexArrayAttribBinding\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexArrayAttribBinding_reset_ptr() {
  *glVertexArrayAttribBinding_p.0.get() = None;
}
/// [glVertexArrayAttribFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribFormat.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexArrayAttribFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexArrayAttribFormat_p.0.get() } {
    Some(glVertexArrayAttribFormat_inner) => glVertexArrayAttribFormat_inner(vaobj, attribindex, size, type_, normalized, relativeoffset),
    None => gl_not_loaded("glVertexArrayAttribFormat"),
  }
}
static glVertexArrayAttribFormat_p: GlFnCell<glVertexArrayAttribFormat_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexArrayAttribFormat_is_loaded() -> bool {
  unsafe { *glVertexArrayAttribFormat_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexArrayAttribFormat_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexArrayAttribFormat_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexArrayAttribFormat_t>>(gl_ptr_filter(f(b"glVertexArrayAttribFormat\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexArrayAttribFormat_reset_ptr() {
  *glVertexArrayAttribFormat_p.0.get() = None;
}
/// [glVertexArrayAttribIFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribIFormat.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribIType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexArrayAttribIFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexArrayAttribIFormat_p.0.get() } {
    Some(glVertexArrayAttribIFormat_inner) => glVertexArrayAttribIFormat_inner(vaobj, attribindex, size, type_, relativeoffset),
    None => gl_not_loaded("glVertexArrayAttribIFormat"),
  }
}
static glVertexArrayAttribIFormat_p: GlFnCell<glVertexArrayAttribIFormat_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexArrayAttribIFormat_is_loaded() -> bool {
  unsafe { *glVertexArrayAttribIFormat_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexArrayAttribIFormat_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexArrayAttribIFormat_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexArrayAttribIFormat_t>>(gl_ptr_filter(f(b"glVertexArrayAttribIFormat\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexArrayAttribIFormat_reset_ptr() {
  *glVertexArrayAttribIFormat_p.0.get() = None;
}
/// [glVertexArrayAttribLFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribLFormat.xhtml)
/// * `vaobj` class: vertex array
/// * `type` group: VertexAttribLType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexArrayAttribLFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexArrayAttribLFormat_p.0.get() } {
    Some(glVertexArrayAttribLFormat_inner) => glVertexArrayAttribLFormat_inner(vaobj, attribindex, size, type_, relativeoffset),
    None => gl_not_loaded("glVertexArrayAttribLFormat"),
  }
}
static glVertexArrayAttribLFormat_p: GlFnCell<glVertexArrayAttribLFormat_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexArrayAttribLFormat_is_loaded() -> bool {
  unsafe { *glVertexArrayAttribLFormat_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexArrayAttribLFormat_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexArrayAttribLFormat_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexArrayAttribLFormat_t>>(gl_ptr_filter(f(b"glVertexArrayAttribLFormat\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexArrayAttribLFormat_reset_ptr() {
  *glVertexArrayAttribLFormat_p.0.get() = None;
}
/// [glVertexArrayBindingDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayBindingDivisor.xhtml)
/// * `vaobj` class: vertex array
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexArrayBindingDivisor(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexArrayBindingDivisor_p.0.get() } {
    Some(glVertexArrayBindingDivisor_inner) => glVertexArrayBindingDivisor_inner(vaobj, bindingindex, divisor),
    None => gl_not_loaded("glVertexArrayBindingDivisor"),
  }
}
static glVertexArrayBindingDivisor_p: GlFnCell<glVertexArrayBindingDivisor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexArrayBindingDivisor_is_loaded() -> bool {
  unsafe { *glVertexArrayBindingDivisor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexArrayBindingDivisor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexArrayBindingDivisor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexArrayBindingDivisor_t>>(gl_ptr_filter(f(b"glVertexArrayBindingDivisor\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexArrayBindingDivisor_reset_ptr() {
  *glVertexArrayBindingDivisor_p.0.get() = None;
}
/// [glVertexArrayElementBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayElementBuffer.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexArrayElementBuffer_p.0.get() } {
    Some(glVertexArrayElementBuffer_inner) => glVertexArrayElementBuffer_inner(vaobj, buffer),
    None => gl_not_loaded("glVertexArrayElementBuffer"),
  }
}
static glVertexArrayElementBuffer_p: GlFnCell<glVertexArrayElementBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexArrayElementBuffer_is_loaded() -> bool {
  unsafe { *glVertexArrayElementBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexArrayElementBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexArrayElementBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexArrayElementBuffer_t>>(gl_ptr_filter(f(b"glVertexArrayElementBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexArrayElementBuffer_reset_ptr() {
  *glVertexArrayElementBuffer_p.0.get() = None;
}
/// [glVertexArrayVertexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexBuffer.xhtml)
/// * `vaobj` class: vertex array
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexArrayVertexBuffer(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexArrayVertexBuffer_p.0.get() } {
    Some(glVertexArrayVertexBuffer_inner) => glVertexArrayVertexBuffer_inner(vaobj, bindingindex, buffer, offset, stride),
    None => gl_not_loaded("glVertexArrayVertexBuffer"),
  }
}
static glVertexArrayVertexBuffer_p: GlFnCell<glVertexArrayVertexBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexArrayVertexBuffer_is_loaded() -> bool {
  unsafe { *glVertexArrayVertexBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexArrayVertexBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexArrayVertexBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexArrayVertexBuffer_t>>(gl_ptr_filter(f(b"glVertexArrayVertexBuffer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexArrayVertexBuffer_reset_ptr() {
  *glVertexArrayVertexBuffer_p.0.get() = None;
}
/// [glVertexArrayVertexBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexBuffers.xhtml)
/// * `vaobj` class: vertex array
/// * `buffers` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexArrayVertexBuffers(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexArrayVertexBuffers_p.0.get() } {
    Some(glVertexArrayVertexBuffers_inner) => glVertexArrayVertexBuffers_inner(vaobj, first, count, buffers, offsets, strides),
    None => gl_not_loaded("glVertexArrayVertexBuffers"),
  }
}
static glVertexArrayVertexBuffers_p: GlFnCell<glVertexArrayVertexBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexArrayVertexBuffers_is_loaded() -> bool {
  unsafe { *glVertexArrayVertexBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexArrayVertexBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexArrayVertexBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexArrayVertexBuffers_t>>(gl_ptr_filter(f(b"glVertexArrayVertexBuffers\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexArrayVertexBuffers_reset_ptr() {
  *glVertexArrayVertexBuffers_p.0.get() = None;
}
/// [glVertexAttrib1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1d(index: GLuint, x: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1d_p.0.get() } {
    Some(glVertexAttrib1d_inner) => glVertexAttrib1d_inner(index, x),
    None => gl_not_loaded("glVertexAttrib1d"),
  }
}
static glVertexAttrib1d_p: GlFnCell<glVertexAttrib1d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1d_is_loaded() -> bool {
  unsafe { *glVertexAttrib1d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1d_t>>(gl_ptr_filter(f(b"glVertexAttrib1d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1d_reset_ptr() {
  *glVertexAttrib1d_p.0.get() = None;
}
/// [glVertexAttrib1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1dv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1dv(index: GLuint, v: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1dv_p.0.get() } {
    Some(glVertexAttrib1dv_inner) => glVertexAttrib1dv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib1dv"),
  }
}
static glVertexAttrib1dv_p: GlFnCell<glVertexAttrib1dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1dv_is_loaded() -> bool {
  unsafe { *glVertexAttrib1dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1dv_t>>(gl_ptr_filter(f(b"glVertexAttrib1dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1dv_reset_ptr() {
  *glVertexAttrib1dv_p.0.get() = None;
}
/// [glVertexAttrib1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1f.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1f(index: GLuint, x: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1f_p.0.get() } {
    Some(glVertexAttrib1f_inner) => glVertexAttrib1f_inner(index, x),
    None => gl_not_loaded("glVertexAttrib1f"),
  }
}
static glVertexAttrib1f_p: GlFnCell<glVertexAttrib1f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1f_is_loaded() -> bool {
  unsafe { *glVertexAttrib1f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1f_t>>(gl_ptr_filter(f(b"glVertexAttrib1f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1f_reset_ptr() {
  *glVertexAttrib1f_p.0.get() = None;
}
/// [glVertexAttrib1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1fv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1fv_p.0.get() } {
    Some(glVertexAttrib1fv_inner) => glVertexAttrib1fv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib1fv"),
  }
}
static glVertexAttrib1fv_p: GlFnCell<glVertexAttrib1fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1fv_is_loaded() -> bool {
  unsafe { *glVertexAttrib1fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1fv_t>>(gl_ptr_filter(f(b"glVertexAttrib1fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1fv_reset_ptr() {
  *glVertexAttrib1fv_p.0.get() = None;
}
/// [glVertexAttrib1s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1s.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1s(index: GLuint, x: GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1s_p.0.get() } {
    Some(glVertexAttrib1s_inner) => glVertexAttrib1s_inner(index, x),
    None => gl_not_loaded("glVertexAttrib1s"),
  }
}
static glVertexAttrib1s_p: GlFnCell<glVertexAttrib1s_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1s_is_loaded() -> bool {
  unsafe { *glVertexAttrib1s_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1s_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1s_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1s_t>>(gl_ptr_filter(f(b"glVertexAttrib1s\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1s_reset_ptr() {
  *glVertexAttrib1s_p.0.get() = None;
}
/// [glVertexAttrib1sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1sv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1sv(index: GLuint, v: *const GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1sv_p.0.get() } {
    Some(glVertexAttrib1sv_inner) => glVertexAttrib1sv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib1sv"),
  }
}
static glVertexAttrib1sv_p: GlFnCell<glVertexAttrib1sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1sv_is_loaded() -> bool {
  unsafe { *glVertexAttrib1sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1sv_t>>(gl_ptr_filter(f(b"glVertexAttrib1sv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1sv_reset_ptr() {
  *glVertexAttrib1sv_p.0.get() = None;
}
/// [glVertexAttrib2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2d_p.0.get() } {
    Some(glVertexAttrib2d_inner) => glVertexAttrib2d_inner(index, x, y),
    None => gl_not_loaded("glVertexAttrib2d"),
  }
}
static glVertexAttrib2d_p: GlFnCell<glVertexAttrib2d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2d_is_loaded() -> bool {
  unsafe { *glVertexAttrib2d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2d_t>>(gl_ptr_filter(f(b"glVertexAttrib2d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2d_reset_ptr() {
  *glVertexAttrib2d_p.0.get() = None;
}
/// [glVertexAttrib2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2dv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2dv(index: GLuint, v: *const [GLdouble; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2dv_p.0.get() } {
    Some(glVertexAttrib2dv_inner) => glVertexAttrib2dv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib2dv"),
  }
}
static glVertexAttrib2dv_p: GlFnCell<glVertexAttrib2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2dv_is_loaded() -> bool {
  unsafe { *glVertexAttrib2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2dv_t>>(gl_ptr_filter(f(b"glVertexAttrib2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2dv_reset_ptr() {
  *glVertexAttrib2dv_p.0.get() = None;
}
/// [glVertexAttrib2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2f.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2f_p.0.get() } {
    Some(glVertexAttrib2f_inner) => glVertexAttrib2f_inner(index, x, y),
    None => gl_not_loaded("glVertexAttrib2f"),
  }
}
static glVertexAttrib2f_p: GlFnCell<glVertexAttrib2f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2f_is_loaded() -> bool {
  unsafe { *glVertexAttrib2f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2f_t>>(gl_ptr_filter(f(b"glVertexAttrib2f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2f_reset_ptr() {
  *glVertexAttrib2f_p.0.get() = None;
}
/// [glVertexAttrib2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2fv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2fv(index: GLuint, v: *const [GLfloat; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2fv_p.0.get() } {
    Some(glVertexAttrib2fv_inner) => glVertexAttrib2fv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib2fv"),
  }
}
static glVertexAttrib2fv_p: GlFnCell<glVertexAttrib2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2fv_is_loaded() -> bool {
  unsafe { *glVertexAttrib2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2fv_t>>(gl_ptr_filter(f(b"glVertexAttrib2fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2fv_reset_ptr() {
  *glVertexAttrib2fv_p.0.get() = None;
}
/// [glVertexAttrib2s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2s.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2s_p.0.get() } {
    Some(glVertexAttrib2s_inner) => glVertexAttrib2s_inner(index, x, y),
    None => gl_not_loaded("glVertexAttrib2s"),
  }
}
static glVertexAttrib2s_p: GlFnCell<glVertexAttrib2s_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2s_is_loaded() -> bool {
  unsafe { *glVertexAttrib2s_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2s_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2s_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2s_t>>(gl_ptr_filter(f(b"glVertexAttrib2s\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2s_reset_ptr() {
  *glVertexAttrib2s_p.0.get() = None;
}
/// [glVertexAttrib2sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2sv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2sv(index: GLuint, v: *const [GLshort; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2sv_p.0.get() } {
    Some(glVertexAttrib2sv_inner) => glVertexAttrib2sv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib2sv"),
  }
}
static glVertexAttrib2sv_p: GlFnCell<glVertexAttrib2sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2sv_is_loaded() -> bool {
  unsafe { *glVertexAttrib2sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2sv_t>>(gl_ptr_filter(f(b"glVertexAttrib2sv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2sv_reset_ptr() {
  *glVertexAttrib2sv_p.0.get() = None;
}
/// [glVertexAttrib3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3d_p.0.get() } {
    Some(glVertexAttrib3d_inner) => glVertexAttrib3d_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttrib3d"),
  }
}
static glVertexAttrib3d_p: GlFnCell<glVertexAttrib3d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3d_is_loaded() -> bool {
  unsafe { *glVertexAttrib3d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3d_t>>(gl_ptr_filter(f(b"glVertexAttrib3d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3d_reset_ptr() {
  *glVertexAttrib3d_p.0.get() = None;
}
/// [glVertexAttrib3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3dv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3dv(index: GLuint, v: *const [GLdouble; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3dv_p.0.get() } {
    Some(glVertexAttrib3dv_inner) => glVertexAttrib3dv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib3dv"),
  }
}
static glVertexAttrib3dv_p: GlFnCell<glVertexAttrib3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3dv_is_loaded() -> bool {
  unsafe { *glVertexAttrib3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3dv_t>>(gl_ptr_filter(f(b"glVertexAttrib3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3dv_reset_ptr() {
  *glVertexAttrib3dv_p.0.get() = None;
}
/// [glVertexAttrib3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3f.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3f_p.0.get() } {
    Some(glVertexAttrib3f_inner) => glVertexAttrib3f_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttrib3f"),
  }
}
static glVertexAttrib3f_p: GlFnCell<glVertexAttrib3f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3f_is_loaded() -> bool {
  unsafe { *glVertexAttrib3f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3f_t>>(gl_ptr_filter(f(b"glVertexAttrib3f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3f_reset_ptr() {
  *glVertexAttrib3f_p.0.get() = None;
}
/// [glVertexAttrib3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3fv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3fv(index: GLuint, v: *const [GLfloat; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3fv_p.0.get() } {
    Some(glVertexAttrib3fv_inner) => glVertexAttrib3fv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib3fv"),
  }
}
static glVertexAttrib3fv_p: GlFnCell<glVertexAttrib3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3fv_is_loaded() -> bool {
  unsafe { *glVertexAttrib3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3fv_t>>(gl_ptr_filter(f(b"glVertexAttrib3fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3fv_reset_ptr() {
  *glVertexAttrib3fv_p.0.get() = None;
}
/// [glVertexAttrib3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3s.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3s_p.0.get() } {
    Some(glVertexAttrib3s_inner) => glVertexAttrib3s_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttrib3s"),
  }
}
static glVertexAttrib3s_p: GlFnCell<glVertexAttrib3s_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3s_is_loaded() -> bool {
  unsafe { *glVertexAttrib3s_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3s_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3s_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3s_t>>(gl_ptr_filter(f(b"glVertexAttrib3s\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3s_reset_ptr() {
  *glVertexAttrib3s_p.0.get() = None;
}
/// [glVertexAttrib3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3sv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3sv(index: GLuint, v: *const [GLshort; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3sv_p.0.get() } {
    Some(glVertexAttrib3sv_inner) => glVertexAttrib3sv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib3sv"),
  }
}
static glVertexAttrib3sv_p: GlFnCell<glVertexAttrib3sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3sv_is_loaded() -> bool {
  unsafe { *glVertexAttrib3sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3sv_t>>(gl_ptr_filter(f(b"glVertexAttrib3sv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3sv_reset_ptr() {
  *glVertexAttrib3sv_p.0.get() = None;
}
/// [glVertexAttrib4Nbv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nbv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nbv(index: GLuint, v: *const [GLbyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nbv_p.0.get() } {
    Some(glVertexAttrib4Nbv_inner) => glVertexAttrib4Nbv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nbv"),
  }
}
static glVertexAttrib4Nbv_p: GlFnCell<glVertexAttrib4Nbv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nbv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nbv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nbv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nbv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nbv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nbv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nbv_reset_ptr() {
  *glVertexAttrib4Nbv_p.0.get() = None;
}
/// [glVertexAttrib4Niv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Niv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Niv(index: GLuint, v: *const [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Niv_p.0.get() } {
    Some(glVertexAttrib4Niv_inner) => glVertexAttrib4Niv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Niv"),
  }
}
static glVertexAttrib4Niv_p: GlFnCell<glVertexAttrib4Niv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Niv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Niv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Niv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Niv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Niv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Niv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Niv_reset_ptr() {
  *glVertexAttrib4Niv_p.0.get() = None;
}
/// [glVertexAttrib4Nsv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nsv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nsv(index: GLuint, v: *const [GLshort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nsv_p.0.get() } {
    Some(glVertexAttrib4Nsv_inner) => glVertexAttrib4Nsv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nsv"),
  }
}
static glVertexAttrib4Nsv_p: GlFnCell<glVertexAttrib4Nsv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nsv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nsv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nsv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nsv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nsv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nsv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nsv_reset_ptr() {
  *glVertexAttrib4Nsv_p.0.get() = None;
}
/// [glVertexAttrib4Nub](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nub.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nub_p.0.get() } {
    Some(glVertexAttrib4Nub_inner) => glVertexAttrib4Nub_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttrib4Nub"),
  }
}
static glVertexAttrib4Nub_p: GlFnCell<glVertexAttrib4Nub_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nub_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nub_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nub_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nub_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nub_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nub\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nub_reset_ptr() {
  *glVertexAttrib4Nub_p.0.get() = None;
}
/// [glVertexAttrib4Nubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nubv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nubv(index: GLuint, v: *const [GLubyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nubv_p.0.get() } {
    Some(glVertexAttrib4Nubv_inner) => glVertexAttrib4Nubv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nubv"),
  }
}
static glVertexAttrib4Nubv_p: GlFnCell<glVertexAttrib4Nubv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nubv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nubv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nubv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nubv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nubv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nubv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nubv_reset_ptr() {
  *glVertexAttrib4Nubv_p.0.get() = None;
}
/// [glVertexAttrib4Nuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nuiv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nuiv(index: GLuint, v: *const [GLuint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nuiv_p.0.get() } {
    Some(glVertexAttrib4Nuiv_inner) => glVertexAttrib4Nuiv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nuiv"),
  }
}
static glVertexAttrib4Nuiv_p: GlFnCell<glVertexAttrib4Nuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nuiv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nuiv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nuiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nuiv_reset_ptr() {
  *glVertexAttrib4Nuiv_p.0.get() = None;
}
/// [glVertexAttrib4Nusv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nusv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nusv(index: GLuint, v: *const [GLushort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nusv_p.0.get() } {
    Some(glVertexAttrib4Nusv_inner) => glVertexAttrib4Nusv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nusv"),
  }
}
static glVertexAttrib4Nusv_p: GlFnCell<glVertexAttrib4Nusv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nusv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nusv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nusv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nusv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nusv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nusv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nusv_reset_ptr() {
  *glVertexAttrib4Nusv_p.0.get() = None;
}
/// [glVertexAttrib4bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4bv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4bv(index: GLuint, v: *const [GLbyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4bv_p.0.get() } {
    Some(glVertexAttrib4bv_inner) => glVertexAttrib4bv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4bv"),
  }
}
static glVertexAttrib4bv_p: GlFnCell<glVertexAttrib4bv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4bv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4bv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4bv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4bv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4bv_t>>(gl_ptr_filter(f(b"glVertexAttrib4bv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4bv_reset_ptr() {
  *glVertexAttrib4bv_p.0.get() = None;
}
/// [glVertexAttrib4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4d_p.0.get() } {
    Some(glVertexAttrib4d_inner) => glVertexAttrib4d_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttrib4d"),
  }
}
static glVertexAttrib4d_p: GlFnCell<glVertexAttrib4d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4d_is_loaded() -> bool {
  unsafe { *glVertexAttrib4d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4d_t>>(gl_ptr_filter(f(b"glVertexAttrib4d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4d_reset_ptr() {
  *glVertexAttrib4d_p.0.get() = None;
}
/// [glVertexAttrib4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4dv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4dv(index: GLuint, v: *const [GLdouble; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4dv_p.0.get() } {
    Some(glVertexAttrib4dv_inner) => glVertexAttrib4dv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4dv"),
  }
}
static glVertexAttrib4dv_p: GlFnCell<glVertexAttrib4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4dv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4dv_t>>(gl_ptr_filter(f(b"glVertexAttrib4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4dv_reset_ptr() {
  *glVertexAttrib4dv_p.0.get() = None;
}
/// [glVertexAttrib4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4f.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4f_p.0.get() } {
    Some(glVertexAttrib4f_inner) => glVertexAttrib4f_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttrib4f"),
  }
}
static glVertexAttrib4f_p: GlFnCell<glVertexAttrib4f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4f_is_loaded() -> bool {
  unsafe { *glVertexAttrib4f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4f_t>>(gl_ptr_filter(f(b"glVertexAttrib4f\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4f_reset_ptr() {
  *glVertexAttrib4f_p.0.get() = None;
}
/// [glVertexAttrib4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4fv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4fv(index: GLuint, v: *const [GLfloat; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4fv_p.0.get() } {
    Some(glVertexAttrib4fv_inner) => glVertexAttrib4fv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4fv"),
  }
}
static glVertexAttrib4fv_p: GlFnCell<glVertexAttrib4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4fv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4fv_t>>(gl_ptr_filter(f(b"glVertexAttrib4fv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4fv_reset_ptr() {
  *glVertexAttrib4fv_p.0.get() = None;
}
/// [glVertexAttrib4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4iv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4iv(index: GLuint, v: *const [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4iv_p.0.get() } {
    Some(glVertexAttrib4iv_inner) => glVertexAttrib4iv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4iv"),
  }
}
static glVertexAttrib4iv_p: GlFnCell<glVertexAttrib4iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4iv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4iv_t>>(gl_ptr_filter(f(b"glVertexAttrib4iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4iv_reset_ptr() {
  *glVertexAttrib4iv_p.0.get() = None;
}
/// [glVertexAttrib4s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4s.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4s_p.0.get() } {
    Some(glVertexAttrib4s_inner) => glVertexAttrib4s_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttrib4s"),
  }
}
static glVertexAttrib4s_p: GlFnCell<glVertexAttrib4s_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4s_is_loaded() -> bool {
  unsafe { *glVertexAttrib4s_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4s_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4s_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4s_t>>(gl_ptr_filter(f(b"glVertexAttrib4s\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4s_reset_ptr() {
  *glVertexAttrib4s_p.0.get() = None;
}
/// [glVertexAttrib4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4sv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4sv(index: GLuint, v: *const [GLshort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4sv_p.0.get() } {
    Some(glVertexAttrib4sv_inner) => glVertexAttrib4sv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4sv"),
  }
}
static glVertexAttrib4sv_p: GlFnCell<glVertexAttrib4sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4sv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4sv_t>>(gl_ptr_filter(f(b"glVertexAttrib4sv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4sv_reset_ptr() {
  *glVertexAttrib4sv_p.0.get() = None;
}
/// [glVertexAttrib4ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4ubv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4ubv(index: GLuint, v: *const [GLubyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4ubv_p.0.get() } {
    Some(glVertexAttrib4ubv_inner) => glVertexAttrib4ubv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4ubv"),
  }
}
static glVertexAttrib4ubv_p: GlFnCell<glVertexAttrib4ubv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4ubv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4ubv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4ubv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4ubv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4ubv_t>>(gl_ptr_filter(f(b"glVertexAttrib4ubv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4ubv_reset_ptr() {
  *glVertexAttrib4ubv_p.0.get() = None;
}
/// [glVertexAttrib4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4uiv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4uiv(index: GLuint, v: *const [GLuint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4uiv_p.0.get() } {
    Some(glVertexAttrib4uiv_inner) => glVertexAttrib4uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4uiv"),
  }
}
static glVertexAttrib4uiv_p: GlFnCell<glVertexAttrib4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4uiv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4uiv_t>>(gl_ptr_filter(f(b"glVertexAttrib4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4uiv_reset_ptr() {
  *glVertexAttrib4uiv_p.0.get() = None;
}
/// [glVertexAttrib4usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4usv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4usv(index: GLuint, v: *const [GLushort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4usv_p.0.get() } {
    Some(glVertexAttrib4usv_inner) => glVertexAttrib4usv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4usv"),
  }
}
static glVertexAttrib4usv_p: GlFnCell<glVertexAttrib4usv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4usv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4usv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4usv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4usv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4usv_t>>(gl_ptr_filter(f(b"glVertexAttrib4usv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4usv_reset_ptr() {
  *glVertexAttrib4usv_p.0.get() = None;
}
/// [glVertexAttribBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribBinding.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribBinding_p.0.get() } {
    Some(glVertexAttribBinding_inner) => glVertexAttribBinding_inner(attribindex, bindingindex),
    None => gl_not_loaded("glVertexAttribBinding"),
  }
}
static glVertexAttribBinding_p: GlFnCell<glVertexAttribBinding_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribBinding_is_loaded() -> bool {
  unsafe { *glVertexAttribBinding_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribBinding_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribBinding_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribBinding_t>>(gl_ptr_filter(f(b"glVertexAttribBinding\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribBinding_reset_ptr() {
  *glVertexAttribBinding_p.0.get() = None;
}
/// [glVertexAttribDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribDivisor.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribDivisor_p.0.get() } {
    Some(glVertexAttribDivisor_inner) => glVertexAttribDivisor_inner(index, divisor),
    None => gl_not_loaded("glVertexAttribDivisor"),
  }
}
static glVertexAttribDivisor_p: GlFnCell<glVertexAttribDivisor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribDivisor_is_loaded() -> bool {
  unsafe { *glVertexAttribDivisor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribDivisor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribDivisor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribDivisor_t>>(gl_ptr_filter(f(b"glVertexAttribDivisor\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribDivisor_reset_ptr() {
  *glVertexAttribDivisor_p.0.get() = None;
}
/// [glVertexAttribFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribFormat.xhtml)
/// * `type` group: VertexAttribType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribFormat(attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribFormat_p.0.get() } {
    Some(glVertexAttribFormat_inner) => glVertexAttribFormat_inner(attribindex, size, type_, normalized, relativeoffset),
    None => gl_not_loaded("glVertexAttribFormat"),
  }
}
static glVertexAttribFormat_p: GlFnCell<glVertexAttribFormat_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribFormat_is_loaded() -> bool {
  unsafe { *glVertexAttribFormat_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribFormat_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribFormat_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribFormat_t>>(gl_ptr_filter(f(b"glVertexAttribFormat\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribFormat_reset_ptr() {
  *glVertexAttribFormat_p.0.get() = None;
}
/// [glVertexAttribI1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1i.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI1i(index: GLuint, x: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI1i_p.0.get() } {
    Some(glVertexAttribI1i_inner) => glVertexAttribI1i_inner(index, x),
    None => gl_not_loaded("glVertexAttribI1i"),
  }
}
static glVertexAttribI1i_p: GlFnCell<glVertexAttribI1i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI1i_is_loaded() -> bool {
  unsafe { *glVertexAttribI1i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI1i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI1i_t>>(gl_ptr_filter(f(b"glVertexAttribI1i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1i_reset_ptr() {
  *glVertexAttribI1i_p.0.get() = None;
}
/// [glVertexAttribI1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1iv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI1iv(index: GLuint, v: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI1iv_p.0.get() } {
    Some(glVertexAttribI1iv_inner) => glVertexAttribI1iv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI1iv"),
  }
}
static glVertexAttribI1iv_p: GlFnCell<glVertexAttribI1iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI1iv_is_loaded() -> bool {
  unsafe { *glVertexAttribI1iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI1iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI1iv_t>>(gl_ptr_filter(f(b"glVertexAttribI1iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1iv_reset_ptr() {
  *glVertexAttribI1iv_p.0.get() = None;
}
/// [glVertexAttribI1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1ui.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI1ui(index: GLuint, x: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI1ui_p.0.get() } {
    Some(glVertexAttribI1ui_inner) => glVertexAttribI1ui_inner(index, x),
    None => gl_not_loaded("glVertexAttribI1ui"),
  }
}
static glVertexAttribI1ui_p: GlFnCell<glVertexAttribI1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI1ui_is_loaded() -> bool {
  unsafe { *glVertexAttribI1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI1ui_t>>(gl_ptr_filter(f(b"glVertexAttribI1ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1ui_reset_ptr() {
  *glVertexAttribI1ui_p.0.get() = None;
}
/// [glVertexAttribI1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1uiv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI1uiv(index: GLuint, v: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI1uiv_p.0.get() } {
    Some(glVertexAttribI1uiv_inner) => glVertexAttribI1uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI1uiv"),
  }
}
static glVertexAttribI1uiv_p: GlFnCell<glVertexAttribI1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI1uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribI1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI1uiv_t>>(gl_ptr_filter(f(b"glVertexAttribI1uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1uiv_reset_ptr() {
  *glVertexAttribI1uiv_p.0.get() = None;
}
/// [glVertexAttribI2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2i.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI2i(index: GLuint, x: GLint, y: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI2i_p.0.get() } {
    Some(glVertexAttribI2i_inner) => glVertexAttribI2i_inner(index, x, y),
    None => gl_not_loaded("glVertexAttribI2i"),
  }
}
static glVertexAttribI2i_p: GlFnCell<glVertexAttribI2i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI2i_is_loaded() -> bool {
  unsafe { *glVertexAttribI2i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI2i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI2i_t>>(gl_ptr_filter(f(b"glVertexAttribI2i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2i_reset_ptr() {
  *glVertexAttribI2i_p.0.get() = None;
}
/// [glVertexAttribI2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2iv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI2iv(index: GLuint, v: *const [GLint; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI2iv_p.0.get() } {
    Some(glVertexAttribI2iv_inner) => glVertexAttribI2iv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI2iv"),
  }
}
static glVertexAttribI2iv_p: GlFnCell<glVertexAttribI2iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI2iv_is_loaded() -> bool {
  unsafe { *glVertexAttribI2iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI2iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI2iv_t>>(gl_ptr_filter(f(b"glVertexAttribI2iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2iv_reset_ptr() {
  *glVertexAttribI2iv_p.0.get() = None;
}
/// [glVertexAttribI2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2ui.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI2ui_p.0.get() } {
    Some(glVertexAttribI2ui_inner) => glVertexAttribI2ui_inner(index, x, y),
    None => gl_not_loaded("glVertexAttribI2ui"),
  }
}
static glVertexAttribI2ui_p: GlFnCell<glVertexAttribI2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI2ui_is_loaded() -> bool {
  unsafe { *glVertexAttribI2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI2ui_t>>(gl_ptr_filter(f(b"glVertexAttribI2ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2ui_reset_ptr() {
  *glVertexAttribI2ui_p.0.get() = None;
}
/// [glVertexAttribI2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2uiv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI2uiv(index: GLuint, v: *const [GLuint; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI2uiv_p.0.get() } {
    Some(glVertexAttribI2uiv_inner) => glVertexAttribI2uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI2uiv"),
  }
}
static glVertexAttribI2uiv_p: GlFnCell<glVertexAttribI2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI2uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribI2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI2uiv_t>>(gl_ptr_filter(f(b"glVertexAttribI2uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2uiv_reset_ptr() {
  *glVertexAttribI2uiv_p.0.get() = None;
}
/// [glVertexAttribI3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3i.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI3i_p.0.get() } {
    Some(glVertexAttribI3i_inner) => glVertexAttribI3i_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttribI3i"),
  }
}
static glVertexAttribI3i_p: GlFnCell<glVertexAttribI3i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI3i_is_loaded() -> bool {
  unsafe { *glVertexAttribI3i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI3i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI3i_t>>(gl_ptr_filter(f(b"glVertexAttribI3i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3i_reset_ptr() {
  *glVertexAttribI3i_p.0.get() = None;
}
/// [glVertexAttribI3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3iv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI3iv(index: GLuint, v: *const [GLint; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI3iv_p.0.get() } {
    Some(glVertexAttribI3iv_inner) => glVertexAttribI3iv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI3iv"),
  }
}
static glVertexAttribI3iv_p: GlFnCell<glVertexAttribI3iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI3iv_is_loaded() -> bool {
  unsafe { *glVertexAttribI3iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI3iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI3iv_t>>(gl_ptr_filter(f(b"glVertexAttribI3iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3iv_reset_ptr() {
  *glVertexAttribI3iv_p.0.get() = None;
}
/// [glVertexAttribI3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3ui.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI3ui_p.0.get() } {
    Some(glVertexAttribI3ui_inner) => glVertexAttribI3ui_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttribI3ui"),
  }
}
static glVertexAttribI3ui_p: GlFnCell<glVertexAttribI3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI3ui_is_loaded() -> bool {
  unsafe { *glVertexAttribI3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI3ui_t>>(gl_ptr_filter(f(b"glVertexAttribI3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3ui_reset_ptr() {
  *glVertexAttribI3ui_p.0.get() = None;
}
/// [glVertexAttribI3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3uiv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI3uiv(index: GLuint, v: *const [GLuint; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI3uiv_p.0.get() } {
    Some(glVertexAttribI3uiv_inner) => glVertexAttribI3uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI3uiv"),
  }
}
static glVertexAttribI3uiv_p: GlFnCell<glVertexAttribI3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI3uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribI3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI3uiv_t>>(gl_ptr_filter(f(b"glVertexAttribI3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3uiv_reset_ptr() {
  *glVertexAttribI3uiv_p.0.get() = None;
}
/// [glVertexAttribI4bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4bv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4bv(index: GLuint, v: *const [GLbyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4bv_p.0.get() } {
    Some(glVertexAttribI4bv_inner) => glVertexAttribI4bv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4bv"),
  }
}
static glVertexAttribI4bv_p: GlFnCell<glVertexAttribI4bv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4bv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4bv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4bv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4bv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4bv_t>>(gl_ptr_filter(f(b"glVertexAttribI4bv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4bv_reset_ptr() {
  *glVertexAttribI4bv_p.0.get() = None;
}
/// [glVertexAttribI4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4i.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4i_p.0.get() } {
    Some(glVertexAttribI4i_inner) => glVertexAttribI4i_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttribI4i"),
  }
}
static glVertexAttribI4i_p: GlFnCell<glVertexAttribI4i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4i_is_loaded() -> bool {
  unsafe { *glVertexAttribI4i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4i_t>>(gl_ptr_filter(f(b"glVertexAttribI4i\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4i_reset_ptr() {
  *glVertexAttribI4i_p.0.get() = None;
}
/// [glVertexAttribI4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4iv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4iv(index: GLuint, v: *const [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4iv_p.0.get() } {
    Some(glVertexAttribI4iv_inner) => glVertexAttribI4iv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4iv"),
  }
}
static glVertexAttribI4iv_p: GlFnCell<glVertexAttribI4iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4iv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4iv_t>>(gl_ptr_filter(f(b"glVertexAttribI4iv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4iv_reset_ptr() {
  *glVertexAttribI4iv_p.0.get() = None;
}
/// [glVertexAttribI4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4sv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4sv(index: GLuint, v: *const [GLshort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4sv_p.0.get() } {
    Some(glVertexAttribI4sv_inner) => glVertexAttribI4sv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4sv"),
  }
}
static glVertexAttribI4sv_p: GlFnCell<glVertexAttribI4sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4sv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4sv_t>>(gl_ptr_filter(f(b"glVertexAttribI4sv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4sv_reset_ptr() {
  *glVertexAttribI4sv_p.0.get() = None;
}
/// [glVertexAttribI4ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4ubv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4ubv(index: GLuint, v: *const [GLubyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4ubv_p.0.get() } {
    Some(glVertexAttribI4ubv_inner) => glVertexAttribI4ubv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4ubv"),
  }
}
static glVertexAttribI4ubv_p: GlFnCell<glVertexAttribI4ubv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4ubv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4ubv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4ubv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4ubv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4ubv_t>>(gl_ptr_filter(f(b"glVertexAttribI4ubv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4ubv_reset_ptr() {
  *glVertexAttribI4ubv_p.0.get() = None;
}
/// [glVertexAttribI4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4ui.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4ui_p.0.get() } {
    Some(glVertexAttribI4ui_inner) => glVertexAttribI4ui_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttribI4ui"),
  }
}
static glVertexAttribI4ui_p: GlFnCell<glVertexAttribI4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4ui_is_loaded() -> bool {
  unsafe { *glVertexAttribI4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4ui_t>>(gl_ptr_filter(f(b"glVertexAttribI4ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4ui_reset_ptr() {
  *glVertexAttribI4ui_p.0.get() = None;
}
/// [glVertexAttribI4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4uiv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4uiv(index: GLuint, v: *const [GLuint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4uiv_p.0.get() } {
    Some(glVertexAttribI4uiv_inner) => glVertexAttribI4uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4uiv"),
  }
}
static glVertexAttribI4uiv_p: GlFnCell<glVertexAttribI4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4uiv_t>>(gl_ptr_filter(f(b"glVertexAttribI4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4uiv_reset_ptr() {
  *glVertexAttribI4uiv_p.0.get() = None;
}
/// [glVertexAttribI4usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4usv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4usv(index: GLuint, v: *const [GLushort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4usv_p.0.get() } {
    Some(glVertexAttribI4usv_inner) => glVertexAttribI4usv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4usv"),
  }
}
static glVertexAttribI4usv_p: GlFnCell<glVertexAttribI4usv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4usv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4usv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4usv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4usv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4usv_t>>(gl_ptr_filter(f(b"glVertexAttribI4usv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4usv_reset_ptr() {
  *glVertexAttribI4usv_p.0.get() = None;
}
/// [glVertexAttribIFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribIFormat.xhtml)
/// * `type` group: VertexAttribIType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribIFormat(attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribIFormat_p.0.get() } {
    Some(glVertexAttribIFormat_inner) => glVertexAttribIFormat_inner(attribindex, size, type_, relativeoffset),
    None => gl_not_loaded("glVertexAttribIFormat"),
  }
}
static glVertexAttribIFormat_p: GlFnCell<glVertexAttribIFormat_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribIFormat_is_loaded() -> bool {
  unsafe { *glVertexAttribIFormat_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribIFormat_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribIFormat_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribIFormat_t>>(gl_ptr_filter(f(b"glVertexAttribIFormat\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribIFormat_reset_ptr() {
  *glVertexAttribIFormat_p.0.get() = None;
}
/// [glVertexAttribIPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribIPointer.xhtml)
/// * `type` group: VertexAttribIType
/// * `pointer` len: COMPSIZE(size,type,stride)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribIPointer(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribIPointer_p.0.get() } {
    Some(glVertexAttribIPointer_inner) => glVertexAttribIPointer_inner(index, size, type_, stride, pointer),
    None => gl_not_loaded("glVertexAttribIPointer"),
  }
}
static glVertexAttribIPointer_p: GlFnCell<glVertexAttribIPointer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribIPointer_is_loaded() -> bool {
  unsafe { *glVertexAttribIPointer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribIPointer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribIPointer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribIPointer_t>>(gl_ptr_filter(f(b"glVertexAttribIPointer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribIPointer_reset_ptr() {
  *glVertexAttribIPointer_p.0.get() = None;
}
/// [glVertexAttribL1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribL1d(index: GLuint, x: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribL1d_p.0.get() } {
    Some(glVertexAttribL1d_inner) => glVertexAttribL1d_inner(index, x),
    None => gl_not_loaded("glVertexAttribL1d"),
  }
}
static glVertexAttribL1d_p: GlFnCell<glVertexAttribL1d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribL1d_is_loaded() -> bool {
  unsafe { *glVertexAttribL1d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL1d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribL1d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribL1d_t>>(gl_ptr_filter(f(b"glVertexAttribL1d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL1d_reset_ptr() {
  *glVertexAttribL1d_p.0.get() = None;
}
/// [glVertexAttribL1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1dv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribL1dv(index: GLuint, v: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribL1dv_p.0.get() } {
    Some(glVertexAttribL1dv_inner) => glVertexAttribL1dv_inner(index, v),
    None => gl_not_loaded("glVertexAttribL1dv"),
  }
}
static glVertexAttribL1dv_p: GlFnCell<glVertexAttribL1dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribL1dv_is_loaded() -> bool {
  unsafe { *glVertexAttribL1dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL1dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribL1dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribL1dv_t>>(gl_ptr_filter(f(b"glVertexAttribL1dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL1dv_reset_ptr() {
  *glVertexAttribL1dv_p.0.get() = None;
}
/// [glVertexAttribL2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribL2d_p.0.get() } {
    Some(glVertexAttribL2d_inner) => glVertexAttribL2d_inner(index, x, y),
    None => gl_not_loaded("glVertexAttribL2d"),
  }
}
static glVertexAttribL2d_p: GlFnCell<glVertexAttribL2d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribL2d_is_loaded() -> bool {
  unsafe { *glVertexAttribL2d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL2d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribL2d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribL2d_t>>(gl_ptr_filter(f(b"glVertexAttribL2d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL2d_reset_ptr() {
  *glVertexAttribL2d_p.0.get() = None;
}
/// [glVertexAttribL2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2dv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribL2dv(index: GLuint, v: *const [GLdouble; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribL2dv_p.0.get() } {
    Some(glVertexAttribL2dv_inner) => glVertexAttribL2dv_inner(index, v),
    None => gl_not_loaded("glVertexAttribL2dv"),
  }
}
static glVertexAttribL2dv_p: GlFnCell<glVertexAttribL2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribL2dv_is_loaded() -> bool {
  unsafe { *glVertexAttribL2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribL2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribL2dv_t>>(gl_ptr_filter(f(b"glVertexAttribL2dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL2dv_reset_ptr() {
  *glVertexAttribL2dv_p.0.get() = None;
}
/// [glVertexAttribL3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribL3d_p.0.get() } {
    Some(glVertexAttribL3d_inner) => glVertexAttribL3d_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttribL3d"),
  }
}
static glVertexAttribL3d_p: GlFnCell<glVertexAttribL3d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribL3d_is_loaded() -> bool {
  unsafe { *glVertexAttribL3d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL3d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribL3d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribL3d_t>>(gl_ptr_filter(f(b"glVertexAttribL3d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL3d_reset_ptr() {
  *glVertexAttribL3d_p.0.get() = None;
}
/// [glVertexAttribL3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3dv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribL3dv(index: GLuint, v: *const [GLdouble; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribL3dv_p.0.get() } {
    Some(glVertexAttribL3dv_inner) => glVertexAttribL3dv_inner(index, v),
    None => gl_not_loaded("glVertexAttribL3dv"),
  }
}
static glVertexAttribL3dv_p: GlFnCell<glVertexAttribL3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribL3dv_is_loaded() -> bool {
  unsafe { *glVertexAttribL3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribL3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribL3dv_t>>(gl_ptr_filter(f(b"glVertexAttribL3dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL3dv_reset_ptr() {
  *glVertexAttribL3dv_p.0.get() = None;
}
/// [glVertexAttribL4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4d.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribL4d_p.0.get() } {
    Some(glVertexAttribL4d_inner) => glVertexAttribL4d_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttribL4d"),
  }
}
static glVertexAttribL4d_p: GlFnCell<glVertexAttribL4d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribL4d_is_loaded() -> bool {
  unsafe { *glVertexAttribL4d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL4d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribL4d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribL4d_t>>(gl_ptr_filter(f(b"glVertexAttribL4d\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL4d_reset_ptr() {
  *glVertexAttribL4d_p.0.get() = None;
}
/// [glVertexAttribL4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4dv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribL4dv(index: GLuint, v: *const [GLdouble; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribL4dv_p.0.get() } {
    Some(glVertexAttribL4dv_inner) => glVertexAttribL4dv_inner(index, v),
    None => gl_not_loaded("glVertexAttribL4dv"),
  }
}
static glVertexAttribL4dv_p: GlFnCell<glVertexAttribL4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribL4dv_is_loaded() -> bool {
  unsafe { *glVertexAttribL4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribL4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribL4dv_t>>(gl_ptr_filter(f(b"glVertexAttribL4dv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribL4dv_reset_ptr() {
  *glVertexAttribL4dv_p.0.get() = None;
}
/// [glVertexAttribLFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribLFormat.xhtml)
/// * `type` group: VertexAttribLType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribLFormat(attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribLFormat_p.0.get() } {
    Some(glVertexAttribLFormat_inner) => glVertexAttribLFormat_inner(attribindex, size, type_, relativeoffset),
    None => gl_not_loaded("glVertexAttribLFormat"),
  }
}
static glVertexAttribLFormat_p: GlFnCell<glVertexAttribLFormat_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribLFormat_is_loaded() -> bool {
  unsafe { *glVertexAttribLFormat_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribLFormat_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribLFormat_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribLFormat_t>>(gl_ptr_filter(f(b"glVertexAttribLFormat\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribLFormat_reset_ptr() {
  *glVertexAttribLFormat_p.0.get() = None;
}
/// [glVertexAttribLPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribLPointer.xhtml)
/// * `type` group: VertexAttribLType
/// * `pointer` len: size
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribLPointer(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, pointer: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribLPointer_p.0.get() } {
    Some(glVertexAttribLPointer_inner) => glVertexAttribLPointer_inner(index, size, type_, stride, pointer),
    None => gl_not_loaded("glVertexAttribLPointer"),
  }
}
static glVertexAttribLPointer_p: GlFnCell<glVertexAttribLPointer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribLPointer_is_loaded() -> bool {
  unsafe { *glVertexAttribLPointer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribLPointer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribLPointer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribLPointer_t>>(gl_ptr_filter(f(b"glVertexAttribLPointer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribLPointer_reset_ptr() {
  *glVertexAttribLPointer_p.0.get() = None;
}
/// [glVertexAttribP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP1ui.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP1ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP1ui_p.0.get() } {
    Some(glVertexAttribP1ui_inner) => glVertexAttribP1ui_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP1ui"),
  }
}
static glVertexAttribP1ui_p: GlFnCell<glVertexAttribP1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP1ui_is_loaded() -> bool {
  unsafe { *glVertexAttribP1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP1ui_t>>(gl_ptr_filter(f(b"glVertexAttribP1ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP1ui_reset_ptr() {
  *glVertexAttribP1ui_p.0.get() = None;
}
/// [glVertexAttribP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP1uiv.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP1uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP1uiv_p.0.get() } {
    Some(glVertexAttribP1uiv_inner) => glVertexAttribP1uiv_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP1uiv"),
  }
}
static glVertexAttribP1uiv_p: GlFnCell<glVertexAttribP1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP1uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribP1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP1uiv_t>>(gl_ptr_filter(f(b"glVertexAttribP1uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP1uiv_reset_ptr() {
  *glVertexAttribP1uiv_p.0.get() = None;
}
/// [glVertexAttribP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP2ui.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP2ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP2ui_p.0.get() } {
    Some(glVertexAttribP2ui_inner) => glVertexAttribP2ui_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP2ui"),
  }
}
static glVertexAttribP2ui_p: GlFnCell<glVertexAttribP2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP2ui_is_loaded() -> bool {
  unsafe { *glVertexAttribP2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP2ui_t>>(gl_ptr_filter(f(b"glVertexAttribP2ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP2ui_reset_ptr() {
  *glVertexAttribP2ui_p.0.get() = None;
}
/// [glVertexAttribP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP2uiv.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP2uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP2uiv_p.0.get() } {
    Some(glVertexAttribP2uiv_inner) => glVertexAttribP2uiv_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP2uiv"),
  }
}
static glVertexAttribP2uiv_p: GlFnCell<glVertexAttribP2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP2uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribP2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP2uiv_t>>(gl_ptr_filter(f(b"glVertexAttribP2uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP2uiv_reset_ptr() {
  *glVertexAttribP2uiv_p.0.get() = None;
}
/// [glVertexAttribP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP3ui.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP3ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP3ui_p.0.get() } {
    Some(glVertexAttribP3ui_inner) => glVertexAttribP3ui_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP3ui"),
  }
}
static glVertexAttribP3ui_p: GlFnCell<glVertexAttribP3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP3ui_is_loaded() -> bool {
  unsafe { *glVertexAttribP3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP3ui_t>>(gl_ptr_filter(f(b"glVertexAttribP3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP3ui_reset_ptr() {
  *glVertexAttribP3ui_p.0.get() = None;
}
/// [glVertexAttribP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP3uiv.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP3uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP3uiv_p.0.get() } {
    Some(glVertexAttribP3uiv_inner) => glVertexAttribP3uiv_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP3uiv"),
  }
}
static glVertexAttribP3uiv_p: GlFnCell<glVertexAttribP3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP3uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribP3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP3uiv_t>>(gl_ptr_filter(f(b"glVertexAttribP3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP3uiv_reset_ptr() {
  *glVertexAttribP3uiv_p.0.get() = None;
}
/// [glVertexAttribP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP4ui.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP4ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP4ui_p.0.get() } {
    Some(glVertexAttribP4ui_inner) => glVertexAttribP4ui_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP4ui"),
  }
}
static glVertexAttribP4ui_p: GlFnCell<glVertexAttribP4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP4ui_is_loaded() -> bool {
  unsafe { *glVertexAttribP4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP4ui_t>>(gl_ptr_filter(f(b"glVertexAttribP4ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP4ui_reset_ptr() {
  *glVertexAttribP4ui_p.0.get() = None;
}
/// [glVertexAttribP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP4uiv.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP4uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP4uiv_p.0.get() } {
    Some(glVertexAttribP4uiv_inner) => glVertexAttribP4uiv_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP4uiv"),
  }
}
static glVertexAttribP4uiv_p: GlFnCell<glVertexAttribP4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP4uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribP4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP4uiv_t>>(gl_ptr_filter(f(b"glVertexAttribP4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP4uiv_reset_ptr() {
  *glVertexAttribP4uiv_p.0.get() = None;
}
/// [glVertexAttribPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml)
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
/// * `pointer` len: COMPSIZE(size,type,stride)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribPointer(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribPointer_p.0.get() } {
    Some(glVertexAttribPointer_inner) => glVertexAttribPointer_inner(index, size, type_, normalized, stride, pointer),
    None => gl_not_loaded("glVertexAttribPointer"),
  }
}
static glVertexAttribPointer_p: GlFnCell<glVertexAttribPointer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribPointer_is_loaded() -> bool {
  unsafe { *glVertexAttribPointer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribPointer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribPointer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribPointer_t>>(gl_ptr_filter(f(b"glVertexAttribPointer\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexAttribPointer_reset_ptr() {
  *glVertexAttribPointer_p.0.get() = None;
}
/// [glVertexBindingDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexBindingDivisor.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexBindingDivisor_p.0.get() } {
    Some(glVertexBindingDivisor_inner) => glVertexBindingDivisor_inner(bindingindex, divisor),
    None => gl_not_loaded("glVertexBindingDivisor"),
  }
}
static glVertexBindingDivisor_p: GlFnCell<glVertexBindingDivisor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexBindingDivisor_is_loaded() -> bool {
  unsafe { *glVertexBindingDivisor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexBindingDivisor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexBindingDivisor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexBindingDivisor_t>>(gl_ptr_filter(f(b"glVertexBindingDivisor\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexBindingDivisor_reset_ptr() {
  *glVertexBindingDivisor_p.0.get() = None;
}
/// [glVertexP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP2ui.xhtml)
/// * `type` group: VertexPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexP2ui(type_: VertexPointerType, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexP2ui_p.0.get() } {
    Some(glVertexP2ui_inner) => glVertexP2ui_inner(type_, value),
    None => gl_not_loaded("glVertexP2ui"),
  }
}
static glVertexP2ui_p: GlFnCell<glVertexP2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexP2ui_is_loaded() -> bool {
  unsafe { *glVertexP2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexP2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexP2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexP2ui_t>>(gl_ptr_filter(f(b"glVertexP2ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexP2ui_reset_ptr() {
  *glVertexP2ui_p.0.get() = None;
}
/// [glVertexP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP2uiv.xhtml)
/// * `type` group: VertexPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexP2uiv(type_: VertexPointerType, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexP2uiv_p.0.get() } {
    Some(glVertexP2uiv_inner) => glVertexP2uiv_inner(type_, value),
    None => gl_not_loaded("glVertexP2uiv"),
  }
}
static glVertexP2uiv_p: GlFnCell<glVertexP2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexP2uiv_is_loaded() -> bool {
  unsafe { *glVertexP2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexP2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexP2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexP2uiv_t>>(gl_ptr_filter(f(b"glVertexP2uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexP2uiv_reset_ptr() {
  *glVertexP2uiv_p.0.get() = None;
}
/// [glVertexP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP3ui.xhtml)
/// * `type` group: VertexPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexP3ui(type_: VertexPointerType, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexP3ui_p.0.get() } {
    Some(glVertexP3ui_inner) => glVertexP3ui_inner(type_, value),
    None => gl_not_loaded("glVertexP3ui"),
  }
}
static glVertexP3ui_p: GlFnCell<glVertexP3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexP3ui_is_loaded() -> bool {
  unsafe { *glVertexP3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexP3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexP3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexP3ui_t>>(gl_ptr_filter(f(b"glVertexP3ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexP3ui_reset_ptr() {
  *glVertexP3ui_p.0.get() = None;
}
/// [glVertexP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP3uiv.xhtml)
/// * `type` group: VertexPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexP3uiv(type_: VertexPointerType, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexP3uiv_p.0.get() } {
    Some(glVertexP3uiv_inner) => glVertexP3uiv_inner(type_, value),
    None => gl_not_loaded("glVertexP3uiv"),
  }
}
static glVertexP3uiv_p: GlFnCell<glVertexP3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexP3uiv_is_loaded() -> bool {
  unsafe { *glVertexP3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexP3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexP3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexP3uiv_t>>(gl_ptr_filter(f(b"glVertexP3uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexP3uiv_reset_ptr() {
  *glVertexP3uiv_p.0.get() = None;
}
/// [glVertexP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP4ui.xhtml)
/// * `type` group: VertexPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexP4ui(type_: VertexPointerType, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexP4ui_p.0.get() } {
    Some(glVertexP4ui_inner) => glVertexP4ui_inner(type_, value),
    None => gl_not_loaded("glVertexP4ui"),
  }
}
static glVertexP4ui_p: GlFnCell<glVertexP4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexP4ui_is_loaded() -> bool {
  unsafe { *glVertexP4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexP4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexP4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexP4ui_t>>(gl_ptr_filter(f(b"glVertexP4ui\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexP4ui_reset_ptr() {
  *glVertexP4ui_p.0.get() = None;
}
/// [glVertexP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP4uiv.xhtml)
/// * `type` group: VertexPointerType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexP4uiv(type_: VertexPointerType, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexP4uiv_p.0.get() } {
    Some(glVertexP4uiv_inner) => glVertexP4uiv_inner(type_, value),
    None => gl_not_loaded("glVertexP4uiv"),
  }
}
static glVertexP4uiv_p: GlFnCell<glVertexP4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexP4uiv_is_loaded() -> bool {
  unsafe { *glVertexP4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexP4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexP4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexP4uiv_t>>(gl_ptr_filter(f(b"glVertexP4uiv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glVertexP4uiv_reset_ptr() {
  *glVertexP4uiv_p.0.get() = None;
}
/// [glViewport](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml)
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glViewport_p.0.get() } {
    Some(glViewport_inner) => glViewport_inner(x, y, width, height),
    None => gl_not_loaded("glViewport"),
  }
}
static glViewport_p: GlFnCell<glViewport_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glViewport_is_loaded() -> bool {
  unsafe { *glViewport_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glViewport_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glViewport_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glViewport_t>>(gl_ptr_filter(f(b"glViewport\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glViewport_reset_ptr() {
  *glViewport_p.0.get() = None;
}
/// [glViewportArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportArrayv.xhtml)
/// * `v` len: COMPSIZE(count)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glViewportArrayv_p.0.get() } {
    Some(glViewportArrayv_inner) => glViewportArrayv_inner(first, count, v),
    None => gl_not_loaded("glViewportArrayv"),
  }
}
static glViewportArrayv_p: GlFnCell<glViewportArrayv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glViewportArrayv_is_loaded() -> bool {
  unsafe { *glViewportArrayv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glViewportArrayv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glViewportArrayv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glViewportArrayv_t>>(gl_ptr_filter(f(b"glViewportArrayv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glViewportArrayv_reset_ptr() {
  *glViewportArrayv_p.0.get() = None;
}
/// [glViewportIndexedf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedf.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glViewportIndexedf_p.0.get() } {
    Some(glViewportIndexedf_inner) => glViewportIndexedf_inner(index, x, y, w, h),
    None => gl_not_loaded("glViewportIndexedf"),
  }
}
static glViewportIndexedf_p: GlFnCell<glViewportIndexedf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glViewportIndexedf_is_loaded() -> bool {
  unsafe { *glViewportIndexedf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glViewportIndexedf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glViewportIndexedf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glViewportIndexedf_t>>(gl_ptr_filter(f(b"glViewportIndexedf\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glViewportIndexedf_reset_ptr() {
  *glViewportIndexedf_p.0.get() = None;
}
/// [glViewportIndexedfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedfv.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glViewportIndexedfv(index: GLuint, v: *const [GLfloat; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glViewportIndexedfv_p.0.get() } {
    Some(glViewportIndexedfv_inner) => glViewportIndexedfv_inner(index, v),
    None => gl_not_loaded("glViewportIndexedfv"),
  }
}
static glViewportIndexedfv_p: GlFnCell<glViewportIndexedfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glViewportIndexedfv_is_loaded() -> bool {
  unsafe { *glViewportIndexedfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glViewportIndexedfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glViewportIndexedfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glViewportIndexedfv_t>>(gl_ptr_filter(f(b"glViewportIndexedfv\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glViewportIndexedfv_reset_ptr() {
  *glViewportIndexedfv_p.0.get() = None;
}
/// [glWaitSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWaitSync.xhtml)
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncBehaviorFlags
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glWaitSync_p.0.get() } {
    Some(glWaitSync_inner) => glWaitSync_inner(sync, flags, timeout),
    None => gl_not_loaded("glWaitSync"),
  }
}
static glWaitSync_p: GlFnCell<glWaitSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glWaitSync_is_loaded() -> bool {
  unsafe { *glWaitSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glWaitSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glWaitSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glWaitSync_t>>(gl_ptr_filter(f(b"glWaitSync\0".as_ptr())));
}
#[doc(hidden)]
pub unsafe fn glWaitSync_reset_ptr() {
  *glWaitSync_p.0.get() = None;
}
